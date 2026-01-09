//! Transaction pool

use crate::blockchain::Transaction;
use crate::types::Hash;
use std::collections::HashMap;

/// Transaction pool with support for time-locked transactions
pub struct TransactionPool {
    transactions: HashMap<Hash, Transaction>,
    max_size: usize,
}

impl TransactionPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            transactions: HashMap::new(),
            max_size,
        }
    }

    pub fn add(&mut self, tx: Transaction) -> Result<(), String> {
        if self.transactions.len() >= self.max_size {
            return Err("Transaction pool is full".to_string());
        }
        self.transactions.insert(tx.hash, tx);
        Ok(())
    }

    pub fn get(&self, hash: &Hash) -> Option<&Transaction> {
        self.transactions.get(hash)
    }

    pub fn remove(&mut self, hash: &Hash) -> Option<Transaction> {
        self.transactions.remove(hash)
    }

    /// Get all transactions that are ready to execute (time-locks satisfied)
    pub fn get_ready_transactions(&self, current_block: u64, current_timestamp: u64) -> Vec<&Transaction> {
        self.transactions
            .values()
            .filter(|tx| tx.is_ready_to_execute(current_block, current_timestamp))
            .collect()
    }

    /// Remove transactions that are no longer valid (e.g., expired time-locks)
    pub fn cleanup_expired(&mut self, current_block: u64, current_timestamp: u64, max_future_blocks: u64) {
        let expired_hashes: Vec<Hash> = self.transactions
            .iter()
            .filter(|(_, tx)| {
                // Remove if time-lock is too far in the future (likely invalid)
                if let Some(execute_at_block) = tx.execute_at_block {
                    if execute_at_block > current_block + max_future_blocks {
                        return true;
                    }
                }
                if let Some(execute_at_timestamp) = tx.execute_at_timestamp {
                    // Remove if timestamp is more than 1 year in the future (likely invalid)
                    if execute_at_timestamp > current_timestamp + 31536000 {
                        return true;
                    }
                }
                false
            })
            .map(|(hash, _)| *hash)
            .collect();
        
        for hash in expired_hashes {
            self.transactions.remove(&hash);
        }
    }

    /// Get all transactions (for mining)
    pub fn get_all(&self) -> Vec<&Transaction> {
        self.transactions.values().collect()
    }

    /// Get count of transactions
    pub fn len(&self) -> usize {
        self.transactions.len()
    }

    /// Check if pool is empty
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }
}

