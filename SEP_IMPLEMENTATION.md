# Security Emergency Proposal (SEP) Implementation

**Automatic Trigger Mechanism**  
**Last Updated**: January 2026

---

## 🚨 Current Status

**Status**: ⚠️ **Partially Implemented**

- ✅ HHI calculation implemented (`ASIC_DOMINANCE_MONITORING.md`)
- ✅ Hashrate distribution tracking implemented
- ⚠️ SEP automatic generation: **Not yet implemented**
- ⚠️ SEP broadcasting: **Not yet implemented**
- ⚠️ Proposal storage: **Not yet implemented**

---

## 📋 Implementation Requirements

### 1. Automatic SEP Generation

**Trigger Conditions**:
1. HHI > 0.25 for **90 consecutive days**
2. Top 3 miners > 50% for **30 consecutive days**

**Implementation Location**: `governance/registry.rs` or new `governance/proposals.rs`

**Code Structure**:
```rust
pub struct SecurityEmergencyProposal {
    pub proposal_id: u64,
    pub trigger_type: TriggerType,
    pub trigger_data: TriggerData,
    pub created_at: u64,
    pub status: ProposalStatus,
}

pub enum TriggerType {
    HhiThreshold { days_above: u64 },
    Top3Dominance { days_above: u64 },
}

pub struct TriggerData {
    pub current_hhi: f64,
    pub top_3_share: f64,
    pub top_miners: Vec<(Address, f64)>,
}

impl NodeRegistry {
    /// Check if SEP should be triggered
    pub fn check_sep_trigger(&self) -> Option<SecurityEmergencyProposal> {
        let hhi = self.calculate_hhi();
        let top_3_share = self.top_n_share(3);
        
        // Check HHI threshold (90 days)
        if hhi > 0.25 && self.days_above_hhi_threshold() >= 90 {
            return Some(SecurityEmergencyProposal {
                proposal_id: generate_proposal_id(),
                trigger_type: TriggerType::HhiThreshold {
                    days_above: self.days_above_hhi_threshold(),
                },
                trigger_data: TriggerData {
                    current_hhi: hhi,
                    top_3_share,
                    top_miners: self.get_top_miners(3),
                },
                created_at: current_timestamp(),
                status: ProposalStatus::Active,
            });
        }
        
        // Check top 3 dominance (30 days)
        if top_3_share > 0.5 && self.days_top_3_dominance() >= 30 {
            return Some(SecurityEmergencyProposal {
                proposal_id: generate_proposal_id(),
                trigger_type: TriggerType::Top3Dominance {
                    days_above: self.days_top_3_dominance(),
                },
                trigger_data: TriggerData {
                    current_hhi: hhi,
                    top_3_share,
                    top_miners: self.get_top_miners(3),
                },
                created_at: current_timestamp(),
                status: ProposalStatus::Active,
            });
        }
        
        None
    }
}
```

---

### 2. Daily Monitoring Task

**Implementation**: Add to `LongevityTracker` or create separate monitoring task

```rust
impl LongevityTracker {
    /// Daily monitoring task (run at midnight UTC)
    pub async fn daily_monitoring(&mut self) {
        // Record daily snapshot
        self.record_daily_snapshot().await;
        
        // Check for SEP trigger
        let mut registry = self.registry.write().await;
        if let Some(sep) = registry.check_sep_trigger() {
            // Broadcast SEP to all nodes
            self.broadcast_sep(&sep).await;
            
            // Store SEP on-chain
            self.store_sep_on_chain(&sep).await;
        }
    }
    
    /// Broadcast SEP to all nodes
    async fn broadcast_sep(&self, sep: &SecurityEmergencyProposal) {
        // In production: Broadcast via P2P network
        // For now: Log and store
        println!("🚨 SECURITY EMERGENCY PROPOSAL TRIGGERED!");
        println!("   Proposal ID: {}", sep.proposal_id);
        println!("   Trigger: {:?}", sep.trigger_type);
        println!("   HHI: {:.3}", sep.trigger_data.current_hhi);
        println!("   Top 3 Share: {:.1}%", sep.trigger_data.top_3_share * 100.0);
    }
    
    /// Store SEP on-chain
    async fn store_sep_on_chain(&self, sep: &SecurityEmergencyProposal) {
        // Create special transaction to store SEP
        // This ensures SEP is verifiable and cannot be suppressed
        // Implementation: Similar to activity snapshots
    }
}
```

---

### 3. RPC Endpoints

**New Methods Needed**:
- `mds_getActiveProposals` - Get all active proposals
- `mds_getProposalDetails` - Get specific proposal details
- `mds_getSepStatus` - Get current SEP trigger status

**Implementation**: Add to `rpc.rs`

---

## 🧪 Testing SEP Trigger

### Manual Trigger Test

**For Testing**: Add manual trigger function

```rust
impl NodeRegistry {
    /// Manually trigger SEP (for testing only)
    #[cfg(test)]
    pub fn manually_trigger_sep(&mut self) -> SecurityEmergencyProposal {
        // Force trigger conditions
        self.force_hhi_above_threshold(90);
        self.check_sep_trigger().unwrap()
    }
}
```

**Test Script**:
```rust
#[test]
fn test_sep_trigger() {
    let mut registry = NodeRegistry::new();
    
    // Simulate 90 days of high HHI
    for day in 0..90 {
        simulate_high_centralization(&mut registry);
        advance_time(86400);
    }
    
    // Check for SEP
    let sep = registry.check_sep_trigger();
    assert!(sep.is_some(), "SEP should be triggered");
    
    let sep = sep.unwrap();
    assert!(matches!(sep.trigger_type, TriggerType::HhiThreshold { .. }));
    assert!(sep.trigger_data.current_hhi > 0.25);
}
```

---

## 📊 Implementation Priority

### Phase 1: Core SEP Generation (High Priority)
- [ ] Implement `check_sep_trigger()` in `NodeRegistry`
- [ ] Add daily monitoring task
- [ ] Store SEP on-chain
- [ ] Add RPC endpoints

### Phase 2: Broadcasting (Medium Priority)
- [ ] P2P network integration
- [ ] Node-to-node SEP propagation
- [ ] Verification of SEP authenticity

### Phase 3: Full Governance (Lower Priority)
- [ ] Voting mechanism
- [ ] Proposal lifecycle management
- [ ] Implementation window tracking

---

## 🎯 Ready for Implementation

**Status**: ✅ **Design complete, ready to code**

The SEP trigger mechanism is fully designed and ready for implementation. The main work is:
1. Adding trigger checking to `NodeRegistry`
2. Creating daily monitoring task
3. Implementing on-chain storage
4. Adding RPC endpoints

**Estimated Effort**: 1-2 days for core implementation

---

**SEP implementation ready to proceed!** 🚨
