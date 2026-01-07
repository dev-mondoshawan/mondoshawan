//! Configuration management
//! 
//! Provides configuration loading and validation for the node.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Node configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Data directory for blockchain storage
    pub data_dir: PathBuf,
    
    /// Network port for P2P communication
    pub port: u16,
    
    /// JSON-RPC API port
    pub rpc_port: u16,
    
    /// Miner address (receives block rewards)
    pub miner_address: [u8; 20],
    
    /// Enable EVM
    pub evm_enabled: bool,
    
    /// Maximum peers to connect to
    pub max_peers: u32,
    
    /// Bootstrap peers (initial peers to connect to)
    pub bootstrap_peers: Vec<String>,
    
    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,
    
    /// Enable metrics
    pub metrics_enabled: bool,
    
    /// Metrics port
    pub metrics_port: u16,
    
    /// RPC rate limit (requests per second)
    pub rpc_rate_limit: u32,
    
    /// Maximum transaction pool size
    pub max_tx_pool_size: usize,
    
    /// Maximum block size (bytes)
    pub max_block_size: usize,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("data"),
            port: 8080,
            rpc_port: 8545,
            miner_address: [1u8; 20], // Default miner address
            evm_enabled: true,
            max_peers: 50,
            bootstrap_peers: vec![],
            log_level: "info".to_string(),
            metrics_enabled: false,
            metrics_port: 9090,
            rpc_rate_limit: 100,
            max_tx_pool_size: 10_000,
            max_block_size: 10_000_000, // 10MB
        }
    }
}

impl NodeConfig {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, String> {
        use std::fs;
        
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        let config: NodeConfig = toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        use std::fs;
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        Ok(())
    }
    
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.port == 0 {
            return Err("Port cannot be 0".to_string());
        }
        
        if self.rpc_port == 0 {
            return Err("RPC port cannot be 0".to_string());
        }
        
        if self.port == self.rpc_port {
            return Err("Port and RPC port must be different".to_string());
        }
        
        if self.max_peers == 0 {
            return Err("Max peers must be greater than 0".to_string());
        }
        
        if self.max_tx_pool_size == 0 {
            return Err("Max transaction pool size must be greater than 0".to_string());
        }
        
        if self.max_block_size == 0 {
            return Err("Max block size must be greater than 0".to_string());
        }
        
        Ok(())
    }
}
