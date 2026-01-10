//! Stop-Loss Manager
//!
//! Manages stop-loss orders with price monitoring and automatic execution.

use crate::types::{Address, Hash};
use crate::blockchain::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha3::{Digest, Keccak256};

/// Type of stop-loss trigger
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StopLossType {
    PriceAbove(u128), // Execute when price >= threshold
    PriceBelow(u128), // Execute when price <= threshold
    PercentChangeUp(f64), // Execute when price increases by X%
    PercentChangeDown(f64), // Execute when price decreases by X%
    PriceRange { min: u128, max: u128 }, // Execute when price leaves range
}

/// Status of a stop-loss order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopLossStatus {
    Active,
    Paused,
    Triggered,
    Cancelled,
    Expired,
}

/// Represents a stop-loss order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopLossOrder {
    pub stop_loss_id: Hash,
    pub wallet_address: Address,
    pub asset_pair: String, // e.g., "BTC/USD", "ETH/MSHW"
    pub trigger_type: StopLossType,
    pub transaction: Transaction, // Transaction to execute when triggered
    pub oracle_feed_id: Option<String>, // Optional: specific oracle feed
    pub created_at: u64,
    pub status: StopLossStatus,
    pub triggered_at: Option<u64>,
    pub triggered_price: Option<u128>,
    pub execution_tx_hash: Option<Hash>,
    pub initial_price: Option<u128>, // For percent change calculations
    pub expiration_date: Option<u64>,
}

impl StopLossOrder {
    pub fn new(
        wallet_address: Address,
        asset_pair: String,
        trigger_type: StopLossType,
        transaction: Transaction,
        oracle_feed_id: Option<String>,
        created_at: u64,
        expiration_date: Option<u64>,
    ) -> Self {
        let stop_loss_id = Self::calculate_id(wallet_address, &asset_pair, &trigger_type, created_at);

        Self {
            stop_loss_id,
            wallet_address,
            asset_pair,
            trigger_type,
            transaction,
            oracle_feed_id,
            created_at,
            status: StopLossStatus::Active,
            triggered_at: None,
            triggered_price: None,
            execution_tx_hash: None,
            initial_price: None,
            expiration_date,
        }
    }

    /// Check if stop-loss should trigger based on current price
    pub fn should_trigger(&self, current_price: u128, current_time: u64) -> bool {
        if self.status != StopLossStatus::Active {
            return false;
        }

        // Check expiration
        if let Some(expiration) = self.expiration_date {
            if current_time >= expiration {
                return false;
            }
        }

        // Set initial price if not set
        let initial_price = self.initial_price.unwrap_or(current_price);

        match &self.trigger_type {
            StopLossType::PriceAbove(threshold) => current_price >= *threshold,
            StopLossType::PriceBelow(threshold) => current_price <= *threshold,
            StopLossType::PercentChangeUp(percent) => {
                let change = ((current_price as f64 - initial_price as f64) / initial_price as f64) * 100.0;
                change >= *percent
            }
            StopLossType::PercentChangeDown(percent) => {
                let change = ((initial_price as f64 - current_price as f64) / initial_price as f64) * 100.0;
                change >= *percent
            }
            StopLossType::PriceRange { min, max } => {
                current_price < *min || current_price > *max
            }
        }
    }

    /// Calculate stop-loss order ID
    fn calculate_id(
        wallet_address: Address,
        asset_pair: &str,
        trigger_type: &StopLossType,
        created_at: u64,
    ) -> Hash {
        let mut hasher = Keccak256::new();
        hasher.update(&wallet_address);
        hasher.update(asset_pair.as_bytes());
        hasher.update(&bincode::serialize(trigger_type).unwrap());
        hasher.update(&created_at.to_le_bytes());
        hasher.finalize().into()
    }
}

/// Manages all stop-loss orders
pub struct StopLossManager {
    orders: HashMap<Hash, StopLossOrder>,
}

impl StopLossManager {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    /// Create a new stop-loss order
    pub fn create_stop_loss(
        &mut self,
        wallet_address: Address,
        asset_pair: String,
        trigger_type: StopLossType,
        transaction: Transaction,
        oracle_feed_id: Option<String>,
        created_at: u64,
        expiration_date: Option<u64>,
    ) -> StopLossOrder {
        let order = StopLossOrder::new(
            wallet_address,
            asset_pair,
            trigger_type,
            transaction,
            oracle_feed_id,
            created_at,
            expiration_date,
        );

        let id = order.stop_loss_id;
        self.orders.insert(id, order.clone());
        order
    }

    /// Cancel a stop-loss order
    pub fn cancel(&mut self, stop_loss_id: &Hash) -> Result<(), String> {
        let order = self.orders.get_mut(stop_loss_id)
            .ok_or("Stop-loss order not found")?;

        if order.status == StopLossStatus::Triggered {
            return Err("Cannot cancel triggered order".to_string());
        }

        order.status = StopLossStatus::Cancelled;
        Ok(())
    }

    /// Get stop-loss order
    pub fn get(&self, stop_loss_id: &Hash) -> Option<&StopLossOrder> {
        self.orders.get(stop_loss_id)
    }

    /// Get all stop-loss orders for an address
    pub fn get_for_address(&self, address: &Address) -> Vec<&StopLossOrder> {
        self.orders
            .values()
            .filter(|order| order.wallet_address == *address)
            .collect()
    }

    /// Get active stop-loss orders for a feed
    pub fn get_active_for_feed(&self, feed_id: &str) -> Vec<&StopLossOrder> {
        self.orders
            .values()
            .filter(|order| {
                order.status == StopLossStatus::Active
                    && order.oracle_feed_id.as_ref().map(|id| id == feed_id).unwrap_or(true)
            })
            .collect()
    }

    /// Mark order as triggered
    pub fn mark_triggered(
        &mut self,
        stop_loss_id: &Hash,
        triggered_price: u128,
        current_time: u64,
    ) -> Result<(), String> {
        let order = self.orders.get_mut(stop_loss_id)
            .ok_or("Stop-loss order not found")?;

        order.status = StopLossStatus::Triggered;
        order.triggered_at = Some(current_time);
        order.triggered_price = Some(triggered_price);
        Ok(())
    }

    /// Set execution transaction hash
    pub fn set_execution_tx(&mut self, stop_loss_id: &Hash, tx_hash: Hash) -> Result<(), String> {
        let order = self.orders.get_mut(stop_loss_id)
            .ok_or("Stop-loss order not found")?;

        order.execution_tx_hash = Some(tx_hash);
        Ok(())
    }

    /// Set initial price (for percent change calculations)
    pub fn set_initial_price(&mut self, stop_loss_id: &Hash, price: u128) -> Result<(), String> {
        let order = self.orders.get_mut(stop_loss_id)
            .ok_or("Stop-loss order not found")?;

        if order.initial_price.is_none() {
            order.initial_price = Some(price);
        }
        Ok(())
    }

    /// Pause a stop-loss order
    pub fn pause(&mut self, stop_loss_id: &Hash) -> Result<(), String> {
        let order = self.orders.get_mut(stop_loss_id)
            .ok_or("Stop-loss order not found")?;

        if order.status != StopLossStatus::Active {
            return Err("Order is not active".to_string());
        }

        order.status = StopLossStatus::Paused;
        Ok(())
    }

    /// Resume a paused stop-loss order
    pub fn resume(&mut self, stop_loss_id: &Hash) -> Result<(), String> {
        let order = self.orders.get_mut(stop_loss_id)
            .ok_or("Stop-loss order not found")?;

        if order.status != StopLossStatus::Paused {
            return Err("Order is not paused".to_string());
        }

        order.status = StopLossStatus::Active;
        Ok(())
    }

    /// Update trigger price (for PriceAbove/PriceBelow types)
    pub fn update_trigger_price(&mut self, stop_loss_id: &Hash, new_price: u128) -> Result<(), String> {
        let order = self.orders.get_mut(stop_loss_id)
            .ok_or("Stop-loss order not found")?;

        match &mut order.trigger_type {
            StopLossType::PriceAbove(threshold) => *threshold = new_price,
            StopLossType::PriceBelow(threshold) => *threshold = new_price,
            _ => return Err("Cannot update price for this trigger type".to_string()),
        }

        Ok(())
    }
}

impl Default for StopLossManager {
    fn default() -> Self {
        Self::new()
    }
}
