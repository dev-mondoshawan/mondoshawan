# Mondoshawan Blockchain - Developer Guide

**Welcome!** This guide will help you understand the codebase and get started contributing to Mondoshawan.

---

## 🎯 Quick Start for New Developers

### 1. Read These First (in order)
1. **[README.md](README.md)** - Project overview
2. **[PROJECT_STATUS.md](PROJECT_STATUS.md)** - Current implementation status
3. **[NEXT_STEPS.md](NEXT_STEPS.md)** - What to work on next
4. **[NODE_QUICK_START.md](NODE_QUICK_START.md)** - How to run the node

### 2. Set Up Your Environment

```powershell
# Navigate to project
cd D:\Mondoshawan

# Set Rust environment (if needed)
$env:CARGO_HOME = "D:\Rust\.cargo"
$env:RUSTUP_HOME = "D:\Rust\.rustup"

# Set MSVC environment (if needed)
$env:LIB = "D:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x64;$env:LIB"

# Build the project
cd Mondoshawan-blockchain
cargo build

# Run tests
cargo test

# Run the node
cargo run --bin node
```

### 3. Explore the Codebase

Start with these key files:
- `src/bin/node.rs` - Entry point, see how everything starts
- `src/node/mod.rs` - Node orchestration
- `src/blockchain/mod.rs` - Core blockchain logic
- `src/consensus.rs` - GhostDAG implementation
- `src/mining.rs` - TriStream mining

---

## 🏗️ Architecture Overview

### System Components

```
┌─────────────────────────────────────────────────────────┐
│                    Node (node.rs)                        │
│                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐ │
│  │   Mining     │  │   Network    │  │  JSON-RPC    │ │
│  │   Manager    │  │   Manager    │  │   Server     │ │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘ │
│         │                  │                  │          │
│         └──────────────────┼──────────────────┘          │
│                            │                             │
│                   ┌─────────▼─────────┐                  │
│                   │   Blockchain      │                  │
│                   │  ┌─────────────┐ │                  │
│                   │  │  GhostDAG   │ │                  │
│                   │  │  Consensus  │ │                  │
│                   │  └─────────────┘ │                  │
│                   │  ┌─────────────┐ │                  │
│                   │  │   Storage   │ │                  │
│                   │  │   (sled)    │ │                  │
│                   │  └─────────────┘ │                  │
│                   └─────────┬─────────┘                  │
│                             │                             │
│                   ┌─────────▼─────────┐                  │
│                   │   Database (sled)  │                  │
│                   └─────────────────────┘                  │
└─────────────────────────────────────────────────────────────┘
```

### Data Flow

1. **Mining Flow**:
   ```
   MiningManager → creates Block → Blockchain.add_block() 
   → GhostDAG.add_block() → Storage.put_block() → Database
   ```

2. **Network Flow**:
   ```
   Peer → NetworkManager → receives Block → Blockchain.add_block() 
   → Validation → Storage → Database
   ```

3. **RPC Flow**:
   ```
   Client → RpcServer → Blockchain query → Response
   ```

4. **Consensus Flow**:
   ```
   Block added → GhostDAG calculates blue scores → Orders blocks 
   → Consensus order available
   ```

---

## 📁 Codebase Structure

### Core Modules

#### `src/blockchain/` - Core Blockchain
- **`mod.rs`**: Main blockchain logic
  - Block validation
  - Transaction processing
  - State management (balances, nonces)
  - GhostDAG integration
  - Storage integration
- **`block.rs`**: Block and transaction structures
- **`tests.rs`**: Unit tests

**Key Functions**:
- `Blockchain::new()` - Create new blockchain
- `Blockchain::with_storage()` - Create with persistence
- `Blockchain::add_block()` - Add and validate block
- `Blockchain::process_transaction()` - Process transaction
- `Blockchain::get_balance()` - Get account balance

#### `src/consensus.rs` - GhostDAG Consensus
- Blue score calculation (BFS algorithm)
- Blue/Red set selection
- Block ordering
- DAG statistics

**Key Functions**:
- `GhostDAG::new()` - Create new GhostDAG
- `GhostDAG::add_block()` - Add block to DAG
- `GhostDAG::get_ordered_blocks()` - Get consensus order
- `GhostDAG::get_stats()` - Get DAG statistics

#### `src/mining.rs` - TriStream Mining
- Three parallel mining streams
- Transaction pool management
- Block creation
- Reward distribution

**Key Functions**:
- `MiningManager::new()` - Create mining manager
- `MiningManager::start_mining()` - Start all streams
- `MiningManager::mine_stream_a/b/c()` - Mine specific stream

#### `src/storage.rs` - Persistence
- `sled` database integration
- Block storage
- State storage (balances, nonces)

**Key Types**:
- `Database` - Database wrapper
- `BlockStore` - Block persistence
- `StateStore` - State persistence

#### `src/network.rs` - P2P Network
- Peer discovery
- Block propagation
- Transaction broadcasting
- Chain synchronization

**Key Functions**:
- `NetworkManager::new()` - Create network manager
- `NetworkManager::start()` - Start network
- `NetworkManager::broadcast_block()` - Send block to peers

#### `src/rpc.rs` - JSON-RPC API
- JSON-RPC 2.0 server
- Ethereum-compatible methods
- Mondoshawan-specific methods

**Key Functions**:
- `RpcServer::new()` - Create RPC server
- `RpcServer::handle_request()` - Handle RPC request

#### `src/node/` - Node Management
- **`mod.rs`**: Node orchestration
  - Node startup
  - Component integration
  - Configuration
- **`pool.rs`**: Transaction pool

#### `src/evm.rs` - EVM Integration
- **Status**: ⚠️ Stubbed (needs implementation)
- EVM executor (placeholder)
- Contract execution (to be implemented)

#### `src/sharding.rs` - Sharding
- **Status**: ⚠️ Basic structure (needs implementation)
- Shard manager (placeholder)
- Transaction routing (to be implemented)

---

## 🔍 Understanding Key Concepts

### TriStream Mining

Three parallel mining streams with different characteristics:

1. **Stream A** (ASIC):
   - Algorithm: Blake3
   - Block time: 10 seconds
   - Max txs: 10,000
   - Reward: 50 tokens

2. **Stream B** (CPU/GPU):
   - Algorithm: KHeavyHash
   - Block time: 1 second
   - Max txs: 5,000
   - Reward: 25 tokens

3. **Stream C** (ZK):
   - Algorithm: ZK proofs
   - Block time: 100ms
   - Max txs: 1,000
   - Reward: Fee-based only

See `TOKENOMICS_AND_MINING.md` for details.

### GhostDAG Consensus

BlockDAG consensus algorithm:
- Blocks form a DAG (not a chain)
- Blue score determines consensus order
- Blue set = selected blocks
- Red set = orphaned blocks

See `GHOSTDAG_IMPLEMENTATION.md` for details.

### Storage Architecture

- **Database**: `sled` embedded database
- **BlockStore**: Persists blocks by hash
- **StateStore**: Persists balances and nonces
- **Location**: `data/` directory (configurable)

See `STORAGE_INTEGRATION_COMPLETE.md` for details.

---

## 🧪 Testing

### Running Tests

```powershell
# All tests
cargo test

# Specific test
cargo test test_blockchain_consensus

# With output
cargo test -- --nocapture

# Integration tests
cargo test --test integration_test
```

### Test Structure

- **Unit Tests**: In `src/blockchain/tests.rs`
- **Integration Tests**: In `tests/` directory
- **Test Coverage**: Core functionality covered

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Test code here
        assert_eq!(expected, actual);
    }
}
```

---

## 🛠️ Development Workflow

### Making Changes

1. **Pick a task** from `NEXT_STEPS.md`
2. **Create a branch** (if using git)
3. **Make changes** to code
4. **Run tests**: `cargo test`
5. **Build**: `cargo build`
6. **Test manually**: `cargo run --bin node`
7. **Update documentation** if needed
8. **Commit changes**

### Code Style

- Follow Rust conventions
- Use `rustfmt` for formatting
- Use `clippy` for linting
- Add comments for complex logic

### Common Tasks

#### Adding a New RPC Method

1. Add method to `src/rpc.rs`:
```rust
async fn my_new_method(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
    // Implementation
}
```

2. Add to method match in `handle_request()`:
```rust
"my_new_method" => self.my_new_method(request.params).await,
```

3. Update `JSON_RPC_API_GUIDE.md`

#### Adding a New Block Validation Rule

1. Add validation in `src/blockchain/mod.rs`:
```rust
fn validate_block_structure(&self, block: &Block) -> Result<(), String> {
    // Existing validations...
    
    // New validation
    if block.header.some_field != expected {
        return Err("Validation error".to_string());
    }
    
    Ok(())
}
```

2. Add test in `src/blockchain/tests.rs`

#### Adding a New Mining Stream

1. Add constants in `src/mining.rs`:
```rust
pub const STREAM_D_REWARD: u128 = ...;
pub const STREAM_D_BLOCK_TIME: Duration = ...;
```

2. Add mining function:
```rust
async fn mine_stream_d(&self) {
    // Implementation
}
```

3. Add to `start_mining()` in `MiningManager`

---

## 🐛 Debugging

### Common Issues

#### Build Errors
- **MSVC not found**: Set `LIB` environment variable
- **Linker errors**: Check Visual Studio installation
- **Dependency errors**: Run `cargo update`

#### Runtime Errors
- **Database errors**: Check `data/` directory permissions
- **Network errors**: Check port availability
- **Validation errors**: Check block/transaction format

### Debugging Tips

1. **Add logging**:
```rust
println!("Debug: {:?}", value);
```

2. **Use debugger**: Set breakpoints in IDE

3. **Check logs**: Node outputs to console

4. **Test components**: Test individual modules

---

## 📚 Key Documentation Files

### Must Read
- `PROJECT_STATUS.md` - Current state
- `NEXT_STEPS.md` - What to work on
- `NODE_QUICK_START.md` - How to run

### Feature-Specific
- `GHOSTDAG_IMPLEMENTATION.md` - Consensus details
- `TOKENOMICS_AND_MINING.md` - Mining details
- `STORAGE_INTEGRATION_COMPLETE.md` - Storage details
- `NETWORK_LAYER_GUIDE.md` - Network details
- `JSON_RPC_API_GUIDE.md` - API reference

### Reference
- `README.md` - Project overview
- `DEVELOPMENT_ROADMAP.md` - Long-term plan

---

## 🎓 Learning Resources

### Rust
- **The Rust Book**: https://doc.rust-lang.org/book/
- **Rust by Example**: https://doc.rust-lang.org/rust-by-example/
- **Tokio Tutorial**: https://tokio.rs/tokio/tutorial

### Blockchain
- **Ethereum Docs**: https://ethereum.org/en/developers/docs/
- **Kaspa GhostDAG**: https://kaspa.org
- **BlockDAG Research**: Academic papers

### Tools
- **REVM**: https://revm.sh/
- **sled**: https://github.com/spacejam/sled
- **serde**: https://serde.rs/

---

## 🤝 Contributing

### Before You Start
1. Read `PROJECT_STATUS.md` to understand current state
2. Read `NEXT_STEPS.md` to see what's needed
3. Pick a task that matches your skill level

### Contribution Guidelines
- ✅ Code must compile without warnings
- ✅ Tests must pass
- ✅ Documentation updated
- ✅ Follow Rust conventions

### Getting Help
- Check documentation first
- Review code comments
- Study similar implementations
- Ask questions (if team available)

---

## ✅ Checklist for New Features

When adding a new feature:

- [ ] Code implemented
- [ ] Tests written
- [ ] Tests pass
- [ ] Documentation updated
- [ ] Code reviewed (if applicable)
- [ ] Manual testing done
- [ ] No warnings/errors

---

## 🚀 Next Steps

1. **Read** `PROJECT_STATUS.md` and `NEXT_STEPS.md`
2. **Pick a task** that interests you
3. **Set up** your development environment
4. **Explore** the codebase
5. **Start coding!**

**Welcome to Mondoshawan development!** 🎉

---

**Questions?** Check the documentation or review the code - everything is well-commented and documented.
