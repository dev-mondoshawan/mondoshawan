# Mondoshawan Tokenomics & Mining Guide

## Overview

Mondoshawan uses a **TriStream Mining Architecture** with three parallel mining streams, each with different characteristics, block times, and reward structures.

---

## TriStream Mining Architecture

### Stream A: ASIC Mining (Blake3)
- **Block Time**: 10 seconds
- **Max Transactions per Block**: 10,000
- **Block Reward**: **50 Mondoshawan tokens**
- **Mining Type**: ASIC-optimized (Blake3 hashing)
- **Purpose**: High-throughput transaction processing

**Tokenomics**: Miners receive 50 tokens for each block they mine. This stream handles the bulk of transactions (up to 10,000 per block).

### Stream B: CPU/GPU Mining (KHeavyHash/RandomX)
- **Block Time**: 1 second
- **Max Transactions per Block**: 5,000
- **Block Reward**: **25 Mondoshawan tokens**
- **Mining Type**: CPU/GPU-friendly (KHeavyHash/RandomX algorithms)
- **Purpose**: Fast transaction confirmation

**Tokenomics**: Miners receive 25 tokens for each block. This stream provides faster confirmation times (1 second) but with lower capacity.

### Stream C: ZK Proof Mining
- **Block Time**: 100 milliseconds (0.1 seconds)
- **Max Transactions per Block**: 1,000
- **Block Reward**: **0 Mondoshawan tokens** (fee-based only)
- **Mining Type**: Zero-knowledge proof generation
- **Purpose**: Ultra-fast finality with ZK proofs

**Tokenomics**: Miners receive **only transaction fees**, no block reward. This incentivizes:
- High-fee transactions (users pay more for fast finality)
- Efficient transaction batching
- ZK proof generation efficiency

---

## Token Supply & Distribution

### Block Rewards
- **Stream A**: 50 tokens per block × ~8,640 blocks/day = **432,000 tokens/day**
- **Stream B**: 25 tokens per block × ~86,400 blocks/day = **2,160,000 tokens/day**
- **Stream C**: 0 tokens (fee-based only)

**Total Daily Emission**: ~2,592,000 tokens/day (from block rewards)

### Transaction Fees
- All streams collect transaction fees
- Fees go to the miner who includes the transaction
- Stream C miners rely entirely on fees

### Token Units
- **Base Unit**: 1 token = 1,000,000,000,000,000,000 (10^18) base units
- This follows Ethereum's wei system for compatibility

---

## How Mining Works

### 1. Transaction Pool
- Transactions are added to a shared pool
- Each stream extracts transactions based on its capacity:
  - Stream A: Up to 10,000 txs
  - Stream B: Up to 5,000 txs
  - Stream C: Up to 1,000 txs

### 2. Block Creation
Each stream independently:
1. Extracts transactions from the pool
2. Gets parent block hashes (DAG structure)
3. Creates a new block with:
   - Block number
   - Parent hashes
   - Transactions
   - Stream type
   - Timestamp
4. Calculates block hash

### 3. Block Validation
- Blocks are validated by the blockchain
- Transactions are checked for:
  - Valid signatures/hashes
  - Sufficient balance
  - Correct nonces
  - Valid gas limits

### 4. Reward Distribution
- **Stream A & B**: Miner receives block reward immediately upon block acceptance
- **Stream C**: Miner receives sum of all transaction fees in the block
- Rewards are added to miner's balance in the blockchain state

### 5. DAG Structure
- Blocks reference multiple parent blocks (not just one)
- GhostDAG consensus orders blocks
- Blue set selection determines canonical chain
- Allows parallel block creation without conflicts

---

## Mining Economics

### Stream A Economics
- **Reward**: 50 tokens per block
- **Frequency**: Every 10 seconds
- **Daily Income**: ~432,000 tokens (at 100% uptime)
- **Best For**: ASIC miners with high hash power

### Stream B Economics
- **Reward**: 25 tokens per block
- **Frequency**: Every 1 second
- **Daily Income**: ~2,160,000 tokens (at 100% uptime)
- **Best For**: CPU/GPU miners, more decentralized

### Stream C Economics
- **Reward**: Transaction fees only
- **Frequency**: Every 100ms
- **Daily Income**: Variable (depends on fee volume)
- **Best For**: ZK proof generators, fee maximization

**Note**: Stream B appears more profitable due to higher frequency, but requires more computational resources per block.

---

## Running the Node

### Start the Node
```bash
cd Mondoshawan-blockchain
cargo run --bin node
```

### What Happens
1. **Genesis Block**: Created automatically
2. **Mining Starts**: All three streams begin mining concurrently
3. **Transaction Pool**: Transactions can be added via API (future)
4. **Stats Reporting**: Every 10 seconds, shows:
   - Total blocks mined
   - Total transactions processed
   - Miner balance (accumulated rewards)

### Example Output
```
🚀 Starting Mondoshawan Node...
   Miner Address: 0101010101010101010101010101010101010101
   Data Directory: data
✅ Genesis block created
⛏️  Starting TriStream mining...
   Stream A: 10s blocks, 10,000 txs, 50 token reward
   Stream B: 1s blocks, 5,000 txs, 25 token reward
   Stream C: 100ms blocks, 1,000 txs, fee-based only

✅ Stream A: Mined block #1 with 50 txs, reward: 50 tokens
✅ Stream B: Mined block #2 with 25 txs, reward: 25 tokens
✅ Stream C: Mined block #3 with 10 txs, fees: 0.01 tokens

📊 Stats:
   Blocks: 3
   Transactions: 85
   Miner Balance: 75 tokens
```

---

## Key Features

### 1. Parallel Mining
- All three streams mine simultaneously
- No conflicts due to DAG structure
- Higher throughput than single-chain systems

### 2. Different Block Times
- Stream A: Slow but high capacity
- Stream B: Medium speed and capacity
- Stream C: Ultra-fast but lower capacity

### 3. Fee Market
- Stream C creates a fee market
- Users can pay higher fees for faster confirmation
- Miners compete for high-fee transactions

### 4. Decentralization
- Stream B (CPU/GPU) is more accessible
- Stream A (ASIC) provides security through hash power
- Stream C (ZK) provides fast finality

---

## Future Enhancements

1. **Difficulty Adjustment**: Dynamic difficulty per stream
2. **Staking**: Proof-of-stake option for Stream B
3. **ZK Proof Verification**: Full ZK proof generation/verification
4. **Network Layer**: P2P block propagation
5. **RPC API**: JSON-RPC interface for transactions

---

## Summary

**Mondoshawan Tokenomics**:
- **Stream A**: 50 tokens/block, 10s blocks → High capacity
- **Stream B**: 25 tokens/block, 1s blocks → Fast confirmation
- **Stream C**: Fees only, 100ms blocks → Ultra-fast finality

**Total Daily Emission**: ~2.6M tokens from block rewards + transaction fees

**Mining**: Three parallel streams create blocks independently, with rewards distributed based on stream type. The DAG structure allows all blocks to coexist without conflicts.
