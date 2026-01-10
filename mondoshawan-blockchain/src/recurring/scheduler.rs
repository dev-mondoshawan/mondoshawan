//! Recurring Transaction Scheduler
//!
//! Handles automatic execution of recurring transactions.

use crate::recurring::manager::{RecurringTransactionManager, RecurringTransaction};
use crate::blockchain::Transaction;
use crate::types::Address;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Scheduler for executing recurring transactions
pub struct RecurringScheduler {
    manager: Arc<RwLock<RecurringTransactionManager>>,
}

impl RecurringScheduler {
    pub fn new(manager: Arc<RwLock<RecurringTransactionManager>>) -> Self {
        Self { manager }
    }

    /// Check and execute due recurring transactions
    pub async fn process_due_transactions(&self, current_time: u64) -> Vec<Transaction> {
        let manager_read = self.manager.read().await;
        let ready = manager_read.get_ready_to_execute(current_time);
        
        let mut transactions = Vec::new();
        
        for recurring in ready {
            // Create transaction from recurring template
            let tx = Transaction::new(
                recurring.from,
                recurring.to,
                recurring.value,
                0, // nonce will be set by transaction pool
                0, // gas_price
                0, // gas_limit
            );
            
            transactions.push(tx);
        }
        
        transactions
    }

    /// Mark transaction as executed
    pub async fn mark_executed(&self, recurring_tx_id: &crate::types::Hash, tx_hash: crate::types::Hash, current_time: u64) -> Result<(), String> {
        let mut manager = self.manager.write().await;
        manager.mark_executed(recurring_tx_id, tx_hash, current_time)
    }

    /// Mark transaction execution as failed
    pub async fn mark_failed(&self, recurring_tx_id: &crate::types::Hash) -> Result<(), String> {
        let mut manager = self.manager.write().await;
        manager.mark_failed(recurring_tx_id)
    }
}
