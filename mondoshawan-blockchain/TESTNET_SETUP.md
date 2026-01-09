# Mondoshawan 3-Node Local Testnet Setup

**Purpose**: Run a local 3-node testnet for multi-node testing and validation  
**Status**: Alpha - Functional with known issues  
**Date**: January 2026

---

## Quick Start

```powershell
# Start 3-node testnet
powershell -ExecutionPolicy Bypass -File d:\Pyrax\start-testnet.ps1

# Test connectivity
powershell -ExecutionPolicy Bypass -File d:\Pyrax\test-testnet.ps1

# Stop testnet
Get-Process node | Stop-Process -Force
```

---

## Architecture

### Node Configuration

| Node | P2P Port | RPC Port | Role | Connects To |
|------|----------|----------|------|-------------|
| Node 1 | 8080 | 8545 | Bootstrap | - |
| Node 2 | 8081 | 8546 | Peer | 127.0.0.1:8080 |
| Node 3 | 8082 | 8547 | Peer | 127.0.0.1:8080 |

### Network Topology

```
        Node 1 (Bootstrap)
        127.0.0.1:8080
              ↑  ↑
             /    \
            /      \
    Node 2          Node 3
    :8081           :8082
```

---

## Prerequisites

### System Requirements
- **OS**: Windows 10/11 (PowerShell 5.1+)
- **Rust**: 1.70+ with cargo
- **RAM**: 2GB minimum (500MB per node)
- **Disk**: 100MB minimum
- **Ports**: 8080-8082 (P2P), 8545-8547 (RPC)

### Build Requirements
```powershell
# Verify Rust installation
cargo --version

# Build release binary
cd d:\Pyrax\mondoshawan-blockchain
cargo build --release --bin node
```

---

## CLI Usage

### Node Binary Arguments

```bash
node [p2p_port] [rpc_port] [peer_addr1] [peer_addr2] ...
```

**Parameters**:
- `p2p_port` - P2P network port (default: 8080)
- `rpc_port` - JSON-RPC API port (default: 8545)
- `peer_addr` - Bootstrap peer addresses (format: IP:PORT)

**Examples**:

```powershell
# Bootstrap node (default ports)
.\target\release\node.exe 8080 8545

# Peer node connecting to bootstrap
.\target\release\node.exe 8081 8546 127.0.0.1:8080

# Peer with multiple bootstrap nodes
.\target\release\node.exe 8082 8547 127.0.0.1:8080 127.0.0.1:8081
```

---

## Automated Scripts

### start-testnet.ps1

**Purpose**: Automated 3-node testnet launcher

**What it does**:
1. Kills any existing node processes
2. Builds release binary
3. Starts Node 1 (bootstrap) on ports 8080/8545
4. Starts Node 2 (peer) on ports 8081/8546 → connects to Node 1
5. Starts Node 3 (peer) on ports 8082/8547 → connects to Node 1
6. Monitors node processes

**Usage**:
```powershell
powershell -ExecutionPolicy Bypass -File d:\Pyrax\start-testnet.ps1
```

**Monitoring**:
- Press `Ctrl+C` to stop monitoring (nodes continue running)
- Nodes run in separate windows for debugging

**Stop Testnet**:
```powershell
Get-Process node | Stop-Process -Force
```

---

### test-testnet.ps1

**Purpose**: Validate testnet connectivity and sync status

**Tests**:
1. RPC endpoint availability (all 3 nodes)
2. Block height per node
3. Peer count per node
4. Process statistics (CPU, memory)

**Usage**:
```powershell
powershell -ExecutionPolicy Bypass -File d:\Pyrax\test-testnet.ps1
```

**Expected Output**:
```
Testing Mondoshawan 3-Node Testnet...

Node 1 (http://127.0.0.1:8545):
  Block Height: 1234
  Peer Count: 2

Node 2 (http://127.0.0.1:8546):
  Block Height: 1234
  Peer Count: 2

Node 3 (http://127.0.0.1:8547):
  Block Height: 1234
  Peer Count: 2

Testnet Status:
  Running nodes: 3
    PID 12345: CPU 5.23s, Memory 45.67 MB
    PID 12346: CPU 5.18s, Memory 45.32 MB
    PID 12347: CPU 5.21s, Memory 45.54 MB
```

---

## Manual Testing

### Test RPC Connectivity

**Using PowerShell**:
```powershell
$body = '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
Invoke-RestMethod -Uri "http://127.0.0.1:8545" -Method Post -Body $body -ContentType "application/json"
```

**Using curl**:
```powershell
curl.exe http://127.0.0.1:8545 -H "Content-Type: application/json" -d '{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}'
```

### Test Peer Connectivity

```powershell
# Check peer count on Node 1
$body = '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}'
Invoke-RestMethod -Uri "http://127.0.0.1:8545" -Method Post -Body $body -ContentType "application/json"
```

### Verify Block Sync

```powershell
# Compare block heights across all nodes
foreach ($port in 8545,8546,8547) {
    $body = '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
    $result = Invoke-RestMethod -Uri "http://127.0.0.1:$port" -Method Post -Body $body -ContentType "application/json"
    Write-Host "Node on port $port: Block $([Convert]::ToInt32($result.result, 16))"
}
```

---

## Known Issues

### Issue 1: Data Directory Conflicts

**Problem**: All nodes use the same `data/` directory, causing conflicts

**Symptoms**:
- Node crashes on startup
- Database lock errors
- Inconsistent state

**Workaround**: Use unique data directories per node

```powershell
# Start nodes with unique data dirs (requires code modification)
# TODO: Add --data-dir CLI argument
```

**Fix**: Update NodeConfig to accept data_dir from CLI

---

### Issue 2: RPC JSON Parsing Errors

**Problem**: RPC server returns "Parse error" for valid JSON

**Symptoms**:
```json
{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}
```

**Investigation needed**:
- Content-Type header handling
- JSON deserialization in RPC server
- Body size limits

**Workaround**: Use direct blockchain queries via node console

---

### Issue 3: Node 2 Stability

**Problem**: Node 2 crashes during multi-node startup

**Possible causes**:
- Port already in use
- Data directory conflicts
- Memory issues
- Race condition in network manager

**Debug steps**:
```powershell
# Check port availability
netstat -ano | findstr "8081"
netstat -ano | findstr "8546"

# Run Node 2 manually to see errors
cd d:\Pyrax\mondoshawan-blockchain
.\target\release\node.exe 8081 8546 127.0.0.1:8080
```

---

## Testing Checklist

### Pre-Launch Tests
- [ ] Ports 8080-8082 available
- [ ] Ports 8545-8547 available
- [ ] No existing node processes running
- [ ] Release binary built successfully
- [ ] Sufficient disk space (100MB+)
- [ ] Sufficient RAM (2GB+)

### Post-Launch Tests
- [ ] All 3 nodes started successfully
- [ ] All 3 node processes running
- [ ] All 3 RPC endpoints responding
- [ ] Peer count = 2 on all nodes
- [ ] Block heights syncing across nodes
- [ ] Mining active on all streams
- [ ] No crash/restart loops

### Functional Tests
- [ ] Block propagation (mine on Node 1, verify on Node 2/3)
- [ ] Transaction propagation (submit to Node 1, verify on Node 2/3)
- [ ] Fork resolution (simulate network partition)
- [ ] Node restart recovery (kill Node 2, restart, verify sync)
- [ ] High-load test (submit 1000+ transactions)

---

## Troubleshooting

### Nodes Won't Start

**Check**:
```powershell
# Port conflicts
netstat -ano | findstr "8080"
netstat -ano | findstr "8081"
netstat -ano | findstr "8082"

# Existing processes
Get-Process node -ErrorAction SilentlyContinue

# Build errors
cargo build --release --bin node
```

**Solution**:
```powershell
# Kill conflicting processes
Get-Process node | Stop-Process -Force

# Rebuild
cargo clean
cargo build --release --bin node
```

---

### RPC Not Responding

**Check**:
```powershell
# Test TCP connection
Test-NetConnection -ComputerName 127.0.0.1 -Port 8545

# Check node logs (in node window)
# Look for "JSON-RPC API started on..." message
```

**Solution**:
- Verify node started successfully
- Check firewall settings
- Try different RPC method (eth_chainId instead of eth_blockNumber)

---

### Nodes Not Syncing

**Check**:
```powershell
# Compare block heights
.\test-testnet.ps1

# Check peer connectivity in node logs
# Look for "Connected to peer" or "Failed to connect" messages
```

**Solution**:
- Verify bootstrap node (Node 1) is running first
- Check network_manager logs for connection errors
- Restart nodes in order: Node 1 → Node 2 → Node 3

---

## Performance Baselines

### Expected Metrics (3-Node Testnet)

| Metric | Expected Value | Notes |
|--------|---------------|-------|
| **Blocks/sec** | 9-10 | Combined all streams |
| **TPS** | 0-100 | Depends on transaction load |
| **CPU per node** | 5-15% | Idle to moderate mining |
| **RAM per node** | 30-60 MB | First hour of operation |
| **Disk growth** | ~1MB/hour | Empty blocks |
| **Peer connections** | 2 per node | Bootstrap topology |
| **Block sync time** | <5 seconds | Between nodes |

### Resource Usage Over Time

**1 hour run**:
- Total blocks: ~36,000 (10 blocks/sec × 3600 sec)
- Total disk: ~4 MB
- Peak RAM: ~70 MB per node
- CPU: Stable 10-15%

**24 hour run** (projected):
- Total blocks: ~864,000
- Total disk: ~100 MB
- Peak RAM: ~100 MB per node
- CPU: Should remain stable

---

## Network Testing Scenarios

### Scenario 1: Normal Operation

**Goal**: Verify basic multi-node functionality

**Steps**:
1. Start 3-node testnet
2. Wait 30 seconds for stabilization
3. Verify all nodes at same block height (±5 blocks)
4. Submit transaction to Node 1
5. Verify transaction appears on Node 2/3 within 10 seconds

**Success criteria**:
- All nodes running
- Block heights synced
- Transaction propagated

---

### Scenario 2: Node Restart

**Goal**: Verify crash recovery and re-sync

**Steps**:
1. Start 3-node testnet
2. Let run for 60 seconds
3. Kill Node 2 process
4. Wait 30 seconds
5. Restart Node 2
6. Verify Node 2 catches up to Node 1/3

**Success criteria**:
- Node 2 restarts successfully
- Node 2 syncs to current height
- No data loss

---

### Scenario 3: Network Partition

**Goal**: Verify fork resolution

**Steps**:
1. Start 3-node testnet
2. Block network between Node 1 and Node 2/3 (firewall rule)
3. Let run for 60 seconds (parallel chains)
4. Remove network block
5. Verify GhostDAG resolves fork correctly

**Success criteria**:
- Nodes detect partition
- Both partitions continue mining
- Fork resolves via blue score
- All nodes converge to same chain

---

### Scenario 4: High Load

**Goal**: Verify performance under stress

**Steps**:
1. Start 3-node testnet
2. Submit 1000 transactions to Node 1
3. Monitor mining performance
4. Verify all transactions included in blocks
5. Verify transactions propagated to Node 2/3

**Success criteria**:
- All transactions processed
- Block times remain stable
- No crashes or deadlocks
- Memory usage stable

---

## Development Notes

### Code Changes Required

1. **Add --data-dir CLI argument**
   - Location: `src/bin/node.rs`
   - Parse from args[4+]
   - Pass to NodeConfig

2. **Fix RPC JSON parsing**
   - Location: `src/rpc.rs`
   - Debug Content-Type handling
   - Check body deserialization

3. **Add network metrics**
   - Location: `src/network.rs`
   - Track peer connections
   - Log connection events
   - Expose via RPC

4. **Improve error handling**
   - All modules: Better error messages
   - Log node startup sequence
   - Catch and report failures

---

## Future Enhancements

### Phase 1: Stability
- [ ] Unique data directories per node
- [ ] Fix RPC parsing issues
- [ ] Improve network error handling
- [ ] Add comprehensive logging

### Phase 2: Observability
- [ ] Per-node Prometheus metrics
- [ ] Grafana dashboard for multi-node view
- [ ] Network topology visualization
- [ ] Block propagation timing

### Phase 3: Advanced Testing
- [ ] Automated chaos testing (random node kills)
- [ ] Network latency simulation
- [ ] Byzantine fault injection
- [ ] Performance benchmarking suite

### Phase 4: Public Testnet
- [ ] Deploy to cloud infrastructure
- [ ] Public RPC endpoints
- [ ] Faucet for test tokens
- [ ] Explorer integration
- [ ] Community node support

---

## References

- Main Node Implementation: `src/bin/node.rs`
- Node Configuration: `src/node/mod.rs`
- Network Manager: `src/network.rs`
- RPC Server: `src/rpc.rs`
- Production Readiness Plan: `PRODUCTION_READINESS_PLAN.md`
- Testnet Roadmap: `TESTNET_ROADMAP.md`

---

**Last Updated**: January 7, 2026  
**Status**: Alpha - Proof of concept successful, stability work needed
