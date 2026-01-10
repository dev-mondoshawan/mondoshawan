# Presale Token Launch Guide - Pump.fun & Alternatives

**Goal**: Launch MSHW presale token on Solana to raise $300k for development

---

## 🎯 Platform Comparison

### **Pump.fun** ⭐ RECOMMENDED

**What It Is**: 
- Viral token launchpad on Solana
- Meme coin focused but works for any token
- Bonding curve mechanism (price increases as tokens sell)

**Token Supply Control**:
- ✅ **Full Control**: You set total supply (300M tokens)
- ✅ **Custom Decimals**: Set to 18 (matches MSHW)
- ✅ **Custom Metadata**: Name, symbol, description, image
- ✅ **No Code Required**: Web interface only

**How It Works**:
1. Create token with custom supply
2. Set initial price (bonding curve starts here)
3. Tokens sell as people buy (price increases)
4. When 100% sold OR $85k raised → auto-lists on Raydium
5. You get SOL from sales

**Costs**:
- **Setup**: ~$50-100 (Solana transaction fees)
- **Platform Fee**: 1% of sales (standard)
- **Total Cost**: Very low (~$100-200 total)

**Pros**:
- ✅ Easiest to use (no coding)
- ✅ Viral potential (Pump.fun has huge audience)
- ✅ Fast launch (minutes to set up)
- ✅ Full control over supply
- ✅ Auto-listing on Raydium when complete

**Cons**:
- ⚠️ Meme coin association (but you can brand professionally)
- ⚠️ Bonding curve model (price increases as tokens sell)
- ⚠️ Less professional than Jupiter

**Best For**: Quick launch, viral marketing, community-driven projects

---

### **Jupiter Launchpad** (Alternative)

**What It Is**:
- Professional token launchpad on Solana
- More enterprise-focused
- Better for serious projects

**Token Supply Control**:
- ✅ **Full Control**: Custom supply
- ✅ **More Options**: Vesting, allocations, etc.
- ✅ **Professional**: Better branding

**Costs**:
- **Setup**: $2,000-5,000
- **Platform Fee**: 2-5% of sales
- **Total Cost**: Higher but more professional

**Pros**:
- ✅ Professional branding
- ✅ More features (vesting, allocations)
- ✅ Better for serious projects
- ✅ Less meme coin association

**Cons**:
- ⚠️ Higher cost
- ⚠️ Smaller initial audience
- ⚠️ More complex setup

**Best For**: Professional projects, institutional investors, complex tokenomics

---

### **Raydium Launchpad** (Alternative)

**What It Is**:
- DEX with launchpad features
- More DeFi-focused
- Requires liquidity provision

**Token Supply Control**:
- ✅ **Full Control**: Custom supply
- ⚠️ **Requires Liquidity**: Must provide initial liquidity

**Costs**:
- **Setup**: $1,000-3,000
- **Liquidity**: Must provide (e.g., $10k-50k)
- **Platform Fee**: 1-2%

**Pros**:
- ✅ Immediate DEX listing
- ✅ Liquidity from start
- ✅ Professional

**Cons**:
- ⚠️ Requires capital for liquidity
- ⚠️ More complex
- ⚠️ Less viral potential

**Best For**: Projects with capital, immediate trading needs

---

### **Direct Launch** (Alternative)

**What It Is**:
- Create SPL token yourself
- Launch on your own website
- Full control

**Token Supply Control**:
- ✅ **Full Control**: Everything custom
- ✅ **No Platform Fees**: Keep 100%
- ✅ **Professional**: Your branding

**Costs**:
- **Setup**: $500-2,000 (development)
- **Marketing**: $5,000-20,000 (to build audience)
- **Total Cost**: Lower fees, higher marketing

**Pros**:
- ✅ Full control
- ✅ No platform fees
- ✅ Professional
- ✅ Direct relationship with buyers

**Cons**:
- ⚠️ Must build audience yourself
- ⚠️ More work (website, payment processing)
- ⚠️ Less viral potential

**Best For**: Projects with existing audience, technical team

---

## 🚀 Pump.fun Step-by-Step Guide

### **Phase 1: Preparation (Before Launch)**

#### **Step 1: Create Solana Wallet**
```bash
# Option 1: Use Phantom wallet (browser extension)
# Download: https://phantom.app
# Create new wallet
# Save seed phrase securely

# Option 2: Use Solana CLI
solana-keygen new --outfile ~/.config/solana/presale-keypair.json
```

**What You Need**:
- ✅ Solana wallet (Phantom recommended)
- ✅ Some SOL for fees (~1-2 SOL = $100-200)
- ✅ Seed phrase backed up securely

---

#### **Step 2: Prepare Token Details**

**Token Information**:
- **Name**: "Mondoshawan Presale" or "MSHW Presale"
- **Symbol**: "MSHW-PRESALE" or "MSHW-P"
- **Description**: 
  ```
  Presale token for Mondoshawan Protocol (MSHW).
  Represents future allocation of native MSHW tokens.
  1:1 migration to native MSHW on mainnet launch.
  
  Total Supply: 300,000,000 MSHW-PRESALE
  Presale Price: $0.001 per token
  Target Raise: $300,000
  
  Website: mondoshawan.network
  Whitepaper: [link]
  ```
- **Image**: Logo/banner (512x512px recommended)
- **Total Supply**: 300,000,000 (300M tokens)
- **Decimals**: 18 (matches native MSHW)

**Create These Assets**:
- Token logo (512x512px PNG)
- Banner image (1200x600px PNG)
- Description text (ready to paste)

---

#### **Step 3: Set Up Multi-Sig Wallet**

**Why**: Security for presale funds

**Options**:
1. **Phantom Multi-Sig** (Easiest)
   - Create multi-sig wallet in Phantom
   - Set up 2-of-3 or 3-of-5 signers
   - Use for receiving presale funds

2. **Squads Protocol** (More Features)
   - Professional multi-sig on Solana
   - Better for larger amounts
   - More control

**Setup**:
- Create multi-sig with 3-5 signers
- Store signer keys securely (separate locations)
- Test with small amount first

---

#### **Step 4: Legal Preparation**

**Checklist**:
- [ ] Terms of sale document
- [ ] Privacy policy
- [ ] KYC/AML requirements (if needed)
- [ ] Regulatory review (consult lawyer)
- [ ] Tax implications
- [ ] Disclaimers (investment risks)

**Key Disclaimers**:
- "This is a presale token, not a security"
- "No guarantee of mainnet launch"
- "Migration is not guaranteed"
- "Do your own research"

---

### **Phase 2: Launch on Pump.fun**

#### **Step 1: Go to Pump.fun**

**Website**: https://pump.fun

**What You'll See**:
- "Create" button (top right)
- Connect wallet option
- Token creation form

---

#### **Step 2: Connect Wallet**

1. Click "Connect Wallet"
2. Select Phantom (or your wallet)
3. Approve connection
4. Wallet connected ✅

---

#### **Step 3: Create Token**

**Click "Create" Button**

**Fill Out Form**:

1. **Token Name**:
   ```
   Mondoshawan Presale
   ```

2. **Token Symbol**:
   ```
   MSHW-PRESALE
   ```

3. **Description**:
   ```
   Presale token for Mondoshawan Protocol (MSHW).
   Represents future allocation of native MSHW tokens.
   1:1 migration to native MSHW on mainnet launch.
   
   Total Supply: 300,000,000 MSHW-PRESALE
   Presale Price: $0.001 per token
   Target Raise: $300,000
   
   Website: mondoshawan.network
   ```

4. **Image**:
   - Upload logo (512x512px)
   - Must be PNG or JPG
   - Under 1MB

5. **Twitter/X** (Optional):
   ```
   @MondoshawanMSHW
   ```

6. **Telegram** (Optional):
   ```
   https://t.me/mondoshawan
   ```

7. **Website** (Optional):
   ```
   https://mondoshawan.network
   ```

---

#### **Step 4: Set Token Supply**

**Important**: Pump.fun uses a bonding curve model

**How It Works**:
- You set **total supply** (300M tokens)
- Initial price is very low (e.g., $0.00001)
- Price increases as tokens are bought
- When 100% sold OR $85k raised → auto-lists on Raydium

**For Your Presale**:
- **Total Supply**: 300,000,000 (300M)
- **Target Price**: $0.001 per token
- **Target Raise**: $300,000

**Note**: With bonding curve, early buyers get better price, later buyers pay more. This is normal for Pump.fun.

**Alternative Approach**:
- Set supply to 300M
- Let bonding curve work
- Or: Set lower supply, do multiple rounds

---

#### **Step 5: Review & Launch**

**Before Clicking "Create"**:
- ✅ Double-check all details
- ✅ Verify wallet has SOL for fees
- ✅ Image uploaded correctly
- ✅ Description is accurate
- ✅ Links work

**Click "Create Token"**

**What Happens**:
1. Transaction sent to Solana
2. Token created (~$50-100 fee)
3. Token page goes live
4. You can start sharing immediately

**Time**: ~2-5 minutes total

---

### **Phase 3: Marketing & Sales**

#### **Step 1: Initial Announcement**

**Where to Post**:
- Twitter/X: Announce presale launch
- Telegram: Share in crypto groups
- Reddit: r/cryptocurrency, r/solana
- Discord: Solana communities
- Pump.fun itself: Share on platform

**Message Template**:
```
🚀 Mondoshawan Presale Now Live on Pump.fun!

🎯 What: Presale token for Mondoshawan Protocol
💰 Price: $0.001 per MSHW-PRESALE
📊 Supply: 300M tokens
🎁 1:1 migration to native MSHW on mainnet

🔗 Link: [pump.fun link]

#Mondoshawan #MSHW #Solana #Presale
```

---

#### **Step 2: Build Momentum**

**Strategies**:
1. **Early Buyers**: Offer incentives (first 100 buyers get bonus)
2. **Community**: Build Discord/Telegram
3. **Content**: Post updates, progress, features
4. **Influencers**: Reach out to Solana influencers
5. **Partnerships**: Partner with other projects

**Daily Tasks**:
- Post updates on Twitter
- Engage with community
- Answer questions
- Share progress
- Build anticipation

---

#### **Step 3: Track Sales**

**Pump.fun Dashboard**:
- View real-time sales
- See buyer addresses
- Track progress
- Monitor price on bonding curve

**Your Tracking**:
- Set up spreadsheet
- Track daily sales
- Calculate funds raised
- Update community weekly

---

### **Phase 4: After Presale Completes**

#### **Step 1: Collect Funds**

**When Presale Completes**:
- Pump.fun auto-lists on Raydium (if 100% sold or $85k raised)
- You receive SOL from sales
- Funds go to your wallet (or multi-sig)

**What to Do**:
1. Verify funds received
2. Transfer to multi-sig wallet (if not already)
3. Document all transactions
4. Prepare for mainnet launch

---

#### **Step 2: Prepare Migration**

**Before Mainnet Launch**:
- [ ] Migration contract ready
- [ ] Presale holder list compiled
- [ ] Migration process documented
- [ ] Communication plan ready

**Migration Contract**:
- Verify presale token ownership
- Mint native MSHW 1:1
- Burn presale tokens
- Track migrations

---

#### **Step 3: Mainnet Launch**

**When Ready**:
1. Launch Mondoshawan mainnet
2. Open migration contract
3. Announce migration to community
4. Presale holders swap 1:1
5. Presale token retired

---

## 📊 Pump.fun Specifics

### **Bonding Curve Model**

**How It Works**:
- Price starts very low (e.g., $0.00001)
- Each purchase increases price slightly
- Price formula: `price = base_price * (1 + tokens_sold / total_supply)`
- When 100% sold → fixed price on Raydium

**For Your Presale**:
- Early buyers: Better price (e.g., $0.0005)
- Middle buyers: Medium price (e.g., $0.001)
- Late buyers: Higher price (e.g., $0.002)

**Strategy**:
- Accept that early buyers get better deal
- This is normal for Pump.fun
- Focus on getting to 100% sold or $85k raised

---

### **Auto-Listing on Raydium**

**When It Happens**:
- 100% of tokens sold, OR
- $85,000 raised (whichever comes first)

**What Happens**:
- Token automatically lists on Raydium DEX
- Liquidity pool created
- Trading begins
- You get remaining SOL from sales

**Important**: 
- If you hit $85k before 100% sold, remaining tokens are yours
- You can burn them or keep for later

---

### **Fees & Costs**

**Pump.fun Fees**:
- **Creation**: ~$50-100 (one-time)
- **Platform Fee**: 1% of sales (standard)
- **Solana Fees**: ~$0.00025 per transaction (negligible)

**Example**:
- Raise $300,000
- Platform fee: $3,000 (1%)
- You receive: $297,000
- Net cost: ~$100 (creation) + $3,000 (fees) = $3,100 total

**Very affordable!** ✅

---

## 🎯 Alternative Launchpads (If Pump.fun Doesn't Work)

### **1. Moonshot (Similar to Pump.fun)**

**What It Is**:
- Similar to Pump.fun
- Solana token launchpad
- Bonding curve model

**Token Supply**:
- ✅ Full control
- ✅ Custom supply

**Costs**:
- Similar to Pump.fun (~$100)

**Best For**: Alternative if Pump.fun has issues

---

### **2. Solana Launchpad (Jupiter)**

**What It Is**:
- More professional
- Better branding
- More features

**Token Supply**:
- ✅ Full control
- ✅ Custom supply
- ✅ More options (vesting, etc.)

**Costs**:
- Higher ($2k-5k)

**Best For**: Professional projects, institutional investors

---

### **3. Direct SPL Token Creation**

**What It Is**:
- Create token yourself
- Launch on your website
- Full control

**How**:
```bash
# Using Solana CLI
spl-token create-token --decimals 18
# Returns: Token mint address

spl-token create-account <TOKEN_MINT>
# Creates token account

spl-token mint <TOKEN_MINT> 300000000
# Mints 300M tokens
```

**Costs**:
- Very low (~$10-50)
- But need to build everything else

**Best For**: Technical teams, custom requirements

---

## ✅ Recommended Approach

### **Use Pump.fun** ⭐

**Why**:
1. ✅ Easiest to use (no coding)
2. ✅ Full control over supply (300M tokens)
3. ✅ Very low cost (~$100)
4. ✅ Viral potential (huge audience)
5. ✅ Fast launch (minutes)
6. ✅ Auto-listing on Raydium

**Steps**:
1. Create token on Pump.fun (300M supply)
2. Set price/description
3. Launch and market
4. Collect funds when complete
5. Migrate to native MSHW on mainnet

**Timeline**:
- **Week 1**: Prepare (wallet, assets, legal)
- **Week 2**: Launch on Pump.fun
- **Week 3-8**: Marketing and sales (30-60 days)
- **After**: Mainnet launch and migration

---

## 📋 Pre-Launch Checklist

### **Before Creating Token**:
- [ ] Solana wallet created (Phantom)
- [ ] Multi-sig wallet set up
- [ ] Token logo ready (512x512px)
- [ ] Description written
- [ ] Legal review done
- [ ] Terms of sale ready
- [ ] Website/links ready
- [ ] Social media accounts ready
- [ ] Community channels set up (Discord/Telegram)
- [ ] Marketing plan ready
- [ ] SOL in wallet for fees (~1-2 SOL)

### **Ready to Launch?**:
- [ ] All assets prepared
- [ ] Legal cleared
- [ ] Marketing ready
- [ ] Community channels active
- [ ] Go live! 🚀

---

## 🎯 Success Metrics

**Track These**:
- Daily sales volume
- Number of unique buyers
- Average purchase size
- Social media engagement
- Community growth
- Funds raised vs. target

**Goals**:
- **Week 1**: $10k-20k raised
- **Week 2**: $30k-50k raised
- **Week 4**: $100k-150k raised
- **Week 8**: $300k target reached

---

## 💡 Pro Tips

1. **Early Momentum**: Get first 10-20 buyers quickly (friends, community)
2. **Content**: Post daily updates, progress, features
3. **Community**: Build Discord/Telegram, engage daily
4. **Transparency**: Share progress, be honest about timeline
5. **Partnerships**: Partner with other Solana projects
6. **Influencers**: Reach out to Solana influencers (small ones first)

---

**Ready to launch?** Follow the steps above and you'll have your presale token live on Pump.fun in under an hour! 🚀

---

**Last Updated**: January 2026  
**Status**: Internal Guide - Not for Public Repository
