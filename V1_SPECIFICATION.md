# Mondoshawan Blockchain v1.0 Specification

**Version**: 1.0.0  
**Status**: Draft (Pending Lock)  
**Last Updated**: January 2026

---

## 📋 Overview

This document defines the complete specification for Mondoshawan Blockchain v1.0, including protocol parameters, API surface, network protocol, consensus rules, and storage format.

**This specification is LOCKED for v1.0** - no breaking changes allowed after mainnet launch.

---

## 🎯 Feature Set

### **Included Features**
- ✅ Core blockchain (validation, transactions, state)
- ✅ GhostDAG consensus
- ✅ TriStream mining (3 streams)
- ✅ Storage persistence
- ✅ P2P networking
- ✅ JSON-RPC API
- ✅ Web explorer
- ⚠️ EVM basic support (if fully completed)

### **Excluded Features**
- ❌ Full sharding (metrics-only)
- ❌ Post-quantum cryptography
- ❌ Verkle trees
- ❌ Advanced MEV/fairness logic

---

## ⚙️ Protocol Parameters

### **Block Times & Rewards**

#### Stream A (ASIC - Blake3)
```rust
BLOCK_TIME: 10 seconds
MAX_TRANSACTIONS: 10,000
BLOCK_REWARD: 50 Mondoshawan tokens (50_000_000_000_000_000_000 base units)
MINING_ALGORITHM: Blake3
```

#### Stream B (CPU/GPU - KHeavyHash)
```rust
BLOCK_TIME: 1 second
MAX_TRANSACTIONS: 5,000
BLOCK_REWARD: 25 Mondoshawan tokens (25_000_000_000_000_000_000 base units)
MINING_ALGORITHM: KHeavyHash/RandomX
```

#### Stream C (ZK Proofs)
```rust
BLOCK_TIME: 100 milliseconds
MAX_TRANSACTIONS: 1,000
BLOCK_REWARD: 0 Mondoshawan tokens (fee-based only)
MINING_ALGORITHM: Zero-knowledge proof generation
```

### **Block & Transaction Limits**

```rust
MAX_BLOCK_SIZE: 10,000,000 bytes (10 MB)
MAX_PARENT_HASHES: 10
MAX_TRANSACTION_DATA_SIZE: 131,072 bytes (128 KB)
MAX_TRANSACTION_POOL_SIZE: 100,000 transactions
MAX_NETWORK_MESSAGE_SIZE: 10,000,000 bytes (10 MB)
```

### **Chain Configuration**

```rust
CHAIN_ID: 0x50595258 // "Mondoshawan" in ASCII hex
ADDRESS_FORMAT: Ethereum-compatible (20 bytes, hex-encoded with 0x prefix)
ADDRESS_LENGTH: 20 bytes
HASH_LENGTH: 32 bytes (Blake3)
SIGNATURE_LENGTH: 64 bytes (Ed25519)
```

### **Gas Configuration** (if EVM enabled)

```rust
BLOCK_GAS_LIMIT: 30,000,000 (30M gas)
MIN_GAS_PRICE: 1,000,000,000 wei (1 gwei)
DEFAULT_GAS_LIMIT: 21,000 (simple transfer)
GAS_PER_BYTE: 16 (for transaction data)
```

### **Fee Model**

- **Transaction Fee**: Gas price × gas used (if EVM enabled)
- **Fixed Fee**: 0.001 Mondoshawan per transaction (if EVM disabled)
- **Fee Recipient**: Miner who includes transaction in block

---

## 🏗️ Consensus Rules

### **GhostDAG Parameters**

```rust
K: 3 // Blue set selection parameter
BLUE_SCORE_ALGORITHM: BFS-based traversal
BLOCK_ORDERING: Blue score (primary), timestamp (secondary)
FINALITY_RULE: k-deep confirmation (k = 6 blocks)
```

### **Block Validation Rules**

1. **Structure Validation**
   - Block must have valid header
   - Block must have valid transactions list
   - Block size must not exceed MAX_BLOCK_SIZE
   - Parent hashes count must not exceed MAX_PARENT_HASHES

2. **Hash Validation**
   - Block hash must match calculated hash
   - Parent hashes must reference existing blocks
   - Transaction hashes must be valid

3. **Timestamp Validation**
   - Block timestamp must be within 2 hours of current time
   - Block timestamp must be greater than all parent timestamps

4. **Transaction Validation**
   - All transactions must be valid (see Transaction Rules)
   - Transaction count must not exceed stream limit
   - Transaction data size must not exceed MAX_TX_DATA_SIZE

### **Transaction Validation Rules**

1. **Signature Validation**
   - Transaction must have valid Ed25519 signature
   - Public key must match address derivation
   - Signature must verify against transaction hash

2. **Nonce Validation**
   - Transaction nonce must equal account nonce + 1
   - Nonce must be sequential (no gaps)

3. **Balance Validation**
   - Account balance must be >= (value + fee)
   - Balance check must account for pending transactions

4. **Data Validation**
   - Transaction data size must not exceed MAX_TX_DATA_SIZE
   - Transaction data must be valid (if EVM: valid bytecode/calldata)

---

## 📡 Network Protocol

### **P2P Message Format**

All messages are serialized using `bincode` and wrapped in `AuthenticatedMessage`:

```rust
struct AuthenticatedMessage {
    signature: Vec<u8>,      // Ed25519 signature (64 bytes)
    public_key: Vec<u8>,     // Ed25519 public key (32 bytes)
    timestamp: u64,          // Unix timestamp (seconds)
    message: NetworkMessage,  // Inner message
}
```

### **Message Types**

```rust
enum NetworkMessage {
    NewBlock(Block),
    NewTransaction(Transaction),
    RequestBlocks { from: u64, to: u64 },
    Blocks(Vec<Block>),
    RequestPeers,
    Peers(Vec<SocketAddr>),
    Ping,
    Pong,
}
```

### **Message Authentication**

- All messages must be signed with Ed25519
- Signature covers: `message_hash || timestamp`
- Timestamp must be within 5 minutes of current time (replay protection)
- Public key must be valid Ed25519 key

### **Peer Discovery**

- Initial peers: Bootstrap nodes (hardcoded or config)
- Peer exchange: RequestPeers/Peers messages
- Connection limits: Max 50 peers per node
- Peer scoring: Track good/bad behavior

### **Block Propagation**

1. Miner creates block
2. Miner signs block with AuthenticatedMessage
3. Miner broadcasts to all connected peers
4. Peers validate and propagate to their peers
5. Peers add block to local DAG

### **Transaction Propagation**

1. Node receives transaction (RPC or network)
2. Node validates transaction
3. Node adds to local transaction pool
4. Node broadcasts to all connected peers
5. Peers validate and add to their pools

---

## 🔌 JSON-RPC API

### **Base URL**
- **Testnet**: `https://rpc.testnet.Mondoshawan.io`
- **Mainnet**: `https://rpc.Mondoshawan.io`

### **Request Format**

```json
{
  "jsonrpc": "2.0",
  "method": "method_name",
  "params": [...],
  "id": 1
}
```

### **Response Format**

```json
{
  "jsonrpc": "2.0",
  "result": {...},
  "id": 1
}
```

### **Error Format**

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32000,
    "message": "Error message"
  },
  "id": 1
}
```

### **Ethereum-Compatible Methods**

#### `eth_getBalance`
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "eth_getBalance",
  "params": ["0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", "latest"],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": "0x2386f26fc10000",
  "id": 1
}
```

#### `eth_getTransactionCount`
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "eth_getTransactionCount",
  "params": ["0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb", "latest"],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": "0x5",
  "id": 1
}
```

#### `eth_blockNumber`
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "eth_blockNumber",
  "params": [],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": "0x1234",
  "id": 1
}
```

#### `eth_getBlockByNumber`
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "eth_getBlockByNumber",
  "params": ["latest", true],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "number": "0x1234",
    "hash": "0x...",
    "parentHashes": ["0x..."],
    "transactions": [...],
    "timestamp": "0x..."
  },
  "id": 1
}
```

#### `eth_sendTransaction` (if implemented)
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "eth_sendTransaction",
  "params": [{
    "from": "0x...",
    "to": "0x...",
    "value": "0x...",
    "gas": "0x...",
    "gasPrice": "0x...",
    "data": "0x..."
  }],
  "id": 1
}
```

### **Mondoshawan-Specific Methods**

#### `Mondoshawan_getDagStats`
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_getDagStats",
  "params": [],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": {
    "totalBlocks": 12345,
    "blueBlocks": 10000,
    "redBlocks": 2345,
    "tps": 1500.5
  },
  "id": 1
}
```

#### `Mondoshawan_getBlueScore`
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_getBlueScore",
  "params": ["0x..."],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": "0x1234",
  "id": 1
}
```

#### `Mondoshawan_getTps`
```json
// Request
{
  "jsonrpc": "2.0",
  "method": "Mondoshawan_getTps",
  "params": [],
  "id": 1
}

// Response
{
  "jsonrpc": "2.0",
  "result": 1500.5,
  "id": 1
}
```

### **Health Check Endpoints**

#### `/health`
```json
{
  "status": "healthy",
  "timestamp": 1234567890
}
```

#### `/ready`
```json
{
  "status": "ready",
  "genesis": true,
  "synced": true
}
```

#### `/metrics` (Prometheus format)
```
# HELP blocks_mined_total Total number of blocks mined
# TYPE blocks_mined_total counter
blocks_mined_total 12345

# HELP transactions_processed_total Total number of transactions processed
# TYPE transactions_processed_total counter
transactions_processed_total 1000000
```

### **Rate Limiting**

- **Default**: 100 requests/second per IP
- **Burst**: 200 requests
- **Error Code**: -32005 (Rate limit exceeded)

### **Authentication**

- **Method**: API key via `X-API-Key` header or `api_key` parameter
- **Public Methods**: `eth_blockNumber`, `net_version`, `eth_chainId`
- **Protected Methods**: All other methods require API key (if enabled)

---

## 💾 Storage Format

### **Database Schema** (sled)

#### Block Storage
- **Key**: Block hash (32 bytes, hex-encoded)
- **Value**: Serialized Block (bincode)
- **Tree**: `blocks`

#### State Storage
- **Key**: Address (20 bytes, hex-encoded) + suffix
  - `balance`: Address + `:balance`
  - `nonce`: Address + `:nonce`
- **Value**: 
  - Balance: u128 (little-endian, 16 bytes)
  - Nonce: u64 (little-endian, 8 bytes)
- **Tree**: `state`

#### EVM Storage (if enabled)
- **Key**: Contract address (20 bytes) + storage key (32 bytes)
- **Value**: Storage value (32 bytes)
- **Tree**: `evm_storage`

#### Code Storage (if enabled)
- **Key**: Contract address (20 bytes, hex-encoded)
- **Value**: Contract bytecode (Vec<u8>)
- **Tree**: `evm_code`

### **Data Integrity**

- **Checksums**: TBD (add block hash verification)
- **Versioning**: TBD (add schema version)
- **Backup**: TBD (define backup strategy)

---

## 🪙 Genesis & Tokenomics

### **Genesis Block**

```rust
struct GenesisBlock {
    timestamp: u64,           // Testnet launch timestamp
    chain_id: u64,           // 0x50595258
    alloc: HashMap<Address, u128>, // Initial allocations
}
```

### **Initial Allocations**

- **Development Team**: 10% (1,000,000,000 Mondoshawan)
- **Community Fund**: 5% (500,000,000 Mondoshawan)
- **Testnet Faucet**: 1% (100,000,000 Mondoshawan)
- **Reserved**: 84% (8,400,000,000 Mondoshawan)

### **Emission Schedule**

- **Stream A**: 432,000 tokens/day
- **Stream B**: 2,160,000 tokens/day
- **Stream C**: 0 tokens (fee-based only)
- **Total**: ~2,592,000 tokens/day
- **Annual**: ~946,080,000 tokens/year
- **Halving**: TBD (every 4 years or never)

---

## 🔐 Security Specifications

### **Cryptography**

- **Hashing**: Blake3 (32-byte output)
- **Signatures**: Ed25519 (64-byte signature, 32-byte public key)
- **Address Derivation**: `Keccak256(public_key)[12:32]` (last 20 bytes)

### **Message Authentication**

- **Algorithm**: Ed25519
- **Message Format**: `hash(message) || timestamp`
- **Timestamp Window**: ±5 minutes
- **Replay Protection**: Timestamp validation

### **Network Security**

- **Message Size Limit**: 10 MB
- **Connection Limits**: 50 peers per node
- **Rate Limiting**: Per-peer and per-IP
- **Peer Scoring**: Track and ban malicious peers

---

## 📊 Performance Targets

### **Throughput**

- **Target TPS**: 1,000+ (without sharding)
- **Peak TPS**: 5,000+ (with all streams)
- **With Sharding**: 10,000+ TPS (10 shards)

### **Latency**

- **Block Time**: 100ms (Stream C), 1s (Stream B), 10s (Stream A)
- **Finality**: 100ms (Stream C with ZK proofs)
- **RPC Latency**: <100ms (p95)

### **Resource Requirements**

- **Full Node**: 4 CPU cores, 8GB RAM, 100GB disk
- **Miner Node**: 8 CPU cores, 16GB RAM, 200GB disk
- **Sync Time**: <24 hours for full node

---

## 🔄 Versioning & Compatibility

### **Versioning Strategy**

- **Semantic Versioning**: MAJOR.MINOR.PATCH
- **Breaking Changes**: MAJOR version bump
- **API Changes**: Documented in changelog
- **Deprecation**: 6-month notice period

### **Backward Compatibility**

- **Block Format**: Immutable (no changes after mainnet)
- **Transaction Format**: Immutable (no changes after mainnet)
- **RPC API**: Backward compatible within MAJOR version
- **Storage Format**: Migration path for schema changes

---

## ✅ Specification Lock

**This specification is LOCKED for v1.0** - no breaking changes allowed after mainnet launch.

**Changes allowed**:
- Bug fixes
- Performance improvements
- Non-breaking API additions
- Documentation updates

**Changes NOT allowed**:
- Breaking protocol changes
- Breaking API changes
- Consensus rule changes
- Block/transaction format changes

---

## 📝 Changelog

### v1.0.0 (Draft)
- Initial specification
- Protocol parameters defined
- API surface defined
- Network protocol defined

---

**Last Updated**: January 2026  
**Status**: Draft (Pending Lock)  
**Next Review**: Before testnet launch
