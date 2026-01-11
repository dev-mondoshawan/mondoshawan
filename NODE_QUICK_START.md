# Mondoshawan Node - Quick Start Guide

## ✅ What's Working

You now have a **fully functional blockchain node** with:

1. ✅ **TriStream Mining** - Three parallel mining streams
2. ✅ **Block Rewards** - Tokenomics implemented
3. ✅ **Transaction Processing** - Full validation and state updates
4. ✅ **Genesis Block** - Automatic creation
5. ✅ **Stats Reporting** - Real-time statistics

---

## 🚀 Running the Node

### Start the Node

**Windows (Recommended)**:
```cmd
# Use Developer Command Prompt for VS 2022
cd mondoshawan-blockchain
cargo run --bin node --release
```

**Linux/macOS**:
```bash
cd mondoshawan-blockchain
cargo run --bin node --release
```

### What You'll See

```
╔═══════════════════════════════════════════════════════════╗
║           Mondoshawan Blockchain Node - TriStream Mining        ║
╚═══════════════════════════════════════════════════════════╝

🚀 Starting Mondoshawan Node...
   Miner Address: 0101010101010101010101010101010101010101
   Data Directory: data
✅ Genesis block created
⛏️  Starting TriStream mining...
   Stream A: 10s blocks, 10,000 txs, 50 token reward
   Stream B: 1s blocks, 5,000 txs, 25 token reward
   Stream C: 100ms blocks, 1,000 txs, fee-based only

📝 Generating test transactions...
   Alice balance: 1000 tokens
   Added transaction 1
   Added transaction 11
   Added transaction 21
   ...

✅ Node is running! Mining blocks...
   Press Ctrl+C to stop

✅ Stream B: Mined block #2 with 25 txs, reward: 25 tokens
✅ Stream C: Mined block #3 with 10 txs, fees: 0.01 tokens
✅ Stream A: Mined block #1 with 50 txs, reward: 50 tokens
✅ Stream B: Mined block #4 with 25 txs, reward: 25 tokens
...

📊 Stats:
   Blocks: 15
   Transactions: 85
   Miner Balance: 375 tokens
```

---

## 📊 Understanding the Output

### Mining Messages
- **Stream A**: Mined every ~10 seconds, 50 token reward
- **Stream B**: Mined every ~1 second, 25 token reward  
- **Stream C**: Mined every ~100ms, fee-based only

### Stats (Every 10 seconds)
- **Blocks**: Total blocks in blockchain
- **Transactions**: Total transactions processed
- **Miner Balance**: Accumulated rewards (in tokens)

---

## 💰 Tokenomics Summary

### Block Rewards
- **Stream A**: 50 tokens per block (10s blocks)
- **Stream B**: 25 tokens per block (1s blocks)
- **Stream C**: 0 tokens (fees only, 100ms blocks)

### Daily Emission
- Stream A: ~432,000 tokens/day
- Stream B: ~2,160,000 tokens/day
- Stream C: Variable (fees only)
- **Total**: ~2.6M tokens/day from rewards

### How Rewards Work
1. Miner creates a block
2. Block is validated and added to blockchain
3. Miner receives reward immediately:
   - Stream A/B: Block reward
   - Stream C: Sum of transaction fees

---

## 🔍 What Mining Actually Does

### 1. Transaction Pool
- Transactions are added to a shared pool
- Each stream extracts transactions based on capacity:
  - Stream A: Up to 10,000 txs
  - Stream B: Up to 5,000 txs
  - Stream C: Up to 1,000 txs

### 2. Block Creation
Each stream:
1. Takes transactions from pool
2. Gets parent block hashes (DAG structure)
3. Creates block with:
   - Block number
   - Parent hashes
   - Transactions
   - Stream type
   - Timestamp
4. Calculates block hash

### 3. Validation & Reward
1. Block is validated (transactions, hashes, etc.)
2. If valid, added to blockchain
3. Miner receives reward:
   - Added to miner's balance
   - Visible in next stats report

### 4. State Updates
- Transaction balances updated
- Nonces incremented
- Fees collected
- Rewards distributed

---

## 🎯 Key Features

### Parallel Mining
- All three streams mine **simultaneously**
- No conflicts (DAG structure)
- Higher throughput than single-chain

### Different Speeds
- **Stream A**: Slow (10s) but high capacity
- **Stream B**: Fast (1s) with medium capacity
- **Stream C**: Ultra-fast (100ms) but lower capacity

### Fee Market
- Stream C is fee-based only
- Creates incentive for high-fee transactions
- Users pay for fast finality

---

## 📝 Next Steps

- **[DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md)** - Contributing to the project
- **[JSON_RPC_API_GUIDE.md](JSON_RPC_API_GUIDE.md)** - API documentation
- **[TOKENOMICS.md](TOKENOMICS.md)** - Token economics
- **[Mondoshawan_WHITEPAPER.md](Mondoshawan_WHITEPAPER.md)** - Technical whitepaper

---

## 🐛 Troubleshooting

### Build Errors

See [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) for detailed troubleshooting.

### Node Won't Start

- Check that genesis block is created
- Verify all dependencies are installed
- Check for port conflicts (default ports: 8545, 8080)
- Ensure Rust toolchain is installed: `rustc --version`

---

## 📚 Documentation

- **[README.md](README.md)** - Project overview
- **[QUICK_START.md](QUICK_START.md)** - Getting started guide
- **[BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md)** - Build instructions
- **[TOKENOMICS.md](TOKENOMICS.md)** - Token economics

---

**You now have a working blockchain node!** 🎉

The node mines blocks, processes transactions, and distributes rewards according to the TriStream architecture. Watch the stats to see your miner balance grow as blocks are mined!
