//! Node Longevity Tracking

use crate::types::{Hash, StreamType};
use crate::governance::node_identity::NodeIdentity;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Node longevity tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeLongevity {
    pub node_identity: NodeIdentity,
    
    /// Total days node has been active (via activity snapshots)
    pub active_days: u64,
    
    /// Total blocks mined (any stream)
    pub blocks_mined: u64,
    
    /// Non-linear uptime index (0.0 to 1.0)
    pub uptime_index: f64,
    
    /// Last seen timestamp
    pub last_seen: u64,
    
    /// Network age when node joined
    pub network_age_at_join: u64,
    
    /// Daily activity snapshots (on-chain record)
    pub activity_snapshots: Vec<ActivitySnapshot>,
    
    /// Consecutive offline days (for reset calculation)
    pub consecutive_offline_days: u64,
}

/// Daily activity snapshot (on-chain record)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySnapshot {
    /// Date (Unix timestamp, midnight UTC)
    pub date: u64,
    
    /// Whether node participated in consensus this day
    pub participated: bool,
    
    /// Type of participation (block, zk_proof, etc.)
    pub participation_type: ParticipationType,
    
    /// Block hash or proof hash (verifiable)
    pub participation_proof: Hash,
}

/// Type of participation in consensus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticipationType {
    BlockMined { stream: StreamType, block_hash: Hash },
    ZkProofSubmitted { proof_hash: Hash },
    ConsensusVote { vote_hash: Hash },
}

impl NodeLongevity {
    /// Create new node longevity tracker
    pub fn new(node_identity: NodeIdentity) -> Self {
        let now = current_timestamp();
        Self {
            node_identity,
            active_days: 0,
            blocks_mined: 0,
            uptime_index: 0.0,
            last_seen: now,
            network_age_at_join: now,
            activity_snapshots: Vec::new(),
            consecutive_offline_days: 0,
        }
    }
    
    /// Calculate longevity weight for governance voting
    pub fn calculate_weight(&self, network_age_days: u64) -> f64 {
        // Eligibility checks
        if self.active_days < 30 {
            return 0.0; // Must be active ≥ 30 days
        }
        
        if self.uptime_index < 0.8 {
            return 0.0; // Must have ≥ 80% uptime index
        }
        
        if self.blocks_mined == 0 {
            return 0.0; // Must have mined ≥ 1 block
        }
        
        // Ratio of active presence to total network age
        let longevity_ratio = self.active_days as f64 / network_age_days as f64;
        
        // Weight is 40% of the total governance vote, capped at 0.1%
        (longevity_ratio * 0.4).min(0.001)
    }
    
    /// Calculate non-linear uptime index
    /// Penalizes long offline periods more than short ones
    pub fn calculate_uptime_index(&self, network_age_days: u64) -> f64 {
        if network_age_days == 0 {
            return 0.0;
        }
        
        // Base uptime: active days / total days
        let base_uptime = self.active_days as f64 / network_age_days as f64;
        
        // Penalty for consecutive offline days (non-linear)
        let offline_penalty = if self.consecutive_offline_days > 30 {
            // Reset to zero if offline > 30 days
            0.0
        } else if self.consecutive_offline_days > 7 {
            // Heavy penalty for > 7 days offline
            base_uptime * (1.0 - (self.consecutive_offline_days as f64 / 30.0))
        } else {
            // Light penalty for < 7 days offline
            base_uptime * 0.95
        };
        
        offline_penalty.max(0.0)
    }
    
    /// Record activity snapshot (called when node participates in consensus)
    pub fn record_activity_snapshot(&mut self, participation: ParticipationType) {
        let current_time = current_timestamp();
        let today = current_time - (current_time % 86400); // Round to midnight
        
        // Check if already recorded for today
        if let Some(last_snapshot) = self.activity_snapshots.last() {
            if last_snapshot.date == today {
                // Update today's snapshot
                let snapshot = ActivitySnapshot {
                    date: today,
                    participated: true,
                    participation_type: participation.clone(),
                    participation_proof: match &participation {
                        ParticipationType::BlockMined { block_hash, .. } => *block_hash,
                        ParticipationType::ZkProofSubmitted { proof_hash } => *proof_hash,
                        ParticipationType::ConsensusVote { vote_hash } => *vote_hash,
                    },
                };
                *self.activity_snapshots.last_mut().unwrap() = snapshot;
                return;
            }
        }
        
        // Create new snapshot
        let snapshot = ActivitySnapshot {
            date: today,
            participated: true,
            participation_type: participation.clone(),
            participation_proof: match &participation {
                ParticipationType::BlockMined { block_hash, .. } => *block_hash,
                ParticipationType::ZkProofSubmitted { proof_hash } => *proof_hash,
                ParticipationType::ConsensusVote { vote_hash } => *vote_hash,
            },
        };
        
        self.activity_snapshots.push(snapshot);
        self.active_days += 1;
        self.consecutive_offline_days = 0;
        self.last_seen = current_time;
        
        // Increment blocks_mined if it's a block
        if matches!(participation, ParticipationType::BlockMined { .. }) {
            self.blocks_mined += 1;
        }
        
        // Recalculate uptime index
        let network_age_days = (current_time - self.network_age_at_join) / 86400;
        self.uptime_index = self.calculate_uptime_index(network_age_days);
    }
    
    /// Record no activity for today (node offline)
    pub fn record_no_activity(&mut self) {
        let current_time = current_timestamp();
        let today = current_time - (current_time % 86400);
        
        // Check if already recorded for today
        if let Some(last_snapshot) = self.activity_snapshots.last() {
            if last_snapshot.date == today {
                return; // Already recorded
            }
        }
        
        // Create snapshot with no participation
        let snapshot = ActivitySnapshot {
            date: today,
            participated: false,
            participation_type: ParticipationType::ConsensusVote { 
                vote_hash: [0u8; 32] // Placeholder
            },
            participation_proof: [0u8; 32],
        };
        
        self.activity_snapshots.push(snapshot);
        self.consecutive_offline_days += 1;
        
        // Reset if offline > 30 days
        if self.consecutive_offline_days > 30 {
            self.active_days = 0;
            self.uptime_index = 0.0;
        } else {
            // Recalculate uptime index with penalty
            let network_age_days = (current_time - self.network_age_at_join) / 86400;
            self.uptime_index = self.calculate_uptime_index(network_age_days);
        }
    }
    
    /// Check if node is still active
    pub fn is_active(&self, current_time: u64) -> bool {
        // Node is active if seen within last 24 hours
        current_time - self.last_seen < 86400
    }
    
    /// Reset longevity if node offline too long
    pub fn check_reset(&mut self, current_time: u64) {
        // If offline > 30 days, reset longevity
        if current_time - self.last_seen > 2592000 { // 30 days
            self.active_days = 0;
            self.uptime_index = 0.0;
            self.consecutive_offline_days = 0;
        }
    }
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
