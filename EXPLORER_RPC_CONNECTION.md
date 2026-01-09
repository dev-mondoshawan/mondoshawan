# Block Explorer - Live Testnet Connection Guide

**Status**: Ready for Implementation  
**Last Updated**: January 2026

---

## Overview

The Mondoshawan block explorer can be connected to a live testnet node using JSON-RPC calls directly. The explorer currently uses a mix of REST API calls (which don't exist) and JSON-RPC calls. This guide shows how to update it to use only JSON-RPC.

---

## Current Setup

**Current Configuration:**
```javascript
const API_BASE = 'http://localhost:8081/api';  // REST API (doesn't exist)
const RPC_BASE = 'http://localhost:8545';      // JSON-RPC (works)
```

**Problem:**
- Explorer tries to use REST API endpoints that don't exist
- Some functions already use JSON-RPC correctly
- Need to convert all REST API calls to JSON-RPC

---

## Solution: Direct JSON-RPC Connection

### Configuration

**Option 1: Local Testnet Node**
```javascript
const RPC_BASE = 'http://localhost:8545';
```

**Option 2: Remote Testnet Node**
```javascript
const RPC_BASE = 'http://testnet.mondoshawan.io:8545';
// or
const RPC_BASE = 'http://192.168.1.100:8545';  // Your testnet node IP
```

**Option 3: Configurable (Recommended)**
```javascript
// Get from URL parameter or localStorage
const urlParams = new URLSearchParams(window.location.search);
const RPC_BASE = urlParams.get('rpc') || 
                 localStorage.getItem('rpc_endpoint') || 
                 'http://localhost:8545';
```

---

## Available JSON-RPC Methods

### Network Statistics
- `eth_blockNumber` - Get latest block number
- `mds_getDagStats` - Get DAG statistics (blocks, transactions, etc.)
- `mds_getTps` - Get transactions per second
- `net_peerCount` - Get connected peer count
- `mds_getNodeStatus` - Get aggregated node status

### Blocks
- `eth_getBlockByNumber` - Get block by number (with transactions)
- `eth_getBlockByHash` - Get block by hash
- `eth_getBlockTransactionCountByNumber` - Get transaction count

### Transactions
- `eth_getTransactionByHash` - Get transaction details
- `mds_getTransactionRisk` - Get transaction risk score

### Addresses
- `eth_getBalance` - Get address balance
- `eth_getTransactionCount` - Get address nonce
- `mds_getShardForAddress` - Get shard for address
- `mds_getRiskScore` - Get address risk score
- `mds_getAddressSummary` - Get address summary

### Sharding
- `mds_getShardStats` - Get shard statistics
- `mds_getCrossShardTransaction` - Get cross-shard transaction info
- `mds_getCrossShardTransactions` - Get all cross-shard transactions

### MEV & Fairness
- `mds_getMevMetrics` - Get MEV metrics
- `mds_getBlockFairness` - Get block fairness
- `mds_getOrderingPolicy` - Get current ordering policy
- `mds_setOrderingPolicy` - Set ordering policy

### Forensics
- `mds_traceFunds` - Trace fund flows
- `mds_getAddressSummary` - Get address summary
- `mds_detectAnomalies` - Detect anomalies

---

## Implementation: Converting REST to JSON-RPC

### Helper Function for JSON-RPC Calls

```javascript
// Generic JSON-RPC call helper
async function rpcCall(method, params = []) {
    const response = await fetch(RPC_BASE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            jsonrpc: '2.0',
            method: method,
            params: params,
            id: 1
        })
    });
    
    const data = await response.json();
    if (data.error) {
        throw new Error(data.error.message || 'RPC error');
    }
    
    return data.result;
}
```

### Dashboard Statistics (Replace REST API)

**Current (REST API - doesn't work):**
```javascript
const [networkStats, chainStats] = await Promise.all([
    fetch(`${API_BASE}/stats/network`).then(r => r.json()),
    fetch(`${API_BASE}/stats/chain`).then(r => r.json())
]);
```

**New (JSON-RPC):**
```javascript
async function loadDashboard() {
    try {
        // Get all stats in parallel
        const [blockNumber, dagStats, tps, peerCount, nodeStatus] = await Promise.all([
            rpcCall('eth_blockNumber'),
            rpcCall('mds_getDagStats'),
            rpcCall('mds_getTps', [60]),  // 60-second window
            rpcCall('net_peerCount'),
            rpcCall('mds_getNodeStatus')
        ]);
        
        // Parse hex values
        const latestBlock = parseInt(blockNumber, 16);
        const peers = parseInt(peerCount, 16);
        const tpsValue = parseFloat(tps) || 0;
        
        // Update UI
        document.getElementById('total-blocks').textContent = latestBlock;
        document.getElementById('total-transactions').textContent = dagStats.total_transactions || 0;
        document.getElementById('peers-connected').textContent = peers;
        document.getElementById('tps').textContent = tpsValue.toFixed(2);
        
        // Update timestamp
        const now = new Date();
        document.getElementById('update-time').textContent = `Updated: ${now.toLocaleTimeString()}`;
    } catch (error) {
        console.error('Error loading dashboard:', error);
        // Show error state
    }
}
```

### Recent Blocks (Replace REST API)

**Current (REST API - doesn't work):**
```javascript
const response = await fetch(`${API_BASE}/blocks/recent?limit=10`);
const blocks = await response.json();
```

**New (JSON-RPC):**
```javascript
async function loadRecentBlocks() {
    try {
        // Get latest block number
        const blockNumberHex = await rpcCall('eth_blockNumber');
        const latestBlock = parseInt(blockNumberHex, 16);
        
        // Get last 10 blocks
        const blockPromises = [];
        const limit = 10;
        for (let i = 0; i < limit && (latestBlock - i) >= 0; i++) {
            const blockNum = latestBlock - i;
            blockPromises.push(
                rpcCall('eth_getBlockByNumber', [`0x${blockNum.toString(16)}`, true])
            );
        }
        
        const blocks = await Promise.all(blockPromises);
        
        // Filter out null blocks and format
        const blocksList = document.getElementById('blocks-list');
        const validBlocks = blocks.filter(b => b !== null);
        
        if (validBlocks.length === 0) {
            blocksList.innerHTML = '<p class="loading">No blocks found</p>';
            return;
        }
        
        blocksList.innerHTML = validBlocks.map(block => {
            const blockNum = parseInt(block.number, 16);
            const timestamp = parseInt(block.timestamp, 16);
            const txCount = block.transactions ? block.transactions.length : 0;
            
            return `
                <div class="block-item">
                    <h3>Block #${blockNum}</h3>
                    <p><strong>Hash:</strong> <code>${block.hash}</code></p>
                    <p><strong>Timestamp:</strong> ${new Date(timestamp * 1000).toLocaleString()}</p>
                    <p><strong>Transactions:</strong> ${txCount}</p>
                </div>
            `;
        }).join('');
    } catch (error) {
        console.error('Error loading blocks:', error);
        document.getElementById('blocks-list').innerHTML = '<p class="error">Error loading blocks</p>';
    }
}
```

### Recent Transactions (Replace REST API)

**Current (REST API - doesn't work):**
```javascript
const response = await fetch(`${API_BASE}/transactions/recent?limit=10`);
const transactions = await response.json();
```

**New (JSON-RPC):**
```javascript
async function loadRecentTransactions() {
    try {
        // Get latest block with transactions
        const blockNumberHex = await rpcCall('eth_blockNumber');
        const latestBlock = parseInt(blockNumberHex, 16);
        
        // Get transactions from recent blocks
        const transactions = [];
        const maxBlocks = 5;  // Check last 5 blocks
        const limit = 10;
        
        for (let i = 0; i < maxBlocks && transactions.length < limit && (latestBlock - i) >= 0; i++) {
            const blockNum = latestBlock - i;
            const block = await rpcCall('eth_getBlockByNumber', [`0x${blockNum.toString(16)}`, true]);
            
            if (block && block.transactions) {
                for (const tx of block.transactions) {
                    if (transactions.length >= limit) break;
                    transactions.push({
                        ...tx,
                        block_number: blockNum,
                        block_hash: block.hash
                    });
                }
            }
        }
        
        // Display transactions
        const transactionsList = document.getElementById('transactions-list');
        
        if (transactions.length === 0) {
            transactionsList.innerHTML = '<p class="loading">No transactions found</p>';
            return;
        }
        
        // Render transactions (same as before, but with real data)
        transactionsList.innerHTML = transactions.map(tx => {
            const value = parseInt(tx.value || '0x0', 16) / 1e18;
            return `
                <div class="transaction-item" data-tx-hash="${tx.hash}">
                    <h3>Transaction</h3>
                    <p><strong>Hash:</strong> <code>${tx.hash}</code></p>
                    <p><strong>From:</strong> <code>${tx.from}</code></p>
                    <p><strong>To:</strong> <code>${tx.to || 'Contract Creation'}</code></p>
                    <p><strong>Value:</strong> ${value.toFixed(6)} MSHW</p>
                    <p><strong>Block:</strong> #${tx.block_number}</p>
                    <div class="risk-indicator" data-tx="${tx.hash}">
                        <span class="risk-loading">Loading risk analysis...</span>
                    </div>
                </div>
            `;
        }).join('');
        
        // Load risk scores asynchronously
        transactions.forEach(async (tx) => {
            try {
                const risk = await getTransactionRisk(tx.hash);
                // Update risk indicator
            } catch (error) {
                // Handle error
            }
        });
    } catch (error) {
        console.error('Error loading transactions:', error);
        document.getElementById('transactions-list').innerHTML = '<p class="error">Error loading transactions</p>';
    }
}
```

---

## Complete Updated Explorer Code

The updated `app.js` will:
1. Remove all REST API calls (`API_BASE`)
2. Use only JSON-RPC calls (`RPC_BASE`)
3. Add configurable RPC endpoint
4. Handle errors gracefully
5. Show connection status

---

## Configuration Options

### Option 1: URL Parameter
```
http://localhost:3000/explorer?rpc=http://testnet.mondoshawan.io:8545
```

### Option 2: LocalStorage
```javascript
// Set RPC endpoint
localStorage.setItem('rpc_endpoint', 'http://testnet.mondoshawan.io:8545');

// Explorer will use it automatically
```

### Option 3: Configuration File
Create `config.js`:
```javascript
const EXPLORER_CONFIG = {
    rpc_endpoint: 'http://testnet.mondoshawan.io:8545',
    refresh_interval: 5000,  // 5 seconds
    blocks_per_page: 10
};
```

---

## Testing Connection

### Test RPC Connection
```javascript
async function testConnection() {
    try {
        const blockNumber = await rpcCall('eth_blockNumber');
        console.log('✅ Connected! Latest block:', parseInt(blockNumber, 16));
        return true;
    } catch (error) {
        console.error('❌ Connection failed:', error);
        return false;
    }
}
```

### Connection Status Indicator
Add to HTML:
```html
<div id="connection-status" class="connection-status">
    <span id="connection-indicator" class="indicator"></span>
    <span id="connection-text">Connecting...</span>
</div>
```

Update in JavaScript:
```javascript
async function updateConnectionStatus() {
    const isConnected = await testConnection();
    const indicator = document.getElementById('connection-indicator');
    const text = document.getElementById('connection-text');
    
    if (isConnected) {
        indicator.className = 'indicator connected';
        text.textContent = 'Connected';
    } else {
        indicator.className = 'indicator disconnected';
        text.textContent = 'Disconnected';
    }
}
```

---

## CORS Configuration

If connecting to a remote testnet node, you may need to configure CORS on the node:

**In node configuration:**
```toml
[rpc]
cors_origins = ["http://localhost:3000", "https://mondoshawan.io"]
```

Or allow all origins for testnet:
```toml
[rpc]
cors_origins = ["*"]
```

---

## Deployment Steps

1. **Update `app.js`** - Replace REST API calls with JSON-RPC
2. **Add RPC configuration** - Make endpoint configurable
3. **Test locally** - Connect to localhost:8545
4. **Test remotely** - Connect to testnet node
5. **Add connection status** - Show connection indicator
6. **Handle errors** - Graceful error handling
7. **Deploy** - Update explorer on website

---

## Example: Complete Updated Functions

See the updated `app.js` file for complete implementation.

---

**Next Steps:**
1. Update `mondoshawan-explorer-frontend/app.js` with JSON-RPC calls
2. Add configuration for testnet RPC endpoint
3. Test connection to live testnet node
4. Deploy updated explorer
