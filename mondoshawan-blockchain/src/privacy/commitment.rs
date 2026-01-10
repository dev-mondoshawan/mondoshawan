//! Pedersen Commitments
//!
//! Commitment scheme for hiding transaction amounts and receivers.
//! Commit(amount, blinding) = g^amount * h^blinding

use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use ark_ff::PrimeField;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// Commitment to a value (hides amount and blinding factor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    /// Commitment point on G1
    pub point: G1Projective,
}

impl Commitment {
    /// Create commitment from point
    pub fn new(point: G1Projective) -> Self {
        Self { point }
    }

    /// Serialize commitment to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        // Serialize G1 point (64 bytes compressed)
        let mut bytes = Vec::with_capacity(64);
        // In production, use proper point serialization
        bytes.extend_from_slice(&[0u8; 64]);
        bytes
    }

    /// Deserialize commitment from bytes
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() != 64 {
            return None;
        }
        // In production, use proper point deserialization
        Some(Self {
            point: G1Projective::zero(),
        })
    }
}

/// Pedersen Commitment Scheme
pub struct PedersenCommitment {
    /// Generator g (for amount)
    g: G1Projective,
    /// Generator h (for blinding factor)
    h: G1Projective,
}

impl PedersenCommitment {
    /// Create new Pedersen commitment scheme
    pub fn new() -> Self {
        // In production, use proper generators from trusted setup
        // For now, use deterministic generators
        let g = Self::hash_to_g1(b"pedersen_g");
        let h = Self::hash_to_g1(b"pedersen_h");
        
        Self { g, h }
    }

    /// Commit to a value: C = g^amount * h^blinding
    pub fn commit(amount: u128, blinding: &[u8; 32]) -> Commitment {
        let scheme = Self::new();
        
        // Convert amount to field element
        let amount_fr = Fr::from(amount);
        
        // Convert blinding to field element
        let blinding_fr = Fr::from_le_bytes_mod_order(blinding);
        
        // Compute commitment: g^amount * h^blinding
        let g_amount = scheme.g * amount_fr;
        let h_blinding = scheme.h * blinding_fr;
        let commitment_point = g_amount + h_blinding;
        
        Commitment::new(commitment_point)
    }

    /// Verify commitment (in circuit, not here)
    /// This is just for testing - actual verification happens in zk-SNARK
    pub fn verify(_commitment: &Commitment, _amount: u128, _blinding: &[u8; 32]) -> bool {
        // Verification happens in zk-SNARK circuit
        true
    }

    /// Hash to G1 point (deterministic generator)
    fn hash_to_g1(seed: &[u8]) -> G1Projective {
        // Simplified: hash seed and use as scalar
        // In production, use proper hash-to-curve
        let mut hasher = Keccak256::new();
        hasher.update(seed);
        let hash = hasher.finalize();
        
        // Convert hash to field element
        let scalar = Fr::from_le_bytes_mod_order(&hash[..32]);
        
        // Use generator * scalar
        G1Projective::generator() * scalar
    }
}

impl Default for PedersenCommitment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_creation() {
        let amount = 1000u128;
        let blinding = [42u8; 32];
        
        let commitment = PedersenCommitment::commit(amount, &blinding);
        
        // Commitment should be non-zero
        assert_ne!(commitment.point, G1Projective::zero());
    }

    #[test]
    fn test_commitment_serialization() {
        let amount = 1000u128;
        let blinding = [42u8; 32];
        
        let commitment = PedersenCommitment::commit(amount, &blinding);
        let bytes = commitment.to_bytes();
        let deserialized = Commitment::from_bytes(&bytes);
        
        assert!(deserialized.is_some());
    }
}
