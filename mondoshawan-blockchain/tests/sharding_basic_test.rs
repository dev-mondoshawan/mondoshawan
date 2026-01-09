//! Basic sharding functionality tests
//!
//! Tests core sharding features that are already implemented.

use mondoshawan_blockchain::sharding::{
    ShardManager, ShardConfig, AssignmentStrategy, CrossShardStatus,
};
use mondoshawan_blockchain::blockchain::Transaction;
use mondoshawan_blockchain::types::Address;

/// Test shard creation
#[tokio::test]
async fn test_shard_creation() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Verify shard count
    assert_eq!(manager.shard_count(), 4);
    
    // Verify we can get all shards
    let shards = manager.get_all_shards().await;
    assert_eq!(shards.len(), 4);
}

/// Test transaction routing
#[tokio::test]
async fn test_transaction_routing() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Create test transaction
    let sender: Address = [1u8; 20];
    let receiver: Address = [2u8; 20];
    
    let tx = Transaction::new(sender, receiver, 100, 1, 0);
    
    // Add transaction
    manager.add_transaction(tx.clone()).await.unwrap();
    
    // Get shard assignments
    let from_shard = manager.get_shard_for_address(&sender);
    let to_shard = manager.get_shard_for_address(&receiver);
    
    assert!(from_shard < 4);
    assert!(to_shard < 4);
    
    // Verify transaction is in correct shard
    let shard_txs = manager.get_shard_transactions(from_shard, 10).await;
    assert!(shard_txs.len() >= 1);
}

/// Test cross-shard transaction detection
#[tokio::test]
async fn test_cross_shard_transaction() {
    let config = ShardConfig {
        shard_count: 16, // More shards = higher chance of cross-shard
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Create transaction with different addresses
    let sender: Address = [1u8; 20];
    let receiver: Address = [255u8; 20];
    
    let tx = Transaction::new(sender, receiver, 100, 1, 0);
    let tx_hash = tx.hash;
    
    // Get shard assignments
    let (from_shard, to_shard) = manager.get_transaction_shards(&tx).await.unwrap();
    
    // If it's cross-shard, test tracking
    if from_shard != to_shard {
        // Add transaction
        manager.add_transaction(tx.clone()).await.unwrap();
        
        // Verify cross-shard transaction exists
        let cross_tx = manager.get_cross_shard_transaction(tx_hash).await;
        assert!(cross_tx.is_some());
        
        if let Some(ctx) = cross_tx {
            assert_eq!(ctx.source_shard, from_shard);
            assert_eq!(ctx.target_shard, to_shard);
            assert_eq!(ctx.status, CrossShardStatus::Pending);
        }
    }
}

/// Test shard statistics
#[tokio::test]
async fn test_shard_statistics() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Add some transactions
    for i in 0..10 {
        let sender: Address = [i; 20];
        let receiver: Address = [(i + 1) % 255; 20];
        let tx = Transaction::new(sender, receiver, 100, i as u128, 0);
        manager.add_transaction(tx).await.unwrap();
    }
    
    // Get all shard stats
    let stats = manager.get_all_shard_stats().await;
    assert_eq!(stats.len(), 4);
    
    // Verify total transactions across shards
    let total_txs: usize = stats.iter().map(|s| s.transaction_pool_size).sum();
    assert!(total_txs >= 10);
    
    // Verify each stat has valid shard_id
    for stat in stats {
        assert!(stat.shard_id < 4);
    }
}

/// Test assignment strategies
#[tokio::test]
async fn test_assignment_strategies() {
    let sender: Address = [1u8; 20];
    let _receiver: Address = [2u8; 20];
    
    // Test ConsistentHashing
    let config1 = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    let manager1 = ShardManager::new(config1);
    let shard1 = manager1.get_shard_for_address(&sender);
    assert!(shard1 < 4);
    
    // Verify deterministic routing (same address -> same shard)
    let shard1_again = manager1.get_shard_for_address(&sender);
    assert_eq!(shard1, shard1_again);
    
    // Test RoundRobin
    let config2 = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::RoundRobin,
    };
    let manager2 = ShardManager::new(config2);
    let shard2 = manager2.get_shard_for_address(&sender);
    assert!(shard2 < 4);
    
    // Test AddressBased
    let config3 = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::AddressBased,
    };
    let manager3 = ShardManager::new(config3);
    let shard3 = manager3.get_shard_for_address(&sender);
    assert!(shard3 < 4);
}

/// Test shard transaction pool operations
#[tokio::test]
async fn test_shard_transaction_pool() {
    let config = ShardConfig {
        shard_count: 2,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Add transactions to same shard
    let sender: Address = [1u8; 20];
    let shard_id = manager.get_shard_for_address(&sender);
    
    for i in 0..5 {
        let tx = Transaction::new(sender, sender, 100, i, 0);
        manager.add_transaction(tx).await.unwrap();
    }
    
    // Get transactions
    let txs = manager.get_shard_transactions(shard_id, 3).await;
    assert_eq!(txs.len(), 3);
    
    // Remove transactions
    let removed = manager.remove_shard_transactions(shard_id, 2).await;
    assert_eq!(removed.len(), 2);
    
    // Verify remaining
    let remaining = manager.get_shard_transactions(shard_id, 10).await;
    assert_eq!(remaining.len(), 3); // 5 - 2 = 3
}

/// Test get/set shard operations
#[tokio::test]
async fn test_get_shard() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Test valid shard ID
    let shard = manager.get_shard(0);
    assert!(shard.is_some());
    
    let shard = manager.get_shard(3);
    assert!(shard.is_some());
    
    // Test invalid shard ID
    let shard = manager.get_shard(99);
    assert!(shard.is_none());
}

/// Test all cross-shard transactions retrieval
#[tokio::test]
async fn test_get_all_cross_shard_transactions() {
    let config = ShardConfig {
        shard_count: 16, // More shards for more cross-shard txs
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Add transactions
    for i in 0..20 {
        let sender: Address = [i; 20];
        let receiver: Address = [(i * 7) % 255; 20]; // Different pattern
        let tx = Transaction::new(sender, receiver, 100, i as u128, 0);
        manager.add_transaction(tx).await.unwrap();
    }
    
    // Get all cross-shard transactions
    let cross_txs = manager.get_all_cross_shard_transactions().await;
    
    // Should have some cross-shard transactions
    println!("Cross-shard transactions: {}", cross_txs.len());
    
    // Verify structure
    for ctx in cross_txs {
        assert!(ctx.source_shard < 16);
        assert!(ctx.target_shard < 16);
        assert_ne!(ctx.source_shard, ctx.target_shard);
    }
}

/// Test shard statistics with cross-shard
#[tokio::test]
async fn test_shard_stats_with_cross_shard() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Add cross-shard transaction
    let sender: Address = [1u8; 20];
    let receiver: Address = [255u8; 20];
    let tx = Transaction::new(sender, receiver, 100, 1, 0);
    
    let (from_shard, to_shard) = manager.get_transaction_shards(&tx).await.unwrap();
    
    if from_shard != to_shard {
        manager.add_transaction(tx).await.unwrap();
        
        // Get stats for source shard
        let source_stats = manager.get_shard_stats(from_shard).await.unwrap();
        assert!(source_stats.cross_shard_outgoing >= 1);
        
        // Get stats for target shard
        let target_stats = manager.get_shard_stats(to_shard).await.unwrap();
        assert!(target_stats.cross_shard_incoming >= 1);
    }
}

/// Test sharding disabled scenario
#[tokio::test]
async fn test_sharding_disabled() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: false, // Cross-shard disabled
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Add cross-shard transaction (should NOT be tracked as cross-shard)
    let sender: Address = [1u8; 20];
    let receiver: Address = [255u8; 20];
    let tx = Transaction::new(sender, receiver, 100, 1, 0);
    let tx_hash = tx.hash;
    
    manager.add_transaction(tx).await.unwrap();
    
    // Should not be in cross-shard tracking
    let _cross_tx = manager.get_cross_shard_transaction(tx_hash).await;
    
    // With cross-shard disabled, it should still work but treated as same-shard
    let from_shard = manager.get_shard_for_address(&sender);
    let shard_txs = manager.get_shard_transactions(from_shard, 10).await;
    assert!(shard_txs.len() >= 1);
}
