//! Integration tests for Account Abstraction

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_abstraction::{WalletFactory, WalletRegistry, SmartContractWallet};
    use crate::blockchain::{Blockchain, Transaction, Block, BlockHeader};
    use crate::types::{Address, StreamType};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    fn create_test_blockchain_with_wallets() -> (Blockchain, Arc<RwLock<WalletRegistry>>) {
        let wallet_registry = Arc::new(RwLock::new(WalletRegistry::new()));
        let mut blockchain = Blockchain::new();
        
        // Note: In a real implementation, we'd need a way to set wallet_registry on Blockchain
        // For now, we'll test the registry separately and integration will be tested via RPC
        
        (blockchain, wallet_registry)
    }

    #[tokio::test]
    async fn test_basic_wallet_creation() {
        let registry = WalletRegistry::new();
        let owner = [1u8; 20];
        let salt = 0u64;
        
        let wallet = WalletFactory::create_basic_wallet(owner, salt);
        
        // Register wallet
        let mut registry = registry;
        assert!(registry.register_wallet(wallet.clone()).is_ok());
        
        // Verify wallet is registered
        assert!(registry.is_contract_wallet(&wallet.address));
        
        // Verify wallet properties
        let retrieved = registry.get_wallet(&wallet.address).unwrap();
        assert_eq!(retrieved.address, wallet.address);
        assert_eq!(retrieved.owner, owner);
        assert_eq!(retrieved.nonce, 0);
    }

    #[tokio::test]
    async fn test_multisig_wallet_creation() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        let salt = 0u64;
        let signers = vec![[2u8; 20], [3u8; 20], [4u8; 20]];
        let threshold = 2;
        
        let wallet = WalletFactory::create_multisig_wallet(owner, salt, signers.clone(), threshold)
            .unwrap();
        
        // Register wallet
        assert!(registry.register_wallet(wallet.clone()).is_ok());
        
        // Verify wallet is registered
        assert!(registry.is_contract_wallet(&wallet.address));
        
        // Verify wallet properties
        let retrieved = registry.get_wallet(&wallet.address).unwrap();
        assert!(retrieved.is_multisig());
        assert_eq!(retrieved.nonce, 0);
    }

    #[tokio::test]
    async fn test_social_recovery_wallet_creation() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        let salt = 0u64;
        let guardians = vec![[2u8; 20], [3u8; 20], [4u8; 20], [5u8; 20]];
        let recovery_threshold = 3;
        let time_delay = 604800; // 7 days
        
        let wallet = WalletFactory::create_social_recovery_wallet(
            owner, salt, guardians.clone(), recovery_threshold, time_delay
        ).unwrap();
        
        // Register wallet
        assert!(registry.register_wallet(wallet.clone()).is_ok());
        
        // Verify wallet is registered
        assert!(registry.is_contract_wallet(&wallet.address));
        
        // Verify wallet properties
        let retrieved = registry.get_wallet(&wallet.address).unwrap();
        assert!(retrieved.has_social_recovery());
        assert_eq!(retrieved.nonce, 0);
    }

    #[tokio::test]
    async fn test_spending_limit_wallet_creation() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        let salt = 0u64;
        let daily_limit = 1000u128;
        let weekly_limit = 5000u128;
        let monthly_limit = 20000u128;
        
        let wallet = WalletFactory::create_spending_limit_wallet(
            owner, salt, daily_limit, weekly_limit, monthly_limit
        );
        
        // Register wallet
        assert!(registry.register_wallet(wallet.clone()).is_ok());
        
        // Verify wallet is registered
        assert!(registry.is_contract_wallet(&wallet.address));
        
        // Verify wallet properties
        let retrieved = registry.get_wallet(&wallet.address).unwrap();
        assert!(retrieved.has_spending_limits());
        assert_eq!(retrieved.nonce, 0);
    }

    #[tokio::test]
    async fn test_wallet_address_derivation() {
        let owner = [1u8; 20];
        let salt = 12345u64;
        
        // Same owner + salt + type = same address
        let wallet1 = WalletFactory::create_basic_wallet(owner, salt);
        let wallet2 = WalletFactory::create_basic_wallet(owner, salt);
        assert_eq!(wallet1.address, wallet2.address);
        
        // Different salt = different address
        let wallet3 = WalletFactory::create_basic_wallet(owner, 12346);
        assert_ne!(wallet1.address, wallet3.address);
        
        // Different type = different address
        let wallet4 = WalletFactory::create_multisig_wallet(
            owner, salt, vec![[2u8; 20]], 1
        ).unwrap();
        assert_ne!(wallet1.address, wallet4.address);
    }

    #[tokio::test]
    async fn test_wallet_nonce_management() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        let wallet = WalletFactory::create_basic_wallet(owner, 0);
        
        // Register wallet
        registry.register_wallet(wallet.clone()).unwrap();
        
        // Initial nonce should be 0
        let retrieved = registry.get_wallet(&wallet.address).unwrap();
        assert_eq!(retrieved.get_nonce(), 0);
        
        // Increment nonce
        registry.update_wallet_nonce(&wallet.address).unwrap();
        
        // Verify nonce incremented
        let retrieved = registry.get_wallet(&wallet.address).unwrap();
        assert_eq!(retrieved.get_nonce(), 1);
        
        // Increment again
        registry.update_wallet_nonce(&wallet.address).unwrap();
        let retrieved = registry.get_wallet(&wallet.address).unwrap();
        assert_eq!(retrieved.get_nonce(), 2);
    }

    #[tokio::test]
    async fn test_owner_wallet_tracking() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        
        // Create multiple wallets for same owner
        let wallet1 = WalletFactory::create_basic_wallet(owner, 0);
        let wallet2 = WalletFactory::create_basic_wallet(owner, 1);
        let wallet3 = WalletFactory::create_multisig_wallet(
            owner, 0, vec![[2u8; 20]], 1
        ).unwrap();
        
        // Register all wallets
        registry.register_wallet(wallet1.clone()).unwrap();
        registry.register_wallet(wallet2.clone()).unwrap();
        registry.register_wallet(wallet3.clone()).unwrap();
        
        // Get all wallets for owner
        let owner_wallets = registry.get_owner_wallets(&owner);
        assert_eq!(owner_wallets.len(), 3);
        
        // Verify all wallets are present
        let addresses: Vec<Address> = owner_wallets.iter().map(|w| w.address).collect();
        assert!(addresses.contains(&wallet1.address));
        assert!(addresses.contains(&wallet2.address));
        assert!(addresses.contains(&wallet3.address));
    }

    #[tokio::test]
    async fn test_spending_limits_enforcement() {
        use crate::account_abstraction::wallet::SpendingLimits;
        
        let mut limits = SpendingLimits::new(1000, 5000, 20000);
        
        // Should allow spending within limit
        assert!(limits.check_limit(500).is_ok());
        limits.record_spending(500);
        assert_eq!(limits.current_period_spending, 500);
        
        // Should allow more spending up to limit
        assert!(limits.check_limit(400).is_ok());
        limits.record_spending(400);
        assert_eq!(limits.current_period_spending, 900);
        
        // Should reject spending over limit
        assert!(limits.check_limit(200).is_err());
        
        // Should allow spending exactly at limit
        assert!(limits.check_limit(100).is_ok());
        limits.record_spending(100);
        assert_eq!(limits.current_period_spending, 1000);
    }

    #[tokio::test]
    async fn test_wallet_registry_uniqueness() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        
        // Create and register first wallet
        let wallet1 = WalletFactory::create_basic_wallet(owner, 0);
        assert!(registry.register_wallet(wallet1.clone()).is_ok());
        
        // Try to register same wallet again (should fail)
        let wallet1_duplicate = wallet1.clone();
        assert!(registry.register_wallet(wallet1_duplicate).is_err());
        
        // Create different wallet with different salt (should succeed)
        let wallet2 = WalletFactory::create_basic_wallet(owner, 1);
        assert!(registry.register_wallet(wallet2).is_ok());
    }

    #[tokio::test]
    async fn test_multisig_threshold_validation() {
        let owner = [1u8; 20];
        
        // Valid: threshold within signer count
        assert!(WalletFactory::create_multisig_wallet(
            owner, 0, vec![[2u8; 20], [3u8; 20], [4u8; 20]], 2
        ).is_ok());
        
        // Invalid: threshold too high
        assert!(WalletFactory::create_multisig_wallet(
            owner, 0, vec![[2u8; 20], [3u8; 20]], 5
        ).is_err());
        
        // Invalid: threshold zero
        assert!(WalletFactory::create_multisig_wallet(
            owner, 0, vec![[2u8; 20], [3u8; 20]], 0
        ).is_err());
        
        // Invalid: empty signers
        assert!(WalletFactory::create_multisig_wallet(
            owner, 0, vec![], 1
        ).is_err());
    }

    #[tokio::test]
    async fn test_social_recovery_threshold_validation() {
        let owner = [1u8; 20];
        
        // Valid: threshold within guardian count
        assert!(WalletFactory::create_social_recovery_wallet(
            owner, 0, vec![[2u8; 20], [3u8; 20], [4u8; 20], [5u8; 20]], 3, 604800
        ).is_ok());
        
        // Invalid: threshold too high
        assert!(WalletFactory::create_social_recovery_wallet(
            owner, 0, vec![[2u8; 20], [3u8; 20]], 5, 604800
        ).is_err());
        
        // Invalid: empty guardians
        assert!(WalletFactory::create_social_recovery_wallet(
            owner, 0, vec![], 1, 604800
        ).is_err());
    }
}
