# Miner's Transition Guide: Algorithm Rotation Phase III

**180-Day Implementation Window**  
**Last Updated**: January 2026

---

## 🎯 Overview

When an Algorithm Rotation (AR) proposal is approved, a **6-month (180-day) transition window** begins. This guide helps miners navigate the transition from the current algorithm (e.g., Blake3) to the new algorithm (e.g., RandomX).

**Purpose**: Give honest miners time to:
- Amortize ASIC hardware costs
- Transition to other chains
- Retire or repurpose hardware
- Prepare for the new algorithm

---

## 📅 Timeline

### Day 0: AR Approval
- **Event**: Proposal passes with 66% majority
- **Action**: 180-day countdown begins
- **Notification**: All nodes receive SEP broadcast
- **Explorer**: Shows countdown timer and sunset block height

### Days 1-90: Early Transition Period
- **Status**: Both algorithms active (optional parallel mining)
- **Recommendation**: Start planning transition
- **Action**: Evaluate hardware compatibility

### Days 91-150: Active Transition Period
- **Status**: New algorithm testing and preparation
- **Recommendation**: Begin hardware migration
- **Action**: Test new algorithm setup

### Days 151-180: Final Countdown
- **Status**: Final preparation phase
- **Recommendation**: Complete migration
- **Action**: Ensure new algorithm is ready

### Day 180: Sunset Block
- **Event**: Hard fork - Stream A switches to new algorithm
- **Status**: Old algorithm blocks no longer accepted
- **Action**: New algorithm must be active

---

## 🔧 Hardware Compatibility

### Blake3 → RandomX Transition

**Blake3 ASICs**:
- ❌ **Not compatible** with RandomX
- ✅ **Options**:
  - Sell hardware (while value remains)
  - Switch to other Blake3 chains
  - Repurpose for other uses
  - Retire hardware

**CPU/GPU Mining**:
- ✅ **Compatible** with RandomX
- ✅ **Advantage**: No hardware change needed
- ⚠️ **Note**: May need software update

**Recommendation**: Start testing RandomX on existing CPU/GPU hardware during transition period

---

### Blake3 → Other Algorithm Transitions

**To ProgPoW**:
- ❌ ASICs: Not compatible
- ✅ GPUs: Compatible (optimized for GPUs)
- ✅ CPUs: Compatible (less efficient)

**To Equihash**:
- ❌ ASICs: Not compatible
- ✅ GPUs: Compatible
- ✅ CPUs: Compatible (memory-hard)

**To RandomX**:
- ❌ ASICs: Not compatible
- ✅ CPUs: Optimized for CPUs
- ⚠️ GPUs: Compatible but less efficient

---

## 💰 Financial Planning

### Hardware Amortization

**ASIC Hardware**:
- **Typical Lifespan**: 2-4 years
- **Amortization**: Calculate remaining value
- **Decision Point**: 
  - If < 6 months remaining: Continue mining
  - If > 6 months remaining: Consider selling

**Example Calculation**:
```
ASIC Cost: $5,000
Purchase Date: 1 year ago
Expected Lifespan: 3 years
Remaining Value: $3,333 (2 years remaining)
Decision: If transition is < 6 months, continue mining
```

### Revenue Projection

**Current Algorithm (Blake3)**:
- Daily revenue: 432,000 MSHW/day (Stream A)
- Remaining days: 180
- Total remaining: ~77.76M MSHW

**New Algorithm (RandomX)**:
- Daily revenue: TBD (depends on difficulty)
- Expected: Similar or higher (less competition initially)

**Recommendation**: Model both scenarios to make informed decision

---

## 🔄 Transition Strategies

### Strategy 1: Immediate Migration (Days 1-90)

**Best For**: CPU/GPU miners, flexible operations

**Steps**:
1. Test new algorithm on testnet
2. Update mining software
3. Switch to new algorithm early
4. Gain experience before sunset

**Advantages**:
- Early experience with new algorithm
- Potentially higher rewards (less competition)
- Smooth transition

**Disadvantages**:
- Miss remaining Blake3 rewards
- New algorithm may have issues

---

### Strategy 2: Gradual Transition (Days 91-150)

**Best For**: ASIC miners, conservative operations

**Steps**:
1. Continue mining Blake3 (maximize revenue)
2. Set up new algorithm in parallel
3. Test new algorithm gradually
4. Complete migration by day 150

**Advantages**:
- Maximize Blake3 revenue
- Lower risk
- Time to test

**Disadvantages**:
- More complex setup
- Higher operational costs

---

### Strategy 3: Last-Minute Migration (Days 151-180)

**Best For**: Miners planning to exit

**Steps**:
1. Continue mining Blake3 until day 179
2. Sell hardware before sunset
3. Exit network or switch to other chains

**Advantages**:
- Maximize revenue from current algorithm
- Exit cleanly

**Disadvantages**:
- No experience with new algorithm
- Hardware value may decrease

---

## 📊 Decision Matrix

### Should I Continue Mining?

| Factor | Continue | Exit |
|--------|----------|------|
| **Hardware Compatibility** | Compatible with new algo | Not compatible |
| **Remaining Hardware Value** | > 6 months | < 6 months |
| **Operational Flexibility** | High | Low |
| **Revenue Projection** | Positive | Negative |
| **Risk Tolerance** | High | Low |

**Recommendation**: Use this matrix to make informed decision

---

## 🛠️ Technical Migration Steps

### Step 1: Test New Algorithm (Day 1-30)

```bash
# Join testnet with new algorithm
./mondoshawan-node --testnet --algorithm randomx

# Test mining
./mondoshawan-miner --stream A --algorithm randomx

# Verify block production
curl http://localhost:8545 -d '{"method":"mds_getDagStats"}'
```

**Checklist**:
- [ ] Node syncs with new algorithm
- [ ] Mining software works
- [ ] Blocks are accepted
- [ ] Rewards are received

---

### Step 2: Parallel Mining Setup (Day 31-90)

**Option A: Dual Mining** (if compatible):
```bash
# Run both algorithms in parallel
./mondoshawan-miner --stream A --algorithm blake3 &
./mondoshawan-miner --stream A --algorithm randomx &
```

**Option B: Time-Based Switching**:
```bash
# Mine Blake3 during peak hours
# Mine RandomX during off-peak hours
# Gradually increase RandomX time
```

---

### Step 3: Full Migration (Day 91-150)

```bash
# Stop Blake3 mining
pkill -f "blake3"

# Start RandomX mining
./mondoshawan-miner --stream A --algorithm randomx

# Monitor performance
watch -n 1 './mondoshawan-stats'
```

**Checklist**:
- [ ] Blake3 mining stopped
- [ ] RandomX mining active
- [ ] Blocks being produced
- [ ] Rewards being received
- [ ] Performance acceptable

---

### Step 4: Pre-Sunset Verification (Day 151-179)

```bash
# Verify new algorithm is ready
./mondoshawan-node --verify-algorithm randomx

# Check sunset block height
curl http://localhost:8545 -d '{
  "method": "mds_getSunsetBlock",
  "params": []
}'

# Ensure node is updated
./mondoshawan-node --version
```

**Checklist**:
- [ ] Node software updated
- [ ] New algorithm tested
- [ ] Sunset block height known
- [ ] Backup plan ready

---

### Step 5: Sunset Block (Day 180)

```bash
# At sunset block height:
# 1. Old algorithm stops working
# 2. New algorithm becomes mandatory
# 3. Network hard forks

# Monitor transition
./mondoshawan-node --monitor-fork

# Verify new blocks
curl http://localhost:8545 -d '{
  "method": "eth_getBlockByNumber",
  "params": ["latest", false]
}'
```

**Checklist**:
- [ ] Node switched to new algorithm
- [ ] Blocks being produced
- [ ] Network consensus maintained
- [ ] Rewards being received

---

## 💡 Best Practices

### 1. Start Early
- Don't wait until day 179
- Begin testing in first 30 days
- Identify issues early

### 2. Test Thoroughly
- Use testnet extensively
- Test on mainnet (small scale)
- Verify all components

### 3. Monitor Closely
- Track sunset block height
- Monitor network status
- Watch for announcements

### 4. Have Backup Plan
- Know your exit strategy
- Have hardware buyers lined up
- Consider other chains

### 5. Stay Informed
- Follow governance updates
- Join community discussions
- Monitor explorer for countdown

---

## 🚨 Common Pitfalls

### Pitfall 1: Waiting Too Long
**Problem**: Starting migration on day 179  
**Solution**: Begin testing in first 30 days

### Pitfall 2: Incomplete Testing
**Problem**: Not testing all components  
**Solution**: Test end-to-end before sunset

### Pitfall 3: Hardware Incompatibility
**Problem**: Assuming hardware will work  
**Solution**: Verify compatibility early

### Pitfall 4: Ignoring Announcements
**Problem**: Missing critical updates  
**Solution**: Monitor official channels

### Pitfall 5: No Exit Strategy
**Problem**: Stuck with incompatible hardware  
**Solution**: Plan exit strategy early

---

## 📞 Support Resources

### Official Channels
- **Explorer**: Check sunset block countdown
- **RPC**: `mds_getSunsetBlock` - Get sunset block height
- **RPC**: `mds_getArStatus` - Get AR proposal status
- **Documentation**: Governance charter and technical docs

### Community Resources
- **Forums**: Community discussions
- **Discord/Telegram**: Real-time support
- **GitHub**: Technical issues and updates

---

## 📋 Pre-Sunset Checklist

### 30 Days Before Sunset
- [ ] New algorithm tested on testnet
- [ ] Hardware compatibility verified
- [ ] Software updated
- [ ] Migration plan finalized

### 14 Days Before Sunset
- [ ] New algorithm running on mainnet (test)
- [ ] Performance verified
- [ ] Backup plan ready
- [ ] Exit strategy confirmed (if needed)

### 7 Days Before Sunset
- [ ] Final software update installed
- [ ] Sunset block height confirmed
- [ ] Monitoring systems ready
- [ ] Support contacts identified

### 1 Day Before Sunset
- [ ] All systems verified
- [ ] Backup systems ready
- [ ] Team briefed (if applicable)
- [ ] Monitoring active

### Sunset Day
- [ ] Node ready for new algorithm
- [ ] Monitoring active
- [ ] Support available
- [ ] Post-sunset verification planned

---

## 🎯 Post-Sunset

### Immediate Actions (Day 181-190)
- [ ] Verify node is on new algorithm
- [ ] Check block production
- [ ] Verify rewards
- [ ] Monitor network stability

### Short-Term (Days 191-210)
- [ ] Optimize new algorithm setup
- [ ] Adjust mining parameters
- [ ] Monitor performance
- [ ] Report issues

### Long-Term (Days 211+)
- [ ] Evaluate profitability
- [ ] Consider optimizations
- [ ] Plan for future
- [ ] Share experience with community

---

## 📊 Transition Metrics

### Track These Metrics

**Before Sunset**:
- Blake3 mining revenue
- Hardware utilization
- Preparation progress

**After Sunset**:
- New algorithm mining revenue
- Hardware performance
- Network participation
- Reward distribution

**Comparison**:
- Revenue change
- Performance change
- Network health
- Miner satisfaction

---

## ⚠️ Important Notes

### Hard Fork Nature
- **Irreversible**: Once sunset block is reached, old algorithm is permanently disabled
- **Network Split**: Nodes not updated will be on separate chain
- **No Rollback**: Cannot revert to old algorithm

### Sunset Block Height
- **Announced**: 180 days before sunset
- **Fixed**: Cannot be changed
- **Verifiable**: Check via RPC `mds_getSunsetBlock`

### Network Stability
- **Initial Period**: May have lower hashrate
- **Difficulty Adjustment**: Will adjust automatically
- **Rewards**: May be higher initially (less competition)

---

## 🎓 Educational Resources

### Algorithm-Specific Guides

**RandomX**:
- CPU optimization guide
- Memory requirements
- Performance tuning

**ProgPoW**:
- GPU optimization
- Hashrate expectations
- Power consumption

**Equihash**:
- Memory-hard mining
- GPU requirements
- Algorithm specifics

---

## 📝 Summary

**Key Points**:
1. **180-day window**: Plenty of time for transition
2. **Start early**: Don't wait until last minute
3. **Test thoroughly**: Verify everything works
4. **Plan exit**: Know your options
5. **Stay informed**: Monitor updates

**Success Factors**:
- Early preparation
- Thorough testing
- Clear decision-making
- Community support

---

## 🚀 Ready for Transition

**Status**: ✅ **Guide complete, ready for use**

When an AR proposal is approved, miners will have:
- ✅ Clear timeline
- ✅ Technical migration steps
- ✅ Financial planning tools
- ✅ Decision matrix
- ✅ Support resources

**The 180-day window ensures a fair, orderly transition for all miners.** ⚖️

---

**Miner's Transition Guide complete!** 🎯
