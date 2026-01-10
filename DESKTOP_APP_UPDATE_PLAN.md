# Desktop App Update Plan

**Date**: January 2026  
**Status**: Planning

---

## 🎯 Overview

The desktop app needs updates to support:
1. **Account Abstraction** (Smart Contract Wallets)
2. **Parallel EVM** (Performance Controls)
3. **Quick Wins** (Time-Locked, Gasless, Reputation)

---

## 📋 Current Features

### **Tabs**:
- ✅ Dashboard (node status, mining)
- ✅ Wallet (view balance/nonce)
- ✅ Send (create wallet, send transactions)
- ✅ History (transaction history)
- ✅ Explorer (blocks, DAG stats)
- ✅ Metrics (TPS, shard stats)

### **Additional Features**:
- ✅ Address Book
- ✅ Multi-Account Management

---

## 🆕 New Features to Add

### **1. Account Abstraction Tab** (NEW)

#### **Smart Contract Wallet Creation**:
- Create basic wallet
- Create multi-sig wallet (n-of-m)
- Create social recovery wallet
- Create spending limit wallet
- Create combined wallet (multi-sig + social recovery + spending limits)

#### **Wallet Management**:
- View all owned wallets
- View wallet details (type, config, nonce)
- Check if address is a contract wallet

#### **Multi-Sig Operations**:
- Create multi-sig transaction
- Add signatures to pending transaction
- View pending multi-sig transactions
- Validate multi-sig transaction

#### **Social Recovery**:
- Initiate recovery request
- Approve recovery (as guardian)
- View recovery status
- Complete recovery
- Cancel recovery

#### **Batch Transactions**:
- Create batch transaction
- Add operations to batch
- Execute batch transaction
- View batch status
- Estimate batch gas

---

### **2. Parallel EVM Section** (Add to Metrics Tab)

#### **Controls**:
- Enable/disable parallel EVM toggle
- Set max parallel transactions
- View current status

#### **Statistics**:
- Parallel execution rate
- Average speedup
- Batch size distribution
- Conflict detection rate
- Performance improvement estimate

---

### **3. Quick Wins Integration**

#### **Time-Locked Transactions** (Add to Send Tab):
- Option to set `execute_at_block`
- Option to set `execute_at_timestamp`
- View pending time-locked transactions

#### **Gasless Transactions** (Add to Send Tab):
- Option to set sponsor address
- View sponsored transactions
- View transactions you're sponsoring

#### **Reputation Display** (Add to Wallet Tab):
- Show reputation score (0-100)
- Show reputation level (High/Medium/Low)
- Show reputation factors:
  - Successful transactions
  - Failed transactions
  - Blocks mined
  - Node longevity
  - Account age
  - Total value transacted
  - Unique contacts
  - Suspicious activities

---

## 🔧 Implementation Details

### **New Tab: "Account Abstraction"**

```typescript
// State
const [wallets, setWallets] = useState<any[]>([]);
const [selectedWallet, setSelectedWallet] = useState<string | null>(null);
const [walletType, setWalletType] = useState<"basic" | "multisig" | "social" | "spending" | "combined">("basic");
const [multisigSigners, setMultisigSigners] = useState<string[]>([]);
const [multisigThreshold, setMultisigThreshold] = useState<number>(2);
const [guardians, setGuardians] = useState<string[]>([]);
const [recoveryThreshold, setRecoveryThreshold] = useState<number>(2);
const [spendingLimit, setSpendingLimit] = useState<string>("");
const [pendingMultisigTxs, setPendingMultisigTxs] = useState<any[]>([]);
const [recoveryStatus, setRecoveryStatus] = useState<any | null>(null);
const [batchOperations, setBatchOperations] = useState<any[]>([]);
```

### **RPC Methods to Add**:

#### **Account Abstraction**:
- `mds_createWallet` - Create smart contract wallet
- `mds_getWallet` - Get wallet info
- `mds_getOwnerWallets` - Get all wallets for owner
- `mds_isContractWallet` - Check if address is contract wallet
- `mds_createMultisigTransaction` - Create multi-sig tx
- `mds_addMultisigSignature` - Add signature
- `mds_getPendingMultisigTransactions` - Get pending multi-sig txs
- `mds_validateMultisigTransaction` - Validate multi-sig tx
- `mds_initiateRecovery` - Start recovery
- `mds_approveRecovery` - Approve recovery
- `mds_getRecoveryStatus` - Get recovery status
- `mds_completeRecovery` - Complete recovery
- `mds_cancelRecovery` - Cancel recovery
- `mds_createBatchTransaction` - Create batch
- `mds_executeBatchTransaction` - Execute batch
- `mds_getBatchStatus` - Get batch status
- `mds_estimateBatchGas` - Estimate batch gas

#### **Parallel EVM**:
- `mds_enableParallelEVM` - Enable/disable
- `mds_getParallelEVMStats` - Get stats
- `mds_estimateParallelImprovement` - Estimate speedup

#### **Quick Wins**:
- `mds_createTimeLockedTransaction` - Create time-locked tx
- `mds_getTimeLockedTransactions` - Get pending time-locked txs
- `mds_createGaslessTransaction` - Create gasless tx
- `mds_getSponsoredTransactions` - Get sponsored txs
- `mds_getReputation` - Get reputation score
- `mds_getReputationFactors` - Get reputation factors

---

## 📝 UI Components Needed

### **1. Account Abstraction Tab**:

#### **Wallet Creation Form**:
```
┌─────────────────────────────────────┐
│ Create Smart Contract Wallet        │
├─────────────────────────────────────┤
│ Wallet Type: [Dropdown]             │
│   - Basic                           │
│   - Multi-Sig                       │
│   - Social Recovery                 │
│   - Spending Limit                  │
│   - Combined                        │
│                                     │
│ [Configuration based on type]        │
│                                     │
│ [Create Wallet Button]             │
└─────────────────────────────────────┘
```

#### **Wallet List**:
```
┌─────────────────────────────────────┐
│ My Smart Contract Wallets           │
├─────────────────────────────────────┤
│ Wallet 1: 0x1234...                 │
│   Type: Multi-Sig (2-of-3)          │
│   Balance: 1,000 MSHW               │
│   [View Details] [Manage]            │
│                                     │
│ Wallet 2: 0x5678...                 │
│   Type: Social Recovery              │
│   Balance: 500 MSHW                 │
│   [View Details] [Manage]            │
└─────────────────────────────────────┘
```

#### **Multi-Sig Transaction Form**:
```
┌─────────────────────────────────────┐
│ Create Multi-Sig Transaction       │
├─────────────────────────────────────┤
│ Wallet: [Select Wallet]             │
│ To: [Address]                      │
│ Value: [Amount MSHW]                │
│ Data: [Optional]                    │
│                                     │
│ [Create Transaction]                │
│                                     │
│ Pending Signatures: 1/3            │
│ [Add Signature]                    │
└─────────────────────────────────────┘
```

#### **Social Recovery Form**:
```
┌─────────────────────────────────────┐
│ Social Recovery                     │
├─────────────────────────────────────┤
│ Wallet: [Select Wallet]             │
│ New Owner: [Address]                 │
│                                     │
│ [Initiate Recovery]                 │
│                                     │
│ Status: Pending                     │
│ Approvals: 1/3                      │
│ Time Remaining: 2 days              │
│ [Approve] [Complete]                │
└─────────────────────────────────────┘
```

#### **Batch Transaction Form**:
```
┌─────────────────────────────────────┐
│ Batch Transaction                   │
├─────────────────────────────────────┤
│ Operation 1: Transfer 100 MSHW      │
│ Operation 2: Approve Token         │
│ Operation 3: Swap Token             │
│                                     │
│ [Add Operation]                    │
│ [Execute Batch]                     │
│                                     │
│ Estimated Gas: 63,000              │
│ Savings: 21,000 (vs sequential)   │
└─────────────────────────────────────┘
```

---

### **2. Parallel EVM Section** (Metrics Tab):

```
┌─────────────────────────────────────┐
│ Parallel EVM                        │
├─────────────────────────────────────┤
│ Status: [Enabled/Disabled Toggle]   │
│ Max Parallel: [100]                 │
│                                     │
│ Statistics:                         │
│   Parallel Execution Rate: 75%     │
│   Average Speedup: 12.5x            │
│   Avg Batch Size: 8.3               │
│   Conflict Rate: 15%                │
│                                     │
│ [Estimate Improvement]                │
└─────────────────────────────────────┘
```

---

### **3. Quick Wins Integration**:

#### **Send Tab - Time-Locked**:
```
┌─────────────────────────────────────┐
│ Send Transaction                     │
├─────────────────────────────────────┤
│ To: [Address]                       │
│ Value: [Amount]                     │
│ Fee: [Amount]                        │
│                                     │
│ [ ] Time-Locked Transaction         │
│   Execute at Block: [Number]        │
│   OR                                │
│   Execute at Timestamp: [Date/Time] │
│                                     │
│ [ ] Gasless Transaction             │
│   Sponsor: [Address]                │
│                                     │
│ [Send Transaction]                  │
└─────────────────────────────────────┘
```

#### **Wallet Tab - Reputation**:
```
┌─────────────────────────────────────┐
│ Address: 0x1234...                  │
│ Balance: 1,000 MSHW                 │
│ Nonce: 42                           │
│                                     │
│ Reputation: 85/100 (High)           │
│                                     │
│ Factors:                            │
│   ✅ Successful Txs: 150            │
│   ❌ Failed Txs: 2                 │
│   ⛏️ Blocks Mined: 10              │
│   📅 Account Age: 365 days          │
│   💰 Value Transacted: 50,000 MSHW │
│   👥 Unique Contacts: 25            │
│   ⚠️ Suspicious: 0                 │
└─────────────────────────────────────┘
```

---

## 🔨 Backend Updates Needed

### **Tauri Commands** (src-tauri/src/lib.rs):

Add new commands for:
- Account Abstraction operations
- Parallel EVM controls
- Quick Wins features

Example:
```rust
#[tauri::command]
async fn create_wallet(
    wallet_type: String,
    owner: String,
    config: serde_json::Value,
) -> Result<String, String> {
    // Call mds_createWallet RPC
}

#[tauri::command]
async fn enable_parallel_evm(enabled: bool) -> Result<bool, String> {
    // Call mds_enableParallelEVM RPC
}

#[tauri::command]
async fn get_reputation(address: String) -> Result<serde_json::Value, String> {
    // Call mds_getReputation RPC
}
```

---

## 📊 Priority Order

### **High Priority**:
1. ✅ Account Abstraction Tab (core functionality)
2. ✅ Parallel EVM controls (Metrics tab)
3. ✅ Reputation display (Wallet tab)

### **Medium Priority**:
4. ⚠️ Time-locked transactions (Send tab)
5. ⚠️ Gasless transactions (Send tab)

### **Low Priority**:
6. ⚠️ Advanced multi-sig UI
7. ⚠️ Batch transaction builder

---

## ✅ Implementation Checklist

### **Account Abstraction**:
- [ ] Add "Account Abstraction" tab
- [ ] Wallet creation form
- [ ] Wallet list view
- [ ] Multi-sig transaction UI
- [ ] Social recovery UI
- [ ] Batch transaction UI
- [ ] Tauri commands for all RPC methods

### **Parallel EVM**:
- [ ] Add section to Metrics tab
- [ ] Enable/disable toggle
- [ ] Statistics display
- [ ] Tauri commands

### **Quick Wins**:
- [ ] Time-locked transaction options (Send tab)
- [ ] Gasless transaction options (Send tab)
- [ ] Reputation display (Wallet tab)
- [ ] Tauri commands

---

## 🎯 Next Steps

1. **Create Account Abstraction Tab** - Full UI and functionality
2. **Add Parallel EVM Section** - Controls and stats
3. **Integrate Quick Wins** - Time-locked, gasless, reputation
4. **Add Tauri Commands** - Backend RPC integration
5. **Test All Features** - End-to-end testing
6. **Update README** - Document new features

---

**Last Updated**: January 2026  
**Status**: Ready for implementation
