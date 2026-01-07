//! Transaction Ordering Policies
//! 
//! Provides configurable ordering strategies for transaction inclusion in blocks
//! to support MEV-aware and fairness-oriented block production

use crate::blockchain::Transaction;
use crate::types::Hash;
use std::collections::HashMap;

/// Transaction ordering policy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderingPolicy {
    /// First-In-First-Out (arrival order) - most fair
    Fifo,
    /// Random ordering - prevents front-running
    Random,
    /// Fee-based ordering (highest fee first) - maximizes miner revenue
    FeeBased,
    /// Hybrid: FIFO with fee boost for high-value transactions
    Hybrid,
    /// Time-weighted fairness: prioritize older transactions but allow fee boosts
    TimeWeighted,
}

impl OrderingPolicy {
    /// Get default ordering policy
    pub fn default() -> Self {
        OrderingPolicy::Fifo
    }
    
    /// Get policy name
    pub fn name(&self) -> &'static str {
        match self {
            OrderingPolicy::Fifo => "FIFO",
            OrderingPolicy::Random => "Random",
            OrderingPolicy::FeeBased => "Fee-Based",
            OrderingPolicy::Hybrid => "Hybrid",
            OrderingPolicy::TimeWeighted => "Time-Weighted",
        }
    }
}

/// Transaction ordering context
pub struct OrderingContext {
    /// Arrival times for transactions (tx_hash -> timestamp)
    arrival_times: HashMap<Hash, u64>,
    /// Current timestamp
    current_time: u64,
}

impl OrderingContext {
    /// Create new ordering context
    pub fn new() -> Self {
        Self {
            arrival_times: HashMap::new(),
            current_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    /// Record transaction arrival
    pub fn record_arrival(&mut self, tx_hash: Hash, timestamp: u64) {
        self.arrival_times.insert(tx_hash, timestamp);
    }
    
    /// Get transaction age in seconds
    pub fn get_age(&self, tx_hash: &Hash) -> u64 {
        if let Some(&arrival) = self.arrival_times.get(tx_hash) {
            self.current_time.saturating_sub(arrival)
        } else {
            0
        }
    }
    
    /// Update current time
    pub fn update_time(&mut self) {
        self.current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// Order transactions according to the specified policy
pub fn order_transactions(
    transactions: Vec<Transaction>,
    policy: OrderingPolicy,
    context: &mut OrderingContext,
) -> Vec<Transaction> {
    match policy {
        OrderingPolicy::Fifo => {
            // Already in arrival order (FIFO)
            transactions
        }
        OrderingPolicy::Random => {
            use rand::Rng;
            let mut txs = transactions;
            // Fisher-Yates shuffle
            let mut rng = rand::thread_rng();
            for i in (1..txs.len()).rev() {
                let j = rng.gen_range(0..=i);
                txs.swap(i, j);
            }
            txs
        }
        OrderingPolicy::FeeBased => {
            let mut txs = transactions;
            // Sort by fee (descending), then by value (descending)
            txs.sort_by(|a, b| {
                match b.fee.cmp(&a.fee) {
                    std::cmp::Ordering::Equal => b.value.cmp(&a.value),
                    other => other,
                }
            });
            txs
        }
        OrderingPolicy::Hybrid => {
            let mut txs = transactions.to_vec();
            // Sort by: fee score (weighted) + age bonus
            // High fee transactions get priority, but old transactions get a boost
            
            // Pre-compute max_fee to avoid borrow issue
            let max_fee = txs.iter().map(|t| t.fee).max().unwrap_or(1) as f64;
            
            txs.sort_by(|a, b| {
                let age_a = context.get_age(&a.hash) as f64;
                let age_b = context.get_age(&b.hash) as f64;
                
                // Fee score (normalized to 0-1)
                let fee_score_a = (a.fee as f64 / max_fee) * 0.7;
                let fee_score_b = (b.fee as f64 / max_fee) * 0.7;
                
                // Age bonus (normalized, max age = 60 seconds)
                let age_bonus_a = (age_a.min(60.0) / 60.0) * 0.3;
                let age_bonus_b = (age_b.min(60.0) / 60.0) * 0.3;
                
                let score_a = fee_score_a + age_bonus_a;
                let score_b = fee_score_b + age_bonus_b;
                
                score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
            });
            txs
        }
        OrderingPolicy::TimeWeighted => {
            let mut txs = transactions;
            // Sort by: age (primary) + fee boost (secondary)
            // Older transactions get priority, but high fees can boost position
            txs.sort_by(|a, b| {
                let age_a = context.get_age(&a.hash);
                let age_b = context.get_age(&b.hash);
                
                match age_b.cmp(&age_a) {
                    // If ages are similar (within 5 seconds), use fee as tiebreaker
                    std::cmp::Ordering::Equal if age_a.abs_diff(age_b) <= 5 => {
                        b.fee.cmp(&a.fee)
                    }
                    other => other,
                }
            });
            txs
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::Transaction;
    use crate::types::Address;
    
    fn create_tx(hash: Hash, fee: u128, value: u128) -> Transaction {
        Transaction::new(
            [0u8; 20],
            [0u8; 20],
            value,
            fee,
            0,
        )
    }
    
    #[test]
    fn test_fifo_ordering() {
        let mut context = OrderingContext::new();
        let t1 = create_tx([1u8; 32], 100, 1000);
        let t2 = create_tx([2u8; 32], 200, 2000);
        let t3 = create_tx([3u8; 32], 50, 500);
        
        let txs = vec![t1.clone(), t2.clone(), t3.clone()];
        let ordered = order_transactions(txs, OrderingPolicy::Fifo, &mut context);
        
        // Should maintain original order
        assert_eq!(ordered[0].hash, t1.hash);
        assert_eq!(ordered[1].hash, t2.hash);
        assert_eq!(ordered[2].hash, t3.hash);
    }
    
    #[test]
    fn test_fee_based_ordering() {
        let mut context = OrderingContext::new();
        let t1 = create_tx([1u8; 32], 100, 1000);
        let t2 = create_tx([2u8; 32], 200, 2000);
        let t3 = create_tx([3u8; 32], 50, 500);
        
        let txs = vec![t1.clone(), t2.clone(), t3.clone()];
        let ordered = order_transactions(txs, OrderingPolicy::FeeBased, &mut context);
        
        // Should be ordered by fee (descending)
        assert_eq!(ordered[0].hash, t2.hash); // Highest fee
        assert_eq!(ordered[1].hash, t1.hash);
        assert_eq!(ordered[2].hash, t3.hash); // Lowest fee
    }
}
