# Pump.fun Token Supply - Important Clarification

**Issue**: Pump.fun doesn't appear to let you set a fixed token supply (like 300M tokens)

---

## 🔍 How Pump.fun Actually Works

### **Bonding Curve Model** (Not Fixed Supply)

**Reality**: Pump.fun uses a **bonding curve**, not a fixed supply model.

**How It Works**:
1. **No Fixed Supply Setting**: You cannot set "300M tokens" directly
2. **Bonding Curve**: Price starts very low, increases as tokens are bought
3. **Auto-Listing**: When $85,000 is raised OR bonding curve completes → auto-lists on Raydium
4. **Supply Determined by Curve**: Total supply depends on how the bonding curve plays out

**Key Point**: Pump.fun doesn't let you pre-set a fixed supply like "300M tokens"

---

## 💡 Alternative Approaches

### **Option 1: Use Description to Communicate Supply** ⭐ RECOMMENDED

**What You Can Do**:
- Put the supply information in the **description** field
- Explain the presale structure
- Set expectations clearly

**Example Description**:
```
Presale token for Mondoshawan Protocol (MSHW).

This presale represents 300,000,000 MSHW tokens (3% of total supply).
1:1 migration to native MSHW on mainnet launch.

Presale Details:
- Total Allocation: 300M MSHW-PRESALE
- Price: $0.001 per token
- Target Raise: $300,000
- Max per Address: 10M tokens

Website: mondoshawan.network
Whitepaper: mondoshawan.io/Mondoshawan_WHITEPAPER.html
```

**Limitation**: This is informational only - Pump.fun's bonding curve will still determine actual supply

---

### **Option 2: Create Token with Custom Supply (Outside Pump.fun)**

**If You Need Exact 300M Supply**:

1. **Create SPL Token Directly** (Not on Pump.fun):
   ```bash
   # Using Solana CLI
   spl-token create-token --decimals 18
   # Returns: Token mint address
   
   spl-token create-account <TOKEN_MINT>
   spl-token mint <TOKEN_MINT> 300000000
   # Mints exactly 300M tokens
   ```

2. **Then List on Pump.fun** (If Possible):
   - Some platforms allow listing existing tokens
   - Or: Create liquidity pool on Raydium directly

**Pros**:
- ✅ Exact supply control (300M)
- ✅ Fixed price possible
- ✅ More control

**Cons**:
- ⚠️ No Pump.fun viral marketing
- ⚠️ Must build audience yourself
- ⚠️ More complex setup

---

### **Option 3: Accept Pump.fun's Bonding Curve Model**

**Reality Check**:
- Pump.fun is designed for meme coins with bonding curves
- It's optimized for viral launches, not fixed-supply presales
- The bonding curve model may not match your presale structure

**What This Means**:
- Early buyers get better prices
- Later buyers pay more
- Total supply depends on curve completion
- May not reach exactly 300M tokens

**Strategy**:
- Use Pump.fun for initial launch and marketing
- Accept that supply will be determined by bonding curve
- Focus on raising funds rather than exact token count
- Migrate based on funds raised, not token count

---

## 🎯 Recommended Approach

### **For Your Presale: Hybrid Model**

**Step 1: Launch on Pump.fun** (For Marketing)
- Use Pump.fun for viral launch
- Accept bonding curve model
- Focus on raising $300k (not exact token count)
- Use description to explain presale structure

**Step 2: Track Actual Supply**
- Monitor how many tokens are actually created
- Track funds raised
- Adjust migration ratio if needed

**Step 3: Migration Strategy**
- Option A: 1:1 migration based on tokens held
- Option B: Migration based on funds contributed (if supply differs)
- Option C: Fixed migration ratio (e.g., 1 MSHW-PRESALE = 1 native MSHW, regardless of purchase price)

**Example Migration**:
```
If Pump.fun creates 250M tokens (not 300M):
- Still migrate 1:1 to native MSHW
- Remaining 50M MSHW allocated from presale pool
- Or: Adjust based on actual funds raised
```

---

## 📝 Updated Pump.fun Description

**Recommended Description for Pump.fun**:

```
🚀 Mondoshawan Protocol Presale

Presale token representing future allocation of native MSHW tokens.

📊 Presale Details:
• Total Allocation: 300,000,000 MSHW (3% of 10B max supply)
• Target Raise: $300,000 USD
• Price: ~$0.001 per MSHW (via bonding curve)
• Max per Address: 10,000,000 MSHW

🔄 Migration:
1:1 migration to native MSHW on mainnet launch.
All presale holders will receive native MSHW tokens.

💎 About Mondoshawan:
• Next-gen Layer 1 blockchain
• TriStream mining (3 parallel streams)
• 160,000+ TPS with sharding
• Post-quantum cryptography
• 97% fair launch (3% presale, 0% team)

🌐 Links:
Website: mondoshawan.network
Whitepaper: mondoshawan.io/Mondoshawan_WHITEPAPER.html
Twitter: @MondoshawanMSHW

⚠️ Important:
This is a presale token. Mainnet launch and migration are planned but not guaranteed. Do your own research.
```

---

## 🔄 Alternative: Direct SPL Token Creation

**If Pump.fun Doesn't Work for Your Needs**:

### **Create Token Directly on Solana**

**Using Solana CLI**:
```bash
# 1. Create token with 18 decimals
spl-token create-token --decimals 18
# Returns: Token mint address (save this!)

# 2. Create token account
spl-token create-account <TOKEN_MINT>

# 3. Mint exactly 300M tokens
spl-token mint <TOKEN_MINT> 300000000

# 4. Create liquidity pool on Raydium
# (Use Raydium interface or program)
```

**Then**:
- Create liquidity pool on Raydium ($80k MSHW/USDC)
- Market the token yourself
- Set fixed price or use AMM

**Pros**:
- ✅ Exact 300M supply
- ✅ Fixed price possible
- ✅ Full control

**Cons**:
- ⚠️ No Pump.fun viral marketing
- ⚠️ Must build audience
- ⚠️ More work

---

## 💡 Key Insight

**Pump.fun Reality**:
- Designed for meme coins with bonding curves
- Not designed for fixed-supply presales
- Supply is determined by bonding curve, not preset

**Your Options**:
1. **Use Pump.fun** - Accept bonding curve, focus on funds raised
2. **Create Directly** - Full control, but no Pump.fun marketing
3. **Hybrid** - Launch on Pump.fun, then migrate based on actual supply

---

## ✅ Recommended Action

**For Your Situation**:

1. **Use Pump.fun** (For Marketing):
   - Launch with bonding curve
   - Put supply info in description
   - Focus on raising $300k
   - Accept that supply may vary

2. **Migration Strategy**:
   - Track actual tokens created
   - Migrate 1:1 based on tokens held
   - Or: Migrate based on funds contributed
   - Adjust if needed

3. **Communication**:
   - Be transparent about bonding curve
   - Explain migration will be 1:1
   - Set clear expectations

**Result**: You get Pump.fun's marketing power, even if supply isn't exactly 300M

---

## 📋 Updated Checklist

### **Before Launch**:
- [ ] Understand Pump.fun uses bonding curve (not fixed supply)
- [ ] Write clear description explaining presale structure
- [ ] Set migration strategy (1:1 based on tokens or funds)
- [ ] Prepare to track actual supply created

### **During Presale**:
- [ ] Monitor tokens created via bonding curve
- [ ] Track funds raised
- [ ] Communicate with community
- [ ] Adjust strategy if needed

### **After Presale**:
- [ ] Calculate actual tokens created
- [ ] Determine migration ratio
- [ ] Execute migration to native MSHW
- [ ] Communicate clearly to holders

---

**Bottom Line**: Pump.fun doesn't let you set fixed supply, but you can still use it for marketing and adjust your migration strategy accordingly.

---

**Last Updated**: January 2026  
**Status**: Internal Guide - Not for Public Repository
