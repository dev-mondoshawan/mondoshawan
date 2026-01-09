# Node Longevity Integration Testing Plan

**Testing Strategy for Governance System**  
**Last Updated**: January 2026

---

## 🎯 Testing Objectives

1. **Verify Longevity Weight Calculation**: 30-day eligibility bar
2. **Test Automatic Reset**: 31-day offline reset mechanism
3. **Test SEP Trigger**: Security Emergency Proposal automatic generation
4. **End-to-End Governance**: Full workflow from registration to voting

---

## 📋 Test Suite 1: Longevity Weight Verification

### Test 1.1: 30-Day Eligibility Bar

**Objective**: Verify that nodes become eligible for governance voting after 30 days of activity.

**Setup**:
```rust
// Create test node
let node_identity = create_test_node();
let mut registry = NodeRegistry::new();
registry.register_node(node_identity.clone())?;

// Simulate 30 days of activity
for day in 0..30 {
    let participation = ParticipationType::BlockMined {
        stream: StreamType::StreamA,
        block_hash: generate_test_hash(day),
    };
    registry.record_participation(&node_identity, participation);
    
    // Advance time by 1 day
    advance_time(86400); // 1 day in seconds
}
```

**Verification**:
```bash
# Via RPC
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "mds_getNodeLongevity",
    "params": ["0x<public_key_hex>"],
    "id": 1
  }'
```

**Expected Results**:
- `active_days`: 30
- `blocks_mined`: 30
- `uptime_index`: ≥ 0.8
- `longevity_weight`: > 0.0 (eligible)

**Success Criteria**: ✅ Weight > 0 after 30 days

---

### Test 1.2: Pre-30-Day Ineligibility

**Objective**: Verify nodes with < 30 days are ineligible.

**Setup**: Same as above, but stop at day 29.

**Expected Results**:
- `active_days`: 29
- `longevity_weight`: 0.0 (ineligible)

**Success Criteria**: ✅ Weight = 0 before 30 days

---

## 📋 Test Suite 2: Automatic Reset Mechanism

### Test 2.1: 31-Day Offline Reset

**Objective**: Verify longevity resets after 31 consecutive days offline.

**Setup**:
```rust
// Create node with 30 days of activity
let node_identity = create_test_node();
let mut registry = NodeRegistry::new();
registry.register_node(node_identity.clone())?;

// 30 days of activity
for day in 0..30 {
    record_participation(&mut registry, &node_identity, day);
    advance_time(86400);
}

// Verify eligible
assert!(get_longevity_weight(&registry, &node_identity) > 0.0);

// Go offline for 31 days
for day in 0..31 {
    registry.get_node_stats_mut(&node_identity)
        .unwrap()
        .record_no_activity();
    advance_time(86400);
}
```

**Verification**:
```bash
# Check longevity after 31 days offline
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "mds_getNodeLongevity",
    "params": ["0x<public_key_hex>"],
    "id": 1
  }'
```

**Expected Results**:
- `active_days`: 0 (reset)
- `uptime_index`: 0.0 (reset)
- `consecutive_offline_days`: 31
- `longevity_weight`: 0.0 (reset)

**Success Criteria**: ✅ All longevity metrics reset to 0

---

### Test 2.2: Partial Offline (No Reset)

**Objective**: Verify nodes offline < 30 days don't reset.

**Setup**: Same as above, but stop at 29 days offline.

**Expected Results**:
- `active_days`: 30 (not reset)
- `uptime_index`: > 0.0 (penalized but not reset)
- `consecutive_offline_days`: 29
- `longevity_weight`: > 0.0 (still eligible)

**Success Criteria**: ✅ No reset if offline < 30 days

---

## 📋 Test Suite 3: SEP Trigger Testing

### Test 3.1: HHI Threshold Trigger (90 Days)

**Objective**: Verify SEP is automatically generated when HHI > 0.25 for 90 consecutive days.

**Setup**:
```rust
// Simulate high centralization
let mut hashrate_distribution = HashrateDistribution::new();

// Create scenario where top 3 miners control > 50% for 90 days
for day in 0..90 {
    // Simulate mining activity with high centralization
    simulate_mining_day(&mut hashrate_distribution, high_centralization = true);
    
    // Check HHI
    let hhi = hashrate_distribution.calculate_hhi();
    assert!(hhi > 0.25, "HHI should exceed threshold");
    
    // Advance time
    advance_time(86400);
}

// Verify SEP was automatically generated
let sep = get_security_emergency_proposal();
assert!(sep.is_some(), "SEP should be automatically generated");
```

**Verification**:
```bash
# Check for active SEP
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "mds_getActiveProposals",
    "params": [],
    "id": 1
  }'
```

**Expected Results**:
- SEP automatically generated
- Proposal visible in registry
- Broadcast to all nodes
- Cannot be suppressed

**Success Criteria**: ✅ SEP generated automatically after 90 days

---

### Test 3.2: Top 3 Dominance Trigger (30 Days)

**Objective**: Verify SEP triggers when top 3 miners > 50% for 30 days.

**Setup**: Similar to above, but check top 3 share instead of HHI.

**Expected Results**:
- SEP generated after 30 days
- Faster trigger than HHI threshold

**Success Criteria**: ✅ SEP generated after 30 days of dominance

---

## 📋 Test Suite 4: End-to-End Governance

### Test 4.1: Full Governance Workflow

**Objective**: Test complete workflow from registration to voting.

**Steps**:
1. Register node
2. Mine blocks for 30+ days
3. Verify eligibility
4. Trigger SEP (manual or automatic)
5. Participate in deliberation
6. Cast vote
7. Verify vote weight calculation

**Success Criteria**: ✅ All steps complete successfully

---

## 🧪 Test Implementation Scripts

### PowerShell Test Script

```powershell
# test-longevity-integration.ps1

Write-Host "Starting Node Longevity Integration Tests..." -ForegroundColor Cyan

# Test 1: Register node
Write-Host "`n[Test 1] Registering node..." -ForegroundColor Yellow
$registerResult = Invoke-RestMethod -Uri "http://localhost:8545" -Method POST -ContentType "application/json" -Body @{
    jsonrpc = "2.0"
    method = "mds_registerNode"
    params = @()
    id = 1
} | ConvertTo-Json

Write-Host "Node registered: $($registerResult.result.success)" -ForegroundColor Green

# Test 2: Check initial longevity (should be 0)
Write-Host "`n[Test 2] Checking initial longevity..." -ForegroundColor Yellow
$longevity = Invoke-RestMethod -Uri "http://localhost:8545" -Method POST -ContentType "application/json" -Body @{
    jsonrpc = "2.0"
    method = "mds_getNodeLongevity"
    params = @($registerResult.result.public_key)
    id = 2
} | ConvertTo-Json

Write-Host "Active days: $($longevity.result.active_days)" -ForegroundColor White
Write-Host "Longevity weight: $($longevity.result.longevity_weight)" -ForegroundColor White

# Test 3: Simulate mining (would need actual mining or test blocks)
Write-Host "`n[Test 3] Simulating block mining..." -ForegroundColor Yellow
# This would require actual mining or test block creation

Write-Host "`n✅ Integration tests complete!" -ForegroundColor Green
```

---

## 🔧 Manual Testing Guide

### 31-Day Testnet Run

**Setup**:
1. Start testnet node
2. Register node via RPC
3. Enable mining
4. Let run for 31 days

**Daily Checks**:
- Day 1-29: Verify `active_days` increments
- Day 30: Verify `longevity_weight` > 0
- Day 31: Verify eligibility maintained

**Commands**:
```bash
# Daily check script
#!/bin/bash
for day in {1..31}; do
    echo "Day $day:"
    curl -X POST http://localhost:8545 \
      -H "Content-Type: application/json" \
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"mds_getNodeLongevity\",\"params\":[\"0x<public_key>\"],\"id\":1}" \
      | jq '.result'
    sleep 86400  # Wait 1 day (or use time acceleration in test)
done
```

---

### Reset Test

**Setup**:
1. Node with 30+ days activity
2. Stop node
3. Wait 31 days
4. Restart and check

**Verification**:
```bash
# After 31 days offline
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "mds_getNodeLongevity",
    "params": ["0x<public_key>"],
    "id": 1
  }' | jq '.result | {active_days, uptime_index, longevity_weight}'
```

**Expected**: All values should be 0

---

## ⚠️ Current Implementation Status

### ✅ Implemented
- Node registration
- Participation tracking
- Longevity calculation
- Activity snapshots
- Reset mechanism
- RPC endpoints

### ⚠️ Partially Implemented
- SEP trigger (needs full implementation)
- Daily snapshot automation (manual for now)
- Geographic clustering (advisory only)

### ❌ Not Yet Implemented
- Full zk-SNARK proofs (placeholder only)
- Automatic SEP broadcasting
- On-chain proposal storage
- Voting mechanism

---

## 🚀 Next Steps

1. **Run 31-Day Testnet**: Verify longevity calculation
2. **Test Reset Mechanism**: Verify 31-day offline reset
3. **Implement SEP Trigger**: Complete automatic proposal generation
4. **Full zk-SNARK**: Ready for implementation (see below)

---

## 🔒 zk-SNARK Integration Readiness

**Current Status**: ✅ Infrastructure ready, placeholder implemented

**What's Needed**:
1. Choose zk-SNARK library (e.g., `arkworks`, `bellman`, `circom`)
2. Implement arithmetic circuit for uniqueness proof
3. Generate proving/verification keys
4. Replace placeholder with actual proof generation

**Estimated Effort**: 2-4 weeks for full implementation

**Recommendation**: 
- ✅ **Ready to proceed** if zk-SNARK is priority
- ⚠️ **Can defer** - current system works with IP-based uniqueness
- 💡 **Hybrid approach**: Use IP-based for now, add ZK for privacy-focused users

---

**Testing plan ready!** 🧪
