//! End-to-end integration tests

use mondoshawan_blockchain::blockchain::{Blockchain, Block, Transaction};
use mondoshawan_blockchain::consensus::GhostDAG;
use mondoshawan_blockchain::storage::{Database, BlockStore, StateStore};
use mondoshawan_blockchain::node::pool::TransactionPool;
use mondoshawan_blockchain::types::Address;
use tempfile::TempDir;

/// Test complete transaction flow
#[tokio::test]
async fn test_complete_transaction_flow() {
    // Setup
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    let database = Database::open(&db_path).unwrap();
    let block_store = BlockStore::new(&database);
    let state_store = StateStore::new(&database);
    
    let mut blockchain = Blockchain::new();
    let mut consensus = GhostDAG::new();
    let mut tx_pool = TransactionPool::new(100);
    
    // 1. Set balance for sender
    let sender: Address = [1u8; 20];
    blockchain.set_balance(sender, 10000);
    
    // 2. Create transaction
    let tx = Transaction::new(
        sender,
        [2u8; 20],
        1000,
        10,
        0,
    );
    
    // 3. Add to pool
    tx_pool.add(tx.clone());
    
    // 4. Create block with transaction
    use mondoshawan_blockchain::blockchain::BlockHeader;
    use mondoshawan_blockchain::types::StreamType;
    
    let genesis_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let genesis = Block::new(genesis_header, vec![], vec![]);
    let genesis_hash = genesis.hash;
    blockchain.add_block(genesis.clone()).unwrap();
    consensus.add_block(genesis);
    
    let block_header = BlockHeader::new(genesis_hash, 1, StreamType::StreamA, 4);
    let block = Block::new(block_header, vec![tx.clone()], vec![genesis_hash]);
    let block_hash = block.hash;
    
    // 5. Add block to blockchain
    blockchain.add_block(block.clone()).unwrap();
    
    // 6. Add to consensus
    consensus.add_block(block.clone());
    
    // 7. Store in database
    block_store.put_block(&block).unwrap();
    state_store.put_latest_hash(&block_hash).unwrap();
    
    // 8. Remove transaction from pool
    tx_pool.remove(&tx.hash);
    
    // Verify everything
    assert_eq!(blockchain.chain_length(), 2); // genesis + block
    // Verify block is in consensus (check ordered blocks)
    let ordered = consensus.get_ordered_blocks();
    assert!(ordered.iter().any(|b| b.hash == block_hash));
    assert_eq!(tx_pool.size(), 0);
    
    // Verify block in database
    let retrieved = block_store.get_block(&block_hash).unwrap();
    assert!(retrieved.is_some());
}

/// Test blockchain state consistency
#[tokio::test]
async fn test_blockchain_state_consistency() {
    use mondoshawan_blockchain::blockchain::BlockHeader;
    use mondoshawan_blockchain::types::StreamType;
    
    let mut blockchain = Blockchain::new();
    let mut consensus = GhostDAG::new();
    
    // Create chain
    let genesis_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let genesis = Block::new(genesis_header, vec![], vec![]);
    let mut prev_hash = genesis.hash;
    blockchain.add_block(genesis.clone()).unwrap();
    consensus.add_block(genesis);
    
    for i in 1..=5 {
        let block_header = BlockHeader::new(prev_hash, i, StreamType::StreamA, 4);
        let block = Block::new(block_header, vec![], vec![prev_hash]);
        prev_hash = block.hash;
        blockchain.add_block(block.clone()).unwrap();
        consensus.add_block(block);
    }
    
    // Verify chain length
    assert_eq!(blockchain.chain_length(), 6); // genesis + 5 blocks
    
    // Verify ordering
    let ordered = consensus.get_ordered_blocks();
    assert_eq!(ordered.len(), 6);
}

