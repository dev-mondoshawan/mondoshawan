# Parallel EVM User Guide

**Mondoshawan Protocol** - Using Parallel EVM for Maximum Performance

---

## 📖 Overview

Parallel EVM enables executing multiple transactions simultaneously when they don't conflict, providing **10-100x performance improvements** for DeFi operations.

---

## 🚀 Quick Start

### **Enable Parallel EVM**

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "mds_enableParallelEVM",
    "params": {"enabled": true},
    "id": 1
  }'
```

### **Check Status**

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "mds_getParallelEVMStats",
    "params": {},
    "id": 1
  }'
```

---

## 💡 How It Works

### **Dependency Analysis**
Parallel EVM analyzes each transaction to determine:
- **Read Set**: What addresses/storage it reads from
- **Write Set**: What addresses/storage it writes to

### **Conflict Detection**
Transactions conflict if:
- Same sender (nonce dependency)
- Read-write conflict (one reads what another writes)
- Write-write conflict (both write to same address/storage)

### **Parallel Execution**
Independent transactions execute concurrently, providing massive speedup.

---

## 📊 Performance

### **When You Get Speedup**:
- ✅ **Independent Transactions**: 10-100x
  - Different senders
  - Different recipients
  - No shared state

- ✅ **Partial Dependencies**: 2-10x
  - Some transactions independent
  - Some have dependencies

### **When You Don't Get Speedup**:
- ❌ **Sequential Chain**: 1x (no improvement)
  - Each transaction depends on previous
  - Must execute in order

---

## 🎯 Use Cases

### **1. DeFi Trading**
Multiple swaps on different pairs execute in parallel:
```
Swap ETH/USDT + Swap BTC/ETH + Swap DAI/USDC
→ All execute in parallel (3x speedup)
```

### **2. NFT Marketplace**
Parallel mints and transfers:
```
Mint NFT #1 + Mint NFT #2 + Transfer NFT #3
→ All execute in parallel (3x speedup)
```

### **3. Gaming**
Concurrent player actions:
```
Player 1 move + Player 2 move + Player 3 move
→ All execute in parallel (3x speedup)
```

### **4. DEX Aggregator**
Multiple swaps in one transaction:
```
Swap 1 + Swap 2 + Swap 3 + Swap 4
→ All execute in parallel (4x speedup)
```

---

## 🔧 Configuration

### **Max Parallel Transactions**
Default: 100 transactions per batch

```rust
executor.set_max_parallel(200); // Increase limit
```

### **Enable/Disable**
```rust
executor.set_enabled(true);  // Enable parallel execution
executor.set_enabled(false); // Disable (sequential only)
```

---

## 📈 Benchmarking

### **Run Benchmark**

```rust
use crate::evm::benchmark::ParallelEvmBenchmark;

let benchmark = ParallelEvmBenchmark::new();

// Generate 100 independent transactions
let transactions = benchmark.generate_test_transactions(100, true);

// Run benchmark
let results = benchmark.benchmark(transactions, executor_fn);

println!("Speedup: {:.2}x", results.speedup);
println!("Sequential: {:?}", results.sequential_time);
println!("Parallel: {:?}", results.parallel_time);
```

### **Expected Results**:
- **10 transactions**: 5-10x speedup
- **50 transactions**: 10-20x speedup
- **100 transactions**: 20-50x speedup

---

## ⚠️ Important Notes

### **State Consistency**
- All transactions validated before parallel execution
- State snapshots ensure safe parallel execution
- Rollback on any failure

### **Gas Costs**
- Gas costs remain the same per transaction
- No additional gas for parallel execution
- Batch optimization reduces overhead

### **Compatibility**
- Full EVM compatibility maintained
- Works with all existing smart contracts
- No changes to contract code required

---

## 🔍 Monitoring

### **Key Metrics**:
- **Parallel Execution Rate**: % of transactions executed in parallel
- **Average Speedup**: Average performance improvement
- **Batch Size**: Average transactions per parallel batch
- **Conflict Rate**: % of transactions with conflicts

### **RPC Methods**:
- `mds_getParallelEVMStats` - Get current statistics
- `mds_estimateParallelImprovement` - Estimate speedup for transactions

---

## 📝 Examples

### **Example 1: Enable and Check**

```javascript
// Enable parallel EVM
await rpc.call('mds_enableParallelEVM', { enabled: true });

// Check status
const stats = await rpc.call('mds_getParallelEVMStats');
console.log('Enabled:', stats.enabled);
console.log('Max Parallel:', stats.maxParallel);
```

### **Example 2: Estimate Improvement**

```javascript
const transactions = [
  { from: '0x...', to: '0x...', value: '0x1000' },
  { from: '0x...', to: '0x...', value: '0x2000' },
  { from: '0x...', to: '0x...', value: '0x3000' },
];

const estimate = await rpc.call('mds_estimateParallelImprovement', {
  transactions
});

console.log('Estimated Speedup:', estimate.estimatedImprovement + 'x');
```

---

## 🎓 Best Practices

### **1. Design Independent Transactions**
- Use different senders when possible
- Avoid shared state when not needed
- Batch independent operations

### **2. Monitor Performance**
- Track parallel execution rate
- Measure actual speedup
- Adjust max_parallel if needed

### **3. Handle Errors**
- Always validate before parallel execution
- Rollback on any failure
- Use state snapshots for safety

---

## 🚀 Advanced Usage

### **Custom Executor**
```rust
let executor = ParallelEvmExecutor::new();
executor.set_max_parallel(200);
executor.set_enabled(true);
```

### **State Snapshots**
```rust
use crate::evm::state_snapshot::StateSnapshotManager;

let mut manager = StateSnapshotManager::new();
manager.create_base_snapshot(&balances, &nonces, block_number, timestamp);
let branch = manager.create_branch_snapshot(0)?;
```

---

## 📚 Additional Resources

- **Implementation Plan**: `PARALLEL_EVM_IMPLEMENTATION_PLAN.md`
- **Status**: `PARALLEL_EVM_STATUS.md`
- **Integration Guide**: `PARALLEL_EVM_INTEGRATION_GUIDE.md`
- **API Reference**: `mondoshawan-blockchain/src/evm/parallel.rs`

---

**Last Updated**: January 2026  
**Status**: Ready for use
