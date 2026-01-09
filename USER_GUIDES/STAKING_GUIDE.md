# Mondoshawan Staking Guide

## 🎯 What is Staking?

Staking allows you to lock Mondoshawan tokens to participate in network validation and earn rewards.

---

## 💡 How Staking Works

### **Validator Staking**
- Lock Mondoshawan tokens
- Participate in GhostDAG consensus
- Validate blocks
- Earn staking rewards
- Risk: Slashing for misbehavior

### **Delegator Staking**
- Delegate Mondoshawan to validators
- Earn rewards (minus validator fee)
- Lower risk (no slashing)
- Can unstake anytime

---

## 🚀 Getting Started

### **Requirements**

```
Minimum Stake: 100,000 Mondoshawan
Hardware: Validator node requirements
Uptime: 99%+ required
Stake Period: Minimum 30 days
```

### **Become a Validator**

```bash
# 1. Set up validator node
Mondoshawan-validator setup

# 2. Stake Mondoshawan
Mondoshawan-validator stake --amount 100000

# 3. Start validator
Mondoshawan-validator start

# 4. Monitor status
Mondoshawan-validator status
```

### **Delegate to Validator**

```bash
# 1. List validators
Mondoshawan-wallet validators

# 2. Delegate to validator
Mondoshawan-wallet delegate \
  --validator validator_id \
  --amount 10000

# 3. View delegation
Mondoshawan-wallet delegations
```

---

## 💰 Staking Rewards

### **Reward Calculation**

```
Annual Reward Rate: 5-10% (variable)
Based on:
- Network participation
- Validator performance
- Total staked amount
- Network fees
```

### **Reward Distribution**

```
Validator Rewards:
- Block rewards: 50%
- Transaction fees: 30%
- Staking rewards: 20%

Delegator Rewards:
- Validator rewards - validator fee (5-10%)
```

### **Example**

```
Stake: 100,000 Mondoshawan
Annual Rate: 8%
Annual Reward: 8,000 Mondoshawan
Monthly Reward: ~667 Mondoshawan
```

---

## ⚠️ Risks & Slashing

### **Slashing Conditions**

```
1. Double Signing
   - Penalty: 100% of stake
   - Automatic slashing

2. Downtime
   - Penalty: 1% per hour
   - Max: 5% per day

3. Invalid Blocks
   - Penalty: 10% of stake
   - Automatic slashing
```

### **Risk Mitigation**

```
✅ Run reliable infrastructure
✅ Monitor validator status
✅ Use backup systems
✅ Keep software updated
✅ Follow best practices
```

---

## 📊 Staking Dashboard

### **View Staking Status**

```bash
# Validator status
Mondoshawan-validator status

# Output:
# Validator ID: validator_123
# Staked: 100,000 Mondoshawan
# Status: Active
# Uptime: 99.5%
# Rewards: 8,000 Mondoshawan/year
# Slashing Risk: Low
```

### **View Rewards**

```bash
# View staking rewards
Mondoshawan-wallet staking-rewards

# Output:
# Total Staked: 100,000 Mondoshawan
# Rewards Earned: 667 Mondoshawan
# Pending Rewards: 50 Mondoshawan
# Annual Rate: 8%
```

---

## 🔄 Unstaking

### **Unstake Process**

```bash
# 1. Request unstake
Mondoshawan-validator unstake --amount 50000

# 2. Wait for unlock period (30 days)
# 3. Withdraw unlocked tokens
Mondoshawan-validator withdraw
```

### **Unlock Period**

```
Unstake Request → 30 Day Lock → Withdraw Available
```

---

## 📚 Best Practices

### **For Validators**
- ✅ Maintain 99%+ uptime
- ✅ Monitor infrastructure
- ✅ Keep software updated
- ✅ Use backup systems
- ✅ Follow security practices

### **For Delegators**
- ✅ Research validators
- ✅ Diversify delegations
- ✅ Monitor validator performance
- ✅ Check validator fees
- ✅ Review slashing history

---

## 🆘 Troubleshooting

### **Validator Offline**
```bash
# Check status
Mondoshawan-validator status

# Restart validator
Mondoshawan-validator restart

# Check logs
Mondoshawan-validator logs
```

### **Low Rewards**
- Check validator performance
- Review network conditions
- Verify stake amount
- Check validator fee

---

## 📖 Additional Resources

- [Validator Setup Guide](VALIDATOR_SETUP.md)
- [Security Guide](SECURITY_GUIDE.md)
- [FAQ](FAQ.md)

---

**Status:** ✅ **Complete Staking Guide**

