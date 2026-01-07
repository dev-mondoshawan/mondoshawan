//! Sharding implementation
//! 
//! Implements horizontal sharding for blockchain scalability.
//! Supports transaction routing, cross-shard transactions, and shard synchronization.

use crate::blockchain::{Blockchain, Block, Transaction};
use crate::types::{Address, Hash, StreamType};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Maximum transaction pool size per shard (DoS protection)
/// When limit is reached, oldest transactions are evicted (FIFO)
pub const MAX_SHARD_TX_POOL_SIZE: usize = 50_000; // 50k transactions per shard max

/// Shard configuration
#[derive(Debug, Clone)]
pub struct ShardConfig {
    pub shard_count: usize,
    pub enable_cross_shard: bool,
    pub assignment_strategy: AssignmentStrategy,
}

/// Assignment strategy for shards
#[derive(Debug, Clone)]
pub enum AssignmentStrategy {
    ConsistentHashing,
    RoundRobin,
    AddressBased,
}

/// Cross-shard transaction status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CrossShardStatus {
    Pending,
    Committed,
    Failed,
}

/// Cross-shard transaction
#[derive(Debug, Clone)]
pub struct CrossShardTransaction {
    pub tx: Transaction,
    pub source_shard: usize,
    pub target_shard: usize,
    pub status: CrossShardStatus,
    pub id: Hash,
}

/// Shard manager
pub struct ShardManager {
    config: ShardConfig,
    shards: Vec<Arc<RwLock<Shard>>>,
    cross_shard_txs: Arc<RwLock<HashMap<Hash, CrossShardTransaction>>>,
    round_robin_counter: Arc<RwLock<usize>>,
}

/// Individual shard
pub struct Shard {
    pub id: usize,
    pub blockchain: Arc<RwLock<Blockchain>>,
    pub transaction_pool: Vec<Transaction>,
    pub cross_shard_outgoing: Vec<Hash>, // Cross-shard tx IDs originating from this shard
    pub cross_shard_incoming: Vec<Hash>, // Cross-shard tx IDs targeting this shard
}

impl Shard {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            blockchain: Arc::new(RwLock::new(Blockchain::new())),
            transaction_pool: Vec::new(),
            cross_shard_outgoing: Vec::new(),
            cross_shard_incoming: Vec::new(),
        }
    }

    pub fn get_transactions(&self, limit: usize) -> Vec<Transaction> {
        self.transaction_pool.iter().take(limit).cloned().collect()
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        // Enforce pool size limit (DoS protection)
        // If pool is full, remove oldest transactions (FIFO eviction)
        while self.transaction_pool.len() >= MAX_SHARD_TX_POOL_SIZE {
            self.transaction_pool.remove(0); // Remove oldest transaction
        }
        
        self.transaction_pool.push(tx);
    }

    pub fn remove_transactions(&mut self, count: usize) -> Vec<Transaction> {
        if count >= self.transaction_pool.len() {
            self.transaction_pool.drain(..).collect()
        } else {
            self.transaction_pool.drain(..count).collect()
        }
    }
}

impl ShardManager {
    /// Create a new shard manager
    pub fn new(config: ShardConfig) -> Self {
        if config.shard_count == 0 {
            panic!("Shard count must be greater than 0");
        }
        
        let mut shards = Vec::new();
        for i in 0..config.shard_count {
            shards.push(Arc::new(RwLock::new(Shard::new(i))));
        }
        
        Self {
            config,
            shards,
            cross_shard_txs: Arc::new(RwLock::new(HashMap::new())),
            round_robin_counter: Arc::new(RwLock::new(0)),
        }
    }

    /// Add a transaction to the appropriate shard
    pub async fn add_transaction(&self, tx: Transaction) -> crate::error::BlockchainResult<()> {
        let from_shard = self.get_shard_for_address(&tx.from);
        let to_shard = if tx.to != [0u8; 20] {
            self.get_shard_for_address(&tx.to)
        } else {
            from_shard // Contract deployment goes to sender's shard
        };
        
        let tx_hash = tx.hash;
        
        // Check if this is a cross-shard transaction
        if from_shard != to_shard && self.config.enable_cross_shard {
            let tx_clone = tx.clone();
            
            // Create cross-shard transaction
            let cross_tx = CrossShardTransaction {
                tx: tx_clone.clone(),
                source_shard: from_shard,
                target_shard: to_shard,
                status: CrossShardStatus::Pending,
                id: tx_hash,
            };
            
            // Store cross-shard transaction
            {
                let mut cross_txs = self.cross_shard_txs.write().await;
                cross_txs.insert(tx_hash, cross_tx);
            }
            
            // Add to source shard (for validation)
            {
                let mut shard = self.shards[from_shard].write().await;
                shard.add_transaction(tx_clone);
                shard.cross_shard_outgoing.push(tx_hash);
            }
            
            // Mark in target shard
            {
                let mut shard = self.shards[to_shard].write().await;
                shard.cross_shard_incoming.push(tx_hash);
            }
        } else {
            // Same-shard transaction
            let mut shard = self.shards[from_shard].write().await;
            // add_transaction enforces MAX_SHARD_TX_POOL_SIZE with FIFO eviction
            shard.add_transaction(tx);
        }
        
        Ok(())
    }

    /// Get shard ID for an address
    pub fn get_shard_for_address(&self, address: &Address) -> usize {
        match self.config.assignment_strategy {
            AssignmentStrategy::ConsistentHashing => {
                // Use consistent hashing on address
                let hash = blake3::hash(address);
                let hash_bytes = hash.as_bytes();
                let hash_value = u64::from_le_bytes([
                    hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
                    hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
                ]);
                (hash_value as usize) % self.config.shard_count
            }
            AssignmentStrategy::RoundRobin => {
                // Round-robin based on address hash
                let hash = blake3::hash(address);
                let hash_bytes = hash.as_bytes();
                let hash_value = u64::from_le_bytes([
                    hash_bytes[0], hash_bytes[1], hash_bytes[2], hash_bytes[3],
                    hash_bytes[4], hash_bytes[5], hash_bytes[6], hash_bytes[7],
                ]);
                (hash_value as usize) % self.config.shard_count
            }
            AssignmentStrategy::AddressBased => {
                // Route based on address bytes
                let addr_value = u64::from_le_bytes([
                    address[0], address[1], address[2], address[3],
                    address[4], address[5], address[6], address[7],
                ]);
                (addr_value as usize) % self.config.shard_count
            }
        }
    }

    /// Route a transaction to determine target shard
    fn route_transaction(&self, tx: &Transaction) -> usize {
        self.get_shard_for_address(&tx.from)
    }

    /// Get all shards
    pub async fn get_all_shards(&self) -> Vec<Arc<RwLock<Shard>>> {
        self.shards.clone()
    }

    /// Get a specific shard
    pub fn get_shard(&self, shard_id: usize) -> Option<&Arc<RwLock<Shard>>> {
        self.shards.get(shard_id)
    }

    /// Get shard count
    pub fn shard_count(&self) -> usize {
        self.config.shard_count
    }

    /// Process cross-shard transaction
    /// 
    /// This handles the two-phase commit for cross-shard transactions
    pub async fn process_cross_shard_transaction(
        &self,
        tx_hash: Hash,
    ) -> crate::error::BlockchainResult<()> {
        let cross_tx = {
            let cross_txs = self.cross_shard_txs.read().await;
            cross_txs.get(&tx_hash).cloned()
        };
        
        if let Some(mut cross_tx) = cross_tx {
            // Phase 1: Validate on source shard
            {
                let shard = self.shards[cross_tx.source_shard].read().await;
                let blockchain = shard.blockchain.read().await;
                let balance = blockchain.get_balance(cross_tx.tx.from);
                let total_cost = cross_tx.tx.value.saturating_add(cross_tx.tx.fee);
                
                if balance < total_cost {
                    cross_tx.status = CrossShardStatus::Failed;
                    let mut cross_txs = self.cross_shard_txs.write().await;
                    cross_txs.insert(tx_hash, cross_tx);
                    return Err(crate::error::BlockchainError::InvalidTransaction(
                        "Insufficient balance for cross-shard transaction".to_string()
                    ));
                }
            }
            
            // Phase 2: Execute on target shard
            {
                let shard = self.shards[cross_tx.target_shard].write().await;
                let mut blockchain = shard.blockchain.write().await;
                
                // Deduct from sender (on source shard, handled separately)
                // Add to receiver (on target shard)
                if cross_tx.tx.to != [0u8; 20] {
                    let current_balance = blockchain.get_balance(cross_tx.tx.to);
                    blockchain.set_balance(cross_tx.tx.to, current_balance + cross_tx.tx.value)?;
                }
            }
            
            // Mark as committed
            cross_tx.status = CrossShardStatus::Committed;
            let mut cross_txs = self.cross_shard_txs.write().await;
            cross_txs.insert(tx_hash, cross_tx);
        }
        
        Ok(())
    }

    /// Get transactions for a shard (for mining)
    pub async fn get_shard_transactions(&self, shard_id: usize, limit: usize) -> Vec<Transaction> {
        if let Some(shard) = self.shards.get(shard_id) {
            let shard = shard.read().await;
            shard.get_transactions(limit)
        } else {
            Vec::new()
        }
    }

    /// Remove transactions from a shard (after mining)
    pub async fn remove_shard_transactions(&self, shard_id: usize, count: usize) -> Vec<Transaction> {
        if let Some(shard) = self.shards.get(shard_id) {
            let mut shard = shard.write().await;
            shard.remove_transactions(count)
        } else {
            Vec::new()
        }
    }

    /// Get cross-shard transaction status
    pub async fn get_cross_shard_status(&self, tx_hash: Hash) -> Option<CrossShardStatus> {
        let cross_txs = self.cross_shard_txs.read().await;
        cross_txs.get(&tx_hash).map(|tx| tx.status.clone())
    }

    /// Synchronize shard state
    /// 
    /// Merges state from all shards into a unified view
    pub async fn synchronize_shards(&self) -> crate::error::BlockchainResult<()> {
        // For now, this is a placeholder
        // In production, this would:
        // 1. Collect state from all shards
        // 2. Resolve conflicts
        // 3. Merge balances
        // 4. Update cross-shard transaction statuses
        
        Ok(())
    }

    /// Get shard statistics
    pub async fn get_shard_stats(&self, shard_id: usize) -> Option<ShardStats> {
        if let Some(shard) = self.shards.get(shard_id) {
            let shard = shard.read().await;
            let blockchain = shard.blockchain.read().await;
            
            Some(ShardStats {
                shard_id,
                block_count: blockchain.get_blocks().len(),
                transaction_pool_size: shard.transaction_pool.len(),
                cross_shard_outgoing: shard.cross_shard_outgoing.len(),
                cross_shard_incoming: shard.cross_shard_incoming.len(),
            })
        } else {
            None
        }
    }

    /// Get all shard statistics
    pub async fn get_all_shard_stats(&self) -> Vec<ShardStats> {
        let mut stats = Vec::new();
        for i in 0..self.config.shard_count {
            if let Some(stat) = self.get_shard_stats(i).await {
                stats.push(stat);
            }
        }
        stats
    }
    
    /// Get cross-shard transaction details
    pub async fn get_cross_shard_transaction(&self, tx_hash: Hash) -> Option<CrossShardTransaction> {
        let cross_txs = self.cross_shard_txs.read().await;
        cross_txs.get(&tx_hash).cloned()
    }
    
    /// Get all cross-shard transactions
    pub async fn get_all_cross_shard_transactions(&self) -> Vec<CrossShardTransaction> {
        let cross_txs = self.cross_shard_txs.read().await;
        cross_txs.values().cloned().collect()
    }
    
    /// Check if a transaction is cross-shard and get shard IDs
    pub async fn get_transaction_shards(&self, tx: &Transaction) -> Option<(usize, usize)> {
        let from_shard = self.get_shard_for_address(&tx.from);
        let to_shard = if tx.to != [0u8; 20] {
            self.get_shard_for_address(&tx.to)
        } else {
            from_shard
        };
        
        Some((from_shard, to_shard))
    }
}

/// Shard statistics
#[derive(Debug, Clone)]
pub struct ShardStats {
    pub shard_id: usize,
    pub block_count: usize,
    pub transaction_pool_size: usize,
    pub cross_shard_outgoing: usize,
    pub cross_shard_incoming: usize,
}

