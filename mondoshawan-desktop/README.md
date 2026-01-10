# Mondoshawan Desktop

**All-in-One Blockchain Experience** — Node control, wallet, mining, explorer, and metrics in one desktop application.

## Features

✅ **Node Dashboard** — Monitor block height, transactions, peers, and mining status in real-time

✅ **Integrated Wallet** — Create wallets, check balances, and send transactions with Ed25519 signing

✅ **One-Click Mining** — Start/stop TriStream mining (ASIC, CPU/GPU, ZK proofs) with a single button

✅ **Live Explorer** — Browse recent blocks, view DAG statistics (blue/red blocks), and monitor network performance

✅ **Performance Metrics** — Real-time TPS tracking, DAG consensus metrics, and per-shard statistics

✅ **Address Book** — Save and manage frequently used addresses with names and notes

✅ **Multi-Account Management** — Track multiple accounts and switch between them easily

✅ **Transaction History** — View transaction history for any address with detailed information

✅ **Account Abstraction** — Create and manage smart contract wallets (multi-sig, social recovery, spending limits)

✅ **Parallel EVM** — Enable/disable parallel execution and view performance statistics

✅ **Time-Locked Transactions** — Schedule transactions to execute at a future block or timestamp

✅ **Gasless Transactions** — Send transactions with a sponsor paying the fees

✅ **Reputation System** — View on-chain reputation scores and factors for any address

✅ **Native Desktop** — Built with Tauri (Rust + React) for Windows, macOS, and Linux

---

## Quick Start

### Prerequisites

1. **Rust** (for building Tauri backend)
2. **Node.js** (for React frontend)
3. **Mondoshawan Node** running on `127.0.0.1:8545`

### Installation

```bash
npm install
```

### Run Development Mode

**Terminal 1 — Start the Mondoshawan node:**
```bash
cd /path/to/mondoshawan-blockchain
cargo run --bin node
```

Wait for: `RPC server listening on 127.0.0.1:8545`

**Terminal 2 — Start the desktop app:**
```bash
cd /path/to/mondoshawan-desktop
npm run tauri dev
```

The desktop window will open automatically.

---

## Usage

### Dashboard Tab
- View node status: height, transaction count, peer count, mining state
- **Start/Stop Mining** with one click
- View TriStream details: block times, max txs, and rewards for all three streams

### Wallet Tab
- Enter any address (0x...) to view balance and nonce
- Balance shown in both raw hex and human-readable MSHW format

### Send Tab
- **Create New Wallet**: Generates a new Ed25519 key pair
- **Send Transaction**: Enter recipient, value (MSHW), and fee (MSHW)
- Transaction signed locally and submitted via `mds_sendRawTransaction`

### Explorer Tab
- View recent blocks with hash, timestamp, and transaction count
- DAG statistics: total blocks, blue/red blocks, avg txs per block
- Auto-refreshes every 10 seconds

### Metrics Tab
- Real-time TPS (60-second window)
- Network performance metrics
- Per-shard statistics (if sharding enabled)
- Cross-shard transaction flows

### History Tab
- View transaction history for any address
- Filter by address, transaction type, or date range
- Detailed transaction information (hash, from, to, value, fee, status)
- Export transaction history

### Address Book (Send Tab)
- **Add Contact**: Save frequently used addresses with names and notes
- **Remove Contact**: Delete contacts you no longer need
- **Quick Select**: Select from saved contacts when sending transactions
- Data persisted to `address_book.json` in app directory

### Multi-Account Management (Wallet Tab)
- **Add Account**: Track multiple wallet addresses with custom names
- **Remove Account**: Remove accounts you no longer need
- **Switch Accounts**: Quickly switch between tracked accounts
- **Account Overview**: View all accounts and their balances at a glance
- Data persisted to `accounts.json` in app directory

### Account Abstraction Tab (NEW!)
- **Create Smart Contract Wallets**: Basic, multi-sig, social recovery, spending limits, or combined
- **Wallet Management**: View all owned wallets, check details, and manage configurations
- **Multi-Signature Support**: Create wallets requiring n-of-m signatures for transactions
- **Social Recovery**: Set up guardian-based recovery for lost wallets
- **Spending Limits**: Configure daily spending limits for enhanced security

### Parallel EVM (Metrics Tab)
- **Enable/Disable**: Toggle parallel EVM execution for performance boost
- **Statistics**: View parallel execution rate, average speedup, and batch metrics
- **Performance Monitoring**: Track improvements from parallel execution

### Time-Locked & Gasless Transactions (Send Tab)
- **Time-Locked**: Schedule transactions to execute at a specific block number or timestamp
- **Gasless**: Send transactions with a sponsor address paying the fees
- **Combined Options**: Use both features together for advanced transaction scenarios

### Reputation Display (Wallet Tab)
- **Reputation Score**: View 0-100 reputation score with level (High/Medium/Low)
- **Detailed Factors**: See successful/failed transactions, blocks mined, account age, value transacted, and more
- **On-Chain Verification**: All reputation data comes directly from the blockchain

---

## Building for Production

### Windows
```bash
npm run tauri build
```
Output: `src-tauri/target/release/bundle/msi/Mondoshawan-Desktop_0.1.0_x64_en-US.msi`

### macOS
```bash
npm run tauri build
```
Output: `src-tauri/target/release/bundle/dmg/Mondoshawan-Desktop_0.1.0_x64.dmg`

### Linux
```bash
npm run tauri build
```
Output: `src-tauri/target/release/bundle/appimage/mondoshawan-desktop_0.1.0_amd64.AppImage`

---

## Security Notes

**Current Implementation (MVP):**
- Keys stored **in memory only** (lost when app closes)
- Address book and account data stored in JSON files (unencrypted)
- No encryption at rest
- No password protection

**For Production:**
- Implement encrypted keystore on disk
- Add password/biometric unlock
- Support hardware wallets
- Add multi-sig options

---

## Troubleshooting

### "Failed to fetch status"
- Ensure Mondoshawan node is running on `127.0.0.1:8545`
- Check node logs for RPC errors

### "No key loaded"
- Click "Create New Wallet" in the Send tab first

### "Invalid nonce"
- Wait for pending transactions to be mined
- Node's nonce doesn't match expected value

### "Insufficient balance"
- Wallet doesn't have enough MSHW + fee
- Use Wallet tab to check balance

---

## Tech Stack

- **Tauri 2.x** — Rust backend for native desktop
- **React 18** — Frontend UI framework
- **TypeScript** — Type-safe JavaScript
- **Vite** — Fast build tool
- **Ed25519-dalek** — Transaction signing
- **Reqwest** — HTTP client for RPC calls

---

## License

MIT License — See LICENSE file for details

---

## Links

- **Website**: [mondoshawan.io](https://mondoshawan.io)
- **Whitepaper**: [Mondoshawan Whitepaper](https://mondoshawan.io/Mondoshawan_WHITEPAPER.html)
- **Explorer**: [Live Blockchain Explorer](https://mondoshawan.io/explorer/)
- **Main Repo**: [mondoshawan-blockchain](https://github.com/dev-mondoshawan/mondoshawan)

---

**Built and operational today — not "coming soon".**
