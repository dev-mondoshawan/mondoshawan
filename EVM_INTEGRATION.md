# EVM Integration - Smart Contract Support

**Status**: ✅ **Basic Implementation Complete**  
**Last Updated**: January 2026

---

## Overview

Mondoshawan blockchain now includes **EVM (Ethereum Virtual Machine) integration** for smart contract support. This enables deployment and execution of Ethereum-compatible smart contracts on the Mondoshawan network.

---

## Current Implementation

### ✅ What's Working

1. **Contract Deployment**
   - Contracts can be deployed via transactions with data
   - Contract addresses are generated deterministically
   - Contract code is stored and retrievable

2. **Contract Calls**
   - Basic structure for calling contracts
   - Transaction data handling
   - Gas limit validation

3. **EVM State Management**
   - Contract code storage
   - Account state tracking
   - Nonce management

4. **Integration**
   - EVM enabled by default in node
   - Integrated with blockchain transaction processing
   - RPC API methods available

### ⚠️ Current Limitations

1. **Bytecode Execution**
   - Currently uses simplified execution
   - Full revm 33.1 integration pending
   - Contract execution results are basic

2. **Gas Metering**
   - Basic gas limits enforced
   - Detailed gas calculation pending
   - Gas refunds not yet implemented

3. **State Storage**
   - Contract code stored in memory
   - EVM state persistence pending
   - Contract storage not yet implemented

---

## Architecture

### Components

```
┌─────────────────────────────────────────┐
│         Blockchain                      │
│  ┌───────────────────────────────────┐ │
│  │   EvmTransactionExecutor          │ │
│  │  ┌─────────────────────────────┐ │ │
│  │  │      EvmState               │ │ │
│  │  │  - Contract storage         │ │ │
│  │  │  - Account state            │ │ │
│  │  │  - Nonce tracking           │ │ │
│  │  └─────────────────────────────┘ │ │
│  └───────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

### Transaction Flow

1. **Contract Deployment**:
   ```
   Transaction (to=0x0, data=bytecode) 
   → EvmTransactionExecutor.execute_transaction()
   → Generate contract address
   → Store contract code
   → Return contract address
   ```

2. **Contract Call**:
   ```
   Transaction (to=contract, data=call_data)
   → EvmTransactionExecutor.execute_transaction()
   → Execute contract (simplified)
   → Return execution result
   ```

---

## Usage

### Deploying a Contract

```rust
use Mondoshawan_blockchain::blockchain::Transaction;
use Mondoshawan_blockchain::types::Address;

// Contract bytecode
let bytecode = vec![/* EVM bytecode */];

// Create deployment transaction
let tx = Transaction::with_data(
    deployer_address,
    [0u8; 20], // Zero address for deployment
    0,         // Value
    0,         // Fee
    nonce,
    bytecode,
    1_000_000, // Gas limit
);

// Add to blockchain
blockchain.add_block(block)?;
```

### Calling a Contract

```rust
// Contract call data (function selector + parameters)
let call_data = vec![/* ABI-encoded call data */];

// Create call transaction
let tx = Transaction::with_data(
    caller_address,
    contract_address,
    0,         // Value
    0,         // Fee
    nonce,
    call_data,
    100_000,   // Gas limit
);

// Add to blockchain
blockchain.add_block(block)?;
```

---

## JSON-RPC API

### New Methods

#### `eth_getCode`

Get contract code at an address.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "eth_getCode",
  "params": ["0x1234..."],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": "0x6080604052...",
  "id": 1
}
```

#### `eth_estimateGas`

Estimate gas for a transaction.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "eth_estimateGas",
  "params": [{
    "from": "0x...",
    "to": "0x...",
    "data": "0x..."
  }],
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": "0x5208",
  "id": 1
}
```

---

## Code Structure

### `src/evm.rs`

Main EVM implementation:

- **`EvmState`**: Manages EVM state (contracts, balances, nonces)
- **`EvmTransactionExecutor`**: Executes EVM transactions
- **`ExecutionResult`**: Execution results

**Key Functions**:
- `EvmTransactionExecutor::execute_transaction()` - Execute transaction
- `EvmTransactionExecutor::deploy_contract()` - Deploy contract
- `EvmTransactionExecutor::call_contract()` - Call contract
- `EvmState::store_contract()` - Store contract code
- `EvmState::get_contract_code()` - Get contract code

### Integration Points

- **`src/blockchain/mod.rs`**: EVM execution in transaction processing
- **`src/node/mod.rs`**: EVM enabled by default
- **`src/rpc.rs`**: EVM-related RPC methods

---

## Future Enhancements

### Phase 1: Full revm Integration
- [ ] Complete revm 33.1 API integration
- [ ] Actual bytecode execution
- [ ] Proper gas metering
- [ ] State changes persistence

### Phase 2: Advanced Features
- [ ] Contract storage (SLOAD/SSTORE)
- [ ] Event logs
- [ ] Gas refunds
- [ ] Precompiles support

### Phase 3: Production Ready
- [ ] EVM state persistence
- [ ] Contract verification
- [ ] Security hardening
- [ ] Performance optimization

---

## Testing

### Manual Testing

1. **Deploy a Contract**:
   ```bash
   # Use JSON-RPC to send deployment transaction
   curl -X POST http://localhost:8545 \
     -H "Content-Type: application/json" \
     -d '{
       "jsonrpc": "2.0",
       "method": "eth_sendTransaction",
       "params": [{
         "from": "0x...",
         "to": "0x0",
         "data": "0x6080604052..."
       }],
       "id": 1
     }'
   ```

2. **Get Contract Code**:
   ```bash
   curl -X POST http://localhost:8545 \
     -H "Content-Type: application/json" \
     -d '{
       "jsonrpc": "2.0",
       "method": "eth_getCode",
       "params": ["0xcontract_address"],
       "id": 1
     }'
   ```

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_deployment() {
        let executor = EvmTransactionExecutor::new();
        // Test deployment
    }

    #[test]
    fn test_contract_call() {
        let executor = EvmTransactionExecutor::new();
        // Test contract call
    }
}
```

---

## Configuration

### Enabling/Disabling EVM

EVM is **enabled by default** in the node. To disable:

```rust
let mut blockchain = Blockchain::new();
blockchain.evm_enabled = false;
blockchain.evm_executor = None;
```

---

## Gas Limits

### Current Gas Limits

- **Base transaction**: 21,000 gas
- **Contract deployment**: 21,000 gas (minimum)
- **Contract call**: 21,000 gas (minimum)

### Recommended Gas Limits

- **Simple transfers**: 21,000
- **Contract deployment**: 1,000,000 - 5,000,000
- **Contract calls**: 100,000 - 1,000,000

---

## Contract Address Generation

Contract addresses are generated deterministically from:
- Sender address
- Sender nonce

```rust
address = keccak256(sender_address || nonce)[12:32]
```

This matches Ethereum's CREATE opcode behavior.

---

## Security Considerations

### Current Status

- ✅ Basic validation
- ✅ Gas limit enforcement
- ✅ Transaction validation
- ⚠️ Full bytecode validation pending
- ⚠️ Reentrancy protection pending
- ⚠️ Gas price validation pending

### Best Practices

1. **Validate Contract Code**: Always validate bytecode before deployment
2. **Set Appropriate Gas Limits**: Prevent out-of-gas errors
3. **Check Contract Addresses**: Verify contract addresses before calling
4. **Monitor Gas Usage**: Track gas consumption for optimization

---

## Troubleshooting

### Common Issues

1. **"Not an EVM transaction" error**
   - Ensure transaction has `data` field
   - For deployment, `to` must be zero address
   - For calls, `to` must be a contract address

2. **Contract not found**
   - Verify contract was deployed
   - Check contract address
   - Ensure EVM is enabled

3. **Gas limit too low**
   - Increase gas limit in transaction
   - Check contract complexity
   - Verify gas estimation

---

## Resources

- **Ethereum EVM**: https://ethereum.org/en/developers/docs/evm/
- **Solidity**: https://soliditylang.org/
- **REVM**: https://revm.sh/
- **JSON-RPC API**: See `JSON_RPC_API_GUIDE.md`

---

## Status Summary

✅ **Basic EVM Integration Complete**
- Contract deployment working
- Contract calls working
- RPC API methods available
- Integrated with blockchain

⚠️ **Next Steps**
- Full revm 33.1 integration
- Actual bytecode execution
- Enhanced gas metering
- State persistence

**The foundation is in place for full smart contract support!**
