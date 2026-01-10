//! Oracle Staking
//!
//! Manages staking for oracle nodes with slashing for false data.

use crate::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Staking information for an oracle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakingInfo {
    pub address: Address,
    pub staked_amount: u128,
    pub slashed_amount: u128,
    pub total_rewards: u128,
    pub slash_count: u64,
}

/// Manages oracle staking and slashing
pub struct OracleStaking {
    stakes: HashMap<Address, StakingInfo>,
    slashing_percentage: f64,
}

impl OracleStaking {
    pub fn new(slashing_percentage: f64) -> Self {
        Self {
            stakes: HashMap::new(),
            slashing_percentage,
        }
    }

    /// Stake tokens for an oracle
    pub fn stake(&mut self, address: Address, amount: u128) {
        let staking_info = self.stakes.entry(address).or_insert_with(|| {
            StakingInfo {
                address,
                staked_amount: 0,
                slashed_amount: 0,
                total_rewards: 0,
                slash_count: 0,
            }
        });

        staking_info.staked_amount += amount;
    }

    /// Unstake tokens (with lock period in production)
    pub fn unstake(&mut self, address: &Address, amount: u128) -> Result<(), String> {
        let staking_info = self.stakes.get_mut(address)
            .ok_or("Oracle not staked")?;

        if staking_info.staked_amount < amount {
            return Err("Insufficient stake".to_string());
        }

        staking_info.staked_amount -= amount;
        Ok(())
    }

    /// Slash oracle for false data
    pub fn slash(&mut self, address: &Address) -> Result<u128, String> {
        let staking_info = self.stakes.get_mut(address)
            .ok_or("Oracle not staked")?;

        let slash_amount = (staking_info.staked_amount as f64 * self.slashing_percentage) as u128;
        
        if staking_info.staked_amount < slash_amount {
            return Err("Insufficient stake to slash".to_string());
        }

        staking_info.staked_amount -= slash_amount;
        staking_info.slashed_amount += slash_amount;
        staking_info.slash_count += 1;

        Ok(slash_amount)
    }

    /// Get staking information
    pub fn get_stake(&self, address: &Address) -> Option<&StakingInfo> {
        self.stakes.get(address)
    }

    /// Get all staking information
    pub fn get_all_stakes(&self) -> Vec<&StakingInfo> {
        self.stakes.values().collect()
    }
}

impl Default for OracleStaking {
    fn default() -> Self {
        Self::new(0.1) // 10% slashing
    }
}
