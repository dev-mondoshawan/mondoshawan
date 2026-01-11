# Quick Start Guide

Get up and running with Mondoshawan in minutes.

## Prerequisites

- Rust 1.92.0+
- Visual Studio Build Tools 2022 (Windows) or clang (Linux/macOS)
- Python 3.12+ (optional, for POC)
- Node.js v22.19.0+ (for frontend)

## Installation

### 1. Clone the Repository

```bash
git clone https://github.com/dev-mondoshawan/mondoshawan.git
cd mondoshawan
```

### 2. Build the Project

**Windows (Recommended)**:
```cmd
# Use Developer Command Prompt for VS 2022
cd mondoshawan-blockchain
cargo build --release
```

**Linux/macOS**:
```bash
cd mondoshawan-blockchain
cargo build --release
```

See [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) for detailed build instructions.

### 3. Run the Node

```bash
cd mondoshawan-blockchain
cargo run --bin node --release
```

The node will:
- Create a genesis block automatically
- Start TriStream mining (3 parallel streams)
- Process transactions
- Display real-time statistics

## Access Points

Once running, you can access:

- **Console Dashboard**: Real-time stats in terminal
- **JSON-RPC API**: `http://localhost:8545`
- **HTTP API**: `http://localhost:8080/api/stats`
- **Web Explorer**: Open `mondoshawan-explorer-frontend/index.html` in a browser

## What You'll See

```
╔═══════════════════════════════════════════════════════════╗
║     Mondoshawan Blockchain Node - TriStream Mining       ║
╚═══════════════════════════════════════════════════════════╝

🚀 Starting Mondoshawan Node...
✅ Genesis block created
⛏️  Starting TriStream mining...

✅ Stream A: Mined block #1 with 50 txs, reward: 50 tokens
✅ Stream B: Mined block #2 with 25 txs, reward: 25 tokens
✅ Stream C: Mined block #3 with 10 txs, fees: 0.01 tokens

📊 Stats:
   Blocks: 15
   Transactions: 85
   Miner Balance: 375 tokens
```

## Next Steps

- **[NODE_QUICK_START.md](NODE_QUICK_START.md)** - Detailed node operation guide
- **[DEVELOPER_GUIDE.md](DEVELOPER_GUIDE.md)** - Contributing to the project
- **[JSON_RPC_API_GUIDE.md](JSON_RPC_API_GUIDE.md)** - API documentation
- **[USER_GUIDES/WALLET_GUIDE.md](USER_GUIDES/WALLET_GUIDE.md)** - Using wallets

## Troubleshooting

### Node Won't Start

- Ensure all dependencies are installed (see [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md))
- Check for port conflicts (default ports: 8545, 8080)
- Verify Rust toolchain: `rustc --version`

### Build Errors

See [BUILD_INSTRUCTIONS.md](BUILD_INSTRUCTIONS.md) for detailed troubleshooting.

## Documentation

- **[README.md](README.md)** - Project overview
- **[Mondoshawan_WHITEPAPER.md](Mondoshawan_WHITEPAPER.md)** - Technical whitepaper
- **[TOKENOMICS.md](TOKENOMICS.md)** - Token economics
- **[SECURITY.md](SECURITY.md)** - Security policy
