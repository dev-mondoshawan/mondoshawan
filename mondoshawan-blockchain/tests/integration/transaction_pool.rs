//! Integration tests for Transaction Pool

use mondoshawan_blockchain::node::pool::TransactionPool;
use mondoshawan_blockchain::blockchain::Transaction;
use mondoshawan_blockchain::types::Hash;

/// Test transaction pool basic operations
#[tokio::test]
async fn test_transaction_pool_basic() {
    let mut pool = TransactionPool::new(100);
    
    // Create transaction
    let tx = Transaction::new(
        [1u8; 20],
        [2u8; 20],
        1000,
        10,
        0,
    );
    let tx_hash = tx.hash;
    
    // Add transaction
    assert!(pool.add(tx.clone()));
    assert_eq!(pool.size(), 1);
    assert!(pool.contains(&tx_hash));
    
    // Get transactions
    let txs = pool.get_transactions(10);
    assert_eq!(txs.len(), 1);
    assert_eq!(txs[0].hash, tx_hash);
    
    // Remove transaction
    let removed = pool.remove(&tx_hash);
    assert!(removed.is_some());
    assert_eq!(pool.size(), 0);
}

/// Test transaction pool priority
#[tokio::test]
async fn test_transaction_pool_priority() {
    let mut pool = TransactionPool::new(100);
    
    // Add transactions with different fees
    let tx1 = Transaction::new([1u8; 20], [2u8; 20], 100, 10, 0);
    let tx2 = Transaction::new([3u8; 20], [4u8; 20], 100, 20, 0); // Higher fee
    let tx3 = Transaction::new([5u8; 20], [6u8; 20], 100, 15, 0);
    
    pool.add(tx1.clone());
    pool.add(tx2.clone());
    pool.add(tx3.clone());
    
    // Get transactions (should be ordered by fee)
    let txs = pool.get_transactions(10);
    assert_eq!(txs.len(), 3);
    // Highest fee should be first
    assert_eq!(txs[0].fee, 20);
    assert_eq!(txs[1].fee, 15);
    assert_eq!(txs[2].fee, 10);
}

/// Test transaction pool size limit
#[tokio::test]
async fn test_transaction_pool_size_limit() {
    let mut pool = TransactionPool::new(5); // Small limit
    
    // Add transactions up to limit
    for i in 0..5 {
        let tx = Transaction::new([1u8; 20], [2u8; 20], 100, 10, i);
        assert!(pool.add(tx));
    }
    
    assert_eq!(pool.size(), 5);
    
    // Try to add one more (should remove oldest low-priority)
    let tx = Transaction::new([1u8; 20], [2u8; 20], 100, 100, 5); // High fee
    assert!(pool.add(tx));
    
    // Pool should still be at limit
    assert_eq!(pool.size(), 5);
}

/// Test transaction pool deduplication
#[tokio::test]
async fn test_transaction_pool_deduplication() {
    let mut pool = TransactionPool::new(100);
    
    let tx = Transaction::new([1u8; 20], [2u8; 20], 100, 10, 0);
    let tx_hash = tx.hash;
    
    // Add transaction twice
    assert!(pool.add(tx.clone()));
    assert!(!pool.add(tx.clone())); // Should fail (duplicate)
    
    assert_eq!(pool.size(), 1);
}

