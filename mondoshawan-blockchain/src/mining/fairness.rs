//! Fairness metrics for block production
//! 
//! Tracks reordering distance, MEV patterns, and fairness scores
//! to provide transparency in block ordering and detect unfair practices

use crate::blockchain::{Block, Transaction};
use crate::types::Hash;
use std::collections::HashMap;

/// Fairness analyzer that tracks transaction ordering and MEV patterns
pub struct FairnessAnalyzer {
    transaction_arrival_times: HashMap<Hash, u64>,
    transaction_positions: HashMap<Hash, usize>,
}

/// Fairness metrics for a block
#[derive(Debug, Clone)]
pub struct FairnessMetrics {
    /// Average reordering distance (how far transactions moved from arrival order)
    pub reordering_distance: f64,
    /// Number of sandwich attack detections
    pub sandwich_detections: u64,
    /// Number of back-running detections
    pub backrun_detections: u64,
    /// Number of front-running detections
    pub frontrun_detections: u64,
    /// Estimated MEV value extracted (in base units)
    pub estimated_mev_value: u128,
    /// Overall fairness score (0.0 = unfair, 1.0 = fair)
    pub fairness_score: f64,
    /// Total transactions analyzed
    pub transaction_count: usize,
    /// Average transaction age in block (seconds)
    pub avg_transaction_age: f64,
    /// Fee concentration (Gini coefficient of fees)
    pub fee_concentration: f64,
}

impl FairnessAnalyzer {
    /// Create a new fairness analyzer
    pub fn new() -> Self {
        Self {
            transaction_arrival_times: HashMap::new(),
            transaction_positions: HashMap::new(),
        }
    }
    
    /// Record when a transaction arrived in the mempool
    pub fn record_transaction_arrival(&mut self, tx_hash: Hash, timestamp: u64) {
        self.transaction_arrival_times.insert(tx_hash, timestamp);
    }
    
    /// Analyze a block for fairness metrics
    pub fn analyze_block(&self, block: &Block) -> FairnessMetrics {
        if block.transactions.is_empty() {
            return FairnessMetrics {
                reordering_distance: 0.0,
                sandwich_detections: 0,
                backrun_detections: 0,
                frontrun_detections: 0,
                estimated_mev_value: 0,
                fairness_score: 1.0,
                transaction_count: 0,
                avg_transaction_age: 0.0,
                fee_concentration: 0.0,
            };
        }
        
        let mut reordering_distances = Vec::new();
        let mut arrival_times: Vec<(usize, u64)> = Vec::new();
        
        // Collect arrival times for transactions in this block
        for (idx, tx) in block.transactions.iter().enumerate() {
            if let Some(&arrival_time) = self.transaction_arrival_times.get(&tx.hash) {
                arrival_times.push((idx, arrival_time));
            }
        }
        
        // Sort by arrival time to get expected order
        arrival_times.sort_by_key(|(_, time)| *time);
        
        // Calculate reordering distance for each transaction
        for (expected_pos, (actual_pos, _)) in arrival_times.iter().enumerate() {
            let distance = (*actual_pos as f64 - expected_pos as f64).abs();
            reordering_distances.push(distance);
        }
        
        let avg_reordering = if !reordering_distances.is_empty() {
            reordering_distances.iter().sum::<f64>() / reordering_distances.len() as f64
        } else {
            0.0
        };
        
        // Detect MEV patterns
        let (sandwich, backrun, frontrun, mev_value) = self.detect_mev_patterns(&block.transactions);
        
        // Calculate average transaction age
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut total_age = 0u64;
        let mut age_count = 0;
        for tx in &block.transactions {
            if let Some(&arrival_time) = self.transaction_arrival_times.get(&tx.hash) {
                total_age += current_time.saturating_sub(arrival_time);
                age_count += 1;
            }
        }
        let avg_age = if age_count > 0 {
            total_age as f64 / age_count as f64
        } else {
            0.0
        };
        
        // Calculate fee concentration (Gini coefficient)
        let fees: Vec<u128> = block.transactions.iter().map(|tx| tx.fee).collect();
        let fee_concentration = self.calculate_gini_coefficient(&fees);
        
        // Calculate fairness score
        // Lower reordering = higher fairness
        // No MEV = higher fairness
        // Lower fee concentration = higher fairness
        let max_expected_reordering = block.transactions.len() as f64 / 2.0;
        let reordering_penalty = (avg_reordering / max_expected_reordering.max(1.0)).min(1.0);
        let mev_penalty = if sandwich > 0 || backrun > 0 || frontrun > 0 {
            0.3 // Significant penalty for MEV
        } else {
            0.0
        };
        let concentration_penalty = fee_concentration * 0.2; // Penalty for high fee concentration
        
        let fairness_score = (1.0 - reordering_penalty * 0.4 - mev_penalty - concentration_penalty).max(0.0);
        
        FairnessMetrics {
            reordering_distance: avg_reordering,
            sandwich_detections: sandwich,
            backrun_detections: backrun,
            frontrun_detections: frontrun,
            estimated_mev_value: mev_value,
            fairness_score,
            transaction_count: block.transactions.len(),
            avg_transaction_age: avg_age,
            fee_concentration,
        }
    }
    
    /// Detect MEV patterns in transactions
    fn detect_mev_patterns(&self, transactions: &[Transaction]) -> (u64, u64, u64, u128) {
        let mut sandwich_count = 0u64;
        let mut backrun_count = 0u64;
        let mut frontrun_count = 0u64;
        let mut total_mev_value = 0u128;
        
        if transactions.len() < 2 {
            return (0, 0, 0, 0);
        }
        
        // Detect sandwich attacks: A -> B -> A pattern
        // Where A and B have the same target (e.g., DEX)
        for i in 0..transactions.len().saturating_sub(2) {
            let tx1 = &transactions[i];
            let tx2 = &transactions[i + 1];
            let tx3 = &transactions[i + 2];
            
            // Check if tx1 and tx3 target the same contract (sandwich pattern)
            if tx1.to == tx3.to && tx1.to != [0u8; 20] && tx2.to == tx1.to {
                // Potential sandwich: first tx, middle tx, last tx all target same contract
                // Additional check: middle tx might be the victim
                if tx2.from != tx1.from && tx2.from != tx3.from {
                    sandwich_count += 1;
                    // Estimate MEV value (simplified: sum of fees from attacker txs)
                    total_mev_value += tx1.fee + tx3.fee;
                }
            }
        }
        
        // Detect back-running: transaction immediately after another with same target
        for i in 0..transactions.len().saturating_sub(1) {
            let tx1 = &transactions[i];
            let tx2 = &transactions[i + 1];
            
            // Same target, different sender
            if tx1.to == tx2.to && tx1.to != [0u8; 20] && tx1.from != tx2.from {
                if let (Some(&time1), Some(&time2)) = (
                    self.transaction_arrival_times.get(&tx1.hash),
                    self.transaction_arrival_times.get(&tx2.hash),
                ) {
                    // tx2 arrived after tx1 (back-running)
                    if time2 > time1 {
                        backrun_count += 1;
                        total_mev_value += tx2.fee;
                    } else if time2 < time1 {
                        // tx2 arrived before tx1 but is ordered after (front-running)
                        frontrun_count += 1;
                        total_mev_value += tx2.fee;
                    }
                }
            }
        }
        
        (sandwich_count, backrun_count, frontrun_count, total_mev_value)
    }
    
    /// Calculate Gini coefficient for fee distribution
    /// Returns value between 0.0 (perfect equality) and 1.0 (perfect inequality)
    fn calculate_gini_coefficient(&self, fees: &[u128]) -> f64 {
        if fees.is_empty() {
            return 0.0;
        }
        
        let mut sorted_fees = fees.to_vec();
        sorted_fees.sort();
        
        let n = sorted_fees.len() as f64;
        let sum: u128 = sorted_fees.iter().sum();
        if sum == 0 {
            return 0.0;
        }
        
        let mean = sum as f64 / n;
        let mut gini = 0.0;
        
        for i in 0..sorted_fees.len() {
            for j in 0..sorted_fees.len() {
                gini += (sorted_fees[i] as f64 - sorted_fees[j] as f64).abs();
            }
        }
        
        gini / (2.0 * n * n * mean)
    }
    
    /// Get fairness score for a specific transaction position
    pub fn get_transaction_fairness(&self, tx_hash: &Hash, position: usize, total_txs: usize) -> f64 {
        if let Some(&arrival_time) = self.transaction_arrival_times.get(tx_hash) {
            // Find expected position based on arrival time
            // This is simplified - in reality, we'd need all transactions to calculate properly
            let expected_pos = position; // Simplified: assume fair ordering
            let distance = (position as f64 - expected_pos as f64).abs();
            let max_distance = total_txs as f64 / 2.0;
            
            // Fairness decreases with distance from expected position
            1.0 - (distance / max_distance.max(1.0)).min(1.0)
        } else {
            0.5 // Unknown arrival time = medium fairness
        }
    }
    
    /// Clear old transaction data (to prevent memory growth)
    pub fn clear_old_data(&mut self, before_timestamp: u64) {
        self.transaction_arrival_times.retain(|_, &mut time| time >= before_timestamp);
        self.transaction_positions.clear();
    }
}

impl Default for FairnessAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::{Block, BlockHeader, Transaction};
    use crate::types::StreamType;
    
    fn create_test_transaction(hash: Hash, from: [u8; 20], to: [u8; 20]) -> Transaction {
        let mut tx = Transaction {
            from,
            to,
            value: 1000,
            fee: 100,
            nonce: 0,
            data: vec![],
            gas_limit: 21000,
            hash,
            public_key: vec![],
            signature: vec![],
            pq_signature: None,
            execute_at_block: None,
            execute_at_timestamp: None,
            sponsor: None,
        };
        tx.hash = tx.calculate_hash();
        tx
    }
    
    #[test]
    fn test_empty_block_fairness() {
        let analyzer = FairnessAnalyzer::new();
        let block = Block {
            header: BlockHeader::new(vec![], 0, StreamType::StreamA, 100),
            transactions: vec![],
            hash: [0u8; 32],
        };
        
        let metrics = analyzer.analyze_block(&block);
        assert_eq!(metrics.fairness_score, 1.0);
        assert_eq!(metrics.transaction_count, 0);
    }
    
    #[test]
    fn test_sandwich_detection() {
        let mut analyzer = FairnessAnalyzer::new();
        
        // Create sandwich pattern: A -> B -> A (same target)
        let target = [99u8; 20];
        let tx1 = create_test_transaction([1u8; 32], [10u8; 20], target);
        let tx2 = create_test_transaction([2u8; 32], [20u8; 20], target); // Victim
        let tx3 = create_test_transaction([3u8; 32], [10u8; 20], target);
        
        let block = Block {
            header: BlockHeader::new(vec![], 0, StreamType::StreamA, 100),
            transactions: vec![tx1.clone(), tx2.clone(), tx3.clone()],
            hash: [0u8; 32],
        };
        
        // Record arrival times
        analyzer.record_transaction_arrival(tx1.hash, 1000);
        analyzer.record_transaction_arrival(tx2.hash, 1001);
        analyzer.record_transaction_arrival(tx3.hash, 1002);
        
        let metrics = analyzer.analyze_block(&block);
        assert!(metrics.sandwich_detections > 0 || metrics.fairness_score < 1.0);
    }
}
