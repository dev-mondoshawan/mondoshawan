//! Nullifier System
//!
//! Prevents double-spending of privacy notes.
//! Nullifier = hash(receiver_secret, note_index)

use crate::types::Hash;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashSet;

/// Nullifier (prevents double-spending)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Nullifier {
    /// Nullifier hash
    pub hash: Hash,
}

impl Nullifier {
    /// Generate nullifier from receiver address and blinding factor
    pub fn generate(receiver: &crate::types::Address, blinding: &[u8; 32]) -> Self {
        let mut hasher = Keccak256::new();
        hasher.update(receiver);
        hasher.update(blinding);
        let hash = hasher.finalize();
        
        let mut nullifier_hash = [0u8; 32];
        nullifier_hash.copy_from_slice(&hash);
        
        Self {
            hash: nullifier_hash,
        }
    }

    /// Convert to bytes
    pub fn to_bytes(&self) -> &[u8; 32] {
        &self.hash
    }
}

/// Nullifier Set (tracks spent notes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NullifierSet {
    /// Set of spent nullifiers
    nullifiers: HashSet<Hash>,
    /// Maximum size (for memory management)
    max_size: usize,
}

impl NullifierSet {
    /// Create new nullifier set
    pub fn new(max_size: usize) -> Self {
        Self {
            nullifiers: HashSet::new(),
            max_size,
        }
    }

    /// Check if nullifier exists (already spent)
    pub fn contains(&self, nullifier: &Nullifier) -> bool {
        self.nullifiers.contains(&nullifier.hash)
    }

    /// Add nullifier (mark as spent)
    pub fn add(&mut self, nullifier: Nullifier) -> Result<(), String> {
        if self.nullifiers.len() >= self.max_size {
            return Err("Nullifier set is full".to_string());
        }
        
        if self.nullifiers.contains(&nullifier.hash) {
            return Err("Nullifier already exists (double-spend attempt)".to_string());
        }
        
        self.nullifiers.insert(nullifier.hash);
        Ok(())
    }

    /// Remove nullifier (for testing/cleanup)
    pub fn remove(&mut self, nullifier: &Nullifier) {
        self.nullifiers.remove(&nullifier.hash);
    }

    /// Get count of nullifiers
    pub fn len(&self) -> usize {
        self.nullifiers.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.nullifiers.is_empty()
    }
}

impl Default for NullifierSet {
    fn default() -> Self {
        Self::new(1_000_000) // Default: 1M nullifiers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nullifier_generation() {
        let receiver = [1u8; 20];
        let blinding = [42u8; 32];
        
        let nullifier = Nullifier::generate(&receiver, &blinding);
        
        // Nullifier should be non-zero
        assert_ne!(nullifier.hash, [0u8; 32]);
    }

    #[test]
    fn test_nullifier_set() {
        let mut set = NullifierSet::new(100);
        let receiver = [1u8; 20];
        let blinding = [42u8; 32];
        
        let nullifier = Nullifier::generate(&receiver, &blinding);
        
        // Should not exist initially
        assert!(!set.contains(&nullifier));
        
        // Add nullifier
        set.add(nullifier.clone()).unwrap();
        
        // Should exist now
        assert!(set.contains(&nullifier));
        
        // Adding again should fail (double-spend)
        assert!(set.add(nullifier).is_err());
    }
}
