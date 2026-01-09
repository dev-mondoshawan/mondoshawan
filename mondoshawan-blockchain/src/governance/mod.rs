//! Governance and Node Longevity System
//! 
//! Implements node identity, hardware fingerprinting, and longevity tracking
//! for governance voting in Algorithm Rotation proposals.

pub mod node_identity;
pub mod longevity;
pub mod registry;

#[cfg(test)]
mod tests;

pub use node_identity::{NodeIdentity, HardwareFingerprint, ZkUniquenessProof};
pub use longevity::{NodeLongevity, ActivitySnapshot, ParticipationType};
pub use registry::{NodeRegistry, LongevityTracker};
