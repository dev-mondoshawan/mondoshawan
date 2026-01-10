//! Stop-Loss Monitor
//!
//! Monitors price feeds and triggers stop-loss orders when conditions are met.

use crate::stop_loss::manager::{StopLossManager, StopLossOrder};
use crate::oracles::price_feed::PriceFeedManager;
use crate::blockchain::Transaction;
use crate::types::Hash;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Monitors price feeds and triggers stop-loss orders
pub struct StopLossMonitor {
    manager: Arc<RwLock<StopLossManager>>,
    price_feeds: Arc<RwLock<PriceFeedManager>>,
}

impl StopLossMonitor {
    pub fn new(
        manager: Arc<RwLock<StopLossManager>>,
        price_feeds: Arc<RwLock<PriceFeedManager>>,
    ) -> Self {
        Self {
            manager,
            price_feeds,
        }
    }

    /// Check all active stop-loss orders and trigger if conditions are met
    pub async fn check_and_trigger(&self, current_time: u64) -> Vec<(Hash, Transaction)> {
        let manager_read = self.manager.read().await;
        let price_feeds_read = self.price_feeds.read().await;
        
        let mut triggered = Vec::new();
        
        // Get all active orders
        let active_orders: Vec<(Hash, StopLossOrder)> = manager_read
            .orders
            .iter()
            .filter(|(_, order)| order.status == crate::stop_loss::manager::StopLossStatus::Active)
            .map(|(id, order)| (*id, order.clone()))
            .collect();
        
        for (order_id, order) in active_orders {
            // Get current price for the asset pair
            if let Some(current_price) = price_feeds_read.get_price(&order.asset_pair) {
                // Set initial price if not set
                if order.initial_price.is_none() {
                    let mut manager_write = self.manager.write().await;
                    manager_write.set_initial_price(&order_id, current_price).ok();
                }
                
                // Check if should trigger
                if order.should_trigger(current_price, current_time) {
                    // Mark as triggered
                    let mut manager_write = self.manager.write().await;
                    manager_write.mark_triggered(&order_id, current_price, current_time).ok();
                    
                    // Return transaction to execute
                    triggered.push((order_id, order.transaction.clone()));
                }
            }
        }
        
        triggered
    }

    /// Get orders ready to execute for a specific feed
    pub async fn check_feed(&self, feed_id: &str, current_time: u64) -> Vec<(Hash, Transaction)> {
        let manager_read = self.manager.read().await;
        let price_feeds_read = self.price_feeds.read().await;
        
        let mut triggered = Vec::new();
        
        // Get active orders for this feed
        let active_orders = manager_read.get_active_for_feed(feed_id);
        
        if let Some(current_price) = price_feeds_read.get_price(feed_id) {
            for order in active_orders {
                // Set initial price if not set
                if order.initial_price.is_none() {
                    let mut manager_write = self.manager.write().await;
                    manager_write.set_initial_price(&order.stop_loss_id, current_price).ok();
                }
                
                // Check if should trigger
                if order.should_trigger(current_price, current_time) {
                    // Mark as triggered
                    let mut manager_write = self.manager.write().await;
                    manager_write.mark_triggered(&order.stop_loss_id, current_price, current_time).ok();
                    
                    // Return transaction to execute
                    triggered.push((order.stop_loss_id, order.transaction.clone()));
                }
            }
        }
        
        triggered
    }
}
