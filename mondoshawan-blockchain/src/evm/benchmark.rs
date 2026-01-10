//! Performance Benchmarking for Parallel EVM
//!
//! Measures and compares performance of sequential vs parallel execution

use crate::blockchain::Transaction;
use crate::evm::parallel::{ParallelEvmExecutor, ParallelExecutionResult};
use crate::types::Address;
use std::time::{Duration, Instant};
use std::sync::Arc;

/// Benchmark results
#[derive(Debug, Clone)]
pub struct BenchmarkResults {
    /// Sequential execution time
    pub sequential_time: Duration,
    /// Parallel execution time
    pub parallel_time: Duration,
    /// Speedup factor (sequential_time / parallel_time)
    pub speedup: f64,
    /// Number of transactions
    pub transaction_count: usize,
    /// Number of parallel batches
    pub parallel_batches: usize,
    /// Transactions executed in parallel
    pub parallel_executed: usize,
}

impl BenchmarkResults {
    /// Calculate speedup
    pub fn calculate_speedup(&self) -> f64 {
        if self.parallel_time.as_nanos() > 0 {
            self.sequential_time.as_nanos() as f64 / self.parallel_time.as_nanos() as f64
        } else {
            1.0
        }
    }

    /// Format results as string
    pub fn format(&self) -> String {
        format!(
            "Benchmark Results:\n\
            - Transactions: {}\n\
            - Sequential Time: {:?}\n\
            - Parallel Time: {:?}\n\
            - Speedup: {:.2}x\n\
            - Parallel Batches: {}\n\
            - Parallel Executed: {}",
            self.transaction_count,
            self.sequential_time,
            self.parallel_time,
            self.speedup,
            self.parallel_batches,
            self.parallel_executed
        )
    }
}

/// Benchmark parallel EVM execution
pub struct ParallelEvmBenchmark {
    executor: ParallelEvmExecutor,
}

impl ParallelEvmBenchmark {
    /// Create a new benchmark instance
    pub fn new() -> Self {
        Self {
            executor: ParallelEvmExecutor::new(),
        }
    }

    /// Run benchmark comparing sequential vs parallel execution
    pub fn benchmark(
        &self,
        transactions: Vec<Transaction>,
        executor_fn: impl Fn(&Transaction) -> Result<crate::evm::ExecutionResult, String>,
    ) -> BenchmarkResults {
        // Sequential execution
        let sequential_start = Instant::now();
        let sequential_results: Vec<_> = transactions
            .iter()
            .map(|tx| executor_fn(tx))
            .collect();
        let sequential_time = sequential_start.elapsed();

        // Parallel execution
        let parallel_start = Instant::now();
        let parallel_results = self.executor
            .execute_parallel_sync(transactions.clone(), &executor_fn)
            .unwrap_or_default();
        let parallel_time = parallel_start.elapsed();

        // Calculate statistics
        let parallel_executed = parallel_results
            .iter()
            .filter(|r| r.executed_in_parallel)
            .count();

        // Estimate batch count (simplified)
        let parallel_batches = if parallel_results.is_empty() {
            0
        } else {
            // Group consecutive parallel executions
            let mut batches = 1;
            let mut last_parallel = false;
            for result in &parallel_results {
                if result.executed_in_parallel && !last_parallel {
                    batches += 1;
                }
                last_parallel = result.executed_in_parallel;
            }
            batches
        };

        let speedup = if parallel_time.as_nanos() > 0 {
            sequential_time.as_nanos() as f64 / parallel_time.as_nanos() as f64
        } else {
            1.0
        };

        BenchmarkResults {
            sequential_time,
            parallel_time,
            speedup,
            transaction_count: transactions.len(),
            parallel_batches,
            parallel_executed,
        }
    }

    /// Run benchmark with async executor
    pub async fn benchmark_async<F, Fut>(
        &self,
        transactions: Vec<Transaction>,
        executor: Arc<F>,
    ) -> BenchmarkResults
    where
        F: Fn(Transaction) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<crate::evm::ExecutionResult, String>> + Send,
    {
        // Sequential execution (simulated)
        let sequential_start = Instant::now();
        for tx in &transactions {
            let tx_clone = tx.clone();
            let _ = executor(tx_clone).await;
        }
        let sequential_time = sequential_start.elapsed();

        // Parallel execution
        let parallel_start = Instant::now();
        let parallel_results = self.executor
            .execute_parallel_async(transactions.clone(), executor.clone())
            .await
            .unwrap_or_default();
        let parallel_time = parallel_start.elapsed();

        // Calculate statistics
        let parallel_executed = parallel_results
            .iter()
            .filter(|r| r.executed_in_parallel)
            .count();

        let parallel_batches = if parallel_results.is_empty() {
            0
        } else {
            let mut batches = 1;
            let mut last_parallel = false;
            for result in &parallel_results {
                if result.executed_in_parallel && !last_parallel {
                    batches += 1;
                }
                last_parallel = result.executed_in_parallel;
            }
            batches
        };

        let speedup = if parallel_time.as_nanos() > 0 {
            sequential_time.as_nanos() as f64 / parallel_time.as_nanos() as f64
        } else {
            1.0
        };

        BenchmarkResults {
            sequential_time,
            parallel_time,
            speedup,
            transaction_count: transactions.len(),
            parallel_batches,
            parallel_executed,
        }
    }

    /// Generate test transactions for benchmarking
    pub fn generate_test_transactions(count: usize, independent: bool) -> Vec<Transaction> {
        if independent {
            // Generate independent transactions (different senders/recipients)
            (0..count)
                .map(|i| {
                    Transaction::with_data(
                        Address::from([i as u8; 20]),
                        Address::from([(i + 100) as u8; 20]),
                        1000,
                        100,
                        0,
                        Vec::new(),
                        21_000,
                    )
                })
                .collect()
        } else {
            // Generate dependent transactions (chain)
            (0..count)
                .map(|i| {
                    Transaction::with_data(
                        Address::from([i as u8; 20]),
                        Address::from([(i + 1) as u8; 20]),
                        1000,
                        100,
                        0,
                        Vec::new(),
                        21_000,
                    )
                })
                .collect()
        }
    }
}

impl Default for ParallelEvmBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_results() {
        let results = BenchmarkResults {
            sequential_time: Duration::from_millis(100),
            parallel_time: Duration::from_millis(50),
            speedup: 2.0,
            transaction_count: 10,
            parallel_batches: 2,
            parallel_executed: 8,
        };

        assert_eq!(results.calculate_speedup(), 2.0);
        assert!(!results.format().is_empty());
    }

    #[test]
    fn test_generate_test_transactions() {
        let transactions = ParallelEvmBenchmark::generate_test_transactions(5, true);
        assert_eq!(transactions.len(), 5);
        
        // Check they're independent (different senders)
        for i in 0..5 {
            assert_eq!(transactions[i].from[0], i as u8);
        }
    }
}
