//! Integration tests for Network + Blockchain

use mondoshawan_blockchain::blockchain::{Blockchain, Block, BlockHeader, Transaction};
use mondoshawan_blockchain::network::NetworkManager;
use mondoshawan_blockchain::types::{StreamType, Hash, Address};

/// Test network manager creation
#[tokio::test]
async fn test_network_blockchain_integration() {
    // Create network manager
    let network_manager = NetworkManager::new().await;
    assert!(network_manager.is_ok());
    
    let mut network = network_manager.unwrap();
    
    // Test listening
    let result = network.listen_on("/ip4/127.0.0.1/tcp/0");
    assert!(result.is_ok());
}

/// Test block broadcasting (simplified - actual broadcasting needs running nodes)
#[tokio::test]
async fn test_block_broadcasting() {
    let mut blockchain = Blockchain::new();
    let mut network = NetworkManager::new().await.unwrap();
    
    // Create a block
    let block_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let block = Block::new(block_header, vec![], vec![]);
    
    // Broadcast block (will work even without peers)
    network.broadcast_block(&block).await;
    
    // Verify network is set up
    assert_eq!(network.connected_peers(), 0); // No peers yet
}

/// Test transaction broadcasting
#[tokio::test]
async fn test_transaction_broadcasting() {
    let mut blockchain = Blockchain::new();
    let sender: Address = [1u8; 20];
    blockchain.set_balance(sender, 10000);
    
    let mut network = NetworkManager::new().await.unwrap();
    
    // Create transaction
    let tx = Transaction::new(sender, [2u8; 20], 1000, 10, 0);
    
    // Broadcast transaction
    network.broadcast_transaction(&tx).await;
    
    // Verify network is set up
    assert_eq!(network.connected_peers(), 0); // No peers yet
}

