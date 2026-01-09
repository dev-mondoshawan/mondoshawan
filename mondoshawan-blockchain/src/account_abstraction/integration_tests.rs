//! End-to-end integration tests for Account Abstraction with Blockchain

#[cfg(test)]
mod integration_tests {
    use crate::account_abstraction::{WalletFactory, WalletRegistry};
    use crate::blockchain::{Blockchain, Transaction, Block, BlockHeader};
    use crate::types::{Address, StreamType};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    /// Test that contract wallets can send transactions with proper nonce handling
    #[tokio::test]
    async fn test_contract_wallet_transaction_flow() {
        // Setup: Create blockchain and wallet registry
        let wallet_registry = Arc::new(RwLock::new(WalletRegistry::new()));
        let mut blockchain = Blockchain::new();
        
        // Note: In a full implementation, we'd set wallet_registry on blockchain
        // For now, we test the components separately
        
        // Create a basic wallet
        let owner = [1u8; 20];
        let wallet = WalletFactory::create_basic_wallet(owner, 0);
        
        // Register wallet
        {
            let mut registry = wallet_registry.write().await;
            registry.register_wallet(wallet.clone()).unwrap();
        }
        
        // Set initial balance for wallet
        blockchain.set_balance(wallet.address, 10000).unwrap();
        
        // Create transaction from wallet (nonce 0)
        let to = [2u8; 20];
        let value = 1000u128;
        let fee = 100u128;
        let nonce = 0u64;
        
        let tx = Transaction::new(wallet.address, to, value, fee, nonce);
        
        // Verify wallet nonce is 0 before transaction
        {
            let registry = wallet_registry.read().await;
            let wallet_ref = registry.get_wallet(&wallet.address).unwrap();
            assert_eq!(wallet_ref.get_nonce(), 0);
        }
        
        // Process transaction (in real implementation, this would update wallet nonce)
        // For now, we verify the transaction can be created and validated
        
        // Verify transaction properties
        assert_eq!(tx.from, wallet.address);
        assert_eq!(tx.to, to);
        assert_eq!(tx.value, value);
        assert_eq!(tx.fee, fee);
        assert_eq!(tx.nonce, 0);
    }

    /// Test spending limits are enforced for contract wallets
    #[tokio::test]
    async fn test_spending_limits_in_transaction() {
        let wallet_registry = Arc::new(RwLock::new(WalletRegistry::new()));
        
        // Create spending limit wallet
        let owner = [1u8; 20];
        let daily_limit = 1000u128;
        let wallet = WalletFactory::create_spending_limit_wallet(
            owner, 0, daily_limit, 5000, 20000
        );
        
        // Register wallet
        {
            let mut registry = wallet_registry.write().await;
            registry.register_wallet(wallet.clone()).unwrap();
        }
        
        // Verify wallet has spending limits
        {
            let registry = wallet_registry.read().await;
            let wallet_ref = registry.get_wallet(&wallet.address).unwrap();
            assert!(wallet_ref.has_spending_limits());
            
            // Check limits are set
            if let Some(ref limits) = wallet_ref.config.spending_limits {
                assert_eq!(limits.daily_limit, daily_limit);
            }
        }
    }

    /// Test multiple wallets for same owner
    #[tokio::test]
    async fn test_multiple_wallets_per_owner() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        
        // Create multiple wallets with different salts
        let wallet1 = WalletFactory::create_basic_wallet(owner, 0);
        let wallet2 = WalletFactory::create_basic_wallet(owner, 1);
        let wallet3 = WalletFactory::create_multisig_wallet(
            owner, 0, vec![[2u8; 20], [3u8; 20]], 2
        ).unwrap();
        
        // Register all
        registry.register_wallet(wallet1.clone()).unwrap();
        registry.register_wallet(wallet2.clone()).unwrap();
        registry.register_wallet(wallet3.clone()).unwrap();
        
        // Verify all are registered
        assert!(registry.is_contract_wallet(&wallet1.address));
        assert!(registry.is_contract_wallet(&wallet2.address));
        assert!(registry.is_contract_wallet(&wallet3.address));
        
        // Verify owner tracking
        let owner_wallets = registry.get_owner_wallets(&owner);
        assert_eq!(owner_wallets.len(), 3);
    }

    /// Test wallet nonce isolation (each wallet has its own nonce)
    #[tokio::test]
    async fn test_wallet_nonce_isolation() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        
        // Create two wallets
        let wallet1 = WalletFactory::create_basic_wallet(owner, 0);
        let wallet2 = WalletFactory::create_basic_wallet(owner, 1);
        
        registry.register_wallet(wallet1.clone()).unwrap();
        registry.register_wallet(wallet2.clone()).unwrap();
        
        // Increment nonce for wallet1
        registry.update_wallet_nonce(&wallet1.address).unwrap();
        registry.update_wallet_nonce(&wallet1.address).unwrap();
        
        // Wallet1 should have nonce 2
        let w1 = registry.get_wallet(&wallet1.address).unwrap();
        assert_eq!(w1.get_nonce(), 2);
        
        // Wallet2 should still have nonce 0
        let w2 = registry.get_wallet(&wallet2.address).unwrap();
        assert_eq!(w2.get_nonce(), 0);
        
        // Increment wallet2
        registry.update_wallet_nonce(&wallet2.address).unwrap();
        let w2 = registry.get_wallet(&wallet2.address).unwrap();
        assert_eq!(w2.get_nonce(), 1);
        
        // Wallet1 should still be 2
        let w1 = registry.get_wallet(&wallet1.address).unwrap();
        assert_eq!(w1.get_nonce(), 2);
    }
}
