# Mondoshawan Protocol - Revised Tokenomics

**Status**: Revised Based on Economic & Security Analysis  
**Last Updated**: January 2026

---

## 🚨 Critical Issues Identified

### 1. Infinite Inflation Problem
- **Issue**: 100% inflationary with no cap or halving
- **Risk**: Perpetual sell pressure, no scarcity narrative
- **Impact**: Token value erosion, miner dumping

### 2. TriStream Security Asymmetry
- **Issue**: Stream B produces 5x more tokens than Stream A
- **Risk**: Attackers can target weaker stream
- **Impact**: Potential reorgs, double-spends

### 3. Stream C Economics
- **Issue**: Zero block rewards, fee-only
- **Risk**: No incentive to run ZK provers
- **Impact**: Centralization or empty stream

### 4. Fair Launch Sustainability
- **Issue**: Zero team allocation
- **Risk**: Project becomes zombie chain
- **Impact**: No funds for development, audits, listings

---

## ✅ Revised Tokenomics Model

### Supply Model: Deflationary with Cap

**New Model:**
- **Max Supply**: 10,000,000,000 MSHW (10 billion)
- **Initial Supply**: 0 (fair launch maintained)
- **Emission**: Mining rewards with halving
- **Burn Mechanism**: Fee burns (optional)

### Revised Block Rewards

#### Stream A: ASIC Mining (Blake3)
- **Block Time**: 10 seconds
- **Initial Reward**: 50 MSHW per block
- **Halving**: Every 4 years (1,261,440 blocks)
- **Purpose**: Security anchor, high throughput

#### Stream B: CPU/GPU Mining (KHeavyHash)
- **Block Time**: 1 second
- **Initial Reward**: 20 MSHW per block (reduced from 25)
- **Halving**: Every 4 years (126,144,000 blocks)
- **Purpose**: Decentralization, accessibility

#### Stream C: ZK Proof Validation
- **Block Time**: 100ms
- **Initial Reward**: 5 MSHW per block (NEW - subsidized)
- **Halving**: Every 4 years (1,261,440,000 blocks)
- **Purpose**: Ultra-fast finality, ZK security

**Rationale**: Stream C now has block rewards to incentivize ZK prover operation.

### Development Fund

**Allocation**: 10% of all block rewards
- **Stream A**: 5 MSHW per block (10% of 50)
- **Stream B**: 2 MSHW per block (10% of 20)
- **Stream C**: 0.5 MSHW per block (10% of 5)

**Total Daily Fund**: ~259,200 MSHW/day
- **Annual Fund**: ~94,608,000 MSHW/year

**Use Cases**:
- Security audits ($100k+)
- Exchange listings
- Developer grants
- Marketing
- Infrastructure
- Legal/compliance

**Governance**: Multi-sig wallet, community governance (future)

---

## 📊 Revised Emission Schedule

### Year 1 (Pre-Halving)
- **Stream A**: 50 MSHW/block × 8,640 blocks/day = 432,000 MSHW/day
- **Stream B**: 20 MSHW/block × 86,400 blocks/day = 1,728,000 MSHW/day
- **Stream C**: 5 MSHW/block × 864,000 blocks/day = 4,320,000 MSHW/day
- **Total Daily**: 6,480,000 MSHW/day
- **Minus Dev Fund (10%)**: 5,832,000 MSHW/day to miners
- **Annual Emission**: ~2,128,680,000 MSHW/year

### Year 5 (Post First Halving)
- **Stream A**: 25 MSHW/block = 216,000 MSHW/day
- **Stream B**: 10 MSHW/block = 864,000 MSHW/day
- **Stream C**: 2.5 MSHW/block = 2,160,000 MSHW/day
- **Total Daily**: 3,240,000 MSHW/day
- **Minus Dev Fund (10%)**: 2,916,000 MSHW/day to miners
- **Annual Emission**: ~1,064,340,000 MSHW/year

### Year 10 (Post Second Halving)
- **Stream A**: 12.5 MSHW/block = 108,000 MSHW/day
- **Stream B**: 5 MSHW/block = 432,000 MSHW/day
- **Stream C**: 1.25 MSHW/block = 1,080,000 MSHW/day
- **Total Daily**: 1,620,000 MSHW/day
- **Minus Dev Fund (10%)**: 1,458,000 MSHW/day to miners
- **Annual Emission**: ~532,170,000 MSHW/year

### Supply Projections

| Year | Cumulative Supply | Annual Emission | Inflation Rate |
|------|-------------------|-----------------|----------------|
| 1 | ~2.13B | 2.13B | N/A (initial) |
| 5 | ~6.65B | 1.06B | ~16% |
| 10 | ~9.33B | 0.53B | ~5.7% |
| 20 | ~10B (cap) | 0 | ~0% |

**Note**: At 10B cap, only transaction fees remain (no block rewards)

---

## 🔒 Security Improvements

### TriStream Balance

**Revised Rewards**:
- Stream A: 50 MSHW (security anchor)
- Stream B: 20 MSHW (reduced from 25)
- Stream C: 5 MSHW (new, incentivized)

**Security Ratio**: 50:20:5 = 10:4:1
- Stream A remains the security anchor
- Stream B reduced to prevent dominance
- Stream C subsidized for operation

### Consensus Clarification

**Block Ordering**:
1. **Stream A blocks** serve as "anchors" (10s intervals)
2. **Stream B blocks** reference Stream A anchors (1s intervals)
3. **Stream C blocks** reference Stream B blocks (100ms intervals)

**Reorg Protection**:
- Stream A blocks require 6 confirmations (60s)
- Stream B blocks require 1 Stream A confirmation (10s)
- Stream C blocks require 1 Stream B confirmation (1s)

**Finality**:
- **Stream A**: 60 seconds (6 blocks)
- **Stream B**: 10 seconds (1 Stream A block)
- **Stream C**: 1 second (1 Stream B block)

---

## 💰 Economic Model Improvements

### Scarcity Mechanisms

1. **Halving Every 4 Years**
   - Reduces emission by 50% every 4 years
   - Creates scarcity narrative
   - Similar to Bitcoin model

2. **Max Supply Cap (10 Billion)**
   - Hard cap prevents infinite inflation
   - After cap: fee-only model
   - Creates long-term value proposition

3. **Fee Burns (Optional)**
   - Burn 50% of transaction fees
   - Deflationary pressure
   - Can be enabled via governance

### Miner Economics

**Stream A (ASIC)**:
- **Reward**: 50 MSHW/block (45 after dev fund)
- **Daily**: ~388,800 MSHW/day (after dev fund)
- **Best For**: High hash power, security-focused

**Stream B (CPU/GPU)**:
- **Reward**: 20 MSHW/block (18 after dev fund)
- **Daily**: ~1,555,200 MSHW/day (after dev fund)
- **Best For**: Decentralized participation

**Stream C (ZK Proofs)**:
- **Reward**: 5 MSHW/block (4.5 after dev fund)
- **Daily**: ~3,888,000 MSHW/day (after dev fund)
- **Best For**: Fast finality, ZK expertise

**Total to Miners**: ~5,832,000 MSHW/day (Year 1)

---

## 🏛️ Development Fund Structure

### Allocation Breakdown

**10% of All Block Rewards**:
- **Stream A**: 5 MSHW/block
- **Stream B**: 2 MSHW/block
- **Stream C**: 0.5 MSHW/block
- **Total**: ~259,200 MSHW/day

### Fund Usage

**Year 1 Priorities**:
1. **Security Audit**: $100,000+ (Trail of Bits, OpenZeppelin)
2. **Exchange Listings**: $50,000+ (listing fees)
3. **Developer Grants**: $200,000+ (ecosystem building)
4. **Infrastructure**: $100,000+ (servers, monitoring)
5. **Marketing**: $100,000+ (community building)
6. **Legal/Compliance**: $50,000+ (regulatory)

**Total Year 1 Budget**: ~$600,000+ (from dev fund)

### Governance

**Initial**: Multi-sig wallet (3-of-5)
- Core team members
- Community representatives
- Technical advisors

**Future**: On-chain governance
- MSHW holders vote on fund allocation
- Proposal system
- Transparent spending

---

## 📈 Revised Supply Projections

### With Halving & Cap

**Year 1**: ~2.13B MSHW (21.3% of cap)
**Year 5**: ~6.65B MSHW (66.5% of cap)
**Year 10**: ~9.33B MSHW (93.3% of cap)
**Year 15**: ~9.8B MSHW (98% of cap)
**Year 20**: ~10B MSHW (100% cap reached)

**After Cap**: Only transaction fees (no block rewards)

### Inflation Rate

- **Year 1**: N/A (initial)
- **Year 5**: ~16% (1.06B / 6.65B)
- **Year 10**: ~5.7% (0.53B / 9.33B)
- **Year 15**: ~2.4% (0.24B / 9.8B)
- **Year 20+**: ~0% (cap reached)

---

## 🔄 Implementation Changes Needed

### Code Changes

1. **Add Halving Logic**:
```rust
fn get_block_reward(stream: StreamType, block_height: u64) -> u128 {
    let halving_period = match stream {
        StreamType::StreamA => 1_261_440,      // 4 years in blocks
        StreamType::StreamB => 126_144_000,   // 4 years in blocks
        StreamType::StreamC => 1_261_440_000, // 4 years in blocks
    };
    let halvings = block_height / halving_period;
    let base_reward = match stream {
        StreamType::StreamA => 50_000_000_000_000_000_000,
        StreamType::StreamB => 20_000_000_000_000_000_000,
        StreamType::StreamC => 5_000_000_000_000_000_000,
    };
    base_reward >> halvings // Divide by 2^halvings
}
```

2. **Add Max Supply Check**:
```rust
const MAX_SUPPLY: u128 = 10_000_000_000_000_000_000_000; // 10B MSHW

fn check_supply_cap(current_supply: u128) -> bool {
    current_supply < MAX_SUPPLY
}
```

3. **Add Dev Fund Allocation**:
```rust
const DEV_FUND_PERCENTAGE: u8 = 10;

fn calculate_rewards(total_reward: u128) -> (u128, u128) {
    let dev_fund = total_reward * DEV_FUND_PERCENTAGE as u128 / 100;
    let miner_reward = total_reward - dev_fund;
    (miner_reward, dev_fund)
}
```

---

## 🎯 Consensus Clarification

### Block Ordering Rules

**Stream Hierarchy**:
1. **Stream A** (10s): Primary anchor blocks
2. **Stream B** (1s): Must reference Stream A blocks
3. **Stream C** (100ms): Must reference Stream B blocks

**Reorg Protection**:
- Stream A: 6 confirmations = 60s finality
- Stream B: 1 Stream A confirmation = 10s finality
- Stream C: 1 Stream B confirmation = 1s finality

**Attack Resistance**:
- Attacking Stream A requires 51% of Stream A hash power
- Attacking Stream B requires 51% of Stream B hash power
- Attacking Stream C requires 51% of Stream C hash power
- All three must be attacked simultaneously for full chain attack

---

## 💡 Additional Improvements

### 1. Fee Burn Mechanism (Optional)

**Proposal**: Burn 50% of transaction fees
- **Deflationary**: Reduces total supply
- **Value Accrual**: Benefits all holders
- **Governance**: Can be enabled/disabled via vote

### 2. Staking Rewards (Future)

**Proposal**: Staking mechanism for validators
- **Reward Source**: Dev fund or additional emission
- **Security**: Additional network security layer
- **Participation**: Broader participation beyond mining

### 3. Governance Token Utility

**Proposal**: MSHW as governance token
- **Voting**: MSHW holders vote on proposals
- **Proposal Costs**: Require MSHW to submit proposals
- **Value**: Additional utility for token

---

## 📊 Comparison: Old vs. New

| Aspect | Old Model | New Model |
|--------|-----------|-----------|
| **Max Supply** | Unlimited | 10 Billion |
| **Halving** | None | Every 4 years |
| **Stream A Reward** | 50 MSHW | 50 MSHW (same) |
| **Stream B Reward** | 25 MSHW | 20 MSHW (reduced) |
| **Stream C Reward** | 0 MSHW | 5 MSHW (NEW) |
| **Dev Fund** | 0% | 10% |
| **Year 1 Emission** | 946M | 2.13B (higher, but capped) |
| **Year 10 Emission** | 946M | 532M (halved) |
| **Inflation Rate (Y10)** | ~10% | ~5.7% |
| **Scarcity Narrative** | None | Strong (halving + cap) |

---

## ✅ Benefits of Revised Model

### Economic
- ✅ **Scarcity**: Halving + cap creates scarcity narrative
- ✅ **Value Accrual**: Dev fund supports project sustainability
- ✅ **Deflationary**: Optional fee burns reduce supply
- ✅ **Predictable**: Clear emission schedule

### Security
- ✅ **Balanced**: Stream rewards more balanced
- ✅ **Subsidized**: Stream C now incentivized
- ✅ **Anchored**: Stream A remains security anchor
- ✅ **Protected**: Reorg protection clarified

### Sustainability
- ✅ **Funded**: Dev fund enables long-term development
- ✅ **Governed**: Multi-sig + future governance
- ✅ **Flexible**: Can adjust via governance
- ✅ **Professional**: Addresses investor concerns

---

## 🚀 Implementation Priority

### Phase 1: Immediate (Testnet)
- [ ] Implement halving logic
- [ ] Add max supply check
- [ ] Implement dev fund allocation
- [ ] Update Stream C rewards (0 → 5 MSHW)
- [ ] Reduce Stream B rewards (25 → 20 MSHW)

### Phase 2: Short-term (Mainnet Prep)
- [ ] Set up dev fund multi-sig wallet
- [ ] Define dev fund governance
- [ ] Implement fee burn (optional)
- [ ] Clarify consensus rules in code
- [ ] Update documentation

### Phase 3: Long-term (Post-Launch)
- [ ] On-chain governance
- [ ] Staking mechanism
- [ ] Community fund allocation
- [ ] Fee burn activation (if desired)

---

## 📝 Summary

**Old Model Issues**:
- ❌ Infinite inflation
- ❌ No halving
- ❌ Stream C unfunded
- ❌ No dev fund
- ❌ No scarcity narrative

**New Model Solutions**:
- ✅ 10B max supply cap
- ✅ Halving every 4 years
- ✅ Stream C subsidized (5 MSHW)
- ✅ 10% dev fund
- ✅ Strong scarcity narrative

**Result**: Sustainable, secure, investor-friendly tokenomics

---

**Ready for implementation!** 💎
