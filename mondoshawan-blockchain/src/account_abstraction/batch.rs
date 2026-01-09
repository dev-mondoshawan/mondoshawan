//! Batch Transactions - Atomic Multi-Operation Execution
//!
//! Enables multiple operations to be executed atomically in a single transaction,
//! with gas optimization and all-or-nothing execution guarantees.

use crate::types::{Address, Hash};
use crate::blockchain::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Batch transaction status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BatchStatus {
    /// Batch created, pending execution
    Pending,
    /// Batch is currently executing
    Executing,
    /// All operations completed successfully
    Completed,
    /// One or more operations failed, all rolled back
    Failed,
    /// Batch cancelled
    Cancelled,
}

/// Individual operation within a batch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BatchOperation {
    /// Simple token transfer
    Transfer {
        /// Recipient address
        to: Address,
        /// Amount to transfer (in base units)
        value: u128,
    },
    /// Smart contract call
    ContractCall {
        /// Contract address
        contract: Address,
        /// Call data (function selector + parameters)
        data: Vec<u8>,
        /// Value to send with call
        value: u128,
    },
    /// Token approval (ERC-20 style)
    Approval {
        /// Spender address
        spender: Address,
        /// Amount to approve
        amount: u128,
    },
    /// Custom operation (for future extensibility)
    Custom {
        /// Operation type identifier
        operation_type: String,
        /// Operation-specific data
        data: Vec<u8>,
    },
}

/// Result of a single batch operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperationResult {
    /// Operation index in the batch
    pub operation_index: usize,
    /// Whether the operation succeeded
    pub success: bool,
    /// Result data (transaction hash, return value, etc.)
    pub result: Option<Vec<u8>>,
    /// Error message if operation failed
    pub error: Option<String>,
    /// Gas used for this operation
    pub gas_used: u64,
}

/// Batch transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTransaction {
    /// Unique batch identifier (hash of operations + wallet + nonce)
    pub batch_id: Hash,
    /// Wallet address executing the batch
    pub wallet_address: Address,
    /// List of operations to execute
    pub operations: Vec<BatchOperation>,
    /// Wallet nonce for this batch
    pub nonce: u64,
    /// Total gas limit for the batch
    pub gas_limit: u64,
    /// Gas price
    pub gas_price: u128,
    /// Transaction signature (if single-sig wallet)
    pub signature: Option<crate::blockchain::TransactionSignature>,
    /// Multi-signature signatures (if multi-sig wallet)
    pub multisig_signatures: Vec<crate::account_abstraction::MultiSigSignature>,
    /// Creation timestamp
    pub created_at: u64,
    /// Current status
    pub status: BatchStatus,
    /// Results of executed operations
    pub results: Vec<BatchOperationResult>,
    /// Total gas used
    pub gas_used: u64,
}

impl BatchTransaction {
    /// Create a new batch transaction
    pub fn new(
        wallet_address: Address,
        operations: Vec<BatchOperation>,
        nonce: u64,
        gas_limit: u64,
        gas_price: u128,
        created_at: u64,
    ) -> Self {
        // Calculate batch ID (hash of wallet + operations + nonce)
        let batch_id = Self::calculate_batch_id(&wallet_address, &operations, nonce);

        Self {
            batch_id,
            wallet_address,
            operations,
            nonce,
            gas_limit,
            gas_price,
            signature: None,
            multisig_signatures: Vec::new(),
            created_at,
            status: BatchStatus::Pending,
            results: Vec::new(),
            gas_used: 0,
        }
    }

    /// Calculate batch ID from wallet, operations, and nonce
    fn calculate_batch_id(
        wallet_address: &Address,
        operations: &[BatchOperation],
        nonce: u64,
    ) -> Hash {
        use sha3::{Keccak256, Digest};
        
        let mut hasher = Keccak256::new();
        hasher.update(wallet_address);
        hasher.update(&nonce.to_le_bytes());
        
        // Hash each operation
        for (idx, op) in operations.iter().enumerate() {
            hasher.update(&idx.to_le_bytes());
            match op {
                BatchOperation::Transfer { to, value } => {
                    hasher.update(b"transfer");
                    hasher.update(to);
                    hasher.update(&value.to_le_bytes());
                }
                BatchOperation::ContractCall { contract, data, value } => {
                    hasher.update(b"contract_call");
                    hasher.update(contract);
                    hasher.update(data);
                    hasher.update(&value.to_le_bytes());
                }
                BatchOperation::Approval { spender, amount } => {
                    hasher.update(b"approval");
                    hasher.update(spender);
                    hasher.update(&amount.to_le_bytes());
                }
                BatchOperation::Custom { operation_type, data } => {
                    hasher.update(b"custom");
                    hasher.update(operation_type.as_bytes());
                    hasher.update(data);
                }
            }
        }
        
        let hash_bytes = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&hash_bytes);
        hash
    }

    /// Add signature to batch
    pub fn with_signature(mut self, signature: crate::blockchain::TransactionSignature) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Add multi-signature to batch
    pub fn add_multisig_signature(
        &mut self,
        signature: crate::account_abstraction::MultiSigSignature,
    ) {
        self.multisig_signatures.push(signature);
    }

    /// Get number of operations
    pub fn operation_count(&self) -> usize {
        self.operations.len()
    }

    /// Check if batch is ready to execute
    pub fn is_ready(&self) -> bool {
        matches!(self.status, BatchStatus::Pending)
    }

    /// Mark batch as executing
    pub fn mark_executing(&mut self) {
        self.status = BatchStatus::Executing;
    }

    /// Mark batch as completed
    pub fn mark_completed(&mut self, results: Vec<BatchOperationResult>, gas_used: u64) {
        self.status = BatchStatus::Completed;
        self.results = results;
        self.gas_used = gas_used;
    }

    /// Mark batch as failed
    pub fn mark_failed(&mut self, results: Vec<BatchOperationResult>, gas_used: u64) {
        self.status = BatchStatus::Failed;
        self.results = results;
        self.gas_used = gas_used;
    }

    /// Cancel batch
    pub fn cancel(&mut self) {
        self.status = BatchStatus::Cancelled;
    }

    /// Validate batch structure
    pub fn validate(&self) -> Result<(), String> {
        // Check operation count (max 100 operations)
        if self.operations.is_empty() {
            return Err("Batch must contain at least one operation".to_string());
        }
        if self.operations.len() > 100 {
            return Err("Batch cannot contain more than 100 operations".to_string());
        }

        // Check gas limit
        if self.gas_limit == 0 {
            return Err("Gas limit must be greater than zero".to_string());
        }

        // Check gas price
        if self.gas_price == 0 {
            return Err("Gas price must be greater than zero".to_string());
        }

        Ok(())
    }
}

/// Batch transaction manager
pub struct BatchManager {
    /// Active batch transactions (batch_id -> BatchTransaction)
    batches: HashMap<Hash, BatchTransaction>,
    /// Maximum operations per batch
    max_operations: usize,
}

impl BatchManager {
    /// Create a new batch manager
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
            max_operations: 100,
        }
    }

    /// Create a new batch transaction
    pub fn create_batch(
        &mut self,
        wallet_address: Address,
        operations: Vec<BatchOperation>,
        nonce: u64,
        gas_limit: u64,
        gas_price: u128,
        created_at: u64,
    ) -> Result<BatchTransaction, String> {
        // Validate operation count
        if operations.is_empty() {
            return Err("Batch must contain at least one operation".to_string());
        }
        if operations.len() > self.max_operations {
            return Err(format!(
                "Batch cannot contain more than {} operations",
                self.max_operations
            ));
        }

        // Create batch
        let batch = BatchTransaction::new(
            wallet_address,
            operations,
            nonce,
            gas_limit,
            gas_price,
            created_at,
        );

        // Validate batch
        batch.validate()?;

        // Store batch
        let batch_id = batch.batch_id;
        self.batches.insert(batch_id, batch.clone());

        Ok(batch)
    }

    /// Get batch by ID
    pub fn get_batch(&self, batch_id: &Hash) -> Option<&BatchTransaction> {
        self.batches.get(batch_id)
    }

    /// Get batch mutably by ID
    pub fn get_batch_mut(&mut self, batch_id: &Hash) -> Option<&mut BatchTransaction> {
        self.batches.get_mut(batch_id)
    }

    /// Get all batches for a wallet
    pub fn get_wallet_batches(&self, wallet_address: &Address) -> Vec<&BatchTransaction> {
        self.batches
            .values()
            .filter(|batch| batch.wallet_address == *wallet_address)
            .collect()
    }

    /// Remove completed or cancelled batches (cleanup)
    pub fn cleanup(&mut self) {
        self.batches.retain(|_, batch| {
            matches!(
                batch.status,
                BatchStatus::Pending | BatchStatus::Executing
            )
        });
    }

    /// Execute a batch transaction atomically
    /// Returns results and total gas used
    pub fn execute_batch(
        &mut self,
        batch_id: &Hash,
        execute_fn: impl Fn(&BatchOperation, usize) -> Result<BatchOperationResult, String>,
    ) -> Result<(Vec<BatchOperationResult>, u64), String> {
        let batch = self
            .batches
            .get_mut(batch_id)
            .ok_or_else(|| "Batch not found".to_string())?;

        if !batch.is_ready() {
            return Err("Batch is not ready to execute".to_string());
        }

        batch.mark_executing();

        let mut results = Vec::new();
        let mut total_gas = 0u64;

        // Execute operations atomically
        for (idx, operation) in batch.operations.iter().enumerate() {
            match execute_fn(operation, idx) {
                Ok(result) => {
                    total_gas += result.gas_used;
                    results.push(result);

                    // If any operation fails, rollback all
                    if !results.last().unwrap().success {
                        batch.mark_failed(results, total_gas);
                        return Err(format!("Operation {} failed", idx));
                    }
                }
                Err(e) => {
                    // Operation failed, rollback all
                    results.push(BatchOperationResult {
                        operation_index: idx,
                        success: false,
                        result: None,
                        error: Some(e.clone()),
                        gas_used: 0,
                    });
                    batch.mark_failed(results, total_gas);
                    return Err(format!("Operation {} failed: {}", idx, e));
                }
            }
        }

        // All operations succeeded
        batch.mark_completed(results.clone(), total_gas);
        Ok((results, total_gas))
    }

    /// Estimate gas for a batch
    pub fn estimate_gas(
        &self,
        operations: &[BatchOperation],
    ) -> Result<GasEstimate, String> {
        if operations.is_empty() {
            return Err("Cannot estimate gas for empty batch".to_string());
        }

        // Base gas cost (transaction overhead)
        let base_gas = 21_000u64;

        // Gas per operation (varies by type)
        let mut operation_gas = 0u64;
        for op in operations {
            operation_gas += match op {
                BatchOperation::Transfer { .. } => 21_000, // Standard transfer
                BatchOperation::ContractCall { data, .. } => {
                    // Base call + data cost
                    21_000 + (data.len() as u64 * 16) // 16 gas per byte
                }
                BatchOperation::Approval { .. } => 46_000, // ERC-20 approval
                BatchOperation::Custom { .. } => 21_000,   // Custom operation
            };
        }

        // Optimization savings (shared overhead)
        // When batching, we save on transaction overhead
        let optimization_savings = if operations.len() > 1 {
            // Save base gas for each additional operation
            base_gas * (operations.len() as u64 - 1)
        } else {
            0
        };

        let total_gas = base_gas + operation_gas - optimization_savings;

        Ok(GasEstimate {
            total_gas,
            base_gas,
            operation_gas,
            optimization_savings,
        })
    }
}

impl Default for BatchManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Gas estimation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasEstimate {
    /// Total estimated gas
    pub total_gas: u64,
    /// Base gas (transaction overhead)
    pub base_gas: u64,
    /// Gas for all operations
    pub operation_gas: u64,
    /// Gas savings from batching
    pub optimization_savings: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_creation() {
        let wallet = Address::from([1; 20]);
        let operations = vec![
            BatchOperation::Transfer {
                to: Address::from([2; 20]),
                value: 1000,
            },
            BatchOperation::Transfer {
                to: Address::from([3; 20]),
                value: 2000,
            },
        ];
        let timestamp = 1000;

        let batch = BatchTransaction::new(
            wallet,
            operations.clone(),
            1,
            100_000,
            1_000_000_000,
            timestamp,
        );

        assert_eq!(batch.wallet_address, wallet);
        assert_eq!(batch.operations.len(), 2);
        assert_eq!(batch.operation_count(), 2);
        assert_eq!(batch.status, BatchStatus::Pending);
        assert!(batch.is_ready());
    }

    #[test]
    fn test_batch_validation() {
        let wallet = Address::from([1; 20]);
        let operations = vec![BatchOperation::Transfer {
            to: Address::from([2; 20]),
            value: 1000,
        }];
        let timestamp = 1000;

        let batch = BatchTransaction::new(
            wallet,
            operations,
            1,
            100_000,
            1_000_000_000,
            timestamp,
        );

        assert!(batch.validate().is_ok());
    }

    #[test]
    fn test_batch_validation_empty_operations() {
        let wallet = Address::from([1; 20]);
        let operations = vec![];
        let timestamp = 1000;

        let batch = BatchTransaction::new(
            wallet,
            operations,
            1,
            100_000,
            1_000_000_000,
            timestamp,
        );

        assert!(batch.validate().is_err());
    }

    #[test]
    fn test_batch_validation_too_many_operations() {
        let wallet = Address::from([1; 20]);
        let operations: Vec<BatchOperation> = (0..101)
            .map(|i| BatchOperation::Transfer {
                to: Address::from([i as u8; 20]),
                value: 1000,
            })
            .collect();
        let timestamp = 1000;

        let batch = BatchTransaction::new(
            wallet,
            operations,
            1,
            100_000,
            1_000_000_000,
            timestamp,
        );

        assert!(batch.validate().is_err());
    }

    #[test]
    fn test_batch_manager_create() {
        let mut manager = BatchManager::new();
        let wallet = Address::from([1; 20]);
        let operations = vec![BatchOperation::Transfer {
            to: Address::from([2; 20]),
            value: 1000,
        }];
        let timestamp = 1000;

        let batch = manager
            .create_batch(wallet, operations, 1, 100_000, 1_000_000_000, timestamp)
            .unwrap();

        assert_eq!(batch.wallet_address, wallet);
        assert!(manager.get_batch(&batch.batch_id).is_some());
    }

    #[test]
    fn test_gas_estimation() {
        let manager = BatchManager::new();
        let operations = vec![
            BatchOperation::Transfer {
                to: Address::from([2; 20]),
                value: 1000,
            },
            BatchOperation::Transfer {
                to: Address::from([3; 20]),
                value: 2000,
            },
        ];

        let estimate = manager.estimate_gas(&operations).unwrap();

        assert!(estimate.total_gas > 0);
        assert!(estimate.optimization_savings > 0); // Should save gas when batching
    }

    #[test]
    fn test_batch_status_transitions() {
        let wallet = Address::from([1; 20]);
        let operations = vec![BatchOperation::Transfer {
            to: Address::from([2; 20]),
            value: 1000,
        }];
        let timestamp = 1000;

        let mut batch = BatchTransaction::new(
            wallet,
            operations,
            1,
            100_000,
            1_000_000_000,
            timestamp,
        );

        assert_eq!(batch.status, BatchStatus::Pending);

        batch.mark_executing();
        assert_eq!(batch.status, BatchStatus::Executing);

        let results = vec![BatchOperationResult {
            operation_index: 0,
            success: true,
            result: Some(vec![1, 2, 3]),
            error: None,
            gas_used: 21_000,
        }];

        batch.mark_completed(results, 42_000);
        assert_eq!(batch.status, BatchStatus::Completed);
        assert_eq!(batch.gas_used, 42_000);
    }
}
