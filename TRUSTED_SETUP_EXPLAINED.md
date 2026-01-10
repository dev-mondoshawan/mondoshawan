# Trusted Setup Ceremony - Explained

**Mondoshawan Protocol - zk-SNARKs Privacy Layer**

---

## 🔐 **What is a Trusted Setup Ceremony?**

A **trusted setup ceremony** is a cryptographic process where multiple parties collaborate to generate the **proving key** and **verifying key** for zk-SNARK circuits. The goal is to ensure that no single party knows the "toxic waste" (secret randomness) used to generate the keys.

---

## 🎯 **Why is it Needed?**

### **The Problem**

When generating zk-SNARK keys, a secret random value (called "toxic waste") is used. If anyone knows this secret, they can create **fake proofs** that appear valid but prove false statements.

### **The Solution**

A **multi-party trusted setup** ensures:
- Multiple parties participate
- Each party adds their own randomness
- The final secret is the combination of all parties' secrets
- As long as **one party** destroys their secret, the system is secure
- No single party can compromise the system

---

## 🔄 **How it Works**

### **Step 1: Circuit-Specific Setup**

1. **Define the Circuit**
   - Our `PrivateTransferCircuit` defines the constraints
   - The circuit structure determines the key size

2. **Generate Randomness**
   - Each participant generates random values
   - These are combined to create the final secret

3. **Create Keys**
   - **Proving Key (PK)**: Used to generate proofs (must be kept secret)
   - **Verifying Key (VK)**: Used to verify proofs (can be public)

### **Step 2: Multi-Party Protocol**

```
Participant 1: Generates random r1 → Creates PK1, VK1
Participant 2: Takes PK1, VK1 → Adds r2 → Creates PK2, VK2
Participant 3: Takes PK2, VK2 → Adds r3 → Creates PK3, VK3
...
Final: PK_final, VK_final (no one knows the combined secret)
```

### **Step 3: Verification**

- Each participant verifies the previous participant's work
- Ensures no one cheated
- Final keys are published

### **Step 4: Destruction**

- Each participant **destroys** their random value
- As long as one participant destroyed their secret, system is secure

---

## 🧪 **Testnet vs Mainnet**

### **For Testnet (Current Situation)** ✅

**You can use a SIMULATED/UNSAFE setup:**

```rust
// This is what we're currently using in keys.rs
let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng)?;
```

**Why this is OK for testnet:**
- ✅ Testnet tokens have no real value
- ✅ Allows immediate testing
- ✅ No security risk (it's just for testing)
- ✅ Can deploy testnet right now

**Limitations:**
- ⚠️ The person who generated the keys can create fake proofs
- ⚠️ Not suitable for mainnet
- ⚠️ Should be clearly marked as "UNSAFE FOR PRODUCTION"

### **For Mainnet (Future)** 🔒

**You MUST use a proper trusted setup ceremony:**

1. **Organize Ceremony**
   - Recruit 5-10 trusted participants
   - Can include community members, developers, auditors
   - More participants = more security

2. **Conduct Ceremony**
   - Each participant generates keys in sequence
   - Each verifies previous participant's work
   - Final keys are published

3. **Destroy Secrets**
   - Each participant destroys their random values
   - Publish proof of destruction (if possible)

4. **Deploy Keys**
   - Deploy verifying key to blockchain
   - Store proving key securely (or use distributed storage)

---

## 🚀 **For Your Testnet**

### **Option 1: Use Simulated Setup (Recommended for Now)** ✅

**Current Implementation:**
```rust
// mondoshawan-blockchain/src/privacy/keys.rs
pub fn generate_keys<R: RngCore>(
    rng: &mut R,
) -> Result<(ProvingKey<Bn254>, VerifyingKey<Bn254>), String> {
    let circuit = PrivateTransferCircuit {
        old_balance: None,
        amount: None,
        new_balance: None,
        nullifier: Fr::zero(),
        commitment: None,
    };

    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit, rng)
        .map_err(|e| format!("Key generation failed: {:?}", e))?;

    Ok((pk, vk))
}
```

**This is PERFECT for testnet!** ✅

**Advantages:**
- ✅ Can deploy testnet immediately
- ✅ No ceremony needed
- ✅ Full functionality for testing
- ✅ Can test all privacy features

**What to do:**
1. Generate keys once for testnet
2. Store them securely (but mark as "TESTNET ONLY")
3. Deploy testnet with these keys
4. Test all privacy features
5. Plan trusted setup ceremony for mainnet

### **Option 2: Simple Multi-Party Setup (Optional)**

If you want to practice the ceremony process:

1. **Generate keys yourself** (Participant 1)
2. **Have a friend generate new keys** (Participant 2)
3. **Combine them** (if using a ceremony protocol)
4. **Use for testnet**

**Note**: For testnet, Option 1 is perfectly fine!

---

## 📋 **Testnet Deployment Checklist**

### **1. Generate Testnet Keys** ✅

```rust
use crate::privacy::generate_keys;
use ark_std::test_rng;

let mut rng = test_rng();
let (proving_key, verifying_key) = generate_keys(&mut rng)?;

// Save these keys for testnet
// Mark clearly as "TESTNET ONLY - UNSAFE FOR PRODUCTION"
```

### **2. Initialize Privacy Manager**

```rust
use crate::privacy::{PrivacyManager, PrivacyVerifier};

let verifier = PrivacyVerifier::new(verifying_key);
let privacy_manager = PrivacyManager::with_verifier(true, verifier);

// Add to blockchain
blockchain.set_privacy_manager(privacy_manager);
```

### **3. Deploy Testnet**

- ✅ Use testnet keys
- ✅ Mark as "TESTNET - UNSAFE SETUP"
- ✅ Test all privacy features
- ✅ Gather feedback

### **4. Plan Mainnet Ceremony**

- ⏳ Organize trusted setup ceremony
- ⏳ Recruit participants
- ⏳ Conduct ceremony
- ⏳ Deploy mainnet keys

---

## 🔒 **Mainnet Trusted Setup Ceremony Plan**

### **Phase 1: Preparation**

1. **Recruit Participants** (5-10 people)
   - Community members
   - Developers
   - Auditors
   - Public figures (for transparency)

2. **Prepare Infrastructure**
   - Secure key generation environment
   - Verification tools
   - Communication channels

3. **Document Process**
   - Step-by-step ceremony protocol
   - Verification procedures
   - Destruction procedures

### **Phase 2: Ceremony Execution**

1. **Participant 1**: Generate initial keys
2. **Participant 2**: Verify and add randomness
3. **Participant 3**: Verify and add randomness
4. ... (continue for all participants)
5. **Final**: Publish final keys

### **Phase 3: Verification**

1. **Public Verification**
   - Publish all intermediate keys
   - Allow public verification
   - Publish verification results

2. **Destruction**
   - Each participant destroys their secret
   - Publish proof of destruction (if possible)

### **Phase 4: Deployment**

1. **Deploy Verifying Key**
   - Add to blockchain contract
   - Make publicly available
   - Document in whitepaper

2. **Secure Proving Key**
   - Store securely (or use distributed storage)
   - Or: Use MPC (Multi-Party Computation) for proving

---

## 💡 **Recommendations**

### **For Testnet (Now)** ✅

1. **Use simulated setup** (current implementation)
2. **Deploy testnet immediately**
3. **Test all privacy features**
4. **Gather community feedback**
5. **Mark clearly as "TESTNET - UNSAFE SETUP"**

### **For Mainnet (Future)** 🔒

1. **Organize trusted setup ceremony** (6-12 months before mainnet)
2. **Recruit 5-10 participants**
3. **Conduct ceremony publicly**
4. **Publish verification results**
5. **Deploy mainnet with secure keys**

---

## 📚 **References**

- [Zcash Ceremony](https://z.cash/technology/paramgen/) - Example of trusted setup
- [Tornado Cash Ceremony](https://github.com/tornadocash/tornado-core) - Another example
- [Perpetual Powers of Tau](https://github.com/privacy-scaling-explorations/perpetual-powers-of-tau) - Ongoing ceremony

---

## ✅ **Summary**

**For Testnet:**
- ✅ Use simulated setup (current implementation)
- ✅ Deploy immediately
- ✅ Test all features
- ✅ Mark as "TESTNET ONLY"

**For Mainnet:**
- ⏳ Organize trusted setup ceremony
- ⏳ Recruit participants
- ⏳ Conduct ceremony
- ⏳ Deploy secure keys

**Your testnet can launch NOW with the current implementation!** 🚀

---

**Last Updated**: January 2026
