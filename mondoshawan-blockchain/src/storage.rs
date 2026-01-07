//! Storage layer for blockchain data

use crate::blockchain::Block;
use crate::types::Hash;
use sled::Db;
use std::path::Path;

/// Database handle
pub struct Database {
    db: Db,
}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> crate::error::BlockchainResult<Self> {
        let db = sled::open(path)
            .map_err(|e| crate::error::BlockchainError::Storage(format!("Failed to open database: {}", e)))?;
        Ok(Self { db })
    }
}

/// Block store
pub struct BlockStore<'a> {
    db: &'a Database,
}

impl<'a> BlockStore<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn put(&self, block: &Block) -> crate::error::BlockchainResult<()> {
        let key = block.hash;
        let value = bincode::serialize(block)?;
        self.db.db.insert(key, value)
            .map_err(|e| crate::error::BlockchainError::Storage(format!("Database error: {}", e)))?;
        Ok(())
    }

    pub fn get(&self, hash: &Hash) -> crate::error::BlockchainResult<Option<Block>> {
        match self.db.db.get(hash)
            .map_err(|e| crate::error::BlockchainError::Storage(format!("Database error: {}", e)))? {
            Some(value) => {
                let block: Block = bincode::deserialize(&value)?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }
}

/// State store
pub struct StateStore<'a> {
    db: &'a Database,
}

impl<'a> StateStore<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    /// Store balance for an address
    pub fn put_balance(&self, address: &crate::types::Address, balance: u128) -> crate::error::BlockchainResult<()> {
        let key = format!("balance:{}", hex::encode(address));
        let value = balance.to_le_bytes().to_vec();
        self.db.db.insert(key.as_bytes(), value)
            .map_err(|e| crate::error::BlockchainError::Storage(format!("Database error: {}", e)))?;
        Ok(())
    }

    /// Get balance for an address
    pub fn get_balance(&self, address: &crate::types::Address) -> crate::error::BlockchainResult<Option<u128>> {
        let key = format!("balance:{}", hex::encode(address));
        match self.db.db.get(key.as_bytes())
            .map_err(|e| crate::error::BlockchainError::Storage(format!("Database error: {}", e)))? {
            Some(value) => {
                if value.len() == 16 {
                    let mut bytes = [0u8; 16];
                    bytes.copy_from_slice(&value);
                    Ok(Some(u128::from_le_bytes(bytes)))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Store nonce for an address
    pub fn put_nonce(&self, address: &crate::types::Address, nonce: u64) -> Result<(), String> {
        let key = format!("nonce:{}", hex::encode(address));
        let value = nonce.to_le_bytes().to_vec();
        self.db.db.insert(key.as_bytes(), value).map_err(|e| format!("Database error: {}", e))?;
        Ok(())
    }

    /// Get nonce for an address
    pub fn get_nonce(&self, address: &crate::types::Address) -> Result<Option<u64>, String> {
        let key = format!("nonce:{}", hex::encode(address));
        match self.db.db.get(key.as_bytes()).map_err(|e| format!("Database error: {}", e))? {
            Some(value) => {
                if value.len() == 8 {
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(&value);
                    Ok(Some(u64::from_le_bytes(bytes)))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

