//! Post-Quantum Account Types
//! 
//! Supports Dilithium and SPHINCS+ signature schemes for quantum-resistant transactions

use crate::types::{Address, Hash};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// PQ account type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PqAccountType {
    /// Dilithium3 (recommended for most use cases)
    Dilithium3,
    /// SPHINCS+-SHA256-128f-simple (smaller signatures, slower)
    SphincsPlus,
    /// Traditional Ed25519 (for backward compatibility)
    Ed25519,
}

impl PqAccountType {
    /// Get signature size in bytes
    pub fn signature_size(&self) -> usize {
        match self {
            PqAccountType::Dilithium3 => 3293, // Dilithium3 signature size
            PqAccountType::SphincsPlus => 7856, // SPHINCS+ signature size
            PqAccountType::Ed25519 => 64,
        }
    }
    
    /// Get public key size in bytes
    pub fn public_key_size(&self) -> usize {
        match self {
            PqAccountType::Dilithium3 => 1952, // Dilithium3 public key size
            PqAccountType::SphincsPlus => 32, // SPHINCS+ public key size
            PqAccountType::Ed25519 => 32,
        }
    }
}

/// PQ signature wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqSignature {
    /// Account type used for this signature
    pub account_type: PqAccountType,
    /// Signature bytes (size depends on account_type)
    pub signature: Vec<u8>,
    /// Public key bytes (size depends on account_type)
    pub public_key: Vec<u8>,
}

impl PqSignature {
    /// Create a new PQ signature
    pub fn new(account_type: PqAccountType, signature: Vec<u8>, public_key: Vec<u8>) -> Self {
        Self {
            account_type,
            signature,
            public_key,
        }
    }
    
    /// Verify signature size matches account type
    pub fn verify_size(&self) -> bool {
        self.signature.len() == self.account_type.signature_size() &&
        self.public_key.len() == self.account_type.public_key_size()
    }
}

/// Post-Quantum Account
/// 
/// Manages PQ keypairs and signing operations
pub struct PqAccount {
    account_type: PqAccountType,
    secret_key: Vec<u8>,
    public_key: Vec<u8>,
    address: Address,
}

impl PqAccount {
    /// Generate a new Dilithium3 account
    pub fn new_dilithium3() -> Self {
        // TODO: Fix pqcrypto API compatibility
        // For now, return a placeholder
        Self {
            account_type: PqAccountType::Dilithium3,
            secret_key: vec![0; 4000],
            public_key: vec![0; 1952],
            address: [0; 20],
        }
    }
    
    /// Generate a new SPHINCS+ account
    pub fn new_sphincsplus() -> Self {
        // TODO: Fix pqcrypto API compatibility
        // For now, return a placeholder
        Self {
            account_type: PqAccountType::SphincsPlus,
            secret_key: vec![0; 64],
            public_key: vec![0; 32],
            address: [0; 20],
        }
    }
    
    /// Create account from existing keypair
    pub fn from_keypair(account_type: PqAccountType, secret_key: Vec<u8>, public_key: Vec<u8>) -> Result<Self, String> {
        // Simple validation: just check sizes match expected
        let expected_public_size = account_type.public_key_size();
        
        if public_key.len() != expected_public_size {
            return Err(format!("Invalid public key size for {:?}: expected {}, got {}", account_type, expected_public_size, public_key.len()));
        }
        
        // For PQ keys, we just store the bytes directly since pqcrypto types
        // don't support round-trip serialization reliably
        
        let address = Self::derive_address(&public_key, account_type);
        
        Ok(Self {
            account_type,
            secret_key,
            public_key,
            address,
        })
    }
    
    /// Sign a message hash
    pub fn sign(&self, message_hash: &Hash) -> PqSignature {
        // TODO: Fix pqcrypto API compatibility
        // For now, return empty signatures
        PqSignature::new(
            self.account_type,
            vec![],
            self.public_key.clone(),
        )
    }
    
    /// Verify a signature
    pub fn verify_signature(_message_hash: &Hash, _signature: &PqSignature) -> bool {
        // TODO: Fix pqcrypto API compatibility
        // For now, always return false (PQ not fully functional)
        false
    }
    
    /// Derive address from public key
    fn derive_address(public_key: &[u8], account_type: PqAccountType) -> Address {
        let mut hasher = Keccak256::new();
        // Include account type in hash to distinguish PQ accounts
        match account_type {
            PqAccountType::Dilithium3 => hasher.update(b"DILITHIUM3"),
            PqAccountType::SphincsPlus => hasher.update(b"SPHINCS+"),
            PqAccountType::Ed25519 => hasher.update(b"ED25519"),
        };
        hasher.update(public_key);
        let hash = hasher.finalize();
        
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..32]);
        address
    }
    
    /// Get account type
    pub fn account_type(&self) -> PqAccountType {
        self.account_type
    }
    
    /// Get public key
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
    
    /// Get address
    pub fn address(&self) -> Address {
        self.address
    }
    
    /// Get secret key (use with caution!)
    pub fn secret_key(&self) -> &[u8] {
        &self.secret_key
    }
}

impl PqAccountType {
    /// Get secret key size in bytes
    pub fn secret_key_size(&self) -> usize {
        match self {
            PqAccountType::Dilithium3 => 4000, // Dilithium3 secret key size
            PqAccountType::SphincsPlus => 64, // SPHINCS+ secret key size
            PqAccountType::Ed25519 => 32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dilithium3_account() {
        let account = PqAccount::new_dilithium3();
        assert_eq!(account.account_type(), PqAccountType::Dilithium3);
        assert_eq!(account.public_key().len(), PqAccountType::Dilithium3.public_key_size());
        
        let message_hash = [1u8; 32];
        let signature = account.sign(&message_hash);
        assert!(PqAccount::verify_signature(&message_hash, &signature));
    }
    
    #[test]
    fn test_sphincsplus_account() {
        let account = PqAccount::new_sphincsplus();
        assert_eq!(account.account_type(), PqAccountType::SphincsPlus);
        assert_eq!(account.public_key().len(), PqAccountType::SphincsPlus.public_key_size());
        
        let message_hash = [1u8; 32];
        let signature = account.sign(&message_hash);
        assert!(PqAccount::verify_signature(&message_hash, &signature));
    }
}
