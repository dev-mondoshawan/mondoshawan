//! Reputation system for addresses based on behavior and Node Longevity

use crate::types::Address;
use crate::governance::NodeRegistry;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Reputation score (0-100)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ReputationScore(f64);

impl ReputationScore {
    pub fn new(score: f64) -> Self {
        Self(score.min(100.0).max(0.0))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn is_high(&self) -> bool {
        self.0 >= 70.0
    }

    pub fn is_medium(&self) -> bool {
        self.0 >= 40.0 && self.0 < 70.0
    }

    pub fn is_low(&self) -> bool {
        self.0 < 40.0
    }
}

/// Reputation factors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationFactors {
    /// Number of successful transactions
    pub successful_txs: u64,
    /// Number of failed transactions
    pub failed_txs: u64,
    /// Number of blocks mined (if node operator)
    pub blocks_mined: u64,
    /// Node longevity score (if node operator)
    pub node_longevity: Option<f64>,
    /// Account age (days since first transaction)
    pub account_age_days: u64,
    /// Total value transacted
    pub total_value_transacted: u128,
    /// Number of unique addresses interacted with
    pub unique_contacts: u64,
    /// Number of suspicious activities detected
    pub suspicious_activities: u64,
}

/// Reputation manager
pub struct ReputationManager {
    /// Address reputation scores
    reputations: HashMap<Address, ReputationScore>,
    /// Reputation factors for each address
    factors: HashMap<Address, ReputationFactors>,
    /// Node registry (for node longevity integration)
    node_registry: Option<Arc<RwLock<NodeRegistry>>>,
}

impl ReputationManager {
    pub fn new() -> Self {
        Self {
            reputations: HashMap::new(),
            factors: HashMap::new(),
            node_registry: None,
        }
    }

    pub fn with_node_registry(node_registry: Arc<RwLock<NodeRegistry>>) -> Self {
        Self {
            reputations: HashMap::new(),
            factors: HashMap::new(),
            node_registry: Some(node_registry),
        }
    }

    /// Calculate reputation score for an address
    pub fn calculate_reputation(&mut self, address: &Address) -> ReputationScore {
        let factors = self.factors.entry(*address).or_insert_with(|| {
            ReputationFactors {
                successful_txs: 0,
                failed_txs: 0,
                blocks_mined: 0,
                node_longevity: None,
                account_age_days: 0,
                total_value_transacted: 0,
                unique_contacts: 0,
                suspicious_activities: 0,
            }
        });

        let mut score = 50.0; // Start at neutral (50)

        // Factor 1: Transaction success rate (0-20 points)
        let total_txs = factors.successful_txs + factors.failed_txs;
        if total_txs > 0 {
            let success_rate = factors.successful_txs as f64 / total_txs as f64;
            score += success_rate * 20.0;
        }

        // Factor 2: Node longevity (0-20 points) - if node operator
        if let Some(longevity) = factors.node_longevity {
            score += longevity * 20.0;
        }

        // Factor 3: Account age (0-15 points)
        if factors.account_age_days > 0 {
            let age_score = (factors.account_age_days.min(365) as f64 / 365.0) * 15.0;
            score += age_score;
        }

        // Factor 4: Blocks mined (0-15 points) - if miner
        if factors.blocks_mined > 0 {
            let blocks_score = (factors.blocks_mined.min(1000) as f64 / 1000.0) * 15.0;
            score += blocks_score;
        }

        // Factor 5: Network participation (0-10 points)
        if factors.unique_contacts > 0 {
            let contacts_score = (factors.unique_contacts.min(100) as f64 / 100.0) * 10.0;
            score += contacts_score;
        }

        // Penalty: Suspicious activities (-30 points max)
        if factors.suspicious_activities > 0 {
            let penalty = (factors.suspicious_activities.min(10) as f64 / 10.0) * 30.0;
            score -= penalty;
        }

        // Penalty: High failure rate (-20 points max)
        if total_txs > 10 {
            let failure_rate = factors.failed_txs as f64 / total_txs as f64;
            if failure_rate > 0.5 {
                score -= (failure_rate - 0.5) * 40.0; // Penalize if >50% failure rate
            }
        }

        let reputation = ReputationScore::new(score);
        self.reputations.insert(*address, reputation);
        reputation
    }

    /// Get reputation score for an address
    pub fn get_reputation(&mut self, address: &Address) -> ReputationScore {
        if let Some(&reputation) = self.reputations.get(address) {
            reputation
        } else {
            self.calculate_reputation(address)
        }
    }

    /// Record a successful transaction
    pub fn record_successful_tx(&mut self, address: &Address, value: u128, to: &Address) {
        let factors = self.factors.entry(*address).or_insert_with(|| {
            ReputationFactors {
                successful_txs: 0,
                failed_txs: 0,
                blocks_mined: 0,
                node_longevity: None,
                account_age_days: 0,
                total_value_transacted: 0,
                unique_contacts: 0,
                suspicious_activities: 0,
            }
        });

        factors.successful_txs += 1;
        factors.total_value_transacted = factors.total_value_transacted.saturating_add(value);
        
        // Track unique contacts (simplified - would need proper tracking)
        if *to != [0u8; 20] {
            factors.unique_contacts += 1;
        }

        // Recalculate reputation
        self.calculate_reputation(address);
    }

    /// Record a failed transaction
    pub fn record_failed_tx(&mut self, address: &Address) {
        let factors = self.factors.entry(*address).or_insert_with(|| {
            ReputationFactors {
                successful_txs: 0,
                failed_txs: 0,
                blocks_mined: 0,
                node_longevity: None,
                account_age_days: 0,
                total_value_transacted: 0,
                unique_contacts: 0,
                suspicious_activities: 0,
            }
        });

        factors.failed_txs += 1;

        // Recalculate reputation
        self.calculate_reputation(address);
    }

    /// Record suspicious activity
    pub fn record_suspicious_activity(&mut self, address: &Address) {
        let factors = self.factors.entry(*address).or_insert_with(|| {
            ReputationFactors {
                successful_txs: 0,
                failed_txs: 0,
                blocks_mined: 0,
                node_longevity: None,
                account_age_days: 0,
                total_value_transacted: 0,
                unique_contacts: 0,
                suspicious_activities: 0,
            }
        });

        factors.suspicious_activities += 1;

        // Recalculate reputation
        self.calculate_reputation(address);
    }

    /// Update node longevity for an address (if it's a node operator)
    /// Note: This requires a mapping from address to NodeIdentity, which would need to be
    /// maintained separately. For now, this is a placeholder for future implementation.
    pub async fn update_node_longevity(&mut self, address: &Address, node_identity: &crate::governance::node_identity::NodeIdentity) {
        if let Some(ref registry) = self.node_registry {
            let (longevity_weight, blocks_mined) = {
                let registry_read = registry.read().await;
                if let Some(node_stats) = registry_read.get_node_stats(node_identity) {
                    let longevity_weight = node_stats.calculate_weight(365); // Use 365 days as network age
                    let blocks_mined = node_stats.blocks_mined;
                    (Some(longevity_weight), blocks_mined)
                } else {
                    (None, 0)
                }
            };
            
            if let Some(longevity_weight) = longevity_weight {
                let factors = self.factors.entry(*address).or_insert_with(|| {
                    ReputationFactors {
                        successful_txs: 0,
                        failed_txs: 0,
                        blocks_mined: 0,
                        node_longevity: None,
                        account_age_days: 0,
                        total_value_transacted: 0,
                        unique_contacts: 0,
                        suspicious_activities: 0,
                    }
                });
                
                factors.node_longevity = Some(longevity_weight);
                factors.blocks_mined = blocks_mined;

                // Recalculate reputation
                self.calculate_reputation(address);
            }
        }
    }

    /// Get reputation factors for an address
    pub fn get_factors(&self, address: &Address) -> Option<&ReputationFactors> {
        self.factors.get(address)
    }
}

impl Default for ReputationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reputation_calculation() {
        let mut manager = ReputationManager::new();
        let address = [1u8; 20];

        // New address should start at neutral
        let reputation = manager.get_reputation(&address);
        assert!(reputation.value() >= 45.0 && reputation.value() <= 55.0);

        // Record successful transactions
        for _ in 0..10 {
            manager.record_successful_tx(&address, 1000, &[2u8; 20]);
        }

        let reputation = manager.get_reputation(&address);
        assert!(reputation.value() > 50.0); // Should be higher after successful txs
    }

    #[test]
    fn test_reputation_penalties() {
        let mut manager = ReputationManager::new();
        let address = [1u8; 20];

        // Record many failed transactions
        for _ in 0..20 {
            manager.record_failed_tx(&address);
        }

        let reputation = manager.get_reputation(&address);
        assert!(reputation.value() < 50.0); // Should be lower after failures

        // Record suspicious activity
        manager.record_suspicious_activity(&address);
        let reputation = manager.get_reputation(&address);
        assert!(reputation.value() < 50.0); // Should be even lower
    }
}
