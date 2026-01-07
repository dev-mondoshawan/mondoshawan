//! Verkle Tree Data Structure
//! 
//! Wide tree (256 children per node) with KZG-style commitments
//! for efficient state proofs

use crate::types::{Address, Hash};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;

/// Verkle tree node
#[derive(Debug, Clone)]
struct VerkleNode {
    /// Branching factor (256 for wide tree)
    width: usize,
    /// Child nodes (indexed by key byte)
    children: Vec<Option<Box<VerkleNode>>>,
    /// Values stored at this node (for leaf nodes)
    values: Vec<Option<Vec<u8>>>,
    /// Commitment hash for this node
    commitment: Option<Hash>,
}

impl VerkleNode {
    fn new(width: usize) -> Self {
        Self {
            width,
            children: vec![None; width],
            values: vec![None; width],
            commitment: None,
        }
    }
    
    /// Insert a key-value pair into the tree
    fn insert(&mut self, key: &[u8], value: Vec<u8>, depth: usize) {
        if depth >= key.len() {
            // Leaf node - store value
            let index = if key.is_empty() { 0 } else { key[0] as usize % self.width };
            self.values[index] = Some(value);
        } else {
            // Internal node - recurse
            let index = key[depth] as usize % self.width;
            if self.children[index].is_none() {
                self.children[index] = Some(Box::new(VerkleNode::new(self.width)));
            }
            if let Some(ref mut child) = self.children[index] {
                child.insert(key, value, depth + 1);
            }
        }
        // Update commitment after insertion
        self.update_commitment();
    }
    
    /// Get value for a key
    fn get(&self, key: &[u8], depth: usize) -> Option<Vec<u8>> {
        if depth >= key.len() {
            // Leaf node
            let index = if key.is_empty() { 0 } else { key[0] as usize % self.width };
            self.values[index].clone()
        } else {
            // Internal node - recurse
            let index = key[depth] as usize % self.width;
            self.children[index]
                .as_ref()
                .and_then(|child| child.get(key, depth + 1))
        }
    }
    
    /// Update commitment hash for this node
    fn update_commitment(&mut self) {
        let mut hasher = Keccak256::new();
        
        // Hash all child commitments
        for child in &self.children {
            if let Some(ref c) = child {
                if let Some(ref comm) = c.commitment {
                    hasher.update(comm);
                } else {
                    hasher.update(&[0u8; 32]);
                }
            } else {
                hasher.update(&[0u8; 32]);
            }
        }
        
        // Hash all values
        for value in &self.values {
            if let Some(ref v) = value {
                hasher.update(v);
            } else {
                hasher.update(&[0u8; 32]);
            }
        }
        
        let hash = hasher.finalize();
        let mut commitment = [0u8; 32];
        commitment.copy_from_slice(&hash);
        self.commitment = Some(commitment);
    }
    
    /// Get proof path for a key
    fn get_proof(&self, key: &[u8], depth: usize, proof: &mut Vec<Hash>) {
        if depth >= key.len() {
            // Leaf node - add sibling values to proof
            let index = if key.is_empty() { 0 } else { key[0] as usize % self.width };
            for (i, value) in self.values.iter().enumerate() {
                if i != index {
                    if let Some(ref v) = value {
                        let mut hasher = Keccak256::new();
                        hasher.update(v);
                        let hash = hasher.finalize();
                        let mut hash_bytes = [0u8; 32];
                        hash_bytes.copy_from_slice(&hash);
                        proof.push(hash_bytes);
                    }
                }
            }
        } else {
            // Internal node - recurse and add sibling commitments
            let index = key[depth] as usize % self.width;
            
            // Add sibling commitments to proof
            for (i, child) in self.children.iter().enumerate() {
                if i != index {
                    if let Some(ref c) = child {
                        if let Some(ref comm) = c.commitment {
                            proof.push(*comm);
                        }
                    }
                }
            }
            
            // Recurse into child
            if let Some(ref child) = self.children[index] {
                child.get_proof(key, depth + 1, proof);
            }
        }
    }
}

/// Verkle tree for state management
pub struct VerkleTree {
    root: VerkleNode,
    size: usize,
}

impl VerkleTree {
    /// Create a new Verkle tree
    pub fn new() -> Self {
        Self {
            root: VerkleNode::new(256), // 256-way branching
            size: 0,
        }
    }
    
    /// Insert a key-value pair
    pub fn insert(&mut self, key: &[u8], value: Vec<u8>) {
        self.root.insert(key, value, 0);
        self.size += 1;
    }
    
    /// Get value for a key
    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.root.get(key, 0)
    }
    
    /// Get proof for a key
    pub fn get_proof(&self, key: &[u8]) -> Vec<Hash> {
        let mut proof = Vec::new();
        self.root.get_proof(key, 0, &mut proof);
        proof
    }
    
    /// Get root hash (state root)
    pub fn root_hash(&self) -> Option<Hash> {
        self.root.commitment
    }
    
    /// Get tree size
    pub fn size(&self) -> usize {
        self.size
    }
}

impl Default for VerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Verkle-backed state manager
pub struct VerkleState {
    tree: VerkleTree,
    /// Cache for quick lookups (optional optimization)
    cache: HashMap<Address, (u128, u64)>, // (balance, nonce)
}

impl VerkleState {
    /// Create new Verkle state
    pub fn new() -> Self {
        Self {
            tree: VerkleTree::new(),
            cache: HashMap::new(),
        }
    }
    
    /// Set balance for an address
    pub fn set_balance(&mut self, address: Address, balance: u128) {
        // Store in Verkle tree
        let key = address;
        let mut value = Vec::with_capacity(24); // 16 bytes balance + 8 bytes nonce
        value.extend_from_slice(&balance.to_le_bytes());
        
        // Get existing nonce or use 0
        let nonce = self.cache.get(&address).map(|(_, n)| *n).unwrap_or(0);
        value.extend_from_slice(&nonce.to_le_bytes());
        
        self.tree.insert(&key, value);
        
        // Update cache
        let entry = self.cache.entry(address).or_insert((0, 0));
        entry.0 = balance;
    }
    
    /// Set nonce for an address
    pub fn set_nonce(&mut self, address: Address, nonce: u64) {
        // Store in Verkle tree
        let key = address;
        let mut value = Vec::with_capacity(24);
        
        // Get existing balance or use 0
        let balance = self.cache.get(&address).map(|(b, _)| *b).unwrap_or(0);
        value.extend_from_slice(&balance.to_le_bytes());
        value.extend_from_slice(&nonce.to_le_bytes());
        
        self.tree.insert(&key, value);
        
        // Update cache
        let entry = self.cache.entry(address).or_insert((0, 0));
        entry.1 = nonce;
    }
    
    /// Get balance for an address
    pub fn get_balance(&self, address: Address) -> u128 {
        // Check cache first
        if let Some((balance, _)) = self.cache.get(&address) {
            return *balance;
        }
        
        // Get from tree
        if let Some(value) = self.tree.get(&address) {
            if value.len() >= 16 {
                let mut bytes = [0u8; 16];
                bytes.copy_from_slice(&value[0..16]);
                return u128::from_le_bytes(bytes);
            }
        }
        
        0
    }
    
    /// Get nonce for an address
    pub fn get_nonce(&self, address: Address) -> u64 {
        // Check cache first
        if let Some((_, nonce)) = self.cache.get(&address) {
            return *nonce;
        }
        
        // Get from tree
        if let Some(value) = self.tree.get(&address) {
            if value.len() >= 24 {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&value[16..24]);
                return u64::from_le_bytes(bytes);
            }
        }
        
        0
    }
    
    /// Get balance with proof
    pub fn get_balance_with_proof(&self, address: Address) -> (u128, Vec<Hash>, Hash) {
        let balance = self.get_balance(address);
        let proof = self.tree.get_proof(&address);
        let root_hash = self.tree.root_hash().unwrap_or([0u8; 32]);
        (balance, proof, root_hash)
    }
    
    /// Get nonce with proof
    pub fn get_nonce_with_proof(&self, address: Address) -> (u64, Vec<Hash>, Hash) {
        let nonce = self.get_nonce(address);
        let proof = self.tree.get_proof(&address);
        let root_hash = self.tree.root_hash().unwrap_or([0u8; 32]);
        (nonce, proof, root_hash)
    }
    
    /// Get state root
    pub fn state_root(&self) -> Hash {
        self.tree.root_hash().unwrap_or([0u8; 32])
    }
    
    /// Get tree size
    pub fn size(&self) -> usize {
        self.tree.size()
    }
}

impl Default for VerkleState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_verkle_tree_insert_get() {
        let mut tree = VerkleTree::new();
        let key = b"test_key";
        let value = b"test_value".to_vec();
        
        tree.insert(key, value.clone());
        assert_eq!(tree.get(key), Some(value));
        assert_eq!(tree.size(), 1);
    }
    
    #[test]
    fn test_verkle_state() {
        let mut state = VerkleState::new();
        let address = [1u8; 20];
        
        state.set_balance(address, 1000);
        assert_eq!(state.get_balance(address), 1000);
        
        state.set_nonce(address, 5);
        assert_eq!(state.get_nonce(address), 5);
        assert_eq!(state.get_balance(address), 1000); // Balance preserved
    }
    
    #[test]
    fn test_proof_generation() {
        let mut state = VerkleState::new();
        let address = [1u8; 20];
        
        state.set_balance(address, 1000);
        let (balance, proof, root) = state.get_balance_with_proof(address);
        
        assert_eq!(balance, 1000);
        assert!(!proof.is_empty());
        assert_ne!(root, [0u8; 32]);
    }
}
