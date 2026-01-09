# Startup Issue - Fixed

## Problem

The node was failing to start with error:
```
Failed to start API server: Only one usage of each socket address (protocol/network address/port) is normally permitted. (os error 10048)
```

## Root Cause

**Port Conflict**: Both the P2P network and HTTP API server were trying to bind to port 8080:
- P2P Network: `config.port` (default: 8080)
- HTTP API Server: Hardcoded to `127.0.0.1:8080` in `node.rs`

## Solution

Changed HTTP API server to use port **8081** instead of 8080.

### Changes Made

1. **`mondoshawan-blockchain/src/bin/node.rs`**:
   - Changed HTTP API binding from `127.0.0.1:8080` → `127.0.0.1:8081`
   - Updated console messages to reflect new port

2. **`mondoshawan-explorer-frontend/app.js`**:
   - Updated API_BASE from `http://localhost:8080/api` → `http://localhost:8081/api`

## Port Configuration

| Service | Port | Purpose |
|---------|------|---------|
| P2P Network | 8080 | Peer-to-peer communication |
| HTTP API | 8081 | REST API for explorer frontend |
| JSON-RPC | 8545 | Ethereum-compatible RPC |

## Testing

After the fix, the node should start successfully:

```powershell
cd D:\Pyrax\mondoshawan-blockchain
cargo run --bin node
```

Expected output:
- ✅ P2P Network started on port 8080
- ✅ JSON-RPC API started on http://127.0.0.1:8545
- ✅ HTTP API server started on http://localhost:8081
- ✅ No port conflict errors

## Status

✅ **Fixed** - Node should now start without port conflicts.
