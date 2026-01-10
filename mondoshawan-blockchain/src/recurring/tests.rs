#[cfg(test)]
mod tests {
    use super::{RecurringTransactionManager, Schedule, RecurringTxStatus, RecurringScheduler};
    use crate::types::Address;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_recurring_transaction_creation() {
        let manager = Arc::new(RwLock::new(RecurringTransactionManager::new()));
        let from = Address::from([1; 20]);
        let to = Address::from([2; 20]);
        let value = 1_000_000_000_000_000_000; // 1 MSHW
        let schedule = Schedule::Daily { hour: 12, minute: 0 };
        let current_time = 1000;

        let recurring = manager.write().await.create_recurring(
            from,
            to,
            value,
            schedule,
            current_time,
            Some(10), // max_executions
            None, // end_date
            current_time,
        );

        assert_eq!(recurring.from, from);
        assert_eq!(recurring.to, to);
        assert_eq!(recurring.value, value);
        assert_eq!(recurring.status, RecurringTxStatus::Active);
    }

    #[tokio::test]
    async fn test_recurring_transaction_cancellation() {
        let manager = Arc::new(RwLock::new(RecurringTransactionManager::new()));
        let from = Address::from([1; 20]);
        let to = Address::from([2; 20]);
        let schedule = Schedule::Daily { hour: 12, minute: 0 };
        let current_time = 1000;

        let recurring = manager.write().await.create_recurring(
            from,
            to,
            1_000_000_000_000_000_000,
            schedule,
            current_time,
            None,
            None,
            current_time,
        );

        // Cancel
        manager.write().await.cancel(&recurring.recurring_tx_id).unwrap();

        let tx = manager.read().await.get(&recurring.recurring_tx_id).unwrap();
        assert_eq!(tx.status, RecurringTxStatus::Cancelled);
    }

    #[tokio::test]
    async fn test_recurring_scheduler() {
        let manager = Arc::new(RwLock::new(RecurringTransactionManager::new()));
        let scheduler = RecurringScheduler::new(manager.clone());
        let from = Address::from([1; 20]);
        let to = Address::from([2; 20]);
        let current_time = 1000;

        // Create recurring transaction
        let recurring = manager.write().await.create_recurring(
            from,
            to,
            1_000_000_000_000_000_000,
            Schedule::Daily { hour: 12, minute: 0 },
            current_time,
            None,
            None,
            current_time,
        );

        // Check for ready transactions (should be empty initially)
        let ready = scheduler.check_and_execute(current_time).await;
        assert_eq!(ready.len(), 0);
    }
}
