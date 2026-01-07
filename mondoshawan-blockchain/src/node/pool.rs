//! Transaction pool

use crate::blockchain::Transaction;
use crate::types::Hash;
use std::collections::HashMap;

/// Transaction pool
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
}

