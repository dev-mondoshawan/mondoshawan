//! State Snapshot System for Parallel EVM Execution
//!
//! Provides state snapshot and restore functionality to enable safe parallel execution
//! of transactions without conflicts.

use crate::blockchain::Transaction;
use crate::types::Address;
use std::collections::HashMap;

/// Snapshot of blockchain state at a point in time
#[derive(Debug, Clone)]
pub struct StateSnapshot {
    /// Account balances
    balances: HashMap<Address, u128>,
    /// Account nonces
    nonces: HashMap<Address, u64>,
    /// Block number at snapshot time
    block_number: u64,
    /// Timestamp at snapshot time
    timestamp: u64,
}

impl StateSnapshot {
    /// Create a new empty snapshot
    pub fn new(block_number: u64, timestamp: u64) -> Self {
        Self {
            balances: HashMap::new(),
            nonces: HashMap::new(),
            block_number,
            timestamp,
        }
    }

    /// Create a snapshot from current state
    pub fn from_state(
        balances: &HashMap<Address, u128>,
        nonces: &HashMap<Address, u64>,
        block_number: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            balances: balances.clone(),
            nonces: nonces.clone(),
            block_number,
            timestamp,
        }
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &Address) -> u128 {
        self.balances.get(address).copied().unwrap_or(0)
    }

    /// Get nonce for an address
    pub fn get_nonce(&self, address: &Address) -> u64 {
        self.nonces.get(address).copied().unwrap_or(0)
    }

    /// Set balance in snapshot
    pub fn set_balance(&mut self, address: Address, balance: u128) {
        self.balances.insert(address, balance);
    }

    /// Set nonce in snapshot
    pub fn set_nonce(&mut self, address: Address, nonce: u64) {
        self.nonces.insert(address, nonce);
    }

    /// Apply a transaction to this snapshot (mutates snapshot)
    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        // Check balance
        let from_balance = self.get_balance(&tx.from);
        let total_required = tx.value.saturating_add(tx.fee);
        
        if from_balance < total_required {
            return Err(format!(
                "Insufficient balance: have {}, need {}",
                from_balance, total_required
            ));
        }

        // Check nonce
        let current_nonce = self.get_nonce(&tx.from);
        if tx.nonce != current_nonce {
            return Err(format!(
                "Invalid nonce: expected {}, got {}",
                current_nonce, tx.nonce
            ));
        }

        // Deduct from sender
        let new_from_balance = from_balance - total_required;
        self.set_balance(tx.from, new_from_balance);
        self.set_nonce(tx.from, current_nonce + 1);

        // Add to recipient (if not zero address)
        if tx.to != [0u8; 20] {
            let new_to_balance = self.get_balance(&tx.to) + tx.value;
            self.set_balance(tx.to, new_to_balance);
        }

        Ok(())
    }

    /// Merge another snapshot into this one (for parallel execution results)
    pub fn merge(&mut self, other: &StateSnapshot) -> Result<(), String> {
        // Merge balances (take maximum to handle conflicts)
        for (address, balance) in &other.balances {
            let current = self.get_balance(address);
            self.set_balance(*address, current.max(*balance));
        }

        // Merge nonces (take maximum)
        for (address, nonce) in &other.nonces {
            let current = self.get_nonce(address);
            self.set_nonce(*address, current.max(*nonce));
        }

        Ok(())
    }

    /// Get all balances
    pub fn get_balances(&self) -> &HashMap<Address, u128> {
        &self.balances
    }

    /// Get all nonces
    pub fn get_nonces(&self) -> &HashMap<Address, u64> {
        &self.nonces
    }

    /// Get block number
    pub fn block_number(&self) -> u64 {
        self.block_number
    }

    /// Get timestamp
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
}

/// State snapshot manager for parallel execution
pub struct StateSnapshotManager {
    /// Base snapshot (before parallel execution)
    base_snapshot: Option<StateSnapshot>,
    /// Snapshots for each parallel execution branch
    branch_snapshots: HashMap<usize, StateSnapshot>,
}

impl StateSnapshotManager {
    /// Create a new snapshot manager
    pub fn new() -> Self {
        Self {
            base_snapshot: None,
            branch_snapshots: HashMap::new(),
        }
    }

    /// Create a base snapshot from current state
    pub fn create_base_snapshot(
        &mut self,
        balances: &HashMap<Address, u128>,
        nonces: &HashMap<Address, u64>,
        block_number: u64,
        timestamp: u64,
    ) {
        self.base_snapshot = Some(StateSnapshot::from_state(
            balances, nonces, block_number, timestamp,
        ));
    }

    /// Create a branch snapshot for parallel execution
    pub fn create_branch_snapshot(&mut self, branch_id: usize) -> Result<StateSnapshot, String> {
        let base = self.base_snapshot.as_ref()
            .ok_or("Base snapshot not created")?
            .clone();
        
        self.branch_snapshots.insert(branch_id, base.clone());
        Ok(base)
    }

    /// Get a branch snapshot
    pub fn get_branch_snapshot(&self, branch_id: usize) -> Option<&StateSnapshot> {
        self.branch_snapshots.get(&branch_id)
    }

    /// Get mutable branch snapshot
    pub fn get_branch_snapshot_mut(&mut self, branch_id: usize) -> Option<&mut StateSnapshot> {
        self.branch_snapshots.get_mut(&branch_id)
    }

    /// Merge all branch snapshots into base
    pub fn merge_all_branches(&mut self) -> Result<StateSnapshot, String> {
        let mut merged = self.base_snapshot.as_ref()
            .ok_or("Base snapshot not created")?
            .clone();

        for (_, branch) in &self.branch_snapshots {
            merged.merge(branch)?;
        }

        Ok(merged)
    }

    /// Clear all snapshots
    pub fn clear(&mut self) {
        self.base_snapshot = None;
        self.branch_snapshots.clear();
    }
}

impl Default for StateSnapshotManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_snapshot_creation() {
        let snapshot = StateSnapshot::new(100, 1000);
        assert_eq!(snapshot.block_number(), 100);
        assert_eq!(snapshot.timestamp(), 1000);
    }

    #[test]
    fn test_state_snapshot_balance_operations() {
        let mut snapshot = StateSnapshot::new(100, 1000);
        let address = Address::from([1; 20]);

        snapshot.set_balance(address, 1000);
        assert_eq!(snapshot.get_balance(&address), 1000);

        snapshot.set_balance(address, 2000);
        assert_eq!(snapshot.get_balance(&address), 2000);
    }

    #[test]
    fn test_state_snapshot_apply_transaction() {
        let mut snapshot = StateSnapshot::new(100, 1000);
        let from = Address::from([1; 20]);
        let to = Address::from([2; 20]);

        // Set initial balance
        snapshot.set_balance(from, 10000);
        snapshot.set_nonce(from, 0);

        // Create transaction
        let tx = Transaction::with_data(from, to, 1000, 100, 0, Vec::new(), 21_000);

        // Apply transaction
        assert!(snapshot.apply_transaction(&tx).is_ok());

        // Check balances
        assert_eq!(snapshot.get_balance(&from), 10000 - 1000 - 100); // value + fee
        assert_eq!(snapshot.get_balance(&to), 1000); // value received
        assert_eq!(snapshot.get_nonce(&from), 1); // nonce incremented
    }

    #[test]
    fn test_state_snapshot_insufficient_balance() {
        let mut snapshot = StateSnapshot::new(100, 1000);
        let from = Address::from([1; 20]);
        let to = Address::from([2; 20]);

        // Set insufficient balance
        snapshot.set_balance(from, 500);
        snapshot.set_nonce(from, 0);

        let tx = Transaction::with_data(from, to, 1000, 100, 0, Vec::new(), 21_000);

        // Should fail
        assert!(snapshot.apply_transaction(&tx).is_err());
    }

    #[test]
    fn test_state_snapshot_merge() {
        let mut snapshot1 = StateSnapshot::new(100, 1000);
        let mut snapshot2 = StateSnapshot::new(100, 1000);

        let address = Address::from([1; 20]);

        snapshot1.set_balance(address, 1000);
        snapshot2.set_balance(address, 2000);

        snapshot1.merge(&snapshot2).unwrap();
        assert_eq!(snapshot1.get_balance(&address), 2000); // Takes maximum
    }

    #[test]
    fn test_snapshot_manager() {
        let mut manager = StateSnapshotManager::new();
        let mut balances = HashMap::new();
        let mut nonces = HashMap::new();

        balances.insert(Address::from([1; 20]), 1000);
        nonces.insert(Address::from([1; 20]), 0);

        manager.create_base_snapshot(&balances, &nonces, 100, 1000);

        let branch1 = manager.create_branch_snapshot(1).unwrap();
        assert_eq!(branch1.get_balance(&Address::from([1; 20])), 1000);
    }
}
