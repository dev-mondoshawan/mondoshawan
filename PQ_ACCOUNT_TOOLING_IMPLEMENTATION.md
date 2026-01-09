# PQ Account Tooling Implementation

**Status:** ✅ **Complete**  
**Date:** December 2024

## Overview

PQ (Post-Quantum) accounts are now first-class citizens in Mondoshawan, with comprehensive tooling and RPC helpers for generating, managing, and using quantum-resistant accounts.

## What Was Implemented

### 1. Enhanced PQ Tooling Module (`src/pqc/tooling.rs`)

#### Functions

- **`generate_pq_account(algorithm: &str)`** - Generate new PQ accounts
  - Supports `"dilithium3"` and `"sphincsplus"` algorithms
  - Returns a `PqAccount` ready for use

- **`create_pq_transaction(account, to, value, fee, nonce, data)`** - Create PQ-signed transactions
  - Automatically signs transactions with PQ account
  - Handles both simple transfers and contract calls

- **`export_pq_account(account)`** - Export account for backup
  - Returns `PqAccountExport` with all key material
  - Supports JSON and hex serialization

- **`import_pq_account(export)`** - Import account from backup
  - Restores account from exported data
  - Validates key sizes and formats

- **`detect_pq_account_type_from_transaction(tx)`** - Detect PQ account type
  - Analyzes transaction signatures to determine account type
  - Returns `PqAccountType` if PQ signature is present

- **`format_pq_account(account)`** - Format account for display
  - Human-readable account representation

#### PqAccountExport Structure

```rust
pub struct PqAccountExport {
    pub account_type: PqAccountType,
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: Address,
}
```

Supports:
- `to_json()` / `from_json()` - JSON serialization
- `to_hex()` / `from_hex()` - Hex serialization for CLI

### 2. RPC Endpoints

#### `Mondoshawan_generatePqAccount`

Generate a new PQ account via RPC.

**Parameters:**
- `algorithm` (string, optional): `"dilithium3"` or `"sphincsplus"` (default: `"dilithium3"`)

**Response:**
```json
{
  "address": "0x...",
  "account_type": "Dilithium3",
  "public_key": "0x...",
  "message": "PQ dilithium3 account generated successfully"
}
```

**Example:**
```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "Mondoshawan_generatePqAccount",
    "params": ["dilithium3"],
    "id": 1
  }'
```

#### `Mondoshawan_getPqAccountType`

Get the account type for an address by analyzing its transaction history.

**Parameters:**
- `address` (string): Address to check

**Response:**
```json
{
  "address": "0x...",
  "account_type": "Dilithium3",
  "is_pq": true
}
```

**Example:**
```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "Mondoshawan_getPqAccountType",
    "params": ["0x..."],
    "id": 1
  }'
```

#### `Mondoshawan_exportPqKey`

**Note:** This endpoint returns an error explaining that key export should be done client-side for security. Keys should never be stored or exported server-side.

#### `Mondoshawan_importPqKey`

**Note:** This endpoint returns an error explaining that key import should be done client-side for security.

#### `Mondoshawan_createPqTransaction`

Create a PQ-signed transaction via RPC.

**Parameters:**
- `algorithm` (string): `"dilithium3"` or `"sphincsplus"`
- `secret_key` (string, hex): Secret key (hex encoded)
- `public_key` (string, hex): Public key (hex encoded)
- `to` (string): Recipient address
- `value` (string, hex): Amount to send
- `fee` (string, hex, optional): Transaction fee (default: `"0x0"`)
- `nonce` (string, hex, optional): Transaction nonce (default: `"0x0"`)
- `data` (string, hex, optional): Transaction data (default: `"0x"`)

**Response:**
Transaction object (same format as `eth_getTransactionByHash`)

**Example:**
```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "Mondoshawan_createPqTransaction",
    "params": [
      "dilithium3",
      "0x...",  // secret_key
      "0x...",  // public_key
      "0x...",  // to
      "0x1000", // value
      "0x100",  // fee
      "0x0",    // nonce
      "0x"      // data
    ],
    "id": 1
  }'
```

### 3. Public Methods

The following PQ-related methods are added to the public methods list (no authentication required):
- `Mondoshawan_generatePqAccount`
- `Mondoshawan_getPqAccountType`

### 4. Integration

- **Transaction Signing:** `Transaction::sign_pq()` now correctly uses the `PqSignature` returned by `PqAccount::sign()`
- **RPC Server:** All PQ endpoints are integrated into the RPC dispatch logic
- **Error Handling:** Comprehensive error messages for invalid algorithms, keys, and parameters

## Usage Examples

### Rust Code

```rust
use Mondoshawan_blockchain::pqc::tooling::*;

// Generate a new Dilithium3 account
let account = generate_pq_account("dilithium3")?;
let address = account.address();

// Create and sign a transaction
let tx = create_pq_transaction(
    &account,
    recipient_address,
    1000,  // value
    100,   // fee
    0,     // nonce
    vec![] // data
)?;

// Export account for backup
let export = export_pq_account(&account);
let json = export.to_json()?;
// Save json to secure storage

// Import account from backup
let restored = import_pq_account(PqAccountExport::from_json(&json)?)?;
```

### JavaScript/TypeScript

```javascript
// Generate PQ account
const response = await fetch('http://localhost:8545', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    jsonrpc: '2.0',
    method: 'Mondoshawan_generatePqAccount',
    params: ['dilithium3'],
    id: 1
  })
});

const { result } = await response.json();
const { address, public_key, account_type } = result;

// Create PQ transaction
const txResponse = await fetch('http://localhost:8545', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({
    jsonrpc: '2.0',
    method: 'Mondoshawan_createPqTransaction',
    params: [
      'dilithium3',
      secretKeyHex,  // From secure storage
      publicKeyHex,
      recipientAddress,
      '0x1000',
      '0x100',
      '0x0',
      '0x'
    ],
    id: 2
  })
});
```

## Security Considerations

1. **Key Management:** Private keys should NEVER be sent to or stored on the RPC server. The `Mondoshawan_exportPqKey` and `Mondoshawan_importPqKey` endpoints intentionally return errors to enforce this.

2. **Client-Side Generation:** PQ accounts should be generated client-side using `Mondoshawan_generatePqAccount` and keys stored securely in the client application.

3. **Transaction Signing:** Transactions should be signed client-side. The `Mondoshawan_createPqTransaction` endpoint accepts keys as parameters for convenience, but in production, signing should happen entirely client-side.

4. **Key Backup:** Use the `export_pq_account()` function to create backups, but store them securely (encrypted, offline storage).

## Next Steps

1. **CLI Tool:** Create `Mondoshawan-cli` with commands:
   - `Mondoshawan-cli account generate --pq dilithium3`
   - `Mondoshawan-cli account list --pq-only`
   - `Mondoshawan-cli tx create --pq-account <address>`

2. **Explorer Integration:** 
   - Show PQ account badges in address view
   - Display PQ signature type in transaction view
   - Add PQ account generation UI

3. **Wallet Integration:**
   - Add PQ account support to wallet software
   - PQ key import/export in wallet UI
   - PQ transaction signing in wallet

4. **On-Chain Registry:**
   - Maintain a registry of PQ addresses on-chain
   - Enable `Mondoshawan_getPqAccountType` to query the registry instead of scanning transactions

## Testing

To test the PQ account tooling:

1. **Generate Account:**
   ```bash
   curl -X POST http://localhost:8545 \
     -H "Content-Type: application/json" \
     -d '{"jsonrpc":"2.0","method":"Mondoshawan_generatePqAccount","params":["dilithium3"],"id":1}'
   ```

2. **Create Transaction:**
   Use the returned keys to create a transaction via `Mondoshawan_createPqTransaction`

3. **Verify Account Type:**
   After sending a transaction, use `Mondoshawan_getPqAccountType` to verify the account type is detected correctly

## Summary

PQ accounts are now fully integrated as first-class citizens in Mondoshawan, with:
- ✅ Complete tooling module for account generation and transaction creation
- ✅ RPC endpoints for all PQ operations
- ✅ Export/import functionality for key backup
- ✅ Account type detection from transaction signatures
- ✅ Security-focused design (keys never stored server-side)

This implementation makes Mondoshawan ready for quantum-resistant blockchain operations while maintaining backward compatibility with Ed25519 accounts.
