//! Privacy Manager
//!
//! Manages privacy operations, nullifiers, and commitments.

use crate::privacy::{Commitment, Nullifier, NullifierSet, PrivacyTransaction, PrivacyVerifier};
use crate::types::Address;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Privacy Manager
pub struct PrivacyManager {
    /// Nullifier set (tracks spent notes)
    nullifier_set: Arc<RwLock<NullifierSet>>,
    /// Commitment to note mapping (for tracking)
    commitments: Arc<RwLock<HashMap<Commitment, Address>>>,
    /// Privacy enabled flag
    enabled: bool,
    /// Privacy verifier (for proof verification)
    verifier: Option<Arc<PrivacyVerifier>>,
}

impl PrivacyManager {
    /// Create new privacy manager
    pub fn new(enabled: bool) -> Self {
        Self {
            nullifier_set: Arc::new(RwLock::new(NullifierSet::default())),
            commitments: Arc::new(RwLock::new(HashMap::new())),
            enabled,
            verifier: None,
        }
    }

    /// Create privacy manager with verifier
    pub fn with_verifier(enabled: bool, verifier: PrivacyVerifier) -> Self {
        Self {
            nullifier_set: Arc::new(RwLock::new(NullifierSet::default())),
            commitments: Arc::new(RwLock::new(HashMap::new())),
            enabled,
            verifier: Some(Arc::new(verifier)),
        }
    }

    /// Set verifier
    pub fn set_verifier(&mut self, verifier: PrivacyVerifier) {
        self.verifier = Some(Arc::new(verifier));
    }

    /// Check if privacy is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Check if nullifier exists (already spent)
    pub async fn is_nullifier_spent(&self, nullifier: &Nullifier) -> bool {
        let set = self.nullifier_set.read().await;
        set.contains(nullifier)
    }

    /// Add nullifier (mark as spent)
    pub async fn add_nullifier(&self, nullifier: Nullifier) -> Result<(), String> {
        let mut set = self.nullifier_set.write().await;
        set.add(nullifier)
    }

    /// Process privacy transaction
    pub async fn process_transaction(
        &self,
        tx: &PrivacyTransaction,
    ) -> Result<(), String> {
        if !self.enabled {
            return Err("Privacy layer is disabled".to_string());
        }

        // Verify proof if verifier is available
        if let Some(ref verifier) = self.verifier {
            // Deserialize proof
            let proof = PrivacyVerifier::deserialize_proof(&tx.proof)
                .map_err(|e| format!("Failed to deserialize proof: {}", e))?;

            // Parse public inputs (nullifier, commitment)
            use ark_bn254::Fr;
            use ark_ff::PrimeField;
            let mut public_inputs = Vec::new();
            for input_bytes in &tx.public_inputs {
                if input_bytes.len() >= 32 {
                    let mut bytes = [0u8; 32];
                    bytes.copy_from_slice(&input_bytes[..32]);
                    let fr = Fr::from_le_bytes_mod_order(&bytes);
                    public_inputs.push(fr);
                }
            }

            // Verify proof
            if !verifier.verify_with_inputs(&proof, &public_inputs) {
                return Err("Privacy proof verification failed".to_string());
            }
        }
        
        // Extract nullifier from public inputs
        // Format: public_inputs[0] = nullifier
        if let Some(nullifier_bytes) = tx.public_inputs.get(0) {
            if nullifier_bytes.len() == 32 {
                let mut nullifier_hash = [0u8; 32];
                nullifier_hash.copy_from_slice(nullifier_bytes);
                let nullifier = Nullifier { hash: nullifier_hash };
                
                // Check if already spent
                if self.is_nullifier_spent(&nullifier).await {
                    return Err("Nullifier already spent (double-spend attempt)".to_string());
                }
                
                // Add nullifier to set (mark as spent)
                self.add_nullifier(nullifier).await?;
            }
        }
        
        Ok(())
    }

    /// Extract nullifier from privacy transaction
    pub fn extract_nullifier(tx: &PrivacyTransaction) -> Option<Nullifier> {
        if let Some(nullifier_bytes) = tx.public_inputs.get(0) {
            if nullifier_bytes.len() == 32 {
                let mut nullifier_hash = [0u8; 32];
                nullifier_hash.copy_from_slice(nullifier_bytes);
                return Some(Nullifier { hash: nullifier_hash });
            }
        }
        None
    }

    /// Get nullifier set size
    pub async fn nullifier_count(&self) -> usize {
        let set = self.nullifier_set.read().await;
        set.len()
    }

    /// Check if privacy is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for PrivacyManager {
    fn default() -> Self {
        Self::new(true)
    }
}
