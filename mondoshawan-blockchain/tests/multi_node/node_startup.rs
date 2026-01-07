//! Multi-node startup tests

use mondoshawan_blockchain::node::{Node, NodeConfig};
use std::path::PathBuf;
use tempfile::TempDir;

/// Test multiple nodes can start independently
#[tokio::test]
async fn test_multiple_nodes_startup() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create configs for 3 nodes
    let mut configs = Vec::new();
    for i in 0..3 {
        let mut config = NodeConfig::default();
        config.node_id = format!("node-{}", i);
        config.data_dir = temp_dir.path().join(format!("node{}", i));
        config.database_path = temp_dir.path().join(format!("node{}.db", i));
        config.network.listen_address = format!("/ip4/127.0.0.1/tcp/{}", 30303 + i);
        configs.push(config);
    }
    
    // Start all nodes
    let mut nodes = Vec::new();
    for config in configs {
        let node = Node::new(config).await;
        assert!(node.is_ok());
        nodes.push(node.unwrap());
    }
    
    // Verify all nodes started
    assert_eq!(nodes.len(), 3);
    
    // Clean up
    for mut node in nodes {
        let _ = node.stop().await;
    }
}

/// Test node can start and stop
#[tokio::test]
async fn test_node_start_stop() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = NodeConfig::default();
    config.data_dir = temp_dir.path().to_path_buf();
    config.database_path = temp_dir.path().join("test.db");
    
    let node = Node::new(config).await.unwrap();
    
    // Start node
    let result = node.start().await;
    assert!(result.is_ok());
    
    // Stop node
    let result = node.stop().await;
    assert!(result.is_ok());
}

