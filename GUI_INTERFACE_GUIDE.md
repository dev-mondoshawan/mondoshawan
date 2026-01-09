# Mondoshawan Mining GUI Interface Guide

## 🎯 Two Ways to View Mining

### 1. **Real-Time Console Dashboard** (Built-in)
The node now includes a **live updating console dashboard** that shows:
- Total blocks mined
- Total transactions processed
- Miner balance (accumulated rewards)
- Mining stream statistics
- Updates every 2 seconds

### 2. **Web Dashboard** (Browser Interface)
A modern web interface for viewing blockchain data.

---

## 🖥️ Console Dashboard

### How to Use

1. **Start the node:**
   ```powershell
   cd D:\Mondoshawan\Mondoshawan-blockchain
   $env:LIB = "D:\Program Files\Microsoft Visual Studio\18\Community\VC\Tools\MSVC\14.50.35717\lib\x64;C:\Program Files (x86)\Windows Kits\10\Lib\10.0.26100.0\um\x64;$env:LIB"
   cargo run --bin node
   ```

2. **What You'll See:**
   ```
   ╔═══════════════════════════════════════════════════════════════════════════════╗
   ║                    Mondoshawan Blockchain - Mining Dashboard                       ║
   ╠═══════════════════════════════════════════════════════════════════════════════╣
   ║  📊 Network Stats                                                             ║
   ║     Total Blocks: 15                                                          ║
   ║     Total Transactions: 85                                                    ║
   ║     Miner Balance: 375 tokens                                                 ║
   ╠═══════════════════════════════════════════════════════════════════════════════╣
   ║  ⛏️  Mining Streams                                                            ║
   ║     Stream A (ASIC):     1 blocks | 50 tokens/block | 10s blocks             ║
   ║     Stream B (CPU/GPU): 10 blocks | 25 tokens/block | 1s blocks             ║
   ║     Stream C (ZK):     100 blocks | Fees only      | 100ms blocks            ║
   ╠═══════════════════════════════════════════════════════════════════════════════╣
   ║  🌐 API: http://localhost:8080/api/stats                                      ║
   ║  📊 Web Dashboard: Open Mondoshawan-explorer-frontend/index.html in browser          ║
   ╚═══════════════════════════════════════════════════════════════════════════════╝
   ```

3. **Features:**
   - **Auto-refreshes** every 2 seconds
   - **Real-time stats** - see blocks and transactions grow
   - **Miner balance** - watch your rewards accumulate
   - **Stream statistics** - see which streams are mining

---

## 🌐 Web Dashboard

### Setup

1. **Start the node** (must be running first):
   ```powershell
   cd D:\Mondoshawan\Mondoshawan-blockchain
   cargo run --bin node
   ```

2. **Open the web dashboard:**
   - Navigate to: `D:\Mondoshawan\Mondoshawan-explorer-frontend\`
   - Open `index.html` in your web browser
   - Or use a local web server:
     ```powershell
     cd D:\Mondoshawan\Mondoshawan-explorer-frontend
     python -m http.server 3000
     # Then open http://localhost:3000
     ```

3. **What You'll See:**
   - Network statistics (blocks, transactions, TPS)
   - Recent blocks list
   - Recent transactions
   - Address lookup
   - Search functionality

### API Endpoints

The node provides a simple HTTP API:

- **GET /api/stats** - Get network statistics
  ```json
  {
    "blocks": 15,
    "transactions": 85,
    "miner_balance": 375
  }
  ```

### Connecting the Frontend

The frontend (`Mondoshawan-explorer-frontend/app.js`) needs to be configured to connect to:
```
http://localhost:8080/api/stats
```

---

## 📊 What You Can See

### Console Dashboard Shows:
- ✅ **Total Blocks** - All blocks mined across all streams
- ✅ **Total Transactions** - All transactions processed
- ✅ **Miner Balance** - Your accumulated rewards in tokens
- ✅ **Stream Statistics** - Blocks mined per stream
- ✅ **Real-time Updates** - Refreshes every 2 seconds

### Web Dashboard Shows:
- ✅ **Network Dashboard** - Overall statistics
- ✅ **Recent Blocks** - Latest blocks with details
- ✅ **Recent Transactions** - Latest transactions
- ✅ **Address Lookup** - Search addresses
- ✅ **Auto-refresh** - Updates every 30 seconds

---

## 🎨 Visual Features

### Console Dashboard:
- **Box-drawing characters** for clean borders
- **Emoji icons** for easy identification
- **Color support** (if terminal supports it)
- **Auto-clearing** screen for smooth updates

### Web Dashboard:
- **Modern design** with cards and grids
- **Responsive layout** - works on mobile
- **Dark theme** (if configured)
- **Interactive elements** - click to explore

---

## 🔧 Troubleshooting

### Console Dashboard Not Updating
- Make sure your terminal supports ANSI escape codes
- Try running in PowerShell or Command Prompt
- Some terminals may not support screen clearing

### Web Dashboard Not Loading
1. **Check node is running:**
   ```powershell
   # Should see "HTTP API server started on http://localhost:8080"
   ```

2. **Check API is accessible:**
   ```powershell
   # Open browser to: http://localhost:8080/api/stats
   # Should see JSON response
   ```

3. **Check frontend path:**
   - Make sure `Mondoshawan-explorer-frontend/index.html` exists
   - Check `app.js` has correct API URL

### API Server Not Starting
- Port 8080 might be in use
- Check firewall settings
- Try a different port (edit node.rs)

---

## 🚀 Quick Start

### Option 1: Console Dashboard (Easiest)
```powershell
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo run --bin node
# Dashboard appears automatically!
```

### Option 2: Web Dashboard
```powershell
# Terminal 1: Start node
cd D:\Mondoshawan\Mondoshawan-blockchain
cargo run --bin node

# Terminal 2: Start web server (optional)
cd D:\Mondoshawan\Mondoshawan-explorer-frontend
python -m http.server 3000

# Browser: Open http://localhost:3000
```

---

## 📝 Notes

- **Console dashboard** updates every 2 seconds
- **Web dashboard** updates every 30 seconds (configurable)
- **Both** show the same data from the same node
- **Mining happens** regardless of which interface you use
- **Press Ctrl+C** to stop the node

---

## 🎯 Recommended Setup

For the best experience:

1. **Run node in one terminal** - see console dashboard
2. **Open web dashboard in browser** - for detailed exploration
3. **Watch both** - console for real-time, web for details

**You now have two ways to monitor your mining!** 🎉
