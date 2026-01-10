//! Blockchain core implementation
//!
//! Copyright (c) 2026 Mondoshawan Protocol
//! Licensed under the MIT License (see LICENSE file)

pub mod block;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_quick_wins;
pub use block::{Block, BlockHeader, Transaction, TransactionSignature, PublicKey};

/// Maximum block size in bytes (10MB)
pub const MAX_BLOCK_SIZE: usize = 10 * 1024 * 1024;

/// Maximum number of parent hashes in a block (DoS protection)
pub const MAX_PARENT_HASHES: usize = 10;

/// Maximum transaction data size in bytes (128KB)
pub const MAX_TX_DATA_SIZE: usize = 128 * 1024;

use crate::types::Address;
use crate::storage::Database;
use crate::consensus::GhostDAG;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// Main blockchain structure
pub struct Blockchain {
    // Storage (optional - None means in-memory only)
    database: Option<Arc<Database>>,
    
    // GhostDAG consensus engine
    ghostdag: GhostDAG,
    
    // In-memory caches for fast access
    blocks: Vec<Block>,
    balances: HashMap<Address, u128>,
    nonces: HashMap<Address, u64>, // Track nonces for each address
    block_hashes: HashSet<crate::types::Hash>, // Fast lookup for block existence
    
    // Verkle tree for stateless mode
    verkle_state: Option<crate::verkle::VerkleState>,
    
    pub evm_enabled: bool,
    pub evm_executor: Option<crate::evm::EvmTransactionExecutor>,
    /// Parallel EVM executor for parallel transaction execution
    parallel_evm_executor: Option<Arc<tokio::sync::RwLock<crate::evm::parallel::ParallelEvmExecutor>>>,
    
    // Account Abstraction: Wallet registry
    wallet_registry: Option<Arc<tokio::sync::RwLock<crate::account_abstraction::WalletRegistry>>>,
    
    // Oracle Network
    oracle_registry: Option<Arc<tokio::sync::RwLock<crate::oracles::OracleRegistry>>>,
    price_feed_manager: Option<Arc<tokio::sync::RwLock<crate::oracles::PriceFeedManager>>>,
    vrf_manager: Option<Arc<tokio::sync::RwLock<crate::oracles::VrfManager>>>,
    oracle_staking: Option<Arc<tokio::sync::RwLock<crate::oracles::OracleStaking>>>,
    
    // Recurring Transactions
    recurring_manager: Option<Arc<tokio::sync::RwLock<crate::recurring::RecurringTransactionManager>>>,
    
    // Stop-Loss
    stop_loss_manager: Option<Arc<tokio::sync::RwLock<crate::stop_loss::StopLossManager>>>,
    // Privacy Layer
    privacy_manager: Option<Arc<tokio::sync::RwLock<crate::privacy::PrivacyManager>>>,
}

impl Blockchain {
    /// Create new blockchain without storage (in-memory only)
    pub fn new() -> Self {
        Self {
            database: None,
            ghostdag: GhostDAG::new(),
            blocks: Vec::new(),
            balances: HashMap::new(),
            nonces: HashMap::new(),
            block_hashes: HashSet::new(),
            verkle_state: None,
            evm_enabled: false,
            evm_executor: None,
            parallel_evm_executor: None,
            wallet_registry: None,
            oracle_registry: None,
            price_feed_manager: None,
            vrf_manager: None,
            oracle_staking: None,
            recurring_manager: None,
            stop_loss_manager: None,
            privacy_manager: None,
        }
    }
    
    /// Create new blockchain with Verkle tree (stateless mode)
    pub fn with_verkle() -> Self {
        Self {
            database: None,
            ghostdag: GhostDAG::new(),
            blocks: Vec::new(),
            balances: HashMap::new(),
            nonces: HashMap::new(),
            block_hashes: HashSet::new(),
            verkle_state: Some(crate::verkle::VerkleState::new()),
            evm_enabled: false,
            evm_executor: None,
            parallel_evm_executor: None,
            wallet_registry: None,
            oracle_registry: None,
            price_feed_manager: None,
            vrf_manager: None,
            oracle_staking: None,
            recurring_manager: None,
            stop_loss_manager: None,
            privacy_manager: None,
        }
    }

    /// Create new blockchain with storage
    pub fn with_storage(database: Arc<Database>) -> crate::error::BlockchainResult<Self> {
        let mut bc = Self {
            database: Some(database),
            ghostdag: GhostDAG::new(),
            blocks: Vec::new(),
            balances: HashMap::new(),
            nonces: HashMap::new(),
            block_hashes: HashSet::new(),
            verkle_state: None,
            evm_enabled: false,
            evm_executor: None,
            parallel_evm_executor: None,
            wallet_registry: None,
            oracle_registry: None,
            price_feed_manager: None,
            vrf_manager: None,
            oracle_staking: None,
            recurring_manager: None,
            stop_loss_manager: None,
            privacy_manager: None,
        };
        
        // Load existing blocks and state from storage
        bc.load_from_storage()?;
        
        Ok(bc)
    }
    
    /// Create new blockchain with storage and Verkle tree
    pub fn with_storage_and_verkle(database: Arc<Database>) -> crate::error::BlockchainResult<Self> {
        let mut bc = Self {
            database: Some(database),
            ghostdag: GhostDAG::new(),
            blocks: Vec::new(),
            balances: HashMap::new(),
            nonces: HashMap::new(),
            block_hashes: HashSet::new(),
            verkle_state: Some(crate::verkle::VerkleState::new()),
            evm_enabled: false,
            evm_executor: None,
            parallel_evm_executor: None,
            wallet_registry: None,
            oracle_registry: None,
            price_feed_manager: None,
            vrf_manager: None,
            oracle_staking: None,
            recurring_manager: None,
            stop_loss_manager: None,
            privacy_manager: None,
        };
        
        // Load existing blocks and state from storage
        bc.load_from_storage()?;
        
        Ok(bc)
    }

    pub fn with_evm(enable: bool) -> Self {
        let mut bc = Self::new();
        bc.evm_enabled = enable;
        if enable {
            bc.evm_executor = Some(crate::evm::EvmTransactionExecutor::new());
        }
        bc
    }
    
    /// Load blocks and state from storage
    fn load_from_storage(&mut self) -> crate::error::BlockchainResult<()> {
        // For now, we'll load blocks and state on-demand when queried
        // In production, you'd maintain an index of block hashes and load state eagerly
        // This keeps startup fast while still providing persistence
        
        Ok(())
    }

    /// Add a block to the blockchain with full validation and transaction processing
    pub fn add_block(&mut self, block: Block) -> crate::error::BlockchainResult<()> {
        // 1. Validate block structure
        self.validate_block_structure(&block)?;
        
        // 2. Validate block hash
        let calculated_hash = block.calculate_hash();
        if block.hash != calculated_hash {
            return Err(crate::error::BlockchainError::InvalidBlock(
                "Invalid block hash".to_string()
            ));
        }
        
        // 3. Check for duplicate block
        if self.block_hashes.contains(&block.hash) {
            return Err(crate::error::BlockchainError::InvalidBlock(
                "Block already exists".to_string()
            ));
        }
        
        // 4. Validate parent hashes (for DAG support)
        self.validate_parent_hashes(&block)?;
        
        // 5. Validate and process transactions
        self.validate_and_process_transactions(&block)?;
        
        // 6. Persist block to storage
        if let Some(db) = &self.database {
            use crate::storage::BlockStore;
            let block_store = BlockStore::new(db);
            block_store.put(&block)?;
        }
        
        // 7. Add block to GhostDAG for consensus ordering
        self.ghostdag.add_block(block.clone());
        
        // 8. Add block to chain
        self.block_hashes.insert(block.hash);
        self.blocks.push(block);
        
        Ok(())
    }

    /// Validate block structure (number, timestamp, etc.)
    fn validate_block_structure(&self, block: &Block) -> crate::error::BlockchainResult<()> {
        // Check block size (DoS protection)
        let block_size = bincode::serialize(block)
            .map_err(|e| crate::error::BlockchainError::Serialization(e.to_string()))?
            .len();
        if block_size > MAX_BLOCK_SIZE {
            return Err(crate::error::BlockchainError::InvalidBlock(
                format!("Block size {} exceeds maximum {}", block_size, MAX_BLOCK_SIZE)
            ));
        }
        
        // Check parent hash count (DoS protection)
        if block.header.parent_hashes.len() > MAX_PARENT_HASHES {
            return Err(crate::error::BlockchainError::InvalidBlock(
                format!("Too many parent hashes: {} (max: {})", block.header.parent_hashes.len(), MAX_PARENT_HASHES)
            ));
        }
        
        // For genesis block (block_number 0), allow empty parent hashes
        if block.header.block_number == 0 {
            if !self.blocks.is_empty() {
                return Err(crate::error::BlockchainError::InvalidBlock(
                    "Genesis block must be first".to_string()
                ));
            }
            return Ok(());
        }
        
        // For non-genesis blocks, must have at least one parent
        if block.header.parent_hashes.is_empty() {
            return Err(crate::error::BlockchainError::InvalidBlock(
                "Non-genesis block must have at least one parent".to_string()
            ));
        }
        
        // Validate timestamp (must be in the future relative to parents, but not too far)
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Allow some clock skew (10 minutes)
        if block.header.timestamp > current_time + 600 {
            return Err(crate::error::BlockchainError::Validation(
                "Block timestamp too far in future".to_string()
            ));
        }
        
        // Check timestamp is reasonable (not before 2020)
        if block.header.timestamp < 1577836800 {
            return Err(crate::error::BlockchainError::Validation(
                "Block timestamp too old".to_string()
            ));
        }
        
        Ok(())
    }

    /// Validate parent hashes exist in the blockchain (DAG support)
    fn validate_parent_hashes(&self, block: &Block) -> crate::error::BlockchainResult<()> {
        if block.header.block_number == 0 {
            // Genesis block - no parents needed
            return Ok(());
        }
        
        // Check that at least one parent exists
        let mut found_parent = false;
        for parent_hash in &block.header.parent_hashes {
            if self.block_hashes.contains(parent_hash) {
                found_parent = true;
                break;
            }
        }
        
        if !found_parent {
            return Err(crate::error::BlockchainError::InvalidBlock(
                "No valid parent hash found".to_string()
            ));
        }
        
        Ok(())
    }

    /// Validate and process all transactions in the block
    fn validate_and_process_transactions(&mut self, block: &Block) -> crate::error::BlockchainResult<()> {
        // Note: Cross-shard transaction detection and processing is handled at the
        // shard manager level when transactions are added to shards. Here we just
        // process all transactions normally. The shard manager tracks cross-shard
        // transactions separately and handles the two-phase commit protocol.
        
        let current_block = block.header.block_number;
        let current_timestamp = block.header.timestamp;
        
        for tx in &block.transactions {
            // Check if time-locked transaction is ready to execute
            if !tx.is_ready_to_execute(current_block, current_timestamp) {
                return Err(crate::error::BlockchainError::InvalidTransaction(
                    format!(
                        "Time-locked transaction not ready: execute_at_block={:?}, execute_at_timestamp={:?}, current_block={}, current_timestamp={}",
                        tx.execute_at_block, tx.execute_at_timestamp, current_block, current_timestamp
                    )
                ));
            }
            
            // Validate transaction (pass current_block and current_timestamp for time-lock checks)
            self.validate_transaction(tx, current_block, current_timestamp)?;
            
            // Process transaction (update state)
            self.process_transaction(tx)?;
        }
        
        Ok(())
    }

    /// Validate a single transaction
    fn validate_transaction(&self, tx: &Transaction, current_block: u64, current_timestamp: u64) -> crate::error::BlockchainResult<()> {
        // For privacy transactions, validate zk-SNARK proof instead of signature
        if let Some(ref privacy_tx) = tx.privacy_data {
            return self.validate_privacy_transaction(tx, privacy_tx);
        }
        
        // For multi-signature transactions, validate multi-sig instead of single signature
        if let Some(ref multisig_sigs) = tx.multisig_signatures {
            // This is a multi-sig transaction - validate multi-sig
            if let Some(ref wallet_registry) = self.wallet_registry {
                if let Ok(registry) = wallet_registry.try_read() {
                    if let Some(wallet) = registry.get_wallet(&tx.from) {
                        if wallet.is_multisig() {
                            // Validate multi-sig transaction
                            match &wallet.wallet_type {
                                crate::account_abstraction::WalletType::MultiSig { signers, threshold } |
                                crate::account_abstraction::WalletType::Combined { signers, threshold, .. } => {
                                    // Check we have enough signatures
                                    if multisig_sigs.len() < *threshold as usize {
                                        return Err(crate::error::BlockchainError::InvalidTransaction(
                                            format!("Insufficient signatures: need {}, have {}", threshold, multisig_sigs.len())
                                        ));
                                    }
                                    
                                    // Check all signers are in expected list
                                    let signed_by: Vec<Address> = multisig_sigs.iter().map(|(addr, _, _)| *addr).collect();
                                    let signers_set: HashSet<Address> = signers.iter().copied().collect();
                                    for signer in &signed_by {
                                        if !signers_set.contains(signer) {
                                            return Err(crate::error::BlockchainError::InvalidTransaction(
                                                format!("Unknown signer: {}", hex::encode(*signer))
                                            ));
                                        }
                                    }
                                    
                                    // Check for duplicate signers
                                    use std::collections::HashSet;
                                    let mut seen = HashSet::new();
                                    for signer in &signed_by {
                                        if seen.contains(signer) {
                                            return Err(crate::error::BlockchainError::InvalidTransaction(
                                                format!("Duplicate signer: {}", hex::encode(*signer))
                                            ));
                                        }
                                        seen.insert(*signer);
                                    }
                                    
                                    // TODO: Verify cryptographic signatures
                                    // For now, we do structural validation only
                                }
                                _ => {
                                    return Err(crate::error::BlockchainError::InvalidTransaction(
                                        "Multi-sig signatures provided but wallet is not multi-sig".to_string()
                                    ));
                                }
                            }
                        } else {
                            return Err(crate::error::BlockchainError::InvalidTransaction(
                                "Multi-sig signatures provided but wallet is not multi-sig".to_string()
                            ));
                        }
                    } else {
                        return Err(crate::error::BlockchainError::InvalidTransaction(
                            "Multi-sig transaction from non-contract wallet".to_string()
                        ));
                    }
                }
            }
        } else {
            // Regular transaction - verify single signature
            if !tx.verify_signature() {
                return Err(crate::error::BlockchainError::InvalidTransaction(
                    "Invalid transaction signature".to_string()
                ));
            }
        }
        
        // Check transaction data size (DoS protection)
        if tx.data.len() > MAX_TX_DATA_SIZE {
            return Err(crate::error::BlockchainError::InvalidTransaction(
                format!("Transaction data size {} exceeds maximum {}", tx.data.len(), MAX_TX_DATA_SIZE)
            ));
        }
        
        // Check transaction hash
        let calculated_hash = tx.calculate_hash();
        if tx.hash != calculated_hash {
            return Err(crate::error::BlockchainError::InvalidTransaction(
                "Invalid transaction hash".to_string()
            ));
        }
        
        // Check nonce (must be exactly equal to current nonce for strict sequential ordering)
        // For contract wallets, use wallet nonce; for EOA, use account nonce
        let current_nonce = if let Some(ref wallet_registry) = self.wallet_registry {
            // Check if sender is a contract wallet
            // Note: Using try_read() for non-blocking access in sync context
            // In production, this would be handled differently (async validation or sync registry)
            if let Ok(registry) = wallet_registry.try_read() {
                if let Some(wallet) = registry.get_wallet(&tx.from) {
                    wallet.get_nonce() // Use wallet nonce
                } else {
                    self.get_nonce(tx.from) // Use account nonce for EOA
                }
            } else {
                // If we can't acquire the lock, fall back to account nonce
                // This is a temporary solution - in production, validation should be async
                self.get_nonce(tx.from)
            }
        } else {
            self.get_nonce(tx.from) // Fallback to account nonce if no registry
        };
        
        if tx.nonce != current_nonce {
            return Err(crate::error::BlockchainError::InvalidTransaction(
                format!("Invalid nonce: expected {}, got {}", current_nonce, tx.nonce)
            ));
        }
        
        // For contract wallets, check spending limits if applicable
        if let Some(ref wallet_registry) = self.wallet_registry {
            if let Ok(registry) = wallet_registry.try_read() {
                if let Some(wallet) = registry.get_wallet(&tx.from) {
                    if wallet.has_spending_limits() {
                        // Check spending limits
                        if let Some(ref limits) = wallet.config.spending_limits {
                            // Clone limits to allow mutation
                            let mut limits_check = limits.clone();
                            if let Err(e) = limits_check.check_limit(tx.value) {
                                return Err(crate::error::BlockchainError::InvalidTransaction(
                                    format!("Spending limit exceeded: {}", e)
                                ));
                            }
                        }
                    }
                }
            }
        }
        
        // Check balance: For gasless transactions, sponsor pays fee; sender pays value
        // For regular transactions, sender pays both value and fee
        let sender_balance = self.get_balance(tx.from);
        let sender_required = tx.value; // Sender always pays the value
        
        if sender_balance < sender_required {
            return Err(crate::error::BlockchainError::InvalidTransaction(
                format!(
                    "Insufficient balance: have {}, need {} (value)",
                    sender_balance, sender_required
                )
            ));
        }
        
        // Check sponsor balance for gasless transactions
        if let Some(sponsor) = tx.sponsor {
            let sponsor_balance = self.get_balance(sponsor);
            if sponsor_balance < tx.fee {
                return Err(crate::error::BlockchainError::InvalidTransaction(
                    format!(
                        "Insufficient sponsor balance: sponsor has {}, needs {} (fee)",
                        sponsor_balance, tx.fee
                    )
                ));
            }
        } else {
            // Regular transaction: sender pays both value and fee
            let total_required = tx.value.saturating_add(tx.fee);
            if sender_balance < total_required {
                return Err(crate::error::BlockchainError::InvalidTransaction(
                    format!(
                        "Insufficient balance: have {}, need {} (value: {} + fee: {})",
                        sender_balance, total_required, tx.value, tx.fee
                    )
                ));
            }
        }
        
        // Validate gas limit (must be reasonable)
        if tx.gas_limit == 0 {
            return Err(crate::error::BlockchainError::Validation(
                "Gas limit cannot be zero".to_string()
            ));
        }
        
        // For EVM transactions, validate data
        if self.evm_enabled && !tx.data.is_empty() {
            if tx.gas_limit < 21_000 {
                return Err(crate::error::BlockchainError::Validation(
                    "Gas limit too low for contract interaction".to_string()
                ));
            }
        }
        
        Ok(())
    }

    /// Validate privacy transaction (zk-SNARK proof)
    fn validate_privacy_transaction(
        &self,
        tx: &Transaction,
        privacy_tx: &crate::privacy::PrivacyTransaction,
    ) -> crate::error::BlockchainResult<()> {
        // Check if privacy manager is available
        let privacy_manager = self.privacy_manager.as_ref()
            .ok_or_else(|| crate::error::BlockchainError::InvalidTransaction(
                "Privacy manager not available".to_string()
            ))?;

        // Verify zk-SNARK proof (would use verifier here)
        // For now, basic validation
        if privacy_tx.proof.is_empty() {
            return Err(crate::error::BlockchainError::InvalidTransaction(
                "Privacy transaction missing proof".to_string()
            ));
        }

        // Check nullifier (prevent double-spending)
        // Extract nullifier from public inputs
        // In production, would properly parse public inputs
        // For now, simplified check
        
        // TODO: Deserialize proof and verify using PrivacyVerifier
        // TODO: Check nullifier in nullifier set
        
        Ok(())
    }

    /// Process a transaction and update state
    fn process_transaction(&mut self, tx: &Transaction) -> crate::error::BlockchainResult<()> {
        // Handle privacy transactions differently
        if let Some(ref privacy_tx) = tx.privacy_data {
            return self.process_privacy_transaction(tx, privacy_tx);
        }
        
        // Handle gasless transactions: sponsor pays fee, sender pays value
        // Handle regular transactions: sender pays both value and fee
        
        let from_balance = self.get_balance(tx.from);
        
        // Deduct value from sender (always)
        if from_balance < tx.value {
            return Err(crate::error::BlockchainError::InvalidTransaction(
                "Insufficient balance for transaction value".to_string()
            ));
        }
        
        let new_from_balance = from_balance - tx.value;
        
        // Deduct fee from sponsor (if gasless) or sender (if regular)
        if let Some(sponsor) = tx.sponsor {
            // Gasless transaction: sponsor pays fee
            let sponsor_balance = self.get_balance(sponsor);
            if sponsor_balance < tx.fee {
                return Err(crate::error::BlockchainError::InvalidTransaction(
                    "Insufficient sponsor balance for transaction fee".to_string()
                ));
            }
            
            let new_sponsor_balance = sponsor_balance - tx.fee;
            
            // Update sponsor balance
            if let Some(ref mut verkle) = self.verkle_state {
                verkle.set_balance(sponsor, new_sponsor_balance);
            } else {
                *self.balances.entry(sponsor).or_insert(0) = new_sponsor_balance;
            }
            
            // Persist sponsor balance change
            if let Some(db) = &self.database {
                use crate::storage::StateStore;
                let state_store = StateStore::new(db);
                state_store.put_balance(&sponsor, new_sponsor_balance)?;
            }
            
            // For gasless transactions, only value was deducted from sender (fee paid by sponsor)
            // Update sender balance (value deducted, fee handled by sponsor)
            if let Some(ref mut verkle) = self.verkle_state {
                verkle.set_balance(tx.from, new_from_balance);
            } else {
                *self.balances.entry(tx.from).or_insert(0) = new_from_balance;
            }
            
            // Persist sender balance change
            if let Some(db) = &self.database {
                use crate::storage::StateStore;
                let state_store = StateStore::new(db);
                state_store.put_balance(&tx.from, new_from_balance)?;
            }
        } else {
            // Regular transaction: sender also pays fee
            if new_from_balance < tx.fee {
                return Err(crate::error::BlockchainError::InvalidTransaction(
                    "Insufficient balance for transaction fee".to_string()
                ));
            }
            
            let new_from_balance_after_fee = new_from_balance - tx.fee;
        
            // Update sender balance (value + fee deducted)
            if let Some(ref mut verkle) = self.verkle_state {
                verkle.set_balance(tx.from, new_from_balance_after_fee);
            } else {
                *self.balances.entry(tx.from).or_insert(0) = new_from_balance_after_fee;
            }
            
            // Persist sender balance change
            if let Some(db) = &self.database {
                use crate::storage::StateStore;
                let state_store = StateStore::new(db);
                state_store.put_balance(&tx.from, new_from_balance_after_fee)?;
            }
        }
        
        // Add value to receiver (if not zero address)
        if tx.to != [0u8; 20] {
            let new_to_balance = self.get_balance(tx.to) + tx.value;
            
            // Update Verkle tree if enabled (canonical source)
            if let Some(ref mut verkle) = self.verkle_state {
                verkle.set_balance(tx.to, new_to_balance);
                // Don't update in-memory cache when Verkle is enabled
            } else {
                // Verkle not enabled - update in-memory cache
                *self.balances.entry(tx.to).or_insert(0) = new_to_balance;
            }
            
            // Persist balance change
            if let Some(db) = &self.database {
                use crate::storage::StateStore;
                let state_store = StateStore::new(db);
                state_store.put_balance(&tx.to, new_to_balance)?;
            }
        }
        
        // Update nonce (transaction was already validated to have correct nonce)
        // For contract wallets, update wallet nonce; for EOA, update account nonce
        if let Some(ref wallet_registry) = self.wallet_registry {
            if let Ok(mut registry) = wallet_registry.try_write() {
                if registry.is_contract_wallet(&tx.from) {
                    // Update wallet nonce
                    if let Err(e) = registry.update_wallet_nonce(&tx.from) {
                        return Err(crate::error::BlockchainError::InvalidTransaction(
                            format!("Failed to update wallet nonce: {}", e)
                        ));
                    }
                    
                    // Update spending limits if applicable
                    if let Some(wallet) = registry.get_wallet_mut(&tx.from) {
                        if wallet.has_spending_limits() {
                            if let Some(ref mut limits) = wallet.config.spending_limits {
                                limits.record_spending(tx.value);
                            }
                        }
                    }
                } else {
                    // Regular EOA: increment account nonce
                    let current_nonce = self.get_nonce(tx.from);
                    let new_nonce = current_nonce + 1;
                    
                    // Update Verkle tree if enabled (canonical source)
                    if let Some(ref mut verkle) = self.verkle_state {
                        verkle.set_nonce(tx.from, new_nonce);
                    } else {
                        // Verkle not enabled - update in-memory cache
                        self.nonces.insert(tx.from, new_nonce);
                    }
                }
            } else {
                // If we can't acquire the lock, fall back to account nonce
                // This is a temporary solution - in production, processing should be async
                let current_nonce = self.get_nonce(tx.from);
                let new_nonce = current_nonce + 1;
                
                if let Some(ref mut verkle) = self.verkle_state {
                    verkle.set_nonce(tx.from, new_nonce);
                } else {
                    self.nonces.insert(tx.from, new_nonce);
                }
            }
        } else {
            // Fallback: increment account nonce
            let current_nonce = self.get_nonce(tx.from);
            let new_nonce = current_nonce + 1;
            
            // Update Verkle tree if enabled (canonical source)
            if let Some(ref mut verkle) = self.verkle_state {
                verkle.set_nonce(tx.from, new_nonce);
            } else {
                // Verkle not enabled - update in-memory cache
                self.nonces.insert(tx.from, new_nonce);
            }
        }
        
        // Persist nonce change (only for EOA accounts, wallet nonces are in registry)
        // Note: For contract wallets, nonce is stored in wallet_registry, not in database
        if let Some(db) = &self.database {
            // Only persist if it's not a contract wallet
            if let Some(ref wallet_registry) = self.wallet_registry {
                if let Ok(registry) = wallet_registry.try_read() {
                    if !registry.is_contract_wallet(&tx.from) {
                        let current_nonce = self.get_nonce(tx.from);
                        let new_nonce = current_nonce + 1;
                        use crate::storage::StateStore;
                        let state_store = StateStore::new(db);
                        state_store.put_nonce(&tx.from, new_nonce)?;
                    }
                }
            } else {
                // No registry, persist as normal
                let current_nonce = self.get_nonce(tx.from);
                let new_nonce = current_nonce + 1;
                use crate::storage::StateStore;
                let state_store = StateStore::new(db);
                state_store.put_nonce(&tx.from, new_nonce)?;
            }
        }
        
        // Process EVM transaction if enabled and has data
        if self.evm_enabled && !tx.data.is_empty() {
            if let Some(executor) = &self.evm_executor {
                // Execute EVM transaction
                let block_number = self.latest_block_number();
                let block_timestamp = if let Some(latest_block) = self.blocks.last() {
                    latest_block.header.timestamp
                } else {
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                };
                
                match executor.execute_transaction(tx, block_number, block_timestamp) {
                    Ok(result) => {
                        if !result.success {
                            return Err(crate::error::BlockchainError::Evm(
                                format!("EVM execution failed: {:?}", result.output)
                            ));
                        }
                        // Gas used is already accounted for in fee
                    }
                    Err(e) => {
                        // If it's not an EVM transaction, that's okay
                        // Otherwise, return error
                        if !e.contains("Not an EVM transaction") {
                            return Err(crate::error::BlockchainError::Evm(
                                format!("EVM execution error: {}", e)
                            ));
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Process privacy transaction
    fn process_privacy_transaction(
        &mut self,
        _tx: &Transaction,
        _privacy_tx: &crate::privacy::PrivacyTransaction,
    ) -> crate::error::BlockchainResult<()> {
        // Privacy transactions hide sender, receiver, and amount
        // We only process the nullifier (mark as spent) and commitment (add to tree)
        
        if let Some(ref privacy_manager) = self.privacy_manager {
            // Extract nullifier from public inputs
            // In production, would properly parse public inputs
            // For now, simplified processing
            
            // TODO: Parse nullifier from public inputs
            // TODO: Add nullifier to nullifier set (mark as spent)
            // TODO: Add commitment to Merkle tree
            
            // Privacy transactions don't update balances directly
            // Balances are managed through commitments and nullifiers
        }
        
        Ok(())
    }

    /// Get the latest block number
    pub fn latest_block_number(&self) -> u64 {
        self.blocks.iter()
            .map(|b| b.header.block_number)
            .max()
            .unwrap_or(0)
    }

    /// Get block by hash
    pub fn get_block_by_hash(&self, hash: &crate::types::Hash) -> Option<Block> {
        // First check in-memory cache
        if let Some(block) = self.blocks.iter().find(|b| b.hash == *hash) {
            return Some(block.clone());
        }
        
        // If not in cache and we have storage, try loading from storage
        if let Some(db) = &self.database {
            use crate::storage::BlockStore;
            let block_store = BlockStore::new(db);
            if let Ok(Some(block)) = block_store.get(hash) {
                return Some(block);
            }
            // Ignore storage errors - just return None
        }
        
        None
    }

    /// Get block by number
    pub fn get_block_by_number(&self, number: u64) -> Option<&Block> {
        self.blocks.iter().find(|b| b.header.block_number == number)
    }

    /// Get all blocks
    pub fn get_blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.blocks.iter().map(|b| b.transactions.len()).sum()
    }

    pub fn set_balance(&mut self, address: Address, balance: u128) -> crate::error::BlockchainResult<()> {
        // If Verkle is enabled, it is the canonical source - update it first
        if let Some(ref mut verkle) = self.verkle_state {
            verkle.set_balance(address, balance);
            // Only update in-memory cache if Verkle is not enabled (for backward compatibility)
            // When Verkle is enabled, cache is read-only and populated from Verkle
        } else {
            // Verkle not enabled - use in-memory cache as source
            self.balances.insert(address, balance);
        }
        
        // Persist balance (for recovery and non-Verkle mode)
        if let Some(db) = &self.database {
            use crate::storage::StateStore;
            let state_store = StateStore::new(db);
            state_store.put_balance(&address, balance)?;
        }
        
        Ok(())
    }
    
    /// Set nonce for an address
    pub fn set_nonce(&mut self, address: Address, nonce: u64) -> crate::error::BlockchainResult<()> {
        // If Verkle is enabled, it is the canonical source - update it first
        if let Some(ref mut verkle) = self.verkle_state {
            verkle.set_nonce(address, nonce);
            // Only update in-memory cache if Verkle is not enabled
        } else {
            // Verkle not enabled - use in-memory cache as source
            self.nonces.insert(address, nonce);
        }
        
        // Persist nonce (for recovery and non-Verkle mode)
        if let Some(db) = &self.database {
            use crate::storage::StateStore;
            let state_store = StateStore::new(db);
            state_store.put_nonce(&address, nonce)?;
        }
        
        Ok(())
    }

    pub fn get_balance(&self, address: Address) -> u128 {
        // If Verkle is enabled, it is the canonical source of truth
        if let Some(ref verkle) = self.verkle_state {
            return verkle.get_balance(address);
        }
        
        // Fallback to in-memory cache
        if let Some(balance) = self.balances.get(&address) {
            return *balance;
        }
        
        // If not in cache and we have storage, try loading from storage
        if let Some(db) = &self.database {
            use crate::storage::StateStore;
            let state_store = StateStore::new(db);
            if let Ok(Some(balance)) = state_store.get_balance(&address) {
                return balance;
            }
            // Ignore storage errors - just return 0
        }
        
        0
    }
    
    /// Get nonce for an address
    pub fn get_nonce(&self, address: Address) -> u64 {
        // If Verkle is enabled, it is the canonical source of truth
        if let Some(ref verkle) = self.verkle_state {
            return verkle.get_nonce(address);
        }
        
        // Fallback to in-memory cache
        if let Some(nonce) = self.nonces.get(&address) {
            return *nonce;
        }
        
        // If not in cache and we have storage, try loading from storage
        if let Some(db) = &self.database {
            use crate::storage::StateStore;
            let state_store = StateStore::new(db);
            if let Ok(Some(nonce)) = state_store.get_nonce(&address) {
                return nonce;
            }
        }
        
        0
    }


    pub fn evm_executor(&self) -> Option<&crate::evm::EvmTransactionExecutor> {
        self.evm_executor.as_ref()
    }

    /// Get GhostDAG consensus engine
    pub fn ghostdag(&self) -> &GhostDAG {
        &self.ghostdag
    }

    /// Get blocks in consensus order (from GhostDAG)
    pub fn get_ordered_blocks(&self) -> Vec<&Block> {
        self.ghostdag.get_ordered_blocks()
    }

    /// Get DAG statistics
    pub fn get_dag_stats(&self) -> crate::consensus::DAGStats {
        self.ghostdag.get_stats()
    }

    /// Get transactions per second
    pub fn get_tps(&self, duration_seconds: u64) -> f64 {
        self.ghostdag.get_tps(duration_seconds)
    }

    /// Check if block is in blue set (consensus selected)
    pub fn is_blue_block(&self, hash: &crate::types::Hash) -> bool {
        self.ghostdag.is_blue(hash)
    }
    
    /// Get state root (Verkle tree root hash)
    pub fn state_root(&self) -> Option<crate::types::Hash> {
        self.verkle_state.as_ref().map(|v| v.state_root())
    }
    
    /// Get balance with proof (for light clients)
    pub fn get_balance_with_proof(&self, address: Address) -> Option<(u128, crate::verkle::StateProof)> {
        self.verkle_state.as_ref().and_then(|verkle| {
            let (balance, proof, root) = verkle.get_balance_with_proof(address);
            let mut value = Vec::with_capacity(24);
            value.extend_from_slice(&balance.to_le_bytes());
            value.extend_from_slice(&verkle.get_nonce(address).to_le_bytes());
            Some((balance, crate::verkle::StateProof::new(address, value, proof, root)))
        })
    }
    
    /// Get nonce with proof (for light clients)
    pub fn get_nonce_with_proof(&self, address: Address) -> Option<(u64, crate::verkle::StateProof)> {
        self.verkle_state.as_ref().and_then(|verkle| {
            let (nonce, proof, root) = verkle.get_nonce_with_proof(address);
            let mut value = Vec::with_capacity(24);
            value.extend_from_slice(&verkle.get_balance(address).to_le_bytes());
            value.extend_from_slice(&nonce.to_le_bytes());
            Some((nonce, crate::verkle::StateProof::new(address, value, proof, root)))
        })
    }
    
    /// Check if Verkle tree is enabled
    pub fn is_verkle_enabled(&self) -> bool {
        self.verkle_state.is_some()
    }
}

