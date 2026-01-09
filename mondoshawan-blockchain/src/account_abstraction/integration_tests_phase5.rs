//! Integration Tests for Account Abstraction Phase 5
//! 
//! Tests end-to-end workflows including wallet creation, multi-sig, recovery, and batch transactions

#[cfg(test)]
mod tests {
    use crate::account_abstraction::*;
    use crate::types::Address;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_e2e_wallet_creation_and_lookup() {
        let registry = Arc::new(RwLock::new(WalletRegistry::new()));
        
        let owner = Address::from([1; 20]);
        let salt = 0;
        
        // Create basic wallet
        let wallet = WalletFactory::create_basic_wallet(owner, salt);
        
        // Register wallet
        {
            let mut reg = registry.write().await;
            assert!(reg.register_wallet(wallet.clone()).is_ok());
        }
        
        // Lookup wallet
        {
            let reg = registry.read().await;
            let found = reg.get_wallet(&wallet.address);
            assert!(found.is_some());
            assert_eq!(found.unwrap().owner, owner);
        }
    }

    #[tokio::test]
    async fn test_e2e_multisig_workflow() {
        let registry = Arc::new(RwLock::new(WalletRegistry::new()));
        let multisig_manager = Arc::new(RwLock::new(MultiSigManager::new()));
        
        let owner = Address::from([1; 20]);
        let signer1 = Address::from([2; 20]);
        let signer2 = Address::from([3; 20]);
        let signer3 = Address::from([4; 20]);
        let signers = vec![signer1, signer2, signer3];
        let threshold = 2;
        
        // Create multi-sig wallet
        let wallet = WalletFactory::create_multisig_wallet(owner, 0, signers.clone(), threshold).unwrap();
        
        // Register wallet
        {
            let mut reg = registry.write().await;
            assert!(reg.register_wallet(wallet.clone()).is_ok());
        }
        
        // Create multi-sig transaction
        let tx_hash = [5u8; 32];
        let to = Address::from([6; 20]);
        let value = 1000u128;
        
        {
            let mut manager = multisig_manager.write().await;
            let result = manager.create_pending_transaction(
                wallet.address,
                tx_hash,
                to,
                value,
                signers.clone(),
                threshold,
            );
            assert!(result.is_ok());
        }
        
        // Verify pending transaction exists
        {
            let manager = multisig_manager.read().await;
            let pending = manager.get_pending_transactions(&wallet.address);
            assert_eq!(pending.len(), 1);
            assert_eq!(pending[0].tx_hash, tx_hash);
        }
    }

    #[tokio::test]
    async fn test_e2e_recovery_workflow() {
        let recovery_manager = Arc::new(RwLock::new(SocialRecoveryManager::new()));
        
        let wallet = Address::from([1; 20]);
        let new_owner = Address::from([2; 20]);
        let guardian1 = Address::from([3; 20]);
        let guardian2 = Address::from([4; 20]);
        let guardian3 = Address::from([5; 20]);
        let guardians = vec![guardian1, guardian2, guardian3];
        let recovery_threshold = 2;
        let timestamp = 1000;
        
        // Initiate recovery
        {
            let mut manager = recovery_manager.write().await;
            let result = manager.initiate_recovery(
                wallet,
                new_owner,
                guardians.clone(),
                recovery_threshold,
                None,
                timestamp,
            );
            assert!(result.is_ok());
        }
        
        // Add guardian approvals
        {
            let mut manager = recovery_manager.write().await;
            assert!(manager.approve_recovery(wallet, guardians[0], timestamp + 100).is_ok());
            assert!(manager.approve_recovery(wallet, guardians[1], timestamp + 200).is_ok());
        }
        
        // Check status
        {
            let manager = recovery_manager.read().await;
            let status = manager.get_recovery_status(&wallet);
            assert!(status.is_some());
            let status = status.unwrap();
            assert_eq!(status.status, RecoveryStatus::Approved);
            assert_eq!(status.approval_count(), 2);
            assert!(status.threshold_met());
        }
    }

    #[tokio::test]
    async fn test_e2e_batch_transaction_workflow() {
        let batch_manager = Arc::new(RwLock::new(BatchManager::new()));
        
        let wallet = Address::from([1; 20]);
        let operations = vec![
            BatchOperation::Transfer {
                to: Address::from([2; 20]),
                value: 1000,
            },
            BatchOperation::Transfer {
                to: Address::from([3; 20]),
                value: 2000,
            },
        ];
        let timestamp = 1000;
        
        // Create batch
        let batch = {
            let mut manager = batch_manager.write().await;
            manager.create_batch(
                wallet,
                operations.clone(),
                1,
                100_000,
                1_000_000_000,
                timestamp,
            ).unwrap()
        };
        
        // Verify batch created
        {
            let manager = batch_manager.read().await;
            let found = manager.get_batch(&batch.batch_id);
            assert!(found.is_some());
            assert_eq!(found.unwrap().operation_count(), 2);
            assert_eq!(found.unwrap().status, BatchStatus::Pending);
        }
        
        // Estimate gas
        {
            let manager = batch_manager.read().await;
            let estimate = manager.estimate_gas(&operations).unwrap();
            assert!(estimate.total_gas > 0);
            assert!(estimate.optimization_savings > 0);
        }
    }

    #[tokio::test]
    async fn test_e2e_wallet_with_spending_limits() {
        let registry = Arc::new(RwLock::new(WalletRegistry::new()));
        
        let owner = Address::from([1; 20]);
        let limits = SpendingLimits {
            daily_limit: 10_000,
            weekly_limit: 50_000,
            monthly_limit: 200_000,
            per_address_limits: vec![],
            current_period_spending: 0,
            last_reset: 0,
        };
        
        // Create spending limit wallet
        let wallet = WalletFactory::create_spending_limit_wallet(owner, 0, limits.clone());
        
        // Register wallet
        {
            let mut reg = registry.write().await;
            assert!(reg.register_wallet(wallet.clone()).is_ok());
        }
        
        // Verify wallet has spending limits
        {
            let reg = registry.read().await;
            let found = reg.get_wallet(&wallet.address);
            assert!(found.is_some());
            let wallet = found.unwrap();
            assert!(wallet.has_spending_limits());
        }
    }

    #[tokio::test]
    async fn test_e2e_combined_wallet_features() {
        let registry = Arc::new(RwLock::new(WalletRegistry::new()));
        
        let owner = Address::from([1; 20]);
        let guardians = vec![Address::from([4; 20]), Address::from([5; 20])];
        
        // Create social recovery wallet
        let wallet = WalletFactory::create_social_recovery_wallet(
            owner,
            0,
            guardians.clone(),
            2,
        );
        
        // Register wallet
        {
            let mut reg = registry.write().await;
            assert!(reg.register_wallet(wallet.clone()).is_ok());
        }
        
        // Verify wallet type
        {
            let reg = registry.read().await;
            let found = reg.get_wallet(&wallet.address);
            assert!(found.is_some());
            match &found.unwrap().wallet_type {
                WalletType::SocialRecovery { guardians: g, .. } => {
                    assert_eq!(g.len(), guardians.len());
                }
                _ => panic!("Expected SocialRecovery wallet type"),
            }
        }
    }
}
