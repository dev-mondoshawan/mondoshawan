//! Light Client Implementation
//! 
//! Provides stateless client mode using Verkle state roots and proofs.
//! Light clients can verify state without storing the full blockchain state.

use crate::types::{Address, Hash};
use crate::verkle::{StateProof, ProofVerifier};
use std::collections::HashMap;

/// Light client that only stores state roots and verifies proofs
pub struct LightClient {
    /// Current state root
    current_state_root: Option<Hash>,
    /// State root history (block number -> state root)
    state_root_history: HashMap<u64, Hash>,
    /// Latest verified block number
    latest_verified_block: u64,
}

impl LightClient {
    /// Create a new light client
    pub fn new() -> Self {
        Self {
            current_state_root: None,
            state_root_history: HashMap::new(),
            latest_verified_block: 0,
        }
    }
    
    /// Update state root (called when syncing with full node)
    pub fn update_state_root(&mut self, block_number: u64, state_root: Hash) {
        self.current_state_root = Some(state_root);
        self.state_root_history.insert(block_number, state_root);
        if block_number > self.latest_verified_block {
            self.latest_verified_block = block_number;
        }
    }
    
    /// Get current state root
    pub fn current_state_root(&self) -> Option<Hash> {
        self.current_state_root
    }
    
    /// Get state root for a specific block
    pub fn get_state_root(&self, block_number: u64) -> Option<Hash> {
        self.state_root_history.get(&block_number).copied()
    }
    
    /// Verify balance proof
    pub fn verify_balance(&self, address: Address, balance: u128, proof: &StateProof) -> bool {
        // Check that proof's state root matches our current state root
        if let Some(current_root) = self.current_state_root {
            if proof.state_root != current_root {
                return false;
            }
        }
        
        // Verify the proof using static method
        ProofVerifier::verify_balance_proof(address, balance, proof)
    }
    
    /// Verify nonce proof
    pub fn verify_nonce(&self, address: Address, nonce: u64, proof: &StateProof) -> bool {
        // Check that proof's state root matches our current state root
        if let Some(current_root) = self.current_state_root {
            if proof.state_root != current_root {
                return false;
            }
        }
        
        // Verify the proof using static method
        ProofVerifier::verify_nonce_proof(address, nonce, proof)
    }
    
    /// Get latest verified block number
    pub fn latest_verified_block(&self) -> u64 {
        self.latest_verified_block
    }
    
    /// Check if light client is synced (has a state root)
    pub fn is_synced(&self) -> bool {
        self.current_state_root.is_some()
    }
    
    /// Get sync status
    pub fn sync_status(&self) -> LightClientSyncStatus {
        LightClientSyncStatus {
            is_synced: self.is_synced(),
            latest_block: self.latest_verified_block,
            current_state_root: self.current_state_root,
            state_root_count: self.state_root_history.len(),
        }
    }
}

impl Default for LightClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Light client sync status
#[derive(Debug, Clone)]
pub struct LightClientSyncStatus {
    pub is_synced: bool,
    pub latest_block: u64,
    pub current_state_root: Option<Hash>,
    pub state_root_count: usize,
}

/// Light client configuration
#[derive(Debug, Clone)]
pub struct LightClientConfig {
    /// Enable light client mode
    pub enabled: bool,
    /// Trusted full node RPC endpoint
    pub trusted_rpc_endpoint: Option<String>,
    /// Sync interval in seconds
    pub sync_interval: u64,
}

impl Default for LightClientConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            trusted_rpc_endpoint: None,
            sync_interval: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_light_client_creation() {
        let client = LightClient::new();
        assert!(!client.is_synced());
        assert_eq!(client.latest_verified_block(), 0);
    }
    
    #[test]
    fn test_state_root_update() {
        let mut client = LightClient::new();
        let state_root = [1u8; 32];
        
        client.update_state_root(100, state_root);
        assert!(client.is_synced());
        assert_eq!(client.latest_verified_block(), 100);
        assert_eq!(client.current_state_root(), Some(state_root));
    }
}
