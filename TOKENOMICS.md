# Mondoshawan Protocol - Tokenomics

**Token Name**: Mondoshawan  
**Ticker**: MSHW  
**Status**: Testnet | Fair Launch  
**Last Updated**: January 2026

---

## 📊 Token Overview

### Basic Information
- **Token Name**: Mondoshawan
- **Ticker Symbol**: MSHW
- **Decimals**: 18 (Ethereum-compatible)
- **Base Unit**: 1 MSHW = 1,000,000,000,000,000,000 base units (10^18)
- **Chain ID**: 0x4D534857 (testnet) / TBD (mainnet)

### Supply Model
- **Type**: Inflationary (no hard cap)
- **Initial Supply**: 0 (fair launch, no pre-mine)
- **Emission**: Mining rewards only
- **Max Supply**: Unlimited (inflationary model)

---

## ⛏️ Mining Rewards

### TriStream Architecture

Mondoshawan uses a unique three-stream mining system:

#### Stream A: ASIC Mining (Blake3)
- **Block Time**: 10 seconds
- **Max Transactions**: 10,000 per block
- **Block Reward**: **50 MSHW**
- **Daily Emission**: ~432,000 MSHW
  - Calculation: 50 MSHW × 8,640 blocks/day = 432,000 MSHW/day
- **Annual Emission**: ~157,680,000 MSHW
- **Purpose**: High-throughput transaction processing

#### Stream B: CPU/GPU Mining (KHeavyHash)
- **Block Time**: 1 second
- **Max Transactions**: 5,000 per block
- **Block Reward**: **25 MSHW**
- **Daily Emission**: ~2,160,000 MSHW
  - Calculation: 25 MSHW × 86,400 blocks/day = 2,160,000 MSHW/day
- **Annual Emission**: ~788,400,000 MSHW
- **Purpose**: Fast transaction confirmation, accessibility

#### Stream C: ZK Proof Validation
- **Block Time**: 100 milliseconds (0.1 seconds)
- **Max Transactions**: 1,000 per block
- **Block Reward**: **0 MSHW** (fee-based only)
- **Daily Emission**: 0 MSHW (from rewards)
- **Annual Emission**: 0 MSHW (from rewards)
- **Purpose**: Ultra-fast finality, fee collection

### Total Daily Emission
- **From Block Rewards**: ~2,592,000 MSHW/day
  - Stream A: 432,000 MSHW/day
  - Stream B: 2,160,000 MSHW/day
  - Stream C: 0 MSHW/day
- **From Transaction Fees**: Variable (collected by miners)
- **Annual Emission**: ~946,080,000 MSHW/year (from block rewards)

---

## 💰 Reward Distribution

### Block Rewards
- **Stream A**: Miner receives 50 MSHW immediately upon block acceptance
- **Stream B**: Miner receives 25 MSHW immediately upon block acceptance
- **Stream C**: Miner receives sum of all transaction fees in the block

### Transaction Fees
- All streams collect transaction fees
- Fees go to the miner who includes the transaction in their block
- Stream C miners rely entirely on fees (no block reward)
- Fee structure: TBD (gas-based if EVM enabled, or fixed fee)

### Reward Mechanics
1. Miner creates and validates block
2. Block is added to blockchain
3. Reward/fees immediately added to miner's balance
4. Balance updated in blockchain state
5. Miner can use rewards immediately

---

## 📈 Supply Projections

### Year 1 Projections
- **Daily Emission**: ~2,592,000 MSHW
- **Monthly Emission**: ~77,760,000 MSHW
- **Annual Emission**: ~946,080,000 MSHW
- **End of Year 1 Supply**: ~946,080,000 MSHW (assuming no other sources)

### Long-Term Projections
- **Year 2**: ~1,892,160,000 MSHW total
- **Year 3**: ~2,838,240,000 MSHW total
- **Year 5**: ~4,730,400,000 MSHW total
- **Year 10**: ~9,460,800,000 MSHW total

*Note: These projections assume constant block times and no halving mechanism*

---

## 🔄 Inflation Model

### Current Model: Constant Inflation
- **No Halving**: Rewards remain constant (no reduction over time)
- **Inflation Rate**: Decreases as supply grows
  - Year 1: ~100% (946M / 0 initial)
  - Year 2: ~50% (946M / 1,892M existing)
  - Year 5: ~20% (946M / 4,730M existing)
  - Year 10: ~10% (946M / 9,460M existing)

### Alternative Models (Future Consideration)
- **Halving Every 4 Years**: Reduce rewards by 50% every 4 years
- **Gradual Reduction**: Small annual reduction (e.g., 1% per year)
- **Supply Cap**: Implement maximum supply (e.g., 10 billion MSHW)

---

## 🎯 Token Utility

### Primary Uses
1. **Transaction Fees**: Pay for blockchain transactions
2. **Smart Contract Gas**: Execute EVM smart contracts
3. **Mining Rewards**: Incentivize network security
4. **Staking** (Future): Potential staking mechanism
5. **Governance** (Future): Potential governance token

### Value Drivers
- **Network Security**: Mining rewards incentivize participation
- **Transaction Demand**: Fees create demand for MSHW
- **Smart Contract Usage**: Gas fees drive utility
- **Scarcity**: Inflationary but decreasing inflation rate
- **Utility**: Essential for using the network

---

## 📊 Distribution Breakdown

### Initial Distribution
- **Pre-mine**: 0 MSHW (fair launch)
- **ICO/IDO**: None (no token sale)
- **Team Allocation**: 0 MSHW (fair launch)
- **Advisor Allocation**: 0 MSHW (fair launch)
- **Community Fund**: 0 MSHW (fair launch)
- **Total Initial**: 0 MSHW

### Ongoing Distribution
- **100% Mining Rewards**: All tokens come from mining
- **No Reserved Supply**: Everything is earned through participation
- **Fair Launch**: Equal opportunity for all participants

---

## 🔐 Security & Fairness

### Fair Launch Principles
- ✅ **No Pre-mine**: Zero tokens allocated before launch
- ✅ **No ICO/IDO**: No token sale or fundraising
- ✅ **Open Mining**: Anyone can participate
- ✅ **Equal Opportunity**: Multiple mining streams for different hardware
- ✅ **Transparent**: All code open source

### Anti-Centralization
- **TriStream Design**: Prevents single hardware type dominance
- **Multiple Algorithms**: ASIC, CPU/GPU, ZK proofs
- **Decentralized Mining**: No mining pools required (but allowed)
- **Accessible**: CPU/GPU mining enables broad participation

---

## 💡 Economic Model

### Mining Economics

**Stream A (ASIC) Economics:**
- **Reward**: 50 MSHW per block
- **Frequency**: Every 10 seconds
- **Daily Potential**: ~432,000 MSHW (at 100% success rate)
- **Best For**: High hash power ASIC miners
- **Competition**: Higher (ASIC-optimized)

**Stream B (CPU/GPU) Economics:**
- **Reward**: 25 MSHW per block
- **Frequency**: Every 1 second
- **Daily Potential**: ~2,160,000 MSHW (at 100% success rate)
- **Best For**: CPU/GPU miners, more decentralized
- **Competition**: Medium (accessible hardware)

**Stream C (ZK Proofs) Economics:**
- **Reward**: Transaction fees only
- **Frequency**: Every 100ms
- **Daily Potential**: Variable (depends on fees)
- **Best For**: ZK proof generators, fee maximizers
- **Competition**: Lower (specialized)

### Fee Economics
- **Transaction Fees**: Collected by miners
- **Fee Structure**: TBD (gas-based or fixed)
- **Fee Market**: Determined by network demand
- **Stream C Reliance**: Stream C miners depend entirely on fees

---

## 📅 Emission Schedule

### Current Schedule (No Halving)
- **Year 1-∞**: Constant emission
  - Stream A: 50 MSHW/block
  - Stream B: 25 MSHW/block
  - Stream C: 0 MSHW/block (fees only)

### Future Considerations
- **Halving Mechanism**: Could be implemented via governance
- **Emission Reduction**: Could reduce rewards over time
- **Supply Cap**: Could implement maximum supply
- **Governance**: Community could vote on changes

---

## 🎯 Tokenomics Summary

### Key Metrics
- **Initial Supply**: 0 MSHW
- **Daily Emission**: ~2,592,000 MSHW
- **Annual Emission**: ~946,080,000 MSHW
- **Inflation Model**: Constant emission, decreasing inflation rate
- **Distribution**: 100% mining rewards
- **Fair Launch**: ✅ Yes (no pre-mine, no ICO)

### Unique Features
- **TriStream Mining**: Three parallel reward streams
- **Multiple Hardware Support**: ASIC, CPU/GPU, ZK proofs
- **Fee-Based Stream**: Stream C relies on fees only
- **Fair Launch**: Equal opportunity for all

### Economic Principles
- **Decentralization**: Multiple mining streams prevent centralization
- **Accessibility**: CPU/GPU mining enables broad participation
- **Sustainability**: Constant rewards maintain network security
- **Utility**: Essential for network operations

---

## 📝 Implementation Status

### ✅ Implemented
- Block rewards (50 MSHW Stream A, 25 MSHW Stream B)
- Fee collection (all streams)
- Reward distribution (immediate upon block acceptance)
- Balance tracking (miner balances updated)

### 🔄 Future Considerations
- Halving mechanism (if desired)
- Staking rewards (if staking implemented)
- Governance token utility (if governance added)
- Burn mechanism (if deflationary model desired)

---

## 🔮 Long-Term Vision

### Economic Sustainability
- **Network Security**: Rewards maintain strong hash power
- **Decentralization**: Multiple streams prevent centralization
- **Accessibility**: Broad participation through multiple hardware types
- **Utility**: Essential token for network operations

### Potential Evolutions
- **Governance**: MSHW could become governance token
- **Staking**: Potential staking mechanism for validators
- **Burns**: Could implement burn mechanism for deflation
- **Emission Adjustments**: Community could adjust via governance

---

**Tokenomics are live and operational on testnet!** 💎
