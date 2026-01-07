//! Block propagation tests

use mondoshawan_blockchain::blockchain::{Blockchain, Block, BlockHeader};
use mondoshawan_blockchain::types::StreamType;

/// Test block structure for propagation
#[tokio::test]
async fn test_block_structure_for_propagation() {
    let mut blockchain = Blockchain::new();
    
    // Create a block
    let block_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let block = Block::new(block_header, vec![], vec![]);
    
    // Verify block is valid
    assert!(block.validate());
    
    // Add to blockchain
    let result = blockchain.add_block(block.clone());
    assert!(result.is_ok());
    
    // Verify block is in chain
    assert_eq!(blockchain.chain_length(), 1);
}

/// Test block validation
#[tokio::test]
async fn test_block_validation() {
    let block_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let block = Block::new(block_header, vec![], vec![]);
    
    // Block should be valid
    assert!(block.validate());
    
    // Block hash should be non-zero
    assert_ne!(block.hash, [0u8; 32]);
}

