# Mondoshawan Testnet Setup Guide

**Status**: Ready for Testnet Deployment  
**Version**: 1.0  
**Date**: January 2026

---

## 🎯 Testnet Overview

The Mondoshawan testnet allows developers and researchers to:
- Test all blockchain features in a safe environment
- Experiment with TriStream mining
- Deploy and test smart contracts
- Explore AI-driven security features
- Participate in network consensus

**Testnet Tokens**: Unlimited (for testing purposes)  
**Network**: Public testnet  
**Status**: Research/Development Phase

---

## 📋 Prerequisites

### Required Software
- **Rust** 1.75+ (for building the node)
- **Docker** (optional, for containerized deployment)
- **Git** (for cloning the repository)

### System Requirements
- **CPU**: 4+ cores recommended
- **RAM**: 8GB minimum, 16GB recommended
- **Storage**: 100GB+ SSD
- **Network**: Stable internet connection

---

## 🚀 Quick Start (Single Node)

### 1. Clone and Build

```bash
git clone https://github.com/dev-mondoshawan/mondoshawan.git
cd mondoshawan/mondoshawan-blockchain
cargo build --release
```

### 2. Run Testnet Node

```bash
# Default configuration (port 8080, RPC 8545)
./target/release/node

# Custom port
./target/release/node 8081

# Connect to existing testnet
./target/release/node 8081 127.0.0.1:8080
```

### 3. Access Services

- **RPC API**: `http://localhost:8545`
- **HTTP API**: `http://localhost:8081`
- **Explorer**: Open `mondoshawan-explorer-frontend/index.html` in browser

---

## 🌐 Multi-Node Testnet Setup

### Option 1: Local Multi-Node (Testing)

Run multiple nodes on the same machine:

**Terminal 1 - Bootstrap Node:**
```bash
./target/release/node 8080
```

**Terminal 2 - Node 2:**
```bash
./target/release/node 8081 127.0.0.1:8080
```

**Terminal 3 - Node 3:**
```bash
./target/release/node 8082 127.0.0.1:8080 127.0.0.1:8081
```

### Option 2: Docker Deployment

Create `docker-compose.testnet.yml`:

```yaml
version: '3.8'

services:
  node1:
    build: ./mondoshawan-blockchain
    ports:
      - "8080:8080"
      - "8545:8545"
    volumes:
      - node1-data:/data
    command: ["8080"]
    
  node2:
    build: ./mondoshawan-blockchain
    ports:
      - "8081:8080"
      - "8546:8545"
    volumes:
      - node2-data:/data
    command: ["8080", "node1:8080"]
    depends_on:
      - node1

volumes:
  node1-data:
  node2-data:
```

Run:
```bash
docker-compose -f docker-compose.testnet.yml up
```

---

## ⚙️ Testnet Configuration

### Create Testnet Config File

Create `testnet.toml`:

```toml
[network]
port = 8080
rpc_port = 8545
max_peers = 50
bootstrap_peers = [
    "127.0.0.1:8080",  # Bootstrap node 1
    "127.0.0.1:8081",  # Bootstrap node 2
]

[mining]
miner_address = "0x0101010101010101010101010101010101010101"
enable_stream_a = true
enable_stream_b = true
enable_stream_c = true

[blockchain]
chain_id = 0x4D534857  # "MSHW" in hex (testnet)
data_dir = "./testnet-data"

[features]
enable_sharding = true
shard_count = 10
enable_verkle = false  # Optional for testnet
enable_evm = true

[api]
rate_limit = 100  # requests per second
```

### Load Config

Modify `node.rs` to load from file:
```rust
let config = if let Ok(cfg) = NodeConfig::from_file("testnet.toml") {
    cfg
} else {
    NodeConfig::default()
};
```

---

## 🔗 Network Bootstrapping

### Step 1: Start Bootstrap Nodes

Start 3-5 bootstrap nodes first (these are the initial network nodes):

```bash
# Bootstrap Node 1 (public IP: YOUR_IP)
./target/release/node 8080

# Bootstrap Node 2
./target/release/node 8081 YOUR_IP:8080

# Bootstrap Node 3
./target/release/node 8082 YOUR_IP:8080 YOUR_IP:8081
```

### Step 2: Share Bootstrap Addresses

Publish bootstrap node addresses:
- `YOUR_IP:8080`
- `YOUR_IP:8081`
- `YOUR_IP:8082`

### Step 3: New Nodes Connect

New nodes connect to bootstrap nodes:
```bash
./target/release/node 8083 YOUR_IP:8080 YOUR_IP:8081
```

---

## 📊 Monitoring Testnet

### Enable Metrics

1. Start Prometheus & Grafana:
```bash
cd grafana
docker-compose up -d
```

2. Access Dashboards:
- Grafana: `http://localhost:3001` (admin/admin)
- Prometheus: `http://localhost:9090`

### Monitor Network Health

- **Block Production**: Check all 3 streams are mining
- **Network Peers**: Verify nodes are connected
- **TPS**: Monitor transaction throughput
- **Block Propagation**: Check blocks are syncing

---

## 🧪 Testing on Testnet

### 1. Send Test Transaction

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "eth_sendTransaction",
    "params": [{
      "from": "0x0101010101010101010101010101010101010101",
      "to": "0x0202020202020202020202020202020202020202",
      "value": "0x1000000000000000000"
    }],
    "id": 1
  }'
```

### 2. Check Balance

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "eth_getBalance",
    "params": ["0x0101010101010101010101010101010101010101", "latest"],
    "id": 1
  }'
```

### 3. Test Mining

Mining starts automatically. Check logs:
```
✅ Stream A: Mined block #1 with 50 txs, reward: 50 MSHW
✅ Stream B: Mined block #1 with 25 txs, reward: 25 MSHW
```

---

## 🔧 Troubleshooting

### Node Won't Start

**Port Already in Use:**
```bash
# Use different port
./target/release/node 8081
```

**Database Locked:**
```bash
# Remove lock file
rm -rf testnet-data/.lock
```

### Can't Connect to Peers

**Firewall Issues:**
- Open ports 8080-8090 (P2P) and 8545-8555 (RPC)
- Check firewall rules

**Network Issues:**
- Verify bootstrap nodes are running
- Check IP addresses are correct
- Test connectivity: `telnet BOOTSTRAP_IP 8080`

### Blocks Not Syncing

**Check Logs:**
- Verify nodes are connected
- Check for consensus errors
- Verify blockchain state is valid

---

## 📝 Testnet Best Practices

1. **Start Small**: Begin with 2-3 nodes locally
2. **Monitor Closely**: Watch logs and metrics
3. **Test Features**: Try all RPC methods
4. **Report Issues**: Document any problems
5. **Backup Data**: Save blockchain state regularly

---

## 🎯 Next Steps

### For Testnet Operators

1. ✅ Set up bootstrap nodes
2. ✅ Configure public endpoints
3. ✅ Set up monitoring
4. ✅ Document network status
5. ✅ Share connection info

### For Testnet Participants

1. ✅ Build and run node
2. ✅ Connect to bootstrap nodes
3. ✅ Start mining
4. ✅ Test transactions
5. ✅ Explore features

---

## 📞 Support

- **GitHub Issues**: https://github.com/dev-mondoshawan/mondoshawan/issues
- **Documentation**: See `DEVELOPER_GUIDE.md`
- **Explorer**: Use web explorer to monitor network

---

## 🔐 Security Notes

⚠️ **Testnet Only**: This is a test network. Do not use real funds.

- Testnet tokens have no value
- All data may be reset
- No guarantees on uptime
- Use for testing only

---

**Ready to launch your testnet node!** 🚀
