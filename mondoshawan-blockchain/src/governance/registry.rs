//! Node Registry and Longevity Tracker

use crate::governance::node_identity::NodeIdentity;
use crate::governance::longevity::{NodeLongevity, ParticipationType};
use crate::blockchain::Blockchain;
use crate::types::Hash;
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Node registry for tracking all nodes and their longevity
pub struct NodeRegistry {
    /// Map of IP address to node identity
    ip_to_node: HashMap<IpAddr, NodeIdentity>,
    
    /// Map of node identity to stats
    node_stats: HashMap<NodeIdentity, NodeLongevity>,
    
    /// Set of registered hardware fingerprints
    fingerprints: HashSet<Hash>,
    
    /// Set of registered commitments (for ZK proofs)
    commitments: HashSet<Hash>,
}

impl NodeRegistry {
    /// Create new node registry
    pub fn new() -> Self {
        Self {
            ip_to_node: HashMap::new(),
            node_stats: HashMap::new(),
            fingerprints: HashSet::new(),
            commitments: HashSet::new(),
        }
    }
    
    /// Register new node with hardware fingerprint verification
    pub fn register_node(&mut self, identity: NodeIdentity) -> Result<(), String> {
        // Verify hardware fingerprint signature
        if !identity.hardware_fingerprint.verify(&identity.public_key) {
            return Err("Invalid hardware fingerprint signature".to_string());
        }
        
        // Check IP uniqueness (if available)
        if let Some(ip) = identity.ip_address {
            if self.ip_to_node.contains_key(&ip) {
                return Err("IP address already registered".to_string());
            }
            self.ip_to_node.insert(ip, identity.clone());
        }
        
        // Check hardware fingerprint uniqueness
        if self.has_fingerprint(&identity.hardware_fingerprint.fingerprint) {
            return Err("Hardware fingerprint already registered".to_string());
        }
        
        // If ZK proof provided, verify it
        if let Some(ref zk_proof) = identity.zk_uniqueness_proof {
            if !zk_proof.verify() {
                return Err("Invalid zero-knowledge uniqueness proof".to_string());
            }
            // Store commitment (not the actual fingerprint)
            self.register_commitment(&zk_proof.commitment);
        } else {
            // Store fingerprint directly (for IP-based nodes)
            self.register_fingerprint(&identity.hardware_fingerprint.fingerprint);
        }
        
        // Initialize stats
        self.node_stats.insert(identity.clone(), NodeLongevity::new(identity));
        
        Ok(())
    }
    
    /// Check if fingerprint exists
    pub fn has_fingerprint(&self, fingerprint: &Hash) -> bool {
        self.fingerprints.contains(fingerprint)
    }
    
    /// Check if commitment exists
    pub fn has_commitment(&self, commitment: &Hash) -> bool {
        self.commitments.contains(commitment)
    }
    
    /// Register fingerprint
    pub fn register_fingerprint(&mut self, fingerprint: &Hash) {
        self.fingerprints.insert(*fingerprint);
    }
    
    /// Register commitment (for ZK proofs)
    pub fn register_commitment(&mut self, commitment: &Hash) {
        self.commitments.insert(*commitment);
    }
    
    /// Get node longevity stats
    pub fn get_node_stats(&self, identity: &NodeIdentity) -> Option<&NodeLongevity> {
        self.node_stats.get(identity)
    }
    
    /// Get node longevity stats (mutable)
    pub fn get_node_stats_mut(&mut self, identity: &NodeIdentity) -> Option<&mut NodeLongevity> {
        self.node_stats.get_mut(identity)
    }
    
    /// Update node stats
    pub fn update_node(&mut self, identity: &NodeIdentity, stats: NodeLongevity) {
        self.node_stats.insert(identity.clone(), stats);
    }
    
    /// Record participation for a node
    pub fn record_participation(&mut self, identity: &NodeIdentity, participation: ParticipationType) {
        if let Some(stats) = self.node_stats.get_mut(identity) {
            stats.record_activity_snapshot(participation);
        }
    }
    
    /// Get all registered nodes
    pub fn get_all_nodes(&self) -> Vec<&NodeIdentity> {
        self.node_stats.keys().collect()
    }
    
    /// Get total node count
    pub fn total_nodes(&self) -> usize {
        self.node_stats.len()
    }
    
    /// Get active node count
    pub fn active_nodes(&self) -> usize {
        let current_time = current_timestamp();
        self.node_stats.values()
            .filter(|stats| stats.is_active(current_time))
            .count()
    }
}

/// Longevity tracker for daily snapshots
pub struct LongevityTracker {
    /// Daily snapshots of node activity (on-chain)
    daily_snapshots: Vec<DailySnapshot>,
    
    /// Node registry
    registry: Arc<RwLock<NodeRegistry>>,
    
    /// Blockchain reference (for on-chain storage)
    blockchain: Arc<RwLock<Blockchain>>,
}

struct DailySnapshot {
    /// Date (Unix timestamp, midnight UTC)
    date: u64,
    
    /// Set of nodes that participated in consensus this day
    active_nodes: HashSet<NodeIdentity>,
    
    /// Participation records (verifiable on-chain)
    participation_records: HashMap<NodeIdentity, ParticipationRecord>,
}

struct ParticipationRecord {
    /// Type of participation
    participation_type: ParticipationType,
    
    /// Block hash or proof hash (verifiable)
    participation_proof: Hash,
    
    /// Timestamp of participation
    timestamp: u64,
}

impl LongevityTracker {
    /// Create new longevity tracker
    pub fn new(
        registry: Arc<RwLock<NodeRegistry>>,
        blockchain: Arc<RwLock<Blockchain>>,
    ) -> Self {
        Self {
            daily_snapshots: Vec::new(),
            registry,
            blockchain,
        }
    }
    
    /// Record daily snapshot (called at midnight UTC)
    pub async fn record_daily_snapshot(&mut self) {
        let current_time = current_timestamp();
        let today = current_time - (current_time % 86400); // Round to midnight
        
        // Get all nodes that participated today
        let active_nodes = self.get_nodes_with_activity_today(today).await;
        let participation_records = self.get_participation_records(today).await;
        
        let snapshot = DailySnapshot {
            date: today,
            active_nodes: active_nodes.clone(),
            participation_records,
        };
        
        // Store snapshot on-chain (simplified - in production, create special transaction)
        // For now, just store in memory
        
        self.daily_snapshots.push(snapshot);
        
        // Update longevity for all nodes
        let mut registry = self.registry.write().await;
        for identity in active_nodes {
            if let Some(stats) = registry.node_stats.get_mut(&identity) {
                // Node participated today - already recorded via record_participation
                // Just ensure it's marked as active
                stats.last_seen = current_time;
            } else {
                // Node did not participate today
                if let Some(stats) = registry.node_stats.get_mut(&identity) {
                    stats.record_no_activity();
                }
            }
        }
    }
    
    async fn get_nodes_with_activity_today(&self, _today: u64) -> HashSet<NodeIdentity> {
        // Simplified - in production, query blockchain for blocks/proofs submitted today
        // For now, return empty set
        HashSet::new()
    }
    
    async fn get_participation_records(&self, _today: u64) -> HashMap<NodeIdentity, ParticipationRecord> {
        // Simplified - in production, query blockchain for participation
        // For now, return empty map
        HashMap::new()
    }
}

fn current_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
