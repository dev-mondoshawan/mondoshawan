//! TriStream Mining Architecture
//! 
//! Implements three parallel mining streams:
//! - Stream A: ASIC mining (Blake3), 10s blocks, 10,000 txs/block, 50 MSHW reward
//! - Stream B: CPU/GPU mining (KHeavyHash), 1s blocks, 5,000 txs/block, 25 MSHW reward
//! - Stream C: ZK proofs, 100ms blocks, 1,000 txs/block, 0 MSHW (fee-based only)

pub mod fairness;
pub mod ordering;

use crate::blockchain::{Blockchain, Block, BlockHeader, Transaction};
use crate::types::{Address, StreamType};
use crate::sharding::ShardManager;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use crossbeam_queue::SegQueue;

/// Block rewards for each stream (in base units, 1 MSHW = 1_000_000_000_000_000_000 base units)
pub const STREAM_A_REWARD: u128 = 50_000_000_000_000_000_000; // 50 MSHW
pub const STREAM_B_REWARD: u128 = 25_000_000_000_000_000_000; // 25 MSHW
pub const STREAM_C_REWARD: u128 = 0; // Fee-based only

/// Maximum transactions per block for each stream
pub const STREAM_A_MAX_TXS: usize = 10_000;
pub const STREAM_B_MAX_TXS: usize = 5_000;
pub const STREAM_C_MAX_TXS: usize = 1_000;

/// Block times for each stream
pub const STREAM_A_BLOCK_TIME: Duration = Duration::from_secs(10);
pub const STREAM_B_BLOCK_TIME: Duration = Duration::from_secs(1);
pub const STREAM_C_BLOCK_TIME: Duration = Duration::from_millis(100);

/// Maximum transaction pool size (DoS protection - prevents memory exhaustion)
/// When limit is reached, oldest transactions are evicted (FIFO)
pub const MAX_TX_POOL_SIZE: usize = 100_000; // 100k transactions max

/// Block submission message for channel-based processing
struct BlockSubmission {
    block: Block,
    stream_type: StreamType,
    block_number: u64,
    reward: u128,
    fees: u128,
}

/// Mining manager for TriStream architecture
pub struct MiningManager {
    blockchain: Arc<RwLock<Blockchain>>,
    tx_pool: Arc<SegQueue<Transaction>>, // Lock-free concurrent queue
    tx_pool_size: Arc<AtomicUsize>, // Atomic counter for pool size
    block_counter: Arc<AtomicU64>, // Lock-free atomic counter (prevents deadlock)
    miner_address: Address, // Address that receives block rewards
    is_mining: Arc<RwLock<bool>>,
    shard_manager: Option<Arc<ShardManager>>, // Optional shard manager
    fairness_analyzer: Arc<tokio::sync::RwLock<fairness::FairnessAnalyzer>>, // Fairness metrics
    ordering_policy: Arc<RwLock<ordering::OrderingPolicy>>, // Transaction ordering policy
    ordering_context: Arc<RwLock<ordering::OrderingContext>>, // Ordering context
    metrics: Option<crate::metrics::MetricsHandle>, // Optional metrics
    block_sender: mpsc::UnboundedSender<BlockSubmission>, // Channel sender for block submissions
    node_registry: Option<Arc<tokio::sync::RwLock<crate::governance::NodeRegistry>>>, // Optional node registry for participation tracking
    node_identity: Option<crate::governance::NodeIdentity>, // Node identity for participation tracking
}

impl MiningManager {
    pub fn new(blockchain: Arc<RwLock<Blockchain>>, miner_address: Address) -> Self {
        // Create channel for block submissions (serializes block additions)
        let (block_sender, block_receiver) = mpsc::unbounded_channel();
        
        // Start block processor task
        let blockchain_processor = blockchain.clone();
        let miner_address_processor = miner_address;
        let fairness_analyzer_processor = Arc::new(tokio::sync::RwLock::new(fairness::FairnessAnalyzer::new()));
        let metrics_processor = None::<crate::metrics::MetricsHandle>;
        let node_registry_processor = None::<Arc<tokio::sync::RwLock<crate::governance::NodeRegistry>>>;
        let node_identity_processor = None::<crate::governance::NodeIdentity>;
        
        tokio::spawn(async move {
            process_blocks(
                block_receiver,
                blockchain_processor,
                miner_address_processor,
                fairness_analyzer_processor,
                metrics_processor,
                node_registry_processor,
                node_identity_processor,
            ).await;
        });
        
        Self {
            blockchain,
            tx_pool: Arc::new(SegQueue::new()),
            tx_pool_size: Arc::new(AtomicUsize::new(0)),
            block_counter: Arc::new(AtomicU64::new(0)), // Lock-free atomic
            miner_address,
            is_mining: Arc::new(RwLock::new(false)),
            shard_manager: None,
            fairness_analyzer: Arc::new(tokio::sync::RwLock::new(fairness::FairnessAnalyzer::new())),
            ordering_policy: Arc::new(RwLock::new(ordering::OrderingPolicy::default())),
            ordering_context: Arc::new(RwLock::new(ordering::OrderingContext::new())),
            metrics: None,
            block_sender,
            node_registry: None,
            node_identity: None,
        }
    }
    
    /// Create mining manager with node registry for participation tracking
    pub fn with_node_registry(
        blockchain: Arc<RwLock<Blockchain>>,
        miner_address: Address,
        node_registry: Arc<tokio::sync::RwLock<crate::governance::NodeRegistry>>,
        node_identity: crate::governance::NodeIdentity,
    ) -> Self {
        // Create channel for block submissions
        let (block_sender, block_receiver) = mpsc::unbounded_channel();
        
        // Start block processor task
        let blockchain_processor = blockchain.clone();
        let miner_address_processor = miner_address;
        let fairness_analyzer_processor = Arc::new(tokio::sync::RwLock::new(fairness::FairnessAnalyzer::new()));
        let metrics_processor = None::<crate::metrics::MetricsHandle>;
        let node_registry_processor = Some(node_registry.clone());
        let node_identity_processor = Some(node_identity.clone());
        
        tokio::spawn(async move {
            process_blocks(
                block_receiver,
                blockchain_processor,
                miner_address_processor,
                fairness_analyzer_processor,
                metrics_processor,
                node_registry_processor,
                node_identity_processor,
            ).await;
        });
        
        Self {
            blockchain,
            tx_pool: Arc::new(SegQueue::new()),
            tx_pool_size: Arc::new(AtomicUsize::new(0)),
            block_counter: Arc::new(AtomicU64::new(0)),
            miner_address,
            is_mining: Arc::new(RwLock::new(false)),
            shard_manager: None,
            fairness_analyzer: Arc::new(tokio::sync::RwLock::new(fairness::FairnessAnalyzer::new())),
            ordering_policy: Arc::new(RwLock::new(ordering::OrderingPolicy::default())),
            ordering_context: Arc::new(RwLock::new(ordering::OrderingContext::new())),
            metrics: None,
            block_sender,
            node_registry: Some(node_registry),
            node_identity: Some(node_identity),
        }
    }

    /// Create mining manager with sharding
    pub fn with_sharding(
        blockchain: Arc<RwLock<Blockchain>>,
        miner_address: Address,
        shard_manager: Arc<ShardManager>,
    ) -> Self {
        // Create channel for block submissions
        let (block_sender, block_receiver) = mpsc::unbounded_channel();
        
        // Start block processor task
        let blockchain_processor = blockchain.clone();
        let miner_address_processor = miner_address;
        let fairness_analyzer_processor = Arc::new(tokio::sync::RwLock::new(fairness::FairnessAnalyzer::new()));
        let metrics_processor = None::<crate::metrics::MetricsHandle>;
        let node_registry_processor = None::<Arc<tokio::sync::RwLock<crate::governance::NodeRegistry>>>;
        let node_identity_processor = None::<crate::governance::NodeIdentity>;
        
        tokio::spawn(async move {
            process_blocks(
                block_receiver,
                blockchain_processor,
                miner_address_processor,
                fairness_analyzer_processor,
                metrics_processor,
                node_registry_processor,
                node_identity_processor,
            ).await;
        });
        
        Self {
            blockchain,
            tx_pool: Arc::new(SegQueue::new()),
            tx_pool_size: Arc::new(AtomicUsize::new(0)),
            block_counter: Arc::new(AtomicU64::new(0)), // Lock-free atomic
            miner_address,
            is_mining: Arc::new(RwLock::new(false)),
            shard_manager: Some(shard_manager),
            fairness_analyzer: Arc::new(tokio::sync::RwLock::new(fairness::FairnessAnalyzer::new())),
            ordering_policy: Arc::new(RwLock::new(ordering::OrderingPolicy::default())),
            ordering_context: Arc::new(RwLock::new(ordering::OrderingContext::new())),
            metrics: None,
            block_sender,
            node_registry: None,
            node_identity: None,
        }
    }
    
    /// Set metrics handle
    pub fn set_metrics(&mut self, metrics: crate::metrics::MetricsHandle) {
        self.metrics = Some(metrics);
    }
    
    /// Clone for mining (internal use)
    /// Clone mining manager for parallel stream mining
    /// Note: node_registry and node_identity are shared across all streams
    fn clone_for_mining(&self) -> Self {
        Self {
            blockchain: self.blockchain.clone(),
            tx_pool: self.tx_pool.clone(),
            tx_pool_size: self.tx_pool_size.clone(),
            block_counter: self.block_counter.clone(),
            miner_address: self.miner_address,
            is_mining: self.is_mining.clone(),
            shard_manager: self.shard_manager.clone(),
            fairness_analyzer: self.fairness_analyzer.clone(),
            ordering_policy: self.ordering_policy.clone(),
            ordering_context: self.ordering_context.clone(),
            metrics: self.metrics.clone(),
            block_sender: self.block_sender.clone(), // Clone sender (receiver is shared)
            node_registry: self.node_registry.clone(),
            node_identity: self.node_identity.clone(),
        }
    }
    
    /// Set transaction ordering policy
    pub async fn set_ordering_policy(&self, policy: ordering::OrderingPolicy) {
        *self.ordering_policy.write().await = policy;
    }
    
    /// Get current ordering policy
    pub async fn get_ordering_policy(&self) -> ordering::OrderingPolicy {
        *self.ordering_policy.read().await
    }

    /// Add transaction to pool
    pub async fn add_transaction(&self, tx: Transaction) -> crate::error::BlockchainResult<()> {        
        // Record transaction arrival time for fairness analysis
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        {
            let mut analyzer = self.fairness_analyzer.write().await;
            analyzer.record_transaction_arrival(tx.hash, timestamp);
        }
        
        // Record arrival in ordering context
        {
            let mut context = self.ordering_context.write().await;
            context.record_arrival(tx.hash, timestamp);
        }
        
        // Check pool size and evict oldest if needed (FIFO eviction)
        let current_size = self.tx_pool_size.load(Ordering::Acquire);
        if current_size >= MAX_TX_POOL_SIZE {
            // Evict oldest transaction (pop from queue and discard)
            if let Some(_evicted) = self.tx_pool.pop() {
                self.tx_pool_size.fetch_sub(1, Ordering::Release);
            }
        }
        
        // Add to lock-free queue
        self.tx_pool.push(tx);
        self.tx_pool_size.fetch_add(1, Ordering::Release);
        Ok(())
    }
    
    /// Get fairness metrics for a block
    pub async fn get_fairness_metrics(&self, block: &Block) -> fairness::FairnessMetrics {
        let analyzer = self.fairness_analyzer.read().await;
        analyzer.analyze_block(block)
    }
    

    /// Get pending transactions count
    pub async fn pending_count(&self) -> usize {
        self.tx_pool_size.load(Ordering::Acquire)
    }

    /// Start mining all streams
    pub async fn start_mining(&self) {
        *self.is_mining.write().await = true;
        
        // Clone the MiningManager for each stream
        let self_a = self.clone_for_mining();
        let self_b = self.clone_for_mining();
        let self_c = self.clone_for_mining();
        
        tokio::spawn(async move {
            self_a.mine_stream_a().await;
        });
        
        tokio::spawn(async move {
            self_b.mine_stream_b().await;
        });
        
        tokio::spawn(async move {
            self_c.mine_stream_c().await;
        });
    }

    /// Stop mining
    pub async fn stop_mining(&self) {
        *self.is_mining.write().await = false;
    }

    /// Check if mining is active
    pub fn is_mining(&self) -> &Arc<RwLock<bool>> {
        &self.is_mining
    }

    /// Mine Stream A blocks (ASIC, 10s blocks, 10,000 txs, 50 MSHW reward)
    async fn mine_stream_a(&self) {
        // Stagger startup to avoid lock contention
        sleep(Duration::from_millis(100)).await;
        
        while *self.is_mining.read().await {
            // Extract transactions from lock-free queue and apply ordering policy
            let txs = {
                // Pop transactions from lock-free queue
                let count = self.tx_pool_size.load(Ordering::Acquire).min(STREAM_A_MAX_TXS);
                let mut txs = Vec::with_capacity(count);
                
                // Pop up to count transactions (non-blocking)
                for _ in 0..count {
                    if let Some(tx) = self.tx_pool.pop() {
                        txs.push(tx);
                        self.tx_pool_size.fetch_sub(1, Ordering::Release);
                    } else {
                        break; // Queue is empty
                    }
                }
                            
                // Apply ordering policy if we have transactions
                // Use try_write to avoid deadlock - if context is busy, skip ordering update
                if !txs.is_empty() {
                    let policy = *self.ordering_policy.read().await;
                    // Use timeout to avoid deadlock - if context is busy, skip update
                    match tokio::time::timeout(Duration::from_millis(10), self.ordering_context.write()).await {
                        Ok(mut context) => {
                            context.update_time();
                            txs = ordering::order_transactions(txs, policy, &mut context);
                        }
                        Err(_) => {
                            // Context is busy, order without updating context (non-critical)
                            let mut temp_context = ordering::OrderingContext::new();
                            txs = ordering::order_transactions(txs, policy, &mut temp_context);
                        }
                    }
                }
                            
                txs
            };

            // Get parent hashes and block number
            let (parent_hashes, block_number) = {
                let blockchain = self.blockchain.read().await;
                let blocks = blockchain.get_blocks();
                let parents = if !blocks.is_empty() {
                    // Get last few blocks as parents (DAG structure)
                    let mut parents = Vec::new();
                    let start_idx = if blocks.len() >= 3 { blocks.len() - 3 } else { 0 };
                    for block in &blocks[start_idx..] {
                        parents.push(block.hash);
                    }
                    parents
                } else {
                    Vec::new()
                };
                drop(blockchain);
                
                // Use atomic fetch_add to avoid deadlock
                let num = self.block_counter.fetch_add(1, Ordering::SeqCst);
                (parents, num)
            };

            // Create block
            let header = BlockHeader::new(parent_hashes.clone(), block_number, StreamType::StreamA, 4);
            let block = Block::new(header, txs.clone(), parent_hashes);

            // Send block to processor via channel (non-blocking, eliminates deadlock)
            let _ = self.block_sender.send(BlockSubmission {
                block,
                stream_type: StreamType::StreamA,
                block_number,
                reward: STREAM_A_REWARD,
                fees: 0,
            });
            
            println!("📤 Stream A: Prepared block #{} with {} txs, reward: {} MSHW", 
                block_number, txs.len(), STREAM_A_REWARD / 1_000_000_000_000_000_000);

            sleep(STREAM_A_BLOCK_TIME).await;
        }
    }

    /// Mine Stream B blocks (CPU/GPU, 1s blocks, 5,000 txs, 25 MSHW reward)
    async fn mine_stream_b(&self) {
        // Stagger startup to avoid lock contention
        sleep(Duration::from_millis(200)).await;
        
        while *self.is_mining.read().await {
            let txs = if let Some(shard_manager) = &self.shard_manager {
                // Get transactions from all shards
                let mut all_txs = Vec::new();
                let shard_count = shard_manager.shard_count();
                let txs_per_shard = STREAM_B_MAX_TXS / shard_count.max(1);
                
                for shard_id in 0..shard_count {
                    let shard_txs = shard_manager.get_shard_transactions(shard_id, txs_per_shard).await;
                    all_txs.extend(shard_txs);
                    if all_txs.len() >= STREAM_B_MAX_TXS {
                        break;
                    }
                }
                
                if all_txs.is_empty() {
                    sleep(STREAM_B_BLOCK_TIME).await;
                    continue;
                }
                
                // Remove transactions from shards
                for shard_id in 0..shard_count {
                    let _ = shard_manager.remove_shard_transactions(shard_id, txs_per_shard).await;
                }
                
                let mut txs: Vec<Transaction> = all_txs.into_iter().take(STREAM_B_MAX_TXS).collect();
                
                // Apply ordering policy
                let policy = *self.ordering_policy.read().await;
                let mut context = self.ordering_context.write().await;
                context.update_time();
                txs = ordering::order_transactions(txs, policy, &mut context);
                drop(context);
                
                txs
            } else {
                // Use lock-free transaction pool
                let count = self.tx_pool_size.load(Ordering::Acquire).min(STREAM_B_MAX_TXS);
                let mut txs = Vec::with_capacity(count);
                
                // Pop up to count transactions (non-blocking)
                for _ in 0..count {
                    if let Some(tx) = self.tx_pool.pop() {
                        txs.push(tx);
                        self.tx_pool_size.fetch_sub(1, Ordering::Release);
                    } else {
                        break; // Queue is empty
                    }
                }
                
                // Apply ordering policy if we have transactions
                // Use try_write to avoid deadlock - if context is busy, skip ordering update
                if !txs.is_empty() {
                    let policy = *self.ordering_policy.read().await;
                    // Use timeout to avoid deadlock - if context is busy, skip update
                    match tokio::time::timeout(Duration::from_millis(10), self.ordering_context.write()).await {
                        Ok(mut context) => {
                            context.update_time();
                            txs = ordering::order_transactions(txs, policy, &mut context);
                        }
                        Err(_) => {
                            // Context is busy, order without updating context (non-critical)
                            let mut temp_context = ordering::OrderingContext::new();
                            txs = ordering::order_transactions(txs, policy, &mut temp_context);
                        }
                    }
                }
                
                txs
            };

            let (parent_hashes, block_number) = {
                let blockchain = self.blockchain.read().await;
                let blocks = blockchain.get_blocks();
                let parents = if !blocks.is_empty() {
                    // Get last few blocks as parents (DAG structure)
                    let mut parents = Vec::new();
                    let start_idx = if blocks.len() >= 3 { blocks.len() - 3 } else { 0 };
                    for block in &blocks[start_idx..] {
                        parents.push(block.hash);
                    }
                    parents
                } else {
                    Vec::new()
                };
                drop(blockchain);
                
                // Use atomic fetch_add to avoid deadlock
                let num = self.block_counter.fetch_add(1, Ordering::SeqCst);
                (parents, num)
            };

            let header = BlockHeader::new(parent_hashes.clone(), block_number, StreamType::StreamB, 4);
            let block = Block::new(header, txs.clone(), parent_hashes);

            // Send block to processor via channel (non-blocking, eliminates deadlock)
            let _ = self.block_sender.send(BlockSubmission {
                block,
                stream_type: StreamType::StreamB,
                block_number,
                reward: STREAM_B_REWARD,
                fees: 0,
            });
            
            println!("📤 Stream B: Prepared block #{} with {} txs, reward: {} MSHW", 
                block_number, txs.len(), STREAM_B_REWARD / 1_000_000_000_000_000_000);

            sleep(STREAM_B_BLOCK_TIME).await;
        }
    }

    /// Mine Stream C blocks (ZK, 100ms blocks, 1,000 txs, fee-based only)
    async fn mine_stream_c(&self) {
        // Stagger startup to avoid lock contention
        sleep(Duration::from_millis(300)).await;
        
        while *self.is_mining.read().await {
            let txs = {
                // Pop transactions from lock-free queue
                let count = self.tx_pool_size.load(Ordering::Acquire).min(STREAM_C_MAX_TXS);
                let mut txs = Vec::with_capacity(count);
                
                // Pop up to count transactions (non-blocking)
                for _ in 0..count {
                    if let Some(tx) = self.tx_pool.pop() {
                        txs.push(tx);
                        self.tx_pool_size.fetch_sub(1, Ordering::Release);
                    } else {
                        break; // Queue is empty
                    }
                }
                
                // Apply ordering policy if we have transactions
                // Use try_write to avoid deadlock - if context is busy, skip ordering update
                if !txs.is_empty() {
                    let policy = *self.ordering_policy.read().await;
                    // Use timeout to avoid deadlock - if context is busy, skip update
                    match tokio::time::timeout(Duration::from_millis(10), self.ordering_context.write()).await {
                        Ok(mut context) => {
                            context.update_time();
                            txs = ordering::order_transactions(txs, policy, &mut context);
                        }
                        Err(_) => {
                            // Context is busy, order without updating context (non-critical)
                            let mut temp_context = ordering::OrderingContext::new();
                            txs = ordering::order_transactions(txs, policy, &mut temp_context);
                        }
                    }
                }
                
                txs
            };

            // Calculate total fees from transactions
            let total_fees: u128 = txs.iter().map(|tx| tx.fee).sum();

            let (parent_hashes, block_number) = {
                let blockchain = self.blockchain.read().await;
                let blocks = blockchain.get_blocks();
                let parents = if !blocks.is_empty() {
                    // Get last few blocks as parents (DAG structure)
                    let mut parents = Vec::new();
                    let start_idx = if blocks.len() >= 3 { blocks.len() - 3 } else { 0 };
                    for block in &blocks[start_idx..] {
                        parents.push(block.hash);
                    }
                    parents
                } else {
                    Vec::new()
                };
                drop(blockchain);
                
                // Use atomic fetch_add to avoid deadlock
                let num = self.block_counter.fetch_add(1, Ordering::SeqCst);
                (parents, num)
            };

            let header = BlockHeader::new(parent_hashes.clone(), block_number, StreamType::StreamC, 4);
            let block = Block::new(header, txs.clone(), parent_hashes);

            // Send block to processor via channel (non-blocking, eliminates deadlock)
            let _ = self.block_sender.send(BlockSubmission {
                block,
                stream_type: StreamType::StreamC,
                block_number,
                reward: 0, // Stream C is fee-based only
                fees: total_fees,
            });
            
            println!("📤 Stream C: Prepared block #{} with {} txs, fees: {} MSHW", 
                block_number, txs.len(), total_fees / 1_000_000_000_000_000_000);

            sleep(STREAM_C_BLOCK_TIME).await;
        }
    }
}

/// Process blocks from channel - serializes block additions to prevent deadlock
async fn process_blocks(
    mut receiver: mpsc::UnboundedReceiver<BlockSubmission>,
    blockchain: Arc<RwLock<Blockchain>>,
    miner_address: Address,
    fairness_analyzer: Arc<tokio::sync::RwLock<fairness::FairnessAnalyzer>>,
    metrics: Option<crate::metrics::MetricsHandle>,
    node_registry: Option<Arc<tokio::sync::RwLock<crate::governance::NodeRegistry>>>,
    node_identity: Option<crate::governance::NodeIdentity>,
) {
    while let Some(submission) = receiver.recv().await {
        let BlockSubmission { block, stream_type, block_number, reward, fees } = submission;
        
        // Add block to blockchain (serialized - no contention)
        {
            let mut blockchain = blockchain.write().await;
            if blockchain.add_block(block.clone()).is_ok() {
                // Give miner the reward/fees
                let current_balance = blockchain.get_balance(miner_address);
                let total_reward = reward + fees;
                if let Err(e) = blockchain.set_balance(miner_address, current_balance + total_reward) {
                    eprintln!("Warning: Failed to persist reward: {}", e);
                }
            } else {
                // Block validation failed, skip
                continue;
            }
        } // Release blockchain lock
        
        // Record participation in node registry (CRITICAL for longevity tracking)
        if let (Some(ref registry), Some(ref identity)) = (&node_registry, &node_identity) {
            let participation = crate::governance::ParticipationType::BlockMined {
                stream: stream_type,
                block_hash: block.hash,
            };
            let mut registry = registry.write().await;
            registry.record_participation(identity, participation);
        }
        
        // Analyze fairness (outside blockchain lock)
        let fairness_metrics = {
            let analyzer = fairness_analyzer.read().await;
            analyzer.analyze_block(&block)
        };
        
        // Record metrics
        if let Some(ref metrics) = metrics {
            let block_size = std::mem::size_of_val(&block) + 
                block.transactions.iter().map(|tx| std::mem::size_of_val(tx)).sum::<usize>();
            if let Ok(m) = (*metrics).lock() {
                let stream_name = match stream_type {
                    StreamType::StreamA => "A",
                    StreamType::StreamB => "B",
                    StreamType::StreamC => "C",
                };
                m.record_block_mined(stream_name, block_size, reward);
            }
        }
        
        // Print success message
        let stream_name = match stream_type {
            StreamType::StreamA => "A",
            StreamType::StreamB => "B",
            StreamType::StreamC => "C",
        };
        let reward_str = if fees > 0 {
            format!("fees: {} MSHW", fees / 1_000_000_000_000_000_000)
        } else {
            format!("reward: {} MSHW", reward / 1_000_000_000_000_000_000)
        };
        
        println!("✅ Stream {}: Mined block #{} with {} txs, {}, fairness: {:.2}%", 
            stream_name, block_number, block.transactions.len(), reward_str,
            fairness_metrics.fairness_score * 100.0);
    }
}
