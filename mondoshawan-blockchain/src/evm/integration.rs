//! Integration helpers for Parallel EVM with Blockchain
//!
//! Provides utilities to integrate parallel execution into blockchain transaction processing

use crate::blockchain::Transaction;
use crate::evm::parallel::ParallelEvmExecutor;
use crate::types::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Helper to process transactions in parallel within a block
pub struct ParallelTransactionProcessor {
    executor: Arc<RwLock<ParallelEvmExecutor>>,
}

impl ParallelTransactionProcessor {
    /// Create a new parallel transaction processor
    pub fn new(executor: Arc<RwLock<ParallelEvmExecutor>>) -> Self {
        Self { executor }
    }

    /// Estimate if parallel execution would help for a set of transactions
    /// 
    /// This is a helper that can be used before processing to determine
    /// if parallel execution would provide benefits.
    pub fn estimate_parallel_improvement(&self, transactions: &[Transaction]) -> f64 {
        // This would need async runtime, so for now return estimate
        if let Ok(executor) = self.executor.try_read() {
            executor.estimate_improvement(transactions)
        } else {
            1.0
        }
    }

    /// Get executor reference
    pub fn executor(&self) -> &Arc<RwLock<ParallelEvmExecutor>> {
        &self.executor
    }

    /// Estimate if parallel execution would help for a set of transactions
    pub fn estimate_improvement(&self, transactions: &[Transaction]) -> f64 {
        // This would need async runtime, so for now return estimate
        if let Ok(executor) = self.executor.try_read() {
            executor.estimate_improvement(transactions)
        } else {
            1.0
        }
    }
}

/// Helper to analyze transactions for parallel execution
/// 
/// This can be used to determine if transactions can be executed in parallel
/// before actual execution.
pub fn analyze_transactions_for_parallel(
    transactions: &[Transaction],
) -> Result<Vec<Vec<Hash>>, String> {
    use crate::evm::parallel::DependencyGraph;
    
    let mut graph = DependencyGraph::new(transactions.to_vec());
    graph.analyze()?;
    
    Ok(graph.group_parallel_batches())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_processor_creation() {
        let executor = Arc::new(RwLock::new(ParallelEvmExecutor::new()));
        let processor = ParallelTransactionProcessor::new(executor);
        // Processor created successfully
        assert!(true);
    }

    #[test]
    fn test_analyze_transactions_for_parallel() {
        let transactions = vec![
            crate::blockchain::Transaction::with_data(
                crate::types::Address::from([1; 20]),
                crate::types::Address::from([2; 20]),
                1000,
                100,
                0,
                Vec::new(),
                21_000,
            ),
            crate::blockchain::Transaction::with_data(
                crate::types::Address::from([3; 20]),
                crate::types::Address::from([4; 20]),
                2000,
                100,
                0,
                Vec::new(),
                21_000,
            ),
        ];

        let batches = analyze_transactions_for_parallel(&transactions).unwrap();
        assert!(!batches.is_empty());
    }
}
