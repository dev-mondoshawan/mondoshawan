//! Built-In Oracle Network
//!
//! Native oracle system for price feeds, randomness, and external data.
//! Provides protocol-level oracles with staking, aggregation, and slashing.

pub mod registry;
pub mod price_feed;
pub mod vrf;
pub mod staking;

#[cfg(test)]
mod tests;

pub use registry::{OracleRegistry, OracleNode, FeedType};
pub use price_feed::{PriceFeedManager, PriceFeed, PriceUpdate};
pub use vrf::{VrfManager, RandomnessRequest, RandomnessProof};
pub use staking::{OracleStaking, StakingInfo};

use serde::{Deserialize, Serialize};

/// Oracle network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OracleConfig {
    /// Minimum stake required to become an oracle
    pub min_stake: u128,
    /// Number of oracles required per feed
    pub min_oracles_per_feed: usize,
    /// Slashing percentage for false data
    pub slashing_percentage: f64,
    /// Update frequency for price feeds (seconds)
    pub price_update_frequency: u64,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            min_stake: 1_000_000_000_000_000_000, // 1 MSHW
            min_oracles_per_feed: 3,
            slashing_percentage: 0.1, // 10% slashing
            price_update_frequency: 60, // 60 seconds
        }
    }
}
