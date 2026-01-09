//! Unit tests for quick win features: Time-Locked Transactions, Gasless Transactions, Reputation System

#[cfg(test)]
mod tests {
    use crate::blockchain::{Blockchain, Transaction};
    use crate::types::Address;
    
    #[test]
    fn test_time_locked_transaction_block() {
        // Create a time-locked transaction that executes at block 10
        let from = [1u8; 20];
        let to = [2u8; 20];
        let value = 1000u128;
        let fee = 100u128;
        let nonce = 0u64;
        
        let tx = Transaction::new(from, to, value, fee, nonce)
            .with_execute_at_block(10);
        
        // Transaction should not be ready at block 5
        assert!(!tx.is_ready_to_execute(5, 0));
        
        // Transaction should be ready at block 10
        assert!(tx.is_ready_to_execute(10, 0));
        
        // Transaction should be ready at block 15
        assert!(tx.is_ready_to_execute(15, 0));
    }
    
    #[test]
    fn test_time_locked_transaction_timestamp() {
        let from = [1u8; 20];
        let to = [2u8; 20];
        let value = 1000u128;
        let fee = 100u128;
        let nonce = 0u64;
        
        let tx = Transaction::new(from, to, value, fee, nonce)
            .with_execute_at_timestamp(1000);
        
        // Transaction should not be ready at timestamp 500
        assert!(!tx.is_ready_to_execute(0, 500));
        
        // Transaction should be ready at timestamp 1000
        assert!(tx.is_ready_to_execute(0, 1000));
        
        // Transaction should be ready at timestamp 1500
        assert!(tx.is_ready_to_execute(0, 1500));
    }
    
    #[test]
    fn test_time_locked_transaction_both() {
        let from = [1u8; 20];
        let to = [2u8; 20];
        let value = 1000u128;
        let fee = 100u128;
        let nonce = 0u64;
        
        let tx = Transaction::new(from, to, value, fee, nonce)
            .with_execute_at_block(10)
            .with_execute_at_timestamp(1000);
        
        // Not ready: block too low
        assert!(!tx.is_ready_to_execute(5, 1500));
        
        // Not ready: timestamp too low
        assert!(!tx.is_ready_to_execute(15, 500));
        
        // Ready: both conditions met
        assert!(tx.is_ready_to_execute(15, 1500));
    }
    
    #[test]
    fn test_gasless_transaction() {
        let from = [1u8; 20];
        let to = [2u8; 20];
        let sponsor = [3u8; 20];
        let value = 1000u128;
        let fee = 100u128;
        let nonce = 0u64;
        
        let tx = Transaction::new(from, to, value, fee, nonce)
            .with_sponsor(sponsor);
        
        assert!(tx.is_gasless());
        assert_eq!(tx.sponsor, Some(sponsor));
    }
    
    #[test]
    fn test_gasless_transaction_processing() {
        let mut blockchain = Blockchain::new();
        
        // Set up balances
        blockchain.set_balance([1u8; 20], 2000).unwrap(); // Sender has enough for value
        blockchain.set_balance([3u8; 20], 500).unwrap();  // Sponsor has enough for fee
        blockchain.set_balance([2u8; 20], 0).unwrap();    // Receiver starts at 0
        
        let from = [1u8; 20];
        let to = [2u8; 20];
        let sponsor = [3u8; 20];
        let value = 1000u128;
        let fee = 100u128;
        let nonce = 0u64;
        
        let tx = Transaction::new(from, to, value, fee, nonce)
            .with_sponsor(sponsor);
        
        // Process transaction
        blockchain.process_transaction(&tx).unwrap();
        
        // Check balances
        assert_eq!(blockchain.get_balance(from), 1000);  // Value deducted
        assert_eq!(blockchain.get_balance(sponsor), 400); // Fee deducted
        assert_eq!(blockchain.get_balance(to), 1000);    // Value received
    }
    
    #[test]
    fn test_regular_transaction_processing() {
        let mut blockchain = Blockchain::new();
        
        // Set up balances
        blockchain.set_balance([1u8; 20], 2000).unwrap(); // Sender has enough for value + fee
        blockchain.set_balance([2u8; 20], 0).unwrap();    // Receiver starts at 0
        
        let from = [1u8; 20];
        let to = [2u8; 20];
        let value = 1000u128;
        let fee = 100u128;
        let nonce = 0u64;
        
        let tx = Transaction::new(from, to, value, fee, nonce);
        
        // Process transaction
        blockchain.process_transaction(&tx).unwrap();
        
        // Check balances
        assert_eq!(blockchain.get_balance(from), 900);   // Value + fee deducted
        assert_eq!(blockchain.get_balance(to), 1000);   // Value received
    }
    
    #[test]
    fn test_reputation_calculation() {
        use crate::reputation::ReputationManager;
        
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
        assert!(reputation.value() <= 100.0); // Should not exceed 100
    }
    
    #[test]
    fn test_reputation_penalties() {
        use crate::reputation::ReputationManager;
        
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
    
    #[test]
    fn test_reputation_factors() {
        use crate::reputation::ReputationManager;
        
        let mut manager = ReputationManager::new();
        let address = [1u8; 20];
        
        // Record some activity
        manager.record_successful_tx(&address, 1000, &[2u8; 20]);
        manager.record_successful_tx(&address, 2000, &[3u8; 20]);
        manager.record_failed_tx(&address);
        
        let factors = manager.get_factors(&address);
        assert!(factors.is_some());
        
        let factors = factors.unwrap();
        assert_eq!(factors.successful_txs, 2);
        assert_eq!(factors.failed_txs, 1);
        assert_eq!(factors.total_value_transacted, 3000);
    }
}
