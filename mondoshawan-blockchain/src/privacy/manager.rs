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
        
        // Add nullifier to set
        // This would happen after proof verification
        
        Ok(())
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
