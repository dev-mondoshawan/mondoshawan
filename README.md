# Mondoshawan Blockchain

**Mondoshawan Protocol** | Ticker: **MSHW**

High-performance sharded blockchain with TriStream mining architecture and GhostDAG consensus. **160,000+ TPS** with 10 shards, **97% fair launch** tokenomics.

**Websites**: [MONDOSHAWAN.network](https://mondoshawan.network) | [MONDOSHAWAN.io](https://mondoshawan.io) | [MONDOSHAWAN.xyz](https://mondoshawan.xyz)

## 🏗️ Project Structure

```
Mondoshawan/
├── mondoshawan_poc/        # Python Proof of Concept
│   ├── tristream.py        # TriStream mining (3 streams)
│   ├── ghostdag.py         # GhostDAG consensus
│   ├── shard_manager.py    # Sharding system
│   ├── mainnet.py          # Mainnet implementation
│   └── benchmark.py        # Performance testing
│
├── mondoshawan_real/       # Real Implementation Components
│   ├── post_quantum_crypto.py  # NIST PQC standards
│   └── verkle_tree.py      # Verkle tree implementation
│
├── mondoshawan-blockchain/ # Rust Blockchain Implementation
│   ├── src/                # Source code
│   │   ├── blockchain/     # Core blockchain
│   │   ├── consensus/      # GhostDAG consensus
│   │   ├── evm/            # EVM integration
│   │   ├── sharding/       # Sharding support
│   │   ├── security/       # AI-driven security & fraud detection
│   │   ├── mining/         # TriStream mining + fairness metrics
│   │   └── ...
│   └── tests/              # Integration tests
│
├── mondoshawan-explorer-frontend/  # Web Block Explorer
│   ├── index.html
│   ├── app.js
│   └── styles.css
│
├── SECURITY/               # Security documentation
└── USER_GUIDES/            # User guides
```

## 🚀 Quick Start

### Prerequisites

- **Rust**: Installed on `D:\Rust\` (rustc 1.92.0+)
- **Python**: Installed on `D:\Python\` (Python 3.12+)
- **Node.js**: v22.19.0+ (for frontend)
- **Visual Studio Build Tools**: Required for Rust MSVC toolchain

### Setup

1. **Install Rust** (if not already installed):
   ```powershell
   # Rust is installed at D:\Rust\
   $env:CARGO_HOME = "D:\Rust\.cargo"
   $env:RUSTUP_HOME = "D:\Rust\.rustup"
   ```

2. **Install Python** (if not already installed):
   ```powershell
   # Python is installed at D:\Python\
   $env:PATH = "D:\Python;D:\Python\Scripts;$env:PATH"
   ```

3. **Install Visual Studio Build Tools**:
   - Download from: https://visualstudio.microsoft.com/downloads/
   - Install "Build Tools for Visual Studio 2022"
   - Select "C++ build tools" and "Windows SDK"

### Running the Python POC

```bash
cd mondoshawan_poc
python -m asyncio  # Run testnet
python -m mainnet  # Run mainnet
python -m benchmark  # Run benchmarks
```

### Building the Rust Project

```bash
cd mondoshawan-blockchain
cargo build
cargo test
```

### Running the Frontend

```bash
cd mondoshawan-explorer-frontend
# Open index.html in browser or use a local server
python -m http.server 3000
```

## 📊 Features

### Performance
- **Base Throughput**: ~16,000 TPS per shard (combined streams)
- **Sharded Throughput**: 160,000+ TPS (10 shards), up to 1.6M+ TPS (100 shards)
- **Block Times**: 10s (Stream A), 1s (Stream B), 100ms (Stream C)
- **Finality**: 1-10 seconds (same-shard), 2-12 seconds (cross-shard)

### TriStream Mining
- **Stream A**: ASIC mining (Blake3, 10s blocks, 50 MSHW/block)
- **Stream B**: CPU/GPU mining (KHeavyHash, 1s blocks, 20 MSHW/block)
- **Stream C**: ZK proofs (100ms blocks, 5 MSHW/block)
- **97% Fair Launch**: 8.7B MSHW from mining, 3% presale for development

### Consensus
- **GhostDAG**: DAG-based consensus algorithm
- Parallel block processing
- High throughput

### Sharding
- **Default: 10 shards** (160,000+ TPS)
- **Scalable: 1-100 shards** (up to 1.6M+ TPS)
- Cross-shard transactions with two-phase commit
- Consistent hashing for deterministic routing
- Per-shard transaction pools with DoS protection

### EVM Compatibility
- Full EVM support
- Smart contract execution
- Gas metering

### AI-Native Features 🔒
- **Security & Fraud Detection:**
  - Rule-based pattern matching (honeypot, mixer, phishing)
  - Real-time risk scoring for addresses and transactions
  - Malicious address blacklist
- **Fairness Metrics:**
  - MEV pattern detection (sandwich attacks, back-running)
  - Transaction reordering distance tracking
  - Block-level fairness scores
- **Explorer Integration:**
  - Color-coded risk visualization
  - Real-time security analysis
  - Risk label badges

## 🧪 Testing

### Python Tests
```bash
cd mondoshawan_poc
python -m pytest  # If pytest is installed
```

### Rust Tests
```bash
cd mondoshawan-blockchain
cargo test
cargo test --test integration_test
```

## 📚 Documentation

### Getting Started
- **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - Complete project status and current capabilities
- **[NEXT_STEPS.md](NEXT_STEPS.md)** - Roadmap and development priorities
- **[NODE_QUICK_START.md](NODE_QUICK_START.md)** - How to run the node
- **[SETUP_GUIDE.md](SETUP_GUIDE.md)** - Environment setup instructions

### Core Features
- **[GHOSTDAG_IMPLEMENTATION.md](GHOSTDAG_IMPLEMENTATION.md)** - GhostDAG consensus algorithm
- **[TOKENOMICS_AND_MINING.md](TOKENOMICS_AND_MINING.md)** - TriStream mining and tokenomics
- **[STORAGE_INTEGRATION_COMPLETE.md](STORAGE_INTEGRATION_COMPLETE.md)** - Persistence layer
- **[NETWORK_LAYER_GUIDE.md](NETWORK_LAYER_GUIDE.md)** - P2P networking guide

### APIs & Interfaces
- **[JSON_RPC_API_GUIDE.md](JSON_RPC_API_GUIDE.md)** - JSON-RPC 2.0 API reference
- **[GUI_INTERFACE_GUIDE.md](GUI_INTERFACE_GUIDE.md)** - Web explorer usage

### AI-Native Features
- **[AI_NATIVE_L1_STRATEGY.md](AI_NATIVE_L1_STRATEGY.md)** - Complete AI strategy and roadmap
- **[AI_IMPLEMENTATION_QUICKSTART.md](AI_IMPLEMENTATION_QUICKSTART.md)** - Implementation guide
- **[AI_IMPLEMENTATION_STATUS.md](AI_IMPLEMENTATION_STATUS.md)** - Current implementation status

### User Guides
- [Security Guide](SECURITY/SECURITY_GUIDE.md)
- [Wallet Guide](USER_GUIDES/WALLET_GUIDE.md)
- [Staking Guide](USER_GUIDES/STAKING_GUIDE.md)
- [Frontend README](mondoshawan-explorer-frontend/README.md)

## 🔧 Development

### Environment Variables

```powershell
# Rust
$env:CARGO_HOME = "D:\Rust\.cargo"
$env:RUSTUP_HOME = "D:\Rust\.rustup"

# Python
$env:PATH = "D:\Python;D:\Python\Scripts;$env:PATH"
```

### Project Status

- ✅ **Core Blockchain**: Complete with full validation
- ✅ **TriStream Mining**: All 3 streams operational
- ✅ **GhostDAG Consensus**: Full BlockDAG implementation
- ✅ **Storage Persistence**: Blocks and state survive restarts
- ✅ **P2P Network**: Multi-node communication working
- ✅ **JSON-RPC API**: Ethereum-compatible interface
- ✅ **Node Application**: Working node with dashboard
- ✅ **Web Explorer**: Frontend visualization
- ✅ **AI Security Module**: Fraud detection and risk scoring
- ✅ **Fairness Metrics**: MEV detection and fairness tracking
- ✅ **EVM Integration**: Basic integration complete (revm 33.1)
- ✅ **Sharding**: Core implementation complete (10 shards default, 160,000+ TPS)
- ✅ **Tokenomics**: 97% fair launch model with 10B max supply, 4-year halving

**See [PROJECT_STATUS.md](PROJECT_STATUS.md) for complete details.**

## 📝 License

MIT OR Apache-2.0

## 🌐 Community & Links

- **Website**: https://mondoshawan.io
- **GitHub**: https://github.com/dev-mondoshawan/mondoshawan
- **Whitepaper**: https://mondoshawan.io/Mondoshawan_WHITEPAPER.html
- **Explorer**: https://mondoshawan.io/explorer/
- **Twitter**: @DevMondoshawan (https://x.com/DevMondoshawan)

## 🤝 Contributing

See [SECURITY/SECURITY_GUIDE.md](SECURITY/SECURITY_GUIDE.md) for security best practices.

