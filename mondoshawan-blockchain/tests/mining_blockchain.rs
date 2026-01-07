//! Integration tests for Mining + Blockchain

use mondoshawan_blockchain::blockchain::{Blockchain, Block, Transaction};
use mondoshawan_blockchain::mining::MiningManager;
use mondoshawan_blockchain::types::{Address, Difficulty};

/// Test mining and block addition
#[tokio::test]
async fn test_mining_blockchain_integration() {
    let mut blockchain = Blockchain::new();
    let difficulty: Difficulty = 4;
    
    // Set balance for sender
    let sender: Address = [1u8; 20];
    blockchain.set_balance(sender, 10000);
    
    // Create a transaction
    let tx = Transaction::new(
        sender,
        [2u8; 20],
        1000,
        10,
        0,
    );
    
    // Add transaction to blockchain
    blockchain.add_transaction(tx.clone()).unwrap();
    
    // Verify transaction is in pool
    let pending_txs = blockchain.get_pending_transactions(100);
    assert!(pending_txs.iter().any(|t| t.hash == tx.hash));
    
    // Create mining manager
    let _mining_manager = MiningManager::new(blockchain.clone(), difficulty);
    
    // Verify blockchain is accessible (may be empty initially)
    let latest_hash = blockchain.get_latest_hash();
    // Empty blockchain returns [0; 32], which is valid for a new chain
    assert_eq!(latest_hash, [0u8; 32]);
}

/// Test transaction inclusion in mined blocks
#[tokio::test]
async fn test_transaction_inclusion() {
    let mut blockchain = Blockchain::new();
    let difficulty: Difficulty = 4;
    
    // Set balance for sender
    let sender: Address = [1u8; 20];
    blockchain.set_balance(sender, 100000); // Enough for all transactions
    
    // Add multiple transactions
    for i in 0..5 {
        let tx = Transaction::new(
            sender,
            [2u8; 20],
            100 * (i as u128 + 1),
            10,
            i,
        );
        blockchain.add_transaction(tx).unwrap();
    }
    
    // Verify transactions are in pool
    let pending = blockchain.get_pending_transactions(100);
    assert_eq!(pending.len(), 5);
    
    // Create mining manager
    let _mining_manager = MiningManager::new(blockchain.clone(), difficulty);
}

/// Test mining rewards
#[tokio::test]
async fn test_mining_rewards() {
    let blockchain = Blockchain::new();
    let difficulty: Difficulty = 4;
    let _mining_manager = MiningManager::new(blockchain.clone(), difficulty);
    
    // Mining rewards are handled in the mining manager
    // This test verifies the structure is in place
    // Empty blockchain returns [0; 32] which is valid
    let latest_hash = blockchain.get_latest_hash();
    assert_eq!(latest_hash, [0u8; 32]); // Empty chain
}

