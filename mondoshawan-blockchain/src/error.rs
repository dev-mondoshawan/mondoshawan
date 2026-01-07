//! Error types for the blockchain
//! 
//! Provides structured error handling with custom error types
//! for better error reporting and debugging.

use thiserror::Error;

/// Main blockchain error type
#[derive(Error, Debug, Clone)]
pub enum BlockchainError {
    #[error("Invalid block: {0}")]
    InvalidBlock(String),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Storage error: {0}")]
    Storage(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("EVM error: {0}")]
    Evm(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for BlockchainError {
    fn from(err: std::io::Error) -> Self {
        BlockchainError::Io(err.to_string())
    }
}

impl From<bincode::Error> for BlockchainError {
    fn from(err: bincode::Error) -> Self {
        BlockchainError::Serialization(err.to_string())
    }
}

impl From<String> for BlockchainError {
    fn from(err: String) -> Self {
        BlockchainError::Unknown(err)
    }
}

/// Result type alias for blockchain operations
pub type BlockchainResult<T> = Result<T, BlockchainError>;
