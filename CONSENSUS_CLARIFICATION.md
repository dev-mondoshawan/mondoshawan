# Mondoshawan Consensus Clarification

**Critical Security Document**  
**Last Updated**: January 2026

---

## 🚨 The Problem

TriStream mining with different block times (10s, 1s, 100ms) creates potential consensus conflicts:
- How do streams interact?
- What prevents reorgs?
- How is finality determined?
- What's the attack surface?

---

## ✅ Consensus Model: Hierarchical Anchoring

### Stream Hierarchy

**Stream A (10s blocks) = Anchor Layer**
- **Role**: Primary security anchor
- **Block Time**: 10 seconds
- **Finality**: 6 confirmations = 60 seconds
- **Security**: Highest hash power requirement

**Stream B (1s blocks) = Intermediate Layer**
- **Role**: Fast confirmation, references Stream A
- **Block Time**: 1 second
- **Finality**: 1 Stream A confirmation = 10 seconds
- **Security**: Medium hash power requirement

**Stream C (100ms blocks) = Speed Layer**
- **Role**: Ultra-fast finality, references Stream B
- **Block Time**: 100 milliseconds
- **Finality**: 1 Stream B confirmation = 1 second
- **Security**: Lower hash power requirement

---

## 🔗 Block Reference Rules

### Stream A Blocks
- **Parents**: Previous Stream A blocks (DAG structure)
- **References**: None required (anchor blocks)
- **Validation**: Standard GhostDAG consensus
- **Finality**: 6 blocks = 60 seconds

### Stream B Blocks
- **Parents**: Previous Stream B blocks + **Latest Stream A block**
- **References**: Must reference current Stream A anchor
- **Validation**: Valid only if Stream A anchor exists
- **Finality**: 1 Stream A block = 10 seconds

### Stream C Blocks
- **Parents**: Previous Stream C blocks + **Latest Stream B block**
- **References**: Must reference current Stream B block
- **Validation**: Valid only if Stream B block exists
- **Finality**: 1 Stream B block = 1 second

---

## 🛡️ Reorg Protection

### Attack Scenarios

**Scenario 1: Stream A Reorg**
- **Attack**: 51% attack on Stream A
- **Impact**: Can reorg Stream A blocks
- **Protection**: Stream B/C blocks invalidated if Stream A reorgs
- **Finality**: Stream A requires 6 confirmations (60s)

**Scenario 2: Stream B Reorg**
- **Attack**: 51% attack on Stream B
- **Impact**: Can reorg Stream B blocks
- **Protection**: Stream C blocks invalidated, but Stream A anchors remain
- **Finality**: Stream B requires 1 Stream A confirmation (10s)

**Scenario 3: Stream C Reorg**
- **Attack**: 51% attack on Stream C
- **Impact**: Can reorg Stream C blocks
- **Protection**: Minimal impact, Stream A/B anchors remain
- **Finality**: Stream C requires 1 Stream B confirmation (1s)

**Scenario 4: Full Chain Attack**
- **Attack**: 51% attack on all three streams simultaneously
- **Impact**: Full chain reorg possible
- **Protection**: Requires massive coordinated attack
- **Cost**: Extremely expensive (3x hash power)

---

## 📊 Finality Guarantees

### Finality Times

| Stream | Block Time | Confirmations | Finality Time |
|-------|-----------|---------------|---------------|
| Stream A | 10s | 6 blocks | 60 seconds |
| Stream B | 1s | 1 Stream A block | 10 seconds |
| Stream C | 100ms | 1 Stream B block | 1 second |

### Use Cases

**Stream A Finality (60s)**:
- High-value transactions
- Smart contract deployments
- Critical state changes
- Exchange deposits

**Stream B Finality (10s)**:
- Standard transactions
- DEX swaps
- NFT transfers
- Most DeFi operations

**Stream C Finality (1s)**:
- Low-value transactions
- Gaming transactions
- Micro-payments
- Real-time applications

---

## 🔒 Security Model

### Hash Power Distribution

**Stream A (ASIC)**:
- **Algorithm**: Blake3 (ASIC-optimized)
- **Security**: Highest (ASIC hash power)
- **Attack Cost**: Highest (ASIC hardware)
- **Role**: Security anchor

**Stream B (CPU/GPU)**:
- **Algorithm**: KHeavyHash (CPU/GPU-friendly)
- **Security**: Medium (distributed hash power)
- **Attack Cost**: Medium (commodity hardware)
- **Role**: Decentralization

**Stream C (ZK Proofs)**:
- **Algorithm**: ZK proof generation
- **Security**: Lower (specialized hardware)
- **Attack Cost**: Lower (but specialized)
- **Role**: Speed layer

### Attack Resistance

**Single Stream Attack**:
- Attacking one stream doesn't compromise others
- Stream A attack requires 51% of Stream A hash power
- Stream B attack requires 51% of Stream B hash power
- Stream C attack requires 51% of Stream C hash power

**Full Chain Attack**:
- Requires 51% of all three streams simultaneously
- Extremely expensive (3x cost)
- Coordinated attack across different hardware types
- Highly unlikely in practice

---

## ⚙️ Implementation Details

### Block Structure

```rust
struct Block {
    header: BlockHeader,
    transactions: Vec<Transaction>,
    stream_type: StreamType,
    // NEW: Reference to anchor block
    anchor_hash: Option<Hash>, // Stream A block hash (for B/C)
    parent_hashes: Vec<Hash>,  // DAG parents
}
```

### Validation Rules

```rust
fn validate_block(block: &Block, blockchain: &Blockchain) -> Result<()> {
    match block.stream_type {
        StreamType::StreamA => {
            // Stream A: Standard GhostDAG validation
            validate_ghostdag(block, blockchain)?;
        }
        StreamType::StreamB => {
            // Stream B: Must reference Stream A anchor
            let anchor = blockchain.get_block(&block.anchor_hash)?;
            ensure!(anchor.stream_type == StreamType::StreamA, "Invalid anchor");
            validate_ghostdag(block, blockchain)?;
        }
        StreamType::StreamC => {
            // Stream C: Must reference Stream B block
            let parent = blockchain.get_block(&block.anchor_hash)?;
            ensure!(parent.stream_type == StreamType::StreamB, "Invalid parent");
            validate_ghostdag(block, blockchain)?;
        }
    }
    Ok(())
}
```

---

## 📈 Economic Security

### Hash Power Incentives

**Stream A**:
- **Reward**: 50 MSHW/block
- **Frequency**: Every 10 seconds
- **Daily**: ~432,000 MSHW/day
- **Incentive**: High reward attracts hash power

**Stream B**:
- **Reward**: 20 MSHW/block (revised)
- **Frequency**: Every 1 second
- **Daily**: ~1,728,000 MSHW/day
- **Incentive**: High frequency attracts participants

**Stream C**:
- **Reward**: 5 MSHW/block (revised)
- **Frequency**: Every 100ms
- **Daily**: ~4,320,000 MSHW/day
- **Incentive**: High frequency + rewards

### Security Balance

**Revised Rewards**: 50:20:5 = 10:4:1
- Stream A: Security anchor (highest reward)
- Stream B: Balanced (reduced from 25 to 20)
- Stream C: Subsidized (increased from 0 to 5)

**Result**: More balanced security across streams

---

## 🎯 Recommendations

### Immediate Actions

1. **Implement Anchor References**
   - Stream B blocks must reference Stream A
   - Stream C blocks must reference Stream B
   - Validation enforces this

2. **Clarify Finality Rules**
   - Document finality times clearly
   - Implement finality checks in code
   - Update RPC methods to show finality

3. **Balance Rewards**
   - Reduce Stream B: 25 → 20 MSHW
   - Increase Stream C: 0 → 5 MSHW
   - Maintain Stream A: 50 MSHW

4. **Add Reorg Protection**
   - Implement confirmation requirements
   - Add reorg detection
   - Alert on potential attacks

### Long-Term Improvements

1. **Finality Proofs**
   - Cryptographic finality proofs
   - Cross-stream verification
   - Fraud proofs

2. **Staking Layer**
   - Additional security layer
   - Slashing for misbehavior
   - Economic security

3. **Governance**
   - Adjust parameters via governance
   - Emergency response mechanism
   - Community oversight

---

## 📝 Summary

**Consensus Model**: Hierarchical anchoring
- Stream A = Anchor (10s, 60s finality)
- Stream B = Intermediate (1s, 10s finality)
- Stream C = Speed (100ms, 1s finality)

**Security**: Multi-layered
- Each stream requires 51% attack
- Full attack requires all three streams
- Anchor blocks protect lower streams

**Finality**: Stream-dependent
- Stream A: 60 seconds (6 blocks)
- Stream B: 10 seconds (1 Stream A)
- Stream C: 1 second (1 Stream B)

**Status**: Ready for implementation

---

**Consensus clarified and secured!** 🔒
