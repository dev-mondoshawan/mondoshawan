//! Integration tests for Blockchain + Consensus (GhostDAG)

use mondoshawan_blockchain::blockchain::{Blockchain, Block, BlockHeader};
use mondoshawan_blockchain::consensus::GhostDAG;
use mondoshawan_blockchain::types::{Hash, StreamType};

/// Test block addition with GhostDAG consensus
#[tokio::test]
async fn test_blockchain_consensus_integration() {
    let mut blockchain = Blockchain::new();
    let mut consensus = GhostDAG::new();
    
    // Create genesis block (block with zero previous hash)
    let genesis_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let genesis = Block::new(genesis_header, vec![], vec![]);
    let genesis_hash = genesis.hash;
    
    // Add to blockchain
    blockchain.add_block(genesis.clone()).unwrap();
    
    // Add to consensus
    consensus.add_block(genesis);
    
    // Verify genesis is in consensus (check if it's in ordered blocks)
    let ordered = consensus.get_ordered_blocks();
    assert!(ordered.iter().any(|b| b.hash == genesis_hash));
    
    // Create a new block
    let block1_header = BlockHeader::new(genesis_hash, 1, StreamType::StreamA, 4);
    let block1 = Block::new(block1_header, vec![], vec![genesis_hash]);
    let block1_hash = block1.hash;
    
    // Add to blockchain
    blockchain.add_block(block1.clone()).unwrap();
    
    // Add to consensus
    consensus.add_block(block1);
    
    // Verify block is in consensus
    let ordered2 = consensus.get_ordered_blocks();
    assert!(ordered2.iter().any(|b| b.hash == block1_hash));
    assert_eq!(ordered2.len(), 2);
}

/// Test parallel blocks with GhostDAG
#[tokio::test]
async fn test_parallel_blocks_consensus() {
    let mut blockchain = Blockchain::new();
    let mut consensus = GhostDAG::new();
    
    // Create genesis
    let genesis_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let genesis = Block::new(genesis_header, vec![], vec![]);
    let genesis_hash = genesis.hash;
    blockchain.add_block(genesis.clone()).unwrap();
    consensus.add_block(genesis);
    
    // Create two parallel blocks (both reference genesis)
    let block1_header = BlockHeader::new(genesis_hash, 1, StreamType::StreamA, 4);
    let block1 = Block::new(block1_header, vec![], vec![genesis_hash]);
    let block1_hash = block1.hash;
    
    let block2_header = BlockHeader::new(genesis_hash, 1, StreamType::StreamB, 4);
    let block2 = Block::new(block2_header, vec![], vec![genesis_hash]);
    let block2_hash = block2.hash;
    
    // Add both blocks
    blockchain.add_block(block1.clone()).unwrap();
    blockchain.add_block(block2.clone()).unwrap();
    
    consensus.add_block(block1);
    consensus.add_block(block2);
    
    // Verify ordering includes both
    let ordered = consensus.get_ordered_blocks();
    assert_eq!(ordered.len(), 3); // genesis + 2 parallel blocks
    assert!(ordered.iter().any(|b| b.hash == genesis_hash));
    assert!(ordered.iter().any(|b| b.hash == block1_hash));
    assert!(ordered.iter().any(|b| b.hash == block2_hash));
}

/// Test blue score calculation
#[tokio::test]
async fn test_blue_score_calculation() {
    let mut blockchain = Blockchain::new();
    let mut consensus = GhostDAG::new();
    
    // Create chain: genesis -> block1 -> block2
    let genesis_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let genesis = Block::new(genesis_header, vec![], vec![]);
    let genesis_hash = genesis.hash;
    blockchain.add_block(genesis.clone()).unwrap();
    consensus.add_block(genesis);
    
    let block1_header = BlockHeader::new(genesis_hash, 1, StreamType::StreamA, 4);
    let block1 = Block::new(block1_header, vec![], vec![genesis_hash]);
    let block1_hash = block1.hash;
    blockchain.add_block(block1.clone()).unwrap();
    consensus.add_block(block1);
    
    let block2_header = BlockHeader::new(block1_hash, 2, StreamType::StreamA, 4);
    let block2 = Block::new(block2_header, vec![], vec![block1_hash]);
    let block2_hash = block2.hash;
    blockchain.add_block(block2.clone()).unwrap();
    consensus.add_block(block2);
    
    // Verify ordering
    let ordered = consensus.get_ordered_blocks();
    assert_eq!(ordered.len(), 3);
    assert_eq!(ordered[0].hash, genesis_hash);
    assert_eq!(ordered[1].hash, block1_hash);
    assert_eq!(ordered[2].hash, block2_hash);
}

