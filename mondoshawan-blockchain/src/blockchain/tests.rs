//! Unit tests for blockchain module

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::{Blockchain, Block, BlockHeader, Transaction};
    use crate::types::{Address, StreamType};

    #[test]
    fn test_genesis_block() {
        let mut blockchain = Blockchain::new();
        let genesis_header = BlockHeader::new(vec![], 0, StreamType::StreamA, 4);
        let genesis = Block::new(genesis_header, vec![], vec![]);
        
        assert!(blockchain.add_block(genesis).is_ok());
        assert_eq!(blockchain.blocks.len(), 1);
        assert_eq!(blockchain.latest_block_number(), 0);
    }

    #[test]
    fn test_add_block_with_transaction() {
        let mut blockchain = Blockchain::new();
        
        // Create genesis
        let genesis_header = BlockHeader::new(vec![], 0, StreamType::StreamA, 4);
        let genesis = Block::new(genesis_header, vec![], vec![]);
        let genesis_hash = genesis.hash;
        blockchain.add_block(genesis).unwrap();
        
        // Set up sender with balance
        let sender: Address = [1u8; 20];
        let receiver: Address = [2u8; 20];
        blockchain.set_balance(sender, 1000);
        
        // Create transaction
        let tx = Transaction::new(sender, receiver, 100, 10, 0);
        
        // Create block with transaction
        let block_header = BlockHeader::new(vec![genesis_hash], 1, StreamType::StreamA, 4);
        let block = Block::new(block_header, vec![tx], vec![genesis_hash]);
        
        assert!(blockchain.add_block(block).is_ok());
        assert_eq!(blockchain.get_balance(sender), 890); // 1000 - 100 - 10
        assert_eq!(blockchain.get_balance(receiver), 100);
        assert_eq!(blockchain.get_nonce(sender), 1);
    }

    #[test]
    fn test_insufficient_balance() {
        let mut blockchain = Blockchain::new();
        
        let genesis_header = BlockHeader::new(vec![], 0, StreamType::StreamA, 4);
        let genesis = Block::new(genesis_header, vec![], vec![]);
        let genesis_hash = genesis.hash;
        blockchain.add_block(genesis).unwrap();
        
        let sender: Address = [1u8; 20];
        let receiver: Address = [2u8; 20];
        blockchain.set_balance(sender, 50); // Not enough for value + fee
        
        let tx = Transaction::new(sender, receiver, 100, 10, 0);
        let block_header = BlockHeader::new(vec![genesis_hash], 1, StreamType::StreamA, 4);
        let block = Block::new(block_header, vec![tx], vec![genesis_hash]);
        
        assert!(blockchain.add_block(block).is_err());
    }

    #[test]
    fn test_invalid_nonce() {
        let mut blockchain = Blockchain::new();
        
        let genesis_header = BlockHeader::new(vec![], 0, StreamType::StreamA, 4);
        let genesis = Block::new(genesis_header, vec![], vec![]);
        let genesis_hash = genesis.hash;
        blockchain.add_block(genesis).unwrap();
        
        let sender: Address = [1u8; 20];
        let receiver: Address = [2u8; 20];
        blockchain.set_balance(sender, 1000);
        
        // First transaction with nonce 0
        let tx1 = Transaction::new(sender, receiver, 100, 10, 0);
        let block1_header = BlockHeader::new(vec![genesis_hash], 1, StreamType::StreamA, 4);
        let block1 = Block::new(block1_header, vec![tx1], vec![genesis_hash]);
        blockchain.add_block(block1).unwrap();
        
        // Second transaction with nonce 0 (should fail)
        let tx2 = Transaction::new(sender, receiver, 100, 10, 0);
        let block2_header = BlockHeader::new(vec![genesis_hash], 2, StreamType::StreamA, 4);
        let block2 = Block::new(block2_header, vec![tx2], vec![genesis_hash]);
        assert!(blockchain.add_block(block2).is_err());
    }

    #[test]
    fn test_duplicate_block() {
        let mut blockchain = Blockchain::new();
        
        let genesis_header = BlockHeader::new(vec![], 0, StreamType::StreamA, 4);
        let genesis = Block::new(genesis_header, vec![], vec![]);
        let genesis_hash = genesis.hash;
        
        blockchain.add_block(genesis.clone()).unwrap();
        
        // Try to add same block again
        assert!(blockchain.add_block(genesis).is_err());
    }
}

