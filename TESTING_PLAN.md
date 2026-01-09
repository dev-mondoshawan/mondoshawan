# Testing & Validation Plan - Mondoshawan Blockchain

## Test Suite Execution

### Unit Tests
- Run: `cargo test`
- Location: `mondoshawan-blockchain/tests/`
- Coverage: Core blockchain, consensus, storage, etc.

### Integration Tests
- Run: `cargo test --test integration_test`
- Location: `mondoshawan-blockchain/tests/integration/`
- Coverage: End-to-end scenarios

## RPC Method Validation

### Test All `mds_*` Methods

**Core Blockchain Methods:**
- `mds_getDagStats` - GhostDAG statistics
- `mds_getBlueScore` - Blue score for block
- `mds_getTps` - Transactions per second

**Sharding Methods:**
- `mds_getShardStats` - Shard statistics
- `mds_getShardForAddress` - Address shard assignment
- `mds_getCrossShardTransaction` - Cross-shard tx details

**Security Methods:**
- `mds_getRiskScore` - Address risk score
- `mds_getRiskLabels` - Risk labels for address
- `mds_getTransactionRisk` - Transaction risk assessment

**Light Client Methods:**
- `mds_getStateRoot` - Current state root
- `mds_getStateProof` - State proof for address
- `mds_verifyStateProof` - Verify state proof

**PQ Account Methods:**
- `mds_generatePqAccount` - Generate PQ account
- `mds_getPqAccountType` - Check account type
- `mds_createPqTransaction` - Create PQ-signed tx

**Security Policy Methods:**
- `mds_addSecurityPolicy` - Add security policy
- `mds_getSecurityPolicies` - List policies
- `mds_evaluateTransactionPolicy` - Evaluate policy

**Forensics Methods:**
- `mds_traceFunds` - Trace fund flow
- `mds_getAddressSummary` - Address summary
- `mds_detectAnomalies` - Anomaly detection

## Performance Testing

### Load Testing
- High transaction volume
- Multiple concurrent requests
- Block generation under load
- Network stress testing

### Benchmarking
- Transaction throughput (TPS)
- Block processing time
- RPC response times
- Memory usage
- CPU usage

## Security Testing

### Attack Vectors
- Invalid transaction handling
- Double-spend attempts
- Malformed blocks
- Signature validation
- Replay attacks

### PQ Security
- PQ signature verification
- PQ account creation
- PQ transaction handling

## Test Scripts

### RPC Test Script
```powershell
# Test all mds_* methods
$methods = @(
    "mds_getDagStats",
    "mds_getTps",
    "mds_getShardStats",
    "mds_getRiskScore",
    # ... all methods
)

foreach ($method in $methods) {
    $body = @{ jsonrpc = "2.0"; method = $method; params = @(); id = 1 } | ConvertTo-Json
    Invoke-RestMethod -Uri "http://localhost:8545" -Method Post -Body $body -ContentType "application/json"
}
```

## Expected Results

### Test Suite
- All unit tests pass
- All integration tests pass
- No panics or crashes
- No memory leaks

### RPC Methods
- All methods respond correctly
- Proper error handling
- Valid JSON responses
- Correct data types

### Performance
- TPS meets targets
- Response times < 100ms
- No resource exhaustion
- Stable under load

## Status Tracking

- [ ] Unit tests run
- [ ] Integration tests run
- [ ] RPC methods validated
- [ ] Performance benchmarks
- [ ] Security tests passed
