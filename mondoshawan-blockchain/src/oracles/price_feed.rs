//! Price Feed Manager
//!
//! Manages price feeds with aggregation from multiple oracle sources.

use crate::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents a price update from an oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceUpdate {
    pub oracle_address: Address,
    pub feed_id: String,
    pub price: u128, // Scaled price (e.g., 1e18 for precision)
    pub timestamp: u64,
    pub signature: Option<Vec<u8>>, // Oracle signature for verification
}

/// Represents a price feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceFeed {
    pub feed_id: String, // e.g., "BTC/USD", "ETH/MSHW"
    pub asset_pair: (String, String), // (base, quote)
    pub current_price: u128, // Aggregated price
    pub last_update: u64,
    pub oracle_count: usize,
    pub update_frequency: u64, // seconds
    pub price_history: Vec<(u64, u128)>, // (timestamp, price) - last 100 updates
}

impl PriceFeed {
    pub fn new(feed_id: String, asset_pair: (String, String), update_frequency: u64) -> Self {
        Self {
            feed_id,
            asset_pair,
            current_price: 0,
            last_update: 0,
            oracle_count: 0,
            update_frequency,
            price_history: Vec::new(),
        }
    }

    /// Add price update to history (keep last 100)
    fn add_to_history(&mut self, timestamp: u64, price: u128) {
        self.price_history.push((timestamp, price));
        if self.price_history.len() > 100 {
            self.price_history.remove(0);
        }
    }
}

/// Manages all price feeds
pub struct PriceFeedManager {
    feeds: HashMap<String, PriceFeed>,
    pending_updates: HashMap<String, Vec<PriceUpdate>>, // feed_id -> updates
    registry: Arc<RwLock<crate::oracles::registry::OracleRegistry>>,
}

impl PriceFeedManager {
    pub fn new(registry: Arc<RwLock<crate::oracles::registry::OracleRegistry>>) -> Self {
        Self {
            feeds: HashMap::new(),
            pending_updates: HashMap::new(),
            registry,
        }
    }

    /// Create a new price feed
    pub fn create_feed(&mut self, feed_id: String, asset_pair: (String, String), update_frequency: u64) {
        let feed = PriceFeed::new(feed_id.clone(), asset_pair, update_frequency);
        self.feeds.insert(feed_id, feed);
    }

    /// Submit a price update from an oracle
    pub fn submit_price_update(&mut self, update: PriceUpdate) -> Result<(), String> {
        // Verify oracle is registered and assigned to this feed
        let registry_read = self.registry.blocking_read();
        let oracle = registry_read.get_oracle(&update.oracle_address)
            .ok_or("Oracle not registered")?;
        
        // Check if feed exists
        if !self.feeds.contains_key(&update.feed_id) {
            return Err("Price feed not found".to_string());
        }

        // Add to pending updates
        self.pending_updates
            .entry(update.feed_id.clone())
            .or_insert_with(Vec::new)
            .push(update);

        Ok(())
    }

    /// Aggregate pending updates for a feed (median of all oracle prices)
    pub fn aggregate_feed(&mut self, feed_id: &str, current_time: u64) -> Result<(), String> {
        let updates = self.pending_updates.remove(feed_id)
            .ok_or("No pending updates for feed")?;

        if updates.is_empty() {
            return Err("No updates to aggregate".to_string());
        }

        // Get assigned oracles for this feed
        let registry_read = self.registry.blocking_read();
        let assigned_oracles = registry_read.get_feed_oracles(feed_id)
            .ok_or("Feed has no assigned oracles")?;

        // Filter updates to only assigned oracles
        let valid_updates: Vec<u128> = updates
            .iter()
            .filter(|update| assigned_oracles.contains(&update.oracle_address))
            .map(|update| update.price)
            .collect();

        if valid_updates.is_empty() {
            return Err("No valid oracle updates".to_string());
        }

        // Calculate median price
        let mut prices = valid_updates;
        prices.sort();
        let median_price = if prices.len() % 2 == 0 {
            // Even number: average of two middle values
            (prices[prices.len() / 2 - 1] + prices[prices.len() / 2]) / 2
        } else {
            // Odd number: middle value
            prices[prices.len() / 2]
        };

        // Update feed
        if let Some(feed) = self.feeds.get_mut(feed_id) {
            feed.current_price = median_price;
            feed.last_update = current_time;
            feed.oracle_count = prices.len();
            feed.add_to_history(current_time, median_price);
        }

        Ok(())
    }

    /// Get current price for a feed
    pub fn get_price(&self, feed_id: &str) -> Option<u128> {
        self.feeds.get(feed_id).map(|feed| feed.current_price)
    }

    /// Get price feed information
    pub fn get_feed(&self, feed_id: &str) -> Option<&PriceFeed> {
        self.feeds.get(feed_id)
    }

    /// Get price history for a feed
    pub fn get_price_history(&self, feed_id: &str, limit: Option<usize>) -> Vec<(u64, u128)> {
        if let Some(feed) = self.feeds.get(feed_id) {
            let limit = limit.unwrap_or(100);
            feed.price_history
                .iter()
                .rev()
                .take(limit)
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all price feeds
    pub fn get_all_feeds(&self) -> Vec<&PriceFeed> {
        self.feeds.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_feed_creation() {
        let registry = Arc::new(RwLock::new(crate::oracles::registry::OracleRegistry::default()));
        let mut manager = PriceFeedManager::new(registry);
        
        manager.create_feed("BTC/USD".to_string(), ("BTC".to_string(), "USD".to_string()), 60);
        
        assert!(manager.get_feed("BTC/USD").is_some());
    }

    #[test]
    fn test_price_aggregation_median() {
        let registry = Arc::new(RwLock::new(crate::oracles::registry::default()));
        let mut manager = PriceFeedManager::new(registry.clone());
        
        manager.create_feed("BTC/USD".to_string(), ("BTC".to_string(), "USD".to_string()), 60);
        
        // Create test oracles
        let oracle1 = Address::from([1; 20]);
        let oracle2 = Address::from([2; 20]);
        let oracle3 = Address::from([3; 20]);
        
        // Register oracles
        registry.blocking_write().register_oracle(
            oracle1,
            vec![crate::oracles::registry::FeedType::Price],
            2_000_000_000_000_000_000,
            1000,
        ).unwrap();
        
        registry.blocking_write().register_oracle(
            oracle2,
            vec![crate::oracles::registry::FeedType::Price],
            2_000_000_000_000_000_000,
            1000,
        ).unwrap();
        
        registry.blocking_write().register_oracle(
            oracle3,
            vec![crate::oracles::registry::FeedType::Price],
            2_000_000_000_000_000_000,
            1000,
        ).unwrap();
        
        // Assign oracles to feed
        registry.blocking_write().assign_oracles_to_feed(
            "BTC/USD".to_string(),
            vec![oracle1, oracle2, oracle3],
        ).unwrap();
        
        // Submit price updates
        manager.submit_price_update(PriceUpdate {
            oracle_address: oracle1,
            feed_id: "BTC/USD".to_string(),
            price: 50_000_000_000_000_000_000, // 50,000 USD (scaled)
            timestamp: 2000,
            signature: None,
        }).unwrap();
        
        manager.submit_price_update(PriceUpdate {
            oracle_address: oracle2,
            feed_id: "BTC/USD".to_string(),
            price: 51_000_000_000_000_000_000, // 51,000 USD
            timestamp: 2000,
            signature: None,
        }).unwrap();
        
        manager.submit_price_update(PriceUpdate {
            oracle_address: oracle3,
            feed_id: "BTC/USD".to_string(),
            price: 52_000_000_000_000_000_000, // 52,000 USD
            timestamp: 2000,
            signature: None,
        }).unwrap();
        
        // Aggregate
        manager.aggregate_feed("BTC/USD", 2000).unwrap();
        
        // Median should be 51,000
        assert_eq!(manager.get_price("BTC/USD"), Some(51_000_000_000_000_000_000));
    }
}
