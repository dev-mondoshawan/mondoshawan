# Mondoshawan Community Presale Structure

**Status**: Draft for Review  
**Last Updated**: January 2026

---

## 🎯 Presale Overview

**Name**: Mondoshawan Community Presale  
**Tagline**: "The Fairest Presale in Crypto"  
**Goal**: Raise $300,000 for development, audits, and exchange listings

---

## 📊 Presale Parameters

### Allocation
- **Total Supply**: 300,000,000 MSHW (3% of 10B max supply)
- **Price**: $0.001 per MSHW (fixed price)
- **Target Raise**: $300,000 USD
- **Currency**: USDC, USDT, ETH, BTC accepted

### Limits
- **Minimum Purchase**: 1,000 MSHW ($1.00)
- **Maximum Purchase**: 10,000,000 MSHW ($10,000) per address
- **Maximum Per Address**: 1% of presale (10M MSHW)
- **Whale Protection**: Hard cap prevents single address dominance

### Timeline
- **Announcement**: TBD
- **Registration Opens**: TBD
- **Presale Starts**: TBD
- **Presale Duration**: 30-60 days (or until sold out)
- **Token Distribution**: Within 7 days after presale ends
- **Mainnet Launch**: TBD (after security audit)

---

## 🔒 Smart Contract Structure

### Contract Features

```solidity
// Simplified structure - full contract to be audited
contract MondoshawanPresale {
    // Presale parameters
    uint256 public constant TOTAL_PRESALE = 300_000_000 * 1e18; // 300M MSHW
    uint256 public constant PRICE = 0.001 * 1e18; // $0.001 per MSHW
    uint256 public constant MIN_PURCHASE = 1_000 * 1e18; // 1,000 MSHW
    uint256 public constant MAX_PURCHASE = 10_000_000 * 1e18; // 10M MSHW
    
    // State
    uint256 public totalRaised;
    uint256 public totalSold;
    bool public presaleActive;
    bool public presaleEnded;
    
    // Whitelist (optional, for KYC)
    mapping(address => bool) public whitelist;
    bool public whitelistOnly;
    
    // Purchase tracking
    mapping(address => uint256) public purchases;
    address[] public contributors;
    
    // Multi-sig wallet for funds
    address public treasuryWallet;
    
    // Events
    event Purchase(address indexed buyer, uint256 amount, uint256 cost);
    event PresaleEnded(uint256 totalRaised, uint256 totalSold);
    
    // Functions
    function purchase(uint256 mshwAmount) external payable;
    function withdrawFunds() external; // Multi-sig only
    function endPresale() external; // Admin only
}
```

### Security Features
- ✅ Multi-sig wallet for fund collection
- ✅ Hard limits (min/max per address)
- ✅ Whitelist support (optional KYC)
- ✅ Time-locked withdrawals
- ✅ Emergency pause function
- ✅ Audit before deployment

---

## 💰 Fund Allocation

### Use of Funds ($300,000)

**Security & Audits (33%)**: $100,000
- Smart contract audit: $50,000
- Blockchain security audit: $30,000
- Penetration testing: $20,000

**Exchange Listings (25%)**: $75,000
- Binance listing fee: $30,000
- Coinbase listing fee: $25,000
- Other exchanges: $20,000

**Marketing & Community (17%)**: $50,000
- Community building: $20,000
- Content creation: $15,000
- Social media: $10,000
- Influencer partnerships: $5,000

**Infrastructure (17%)**: $50,000
- Servers & hosting: $20,000
- Monitoring tools: $10,000
- Development tools: $10,000
- Backup systems: $10,000

**Legal & Compliance (8%)**: $25,000
- Legal structure: $15,000
- Regulatory compliance: $10,000

---

## 🔐 Transparency & Governance

### Public Transparency
- ✅ **Public Wallet**: All funds go to publicly visible multi-sig wallet
- ✅ **Real-time Tracking**: Live dashboard showing raised amount
- ✅ **Spending Reports**: Monthly reports on fund usage
- ✅ **Audit Trail**: All transactions on-chain and verifiable

### Governance
- **Multi-sig Wallet**: 3-of-5 signatures required
  - Core team: 2 signatures
  - Community representatives: 2 signatures
  - Technical advisor: 1 signature
- **Spending Approval**: All spending requires multi-sig approval
- **Public Proposals**: Community can propose fund usage
- **Voting**: MSHW holders vote on major spending decisions

---

## 📋 Presale Process

### Step 1: Registration (Optional)
- Visit presale website
- Complete KYC (if required by jurisdiction)
- Get whitelisted address
- Receive confirmation email

### Step 2: Purchase
- Connect wallet (MetaMask, WalletConnect, etc.)
- Enter amount (1,000 - 10,000,000 MSHW)
- Confirm transaction
- Receive confirmation

### Step 3: Tracking
- View purchase on dashboard
- Track total raised
- See your contribution percentage
- Monitor presale progress

### Step 4: Distribution
- Presale ends
- Tokens distributed within 7 days
- Claim tokens via website
- Tokens locked until mainnet (optional)

---

## 🎯 Token Distribution

### Distribution Schedule

**Option A: Immediate Distribution**
- Tokens distributed within 7 days
- Available immediately after mainnet launch
- No vesting period

**Option B: Locked Until Mainnet**
- Tokens distributed but locked
- Unlock on mainnet launch
- Prevents pre-launch trading

**Option C: Gradual Unlock**
- 25% unlocked at mainnet
- 25% after 3 months
- 25% after 6 months
- 25% after 12 months

**Recommendation**: Option B (locked until mainnet) - maintains fairness

---

## 📊 Presale Dashboard Features

### Public Dashboard
- Total raised (USD)
- Total sold (MSHW)
- Number of contributors
- Average purchase size
- Time remaining
- Progress bar
- Top contributors (anonymized)

### Personal Dashboard
- Your purchase amount
- Your MSHW allocation
- Your contribution percentage
- Purchase history
- Token claim status

---

## 🔒 Security Measures

### Smart Contract Security
- ✅ Professional audit before deployment
- ✅ Bug bounty program
- ✅ Multi-sig wallet
- ✅ Time locks on critical functions
- ✅ Emergency pause mechanism

### Fund Security
- ✅ Multi-sig wallet (3-of-5)
- ✅ Cold storage for majority of funds
- ✅ Insurance (if available)
- ✅ Regular security audits

### User Security
- ✅ KYC/AML compliance (if required)
- ✅ Wallet verification
- ✅ Anti-bot measures
- ✅ Rate limiting
- ✅ Maximum purchase limits

---

## 📝 Legal Considerations

### Compliance
- **KYC/AML**: Required in some jurisdictions
- **Securities Law**: Consultation with crypto lawyer
- **Tax**: Clear tax implications for contributors
- **Terms of Sale**: Clear terms and conditions
- **Disclaimers**: Risk warnings, no guarantees

### Legal Structure
- **Entity**: Foundation, DAO, or LLC
- **Jurisdiction**: Crypto-friendly jurisdiction
- **Legal Counsel**: Specialized crypto lawyer
- **Regulatory Review**: Compliance check before launch

---

## 🚀 Marketing Strategy

### Pre-Presale
- Announcement blog post
- Social media campaign
- Community building
- Influencer partnerships
- Press releases

### During Presale
- Real-time updates
- Community engagement
- Progress milestones
- Referral program (optional)
- AMA sessions

### Post-Presale
- Thank you message
- Fund usage updates
- Development progress
- Audit results
- Mainnet countdown

---

## 📈 Success Metrics

### Presale Goals
- **Soft Cap**: $150,000 (50% sold)
- **Hard Cap**: $300,000 (100% sold)
- **Minimum Contributors**: 100+ addresses
- **Average Purchase**: $1,000-3,000

### Success Criteria
- ✅ Reach soft cap ($150k)
- ✅ 100+ unique contributors
- ✅ No single address > 5% of total
- ✅ Positive community feedback
- ✅ Transparent fund usage

---

## ⚠️ Risk Disclosures

### For Contributors
- **No Guarantees**: Presale does not guarantee mainnet launch
- **Regulatory Risk**: Regulations may change
- **Technical Risk**: Smart contract bugs possible
- **Market Risk**: Token value may decrease
- **Liquidity Risk**: Tokens may not be immediately tradeable

### Mitigation
- Professional audit
- Insurance (if available)
- Legal compliance
- Transparent communication
- Emergency procedures

---

## ✅ Implementation Checklist

### Pre-Launch
- [ ] Legal structure established
- [ ] Smart contract developed
- [ ] Smart contract audited
- [ ] Multi-sig wallet set up
- [ ] Website developed
- [ ] KYC system (if needed)
- [ ] Marketing materials
- [ ] Community channels

### Launch
- [ ] Presale announcement
- [ ] Website goes live
- [ ] Registration opens
- [ ] Presale starts
- [ ] Real-time tracking active
- [ ] Support channels ready

### Post-Launch
- [ ] Monitor presale progress
- [ ] Regular updates
- [ ] Community engagement
- [ ] Fund collection
- [ ] Token distribution
- [ ] Spending reports

---

## 📝 Terms & Conditions (Draft)

### Eligibility
- Must be 18+ years old
- Must pass KYC (if required)
- Must comply with local laws
- Must not be in restricted jurisdictions

### Purchase Terms
- Fixed price: $0.001 per MSHW
- Minimum: 1,000 MSHW
- Maximum: 10,000,000 MSHW per address
- No refunds after purchase
- Tokens locked until mainnet

### Fund Usage
- Funds used for development only
- Transparent reporting
- Community oversight
- Multi-sig approval required

### Disclaimers
- No investment advice
- High risk investment
- No guarantees
- Regulatory uncertainty
- Technical risks

---

**Status**: Ready for legal review and implementation! 💎
