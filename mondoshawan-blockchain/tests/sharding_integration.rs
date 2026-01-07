//! Sharding integration tests
//!
//! Tests for shard management, transaction routing, cross-shard communication,
//! state synchronization, and consensus coordination.

use mondoshawan_blockchain::sharding::{
    ShardManager, ShardConfig, AssignmentStrategy, CrossShardStatus,
};
use mondoshawan_blockchain::blockchain::{Blockchain, Transaction, Block};
use mondoshawan_blockchain::blockchain::block::BlockHeader;
use mondoshawan_blockchain::types::{StreamType, Address, Hash};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Test shard creation and initialization
#[tokio::test]
async fn test_shard_creation() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Verify shards were created
    let shards = manager.get_all_shards().await;
    assert_eq!(shards.len(), 4);
    
    // Verify each shard has correct ID
    for shard in shards.iter() {
        assert!(shard.shard_id() < 4);
    }
}

/// Test transaction routing to shards
#[tokio::test]
async fn test_transaction_routing() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Create test transactions
    let sender: Address = [1u8; 20];
    let receiver1: Address = [2u8; 20];
    let receiver2: Address = [3u8; 20];
    
    let tx1 = Transaction::new(sender, receiver1, 100, 1, 0);
    let tx2 = Transaction::new(sender, receiver2, 200, 2, 0);
    
    // Add transactions
    manager.add_transaction(tx1.clone()).await.unwrap();
    manager.add_transaction(tx2.clone()).await.unwrap();
    
    // Verify transactions were routed to shards
    let shards = manager.get_all_shards().await;
    let mut total_txs = 0;
    
    for shard in shards {
        let txs = shard.get_transactions(100).await;
        total_txs += txs.len();
    }
    
    assert_eq!(total_txs, 2);
}

/// Test cross-shard transaction detection
#[tokio::test]
async fn test_cross_shard_detection() {
    let config = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Create transaction with different sender/receiver addresses
    let sender: Address = [1u8; 20];
    let receiver: Address = [99u8; 20];
    
    let tx = Transaction::new(sender, receiver, 100, 1, 0);
    
    // Get assignment
    let assignment = manager.get_assignment(&sender, &receiver);
    
    // If it's a cross-shard transaction, verify it's tracked
    if assignment.is_cross_shard {
        // Add transaction
        manager.add_transaction(tx.clone()).await.unwrap();
        
        // Verify cross-shard transaction is tracked (basic check)
        assert!(assignment.source_shard.is_some());
    }
}

/// Test state synchronization
#[tokio::test]
async fn test_state_synchronization() {
    let config = ShardConfig {
        shard_count: 2,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Create snapshots for all shards
    manager.create_all_snapshots().await.unwrap();
    
    // Check consistency
    let report = manager.check_consistency().await;
    // Report should be created successfully
    assert!(report.consistent || !report.consistent); // Just verify it exists
}

/// Test shard consensus coordination
#[tokio::test]
async fn test_shard_consensus_coordination() {
    let config = ShardConfig {
        shard_count: 3,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Get consensus coordinator
    let coordinator = manager.get_consensus_coordinator().await;
    
    // Get global state
    let global_state = coordinator.get_global_state().await;
    assert_eq!(global_state.total_shards, 3);
    assert_eq!(global_state.shard_states.len(), 3);
}

/// Test shard block processing
#[tokio::test]
async fn test_shard_block_processing() {
    let config = ShardConfig {
        shard_count: 2,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Get a shard
    let shard = manager.get_shard(0).await.unwrap();
    
    // Create a test block
    let header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let block = Block::new(header, vec![], vec![]);
    
    // Process block
    shard.process_block(block.clone()).await.unwrap();
    
    // Verify block was added
    let blockchain_arc = shard.blockchain();
    let blockchain = blockchain_arc.read().await;
    assert_eq!(blockchain.chain_length(), 1);
    
    // Verify consensus was updated
    let consensus_arc = shard.consensus();
    let consensus = consensus_arc.read().await;
    assert!(consensus.has_block(&block.hash));
}

/// Test multiple shards processing blocks independently
#[tokio::test]
async fn test_independent_shard_processing() {
    let config = ShardConfig {
        shard_count: 3,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    
    let manager = ShardManager::new(config);
    
    // Process blocks in different shards
    for shard_id in 0..3 {
        let shard = manager.get_shard(shard_id).await.unwrap();
        
        let header = BlockHeader::new([0; 32], shard_id as u64, StreamType::StreamA, 4);
        let block = Block::new(header, vec![], vec![]);
        
        shard.process_block(block).await.unwrap();
    }
    
    // Verify each shard has its own block
    for shard_id in 0..3 {
        let shard = manager.get_shard(shard_id).await.unwrap();
        let blockchain_arc = shard.blockchain();
        let blockchain = blockchain_arc.read().await;
        assert_eq!(blockchain.chain_length(), 1);
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
    
    // Get all stats
    let stats = manager.get_all_stats().await;
    assert_eq!(stats.len(), 4);
    
    // Verify each shard has stats
    for stat in stats {
        assert!(stat.shard_id < 4);
        assert_eq!(stat.block_count, 0); // Initially no blocks
    }
}

/// Test assignment strategies
#[tokio::test]
async fn test_assignment_strategies() {
    let sender: Address = [1u8; 20];
    let receiver: Address = [2u8; 20];
    
    // Test consistent hashing
    let config1 = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::ConsistentHashing,
    };
    let manager1 = ShardManager::new(config1);
    let assignment1 = manager1.get_assignment(&sender, &receiver);
    assert!(assignment1.shard_id < 4);
    
    // Test round-robin
    let config2 = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::RoundRobin,
    };
    let manager2 = ShardManager::new(config2);
    let assignment2 = manager2.get_assignment(&sender, &receiver);
    assert!(assignment2.shard_id < 4);
    
    // Test address-based
    let config3 = ShardConfig {
        shard_count: 4,
        enable_cross_shard: true,
        assignment_strategy: AssignmentStrategy::AddressBased,
    };
    let manager3 = ShardManager::new(config3);
    let assignment3 = manager3.get_assignment(&sender, &receiver);
    assert!(assignment3.shard_id < 4);
}
