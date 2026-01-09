//! Integration tests for Multi-Signature functionality

#[cfg(test)]
mod integration_tests {
    use crate::account_abstraction::{MultiSigTransaction, MultiSigManager, WalletFactory, WalletRegistry};
    use crate::blockchain::Transaction;
    use crate::types::Address;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[tokio::test]
    async fn test_multisig_transaction_flow() {
        // Setup: Create wallet registry and multi-sig wallet
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        let signers = vec![[2u8; 20], [3u8; 20], [4u8; 20]];
        let threshold = 2;
        
        let wallet = WalletFactory::create_multisig_wallet(owner, 0, signers.clone(), threshold).unwrap();
        registry.register_wallet(wallet.clone()).unwrap();
        
        // Create multi-sig manager
        let mut manager = MultiSigManager::new();
        
        // Create transaction
        let to = [5u8; 20];
        let tx = Transaction::new(wallet.address, to, 1000, 100, 0);
        
        // Create multi-sig transaction
        let mut multisig_tx = MultiSigTransaction::new(
            wallet.address,
            tx,
            signers.clone(),
            threshold
        ).unwrap();
        
        // Initially not ready
        assert!(!multisig_tx.is_ready());
        assert_eq!(multisig_tx.signature_count(), 0);
        
        // Add first signature
        assert!(multisig_tx.add_signature([2u8; 20], vec![1; 64], vec![2; 32]).is_ok());
        assert_eq!(multisig_tx.signature_count(), 1);
        assert!(!multisig_tx.is_ready()); // Still need one more
        
        // Add second signature
        assert!(multisig_tx.add_signature([3u8; 20], vec![3; 64], vec![4; 32]).is_ok());
        assert_eq!(multisig_tx.signature_count(), 2);
        assert!(multisig_tx.is_ready()); // Now ready!
    }

    #[tokio::test]
    async fn test_multisig_manager_tracking() {
        let mut manager = MultiSigManager::new();
        let wallet = [1u8; 20];
        let signers = vec![[2u8; 20], [3u8; 20]];
        let threshold = 2;
        
        // Create transaction
        let tx = Transaction::new(wallet, [5u8; 20], 1000, 100, 0);
        let multisig_tx = MultiSigTransaction::new(wallet, tx, signers, threshold).unwrap();
        
        // Add to manager
        manager.add_pending_transaction(multisig_tx);
        
        // Get pending transactions
        let pending = manager.get_pending_transactions(&wallet);
        assert_eq!(pending.len(), 1);
        
        // Add signature
        let tx_hash = pending[0].transaction.hash;
        assert!(manager.add_signature_to_pending(&wallet, &tx_hash, [2u8; 20], vec![1; 64], vec![2; 32]).is_ok());
        
        // Verify signature was added
        let pending = manager.get_pending_transactions(&wallet);
        assert_eq!(pending[0].signature_count(), 1);
    }

    #[tokio::test]
    async fn test_multisig_validation_errors() {
        let wallet = [1u8; 20];
        let signers = vec![[2u8; 20], [3u8; 20], [4u8; 20]];
        let threshold = 2;
        
        let tx = Transaction::new(wallet, [5u8; 20], 1000, 100, 0);
        let mut multisig_tx = MultiSigTransaction::new(wallet, tx, signers, threshold).unwrap();
        
        // Try to add signature from unknown signer
        assert!(multisig_tx.add_signature([99u8; 20], vec![1; 64], vec![2; 32]).is_err());
        
        // Add valid signature
        assert!(multisig_tx.add_signature([2u8; 20], vec![1; 64], vec![2; 32]).is_ok());
        
        // Try to add duplicate signature
        assert!(multisig_tx.add_signature([2u8; 20], vec![3; 64], vec![4; 32]).is_err());
    }

    #[tokio::test]
    async fn test_multisig_pending_signers() {
        let wallet = [1u8; 20];
        let signers = vec![[2u8; 20], [3u8; 20], [4u8; 20]];
        let threshold = 2;
        
        let tx = Transaction::new(wallet, [5u8; 20], 1000, 100, 0);
        let mut multisig_tx = MultiSigTransaction::new(wallet, tx, signers.clone(), threshold).unwrap();
        
        // Initially all signers are pending
        let pending = multisig_tx.pending_signers();
        assert_eq!(pending.len(), 3);
        
        // Add one signature
        multisig_tx.add_signature([2u8; 20], vec![1; 64], vec![2; 32]).unwrap();
        let pending = multisig_tx.pending_signers();
        assert_eq!(pending.len(), 2);
        assert!(!pending.contains(&[2u8; 20]));
        assert!(pending.contains(&[3u8; 20]));
        assert!(pending.contains(&[4u8; 20]));
        
        // Check signed_by
        let signed = multisig_tx.signed_by();
        assert_eq!(signed.len(), 1);
        assert_eq!(signed[0], [2u8; 20]);
    }
}
