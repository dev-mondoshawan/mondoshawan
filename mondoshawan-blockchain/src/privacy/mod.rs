//! Privacy Layer with zk-SNARKs
//!
//! Native privacy transactions using zero-knowledge proofs.
//! Enables private transfers, private balance queries, and private smart contract calls.

pub mod circuit;
pub mod commitment;
pub mod keys;
pub mod manager;
pub mod merkle;
pub mod nullifier;
pub mod prover;
pub mod transaction;
pub mod verifier;

pub use circuit::{PrivateTransferCircuit, PrivacyCircuit};
pub use commitment::{Commitment, PedersenCommitment};
pub use keys::{generate_keys, load_keys_from_bytes, serialize_keys};
pub use manager::PrivacyManager;
pub use nullifier::{Nullifier, NullifierSet};
pub use prover::PrivacyProver;
pub use transaction::{PrivacyTransaction, PrivacyTxType};
pub use verifier::PrivacyVerifier;

use serde::{Deserialize, Serialize};

/// Privacy layer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Enable privacy layer
    pub enabled: bool,
    /// Curve to use for zk-SNARKs (BN254 or BLS12-381)
    pub curve: String,
    /// Merkle tree depth for private notes
    pub merkle_depth: usize,
    /// Maximum nullifiers to track
    pub max_nullifiers: usize,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            curve: "BN254".to_string(),
            merkle_depth: 20, // Supports 2^20 = 1M private notes
            max_nullifiers: 1_000_000,
        }
    }
}

/// Privacy note (represents a private balance)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyNote {
    /// Commitment to the note (hides amount and receiver)
    pub commitment: Commitment,
    /// Nullifier (prevents double-spending)
    pub nullifier: Nullifier,
    /// Amount (only known to sender/receiver)
    pub amount: u128,
    /// Receiver address (only known to receiver)
    pub receiver: crate::types::Address,
    /// Blinding factor (for commitment)
    pub blinding: [u8; 32],
}

impl PrivacyNote {
    /// Create a new privacy note
    pub fn new(
        amount: u128,
        receiver: crate::types::Address,
        blinding: [u8; 32],
    ) -> Self {
        let commitment = PedersenCommitment::commit(amount, &blinding);
        let nullifier = Nullifier::generate(&receiver, &blinding);
        
        Self {
            commitment,
            nullifier,
            amount,
            receiver,
            blinding,
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod benchmarks;

#[cfg(test)]
mod end_to_end_tests;
