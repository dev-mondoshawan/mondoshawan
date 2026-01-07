//! Verkle Tree Implementation for Stateless Mode
//! 
//! Provides Verkle tree-backed state management with proof generation
//! for light client verification. Based on Ethereum's Verkle tree research.

pub mod tree;
pub mod proof;

pub use tree::{VerkleTree, VerkleState};
pub use proof::{StateProof, ProofVerifier};
