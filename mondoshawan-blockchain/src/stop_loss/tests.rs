#[cfg(test)]
mod tests {
    use super::{StopLossManager, StopLossType, StopLossStatus};
    use crate::types::Address;
    use crate::blockchain::Transaction;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_stop_loss_creation() {
        let manager = Arc::new(RwLock::new(StopLossManager::new()));
        let wallet = Address::from([1; 20]);
        let to = Address::from([2; 20]);
        let trigger_type = StopLossType::PriceBelow(50_000_000_000_000_000_000);
        let transaction = Transaction::new(wallet, to, 1_000_000_000_000_000_000, 0, 0);
        let current_time = 1000;

        let order = manager.write().await.create_stop_loss(
            wallet,
            "BTC/USD".to_string(),
            trigger_type,
            transaction,
            Some("BTC/USD".to_string()),
            current_time,
            None,
        );

        assert_eq!(order.wallet_address, wallet);
        assert_eq!(order.asset_pair, "BTC/USD");
        assert_eq!(order.status, StopLossStatus::Active);
    }

    #[tokio::test]
    async fn test_stop_loss_trigger() {
        let manager = Arc::new(RwLock::new(StopLossManager::new()));
        let wallet = Address::from([1; 20]);
        let to = Address::from([2; 20]);
        let trigger_price = 50_000_000_000_000_000_000;
        let trigger_type = StopLossType::PriceBelow(trigger_price);
        let transaction = Transaction::new(wallet, to, 1_000_000_000_000_000_000, 0, 0);
        let current_time = 1000;

        let order = manager.write().await.create_stop_loss(
            wallet,
            "BTC/USD".to_string(),
            trigger_type,
            transaction,
            Some("BTC/USD".to_string()),
            current_time,
            None,
        );

        // Set initial price
        manager.write().await.set_initial_price(&order.stop_loss_id, 60_000_000_000_000_000_000).unwrap();

        // Price drops below threshold
        let current_price = 45_000_000_000_000_000_000;
        assert!(order.should_trigger(current_price, current_time));

        // Mark as triggered
        manager.write().await.mark_triggered(&order.stop_loss_id, current_price, current_time).unwrap();

        let updated_order = manager.read().await.get(&order.stop_loss_id).unwrap();
        assert_eq!(updated_order.status, StopLossStatus::Triggered);
        assert_eq!(updated_order.triggered_price, Some(current_price));
    }

    #[tokio::test]
    async fn test_stop_loss_cancellation() {
        let manager = Arc::new(RwLock::new(StopLossManager::new()));
        let wallet = Address::from([1; 20]);
        let to = Address::from([2; 20]);
        let trigger_type = StopLossType::PriceBelow(50_000_000_000_000_000_000);
        let transaction = Transaction::new(wallet, to, 1_000_000_000_000_000_000, 0, 0);
        let current_time = 1000;

        let order = manager.write().await.create_stop_loss(
            wallet,
            "BTC/USD".to_string(),
            trigger_type,
            transaction,
            Some("BTC/USD".to_string()),
            current_time,
            None,
        );

        // Cancel
        manager.write().await.cancel(&order.stop_loss_id).unwrap();

        let updated_order = manager.read().await.get(&order.stop_loss_id).unwrap();
        assert_eq!(updated_order.status, StopLossStatus::Cancelled);
    }
}
