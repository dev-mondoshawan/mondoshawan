//! Recurring Transaction Manager
//!
//! Manages creation, cancellation, and tracking of recurring transactions.

use crate::types::{Address, Hash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha3::{Digest, Keccak256};

/// Schedule for recurring transactions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Schedule {
    Daily { hour: u8, minute: u8 }, // 0-23, 0-59
    Weekly { day_of_week: u8, hour: u8, minute: u8 }, // 0-6 (Sun-Sat)
    Monthly { day_of_month: u8, hour: u8, minute: u8 }, // 1-31
    Custom { interval_seconds: u64 }, // Every N seconds
}

/// Status of a recurring transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurringTxStatus {
    Active,
    Paused,
    Cancelled,
    Completed, // Reached max_executions or end_date
    Failed, // Too many failures
}

/// Represents a recurring transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringTransaction {
    pub recurring_tx_id: Hash,
    pub from: Address,
    pub to: Address,
    pub value: u128,
    pub schedule: Schedule,
    pub created_at: u64,
    pub start_date: u64,
    pub end_date: Option<u64>,
    pub next_execution: u64,
    pub max_executions: Option<u64>,
    pub execution_count: u64,
    pub status: RecurringTxStatus,
    pub last_execution: Option<u64>,
    pub last_execution_tx_hash: Option<Hash>,
    pub failure_count: u64,
}

impl RecurringTransaction {
    pub fn new(
        from: Address,
        to: Address,
        value: u128,
        schedule: Schedule,
        start_date: u64,
        end_date: Option<u64>,
        max_executions: Option<u64>,
        created_at: u64,
    ) -> Self {
        let recurring_tx_id = Self::calculate_id(from, to, value, &schedule, created_at);
        let next_execution = start_date;

        Self {
            recurring_tx_id,
            from,
            to,
            value,
            schedule,
            created_at,
            start_date,
            end_date,
            next_execution,
            max_executions,
            execution_count: 0,
            status: RecurringTxStatus::Active,
            last_execution: None,
            last_execution_tx_hash: None,
            failure_count: 0,
        }
    }

    /// Check if transaction should execute now
    pub fn should_execute(&self, current_time: u64) -> bool {
        if self.status != RecurringTxStatus::Active {
            return false;
        }

        if current_time < self.start_date {
            return false;
        }

        if let Some(end_date) = self.end_date {
            if current_time > end_date {
                return false;
            }
        }

        if let Some(max) = self.max_executions {
            if self.execution_count >= max {
                return false;
            }
        }

        current_time >= self.next_execution
    }

    /// Calculate next execution time based on schedule
    pub fn calculate_next_execution(&self, current_time: u64) -> u64 {
        match &self.schedule {
            Schedule::Daily { hour, minute } => {
                // Next day at same time
                current_time + 86400 // 24 hours in seconds
            }
            Schedule::Weekly { day_of_week: _, hour: _, minute: _ } => {
                // Next week at same time
                current_time + 604800 // 7 days in seconds
            }
            Schedule::Monthly { day_of_month: _, hour: _, minute: _ } => {
                // Next month at same time (approximate)
                current_time + 2592000 // 30 days in seconds
            }
            Schedule::Custom { interval_seconds } => {
                current_time + interval_seconds
            }
        }
    }

    /// Calculate recurring transaction ID
    fn calculate_id(
        from: Address,
        to: Address,
        value: u128,
        schedule: &Schedule,
        created_at: u64,
    ) -> Hash {
        let mut hasher = Keccak256::new();
        hasher.update(&from);
        hasher.update(&to);
        hasher.update(&value.to_le_bytes());
        hasher.update(&serde_json::to_string(schedule).unwrap().as_bytes());
        hasher.update(&created_at.to_le_bytes());
        hasher.finalize().into()
    }
}

/// Manages all recurring transactions
pub struct RecurringTransactionManager {
    transactions: HashMap<Hash, RecurringTransaction>,
}

impl RecurringTransactionManager {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }

    /// Create a new recurring transaction
    pub fn create_recurring(
        &mut self,
        from: Address,
        to: Address,
        value: u128,
        schedule: Schedule,
        start_date: u64,
        end_date: Option<u64>,
        max_executions: Option<u64>,
        created_at: u64,
    ) -> RecurringTransaction {
        let recurring = RecurringTransaction::new(
            from, to, value, schedule, start_date, end_date, max_executions, created_at,
        );

        let id = recurring.recurring_tx_id;
        self.transactions.insert(id, recurring.clone());
        recurring
    }

    /// Cancel a recurring transaction
    pub fn cancel(&mut self, recurring_tx_id: &Hash) -> Result<(), String> {
        let recurring = self.transactions.get_mut(recurring_tx_id)
            .ok_or("Recurring transaction not found")?;

        if recurring.status == RecurringTxStatus::Cancelled {
            return Err("Already cancelled".to_string());
        }

        recurring.status = RecurringTxStatus::Cancelled;
        Ok(())
    }

    /// Get recurring transaction
    pub fn get(&self, recurring_tx_id: &Hash) -> Option<&RecurringTransaction> {
        self.transactions.get(recurring_tx_id)
    }

    /// Get all recurring transactions for an address
    pub fn get_for_address(&self, address: &Address) -> Vec<&RecurringTransaction> {
        self.transactions
            .values()
            .filter(|tx| tx.from == *address || tx.to == *address)
            .collect()
    }

    /// Get transactions ready to execute
    pub fn get_ready_to_execute(&self, current_time: u64) -> Vec<&RecurringTransaction> {
        self.transactions
            .values()
            .filter(|tx| tx.should_execute(current_time))
            .collect()
    }

    /// Mark transaction as executed
    pub fn mark_executed(&mut self, recurring_tx_id: &Hash, tx_hash: Hash, current_time: u64) -> Result<(), String> {
        let recurring = self.transactions.get_mut(recurring_tx_id)
            .ok_or("Recurring transaction not found")?;

        recurring.execution_count += 1;
        recurring.last_execution = Some(current_time);
        recurring.last_execution_tx_hash = Some(tx_hash);
        recurring.next_execution = recurring.calculate_next_execution(current_time);

        // Check if completed
        if let Some(max) = recurring.max_executions {
            if recurring.execution_count >= max {
                recurring.status = RecurringTxStatus::Completed;
            }
        }

        if let Some(end_date) = recurring.end_date {
            if current_time >= end_date {
                recurring.status = RecurringTxStatus::Completed;
            }
        }

        Ok(())
    }

    /// Mark transaction execution as failed
    pub fn mark_failed(&mut self, recurring_tx_id: &Hash) -> Result<(), String> {
        let recurring = self.transactions.get_mut(recurring_tx_id)
            .ok_or("Recurring transaction not found")?;

        recurring.failure_count += 1;

        // Mark as failed if too many failures
        if recurring.failure_count >= 5 {
            recurring.status = RecurringTxStatus::Failed;
        }

        Ok(())
    }

    /// Pause a recurring transaction
    pub fn pause(&mut self, recurring_tx_id: &Hash) -> Result<(), String> {
        let recurring = self.transactions.get_mut(recurring_tx_id)
            .ok_or("Recurring transaction not found")?;

        if recurring.status != RecurringTxStatus::Active {
            return Err("Transaction is not active".to_string());
        }

        recurring.status = RecurringTxStatus::Paused;
        Ok(())
    }

    /// Resume a paused recurring transaction
    pub fn resume(&mut self, recurring_tx_id: &Hash) -> Result<(), String> {
        let recurring = self.transactions.get_mut(recurring_tx_id)
            .ok_or("Recurring transaction not found")?;

        if recurring.status != RecurringTxStatus::Paused {
            return Err("Transaction is not paused".to_string());
        }

        recurring.status = RecurringTxStatus::Active;
        Ok(())
    }
}

impl Default for RecurringTransactionManager {
    fn default() -> Self {
        Self::new()
    }
}
