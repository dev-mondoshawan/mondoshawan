//! Mondoshawan Blockchain Node
//! 
//! A working blockchain node with TriStream mining
//! 
//! Features:
//! - Real-time console dashboard
//! - HTTP API server (port 8080)
//! - Web dashboard support
//!
//! Copyright (c) 2026 Mondoshawan Protocol
//! Licensed under the MIT License (see LICENSE file)

use mondoshawan_blockchain::node::Node;
use mondoshawan_blockchain::node::NodeConfig;
use mondoshawan_blockchain::blockchain::Transaction;
use mondoshawan_blockchain::types::Address;
use std::sync::Arc;
use tokio::time::{sleep, Duration, interval};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║      Mondoshawan Protocol (MSHW) - TriStream Mining        ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // Create node with default config
    let miner_address: Address = [1u8; 20];
    let mut config = NodeConfig {
        miner_address,
        ..Default::default()
    };
    
    // Parse command line arguments
    // Usage: node [p2p_port] [rpc_port] [--data-dir <path>] [peer_addr1] [peer_addr2] ...
    let args: Vec<String> = std::env::args().collect();
    let mut peer_start_idx = 3;
    
    // Parse P2P port
    if args.len() > 1 {
        if let Ok(port) = args[1].parse::<u16>() {
            config.port = port;
            println!("Using P2P port: {}", port);
        }
    }
    
    // Parse RPC port
    if args.len() > 2 {
        if let Ok(rpc_port) = args[2].parse::<u16>() {
            config.rpc_port = rpc_port;
            println!("Using RPC port: {}", rpc_port);
        }
    }
    
    // Parse --data-dir flag
    if args.len() > 3 && args[3] == "--data-dir" {
        if args.len() > 4 {
            config.data_dir = args[4].clone();
            println!("Using data directory: {}", config.data_dir);
            peer_start_idx = 5;
        } else {
            eprintln!("Error: --data-dir requires a path argument");
            std::process::exit(1);
        }
    }
    
    let node = Arc::new(Node::new(config.clone()));
    
    // Start the node
    node.start().await?;
    
    // Connect to peers if provided
    println!("🔍 Checking for peer addresses starting at index {}...", peer_start_idx);
    println!("🔍 Total args: {}", args.len());
    
    for (idx, arg) in args.iter().enumerate().skip(peer_start_idx) {
        println!("🔍 Arg[{}]: {}", idx, arg);
        if let Ok(peer_addr) = arg.parse::<std::net::SocketAddr>() {
            println!("🔗 Connecting to peer: {}", peer_addr);
            if let Err(e) = node.connect_peer(peer_addr).await {
                eprintln!("⚠️  Failed to connect to {}: {}", peer_addr, e);
            }
        } else {
            println!("⚠️  Could not parse '{}' as socket address", arg);
        }
    }
    
    // Generate some test transactions
    println!("\n📝 Generating test transactions...");
    let mining_manager = node.mining_manager();
    
    // Create some test addresses
    let alice: Address = [2u8; 20];
    let bob: Address = [3u8; 20];
    
    // Give Alice some initial balance (simulate)
    {
        let blockchain = node.blockchain();
        let mut bc = blockchain.write().await;
        bc.set_balance(alice, 1000_000_000_000_000_000_000).unwrap_or_else(|e| {
            eprintln!("Warning: Failed to set balance: {}", e);
        }); // 1000 tokens
        println!("   Alice balance: 1000 tokens");
    }
    
    // Add some transactions to the pool
    for i in 0..50 {
        let tx = Transaction::new(
            alice,
            bob,
            10_000_000_000_000_000, // 0.01 tokens
            1_000_000_000_000_000,   // 0.001 token fee
            i,
        );
        if mining_manager.add_transaction(tx).await.is_ok() {
            if i % 10 == 0 {
                println!("   Added transaction {}", i + 1);
            }
        }
    }
    
    println!("\n✅ Node is running!");
    println!("   Press Ctrl+C to stop\n");
    
    // Setup graceful shutdown
    let node_shutdown = node.clone();
    tokio::spawn(async move {
        let _ = signal::ctrl_c().await;
        println!("\n\n🛑 Shutdown signal received...");
        if let Err(e) = (*node_shutdown).shutdown().await {
            eprintln!("⚠️  Error during shutdown: {}", e);
        }
        std::process::exit(0);
    });
    
    // Track mining stats
    let stream_a_blocks = Arc::new(AtomicU64::new(0));
    let stream_b_blocks = Arc::new(AtomicU64::new(0));
    let stream_c_blocks = Arc::new(AtomicU64::new(0));
    
    // Start HTTP API server (disabled for multi-node - use RPC instead)
    // let blockchain_api = node.blockchain();
    // let mining_manager_api = node.mining_manager();
    // let stream_a_count = stream_a_blocks.clone();
    // let stream_b_count = stream_b_blocks.clone();
    // let stream_c_count = stream_c_blocks.clone();
    // 
    // tokio::spawn(async move {
    //     start_api_server(blockchain_api, mining_manager_api, stream_a_count, stream_b_count, stream_c_count).await;
    // });
    
    // println!("🌐 HTTP API server started on http://localhost:8081");
    println!("🔌 RPC API available on http://127.0.0.1:{}", config.rpc_port);
    println!("📊 Web Dashboard: Open mondoshawan-explorer-frontend/index.html in browser\n");
    sleep(Duration::from_secs(1)).await;
    
    // Real-time console dashboard
    let blockchain_dashboard = node.blockchain();
    let mut stats_interval = interval(Duration::from_secs(2));
    
    loop {
        stats_interval.tick().await;
        
        let blockchain = blockchain_dashboard.read().await;
        let latest = blockchain.latest_block_number();
        let tx_count = blockchain.transaction_count();
        let miner_balance = blockchain.get_balance(miner_address);
        let dag_stats = blockchain.get_dag_stats();
        let tps = blockchain.get_tps(60);
        let stream_a = stream_a_blocks.load(Ordering::Relaxed);
        let stream_b = stream_b_blocks.load(Ordering::Relaxed);
        let stream_c = stream_c_blocks.load(Ordering::Relaxed);
        
        // Update counters based on block numbers (simplified)
        // In real implementation, we'd track this in mining.rs
        drop(blockchain);
        
        // Show dashboard
        show_dashboard(latest, tx_count, miner_balance, stream_a, stream_b, stream_c, dag_stats, tps);
    }
}

/// Show real-time dashboard
fn show_dashboard(
    blocks: u64, 
    txs: usize, 
    miner_balance: u128, 
    stream_a: u64, 
    stream_b: u64, 
    stream_c: u64,
    dag_stats: mondoshawan_blockchain::consensus::DAGStats,
    tps: f64,
) {
    // Clear screen (works on most terminals)
    print!("\x1B[2J\x1B[1;1H");
    
    let blue_ratio = if dag_stats.total_blocks > 0 {
        (dag_stats.blue_blocks as f64 / dag_stats.total_blocks as f64) * 100.0
    } else {
        0.0
    };
    
    println!("╔═══════════════════════════════════════════════════════════════════════════════╗");
    println!("║              Mondoshawan Blockchain - Mining Dashboard                       ║");
    println!("╠═══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  📊 Network Stats                                                             ║");
    println!("║     Total Blocks: {:<60} ║", blocks + 1);
    println!("║     Total Transactions: {:<56} ║", txs);
    println!("║     Miner Balance: {:<58} MSHW ║", miner_balance / 1_000_000_000_000_000_000);
    println!("╠═══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  ⛏️  Mining Streams                                                            ║");
    println!("║     Stream A (ASIC):     {:<4} blocks | 50 MSHW/block | 10s blocks    ║", stream_a);
    println!("║     Stream B (CPU/GPU): {:<4} blocks | 25 MSHW/block | 1s blocks     ║", stream_b);
    println!("║     Stream C (ZK):       {:<4} blocks | Fees only      | 100ms blocks   ║", stream_c);
    println!("╠═══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  🔷 GhostDAG Consensus                                                         ║");
    println!("║     Blue Blocks: {:<4} | Red Blocks: {:<4} | Blue Ratio: {:.1}%        ║", 
             dag_stats.blue_blocks, dag_stats.red_blocks, blue_ratio);
    println!("║     TPS (60s): {:<62.2} ║", tps);
    println!("║     Avg Block Size: {:<4} bytes | Avg Txs/Block: {:.1}              ║",
             dag_stats.avg_block_size, dag_stats.avg_txs_per_block);
    println!("╠═══════════════════════════════════════════════════════════════════════════════╣");
    println!("║  🔌 Press Ctrl+C to stop mining                                                 ║");
    println!("║  📊 Web Dashboard: Open mondoshawan-explorer-frontend/index.html in browser          ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════╝");
}

/// Start HTTP API server
async fn start_api_server(
    blockchain: Arc<tokio::sync::RwLock<mondoshawan_blockchain::blockchain::Blockchain>>,
    _mining_manager: Arc<mondoshawan_blockchain::mining::MiningManager>,
    _stream_a: Arc<std::sync::atomic::AtomicU64>,
    _stream_b: Arc<std::sync::atomic::AtomicU64>,
    _stream_c: Arc<std::sync::atomic::AtomicU64>,
) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    
    // Use port 8081 for HTTP API (8080 is used by P2P network)
    let listener = match TcpListener::bind("127.0.0.1:8081").await {
        Ok(l) => {
            println!("✅ HTTP API server listening on http://127.0.0.1:8081");
            l
        },
        Err(e) => {
            eprintln!("❌ Failed to start API server on port 8081: {}", e);
            return;
        }
    };
    
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let blockchain_clone = blockchain.clone();
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                if let Ok(n) = stream.read(&mut buffer).await {
                    let request = String::from_utf8_lossy(&buffer[..n]);
                    
                    let response = if request.starts_with("GET /api/stats/network") {
                        let bc = blockchain_clone.read().await;
                        let latest = bc.latest_block_number();
                        let tx_count = bc.transaction_count();
                        let dag_stats = bc.get_dag_stats();
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{{\"total_blocks\":{},\"total_transactions\":{},\"peers_connected\":0}}",
                            latest + 1,
                            tx_count
                        )
                    } else if request.starts_with("GET /api/stats/chain") {
                        let bc = blockchain_clone.read().await;
                        let latest = bc.latest_block_number();
                        let tx_count = bc.transaction_count();
                        let dag_stats = bc.get_dag_stats();
                        let tps = bc.get_tps(60);
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{{\"blocks\":{},\"transactions\":{},\"tps\":{:.2},\"dag\":{{\"blue_blocks\":{},\"red_blocks\":{},\"total_blocks\":{}}}}}",
                            latest + 1,
                            tx_count,
                            tps,
                            dag_stats.blue_blocks,
                            dag_stats.red_blocks,
                            dag_stats.total_blocks
                        )
                    } else if request.starts_with("GET /api/stats") {
                        let bc = blockchain_clone.read().await;
                        let latest = bc.latest_block_number();
                        let tx_count = bc.transaction_count();
                        let miner_balance = bc.get_balance([1u8; 20]);
                        let dag_stats = bc.get_dag_stats();
                        let tps = bc.get_tps(60);
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n{{\"blocks\":{},\"transactions\":{},\"miner_balance\":{},\"dag\":{{\"blue_blocks\":{},\"red_blocks\":{},\"total_blocks\":{},\"tps\":{:.2}}}}}",
                            latest + 1,
                            tx_count,
                            miner_balance / 1_000_000_000_000_000_000,
                            dag_stats.blue_blocks,
                            dag_stats.red_blocks,
                            dag_stats.total_blocks,
                            tps
                        )
                    } else if request.starts_with("GET /api/blocks/recent") {
                        let bc = blockchain_clone.read().await;
                        let blocks = bc.get_blocks();
                        let recent: Vec<_> = blocks.iter().rev().take(10).map(|b| {
                            format!(
                                "{{\"hash\":\"{}\",\"number\":{},\"timestamp\":{},\"transaction_count\":{}}}",
                                hex::encode(&b.hash),
                                b.header.block_number,
                                b.header.timestamp,
                                b.transactions.len()
                            )
                        }).collect();
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n[{}]",
                            recent.join(",")
                        )
                    } else if request.starts_with("GET /api/transactions/recent") {
                        let bc = blockchain_clone.read().await;
                        let blocks = bc.get_blocks();
                        let mut recent_txs = Vec::new();
                        for block in blocks.iter().rev().take(10) {
                            for tx in &block.transactions {
                                recent_txs.push(format!(
                                    "{{\"hash\":\"{}\",\"from\":\"{}\",\"to\":\"{}\",\"value\":{}}}",
                                    hex::encode(&tx.hash),
                                    hex::encode(&tx.from),
                                    hex::encode(&tx.to),
                                    tx.value
                                ));
                            }
                        }
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\n\r\n[{}]",
                            recent_txs.iter().take(20).map(|s| s.as_str()).collect::<Vec<_>>().join(",")
                        )
                    } else {
                        "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
                    };
                    
                    let _ = stream.write_all(response.as_bytes()).await;
                }
            });
        }
    }
}
