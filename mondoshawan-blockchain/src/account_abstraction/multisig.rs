//! Multi-Signature Validation for Smart Contract Wallets

use crate::types::Address;
use crate::blockchain::Transaction;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Multi-signature transaction signature
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MultiSigSignature {
    /// Signer address
    pub signer: Address,
    /// Signature bytes
    pub signature: Vec<u8>,
    /// Public key (for verification)
    pub public_key: Vec<u8>,
}

/// Multi-signature transaction payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigTransaction {
    /// Wallet address (contract wallet)
    pub wallet_address: Address,
    /// Transaction data (to, value, data, etc.)
    pub transaction: Transaction,
    /// Collected signatures
    pub signatures: Vec<MultiSigSignature>,
    /// Required threshold
    pub threshold: u8,
    /// Expected signers
    pub expected_signers: Vec<Address>,
}

/// Multi-signature validation result
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MultiSigValidationResult {
    /// Valid - has enough signatures
    Valid,
    /// Invalid - insufficient signatures
    InsufficientSignatures {
        required: u8,
        provided: u8,
    },
    /// Invalid - duplicate signer
    DuplicateSigner(Address),
    /// Invalid - unknown signer
    UnknownSigner(Address),
    /// Invalid - signature verification failed
    InvalidSignature(Address),
}

impl MultiSigTransaction {
    /// Create a new multi-signature transaction
    pub fn new(
        wallet_address: Address,
        transaction: Transaction,
        expected_signers: Vec<Address>,
        threshold: u8,
    ) -> Result<Self, String> {
        if threshold == 0 {
            return Err("Threshold cannot be zero".to_string());
        }
        if threshold > expected_signers.len() as u8 {
            return Err("Threshold cannot exceed number of signers".to_string());
        }
        if expected_signers.is_empty() {
            return Err("Expected signers list cannot be empty".to_string());
        }

        Ok(Self {
            wallet_address,
            transaction,
            signatures: Vec::new(),
            threshold,
            expected_signers,
        })
    }

    /// Add a signature to the transaction
    pub fn add_signature(
        &mut self,
        signer: Address,
        signature: Vec<u8>,
        public_key: Vec<u8>,
    ) -> Result<(), String> {
        // Check if signer is in expected list
        if !self.expected_signers.contains(&signer) {
            return Err(format!("Signer {} is not in expected signers list", hex::encode(signer)));
        }

        // Check for duplicate signatures from same signer
        if self.signatures.iter().any(|s| s.signer == signer) {
            return Err(format!("Duplicate signature from signer {}", hex::encode(signer)));
        }

        // Add signature
        self.signatures.push(MultiSigSignature {
            signer,
            signature,
            public_key,
        });

        Ok(())
    }

    /// Validate the multi-signature transaction
    pub fn validate(&self) -> MultiSigValidationResult {
        // Check if we have enough signatures
        if self.signatures.len() < self.threshold as usize {
            return MultiSigValidationResult::InsufficientSignatures {
                required: self.threshold,
                provided: self.signatures.len() as u8,
            };
        }

        // Check for duplicate signers
        let mut seen_signers = HashSet::new();
        for sig in &self.signatures {
            if seen_signers.contains(&sig.signer) {
                return MultiSigValidationResult::DuplicateSigner(sig.signer);
            }
            seen_signers.insert(sig.signer);
        }

        // Verify all signers are in expected list
        for sig in &self.signatures {
            if !self.expected_signers.contains(&sig.signer) {
                return MultiSigValidationResult::UnknownSigner(sig.signer);
            }
        }

        // Verify all signatures cryptographically
        let tx_hash = self.transaction.hash;
        for sig in &self.signatures {
            // Basic validation: signature should not be empty
            if sig.signature.is_empty() {
                return MultiSigValidationResult::InvalidSignature(sig.signer);
            }

            // Verify Ed25519 signature
            if !verify_ed25519_signature(&tx_hash, &sig.signature, &sig.public_key) {
                return MultiSigValidationResult::InvalidSignature(sig.signer);
            }
        }

        MultiSigValidationResult::Valid
    }

    /// Check if transaction has enough signatures to be executed
    pub fn is_ready(&self) -> bool {
        matches!(self.validate(), MultiSigValidationResult::Valid)
    }

    /// Get number of signatures collected
    pub fn signature_count(&self) -> usize {
        self.signatures.len()
    }

    /// Get list of signers who have signed
    pub fn signed_by(&self) -> Vec<Address> {
        self.signatures.iter().map(|s| s.signer).collect()
    }

    /// Get list of signers who haven't signed yet
    pub fn pending_signers(&self) -> Vec<Address> {
        let signed: HashSet<Address> = self.signatures.iter().map(|s| s.signer).collect();
        self.expected_signers
            .iter()
            .filter(|s| !signed.contains(*s))
            .copied()
            .collect()
    }
}

/// Multi-signature manager for handling multi-sig operations
pub struct MultiSigManager {
    /// Pending multi-sig transactions (wallet_address -> transaction)
    pending_transactions: std::collections::HashMap<Address, Vec<MultiSigTransaction>>,
}

impl MultiSigManager {
    /// Create new multi-signature manager
    pub fn new() -> Self {
        Self {
            pending_transactions: std::collections::HashMap::new(),
        }
    }

    /// Add a pending multi-sig transaction
    pub fn add_pending_transaction(&mut self, tx: MultiSigTransaction) {
        let wallet = tx.wallet_address;
        self.pending_transactions
            .entry(wallet)
            .or_insert_with(Vec::new)
            .push(tx);
    }

    /// Get pending transactions for a wallet
    pub fn get_pending_transactions(&self, wallet_address: &Address) -> Vec<&MultiSigTransaction> {
        self.pending_transactions
            .get(wallet_address)
            .map(|txs| txs.iter().collect())
            .unwrap_or_default()
    }

    /// Add signature to a pending transaction
    pub fn add_signature_to_pending(
        &mut self,
        wallet_address: &Address,
        tx_hash: &[u8; 32],
        signer: Address,
        signature: Vec<u8>,
        public_key: Vec<u8>,
    ) -> Result<(), String> {
        let transactions = self.pending_transactions
            .get_mut(wallet_address)
            .ok_or_else(|| "No pending transactions for wallet".to_string())?;

        // Find transaction by hash
        let tx = transactions
            .iter_mut()
            .find(|tx| tx.transaction.hash == *tx_hash)
            .ok_or_else(|| "Transaction not found".to_string())?;

        tx.add_signature(signer, signature, public_key)
    }

    /// Remove completed transactions
    pub fn remove_completed(&mut self, wallet_address: &Address, tx_hash: &[u8; 32]) {
        if let Some(transactions) = self.pending_transactions.get_mut(wallet_address) {
            transactions.retain(|tx| tx.transaction.hash != *tx_hash);
        }
    }

    /// Get all pending transactions
    pub fn get_all_pending(&self) -> Vec<&MultiSigTransaction> {
        self.pending_transactions
            .values()
            .flatten()
            .collect()
    }
}

impl Default for MultiSigManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::Transaction;

    #[test]
    fn test_multisig_transaction_creation() {
        let wallet = [1u8; 20];
        let to = [2u8; 20];
        let tx = Transaction::new(wallet, to, 1000, 100, 0);
        let signers = vec![[3u8; 20], [4u8; 20], [5u8; 20]];
        let threshold = 2;

        let multisig_tx = MultiSigTransaction::new(wallet, tx, signers.clone(), threshold).unwrap();

        assert_eq!(multisig_tx.wallet_address, wallet);
        assert_eq!(multisig_tx.threshold, 2);
        assert_eq!(multisig_tx.expected_signers, signers);
        assert_eq!(multisig_tx.signature_count(), 0);
    }

    #[test]
    fn test_multisig_add_signatures() {
        let wallet = [1u8; 20];
        let to = [2u8; 20];
        let tx = Transaction::new(wallet, to, 1000, 100, 0);
        let signers = vec![[3u8; 20], [4u8; 20], [5u8; 20]];
        let threshold = 2;

        let mut multisig_tx = MultiSigTransaction::new(wallet, tx, signers, threshold).unwrap();

        // Add first signature
        assert!(multisig_tx.add_signature([3u8; 20], vec![1, 2, 3], vec![4, 5, 6]).is_ok());
        assert_eq!(multisig_tx.signature_count(), 1);
        assert!(!multisig_tx.is_ready()); // Not enough signatures yet

        // Add second signature
        assert!(multisig_tx.add_signature([4u8; 20], vec![7, 8, 9], vec![10, 11, 12]).is_ok());
        assert_eq!(multisig_tx.signature_count(), 2);
        assert!(multisig_tx.is_ready()); // Now has enough signatures
    }

    #[test]
    fn test_multisig_duplicate_signature() {
        let wallet = [1u8; 20];
        let to = [2u8; 20];
        let tx = Transaction::new(wallet, to, 1000, 100, 0);
        let signers = vec![[3u8; 20], [4u8; 20]];
        let threshold = 2;

        let mut multisig_tx = MultiSigTransaction::new(wallet, tx, signers, threshold).unwrap();

        // Add signature
        assert!(multisig_tx.add_signature([3u8; 20], vec![1, 2, 3], vec![4, 5, 6]).is_ok());

        // Try to add duplicate
        assert!(multisig_tx.add_signature([3u8; 20], vec![7, 8, 9], vec![10, 11, 12]).is_err());
    }

    #[test]
    fn test_multisig_unknown_signer() {
        let wallet = [1u8; 20];
        let to = [2u8; 20];
        let tx = Transaction::new(wallet, to, 1000, 100, 0);
        let signers = vec![[3u8; 20], [4u8; 20]];
        let threshold = 2;

        let mut multisig_tx = MultiSigTransaction::new(wallet, tx, signers, threshold).unwrap();

        // Try to add signature from unknown signer
        assert!(multisig_tx.add_signature([99u8; 20], vec![1, 2, 3], vec![4, 5, 6]).is_err());
    }

    #[test]
    fn test_multisig_validation() {
        let wallet = [1u8; 20];
        let to = [2u8; 20];
        let tx = Transaction::new(wallet, to, 1000, 100, 0);
        let signers = vec![[3u8; 20], [4u8; 20], [5u8; 20]];
        let threshold = 2;

        let mut multisig_tx = MultiSigTransaction::new(wallet, tx, signers, threshold).unwrap();

        // Not enough signatures
        assert!(matches!(
            multisig_tx.validate(),
            MultiSigValidationResult::InsufficientSignatures { .. }
        ));

        // Add one signature - still not enough
        multisig_tx.add_signature([3u8; 20], vec![1, 2, 3], vec![4, 5, 6]).unwrap();
        assert!(matches!(
            multisig_tx.validate(),
            MultiSigValidationResult::InsufficientSignatures { .. }
        ));

        // Add second signature - now valid
        multisig_tx.add_signature([4u8; 20], vec![7, 8, 9], vec![10, 11, 12]).unwrap();
        assert!(matches!(multisig_tx.validate(), MultiSigValidationResult::Valid));
    }

    #[test]
    fn test_multisig_pending_signers() {
        let wallet = [1u8; 20];
        let to = [2u8; 20];
        let tx = Transaction::new(wallet, to, 1000, 100, 0);
        let signers = vec![[3u8; 20], [4u8; 20], [5u8; 20]];
        let threshold = 2;

        let mut multisig_tx = MultiSigTransaction::new(wallet, tx, signers.clone(), threshold).unwrap();

        // Initially all are pending
        let pending = multisig_tx.pending_signers();
        assert_eq!(pending.len(), 3);

        // Add one signature
        multisig_tx.add_signature([3u8; 20], vec![1, 2, 3], vec![4, 5, 6]).unwrap();
        let pending = multisig_tx.pending_signers();
        assert_eq!(pending.len(), 2);
        assert!(!pending.contains(&[3u8; 20]));
        assert!(pending.contains(&[4u8; 20]));
        assert!(pending.contains(&[5u8; 20]));
    }
}

/// Verify Ed25519 signature
fn verify_ed25519_signature(message: &[u8; 32], signature: &[u8], public_key: &[u8]) -> bool {
    use ed25519_dalek::{VerifyingKey, Signature};
    
    // Convert public key
    if public_key.len() != 32 {
        return false;
    }
    let pk_bytes: [u8; 32] = match public_key.try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };
    
    let verifying_key = match VerifyingKey::from_bytes(&pk_bytes) {
        Ok(k) => k,
        Err(_) => return false,
    };
    
    // Convert signature
    if signature.len() != 64 {
        return false;
    }
    let sig_bytes: [u8; 64] = match signature.try_into() {
        Ok(b) => b,
        Err(_) => return false,
    };
    
    let sig = Signature::from_bytes(&sig_bytes);
    
    // Verify signature
    verifying_key.verify_strict(message, &sig).is_ok()
}
