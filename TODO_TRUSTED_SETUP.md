# TODO: Trusted Setup Ceremony (Mainnet)

**Status**: ⏳ **Pending - For Mainnet Deployment**  
**Priority**: Medium  
**Timeline**: Before mainnet launch

---

## 📋 **Task**

Organize and conduct a **multi-party trusted setup ceremony** to generate secure zk-SNARK keys for mainnet deployment.

---

## 🎯 **Why This is Needed**

- Current testnet uses **simulated setup** (fine for testnet)
- Mainnet requires **secure trusted setup** to prevent fake proofs
- Multi-party ceremony ensures no single party can compromise the system

---

## ⏰ **When to Do This**

- ✅ **NOT needed for testnet** - Current implementation is fine
- ⏳ **Needed before mainnet** - Plan 6-12 months before mainnet launch
- 📅 **Timeline**: After testnet is stable and before mainnet planning

---

## 📝 **What Needs to Be Done**

### **Phase 1: Planning** (2-3 months before ceremony)
- [ ] Recruit 5-10 trusted participants
- [ ] Prepare ceremony infrastructure
- [ ] Document ceremony protocol
- [ ] Set up verification tools

### **Phase 2: Ceremony Execution** (1-2 weeks)
- [ ] Conduct multi-party ceremony
- [ ] Verify each participant's contribution
- [ ] Generate final keys
- [ ] Publish verification results

### **Phase 3: Key Deployment** (Before mainnet)
- [ ] Deploy verifying key to blockchain
- [ ] Secure proving key storage
- [ ] Update documentation
- [ ] Announce to community

---

## 🔗 **References**

- See `TRUSTED_SETUP_EXPLAINED.md` for detailed explanation
- See `TESTNET_DEPLOYMENT_GUIDE.md` for testnet deployment (uses simulated setup)

---

## ✅ **Current Status**

- ✅ Testnet can deploy with simulated setup (current implementation)
- ⏳ Mainnet trusted setup ceremony - **TODO for later**

---

**Created**: January 2026  
**Status**: ⏳ Pending - Will handle before mainnet launch
