# Bug Fixes: Nonce Validation and Shard Pool Size

**Status:** ✅ **Fixed**  
**Date:** December 2024

## Bug 1: Nonce Skipping Vulnerability ✅ FIXED

### Problem
The nonce validation accepted `tx.nonce >= current_nonce` but the update logic set `new_nonce = tx.nonce + 1`. This allowed transactions to skip nonce values, breaking sequential ordering.

**Example:**
- Current nonce: 5
- Transaction with nonce 7 is accepted (7 >= 5)
- New nonce becomes 8 (7 + 1)
- Nonce 6 is now forever unusable

### Fix
Changed validation to require **strict sequential ordering**: `tx.nonce == current_nonce`

**Changes Made:**
- `src/blockchain/mod.rs` line 302-307: Changed validation from `>=` to `==`
- `src/blockchain/mod.rs` line 392-412: Simplified update logic to always increment by 1

**Before:**
```rust
// Validation
if tx.nonce < current_nonce { ... }

// Update
if tx.nonce >= current_nonce {
    let new_nonce = tx.nonce + 1; // Can skip values!
}
```

**After:**
```rust
// Validation
if tx.nonce != current_nonce { ... } // Strict sequential

// Update
let new_nonce = current_nonce + 1; // Always increment by 1
```

### Impact
- ✅ Prevents nonce gaps
- ✅ Maintains strict sequential ordering
- ✅ Matches Ethereum-style nonce behavior
- ⚠️ Breaking change: Transactions with non-sequential nonces will now be rejected

## Bug 2: Missing Shard Pool Size Enforcement ✅ FIXED

### Problem
`MAX_SHARD_TX_POOL_SIZE` was defined but never enforced in `Shard::add_transaction()`. The method simply pushed transactions without checking pool size, allowing unbounded memory growth per shard.

**Issues:**
- Line 14: `MAX_SHARD_TX_POOL_SIZE = 50_000` defined
- Line 82-84: `add_transaction()` just pushes without size check
- Line 160: Comment claims "handles pool size limits internally" but doesn't
- No FIFO eviction implemented

### Fix
Implemented FIFO eviction in `Shard::add_transaction()` matching the pattern used in `MiningManager::add_transaction()`.

**Changes Made:**
- `src/sharding.rs` line 82-84: Added pool size check and FIFO eviction
- `src/sharding.rs` line 160: Updated comment to reflect actual behavior

**Before:**
```rust
pub fn add_transaction(&mut self, tx: Transaction) {
    self.transaction_pool.push(tx); // No size check!
}
```

**After:**
```rust
pub fn add_transaction(&mut self, tx: Transaction) {
    // Enforce pool size limit (DoS protection)
    // If pool is full, remove oldest transactions (FIFO eviction)
    while self.transaction_pool.len() >= MAX_SHARD_TX_POOL_SIZE {
        self.transaction_pool.remove(0); // Remove oldest transaction
    }
    
    self.transaction_pool.push(tx);
}
```

### Impact
- ✅ Prevents unbounded memory growth per shard
- ✅ Enforces DoS protection as intended
- ✅ Matches behavior of main transaction pool
- ✅ FIFO eviction ensures oldest transactions are removed first

## Testing Recommendations

### Test Bug 1 Fix
```rust
#[test]
fn test_nonce_sequential_ordering() {
    let mut blockchain = Blockchain::new();
    let address = [1u8; 20];
    
    // Set initial nonce
    blockchain.set_nonce(address, 5).unwrap();
    
    // Transaction with nonce 5 should succeed
    let tx1 = create_tx_with_nonce(address, 5);
    assert!(blockchain.validate_transaction(&tx1).is_ok());
    
    // Transaction with nonce 7 should fail (skipping 6)
    let tx2 = create_tx_with_nonce(address, 7);
    assert!(blockchain.validate_transaction(&tx2).is_err());
    
    // After processing tx1, nonce should be 6
    blockchain.process_transaction(&tx1).unwrap();
    assert_eq!(blockchain.get_nonce(address), 6);
}
```

### Test Bug 2 Fix
```rust
#[test]
fn test_shard_pool_size_limit() {
    let mut shard = Shard::new(0);
    
    // Add transactions up to limit
    for i in 0..MAX_SHARD_TX_POOL_SIZE {
        let tx = create_test_transaction(i);
        shard.add_transaction(tx);
    }
    
    assert_eq!(shard.transaction_pool.len(), MAX_SHARD_TX_POOL_SIZE);
    
    // Adding one more should trigger FIFO eviction
    let tx_overflow = create_test_transaction(MAX_SHARD_TX_POOL_SIZE);
    shard.add_transaction(tx_overflow);
    
    // Pool should still be at limit
    assert_eq!(shard.transaction_pool.len(), MAX_SHARD_TX_POOL_SIZE);
    
    // First transaction should be removed (FIFO)
    assert_ne!(shard.transaction_pool[0].nonce, 0);
}
```

## Related Files

- `src/blockchain/mod.rs` - Nonce validation and update logic
- `src/sharding.rs` - Shard transaction pool management
- `src/mining.rs` - Reference implementation for FIFO eviction (lines 109-111)

---

**Status:** Both bugs verified and fixed! ✅
