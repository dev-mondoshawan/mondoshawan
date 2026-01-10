//! Privacy Manager
//!
//! Manages privacy operations, nullifiers, and commitments.

use crate::privacy::{Commitment, Nullifier, NullifierSet, PrivacyTransaction};
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
}

impl PrivacyManager {
    /// Create new privacy manager
    pub fn new(enabled: bool) -> Self {
        Self {
            nullifier_set: Arc::new(RwLock::new(NullifierSet::default())),
            commitments: Arc::new(RwLock::new(HashMap::new())),
            enabled,
        }
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

        // Verify proof (would use verifier here)
        // For now, just check nullifier
        
        // Extract nullifier from public inputs (simplified)
        // In production, would properly parse public inputs
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
}

impl Default for PrivacyManager {
    fn default() -> Self {
        Self::new(true)
    }
}
