//! Integration tests for Storage + Blockchain

use mondoshawan_blockchain::blockchain::{Blockchain, Block};
use mondoshawan_blockchain::storage::{Database, BlockStore, StateStore};
use mondoshawan_blockchain::types::Hash;
use tempfile::TempDir;

/// Test block persistence
#[tokio::test]
async fn test_block_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Create database
    let database = Database::open(&db_path).unwrap();
    let block_store = BlockStore::new(&database);
    
    // Create blockchain and add block
    use mondoshawan_blockchain::blockchain::BlockHeader;
    use mondoshawan_blockchain::types::StreamType;
    
    let mut blockchain = Blockchain::new();
    let genesis_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let genesis = Block::new(genesis_header, vec![], vec![]);
    let genesis_hash = genesis.hash;
    
    blockchain.add_block(genesis.clone()).unwrap();
    
    // Store block in database
    block_store.put_block(&genesis).unwrap();
    
    // Retrieve block
    let retrieved = block_store.get_block(&genesis_hash).unwrap();
    assert!(retrieved.is_some());
    let retrieved_block = retrieved.unwrap();
    assert_eq!(retrieved_block.hash, genesis_hash);
}

/// Test state persistence
#[tokio::test]
async fn test_state_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Create database
    let database = Database::open(&db_path).unwrap();
    let state_store = StateStore::new(&database);
    
    // Store latest block hash
    let test_hash: Hash = [1u8; 32];
    state_store.put_latest_hash(&test_hash).unwrap();
    
    // Retrieve latest block hash
    let retrieved = state_store.get_latest_hash().unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), test_hash);
}

/// Test chain length persistence
#[tokio::test]
async fn test_chain_length_persistence() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Create database
    let database = Database::open(&db_path).unwrap();
    let state_store = StateStore::new(&database);
    
    // Store chain length
    state_store.put_chain_length(100).unwrap();
    
    // Retrieve chain length
    let retrieved = state_store.get_chain_length().unwrap();
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), 100);
}

/// Test database recovery
#[tokio::test]
async fn test_database_recovery() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test.db");
    
    // Create database and add block
    use mondoshawan_blockchain::blockchain::BlockHeader;
    use mondoshawan_blockchain::types::StreamType;
    
    let genesis_header = BlockHeader::new([0; 32], 0, StreamType::StreamA, 4);
    let genesis = Block::new(genesis_header, vec![], vec![]);
    let genesis_hash = genesis.hash;
    
    {
        let database = Database::open(&db_path).unwrap();
        let block_store = BlockStore::new(&database);
        block_store.put_block(&genesis).unwrap();
    }
    
    // Reopen database and verify block is still there
    let database = Database::open(&db_path).unwrap();
    let block_store = BlockStore::new(&database);
    
    let retrieved = block_store.get_block(&genesis_hash).unwrap();
    assert!(retrieved.is_some());
}

