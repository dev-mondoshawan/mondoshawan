#[cfg(test)]
mod tests {
    use super::{OracleRegistry, PriceFeedManager, PriceUpdate, FeedType, VrfManager};
    use crate::types::Address;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_oracle_registration_and_price_feed() {
        let registry = Arc::new(RwLock::new(OracleRegistry::default()));
        let price_feeds = Arc::new(RwLock::new(PriceFeedManager::new()));

        let oracle1 = Address::from([1; 20]);
        let oracle2 = Address::from([2; 20]);
        let oracle3 = Address::from([3; 20]);

        // Register oracles
        registry.write().await.register_oracle(
            oracle1,
            vec![FeedType::Price],
            2_000_000_000_000_000_000,
            1000,
        ).unwrap();

        registry.write().await.register_oracle(
            oracle2,
            vec![FeedType::Price],
            2_000_000_000_000_000_000,
            1000,
        ).unwrap();

        registry.write().await.register_oracle(
            oracle3,
            vec![FeedType::Price],
            2_000_000_000_000_000_000,
            1000,
        ).unwrap();

        // Assign oracles to feed
        registry.write().await.assign_oracles_to_feed(
            "BTC/USD".to_string(),
            vec![oracle1, oracle2, oracle3],
        ).unwrap();

        // Submit price updates
        price_feeds.write().await.submit_price_update(PriceUpdate {
            oracle_address: oracle1,
            feed_id: "BTC/USD".to_string(),
            price: 50_000_000_000_000_000_000,
            timestamp: 2000,
            signature: None,
        }).unwrap();

        price_feeds.write().await.submit_price_update(PriceUpdate {
            oracle_address: oracle2,
            feed_id: "BTC/USD".to_string(),
            price: 51_000_000_000_000_000_000,
            timestamp: 2000,
            signature: None,
        }).unwrap();

        price_feeds.write().await.submit_price_update(PriceUpdate {
            oracle_address: oracle3,
            feed_id: "BTC/USD".to_string(),
            price: 52_000_000_000_000_000_000,
            timestamp: 2000,
            signature: None,
        }).unwrap();

        // Aggregate
        price_feeds.write().await.aggregate_feed("BTC/USD", 2000).unwrap();

        // Median should be 51,000
        assert_eq!(price_feeds.read().await.get_price("BTC/USD"), Some(51_000_000_000_000_000_000));
    }

    #[tokio::test]
    async fn test_vrf_randomness() {
        let vrf = Arc::new(RwLock::new(VrfManager::new()));
        let requester = Address::from([1; 20]);
        let seed = [42u8; 32];
        let timestamp = 1000;

        let request_id = vrf.write().await.request_randomness(requester, seed, timestamp);
        
        // Fulfill request
        vrf.write().await.fulfill_randomness(&request_id, requester).unwrap();

        let request = vrf.read().await.get_request(&request_id).unwrap();
        assert!(request.fulfilled);
        assert!(request.randomness.is_some());
    }
}
