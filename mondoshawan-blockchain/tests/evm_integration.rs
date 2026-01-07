//! EVM Integration Tests
//!
//! Tests for contract deployment and execution.

use mondoshawan_blockchain::blockchain::{Blockchain, Block, BlockHeader, Transaction};
use mondoshawan_blockchain::evm::{EvmState, EvmTransactionExecutor};
use mondoshawan_blockchain::types::{Address, StreamType};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_contract_deployment() {
    // Create blockchain with EVM enabled
    let blockchain = Arc::new(RwLock::new(Blockchain::with_evm(true)));
    
    // Create deployer address
    let deployer: Address = [1u8; 20];
    
    // Set initial balance
    {
        let mut bc = blockchain.write().await;
        bc.set_balance(deployer, 1_000_000_000_000_000_000); // 1 ETH
    }
    
    // Create contract deployment transaction
    // Simple contract bytecode (just a placeholder - in real tests, use actual Solidity bytecode)
    let contract_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0x52]; // PUSH1 0x00 PUSH1 0x00 MSTORE (minimal contract)
    
    let tx = Transaction::with_data(
        deployer,
        [0u8; 20], // Zero address for deployment
        0,
        1_000_000, // Fee
        0, // Nonce
        contract_bytecode,
        1_000_000, // Gas limit
    );
    
    // Execute transaction
    {
        let bc = blockchain.read().await;
        if let Some(executor) = bc.evm_executor() {
            let result = executor.execute_transaction(&tx).await;
            if let Err(e) = &result {
                eprintln!("Contract deployment error: {}", e);
            }
            assert!(result.is_ok(), "Contract deployment should succeed: {:?}", result.err());
            
            let exec_result = result.unwrap();
            if !exec_result.success {
                eprintln!("Execution failed: {:?}", exec_result.error);
            }
            assert!(exec_result.success, "Contract deployment should be successful: {:?}", exec_result.error);
            assert!(exec_result.contract_address.is_some(), "Should return contract address");
            
            // Verify gas was used
            assert!(exec_result.gas_used > 0, "Should consume gas");
        } else {
            panic!("EVM executor should be available");
        }
    }
}

#[tokio::test]
async fn test_regular_transaction() {
    // Create blockchain with EVM enabled
    let blockchain = Arc::new(RwLock::new(Blockchain::with_evm(true)));
    
    // Create addresses
    let sender: Address = [1u8; 20];
    let receiver: Address = [2u8; 20];
    
    // Set initial balance
    {
        let mut bc = blockchain.write().await;
        bc.set_balance(sender, 1_000_000_000_000_000_000); // 1 ETH
    }
    
    // Create regular transaction
    let tx = Transaction::new(
        sender,
        receiver,
        100_000_000_000_000_000, // 0.1 ETH
        1_000_000, // Fee
        0, // Nonce
    );
    
    // Execute transaction
    {
        let bc = blockchain.read().await;
        if let Some(executor) = bc.evm_executor() {
            let result = executor.execute_transaction(&tx).await;
            assert!(result.is_ok(), "Regular transaction should succeed");
            
            let exec_result = result.unwrap();
            assert!(exec_result.success, "Transaction should be successful");
            assert_eq!(exec_result.contract_address, None, "Should not have contract address");
            
            // Verify base gas was used
            assert_eq!(exec_result.gas_used, 21_000, "Should use base transaction cost");
        } else {
            panic!("EVM executor should be available");
        }
    }
}

#[tokio::test]
async fn test_gas_limit_validation() {
    use mondoshawan_blockchain::evm::{GasMeter, GasConfig};
    
    // Create gas config
    let config = GasConfig::default();
    
    // Create transaction with excessive gas limit
    let tx = Transaction::with_data(
        [1u8; 20],
        [0u8; 20],
        0,
        0,
        0,
        vec![],
        100_000_000, // Exceeds max_gas (30M)
    );
    
    // Should fail validation
    let result = GasMeter::validate_gas_limit(&tx, &config);
    assert!(result.is_err(), "Should reject excessive gas limit");
    
    // Create transaction with valid gas limit
    let tx_valid = Transaction::with_data(
        [1u8; 20],
        [0u8; 20],
        0,
        0,
        0,
        vec![],
        1_000_000, // Valid gas limit
    );
    
    // Should pass validation
    let result = GasMeter::validate_gas_limit(&tx_valid, &config);
    assert!(result.is_ok(), "Should accept valid gas limit");
}

#[tokio::test]
async fn test_gas_estimation() {
    use mondoshawan_blockchain::evm::{GasEstimator, GasConfig};
    
    let config = GasConfig::default();
    
    // Test regular transaction
    let tx = Transaction::new(
        [1u8; 20],
        [2u8; 20],
        100,
        0,
        0,
    );
    
    let estimated = GasEstimator::estimate_gas(&tx, &config);
    assert_eq!(estimated, 21_000, "Regular transaction should estimate base cost");
    
    // Test contract deployment
    let deploy_tx = Transaction::with_data(
        [1u8; 20],
        [0u8; 20],
        0,
        0,
        0,
        vec![0x60, 0x00, 0x60, 0x00, 0x52], // Some bytecode
        1_000_000,
    );
    
    let estimated_deploy = GasEstimator::estimate_gas(&deploy_tx, &config);
    assert!(estimated_deploy > 21_000, "Deployment should cost more than base");
    assert!(estimated_deploy <= config.max_gas, "Should not exceed max gas");
}

#[tokio::test]
async fn test_gas_meter() {
    use mondoshawan_blockchain::evm::{GasMeter, GasConfig};
    
    let config = GasConfig::default();
    let gas_limit = 100_000;
    
    let mut meter = GasMeter::new(config, gas_limit);
    
    // Consume some gas
    assert!(meter.consume_gas(50_000).is_ok(), "Should consume gas");
    assert_eq!(meter.gas_used(), 50_000, "Should track gas used");
    assert_eq!(meter.remaining_gas(), 50_000, "Should calculate remaining gas");
    
    // Try to exceed limit
    assert!(meter.consume_gas(60_000).is_err(), "Should reject exceeding limit");
    
    // Reset
    meter.reset();
    assert_eq!(meter.gas_used(), 0, "Should reset gas used");
}

#[tokio::test]
async fn test_block_with_contract_transaction() {
    // Create blockchain with EVM enabled
    let blockchain = Arc::new(RwLock::new(Blockchain::with_evm(true)));
    
    // Create deployer
    let deployer: Address = [1u8; 20];
    
    // Set initial balance
    {
        let mut bc = blockchain.write().await;
        bc.set_balance(deployer, 1_000_000_000_000_000_000);
    }
    
    // Create contract deployment transaction
    let contract_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0x52];
    let tx = Transaction::with_data(
        deployer,
        [0u8; 20],
        0,
        1_000_000,
        0,
        contract_bytecode,
        1_000_000,
    );
    
    // Execute EVM transaction
    {
        let mut bc = blockchain.write().await;
        let results = bc.execute_evm_transactions(&[tx.clone()], 1).await;
        assert!(results.is_ok(), "Should execute EVM transaction");
        
        let exec_results = results.unwrap();
        assert_eq!(exec_results.len(), 1, "Should have one result");
        assert!(exec_results[0].success, "Transaction should succeed");
    }
    
    // Create block with transaction
    let header = BlockHeader::new(
        [0; 32],
        0,
        StreamType::StreamA,
        4,
    );
    
    let block = Block::new(header, vec![tx], vec![]);
    
    // Add block
    {
        let mut bc = blockchain.write().await;
        let result = bc.add_block(block);
        assert!(result.is_ok(), "Should add block successfully");
    }
}

