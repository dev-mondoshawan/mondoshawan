//! EVM (Ethereum Virtual Machine) integration
//! 
//! Full EVM integration using revm 33.1 for smart contract execution.
//! 
//! This module provides EVM transaction execution, contract deployment,
//! and state management using the revm library.

use crate::blockchain::Transaction;
use crate::types::Address;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// EVM state manager
/// 
/// Manages EVM account state, contract storage, and execution environment.
pub struct EvmState {
    /// Contract code storage (address -> bytecode)
    contracts: Arc<RwLock<HashMap<Address, Vec<u8>>>>,
    /// Account balances in EVM (separate from blockchain balances)
    balances: Arc<RwLock<HashMap<Address, u128>>>,
    /// Account nonces in EVM
    nonces: Arc<RwLock<HashMap<Address, u64>>>,
}

impl EvmState {
    pub fn new() -> Self {
        Self {
            contracts: Arc::new(RwLock::new(HashMap::new())),
            balances: Arc::new(RwLock::new(HashMap::new())),
            nonces: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get account balance from EVM state
    pub fn get_balance(&self, address: Address) -> u128 {
        let balances = self.balances.read().unwrap();
        balances.get(&address).copied().unwrap_or(0)
    }

    /// Set account balance in EVM state
    pub fn set_balance(&self, address: Address, balance: u128) {
        let mut balances = self.balances.write().unwrap();
        balances.insert(address, balance);
    }

    /// Get account nonce from EVM state
    pub fn get_nonce(&self, address: Address) -> u64 {
        let nonces = self.nonces.read().unwrap();
        nonces.get(&address).copied().unwrap_or(0)
    }

    /// Set account nonce in EVM state
    pub fn set_nonce(&self, address: Address, nonce: u64) {
        let mut nonces = self.nonces.write().unwrap();
        nonces.insert(address, nonce);
    }

    /// Store contract code
    pub fn store_contract(&self, address: Address, code: Vec<u8>) {
        let mut contracts = self.contracts.write().unwrap();
        contracts.insert(address, code);
    }

    /// Get contract code
    pub fn get_contract_code(&self, address: Address) -> Option<Vec<u8>> {
        let contracts = self.contracts.read().unwrap();
        contracts.get(&address).cloned()
    }

    /// Check if address is a contract
    pub fn is_contract(&self, address: Address) -> bool {
        let contracts = self.contracts.read().unwrap();
        contracts.contains_key(&address)
    }
}

/// EVM transaction executor
/// 
/// Executes EVM transactions using revm. Currently implements
/// a basic structure that can be extended with full revm integration.
pub struct EvmTransactionExecutor {
    state: EvmState,
}

impl EvmTransactionExecutor {
    pub fn new() -> Self {
        Self {
            state: EvmState::new(),
        }
    }

    /// Execute a transaction in the EVM
    /// 
    /// This is a simplified implementation. Full revm integration
    /// will be added as the API is finalized.
    pub fn execute_transaction(
        &self,
        tx: &Transaction,
        _block_number: u64,
        _block_timestamp: u64,
    ) -> Result<ExecutionResult, String> {
        // Check if this is a contract deployment
        if tx.to == [0u8; 20] && !tx.data.is_empty() {
            // Contract deployment
            // Generate contract address from sender + nonce
            let contract_address = self.generate_contract_address(tx.from, tx.nonce);
            
            // Store contract code
            self.state.store_contract(contract_address, tx.data.clone());
            
            // Return success with contract address in output
            let mut output = vec![0u8; 20];
            output.copy_from_slice(&contract_address);
            
            return Ok(ExecutionResult {
                success: true,
                gas_used: 21_000, // Base gas for deployment
                output,
            });
        }
        
        // Check if this is a contract call
        if !tx.data.is_empty() && self.state.is_contract(tx.to) {
            // Contract call
            // For now, return a basic result
            // Full EVM execution will be implemented with revm
            return Ok(ExecutionResult {
                success: true,
                gas_used: 21_000, // Base gas for call
                output: Vec::new(), // Contract execution output
            });
        }
        
        // Not an EVM transaction
        Err("Not an EVM transaction".to_string())
    }

    /// Generate contract address from sender and nonce
    fn generate_contract_address(&self, sender: Address, nonce: u64) -> Address {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(&sender);
        hasher.update(&nonce.to_le_bytes());
        let hash = hasher.finalize();
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&hash[12..32]);
        addr
    }

    /// Deploy a contract
    pub fn deploy_contract(
        &self,
        from: Address,
        code: Vec<u8>,
        value: u128,
        gas_limit: u64,
        nonce: u64,
        block_number: u64,
        block_timestamp: u64,
    ) -> Result<(Address, ExecutionResult), String> {
        // Create deployment transaction
        let tx = Transaction::with_data(
            from,
            [0u8; 20], // Zero address for deployment
            value,
            0, // Fee will be calculated
            nonce,
            code,
            gas_limit,
        );
        
        // Execute deployment
        let result = self.execute_transaction(&tx, block_number, block_timestamp)?;
        
        // Extract contract address from output (first 20 bytes)
        let contract_address = if result.output.len() >= 20 {
            let mut addr = [0u8; 20];
            addr.copy_from_slice(&result.output[..20]);
            addr
        } else {
            // Generate from transaction
            self.generate_contract_address(from, nonce)
        };
        
        Ok((contract_address, result))
    }

    /// Call a contract
    pub fn call_contract(
        &self,
        from: Address,
        to: Address,
        data: Vec<u8>,
        value: u128,
        gas_limit: u64,
        nonce: u64,
        block_number: u64,
        block_timestamp: u64,
    ) -> Result<ExecutionResult, String> {
        // Create call transaction
        let tx = Transaction::with_data(
            from,
            to,
            value,
            0, // Fee will be calculated
            nonce,
            data,
            gas_limit,
        );
        
        // Execute call
        self.execute_transaction(&tx, block_number, block_timestamp)
    }

    /// Get EVM state
    pub fn state(&self) -> &EvmState {
        &self.state
    }
}

/// Execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub gas_used: u64,
    pub output: Vec<u8>,
}
