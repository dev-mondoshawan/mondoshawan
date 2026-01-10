//! Oracle Registry
//!
//! Manages oracle node registration, reputation, and feed assignments.

use crate::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Types of data feeds an oracle can provide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedType {
    Price,      // Price feeds (crypto, stocks, commodities)
    Randomness, // Verifiable Random Function
    Custom,     // Custom data feeds
}

/// Represents an oracle node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleNode {
    pub address: Address,
    pub feed_types: Vec<FeedType>,
    pub stake_amount: u128,
    pub reputation_score: f64, // 0.0 - 100.0
    pub last_update: u64,
    pub accuracy_rate: f64, // 0.0 - 1.0
    pub total_reports: u64,
    pub accurate_reports: u64,
    pub registered_at: u64,
}

impl OracleNode {
    pub fn new(address: Address, feed_types: Vec<FeedType>, stake_amount: u128, registered_at: u64) -> Self {
        Self {
            address,
            feed_types,
            stake_amount,
            reputation_score: 50.0, // Start at neutral
            last_update: registered_at,
            accuracy_rate: 1.0, // Assume accurate until proven otherwise
            total_reports: 0,
            accurate_reports: 0,
            registered_at,
        }
    }

    /// Update accuracy based on report
    pub fn report_accuracy(&mut self, is_accurate: bool) {
        self.total_reports += 1;
        if is_accurate {
            self.accurate_reports += 1;
        }
        
        // Update accuracy rate
        if self.total_reports > 0 {
            self.accuracy_rate = self.accurate_reports as f64 / self.total_reports as f64;
        }
        
        // Update reputation score (0-100)
        // Higher accuracy = higher reputation
        self.reputation_score = (self.accuracy_rate * 100.0).min(100.0).max(0.0);
    }

    /// Check if oracle supports a feed type
    pub fn supports_feed_type(&self, feed_type: FeedType) -> bool {
        self.feed_types.contains(&feed_type)
    }
}

/// Manages all registered oracle nodes
pub struct OracleRegistry {
    nodes: HashMap<Address, OracleNode>,
    feed_assignments: HashMap<String, Vec<Address>>, // feed_id -> oracle addresses
    config: crate::oracles::OracleConfig,
}

impl OracleRegistry {
    pub fn new(config: crate::oracles::OracleConfig) -> Self {
        Self {
            nodes: HashMap::new(),
            feed_assignments: HashMap::new(),
            config,
        }
    }

    /// Register a new oracle node
    pub fn register_oracle(
        &mut self,
        address: Address,
        feed_types: Vec<FeedType>,
        stake_amount: u128,
        current_time: u64,
    ) -> Result<(), String> {
        // Check if already registered
        if self.nodes.contains_key(&address) {
            return Err("Oracle already registered".to_string());
        }

        // Check minimum stake
        if stake_amount < self.config.min_stake {
            return Err(format!(
                "Stake amount {} is below minimum {}",
                stake_amount, self.config.min_stake
            ));
        }

        // Create oracle node
        let oracle = OracleNode::new(address, feed_types, stake_amount, current_time);
        self.nodes.insert(address, oracle);

        Ok(())
    }

    /// Unregister an oracle node
    pub fn unregister_oracle(&mut self, address: &Address) -> Result<(), String> {
        if !self.nodes.contains_key(address) {
            return Err("Oracle not registered".to_string());
        }

        // Remove from all feed assignments
        for feed_oracles in self.feed_assignments.values_mut() {
            feed_oracles.retain(|&addr| addr != *address);
        }

        self.nodes.remove(address);
        Ok(())
    }

    /// Get oracle node information
    pub fn get_oracle(&self, address: &Address) -> Option<&OracleNode> {
        self.nodes.get(address)
    }

    /// Get all oracles supporting a feed type
    pub fn get_oracles_for_feed_type(&self, feed_type: FeedType) -> Vec<&OracleNode> {
        self.nodes
            .values()
            .filter(|oracle| oracle.supports_feed_type(feed_type))
            .collect()
    }

    /// Assign oracles to a feed
    pub fn assign_oracles_to_feed(&mut self, feed_id: String, oracle_addresses: Vec<Address>) -> Result<(), String> {
        // Verify all oracles are registered
        for addr in &oracle_addresses {
            if !self.nodes.contains_key(addr) {
                return Err(format!("Oracle {} not registered", hex::encode(addr)));
            }
        }

        // Check minimum oracle count
        if oracle_addresses.len() < self.config.min_oracles_per_feed {
            return Err(format!(
                "Need at least {} oracles, got {}",
                self.config.min_oracles_per_feed,
                oracle_addresses.len()
            ));
        }

        self.feed_assignments.insert(feed_id, oracle_addresses);
        Ok(())
    }

    /// Get oracles assigned to a feed
    pub fn get_feed_oracles(&self, feed_id: &str) -> Option<&Vec<Address>> {
        self.feed_assignments.get(feed_id)
    }

    /// Report oracle accuracy (for slashing/rewards)
    pub fn report_oracle_accuracy(&mut self, address: &Address, is_accurate: bool) -> Result<(), String> {
        let oracle = self.nodes.get_mut(address)
            .ok_or("Oracle not found")?;
        
        oracle.report_accuracy(is_accurate);
        Ok(())
    }

    /// Get all registered oracles
    pub fn get_all_oracles(&self) -> Vec<&OracleNode> {
        self.nodes.values().collect()
    }

    /// Get oracle count
    pub fn oracle_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for OracleRegistry {
    fn default() -> Self {
        Self::new(crate::oracles::OracleConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle_registration() {
        let mut registry = OracleRegistry::default();
        let address = Address::from([1; 20]);
        let feed_types = vec![FeedType::Price, FeedType::Randomness];
        let stake = 2_000_000_000_000_000_000; // 2 MSHW

        assert!(registry.register_oracle(address, feed_types.clone(), stake, 1000).is_ok());
        assert!(registry.get_oracle(&address).is_some());
        
        let oracle = registry.get_oracle(&address).unwrap();
        assert_eq!(oracle.feed_types, feed_types);
        assert_eq!(oracle.stake_amount, stake);
    }

    #[test]
    fn test_oracle_minimum_stake() {
        let mut registry = OracleRegistry::default();
        let address = Address::from([1; 20]);
        let stake = 500_000_000_000_000_000; // 0.5 MSHW (below minimum)

        assert!(registry.register_oracle(address, vec![FeedType::Price], stake, 1000).is_err());
    }

    #[test]
    fn test_oracle_accuracy_reporting() {
        let mut registry = OracleRegistry::default();
        let address = Address::from([1; 20]);
        let stake = 2_000_000_000_000_000_000;

        registry.register_oracle(address, vec![FeedType::Price], stake, 1000).unwrap();
        
        // Report accurate data
        registry.report_oracle_accuracy(&address, true).unwrap();
        registry.report_oracle_accuracy(&address, true).unwrap();
        
        let oracle = registry.get_oracle(&address).unwrap();
        assert_eq!(oracle.accuracy_rate, 1.0);
        assert_eq!(oracle.total_reports, 2);
        assert_eq!(oracle.accurate_reports, 2);
        
        // Report inaccurate data
        registry.report_oracle_accuracy(&address, false).unwrap();
        
        let oracle = registry.get_oracle(&address).unwrap();
        assert_eq!(oracle.accuracy_rate, 2.0 / 3.0);
        assert_eq!(oracle.total_reports, 3);
        assert_eq!(oracle.accurate_reports, 2);
    }
}
