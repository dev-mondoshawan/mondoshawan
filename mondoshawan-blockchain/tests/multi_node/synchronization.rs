//! Network synchronization tests

use mondoshawan_blockchain::blockchain::{Blockchain, Block, BlockHeader};
use mondoshawan_blockchain::consensus::GhostDAG;
use mondoshawan_blockchain::types::StreamType;

/// Test chain synchronization structure
#[tokio::test]
async fn test_chain_synchronization() {
    let mut blockchain = Blockchain::new();
    let mut consensus = GhostDAG::new();
    
    // Create chain of blocks
    let mut prev_hash = [0u8; 32];
    for i in 0..5 {
        let block_header = BlockHeader::new(prev_hash, i, StreamType::StreamA, 4);
        let block = Block::new(block_header, vec![], vec![prev_hash]);
        prev_hash = block.hash;
        
        blockchain.add_block(block.clone()).unwrap();
        consensus.add_block(block);
    }
    
    // Verify chain length
    assert_eq!(blockchain.chain_length(), 5);
    
    // Verify consensus has all blocks
    let ordered = consensus.get_ordered_blocks();
    assert_eq!(ordered.len(), 5);
}

/// Test state consistency across nodes (simulated)
#[tokio::test]
async fn test_state_consistency() {
    let mut blockchain1 = Blockchain::new();
    let mut blockchain2 = Blockchain::new();
    
    // Add same blocks to both
    let block_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let block = Block::new(block_header, vec![], vec![]);
    
    blockchain1.add_block(block.clone()).unwrap();
    blockchain2.add_block(block.clone()).unwrap();
    
    // Both should have same chain length
    assert_eq!(blockchain1.chain_length(), blockchain2.chain_length());
    assert_eq!(blockchain1.chain_length(), 1);
}

