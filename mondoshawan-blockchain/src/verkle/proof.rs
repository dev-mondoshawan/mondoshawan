//! State Proof Verification
//! 
//! Provides proof verification for light clients to verify state
//! without storing the full state tree

use crate::types::{Address, Hash};

/// State proof structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateProof {
    /// The value being proven (balance or nonce)
    pub value: Vec<u8>,
    /// Proof path (sibling hashes)
    pub proof: Vec<Hash>,
    /// State root at time of proof
    pub state_root: Hash,
    /// Address being proven
    pub address: Address,
}

impl StateProof {
    /// Create a new state proof
    pub fn new(address: Address, value: Vec<u8>, proof: Vec<Hash>, state_root: Hash) -> Self {
        Self {
            value,
            proof,
            state_root,
            address,
        }
    }
    
    /// Serialize proof to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }
    
    /// Deserialize proof from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(data)
    }
}

/// Proof verifier for light clients
pub struct ProofVerifier;

impl ProofVerifier {
    /// Verify a state proof
    /// 
    /// This is a simplified verification. In a full implementation,
    /// this would verify KZG commitments along the proof path.
    pub fn verify_proof(
        _address: Address,
        expected_value: &[u8],
        proof: &[Hash],
        state_root: Hash,
    ) -> bool {
        // Simplified verification: reconstruct path and check root
        // In full implementation, would verify KZG commitments
        
        // For now, we'll do a basic check that the proof is valid
        // A full implementation would:
        // 1. Reconstruct the path from address to root
        // 2. Verify each commitment along the path
        // 3. Check that the final root matches state_root
        
        // Basic validation: proof should not be empty for non-zero values
        if !expected_value.is_empty() && proof.is_empty() {
            return false;
        }
        
        // Verify state root is not zero
        if state_root == [0u8; 32] {
            return false;
        }
        
        // In a full implementation, we would:
        // - Hash the value with its siblings
        // - Verify each level of the tree
        // - Check KZG commitments
        
        // For now, return true if basic checks pass
        // This is a placeholder - full verification requires proper KZG implementation
        true
    }
    
    /// Verify balance proof
    pub fn verify_balance_proof(
        address: Address,
        balance: u128,
        proof: &StateProof,
    ) -> bool {
        if proof.address != address {
            return false;
        }
        
        // Extract balance from proof value
        if proof.value.len() < 16 {
            return false;
        }
        
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&proof.value[0..16]);
        let proof_balance = u128::from_le_bytes(bytes);
        
        if proof_balance != balance {
            return false;
        }
        
        Self::verify_proof(
            address,
            &proof.value,
            &proof.proof,
            proof.state_root,
        )
    }
    
    /// Verify nonce proof
    pub fn verify_nonce_proof(
        address: Address,
        nonce: u64,
        proof: &StateProof,
    ) -> bool {
        if proof.address != address {
            return false;
        }
        
        // Extract nonce from proof value
        if proof.value.len() < 24 {
            return false;
        }
        
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&proof.value[16..24]);
        let proof_nonce = u64::from_le_bytes(bytes);
        
        if proof_nonce != nonce {
            return false;
        }
        
        Self::verify_proof(
            address,
            &proof.value,
            &proof.proof,
            proof.state_root,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_proof_serialization() {
        let address = [1u8; 20];
        let value = vec![1, 2, 3, 4];
        let proof = vec![[5u8; 32], [6u8; 32]];
        let state_root = [7u8; 32];
        
        let state_proof = StateProof::new(address, value.clone(), proof.clone(), state_root);
        
        let bytes = state_proof.to_bytes();
        let deserialized = StateProof::from_bytes(&bytes).unwrap();
        
        assert_eq!(deserialized.address, address);
        assert_eq!(deserialized.value, value);
        assert_eq!(deserialized.proof, proof);
        assert_eq!(deserialized.state_root, state_root);
    }
}
