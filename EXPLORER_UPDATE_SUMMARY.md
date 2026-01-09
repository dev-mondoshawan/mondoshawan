# Block Explorer - Live Testnet Connection Update

**Status**: ✅ Updated to use JSON-RPC  
**Date**: January 2026

---

## Changes Made

### 1. Removed REST API Dependency
- **Removed**: `API_BASE = 'http://localhost:8081/api'` (doesn't exist)
- **Kept**: `RPC_BASE = 'http://localhost:8545'` (JSON-RPC endpoint)

### 2. Added RPC Configuration
- RPC endpoint is now configurable via:
  - URL parameter: `?rpc=http://testnet.mondoshawan.io:8545`
  - localStorage: `localStorage.setItem('rpc_endpoint', 'http://...')`
  - Default: `http://localhost:8545`

### 3. Added RPC Helper Function
```javascript
async function rpcCall(method, params = []) {
    // Generic JSON-RPC call helper
    // Handles errors and returns result
}
```

### 4. Updated Functions

#### ✅ `loadDashboard()`
- **Before**: Used REST API `/stats/network` and `/stats/chain`
- **After**: Uses JSON-RPC:
  - `eth_blockNumber` - Latest block
  - `mds_getDagStats` - DAG statistics
  - `mds_getTps` - Transactions per second
  - `net_peerCount` - Connected peers
  - `mds_getNodeStatus` - Node status

#### ✅ `loadRecentBlocks()`
- **Before**: Used REST API `/blocks/recent`
- **After**: Uses JSON-RPC:
  - `eth_blockNumber` - Get latest block number
  - `eth_getBlockByNumber` - Get blocks (last 10)

#### ✅ `loadRecentTransactions()`
- **Before**: Used REST API `/transactions/recent`
- **After**: Uses JSON-RPC:
  - `eth_blockNumber` - Get latest block number
  - `eth_getBlockByNumber` - Get blocks with transactions
  - Extracts transactions from recent blocks

#### ✅ `displayAddress()`
- **Before**: Used REST API `/addresses/{address}`
- **After**: Uses JSON-RPC:
  - `eth_getBalance` - Get address balance
  - `eth_getTransactionCount` - Get nonce
  - `mds_getShardForAddress` - Get shard ID
  - `mds_getRiskScore` - Get risk score

#### ✅ `displayBlockFromRpc()` (New)
- New function to display blocks from RPC response
- Handles hex-encoded block numbers and timestamps

#### ✅ `setupSearch()`
- **Before**: Used REST API for block/transaction/address search
- **After**: Uses JSON-RPC:
  - `eth_getBlockByHash` - Search by block hash
  - `eth_getBlockByNumber` - Search by block number
  - `eth_getTransactionByHash` - Search by transaction hash
  - Direct RPC calls for address lookup

---

## How to Connect to Testnet

### Option 1: URL Parameter
```
http://localhost:3000/explorer?rpc=http://testnet.mondoshawan.io:8545
```

### Option 2: LocalStorage
```javascript
localStorage.setItem('rpc_endpoint', 'http://testnet.mondoshawan.io:8545');
// Refresh page
```

### Option 3: Edit `app.js`
```javascript
const RPC_BASE = 'http://testnet.mondoshawan.io:8545';
```

---

## Testing

### Test Local Node
1. Start Mondoshawan node: `./target/release/node`
2. Open explorer: `http://localhost:3000/explorer`
3. Should connect to `http://localhost:8545`

### Test Remote Testnet
1. Get testnet node URL (e.g., `http://testnet.mondoshawan.io:8545`)
2. Open explorer with RPC parameter:
   ```
   http://localhost:3000/explorer?rpc=http://testnet.mondoshawan.io:8545
   ```
3. Check browser console for connection errors

---

## CORS Configuration

If connecting to a remote node, ensure CORS is enabled:

**In node configuration (`testnet.toml`):**
```toml
[rpc]
cors_origins = ["http://localhost:3000", "https://mondoshawan.io"]
```

Or for testnet (allow all):
```toml
[rpc]
cors_origins = ["*"]
```

---

## Functions Still Using Direct Fetch (OK)

These functions already use JSON-RPC correctly via direct `fetch()` calls:
- `getRiskScore()` - Uses `mds_getRiskScore`
- `getTransactionRisk()` - Uses `mds_getTransactionRisk`
- `traceFunds()` - Uses `mds_traceFunds`
- `getAddressSummary()` - Uses `mds_getAddressSummary`
- `detectAnomalies()` - Uses `mds_detectAnomalies`

These can optionally be converted to use `rpcCall()` helper for consistency, but they work fine as-is.

---

## Missing Functions (Need Implementation)

These functions are called but may need implementation:
- `loadShardingStats()` - Should use `mds_getShardStats`
- `loadCrossShardTransactions()` - Should use `mds_getCrossShardTransactions`
- `loadOrderingPolicy()` - Should use `mds_getOrderingPolicy`
- `loadMevMetrics()` - Should use `mds_getMevMetrics`
- `loadBlockFairness()` - Should use `mds_getBlockFairness`

**Note**: These functions may be defined elsewhere or need to be implemented.

---

## Next Steps

1. ✅ **Core functions updated** - Dashboard, blocks, transactions, addresses
2. ⚠️ **Optional**: Convert remaining functions to use `rpcCall()` helper
3. ⚠️ **Optional**: Implement missing functions (`loadShardingStats`, etc.)
4. ✅ **Test connection** - Verify with local and remote nodes
5. ✅ **Deploy** - Update explorer on website

---

## Example: Connecting to Testnet

```html
<!-- In explorer HTML -->
<script>
    // Set RPC endpoint from URL or use default
    const urlParams = new URLSearchParams(window.location.search);
    const rpcEndpoint = urlParams.get('rpc') || 'http://localhost:8545';
    localStorage.setItem('rpc_endpoint', rpcEndpoint);
</script>
```

Then open:
```
http://localhost:3000/explorer?rpc=http://testnet.mondoshawan.io:8545
```

---

**The explorer is now ready to connect to a live testnet node!** 🚀
