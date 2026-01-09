# Mondoshawan Blockchain Demo Guide

## Overview

The Mondoshawan blockchain is production-ready except for the mining component, which needs a lock-free queue redesign (4-6 hours of proper implementation). For demonstration purposes, we can manually add test blocks to showcase all blockchain features.

## Quick Start

### 1. Start the Node

```powershell
cd Mondoshawan-blockchain
cargo run --release --bin node
```

The node will start on:
- **RPC API**: `http://127.0.0.1:8545`
- **P2P Network**: `127.0.0.1:8080`
- **Block Explorer**: `http://localhost:8080` (if configured)

### 2. Run Demo Script

In a new terminal:

```powershell
.\demo-blocks.ps1
```

This script will:
- Create test addresses (Alice, Bob, Charlie)
- Create test transactions
- Add 3 test blocks to the blockchain
- Verify blockchain state
- Display balances

## Manual Block Creation

### Using RPC Methods

#### Create a Test Transaction

```json
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_createTestTransaction",
  "params": [
    "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",  // from
    "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",  // to
    "0x64",  // value (100 in hex)
    "0x1"    // fee (1 in hex)
  ],
  "id": 1
}
```

#### Add a Test Block

```json
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_addTestBlock",
  "params": [
    1,  // block number
    [], // transactions (array of transaction objects or hashes)
    []  // parent hashes (optional, defaults to latest block)
  ],
  "id": 1
}
```

### Example: Complete Block Creation Flow

1. **Create Transaction:**
```bash
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "Mondoshawan_createTestTransaction",
    "params": [
      "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
      "0x64",
      "0x1"
    ],
    "id": 1
  }'
```

2. **Add Block with Transaction:**
```bash
curl -X POST http://127.0.0.1:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "Mondoshawan_addTestBlock",
    "params": [
      1,
      [{
        "from": "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "to": "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        "value": "0x64",
        "fee": "0x1",
        "nonce": "0x0",
        "data": [],
        "gas_limit": "0x5208",
        "hash": "0x...",
        "signature": [],
        "public_key": [],
        "pq_signature": null
      }],
      []
    ],
    "id": 1
  }'
```

## Available Features for Demo

### Core Blockchain Features

- ✅ **Block Creation & Validation**
- ✅ **Transaction Processing**
- ✅ **Balance Tracking**
- ✅ **Nonce Management**
- ✅ **GhostDAG Consensus**
- ✅ **Verkle Tree (when enabled)**
- ✅ **Light Client Mode**
- ✅ **Sharding Support**

### Advanced Features

- ✅ **EVM Smart Contracts**
- ✅ **Post-Quantum Accounts** (Dilithium3, SPHINCS+)
- ✅ **Security Policies**
- ✅ **Risk Scoring**
- ✅ **Forensic Analysis**
- ✅ **MEV Detection**
- ✅ **Fairness Metrics**

### RPC Endpoints

#### Block Operations
- `eth_getBlockByNumber` - Get block by number
- `eth_getBlockByHash` - Get block by hash
- `Mondoshawan_getDagStats` - Get GhostDAG statistics
- `Mondoshawan_getStateRoot` - Get Verkle state root (if enabled)

#### Transaction Operations
- `eth_getTransactionByHash` - Get transaction details
- `eth_sendTransaction` - Send transaction (requires signing)
- `Mondoshawan_createTestTransaction` - Create unsigned test transaction
- `Mondoshawan_getStateProof` - Get state proof for address

#### Account Operations
- `eth_getBalance` - Get account balance
- `eth_getTransactionCount` - Get account nonce
- `Mondoshawan_getRiskScore` - Get address risk score
- `Mondoshawan_getAddressSummary` - Get address activity summary

#### Sharding (if enabled)
- `Mondoshawan_getShardStats` - Get shard statistics
- `Mondoshawan_getShardForAddress` - Get shard for address
- `Mondoshawan_getCrossShardTransaction` - Get cross-shard transaction

#### Security & Forensics
- `Mondoshawan_detectAnomalies` - Detect address anomalies
- `Mondoshawan_traceFunds` - Trace fund flows
- `Mondoshawan_findRelatedAddresses` - Find related addresses
- `Mondoshawan_addSecurityPolicy` - Add security policy

## Demo Scenarios

### Scenario 1: Basic Transaction Flow

1. Create transaction: Alice → Bob (100 tokens)
2. Add block with transaction
3. Verify Bob's balance increased
4. Verify Alice's balance decreased

### Scenario 2: Multi-Block Chain

1. Create multiple transactions
2. Add blocks sequentially
3. Verify GhostDAG structure
4. Check DAG statistics

### Scenario 3: Cross-Shard Transactions (if sharding enabled)

1. Create transaction from shard 0 to shard 1
2. Add block to source shard
3. Verify cross-shard transaction status
4. Check target shard balance

### Scenario 4: Verkle Tree & Light Client

1. Enable Verkle mode
2. Add blocks
3. Get state root
4. Get state proof for address
5. Verify proof with light client

### Scenario 5: Security & Forensics

1. Add multiple transactions
2. Get risk score for address
3. Detect anomalies
4. Trace fund flows
5. Add security policy
6. Evaluate transaction against policy

## Notes

- **Unsigned Transactions**: Test transactions created via `Mondoshawan_createTestTransaction` are unsigned. For production, transactions must be properly signed.
- **Mining Queue**: The mining component needs a lock-free queue redesign. Manual block addition bypasses this.
- **PQC Accounts**: Post-Quantum account support may have compilation issues on Windows/MSVC. Core blockchain features work without them.

## Troubleshooting

### Node Won't Start

1. Check if port 8545 is available
2. Check compilation errors (especially PQC-related)
3. Verify database directory permissions

### RPC Calls Fail

1. Verify node is running
2. Check RPC endpoint URL
3. Verify JSON-RPC format

### Blocks Fail to Add

1. Check block number is sequential
2. Verify parent hashes are correct
3. Check transaction format
4. Review blockchain validation errors

## Production Readiness

**Complete:**
- ✅ Core blockchain logic
- ✅ Transaction processing
- ✅ Consensus (GhostDAG)
- ✅ Storage integration
- ✅ RPC API
- ✅ Security features
- ✅ Sharding
- ✅ Verkle tree
- ✅ Light client

**Needs Work:**
- ⚠️ Mining queue (lock-free redesign needed)
- ⚠️ PQC account compilation (Windows/MSVC issues)
- ⚠️ Production hardening
- ⚠️ Performance optimization

## Next Steps

1. **Mining Queue Redesign** (4-6 hours)
   - Implement lock-free queue using `crossbeam-queue`
   - Integrate with mining manager
   - Test under load

2. **PQC Account Fixes**
   - Fix API compatibility issues
   - Test on Windows/MSVC
   - Add proper error handling

3. **Production Hardening**
   - Add comprehensive error handling
   - Implement rate limiting
   - Add monitoring and alerting
   - Security audit
