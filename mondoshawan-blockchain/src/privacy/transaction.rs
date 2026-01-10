//! Privacy Transaction Types
//!
//! Defines transaction types for privacy operations.

use crate::types::{Address, Hash};
use serde::{Deserialize, Serialize};

/// Privacy transaction type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyTxType {
    /// Private transfer (hidden sender, receiver, amount)
    PrivateTransfer,
    /// Private balance query (prove balance without revealing amount)
    PrivateBalanceQuery,
    /// Private smart contract call
    PrivateContractCall,
}

/// Privacy Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyTransaction {
    /// Transaction type
    pub tx_type: PrivacyTxType,
    /// zk-SNARK proof
    pub proof: Vec<u8>,
    /// Public inputs (nullifier, commitment, etc.)
    pub public_inputs: Vec<Vec<u8>>,
    /// Transaction hash
    pub hash: Hash,
    /// Optional: encrypted data (for receiver to decrypt)
    pub encrypted_data: Option<Vec<u8>>,
}

impl PrivacyTransaction {
    /// Create new privacy transaction
    pub fn new(
        tx_type: PrivacyTxType,
        proof: Vec<u8>,
        public_inputs: Vec<Vec<u8>>,
    ) -> Self {
        // Calculate hash
        let mut hasher = sha3::Keccak256::new();
        hasher.update(&proof);
        for input in &public_inputs {
            hasher.update(input);
        }
        let hash = hasher.finalize();
        let mut hash_bytes = [0u8; 32];
        hash_bytes.copy_from_slice(&hash);

        Self {
            tx_type,
            proof,
            public_inputs,
            hash: hash_bytes,
            encrypted_data: None,
        }
    }

    /// Add encrypted data
    pub fn with_encrypted_data(mut self, data: Vec<u8>) -> Self {
        self.encrypted_data = Some(data);
        self
    }
}
