//! P2P Network layer for multi-node communication
//! 
//! Features:
//! - Peer discovery
//! - Block propagation
//! - Transaction propagation
//! - Chain synchronization

use crate::blockchain::{Blockchain, Block, Transaction, PublicKey};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Maximum network message size (10MB - DoS protection)
pub const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024;

/// Authenticated network message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticatedMessage {
    /// The actual message payload
    pub message: NetworkMessage,
    /// Ed25519 signature (64 bytes) - signs the serialized message
    pub signature: Vec<u8>,
    /// Ed25519 public key (32 bytes) - for signature verification
    pub public_key: PublicKey,
    /// Message timestamp (Unix epoch seconds) - prevents replay attacks
    pub timestamp: u64,
}

/// Network message types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    /// Handshake - announce listen address
    Handshake { listen_addr: String },
    /// Announce a new block
    NewBlock { block: Block },
    /// Announce a new block from a specific shard
    NewShardBlock { block: Block, shard_id: usize },
    /// Announce a new transaction
    NewTransaction { transaction: Transaction },
    /// Request blocks (for sync)
    RequestBlocks { from_block: u64, count: u64 },
    /// Request blocks from a specific shard
    RequestShardBlocks { shard_id: usize, from_block: u64, count: u64 },
    /// Send blocks (response to RequestBlocks)
    Blocks { blocks: Vec<Block> },
    /// Send blocks from a specific shard
    ShardBlocks { shard_id: usize, blocks: Vec<Block> },
    /// Ping (keepalive)
    Ping,
    /// Pong (response to ping)
    Pong,
    /// Peer list request
    RequestPeers,
    /// Peer list response
    Peers { addresses: Vec<String> },
}

/// Network manager for P2P communication
pub struct NetworkManager {
    blockchain: Arc<RwLock<Blockchain>>,
    peers: Arc<RwLock<HashSet<SocketAddr>>>,
    listen_addr: SocketAddr,
    is_running: Arc<RwLock<bool>>,
    /// Node's signing key for message authentication (32 bytes Ed25519 secret key)
    node_secret_key: Option<[u8; 32]>,
    /// Node's public key (derived from secret key)
    node_public_key: Option<PublicKey>,
    /// Kyber key exchange for PQ-encrypted P2P communication
    kyber_keys: Option<crate::pqc::KyberKeyExchange>,
    /// Active session keys for encrypted communication (peer_addr -> session_key)
    session_keys: Arc<RwLock<std::collections::HashMap<SocketAddr, crate::pqc::SessionKey>>>,
    /// Shard manager for shard-aware block/transaction propagation
    shard_manager: Option<Arc<crate::sharding::ShardManager>>,
    /// Active peer connections for broadcasting (peer_addr -> stream)
    peer_connections: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>,
}

impl NetworkManager {
    /// Create new network manager
    pub fn new(blockchain: Arc<RwLock<Blockchain>>, listen_addr: SocketAddr) -> Self {
        Self {
            blockchain,
            peers: Arc::new(RwLock::new(HashSet::new())),
            listen_addr,
            is_running: Arc::new(RwLock::new(false)),
            node_secret_key: None,
            node_public_key: None,
            kyber_keys: None,
            session_keys: Arc::new(RwLock::new(std::collections::HashMap::new())),
            shard_manager: None,
            peer_connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create network manager with node identity (for message signing)
    pub fn with_identity(blockchain: Arc<RwLock<Blockchain>>, listen_addr: SocketAddr, secret_key: [u8; 32]) -> Self {
        use ed25519_dalek::{SigningKey, VerifyingKey};
        
        // Derive public key from secret key
        let signing_key = SigningKey::from_bytes(&secret_key);
        let verifying_key = signing_key.verifying_key();
        let public_key = verifying_key.to_bytes().to_vec();
        
        // Generate Kyber keys for PQ-encrypted communication
        // NOTE: Kyber is currently disabled due to Windows/MSVC build issues
        // The node will work without Kyber encryption - it's optional
        let kyber_keys = None; // Some(crate::pqc::KyberKeyExchange::generate());
        
        Self {
            blockchain,
            peers: Arc::new(RwLock::new(HashSet::new())),
            listen_addr,
            is_running: Arc::new(RwLock::new(false)),
            node_secret_key: Some(secret_key),
            node_public_key: Some(public_key),
            kyber_keys,
            session_keys: Arc::new(RwLock::new(std::collections::HashMap::new())),
            shard_manager: None,
            peer_connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Set shard manager for shard-aware propagation
    pub fn set_shard_manager(&mut self, shard_manager: Arc<crate::sharding::ShardManager>) {
        self.shard_manager = Some(shard_manager);
    }
    
    /// Enable PQ-encrypted P2P communication
    pub fn enable_pq_encryption(&mut self) {
        // NOTE: Kyber is currently disabled
        // if self.kyber_keys.is_none() {
        //     self.kyber_keys = Some(crate::pqc::KyberKeyExchange::generate());
        // }
    }
    
    /// Get Kyber public key for handshake
    pub fn get_kyber_public_key(&self) -> Option<Vec<u8>> {
        self.kyber_keys.as_ref().map(|k| k.public_key_bytes())
    }

    /// Sign a network message
    fn sign_message(&self, message: NetworkMessage) -> crate::error::BlockchainResult<AuthenticatedMessage> {
        use ed25519_dalek::{SigningKey, Signer};
        use bincode;
        
        // Serialize message for signing
        let message_bytes = bincode::serialize(&message)
            .map_err(|e| crate::error::BlockchainError::Serialization(e.to_string()))?;
        
        // Sign message if we have a secret key
        let (signature, public_key) = if let Some(secret_key) = self.node_secret_key {
            let signing_key = SigningKey::from_bytes(&secret_key);
            let verifying_key = signing_key.verifying_key();
            let public_key_bytes: [u8; 32] = verifying_key.to_bytes();
            
            let signature = signing_key.sign(&message_bytes);
            (signature.to_bytes().to_vec(), public_key_bytes.to_vec())
        } else {
            // No identity - use empty signature (not recommended for production)
            (vec![0; 64], vec![])
        };
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Ok(AuthenticatedMessage {
            message,
            signature,
            public_key,
            timestamp,
        })
    }

    /// Verify an authenticated message
    fn verify_message(msg: &AuthenticatedMessage) -> crate::error::BlockchainResult<()> {
        use ed25519_dalek::{VerifyingKey, Signature, Verifier};
        use bincode;
        
        // Check timestamp (prevent replay attacks - allow 5 minute window)
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Allow 5 minute clock skew
        if msg.timestamp > current_time + 300 || msg.timestamp < current_time.saturating_sub(300) {
            return Err(crate::error::BlockchainError::Network(
                "Message timestamp out of acceptable range (possible replay attack)".to_string()
            ));
        }
        
        // Skip verification if no signature (not recommended, but allow for backward compatibility)
        if msg.signature.is_empty() || msg.public_key.is_empty() {
            return Ok(()); // Allow unsigned messages for now (backward compatibility)
        }
        
        // Verify signature
        if msg.signature.len() != 64 {
            return Err(crate::error::BlockchainError::Network(
                "Invalid signature length".to_string()
            ));
        }
        
        if msg.public_key.len() != 32 {
            return Err(crate::error::BlockchainError::Network(
                "Invalid public key length".to_string()
            ));
        }
        
        // Parse public key
        let pub_key_bytes: [u8; 32] = msg.public_key.as_slice().try_into()
            .map_err(|_| crate::error::BlockchainError::Network(
                "Invalid public key format".to_string()
            ))?;
        
        let verifying_key = VerifyingKey::from_bytes(&pub_key_bytes)
            .map_err(|_| crate::error::BlockchainError::Network(
                "Invalid public key".to_string()
            ))?;
        
        // Parse signature
        let sig_bytes: [u8; 64] = msg.signature.as_slice().try_into()
            .map_err(|_| crate::error::BlockchainError::Network(
                "Invalid signature format".to_string()
            ))?;
        
        let signature = Signature::try_from(&sig_bytes[..])
            .map_err(|_| crate::error::BlockchainError::Network(
                "Invalid signature".to_string()
            ))?;
        
        // Serialize message for verification
        let message_bytes = bincode::serialize(&msg.message)
            .map_err(|e| crate::error::BlockchainError::Serialization(e.to_string()))?;
        
        // Verify signature
        verifying_key.verify(&message_bytes, &signature)
            .map_err(|_| crate::error::BlockchainError::Network(
                "Message signature verification failed".to_string()
            ))?;
        
        Ok(())
    }

    /// Start the network layer
    pub async fn start(&self) -> crate::error::BlockchainResult<()> {
        *self.is_running.write().await = true;
        
        println!("🌐 Starting P2P network on {}", self.listen_addr);
        
        // Start listening for incoming connections
        let listener = TcpListener::bind(self.listen_addr)
            .await
            .map_err(|e| crate::error::BlockchainError::Network(
                format!("Failed to bind to {}: {}", self.listen_addr, e)
            ))?;
        
        println!("✅ Listening for peers on {}", self.listen_addr);
        
        let peers = self.peers.clone();
        let blockchain = self.blockchain.clone();
        let is_running = self.is_running.clone();
        let connections = self.peer_connections.clone();
        
        // Accept incoming connections
        tokio::spawn(async move {
            while *is_running.read().await {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        println!("📥 New peer connected from: {} (ephemeral port, not added to peers)", addr);
                        // Note: We don't add the ephemeral address to peers.
                        // Only explicit connect_peer() calls add peers for broadcasting.
                        
                        let blockchain_clone = blockchain.clone();
                        let peers_clone = peers.clone();
                        let is_running_clone = is_running.clone();
                        let connections_clone = connections.clone();
                        
                        // Handle peer connection
                        tokio::spawn(async move {
                            handle_peer(stream, addr, blockchain_clone, peers_clone, is_running_clone, connections_clone).await;
                        });
                    }
                    Err(e) => {
                        eprintln!("⚠️  Error accepting connection: {}", e);
                    }
                }
            }
        });
        
        Ok(())
    }

    /// Stop the network layer
    pub async fn stop(&self) {
        *self.is_running.write().await = false;
    }

    /// Connect to a peer
    pub async fn connect_peer(&self, addr: SocketAddr) -> crate::error::BlockchainResult<()> {
        println!("🔗 [CONNECT] Attempting to connect to peer: {}", addr);
        
        // Also log to file
        let log_msg = format!("[CONNECT] Attempting to connect to peer: {}\n", addr);
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("d:\\Pyrax\\network-debug.log")
            .and_then(|mut f| std::io::Write::write_all(&mut f, log_msg.as_bytes()));
        
        // Try to connect with retry logic
        let mut attempts = 0;
        const MAX_ATTEMPTS: u32 = 3;
        const RETRY_DELAY_MS: u64 = 2000;
        
        let stream = loop {
            match TcpStream::connect(addr).await {
                Ok(s) => break s,
                Err(e) => {
                    attempts += 1;
                    if attempts >= MAX_ATTEMPTS {
                        let err_msg = format!("❌ [CONNECT] Failed to connect to {} after {} attempts: {}", addr, attempts, e);
                        eprintln!("{}", err_msg);
                        let _ = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("d:\\Pyrax\\network-debug.log")
                            .and_then(|mut f| std::io::Write::write_all(&mut f, format!("{}\n", err_msg).as_bytes()));
                        return Err(crate::error::BlockchainError::Network(
                            format!("Failed to connect to {}: {}", addr, e)
                        ));
                    }
                    println!("⚠️  [CONNECT] Attempt {}/{} failed, retrying in {}ms...", attempts, MAX_ATTEMPTS, RETRY_DELAY_MS);
                    tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
                }
            }
        };
        
        let success_msg = format!("✅ [CONNECT] TCP connection established to: {} (attempt {})", addr, attempts + 1);
        println!("{}", success_msg);
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("d:\\Pyrax\\network-debug.log")
            .and_then(|mut f| std::io::Write::write_all(&mut f, format!("{}\n", success_msg).as_bytes()));
        
        self.peers.write().await.insert(addr);
        println!("✅ [CONNECT] Added {} to peers list", addr);
        
        let blockchain = self.blockchain.clone();
        let peers = self.peers.clone();
        let is_running = self.is_running.clone();
        let connections = self.peer_connections.clone();
        
        println!("🔄 [CONNECT] Spawning handle_peer for {}", addr);
        
        // Handle peer connection
        tokio::spawn(async move {
            handle_peer(stream, addr, blockchain, peers, is_running, connections).await;
        });
        
        Ok(())
    }

    /// Broadcast a block to all peers
    pub async fn broadcast_block(&self, block: &Block) -> crate::error::BlockchainResult<()> {
        let peers = self.peers.read().await;
        println!("📡 [BROADCAST] Starting broadcast of block #{}, peer count: {}", 
            block.header.block_number, peers.len());
        
        if peers.is_empty() {
            println!("⚠️  [BROADCAST] No peers available, skipping broadcast");
            return Ok(());
        }
        
        // If sharding is enabled, try to determine shard ID from block transactions
        let shard_id = if let Some(shard_mgr) = &self.shard_manager {
            // Determine shard from first transaction (if any)
            if let Some(first_tx) = block.transactions.first() {
                Some(shard_mgr.get_shard_for_address(&first_tx.from))
            } else {
                None
            }
        } else {
            None
        };
        
        let message = if let Some(shard) = shard_id {
            NetworkMessage::NewShardBlock { block: block.clone(), shard_id: shard }
        } else {
            NetworkMessage::NewBlock { block: block.clone() }
        };
        
        let authenticated = self.sign_message(message)?;
        let data = bincode::serialize(&authenticated)
            .map_err(|e| crate::error::BlockchainError::Serialization(e.to_string()))?;
        
        // Check message size
        if data.len() > MAX_MESSAGE_SIZE {
            return Err(crate::error::BlockchainError::Network(
                format!("Message size {} exceeds maximum {}", data.len(), MAX_MESSAGE_SIZE)
            ));
        }
        
        for &peer_addr in peers.iter() {
            println!("📤 [BROADCAST] Attempting to send block #{} to peer {}", block.header.block_number, peer_addr);
            
            // Try to use stored connection first
            let connections_map = self.peer_connections.lock().await;
            println!("🔍 [BROADCAST] Checking stored connections, total: {}", connections_map.len());
            
            if let Some(stream_arc) = connections_map.get(&peer_addr) {
                println!("✅ [BROADCAST] Found stored connection for {}", peer_addr);
                let mut stream = stream_arc.lock().await;
                
                // Send via stored connection
                match stream.write_u32(data.len() as u32).await {
                    Ok(_) => {
                        match stream.write_all(&data).await {
                            Ok(_) => {
                                println!("✅ Block #{} sent to {} via stored connection", block.header.block_number, peer_addr);
                                continue;
                            }
                            Err(e) => {
                                eprintln!("⚠️  Failed to send block data to {}: {}", peer_addr, e);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("⚠️  Failed to send block length to {}: {}", peer_addr, e);
                    }
                }
            } else {
                eprintln!("⚠️  No stored connection for peer {}, skipping", peer_addr);
            }
        }
        
        Ok(())
    }
    
    /// Broadcast a block from a specific shard
    pub async fn broadcast_shard_block(&self, block: &Block, shard_id: usize) -> crate::error::BlockchainResult<()> {
        let peers = self.peers.read().await;
        if peers.is_empty() {
            return Ok(());
        }
        
        let message = NetworkMessage::NewShardBlock { block: block.clone(), shard_id };
        let authenticated = self.sign_message(message)?;
        let data = bincode::serialize(&authenticated)
            .map_err(|e| crate::error::BlockchainError::Serialization(e.to_string()))?;
        
        // Check message size
        if data.len() > MAX_MESSAGE_SIZE {
            return Err(crate::error::BlockchainError::Network(
                format!("Message size {} exceeds maximum {}", data.len(), MAX_MESSAGE_SIZE)
            ));
        }
        
        for &peer_addr in peers.iter() {
            if let Err(e) = send_to_peer(peer_addr, &data).await {
                eprintln!("⚠️  Failed to send shard block to {}: {}", peer_addr, e);
            }
        }
        
        Ok(())
    }

    /// Broadcast a transaction to all peers
    pub async fn broadcast_transaction(&self, tx: &Transaction) -> crate::error::BlockchainResult<()> {
        let peers = self.peers.read().await;
        if peers.is_empty() {
            return Ok(());
        }
        
        let message = NetworkMessage::NewTransaction { transaction: tx.clone() };
        let authenticated = self.sign_message(message)?;
        let data = bincode::serialize(&authenticated)
            .map_err(|e| crate::error::BlockchainError::Serialization(e.to_string()))?;
        
        // Check message size
        if data.len() > MAX_MESSAGE_SIZE {
            return Err(crate::error::BlockchainError::Network(
                format!("Message size {} exceeds maximum {}", data.len(), MAX_MESSAGE_SIZE)
            ));
        }
        
        for &peer_addr in peers.iter() {
            if let Err(e) = send_to_peer(peer_addr, &data).await {
                eprintln!("⚠️  Failed to send transaction to {}: {}", peer_addr, e);
            }
        }
        
        Ok(())
    }

    /// Get connected peers count
    pub async fn peer_count(&self) -> usize {
        self.peers.read().await.len()
    }

    /// Get peer addresses
    pub async fn get_peers(&self) -> Vec<SocketAddr> {
        self.peers.read().await.iter().copied().collect()
    }
}

/// Handle a peer connection
async fn handle_peer(
    mut stream: TcpStream,
    addr: SocketAddr,
    blockchain: Arc<RwLock<Blockchain>>,
    peers: Arc<RwLock<HashSet<SocketAddr>>>,
    is_running: Arc<RwLock<bool>>,
    connections: Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>,
) {
    println!("🎯 [HANDLER] Started for peer: {}", addr);
    let mut buffer = vec![0u8; 1024 * 1024]; // 1MB buffer
    
    // Store this connection for broadcasting (use the peer's listen address if known)
    // For now, we store using the provided addr
    let stream_arc = Arc::new(Mutex::new(stream));
    connections.lock().await.insert(addr, stream_arc.clone());
    println!("✅ [HANDLER] Stored connection for peer: {}", addr);
    
    // Log connection pool status
    let conn_count = connections.lock().await.len();
    println!("📊 [HANDLER] Total stored connections: {}", conn_count);
    
    let mut stream = stream_arc.lock().await;
    
    // Set read timeout to prevent blocking forever
    // This allows the loop to check is_running periodically
    let timeout = std::time::Duration::from_secs(1);
    
    while *is_running.read().await {
        // Use timeout on read to periodically check is_running
        let len_result = tokio::time::timeout(
            timeout,
            stream.read_u32()
        ).await;
        
        let len = match len_result {
            Ok(Ok(len)) => len as usize,
            Ok(Err(_)) => {
                println!("📤 Peer disconnected: {}", addr);
                peers.write().await.remove(&addr);
                break;
            }
            Err(_) => {
                // Timeout - connection still alive, just no data
                continue;
            }
        };
        
        // Check message size (DoS protection)
        if len > MAX_MESSAGE_SIZE {
            eprintln!("⚠️  Message from {} exceeds maximum size: {} bytes", addr, len);
            break;
        }
        
        // Resize buffer if needed
        if len > buffer.len() {
            buffer.resize(len, 0);
        }
        
        // Read message data
        match stream.read_exact(&mut buffer[..len]).await {
            Ok(_) => {
                // Try to deserialize as authenticated message first
                if let Ok(authenticated) = bincode::deserialize::<AuthenticatedMessage>(&buffer[..len]) {
                    // Verify message signature
                    if let Err(e) = NetworkManager::verify_message(&authenticated) {
                        eprintln!("⚠️  Message verification failed from {}: {}", addr, e);
                        continue;
                    }
                    
                    // Process the verified message
                    if let Err(e) = process_message(authenticated.message, &blockchain, &peers, &mut stream, addr).await {
                        eprintln!("⚠️  Error processing message from {}: {}", addr, e);
                    }
                } else {
                    // Try to deserialize as old format (backward compatibility)
                    if let Ok(message) = bincode::deserialize::<NetworkMessage>(&buffer[..len]) {
                        eprintln!("⚠️  Received unsigned message from {} (backward compatibility mode)", addr);
                        if let Err(e) = process_message(message, &blockchain, &peers, &mut stream, addr).await {
                            eprintln!("⚠️  Error processing message from {}: {}", addr, e);
                        }
                    } else {
                        eprintln!("⚠️  Failed to deserialize message from {}", addr);
                    }
                }
            }
            Err(e) => {
                eprintln!("⚠️  Error reading from {}: {}", addr, e);
                peers.write().await.remove(&addr);
                break;
            }
        }
    }
}

/// Process incoming network message
async fn process_message(
    message: NetworkMessage,
    blockchain: &Arc<RwLock<Blockchain>>,
    peers: &Arc<RwLock<HashSet<SocketAddr>>>,
    stream: &mut TcpStream,
    from_addr: SocketAddr,
) -> crate::error::BlockchainResult<()> {
    match message {
        NetworkMessage::Handshake { listen_addr } => {
            println!("🤝 Received handshake from {}, listen address: {}", from_addr, listen_addr);
            // Parse listen address and add to peers
            if let Ok(listen_sock_addr) = listen_addr.parse::<SocketAddr>() {
                peers.write().await.insert(listen_sock_addr);
                println!("✅ Added peer listen address: {}", listen_sock_addr);
            } else {
                eprintln!("⚠️  Invalid listen address in handshake: {}", listen_addr);
            }
        }
        NetworkMessage::NewBlock { block } => {
            println!("📦 Received block #{} from {}", block.header.block_number, from_addr);
            let mut bc = blockchain.write().await;
            match bc.add_block(block.clone()) {
                Ok(_) => {
                    println!("✅ Successfully added block #{} from peer", block.header.block_number);
                }
                Err(e) => {
                    eprintln!("❌ Failed to add block #{}: {}", block.header.block_number, e);
                }
            }
        }
        NetworkMessage::NewShardBlock { block, shard_id } => {
            println!("📦 Received shard {} block #{} from {}", shard_id, block.header.block_number, from_addr);
            let mut bc = blockchain.write().await;
            if let Err(e) = bc.add_block(block) {
                eprintln!("⚠️  Failed to add shard block: {}", e);
            }
        }
        NetworkMessage::NewTransaction { transaction: _transaction } => {
            println!("💸 Received transaction from {}", from_addr);
            // Transaction will be added to pool by mining manager
            // For now, we just acknowledge receipt
        }
        NetworkMessage::RequestBlocks { from_block, count } => {
            println!("📥 Peer {} requested blocks from {} (count: {})", from_addr, from_block, count);
            let bc = blockchain.read().await;
            let blocks: Vec<Block> = bc.get_blocks()
                .iter()
                .filter(|b| b.header.block_number >= from_block)
                .take(count as usize)
                .cloned()
                .collect();
            
            let response = NetworkMessage::Blocks { blocks };
            // Note: Response messages are not signed in this implementation
            // In production, you should sign all messages including responses
            let data = bincode::serialize(&response)?;
            
            // Send response back through the same stream
            stream.write_u32(data.len() as u32).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write length: {}", e)
                ))?;
            stream.write_all(&data).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write data: {}", e)
                ))?;
        }
        NetworkMessage::RequestShardBlocks { shard_id, from_block, count } => {
            println!("📥 Peer {} requested shard {} blocks from {} (count: {})", from_addr, shard_id, from_block, count);
            // For now, return all blocks (shard-specific block storage not yet implemented)
            let bc = blockchain.read().await;
            let blocks: Vec<Block> = bc.get_blocks()
                .iter()
                .filter(|b| b.header.block_number >= from_block)
                .take(count as usize)
                .cloned()
                .collect();
            
            let response = NetworkMessage::ShardBlocks { shard_id, blocks };
            let data = bincode::serialize(&response)?;
            
            stream.write_u32(data.len() as u32).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write length: {}", e)
                ))?;
            stream.write_all(&data).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write data: {}", e)
                ))?;
        }
        NetworkMessage::Blocks { blocks } => {
            println!("📦 Received {} blocks from {}", blocks.len(), from_addr);
            let mut bc = blockchain.write().await;
            for block in blocks {
                if let Err(e) = bc.add_block(block) {
                    eprintln!("⚠️  Failed to add block: {}", e);
                }
            }
        }
        NetworkMessage::ShardBlocks { shard_id, blocks } => {
            println!("📦 Received {} blocks from shard {} from {}", blocks.len(), shard_id, from_addr);
            let mut bc = blockchain.write().await;
            for block in blocks {
                if let Err(e) = bc.add_block(block) {
                    eprintln!("⚠️  Failed to add shard block: {}", e);
                }
            }
        }
        NetworkMessage::Ping => {
            let response = NetworkMessage::Pong;
            // Note: Ping/Pong are not signed for performance
            // In production, consider signing these as well
            let data = bincode::serialize(&response)?;
            stream.write_u32(data.len() as u32).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write length: {}", e)
                ))?;
            stream.write_all(&data).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write data: {}", e)
                ))?;
        }
        NetworkMessage::Pong => {
            // Keepalive response - do nothing
        }
        NetworkMessage::RequestPeers => {
            let peer_list: Vec<String> = peers.read().await
                .iter()
                .map(|addr| addr.to_string())
                .collect();
            
            let response = NetworkMessage::Peers { addresses: peer_list };
            // Note: Peer list responses are not signed in this implementation
            let data = bincode::serialize(&response)?;
            stream.write_u32(data.len() as u32).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write length: {}", e)
                ))?;
            stream.write_all(&data).await
                .map_err(|e| crate::error::BlockchainError::Network(
                    format!("Failed to write data: {}", e)
                ))?;
        }
        NetworkMessage::Peers { addresses } => {
            println!("👥 Received {} peer addresses from {}", addresses.len(), from_addr);
            // Could connect to these peers, but for now we just log
        }
    }
    
    Ok(())
}

/// Send data to a peer
async fn send_to_peer(addr: SocketAddr, data: &[u8]) -> crate::error::BlockchainResult<()> {
    let mut stream = TcpStream::connect(addr)
        .await
        .map_err(|e| crate::error::BlockchainError::Network(
            format!("Failed to connect to {}: {}", addr, e)
        ))?;
    
    // Send length prefix
    let len = data.len() as u32;
    stream.write_u32(len).await
        .map_err(|e| crate::error::BlockchainError::Network(
            format!("Failed to write length: {}", e)
        ))?;
    
    // Send data
    stream.write_all(data).await
        .map_err(|e| crate::error::BlockchainError::Network(
            format!("Failed to write data: {}", e)
        ))?;
    
    Ok(())
}

