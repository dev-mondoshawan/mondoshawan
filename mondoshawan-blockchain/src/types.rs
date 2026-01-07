//! Common types used throughout the blockchain

use serde::{Deserialize, Serialize};

/// 20-byte Ethereum-style address
pub type Address = [u8; 20];

/// 32-byte hash
pub type Hash = [u8; 32];

/// Mining stream types for TriStream architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StreamType {
    /// Stream A: ASIC mining (Blake3, 10s blocks)
    StreamA,
    /// Stream B: CPU/GPU mining (KHeavyHash, 1s blocks)
    StreamB,
    /// Stream C: ZK proofs (100ms blocks)
    StreamC,
}

/// Mining difficulty
pub type Difficulty = u64;

