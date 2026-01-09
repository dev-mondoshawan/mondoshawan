# Account Abstraction User Guide

**Mondoshawan Protocol** - Smart Contract Wallets

---

## 📖 Overview

Account Abstraction enables smart contract wallets as first-class accounts on Mondoshawan. This allows for programmable wallets with features like multi-signature, social recovery, spending limits, and batch transactions.

---

## 🔐 Wallet Types

### **1. Basic Wallet**
Simple programmable wallet with custom authentication logic.

**Use Cases**:
- Standard wallet with programmability
- Custom validation rules
- Future extensibility

**Creation**:
```json
{
  "jsonrpc": "2.0",
  "method": "mds_createWallet",
  "params": {
    "owner": "0x...",
    "walletType": "basic"
  },
  "id": 1
}
```

---

### **2. Multi-Signature Wallet**
Requires multiple signatures (n-of-m) to execute transactions.

**Use Cases**:
- Enterprise wallets
- Shared funds
- High-security accounts
- DAO treasuries

**Creation**:
```json
{
  "jsonrpc": "2.0",
  "method": "mds_createWallet",
  "params": {
    "owner": "0x...",
    "walletType": "multisig",
    "config": {
      "signers": ["0x...", "0x...", "0x..."],
      "threshold": 2
    }
  },
  "id": 1
}
```

**Example**: 3 signers, 2 required = 2-of-3 multi-sig

---

### **3. Social Recovery Wallet**
Recover wallet access via trusted guardians.

**Use Cases**:
- User-friendly recovery
- No seed phrase management
- Family/trusted contact recovery

**Creation**:
```json
{
  "jsonrpc": "2.0",
  "method": "mds_createWallet",
  "params": {
    "owner": "0x...",
    "walletType": "socialRecovery",
    "config": {
      "guardians": ["0x...", "0x...", "0x..."],
      "recoveryThreshold": 2
    }
  },
  "id": 1
}
```

**Recovery Process**:
1. Initiate recovery: `mds_initiateRecovery`
2. Guardians approve: `mds_approveRecovery`
3. Wait for time delay (default: 7 days)
4. Complete recovery: `mds_completeRecovery`

---

### **4. Spending Limit Wallet**
Enforces daily/weekly/monthly spending limits.

**Use Cases**:
- Budget management
- Child/parental controls
- Risk mitigation
- Enterprise expense controls

**Creation**:
```json
{
  "jsonrpc": "2.0",
  "method": "mds_createWallet",
  "params": {
    "owner": "0x...",
    "walletType": "spendingLimit",
    "config": {
      "spendingLimits": {
        "dailyLimit": "1000000000000000000",
        "weeklyLimit": "5000000000000000000",
        "monthlyLimit": "20000000000000000000"
      }
    }
  },
  "id": 1
}
```

---

## 🔄 Multi-Signature Transactions

### **Create Multi-Sig Transaction**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_createMultisigTransaction",
  "params": {
    "walletAddress": "0x...",
    "to": "0x...",
    "value": "0x3b9aca00",
    "data": "0x"
  },
  "id": 1
}
```

### **Add Signature**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_addMultisigSignature",
  "params": {
    "walletAddress": "0x...",
    "txHash": "0x...",
    "signer": "0x...",
    "signature": "0x...",
    "publicKey": "0x..."
  },
  "id": 1
}
```

### **View Pending Transactions**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_getPendingMultisigTransactions",
  "params": ["0x..."],
  "id": 1
}
```

---

## 🔓 Social Recovery

### **Initiate Recovery**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_initiateRecovery",
  "params": {
    "walletAddress": "0x...",
    "newOwner": "0x...",
    "guardians": ["0x...", "0x...", "0x..."],
    "recoveryThreshold": 2,
    "timeDelay": 604800
  },
  "id": 1
}
```

### **Approve Recovery (Guardian)**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_approveRecovery",
  "params": {
    "walletAddress": "0x...",
    "guardian": "0x..."
  },
  "id": 1
}
```

### **Get Recovery Status**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_getRecoveryStatus",
  "params": {
    "walletAddress": "0x..."
  },
  "id": 1
}
```

### **Complete Recovery**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_completeRecovery",
  "params": {
    "walletAddress": "0x..."
  },
  "id": 1
}
```

---

## 📦 Batch Transactions

### **Create Batch Transaction**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_createBatchTransaction",
  "params": {
    "walletAddress": "0x...",
    "operations": [
      {
        "type": "transfer",
        "to": "0x...",
        "value": "0x3b9aca00"
      },
      {
        "type": "contractCall",
        "contract": "0x...",
        "data": "0x...",
        "value": "0x0"
      }
    ],
    "nonce": 1,
    "gasLimit": "0x100000",
    "gasPrice": "0x3b9aca00"
  },
  "id": 1
}
```

### **Estimate Batch Gas**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_estimateBatchGas",
  "params": {
    "operations": [
      {
        "type": "transfer",
        "to": "0x...",
        "value": "0x3b9aca00"
      }
    ]
  },
  "id": 1
}
```

### **Get Batch Status**

```json
{
  "jsonrpc": "2.0",
  "method": "mds_getBatchStatus",
  "params": {
    "batchId": "0x..."
  },
  "id": 1
}
```

---

## 💡 Use Cases

### **Enterprise Wallet**
- Multi-sig wallet (5-of-7)
- Spending limits per department
- Batch payroll processing

### **Family Wallet**
- Social recovery (family members as guardians)
- Spending limits for children
- Multi-sig for large purchases

### **DeFi Wallet**
- Batch operations (swap + approve + stake)
- Multi-sig for security
- Gas optimization through batching

### **Gaming Wallet**
- Batch claim rewards
- Social recovery (guild members)
- Spending limits for in-game purchases

---

## 🔍 Explorer Usage

### **Create Wallet**
1. Navigate to "Account Abstraction" section
2. Select wallet type
3. Enter owner address
4. Configure wallet (signers, guardians, limits)
5. Click "Create Wallet"
6. Copy wallet address

### **View Wallet**
1. Enter wallet address
2. Click "Lookup Wallet"
3. View wallet type, owner, and configuration

### **Multi-Sig Transactions**
1. Enter multi-sig wallet address
2. Click "View Pending Transactions"
3. See transactions requiring signatures
4. Add signatures via RPC or wallet

### **Recovery Status**
1. Enter wallet address
2. Click "View Recovery Status"
3. See guardian approvals and time remaining

### **Batch Status**
1. Enter batch ID
2. Click "View Batch Status"
3. See operation results and gas usage

---

## ⚠️ Important Notes

### **Security**
- Always verify wallet addresses
- Use trusted guardians for recovery
- Set appropriate multi-sig thresholds
- Monitor spending limits

### **Gas Optimization**
- Batch transactions save 10-30% gas
- Multiple operations in one transaction
- Shared transaction overhead

### **Recovery**
- Time delay prevents immediate attacks
- Guardian selection is critical
- Recovery threshold should be balanced

---

## 📚 Additional Resources

- **RPC Documentation**: See `mondoshawan-blockchain/src/rpc.rs`
- **API Reference**: All `mds_*` methods for Account Abstraction
- **Explorer**: `mondoshawan-explorer-frontend/` for UI examples

---

**Last Updated**: January 2026  
**Status**: Complete and ready for use
