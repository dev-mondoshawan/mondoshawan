//! Node implementation

pub mod pool;

use crate::blockchain::Blockchain;
use crate::mining::MiningManager;
use crate::network::NetworkManager;
use crate::rpc::RpcServer;
use crate::sharding::{ShardManager, ShardConfig, AssignmentStrategy};
use crate::storage::Database;
use crate::types::Address;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;

/// Node configuration
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub port: u16,
    pub rpc_port: u16,
    pub miner_address: Address,
    pub data_dir: String,
    /// Enable sharding
    pub enable_sharding: bool,
    /// Number of shards (if sharding enabled)
    pub shard_count: usize,
    /// Enable Verkle tree (stateless mode)
    pub enable_verkle: bool,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            rpc_port: 8545,
            miner_address: [1u8; 20], // Default miner address
            data_dir: "data".to_string(),
            enable_sharding: false, // Disabled by default
            shard_count: 10, // 10 shards if enabled
            enable_verkle: false, // Disabled by default
        }
    }
}

/// Node
pub struct Node {
    config: NodeConfig,
    blockchain: Arc<RwLock<Blockchain>>,
    mining_manager: Arc<MiningManager>,
    network_manager: Arc<NetworkManager>,
    rpc_server: Arc<RpcServer>,
    shard_manager: Option<Arc<ShardManager>>,
    metrics: Option<crate::metrics::MetricsHandle>,
    shutdown_signal: Arc<tokio::sync::Notify>,
}

impl Node {
    pub fn new(config: NodeConfig) -> Self {
        // Create or open database
        let database = match Database::open(&config.data_dir) {
            Ok(db) => {
                println!("📦 Opened database at: {}", config.data_dir);
                Some(Arc::new(db))
            }
            Err(e) => {
                eprintln!("⚠️  Failed to open database: {}. Using in-memory mode.", e);
                None
            }
        };
        
        // Create metrics collector
        let shard_count_for_metrics = if config.enable_sharding { config.shard_count } else { 0 };
        let metrics = match crate::metrics::create_metrics(shard_count_for_metrics) {
            Ok(m) => {
                println!("📊 Metrics collection enabled");
                Some(m)
            }
            Err(e) => {
                eprintln!("⚠️  Failed to create metrics: {}. Metrics disabled.", e);
                None
            }
        };
        
        // Create shard manager if enabled (needed before blockchain creation)
        let shard_manager: Option<Arc<ShardManager>> = if config.enable_sharding && config.shard_count > 0 {
            let shard_config = ShardConfig {
                shard_count: config.shard_count,
                enable_cross_shard: true,
                assignment_strategy: AssignmentStrategy::ConsistentHashing,
            };
            println!("🔷 Sharding enabled with {} shards", config.shard_count);
            Some(Arc::new(ShardManager::new(shard_config)))
        } else {
            None
        };
        
        // Create blockchain with or without storage and Verkle
        let mut blockchain = if config.enable_verkle {
            // Verkle mode (stateless)
            if let Some(db) = database {
                match Blockchain::with_storage_and_verkle(db) {
                    Ok(mut bc) => {
                        bc.evm_enabled = true;
                        bc.evm_executor = Some(crate::evm::EvmTransactionExecutor::new());
                        println!("✅ Loaded blockchain state from storage");
                        println!("✅ Verkle tree enabled (stateless mode)");
                        println!("✅ EVM enabled for smart contract support");
                        bc
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to load from storage: {}. Using in-memory Verkle mode.", e);
                        let mut bc = Blockchain::with_verkle();
                        bc.evm_enabled = true;
                        bc.evm_executor = Some(crate::evm::EvmTransactionExecutor::new());
                        println!("✅ Verkle tree enabled (stateless mode)");
                        println!("✅ EVM enabled for smart contract support");
                        bc
                    }
                }
            } else {
                let mut bc = Blockchain::with_verkle();
                bc.evm_enabled = true;
                bc.evm_executor = Some(crate::evm::EvmTransactionExecutor::new());
                println!("✅ Verkle tree enabled (stateless mode)");
                println!("✅ EVM enabled for smart contract support");
                bc
            }
        } else {
            // Traditional mode (with storage)
            if let Some(db) = database {
                match Blockchain::with_storage(db) {
                    Ok(mut bc) => {
                        bc.evm_enabled = true;
                        bc.evm_executor = Some(crate::evm::EvmTransactionExecutor::new());
                        println!("✅ Loaded blockchain state from storage");
                        println!("✅ EVM enabled for smart contract support");
                        bc
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to load from storage: {}. Using in-memory mode.", e);
                        let mut bc = Blockchain::new();
                        bc.evm_enabled = true;
                        bc.evm_executor = Some(crate::evm::EvmTransactionExecutor::new());
                        println!("✅ EVM enabled for smart contract support");
                        bc
                    }
                }
            } else {
                let mut bc = Blockchain::new();
                bc.evm_enabled = true;
                bc.evm_executor = Some(crate::evm::EvmTransactionExecutor::new());
                println!("✅ EVM enabled for smart contract support");
                bc
            }
        };
        
        // Set shard manager in blockchain if sharding is enabled
        // Note: We don't actually need to set it in blockchain for now since
        // cross-shard transactions are handled at the shard manager level
        let blockchain_arc = Arc::new(RwLock::new(blockchain));
        
        // Create mining manager (with sharding if enabled)
        let mining_manager: Arc<MiningManager> = if let Some(ref shard_mgr) = shard_manager {
            Arc::new(MiningManager::with_sharding(
                blockchain_arc.clone(),
                config.miner_address,
                shard_mgr.clone(),
            ))
        } else {
            Arc::new(MiningManager::new(
                blockchain_arc.clone(),
                config.miner_address,
            ))
        };
        
        // Create network manager
        let listen_addr = format!("127.0.0.1:{}", config.port)
            .parse::<SocketAddr>()
            .unwrap_or_else(|_| "127.0.0.1:8080".parse().unwrap());
        
        let mut network_manager = NetworkManager::new(
            blockchain_arc.clone(),
            listen_addr,
        );
        
        // Set shard manager in network manager if sharding is enabled
        if let Some(ref shard_mgr) = shard_manager {
            network_manager.set_shard_manager(shard_mgr.clone());
        }
        
        let network_manager = Arc::new(network_manager);
        
        // Create security scorer for AI-driven fraud detection
        let security_scorer = Arc::new(tokio::sync::RwLock::new(crate::security::RiskScorer::new()));
        println!("🔒 Security scoring enabled (AI-driven fraud detection)");
        
        // Create forensic analyzer (will be indexed as blocks are added)
        let forensic_analyzer = Arc::new(tokio::sync::RwLock::new(crate::security::ForensicAnalyzer::new()));
        println!("🔍 Forensic analyzer initialized");
        
        // Create light client for stateless mode
        let light_client = Arc::new(tokio::sync::RwLock::new(crate::light_client::LightClient::new()));
        if config.enable_verkle {
            println!("🔦 Light client initialized (will sync on first block)");
        }
        
        // Create RPC server (with sharding if enabled)
        let mut rpc_server: RpcServer = if let Some(ref shard_mgr) = shard_manager {
            RpcServer::with_sharding(
                blockchain_arc.clone(),
                shard_mgr.clone(),
            )
        } else {
            RpcServer::new(blockchain_arc.clone())
        };
        
        // Set security scorer in RPC server
        rpc_server.set_security_scorer(security_scorer.clone());
        
        // Set mining manager in RPC server for fairness metrics
        rpc_server.set_mining_manager(mining_manager.clone());
        
        // Set forensic analyzer in RPC server
        rpc_server.set_forensic_analyzer(forensic_analyzer.clone());
        
        // Set light client in RPC server
        rpc_server.set_light_client(light_client.clone());
        
        // Set network manager in RPC server for peer count
        rpc_server.set_network_manager(network_manager.clone());
        
        // Create and set policy manager
        let policy_manager = Arc::new(tokio::sync::RwLock::new(crate::security::SecurityPolicyManager::new()));
        rpc_server.set_policy_manager(policy_manager.clone());
        println!("🛡️  Security policy manager initialized");
        
        let rpc_server = Arc::new(rpc_server);
        
        Self {
            config,
            blockchain: blockchain_arc,
            mining_manager,
            network_manager,
            rpc_server,
            shard_manager,
            metrics,
            shutdown_signal: Arc::new(tokio::sync::Notify::new()),
        }
    }

    /// Start the node
    pub async fn start(&self) -> Result<(), String> {
        println!("🚀 Starting Mondoshawan Node...");
        println!("   Miner Address: {}", hex::encode(self.config.miner_address));
        println!("   Data Directory: {}", self.config.data_dir);
        
        // Create genesis block (only if blockchain is empty)
        // Use deterministic genesis so all nodes start with the same chain
        {
            let mut blockchain = self.blockchain.write().await;
            if blockchain.get_blocks().is_empty() {
                let genesis = create_deterministic_genesis();
                blockchain.add_block(genesis)
                    .map_err(|e| e.to_string())?;
                println!("✅ Genesis block created (deterministic)");
            } else {
                println!("✅ Loaded existing blockchain ({} blocks)", blockchain.get_blocks().len());
            }
        }

        // Start network layer
        self.network_manager.start().await
            .map_err(|e| e.to_string())?;
        println!("🌐 P2P Network started on port {}", self.config.port);
        
        // Start JSON-RPC server
        let rpc_addr = format!("127.0.0.1:{}", self.config.rpc_port);
        let rpc_addr_display = rpc_addr.clone();
        let rpc_server = self.rpc_server.clone();
        let metrics = self.metrics.clone();
        tokio::spawn(async move {
            start_rpc_server(rpc_addr, rpc_server, metrics).await;
        });
        println!("🔌 JSON-RPC API started on http://{}", rpc_addr_display);
        if self.metrics.is_some() {
            println!("📊 Metrics endpoint available at http://{}/metrics", rpc_addr_display);
        }
        
        // Start mining
        println!("⛏️  Starting TriStream mining...");
        println!("   Stream A: 10s blocks, 10,000 txs, 50 MSHW reward");
        println!("   Stream B: 1s blocks, 5,000 txs, 25 MSHW reward");
        println!("   Stream C: 100ms blocks, 1,000 txs, fee-based only");
        
        let mining_manager = self.mining_manager.clone();
        let network_manager = self.network_manager.clone();
        let blockchain = self.blockchain.clone();
        
        // Start mining in background
        tokio::spawn(async move {
            mining_manager.start_mining().await;
        });
        
        // Broadcast blocks when mined
        let blockchain_broadcast = blockchain.clone();
        let network_broadcast = network_manager.clone();
        tokio::spawn(async move {
            let mut last_block_count = 0;
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            loop {
                interval.tick().await;
                let bc = blockchain_broadcast.read().await;
                let current_count = bc.get_blocks().len();
                if current_count > last_block_count {
                    // New blocks mined - broadcast them
                    let new_blocks: Vec<_> = bc.get_blocks()
                        .iter()
                        .skip(last_block_count)
                        .cloned()
                        .collect();
                    drop(bc);
                    
                    for block in new_blocks {
                        if let Err(e) = network_broadcast.broadcast_block(&block).await {
                            // Log error but don't fail - network errors are non-fatal
                            eprintln!("⚠️  Failed to broadcast block: {}", e);
                        }
                    }
                    last_block_count = current_count;
                }
            }
        });

        // Stats reporting loop
        let blockchain_stats = self.blockchain.clone();
        let network_stats = self.network_manager.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));
            loop {
                interval.tick().await;
                let blockchain = blockchain_stats.read().await;
                let latest = blockchain.latest_block_number();
                let tx_count = blockchain.transaction_count();
                let miner_balance = blockchain.get_balance([1u8; 20]);
                let peer_count = network_stats.peer_count().await;
                println!("\n📊 Stats:");
                println!("   Blocks: {}", latest + 1);
                println!("   Transactions: {}", tx_count);
                println!("   Miner Balance: {} MSHW", miner_balance / 1_000_000_000_000_000_000);
                println!("   Connected Peers: {}", peer_count);
            }
        });

        Ok(())
    }

    /// Get mining manager reference
    pub fn mining_manager(&self) -> Arc<MiningManager> {
        self.mining_manager.clone()
    }

    /// Get blockchain reference
    pub fn blockchain(&self) -> Arc<RwLock<Blockchain>> {
        self.blockchain.clone()
    }

    /// Get network manager reference
    pub fn network_manager(&self) -> Arc<NetworkManager> {
        self.network_manager.clone()
    }

    /// Connect to a peer
    pub async fn connect_peer(&self, addr: SocketAddr) -> Result<(), String> {
        self.network_manager.connect_peer(addr).await
            .map_err(|e| e.to_string())
    }

    /// Shutdown the node gracefully
    pub async fn shutdown(&self) -> Result<(), String> {
        println!("\n🛑 Shutting down node gracefully...");
        
        // 1. Stop mining
        println!("   Stopping mining...");
        self.mining_manager.stop_mining().await;
        
        // 2. Wait a bit for current operations to complete
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        // 3. Flush database writes
        println!("   Flushing database...");
        // The sled database will flush on drop, but we can explicitly flush if needed
        // For now, we'll rely on drop behavior
        
        // 4. Notify shutdown
        self.shutdown_signal.notify_waiters();
        
        println!("✅ Node shutdown complete");
        Ok(())
    }
}

/// Start JSON-RPC HTTP server
async fn start_rpc_server(addr: String, rpc_server: Arc<crate::rpc::RpcServer>, metrics: Option<crate::metrics::MetricsHandle>) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => {
            println!("✅ JSON-RPC server listening on {}", addr);
            l
        }
        Err(e) => {
            eprintln!("⚠️  Failed to start RPC server on {}: {}", addr, e);
            return;
        }
    };
    
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let rpc_server_clone = rpc_server.clone();
            let metrics_clone = metrics.clone();
            tokio::spawn(async move {
                let mut buffer = vec![0u8; 1024 * 1024]; // 1MB buffer
                
                match stream.read(&mut buffer).await {
                    Ok(n) => {
                        let request_str = String::from_utf8_lossy(&buffer[..n]);
                        
                        // Check for /health endpoint
                        if request_str.starts_with("GET /health") {
                            let health_status = serde_json::json!({
                                "status": "healthy",
                                "timestamp": std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs()
                            });
                            let response_json = serde_json::to_string(&health_status).unwrap();
                            let http_response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                                response_json.len(),
                                response_json
                            );
                            let _ = stream.write_all(http_response.as_bytes()).await;
                            return;
                        }
                        
                        // Check for /ready endpoint
                        if request_str.starts_with("GET /ready") {
                            // Use a simple RPC call to check if blockchain is ready
                            // We'll use eth_blockNumber as a proxy for readiness
                            let test_request = crate::rpc::JsonRpcRequest {
                                jsonrpc: "2.0".to_string(),
                                method: "eth_blockNumber".to_string(),
                                params: Some(serde_json::Value::Array(Vec::new())),
                                id: Some(serde_json::Value::Null),
                            };
                            let test_response = rpc_server_clone.handle_request(test_request, None).await;
                            
                            // If we get a valid response, node is ready
                            let ready = test_response.error.is_none();
                            let status_code = if ready { "200 OK" } else { "503 Service Unavailable" };
                            let ready_status = serde_json::json!({
                                "ready": ready,
                                "timestamp": std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs()
                            });
                            let response_json = serde_json::to_string(&ready_status).unwrap();
                            let http_response = format!(
                                "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                                status_code,
                                response_json.len(),
                                response_json
                            );
                            let _ = stream.write_all(http_response.as_bytes()).await;
                            return;
                        }
                        
                        // Check for /metrics endpoint
                        if request_str.starts_with("GET /metrics") {
                            let metrics_result = if let Some(ref metrics_handle) = metrics_clone {
                                let metrics_guard = metrics_handle.lock().unwrap();
                                metrics_guard.gather()
                            } else {
                                Err(prometheus::Error::Msg("Metrics not enabled".to_string()))
                            };
                            
                            match metrics_result {
                                Ok(metrics_text) => {
                                    let http_response = format!(
                                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; version=0.0.4\r\nContent-Length: {}\r\n\r\n{}",
                                        metrics_text.len(),
                                        metrics_text
                                    );
                                    let _ = stream.write_all(http_response.as_bytes()).await;
                                }
                                Err(_) => {
                                    let error_msg = "Metrics unavailable";
                                    let http_response = format!(
                                        "HTTP/1.1 503 Service Unavailable\r\nContent-Length: {}\r\n\r\n{}",
                                        error_msg.len(),
                                        error_msg
                                    );
                                    let _ = stream.write_all(http_response.as_bytes()).await;
                                }
                            }
                            return;
                        }
                        
                        // Extract JSON body from HTTP request
                        let json_body = if let Some(body_start) = request_str.find("\r\n\r\n") {
                            &request_str[body_start + 4..]
                        } else if let Some(body_start) = request_str.find("\n\n") {
                            &request_str[body_start + 2..]
                        } else {
                            &request_str
                        };
                        
                        // Extract X-API-Key header if present
                        let api_key = request_str.lines()
                            .find(|line| line.starts_with("X-API-Key:") || line.starts_with("x-api-key:"))
                            .and_then(|line| line.split(':').nth(1))
                            .map(|key| key.trim().to_string());
                        
                        // Parse JSON-RPC request
                        if let Ok(request) = serde_json::from_str::<crate::rpc::JsonRpcRequest>(json_body) {
                            let response = rpc_server_clone.handle_request(request, api_key.as_deref()).await;
                            let response_json = serde_json::to_string(&response).unwrap_or_else(|_| "{}".to_string());
                            
                            let http_response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                                response_json.len(),
                                response_json
                            );
                            
                            let _ = stream.write_all(http_response.as_bytes()).await;
                        } else {
                            // Try batch request
                            if let Ok(requests) = serde_json::from_str::<Vec<crate::rpc::JsonRpcRequest>>(json_body) {
                                let api_key_ref = api_key.as_deref();
                                let responses: Vec<_> = futures::future::join_all(
                                    requests.into_iter().map(|req| rpc_server_clone.handle_request(req, api_key_ref))
                                ).await;
                                
                                let response_json = serde_json::to_string(&responses).unwrap_or_else(|_| "[]".to_string());
                                let http_response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                                    response_json.len(),
                                    response_json
                                );
                                let _ = stream.write_all(http_response.as_bytes()).await;
                            } else {
                                // Invalid request
                                let error_response = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}"#;
                                let http_response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                                    error_response.len(),
                                    error_response
                                );
                                let _ = stream.write_all(http_response.as_bytes()).await;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  Error reading RPC request: {}", e);
                    }
                }
            });
        }
    }
}

/// Create a deterministic genesis block that all nodes will share
/// This ensures all nodes start from the same chain state
fn create_deterministic_genesis() -> crate::blockchain::Block {
    use crate::blockchain::{Block, BlockHeader};
    use crate::types::StreamType;
    
    // Fixed timestamp for genesis (January 1, 2026, 00:00:00 UTC)
    const GENESIS_TIMESTAMP: u64 = 1735689600;
    
    // Create genesis header with fixed parameters
    let mut header = BlockHeader::new(
        vec![],  // No parent hashes
        0,       // Block number 0
        StreamType::StreamA,
        4,       // K parameter
    );
    
    // Override timestamp to be deterministic
    header.timestamp = GENESIS_TIMESTAMP;
    
    // Create genesis block with no transactions
    Block::new(header, vec![], vec![])
}
