# Testnet Deployment Guide - Privacy Layer

**Mondoshawan Protocol - Deploying Privacy Layer to Testnet**

---

## 🚀 **Quick Start: Deploy Testnet Now**

Your testnet can be deployed **immediately** using the simulated trusted setup. This is **perfectly safe** for testnet (testnet tokens have no real value).

---

## 📋 **Step-by-Step Deployment**

### **Step 1: Generate Testnet Keys**

Create a script to generate and save testnet keys:

```rust
// scripts/generate_testnet_keys.rs
use mondoshawan_blockchain::privacy::generate_keys;
use ark_std::test_rng;
use std::fs;

fn main() {
    let mut rng = test_rng();
    
    println!("Generating testnet keys...");
    let (pk, vk) = generate_keys(&mut rng)
        .expect("Failed to generate keys");
    
    // Serialize keys
    let (pk_bytes, vk_bytes) = serialize_keys(&pk, &vk)
        .expect("Failed to serialize keys");
    
    // Save to files
    fs::write("testnet_proving_key.bin", &pk_bytes)
        .expect("Failed to write proving key");
    fs::write("testnet_verifying_key.bin", &vk_bytes)
        .expect("Failed to write verifying key");
    
    println!("✅ Testnet keys generated!");
    println!("⚠️  WARNING: These keys are for TESTNET ONLY");
    println!("⚠️  DO NOT use for mainnet!");
}
```

### **Step 2: Initialize Privacy Manager in Node**

```rust
// src/node/mod.rs or wherever you initialize the blockchain

use crate::privacy::{PrivacyManager, PrivacyVerifier, load_keys_from_bytes};
use std::fs;

// Load testnet keys
let pk_bytes = fs::read("testnet_proving_key.bin")?;
let vk_bytes = fs::read("testnet_verifying_key.bin")?;
let (_pk, vk) = load_keys_from_bytes(&pk_bytes, &vk_bytes)?;

// Create verifier
let verifier = PrivacyVerifier::new(vk);

// Create privacy manager
let privacy_manager = Arc::new(tokio::sync::RwLock::new(
    PrivacyManager::with_verifier(true, verifier)
));

// Add to blockchain
blockchain.set_privacy_manager(privacy_manager.clone());

// Add to RPC server
rpc_server.with_privacy_manager(privacy_manager);
```

### **Step 3: Add to Blockchain Initialization**

```rust
// src/blockchain/mod.rs

impl Blockchain {
    pub fn set_privacy_manager(
        &mut self,
        manager: Arc<tokio::sync::RwLock<PrivacyManager>>,
    ) {
        self.privacy_manager = Some(manager);
    }
}
```

### **Step 4: Deploy Testnet**

1. **Generate keys** (one-time)
2. **Start node** with privacy enabled
3. **Test privacy features**
4. **Document testnet keys** (mark as "TESTNET ONLY")

---

## ⚠️ **Important Warnings**

### **Testnet Keys**

```
⚠️  WARNING: TESTNET KEYS - UNSAFE FOR PRODUCTION
⚠️  These keys were generated with a simulated trusted setup
⚠️  The person who generated these keys can create fake proofs
⚠️  DO NOT use these keys for mainnet
⚠️  Mainnet requires a proper trusted setup ceremony
```

### **What This Means**

- ✅ **Safe for testnet**: Testnet tokens have no value
- ✅ **Full functionality**: All privacy features work
- ✅ **Can test everything**: Proofs, verification, transactions
- ⚠️ **Not for mainnet**: Must use trusted setup ceremony

---

## 🧪 **Testing Checklist**

Once testnet is deployed, test:

- [ ] Key generation works
- [ ] Proof generation works
- [ ] Proof verification works
- [ ] Privacy transactions process correctly
- [ ] Double-spend prevention works
- [ ] RPC methods work
- [ ] Nullifier tracking works
- [ ] Performance is acceptable

---

## 📝 **Testnet Configuration**

### **Recommended Settings**

```rust
// Privacy configuration for testnet
let privacy_config = PrivacyConfig {
    enabled: true,
    curve: "BN254".to_string(),
    merkle_depth: 20, // 2^20 = 1M notes
    max_nullifiers: 1_000_000,
};
```

### **RPC Endpoints**

Test these endpoints:
- `mds_createPrivateTransaction`
- `mds_verifyPrivacyProof`
- `mds_proveBalance`
- `mds_getPrivacyStats`

---

## 🔄 **Migration to Mainnet**

When ready for mainnet:

1. **Organize trusted setup ceremony**
2. **Generate mainnet keys** (via ceremony)
3. **Deploy mainnet keys**
4. **Update documentation**
5. **Launch mainnet**

---

## ✅ **Summary**

**For Testnet:**
- ✅ Use simulated setup (current implementation)
- ✅ Generate keys once
- ✅ Deploy immediately
- ✅ Test all features
- ✅ Mark as "TESTNET ONLY"

**Your testnet is ready to deploy!** 🚀

---

**Last Updated**: January 2026
