# Mondoshawan Tokenomics - Implementation Guide

**Status**: Tokenomics Implemented and Operational  
**Last Updated**: January 2026

---

## ✅ Current Implementation

### Block Rewards (Implemented)

**Location**: `mondoshawan-blockchain/src/mining.rs`

```rust
// Block rewards in base units (1 MSHW = 10^18 base units)
pub const STREAM_A_REWARD: u128 = 50_000_000_000_000_000_000; // 50 MSHW
pub const STREAM_B_REWARD: u128 = 25_000_000_000_000_000_000; // 25 MSHW
pub const STREAM_C_REWARD: u128 = 0; // Fee-based only
```

### Reward Distribution (Implemented)

**Process**:
1. Miner creates block
2. Block validated and added to blockchain
3. Reward immediately added to miner's balance
4. Balance updated in blockchain state

**Code Location**: `mondoshawan-blockchain/src/mining.rs` (block processing)

### Token Decimals (Implemented)

- **Decimals**: 18 (Ethereum-compatible)
- **Base Unit**: 1 MSHW = 1,000,000,000,000,000,000 base units
- **Format**: Same as Ethereum's wei system

---

## 📊 Emission Calculations

### Daily Emission

**Stream A:**
- Block time: 10 seconds
- Blocks per day: 86,400 seconds ÷ 10 = 8,640 blocks
- Reward per block: 50 MSHW
- Daily emission: 8,640 × 50 = **432,000 MSHW/day**

**Stream B:**
- Block time: 1 second
- Blocks per day: 86,400 blocks
- Reward per block: 25 MSHW
- Daily emission: 86,400 × 25 = **2,160,000 MSHW/day**

**Stream C:**
- Block time: 0.1 seconds
- Blocks per day: 864,000 blocks
- Reward per block: 0 MSHW (fees only)
- Daily emission: **0 MSHW/day** (from rewards)

**Total Daily Emission**: 432,000 + 2,160,000 = **2,592,000 MSHW/day**

### Annual Emission

- Daily: 2,592,000 MSHW
- Annual: 2,592,000 × 365 = **946,080,000 MSHW/year**

---

## 🔧 Configuration

### Current Settings (Hardcoded)

Rewards are currently hardcoded in `src/mining.rs`. To modify:

1. **Change Reward Amounts**:
```rust
pub const STREAM_A_REWARD: u128 = 50_000_000_000_000_000_000; // Change 50 to desired amount
pub const STREAM_B_REWARD: u128 = 25_000_000_000_000_000_000; // Change 25 to desired amount
```

2. **Rebuild**:
```bash
cargo build --release
```

### Future: Configurable Rewards

Could be moved to configuration file:
```toml
[mining.rewards]
stream_a = 50_000_000_000_000_000_000  # 50 MSHW
stream_b = 25_000_000_000_000_000_000  # 25 MSHW
stream_c = 0                            # Fee-based only
```

---

## 💰 Fee Collection

### Current Implementation

**Transaction Fees**:
- Collected by miners who include transactions
- Added to block reward (Stream A & B)
- Only source of income (Stream C)
- Fees calculated per transaction

**Code Location**: `mondoshawan-blockchain/src/mining.rs` (fee calculation)

### Fee Structure

**Current**: TBD (gas-based if EVM enabled, or fixed fee)

**Future Options**:
- Gas-based (like Ethereum)
- Fixed fee per transaction
- Dynamic fee based on network load
- Priority fee (for faster inclusion)

---

## 📈 Supply Tracking

### Current Implementation

**Supply Calculation**:
- Total supply = sum of all balances
- Can be queried via RPC: `mds_getTotalSupply` (if implemented)
- Or calculated: sum of all account balances

### Future Enhancements

**Supply Metrics**:
- Total supply tracking
- Circulating supply (total - locked)
- Emission rate monitoring
- Inflation rate calculation

---

## 🎯 Tokenomics Parameters

### Fixed Parameters (Current)

| Parameter | Value | Location |
|-----------|-------|----------|
| Stream A Reward | 50 MSHW | `src/mining.rs` |
| Stream B Reward | 25 MSHW | `src/mining.rs` |
| Stream C Reward | 0 MSHW | `src/mining.rs` |
| Decimals | 18 | Standard (Ethereum-compatible) |
| Initial Supply | 0 | Fair launch |

### Variable Parameters

| Parameter | Current | Notes |
|-----------|---------|-------|
| Transaction Fees | TBD | To be determined |
| Fee Structure | TBD | Gas-based or fixed |
| Halving | None | Could be added |
| Max Supply | Unlimited | Could be capped |

---

## 🔄 Future Tokenomics Features

### Potential Additions

1. **Halving Mechanism**
   - Reduce rewards by 50% every 4 years
   - Implement via block height checks
   - Requires code update

2. **Staking Rewards**
   - If staking mechanism added
   - Additional token emission
   - Requires staking implementation

3. **Burn Mechanism**
   - Deflationary model
   - Burn portion of fees
   - Reduce total supply over time

4. **Governance Token**
   - MSHW used for voting
   - Proposal submission costs
   - Governance rewards

---

## 📝 Implementation Checklist

### ✅ Completed
- [x] Block rewards implemented (50/25/0 MSHW)
- [x] Reward distribution working
- [x] Balance tracking operational
- [x] Fee collection structure in place
- [x] 18 decimals (Ethereum-compatible)

### 🔄 In Progress
- [ ] Fee structure finalized
- [ ] Supply tracking metrics
- [ ] Emission rate monitoring

### 📋 Future
- [ ] Halving mechanism (if desired)
- [ ] Staking rewards (if staking added)
- [ ] Burn mechanism (if deflationary model)
- [ ] Governance integration (if governance added)

---

## 🎯 Tokenomics Summary

**Current Tokenomics:**
- ✅ Fair launch (0 initial supply)
- ✅ Mining rewards only (no pre-mine)
- ✅ TriStream rewards (50/25/0 MSHW)
- ✅ ~2.6M MSHW/day emission
- ✅ Inflationary model (no cap)
- ✅ 18 decimals (Ethereum-compatible)

**Status**: Operational on testnet, ready for mainnet (when ready)

---

**Tokenomics are implemented and working!** 💎
