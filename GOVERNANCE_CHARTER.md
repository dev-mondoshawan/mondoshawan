# Mondoshawan Governance Charter: Algorithm Rotation (AR)

**Critical Governance Document**  
**Status**: Final Specification  
**Last Updated**: January 2026

---

## 🎯 Charter Purpose

**Building a governance charter for something as critical as an Algorithm Rotation (AR) requires a delicate balance between automation and human oversight. You need a system that can move fast during a security crisis but remains fair enough that it doesn't just become a tool for the biggest token holders to bully everyone else.**

This charter defines the **"rules of engagement"** for rotating the Stream A algorithm.

**Core Principles**:
- ✅ Transparent
- ✅ Community-driven
- ✅ Resistant to gaming
- ✅ Fair to all stakeholders
- ✅ Fast during security crises
- ✅ Balanced between automation and human oversight

---

## 🏛️ Governance Principles

### Core Tenets

1. **Transparency First**: All monitoring data public, all votes on-chain
2. **Community Control**: No single entity can trigger rotation
3. **Fair Weighting**: Miners, holders, and nodes all have voice
4. **Anti-Gaming**: Mechanisms prevent manipulation
5. **Gradual Transition**: 6-month window prevents disruption

---

## 🔴 Part 1: The Trigger Mechanism

### Automatic Trigger (Red Zone)

**Security Emergency Proposal (SEP) is automatically generated if:**

1. **HHI Threshold**: Herfindahl-Hirschman Index > 0.25 for **90 consecutive days**
   - Measured daily at midnight UTC
   - Requires 90/90 days above threshold
   - No manual intervention possible

2. **Top 3 Dominance**: Top 3 miners control >50% hashrate for **30 consecutive days**
   - Measured daily at midnight UTC
   - Requires 30/30 days above threshold
   - Triggers immediate SEP

**SEP Generation:**
- Protocol automatically creates proposal
- Broadcasts to all nodes
- Visible in explorer immediately
- Cannot be suppressed or delayed

### 2. Community-Initiated Proposal

**Any stakeholder can propose a rotation if the network is in the Yellow Zone (HHI 0.15–0.25).**

**Requirements:**
1. **Stake Requirement**: Proposer must hold **≥0.1% of the circulating MSHW supply**
2. **Proposal Fee**: **10,000 MSHW (burned)** to prevent spam
3. **Technical Justification**: Must include:
   - Current HHI data
   - Proposed algorithm
   - Technical analysis
   - Migration plan

**Proposal Requirements:**
- Must be submitted via RPC or explorer
- Requires technical review (Dev Fund sponsors)
- Community can challenge (14-day challenge period)

---

## 🗳️ Part 2: The Voting Hierarchy (Hybrid Model)

**To prevent "whale dominance," your voting power is split between your economic stake and your actual contribution to the network's infrastructure.**

### Token Weight (60%)

**Proportional to your MSHW balance**

```
Token Weight = (Holder Balance / Circulating Supply) × 0.6
```

**The Whale Cap**: No single address can represent more than **5% of the total vote**.

**Effect**: Prevents whale dominance while still giving economic stake appropriate weight.

---

### Node Longevity Weight (40%)

**Proportional to your node's age and activity**

```
Node Longevity Weight = (Node Uptime Days / Network Age Days) × 0.4
```

**Eligibility Requirements**:
- Node must be active **≥30 days**
- Node must have **≥80% uptime**
- Node must have mined **≥1 block** (any stream)

**The Stability Cap**: Each unique node's influence is capped at **0.1% of the total vote**.

**Anti-Gaming**:
- Node identity tied to public key (cannot be transferred)
- Hardware fingerprinting prevents Sybil attacks
- Longevity resets if node offline > 30 days

---

### The Miner's Veto Protection

**Existing miners are vital stakeholders, but they have a natural conflict of interest when voting on their own hardware's obsolescence.**

**The Miner Cap**: Total voting power from confirmed miners is capped at **20% of the total vote**.

**Scaling**: If miner votes exceed this cap, they are scaled down proportionally to ensure the community—not the hardware manufacturers—controls the network's destiny.

**Calculation:**
```
If (Sum of All Miner Votes) > 20% of Total Votes:
    Scale down all miner votes proportionally
```

**Rationale**:
- Miners are stakeholders, but not the only stakeholders
- Prevents conflict of interest
- Protects network from miner capture
- Ensures community control

---

## 📋 Part 3: The AR Lifecycle

**Rotating an algorithm is a "nuclear option" that requires a clear sunset period to avoid market chaos.**

### Phase I: Deliberation (14 Days)

**The 10% Development Fund sponsors an independent technical audit of the proposed replacement algorithm.**

**Activities:**
1. **Technical Audit**: Dev Fund pays for:
   - Independent security review
   - Performance analysis
   - Migration cost estimate
   - Compatibility assessment

2. **Public Q&A**: Technical review period for all stakeholders
   - Public forum discussion
   - Technical Q&A sessions
   - Alternative proposals
   - Challenge period (anyone can challenge)

3. **Transparency**:
   - All data public
   - All discussions recorded
   - All challenges addressed

**Outcome**: Proposal either proceeds to vote or is withdrawn

---

### Phase II: The Snapshot Vote (7 Days)

**Quorum**: **30% of the circulating supply** must participate.

**Approval**: A **66% (two-thirds) majority** is required to pass.

**Voting Mechanism:**
- On-chain voting (transparent, verifiable)
- Binary: "Yes" or "No"
- Weighted by hybrid model (Token 60% + Node 40%)

**Vote Counting:**
```
Total Votes = Sum of (Token Weight + Node Longevity Weight) for all participants

Yes Votes = Sum of weights for "Yes" votes
No Votes = Sum of weights for "No" votes

Approval = (Yes Votes / Total Votes) ≥ 0.66 AND Total Votes ≥ 30% of Supply
```

**Transparency:**
- Real-time vote count in explorer
- All votes public (address + weight)
- Cannot be changed once cast

**Duration**: 7 days (can extend to 14 days if quorum not met)

---

### Phase III: The Implementation Window (180 Days)

**If approved, a 6-month countdown begins. This gives honest miners time to amortize their ASIC costs or transition to other chains.**

**Timeline:**
1. **Countdown Begins**: 180-day transition period
   - Block height calculated
   - Sunset block announced
   - Countdown timer in explorer

2. **Legacy Support**:
   - Stream B (GPU) and Stream C (ZK) unaffected
   - Network stability maintained
   - Only Stream A transitions

3. **Dev Fund Subsidy**:
   - First 3 months: 150% of normal rewards on new algorithm
   - Encourages immediate migration
   - Ensures network security

4. **Hardware Amortization**:
   - Honest miners have 6 months to:
     - Amortize Blake3 ASIC costs
     - Switch to other chains
     - Retire hardware
   - No sudden obsolescence

**The Sunset Block**: At the end of the window, Stream A switches to the new algorithm via a hard fork.

**See**: `MINERS_TRANSITION_GUIDE.md` for detailed migration instructions

---

## ⚙️ Part 4: Implementation & Hard Fork

### The Sunset Block

**At predetermined block height:**
- Stream A switches to new algorithm
- Old Blake3 blocks still valid (for history)
- New blocks must use new algorithm
- Automatic difficulty adjustment

**Legacy Support:**
- Old blocks remain in chain
- Historical data preserved
- Explorer shows transition clearly

### Network Stability

**During Transition:**
- Stream B (GPU) continues normally
- Stream C (ZK) continues normally
- Only Stream A affected
- Network remains operational

**Post-Transition:**
- New algorithm active
- Difficulty adjusts automatically
- Rewards continue (subsidized first 3 months)
- Network security maintained

---

## 🛡️ Anti-Gaming Mechanisms

### 1. Sybil Attack Prevention

**Node Identity:**
- One node per public key
- IP-based uniqueness (or proof-of-uniqueness)
- Cannot transfer node identity
- Longevity resets if offline > 30 days

### 2. Whale Dominance Prevention

**Token Weight Caps:**
- Maximum 5% per address
- Encourages distribution
- Prevents single-entity control

### 3. Miner Veto Prevention

**Miner Vote Cap:**
- Total miner votes capped at 20%
- Prevents ASIC miners from blocking rotation
- Protects network from capture

### 4. Proposal Spam Prevention

**Proposal Fee:**
- 10,000 MSHW burned on proposal
- Prevents spam proposals
- Ensures serious intent

### 5. Vote Manipulation Prevention

**Vote Transparency:**
- All votes public
- All weights public
- Real-time counting
- Cannot be changed once cast

---

## 📊 Implementation Details

### Smart Contract Logic (Conceptual)

**Note**: Mondoshawan is Rust-based, not EVM. This is conceptual design.

```rust
// Governance state
pub struct AlgorithmRotationProposal {
    pub proposal_id: u64,
    pub proposer: Address,
    pub trigger_type: TriggerType, // Automatic or Community
    pub current_hhi: f64,
    pub proposed_algorithm: String,
    pub phase: ProposalPhase,
    pub votes: HashMap<Address, Vote>,
    pub start_block: u64,
    pub end_block: u64,
}

pub enum ProposalPhase {
    Deliberation { end_block: u64 },
    Voting { end_block: u64 },
    Approved { sunset_block: u64 },
    Rejected,
}

pub struct Vote {
    pub address: Address,
    pub choice: VoteChoice,
    pub token_weight: f64,
    pub node_weight: f64,
    pub total_weight: f64,
}

pub enum VoteChoice {
    Yes,
    No,
}

// Voting logic
impl AlgorithmRotationProposal {
    pub fn calculate_vote_weight(&self, voter: Address, blockchain: &Blockchain) -> f64 {
        let token_weight = self.token_weight(voter, blockchain);
        let node_weight = self.node_longevity_weight(voter, blockchain);
        
        // Apply caps
        let token_weight = token_weight.min(0.05); // 5% max
        let node_weight = node_weight.min(0.001); // 0.1% max per node
        
        token_weight + node_weight
    }
    
    fn token_weight(&self, voter: Address, blockchain: &Blockchain) -> f64 {
        let balance = blockchain.get_balance(voter);
        let supply = blockchain.get_circulating_supply();
        (balance as f64 / supply as f64) * 0.6
    }
    
    fn node_longevity_weight(&self, voter: Address, blockchain: &Blockchain) -> f64 {
        let node_stats = blockchain.get_node_stats(voter);
        
        // Node must be active ≥ 30 days
        if node_stats.uptime_days < 30 {
            return 0.0;
        }
        
        // Node must be online ≥ 80% of time
        if node_stats.uptime_percentage < 0.8 {
            return 0.0;
        }
        
        // Node must have mined ≥ 1 block
        if node_stats.blocks_mined == 0 {
            return 0.0;
        }
        
        let network_age = blockchain.get_network_age_days();
        let longevity_ratio = node_stats.uptime_days as f64 / network_age as f64;
        
        (longevity_ratio * 0.4).min(0.001) // 0.1% max per node
    }
    
    pub fn apply_miner_cap(&mut self, blockchain: &Blockchain) {
        let total_votes: f64 = self.votes.values()
            .map(|v| v.total_weight)
            .sum();
        
        let miner_votes: f64 = self.votes.iter()
            .filter(|(addr, _)| blockchain.is_miner(addr))
            .map(|(_, vote)| vote.total_weight)
            .sum();
        
        let miner_cap = total_votes * 0.2; // 20% cap
        
        if miner_votes > miner_cap {
            let scale_factor = miner_cap / miner_votes;
            
            // Scale down all miner votes
            for (addr, vote) in self.votes.iter_mut() {
                if blockchain.is_miner(addr) {
                    vote.total_weight *= scale_factor;
                }
            }
        }
    }
    
    pub fn check_quorum(&self, blockchain: &Blockchain) -> bool {
        let total_weight: f64 = self.votes.values()
            .map(|v| v.total_weight)
            .sum();
        
        let supply = blockchain.get_circulating_supply();
        let quorum_threshold = supply as f64 * 0.3; // 30% of supply
        
        total_weight >= quorum_threshold
    }
    
    pub fn check_approval(&self) -> bool {
        let yes_votes: f64 = self.votes.values()
            .filter(|v| matches!(v.choice, VoteChoice::Yes))
            .map(|v| v.total_weight)
            .sum();
        
        let total_votes: f64 = self.votes.values()
            .map(|v| v.total_weight)
            .sum();
        
        if total_votes == 0.0 {
            return false;
        }
        
        let approval_ratio = yes_votes / total_votes;
        approval_ratio >= 0.66 // 66% majority
    }
}
```

---

## 🔍 Transparency Requirements

### Explorer Integration

**Must Display:**
1. **Current HHI**: Real-time, always visible
2. **Top Miners**: Share percentages
3. **Active Proposals**: All AR proposals
4. **Vote Status**: Real-time vote counts
5. **Node Stats**: Longevity metrics

**Example Display:**
```
┌─────────────────────────────────────┐
│ Algorithm Rotation Status          │
├─────────────────────────────────────┤
│ Current HHI: 0.18 (Yellow Zone)    │
│ Top 3 Share: 42%                    │
│                                     │
│ Active Proposals: 0                 │
│ Last Rotation: Never                │
│                                     │
│ [View Full Details]                 │
└─────────────────────────────────────┘
```

### RPC Endpoints

**New RPC Methods:**
- `mds_getGovernanceStatus` - Current governance state
- `mds_getActiveProposals` - All active AR proposals
- `mds_getProposalDetails(id)` - Specific proposal details
- `mds_submitProposal(algorithm, justification)` - Submit AR proposal
- `mds_vote(proposal_id, choice)` - Cast vote
- `mds_getVoteStatus(proposal_id)` - Vote status

---

## 📝 Honest Reality Check

### Sybil Risk

**Even with hardware fingerprinting, determined attackers will try to "farm" nodes for longevity weight.**

**Defense**: Your **30-day offline reset** is your best defense against this.

**Additional Measures**:
- Hardware fingerprinting (PUF-based)
- IP-based uniqueness
- ZK proofs for privacy-focused users
- Ongoing monitoring and detection

---

### Quorum Challenges

**30% participation is a high bar.**

**Mitigation**: You may need to use the **Dev Fund to incentivize voting** if participation is low.

**Options**:
- Voting rewards (small MSHW incentive)
- Extended voting period (up to 14 days)
- Community outreach campaigns
- Lower quorum threshold (if needed, via governance)

**Reality**: Most governance votes struggle with participation, but 30% is achievable with proper incentives.

### What This Charter Achieves

✅ **Transparency**: All data public, all votes visible  
✅ **Fairness**: Multiple stakeholder types, caps prevent dominance  
✅ **Security**: Automatic triggers, cannot be suppressed  
✅ **Gradual**: 6-month transition prevents disruption  
✅ **Community Control**: No single entity can control outcome

### What It Doesn't Solve

❌ **Perfect Fairness**: Impossible, but we minimize  
❌ **100% Participation**: Unrealistic, but 30% is achievable  
❌ **Zero Gaming**: Impossible, but we make it expensive  
❌ **Instant Consensus**: Governance takes time, by design

---

## 📝 Implementation Checklist

### Phase 1: Monitoring (Immediate)
- [ ] Implement HHI tracking
- [ ] Implement top miner tracking
- [ ] Add automatic SEP generation
- [ ] Display in explorer

### Phase 2: Governance Infrastructure (Short-term)
- [ ] Implement proposal system
- [ ] Implement voting system
- [ ] Implement node longevity tracking
- [ ] Add RPC endpoints

### Phase 3: Smart Contract Logic (Medium-term)
- [ ] Implement vote weighting
- [ ] Implement caps (whale, miner)
- [ ] Implement quorum checking
- [ ] Implement approval checking

### Phase 4: Testing (Before Mainnet)
- [ ] Test automatic triggers
- [ ] Test community proposals
- [ ] Test voting mechanism
- [ ] Test anti-gaming measures

---

## 🎯 Summary

**Governance Principles:**
- Transparency first
- Community control
- Fair weighting
- Anti-gaming
- Gradual transition

**Trigger Mechanisms:**
- Automatic (HHI > 0.25 for 90 days)
- Community (HHI 0.15-0.25, ≥0.1% stake)

**Voting Hierarchy:**
- Token weight (60%, 5% cap per address)
- Node longevity (40%, 0.1% cap per node)
- Miner cap (20% total)

**Voting Phases:**
- Deliberation (14 days)
- Snapshot vote (7 days, 30% quorum, 66% approval)
- Implementation (180 days)

**Result**: Rigid, transparent, community-controlled governance

---

**Ready for implementation!** 🏛️
