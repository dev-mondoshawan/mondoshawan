# Multi-Node Testnet Deployment - Cloud Platforms

**Status**: ✅ Yes, nodes can communicate!  
**Date**: January 2026

---

## 🎯 Overview

**Yes, testnet nodes can communicate with each other once deployed on cloud platforms!** However, there are some important considerations:

### How Nodes Communicate

Mondoshawan nodes use **TCP P2P connections** for:
- Block propagation
- Transaction broadcasting
- Chain synchronization
- Peer discovery

**Connection Flow:**
```
Node A → TCP Connect → Node B (Port 8080)
Node B → Accept Connection → Bidirectional Communication
```

---

## 🔧 Challenges with Cloud Deployment

### 1. **Public IP Addresses**
- Cloud platforms assign public URLs (e.g., `your-app.railway.app`)
- Nodes need to know each other's **public addresses**
- DNS resolution must work

### 2. **Port Configuration**
- Cloud platforms may use different ports internally
- Need to map internal ports to external ports
- P2P port (8080) must be accessible

### 3. **Firewall Rules**
- Cloud platforms may block incoming connections
- Need to configure firewall to allow P2P traffic
- Some platforms require explicit port exposure

### 4. **Service Discovery**
- Nodes need bootstrap peer addresses
- Can't use `localhost` or `127.0.0.1`
- Must use public URLs or IPs

---

## 🚀 Solutions by Platform

### Option 1: Railway.app (Recommended)

**Advantages:**
- ✅ Exposes ports automatically
- ✅ Provides public URL
- ✅ Easy to configure

**Configuration:**

1. **Deploy First Node (Bootstrap Node 1)**
   - Deploy to Railway
   - Get public URL: `https://node1.railway.app`
   - Note the **internal port** (usually 8080)

2. **Configure Node 1**
   ```bash
   # Environment variables in Railway dashboard:
   P2P_PORT=8080
   RPC_PORT=8545
   LISTEN_ADDR=0.0.0.0:8080
   ```

3. **Deploy Second Node (Bootstrap Node 2)**
   - Deploy to Railway
   - Get public URL: `https://node2.railway.app`

4. **Configure Node 2 to Connect to Node 1**
   ```bash
   # Environment variables:
   P2P_PORT=8080
   RPC_PORT=8545
   BOOTSTRAP_PEERS=node1.railway.app:8080
   ```

5. **Update Node 1 to Connect to Node 2**
   - Add environment variable:
   ```bash
   BOOTSTRAP_PEERS=node2.railway.app:8080
   ```

**Important**: Railway uses **internal service names** for service-to-service communication. You may need to:
- Use Railway's **private networking** (if available)
- Or use **public URLs** with proper DNS

---

### Option 2: Render.com

**Configuration:**

1. **Deploy Bootstrap Node 1**
   - Service URL: `https://node1.onrender.com`
   - **Issue**: Render.com free tier may not expose custom ports
   - **Solution**: Use Render's built-in port (10000) or upgrade to paid

2. **Configure Ports**
   ```bash
   # Render sets PORT env var automatically
   # Your node should listen on PORT env var
   ```

3. **For P2P Communication:**
   - Render.com may require **paid plan** for custom ports
   - Or use **reverse proxy** to map ports
   - Or use **WebSocket** over HTTPS (port 443)

**Workaround**: Use Render's internal service discovery (if available) or upgrade to paid plan.

---

### Option 3: Fly.io (Best for Multi-Node)

**Advantages:**
- ✅ **Excellent for multi-node** deployments
- ✅ **Private networking** between services
- ✅ **Public IPs** available
- ✅ **Port exposure** is straightforward

**Configuration:**

1. **Deploy Node 1**
   ```bash
   # Create fly.toml
   fly launch
   
   # Configure ports
   [[services]]
     internal_port = 8080
     protocol = "tcp"
   
   [[services.ports]]
     port = 8080
     handlers = ["tcp"]
   ```

2. **Get Node 1's Public IP**
   ```bash
   fly ips list
   # Returns: 123.45.67.89
   ```

3. **Deploy Node 2**
   ```bash
   fly launch
   
   # Set bootstrap peer
   fly secrets set BOOTSTRAP_PEERS=123.45.67.89:8080
   ```

4. **Private Networking (Recommended)**
   ```bash
   # Create private network
   fly networks create testnet
   
   # Deploy nodes to private network
   fly deploy --network testnet
   
   # Nodes can communicate via private IPs
   ```

---

### Option 4: DigitalOcean App Platform

**Configuration:**

1. **Deploy Node 1**
   - Get public URL: `https://node1.ondigitalocean.app`
   - Configure port mapping in `app.yaml`

2. **Deploy Node 2**
   - Set environment variable:
   ```yaml
   envs:
     - key: BOOTSTRAP_PEERS
       value: node1.ondigitalocean.app:8080
   ```

---

## 📝 Step-by-Step: Multi-Node Deployment

### Prerequisites

1. **3 Bootstrap Nodes** (minimum for testnet)
2. **Public URLs** for each node
3. **Port Configuration** (P2P port 8080 must be accessible)

### Step 1: Deploy Bootstrap Node 1

**On Railway.app:**

1. Create new project: "mondoshawan-node1"
2. Deploy from GitHub
3. Set environment variables:
   ```
   P2P_PORT=8080
   RPC_PORT=8545
   LISTEN_ADDR=0.0.0.0:8080
   ```
4. Deploy → Get URL: `https://node1.railway.app`
5. **Note the internal port** (check Railway logs)

### Step 2: Deploy Bootstrap Node 2

1. Create new project: "mondoshawan-node2"
2. Deploy from GitHub
3. Set environment variables:
   ```
   P2P_PORT=8080
   RPC_PORT=8545
   LISTEN_ADDR=0.0.0.0:8080
   BOOTSTRAP_PEERS=node1.railway.app:8080
   ```
4. Deploy → Get URL: `https://node2.railway.app`

### Step 3: Update Node 1 to Connect to Node 2

**In Node 1's Railway dashboard:**

1. Add environment variable:
   ```
   BOOTSTRAP_PEERS=node2.railway.app:8080
   ```
2. Redeploy Node 1

### Step 4: Deploy Bootstrap Node 3

1. Create new project: "mondoshawan-node3"
2. Set environment variables:
   ```
   P2P_PORT=8080
   RPC_PORT=8545
   LISTEN_ADDR=0.0.0.0:8080
   BOOTSTRAP_PEERS=node1.railway.app:8080,node2.railway.app:8080
   ```
3. Deploy

### Step 5: Verify Connections

**Test from Node 1:**
```bash
curl -X POST https://node1.railway.app:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "net_peerCount",
    "params": [],
    "id": 1
  }'
```

**Expected Response:**
```json
{
  "jsonrpc": "2.0",
  "result": "0x2",  // 2 peers connected
  "id": 1
}
```

---

## 🔧 Code Changes Needed

### Update Node to Read Bootstrap Peers from Environment

**In `src/bin/node.rs` or `src/node/mod.rs`:**

```rust
// Read bootstrap peers from environment
let bootstrap_peers = std::env::var("BOOTSTRAP_PEERS")
    .unwrap_or_default()
    .split(',')
    .filter_map(|s| {
        let s = s.trim();
        if s.is_empty() {
            None
        } else {
            // Handle both IP:port and hostname:port
            s.parse::<SocketAddr>().ok()
                .or_else(|| {
                    // Try DNS resolution
                    // Note: This is simplified - you may need async DNS
                    format!("{}:8080", s).parse().ok()
                })
        }
    })
    .collect::<Vec<_>>();

// Connect to bootstrap peers
for peer_addr in bootstrap_peers {
    println!("🔗 Connecting to bootstrap peer: {}", peer_addr);
    if let Err(e) = node.connect_peer(peer_addr).await {
        eprintln!("⚠️  Failed to connect to {}: {}", peer_addr, e);
    }
}
```

### Update Network Manager for DNS Resolution

**In `src/network.rs`:**

```rust
use tokio::net::lookup_host;

pub async fn connect_peer_dns(&self, host: &str, port: u16) -> Result<()> {
    // Resolve DNS to IP addresses
    let addr = format!("{}:{}", host, port);
    let mut addrs = lookup_host(&addr).await?;
    
    // Try each resolved address
    while let Some(addr) = addrs.next() {
        match self.connect_peer(addr).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                eprintln!("Failed to connect to {}: {}", addr, e);
                continue;
            }
        }
    }
    
    Err(BlockchainError::Network("Failed to connect to any resolved address".to_string()))
}
```

---

## 🌐 Platform-Specific Solutions

### Railway.app: Service-to-Service Communication

**Option A: Use Public URLs**
```bash
# Node 2 connects to Node 1 via public URL
BOOTSTRAP_PEERS=node1.railway.app:8080
```

**Option B: Use Railway's Private Networking** (if available)
```bash
# Use service name for internal communication
BOOTSTRAP_PEERS=node1:8080
```

### Fly.io: Private Networking (Recommended)

```bash
# Create private network
fly networks create mondoshawan-testnet

# Deploy nodes to private network
fly deploy --network mondoshawan-testnet

# Nodes communicate via private IPs
# No need for public URLs for P2P
```

### Render.com: Port Mapping

**Issue**: Render.com may not expose custom ports on free tier.

**Solution 1**: Upgrade to paid plan
**Solution 2**: Use reverse proxy (nginx) to map ports
**Solution 3**: Use WebSocket over HTTPS (port 443)

---

## 🔍 Testing Multi-Node Communication

### Test 1: Check Peer Count

**From Node 1:**
```bash
curl -X POST https://node1.railway.app:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"net_peerCount","params":[],"id":1}'
```

**Expected**: `"result": "0x2"` (2 peers)

### Test 2: Check Block Synchronization

**Mine a block on Node 1**, then check Node 2:
```bash
curl -X POST https://node2.railway.app:8545 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}'
```

**Expected**: Both nodes should have similar block numbers (within 1-2 blocks)

### Test 3: Send Transaction

**Send transaction to Node 1:**
```bash
curl -X POST https://node1.railway.app:8545 \
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

**Check Node 2** - transaction should appear in mempool

---

## 🐛 Troubleshooting

### Nodes Can't Connect

**Problem**: `net_peerCount` returns `0x0`

**Solutions:**
1. **Check firewall rules** - Ensure port 8080 is open
2. **Verify DNS resolution** - Test: `nslookup node1.railway.app`
3. **Check logs** - Look for connection errors
4. **Verify port mapping** - Ensure P2P port is exposed
5. **Test connectivity** - `telnet node1.railway.app 8080`

### Blocks Not Propagating

**Problem**: Nodes mine blocks but others don't see them

**Solutions:**
1. **Verify peer connections** - Check `net_peerCount`
2. **Check broadcast logic** - Ensure `broadcast_block()` is called
3. **Verify network topology** - Full mesh is recommended
4. **Check logs** - Look for broadcast errors

### DNS Resolution Issues

**Problem**: Can't resolve hostnames

**Solutions:**
1. **Use IP addresses** instead of hostnames (temporary)
2. **Implement async DNS** in Rust code
3. **Use platform's service discovery** (if available)
4. **Configure DNS servers** in container

---

## 📊 Recommended Architecture

### For 3-Node Testnet:

```
┌─────────────────┐
│  Bootstrap 1    │
│  (Railway.app)  │
│  node1.railway  │
└────────┬────────┘
         │
    ┌────┴────┐
    │        │
┌───▼───┐ ┌──▼────┐
│Node 2 │ │Node 3 │
│Railway│ │Fly.io │
└───────┘ └───────┘

Full Mesh: All nodes connect to all others
```

### Configuration:

**Node 1:**
```
BOOTSTRAP_PEERS=node2.railway.app:8080,node3.fly.dev:8080
```

**Node 2:**
```
BOOTSTRAP_PEERS=node1.railway.app:8080,node3.fly.dev:8080
```

**Node 3:**
```
BOOTSTRAP_PEERS=node1.railway.app:8080,node2.railway.app:8080
```

---

## ✅ Checklist for Multi-Node Deployment

- [ ] Deploy Bootstrap Node 1
- [ ] Get public URL/IP for Node 1
- [ ] Deploy Bootstrap Node 2
- [ ] Configure Node 2 to connect to Node 1
- [ ] Update Node 1 to connect to Node 2
- [ ] Deploy Bootstrap Node 3
- [ ] Configure all nodes for full mesh
- [ ] Test peer connections (`net_peerCount`)
- [ ] Test block propagation
- [ ] Test transaction broadcasting
- [ ] Monitor logs for errors
- [ ] Verify all nodes stay in sync

---

## 🎯 Summary

**Yes, nodes can communicate!** Here's what you need:

1. **Public addresses** for each node (URLs or IPs)
2. **Port configuration** (P2P port 8080 must be accessible)
3. **Bootstrap peer addresses** (set via environment variables)
4. **DNS resolution** (or use IP addresses)
5. **Firewall rules** (allow incoming connections on port 8080)

**Best Platform for Multi-Node:**
- **Fly.io** - Best private networking
- **Railway.app** - Easy setup, good for small testnets
- **DigitalOcean** - Reliable, good for production testnets

**Next Steps:**
1. Deploy first bootstrap node
2. Get its public address
3. Deploy second node with bootstrap peer address
4. Test connections
5. Deploy additional nodes

---

**Your nodes will communicate once properly configured!** 🚀
