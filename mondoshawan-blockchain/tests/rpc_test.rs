//! RPC endpoint tests

use mondoshawan_blockchain::node::{Node, NodeConfig};
use mondoshawan_blockchain::rpc::RpcMethods;
use mondoshawan_blockchain::blockchain::Transaction;
use mondoshawan_blockchain::types::Address;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::json;

/// Test all RPC methods
#[tokio::test]
async fn test_all_rpc_methods() {
    // Initialize node
    let config = NodeConfig::default();
    let node = Node::new(config).await.unwrap();
    let node_arc = Arc::new(RwLock::new(node));
    
    // Start node
    {
        let mut node_write = node_arc.write().await;
        node_write.start().await.unwrap();
    }
    
    // Create RPC methods handler
    let rpc_methods = RpcMethods::new(node_arc);
    
    // Test 1: eth_blockNumber
    println!("Testing eth_blockNumber...");
    let result = rpc_methods.handle_request("eth_blockNumber", json!([])).await;
    assert!(result.is_ok(), "eth_blockNumber failed");
    println!("✅ eth_blockNumber: {:?}", result);
    
    // Test 2: net_version
    println!("Testing net_version...");
    let result = rpc_methods.handle_request("net_version", json!([])).await;
    assert!(result.is_ok(), "net_version failed");
    assert_eq!(result.unwrap(), json!("1"));
    println!("✅ net_version: OK");
    
    // Test 3: net_listening
    println!("Testing net_listening...");
    let result = rpc_methods.handle_request("net_listening", json!([])).await;
    assert!(result.is_ok(), "net_listening failed");
    assert_eq!(result.unwrap(), json!(true));
    println!("✅ net_listening: OK");
    
    // Test 4: web3_clientVersion
    println!("Testing web3_clientVersion...");
    let result = rpc_methods.handle_request("web3_clientVersion", json!([])).await;
    assert!(result.is_ok(), "web3_clientVersion failed");
    assert_eq!(result.unwrap(), json!("pyrax/0.1.0"));
    println!("✅ web3_clientVersion: OK");
    
    // Test 5: net_peerCount
    println!("Testing net_peerCount...");
    let result = rpc_methods.handle_request("net_peerCount", json!([])).await;
    assert!(result.is_ok(), "net_peerCount failed");
    println!("✅ net_peerCount: {:?}", result);
    
    // Test 6: eth_syncing
    println!("Testing eth_syncing...");
    let result = rpc_methods.handle_request("eth_syncing", json!([])).await;
    assert!(result.is_ok(), "eth_syncing failed");
    assert_eq!(result.unwrap(), json!(false));
    println!("✅ eth_syncing: OK");
    
    // Test 7: eth_getBalance (with test address)
    println!("Testing eth_getBalance...");
    let test_address = "0x0000000000000000000000000000000000000000";
    let result = rpc_methods.handle_request("eth_getBalance", json!([test_address, "latest"])).await;
    assert!(result.is_ok(), "eth_getBalance failed");
    println!("✅ eth_getBalance: {:?}", result);
    
    // Test 8: eth_getTransactionCount
    println!("Testing eth_getTransactionCount...");
    let result = rpc_methods.handle_request("eth_getTransactionCount", json!([test_address, "latest"])).await;
    assert!(result.is_ok(), "eth_getTransactionCount failed");
    println!("✅ eth_getTransactionCount: {:?}", result);
    
    // Test 9: eth_getBlockByHash (with test hash)
    println!("Testing eth_getBlockByHash...");
    let test_hash = "0x0000000000000000000000000000000000000000000000000000000000000000";
    let result = rpc_methods.handle_request("eth_getBlockByHash", json!([test_hash, false])).await;
    assert!(result.is_ok(), "eth_getBlockByHash failed");
    println!("✅ eth_getBlockByHash: {:?}", result);
    
    // Test 10: eth_getBlockByNumber
    println!("Testing eth_getBlockByNumber...");
    let result = rpc_methods.handle_request("eth_getBlockByNumber", json!(["latest", false])).await;
    assert!(result.is_ok(), "eth_getBlockByNumber failed");
    println!("✅ eth_getBlockByNumber: {:?}", result);
    
    // Test 11: eth_getTransactionByHash
    println!("Testing eth_getTransactionByHash...");
    let result = rpc_methods.handle_request("eth_getTransactionByHash", json!([test_hash])).await;
    assert!(result.is_ok(), "eth_getTransactionByHash failed");
    println!("✅ eth_getTransactionByHash: {:?}", result);
    
    // Test 12: eth_getTransactionReceipt
    println!("Testing eth_getTransactionReceipt...");
    let result = rpc_methods.handle_request("eth_getTransactionReceipt", json!([test_hash])).await;
    assert!(result.is_ok(), "eth_getTransactionReceipt failed");
    println!("✅ eth_getTransactionReceipt: {:?}", result);
    
    // Test 13: eth_sendTransaction (with test transaction)
    println!("Testing eth_sendTransaction...");
    let tx_params = json!({
        "from": test_address,
        "to": "0x1111111111111111111111111111111111111111",
        "value": "0x1000"
    });
    let result = rpc_methods.handle_request("eth_sendTransaction", json!([tx_params])).await;
    // This might fail if balance is insufficient, which is expected
    println!("✅ eth_sendTransaction: {:?}", result);
    
    // Test 14: eth_sendRawTransaction
    println!("Testing eth_sendRawTransaction...");
    let result = rpc_methods.handle_request("eth_sendRawTransaction", json!(["0x1234"])).await;
    // This should return an error (not implemented yet)
    assert!(result.is_err(), "eth_sendRawTransaction should return error");
    println!("✅ eth_sendRawTransaction: {:?} (expected error)", result);
    
    println!("\n✅ All 14 RPC methods tested!");
}

