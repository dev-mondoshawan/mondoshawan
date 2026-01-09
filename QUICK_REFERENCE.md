# Mondoshawan Blockchain - Quick Reference

**One-page guide for developers**

---

## 🚀 Quick Start

```powershell
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo build --bin node
cargo run --bin node
```

**Access Points**:
- Console Dashboard: Terminal output
- HTTP API: http://localhost:8080/api/stats
- JSON-RPC: http://localhost:8545
- Web Explorer: `Mondoshawan-explorer-frontend/index.html`

---

## 📚 Essential Documentation

| Document | Purpose |
|----------|---------|
| **PROJECT_STATUS.md** | Complete current state |
| **NEXT_STEPS.md** | What to work on |
| **DEVELOPER_GUIDE.md** | Developer onboarding |
| **NODE_QUICK_START.md** | How to run node |

---

## 🏗️ Architecture

```
Node → Mining/Network/RPC → Blockchain → GhostDAG/Storage → Database
```

**Key Modules**:
- `src/blockchain/` - Core logic
- `src/consensus.rs` - GhostDAG
- `src/mining.rs` - TriStream
- `src/storage.rs` - Persistence
- `src/network.rs` - P2P
- `src/rpc.rs` - JSON-RPC API

---

## ✅ Current Status

**Complete**:
- ✅ Core blockchain
- ✅ TriStream mining (3 streams)
- ✅ GhostDAG consensus
- ✅ Storage persistence
- ✅ P2P network
- ✅ JSON-RPC API
- ✅ Node application

**Needs Work**:
- ⚠️ EVM integration (stubbed)
- ⚠️ Sharding (basic structure)

---

## 🎯 Next Steps

1. **EVM Integration** (2-3 days) - Enable smart contracts
2. **Production Hardening** (1-2 weeks) - Security & performance
3. **Sharding** (1-2 weeks) - Horizontal scaling

See `NEXT_STEPS.md` for details.

---

## 🔧 Common Commands

```powershell
# Build
cargo build --bin node

# Test
cargo test

# Run
cargo run --bin node

# Check
cargo check

# Format
cargo fmt

# Lint
cargo clippy
```

---

## 📖 Key Concepts

**TriStream Mining**: 3 parallel streams (ASIC/CPU/GPU/ZK)  
**GhostDAG**: BlockDAG consensus with blue scores  
**Storage**: `sled` database for persistence  
**Network**: P2P with peer discovery

---

## 🐛 Troubleshooting

- **Build errors**: Check MSVC environment (`LIB` variable)
- **Runtime errors**: Check `data/` directory permissions
- **Network errors**: Check port availability

---

## 📞 Resources

- **Status**: `PROJECT_STATUS.md`
- **Roadmap**: `NEXT_STEPS.md`
- **Onboarding**: `DEVELOPER_GUIDE.md`
- **Features**: `GHOSTDAG_*.md`, `NETWORK_*.md`, etc.

---

**For details, see the full documentation files!**
