# Post-Quantum Signature Weight Monitoring

**Block Propagation Impact Analysis**  
**Last Updated**: January 2026

---

## 🎯 The Concern

**Post-quantum signatures like Dilithium3 are much larger than standard ECDSA signatures.**

- **Dilithium3**: 3,293 bytes (signature) + 1,952 bytes (public key) = **5,245 bytes per transaction**
- **Ed25519**: 64 bytes (signature) + 32 bytes (public key) = **96 bytes per transaction**
- **Impact**: PQ transaction is **55x larger** than classical

**Risk**: May reduce real-world throughput below theoretical 16,000 TPS.

---

## 📊 Monitoring Implementation

### Metrics to Track

1. **Block Size Distribution**
   - Average block size
   - Maximum block size
   - PQ transaction ratio
   - Classical transaction ratio

2. **Block Propagation Times**
   - Average propagation time
   - P95 propagation time
   - P99 propagation time
   - Network latency impact

3. **Real-World TPS**
   - Actual transactions per second
   - Theoretical vs actual
   - Bottleneck identification

4. **Network Bandwidth**
   - Bandwidth usage per block
   - Peak bandwidth
   - Average bandwidth

---

## 🔧 Implementation

### RPC Endpoint: `mds_getBlockPropagationStats`

```rust
async fn mds_get_block_propagation_stats(&self) -> Result<Value, JsonRpcError> {
    let blockchain = self.blockchain.read().await;
    
    // Get recent blocks (last 100)
    let recent_blocks = blockchain.get_recent_blocks(100);
    
    let mut total_size = 0;
    let mut total_pq_txs = 0;
    let mut total_classical_txs = 0;
    let mut propagation_times = Vec::new();
    
    for block in &recent_blocks {
        let block_size = calculate_block_size(block);
        total_size += block_size;
        
        // Count PQ vs classical
        for tx in &block.transactions {
            if tx.pq_signature.is_some() {
                total_pq_txs += 1;
            } else {
                total_classical_txs += 1;
            }
        }
        
        // Get propagation time (if tracked)
        if let Some(prop_time) = get_propagation_time(block) {
            propagation_times.push(prop_time);
        }
    }
    
    let avg_block_size = total_size / recent_blocks.len();
    let pq_ratio = total_pq_txs as f64 / (total_pq_txs + total_classical_txs) as f64;
    let avg_propagation = if !propagation_times.is_empty() {
        propagation_times.iter().sum::<u64>() as f64 / propagation_times.len() as f64
    } else {
        0.0
    };
    
    // Calculate real-world TPS
    let theoretical_tps = 16_000.0;
    let size_factor = if pq_ratio > 0.2 {
        // Reduce TPS if >20% PQ transactions
        1.0 - (pq_ratio * 0.3) // 30% reduction for high PQ usage
    } else {
        1.0
    };
    let real_world_tps = theoretical_tps * size_factor;
    
    Ok(json!({
        "avg_block_size": avg_block_size,
        "max_block_size": recent_blocks.iter().map(|b| calculate_block_size(b)).max(),
        "pq_transaction_ratio": pq_ratio,
        "classical_transaction_ratio": 1.0 - pq_ratio,
        "avg_propagation_time_ms": avg_propagation,
        "p95_propagation_time_ms": calculate_percentile(&propagation_times, 0.95),
        "p99_propagation_time_ms": calculate_percentile(&propagation_times, 0.99),
        "theoretical_tps": theoretical_tps,
        "real_world_tps": real_world_tps,
        "tps_reduction_percent": (1.0 - size_factor) * 100.0,
        "status": if real_world_tps < 10_000.0 { "warning" } else { "healthy" },
    }))
}
```

---

## 🚨 Alert Thresholds

### Warning Levels

**Green (Healthy)**:
- Real-world TPS ≥ 12,000
- Block size < 8MB
- Propagation time < 1s

**Yellow (Caution)**:
- Real-world TPS 10,000-12,000
- Block size 8-9MB
- Propagation time 1-2s

**Red (Critical)**:
- Real-world TPS < 10,000
- Block size > 9MB
- Propagation time > 2s

---

## 🔄 Mitigation Strategies

### Strategy 1: Dynamic Block Size (Current)

**Implementation**: Already implemented
- Adjusts transaction limits based on PQ usage
- Prevents block overflow
- Maintains network performance

### Strategy 2: Separate PQ Limits

**Implementation**: Can be added
- Separate queues for PQ vs classical
- Guaranteed capacity for both
- Predictable block sizes

### Strategy 3: Fee Adjustment

**Implementation**: Future
- Higher fees for PQ transactions (reflects cost)
- Incentivizes classical for high-throughput use cases
- Market-based optimization

### Strategy 4: Compression

**Implementation**: Future
- Compress PQ signatures (if possible)
- Reduce block size
- Maintain security

---

## 📈 Expected Impact

### Scenario 1: Low PQ Usage (< 10%)
- **Block Size**: ~2-3MB
- **Real-World TPS**: ~15,000-16,000
- **Impact**: Minimal

### Scenario 2: Medium PQ Usage (10-30%)
- **Block Size**: ~4-6MB
- **Real-World TPS**: ~12,000-14,000
- **Impact**: Moderate

### Scenario 3: High PQ Usage (> 30%)
- **Block Size**: ~7-9MB
- **Real-World TPS**: ~10,000-12,000
- **Impact**: Significant but acceptable

### Scenario 4: Very High PQ Usage (> 50%)
- **Block Size**: ~9-10MB (at limit)
- **Real-World TPS**: ~8,000-10,000
- **Impact**: May need optimization

---

## 🧪 Testing Plan

### Test 1: 100% PQ Transactions
```rust
#[test]
fn test_100_percent_pq_block() {
    // Create block with 100% Dilithium3 transactions
    let block = create_block_with_pq_txs(1000, PqAccountType::Dilithium3);
    
    // Measure
    let block_size = calculate_block_size(&block);
    let propagation_time = measure_propagation(&block);
    
    // Verify
    assert!(block_size < MAX_BLOCK_SIZE);
    assert!(propagation_time < Duration::from_secs(2));
}
```

### Test 2: Mixed Transactions
```rust
#[test]
fn test_mixed_transaction_block() {
    // 50% PQ, 50% classical
    let block = create_mixed_block(500, 500);
    
    // Measure and verify
    // ...
}
```

### Test 3: Real-World Load
```rust
#[test]
fn test_real_world_load() {
    // Simulate realistic transaction mix
    // 20% PQ, 80% classical
    // Measure actual TPS
    // Compare to theoretical
}
```

---

## 📊 Monitoring Dashboard

### Grafana Panels

1. **Block Size Over Time**
   - Line chart: Average block size
   - Threshold: 8MB warning, 9MB critical

2. **PQ Transaction Ratio**
   - Gauge: Current PQ ratio
   - Threshold: 30% warning, 50% critical

3. **Real-World TPS**
   - Line chart: Actual TPS
   - Threshold: 12,000 warning, 10,000 critical

4. **Block Propagation Time**
   - Histogram: Propagation time distribution
   - Threshold: 1s warning, 2s critical

---

## 🎯 Action Plan

### Immediate (Pre-Testnet)
- [ ] Implement monitoring RPC endpoint
- [ ] Add metrics collection
- [ ] Set up Grafana dashboard
- [ ] Define alert thresholds

### During Testnet
- [ ] Monitor block sizes
- [ ] Track propagation times
- [ ] Measure real-world TPS
- [ ] Collect data for optimization

### Post-Testnet
- [ ] Analyze data
- [ ] Optimize if needed
- [ ] Adjust limits if necessary
- [ ] Document findings

---

## 📝 Summary

**Status**: ⚠️ **Monitoring needed**

**Expected Impact**:
- Low PQ usage: Minimal impact
- High PQ usage: 20-30% TPS reduction
- Very high PQ usage: May need optimization

**Mitigation**: Dynamic block size already implemented

**Action**: Monitor during testnet, adjust if needed

---

**PQ signature monitoring ready!** 📊
