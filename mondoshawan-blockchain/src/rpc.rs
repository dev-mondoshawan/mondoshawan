//! JSON-RPC 2.0 API Server
//! 
//! Provides Ethereum-compatible JSON-RPC methods for external tool integration

pub mod rate_limit;

use crate::blockchain::{Blockchain, Block, Transaction};
use crate::types::Address;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::RwLock;

/// JSON-RPC 2.0 Request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
    pub id: Option<Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Option<Value>,
}

/// JSON-RPC Error
#[derive(Debug, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// RPC server
pub struct RpcServer {
    blockchain: Arc<RwLock<Blockchain>>,
    network_manager: Option<Arc<crate::network::NetworkManager>>,
    rate_limiter: Option<Arc<rate_limit::RateLimiter>>,
    shard_manager: Option<Arc<crate::sharding::ShardManager>>,
    metrics: Option<crate::metrics::MetricsHandle>,
    /// Security scorer for risk analysis
    security_scorer: Option<Arc<tokio::sync::RwLock<crate::security::RiskScorer>>>,
    /// Mining manager for fairness metrics
    mining_manager: Option<Arc<crate::mining::MiningManager>>,
    /// Forensic analyzer for fund tracing and address analysis
    forensic_analyzer: Option<Arc<tokio::sync::RwLock<crate::security::ForensicAnalyzer>>>,
    /// Light client for stateless mode
    light_client: Option<Arc<tokio::sync::RwLock<crate::light_client::LightClient>>>,
    /// Security policy manager for opt-in behavior gating
    policy_manager: Option<Arc<tokio::sync::RwLock<crate::security::SecurityPolicyManager>>>,
    /// Node registry for governance and longevity tracking
    node_registry: Option<Arc<tokio::sync::RwLock<crate::governance::NodeRegistry>>>,
    /// Reputation manager for trust scores
    reputation_manager: Option<Arc<tokio::sync::RwLock<crate::reputation::ReputationManager>>>,
    /// Wallet registry for account abstraction
    wallet_registry: Option<Arc<tokio::sync::RwLock<crate::account_abstraction::WalletRegistry>>>,
    /// Multi-signature manager for pending transactions
    multisig_manager: Option<Arc<tokio::sync::RwLock<crate::account_abstraction::MultiSigManager>>>,
    /// Social recovery manager for wallet recovery
    social_recovery_manager: Option<Arc<tokio::sync::RwLock<crate::account_abstraction::SocialRecoveryManager>>>,
    /// API key for authentication (if None, authentication is disabled)
    api_key: Option<String>,
    /// Methods that don't require authentication (public methods)
    public_methods: std::collections::HashSet<String>,
}

impl RpcServer {
    pub fn new(blockchain: Arc<RwLock<Blockchain>>) -> Self {
        let mut public_methods = HashSet::new();
        // Public methods that don't require authentication
        public_methods.insert("eth_blockNumber".to_string());
        public_methods.insert("net_version".to_string());
        public_methods.insert("eth_chainId".to_string());
        public_methods.insert("eth_syncing".to_string());
        public_methods.insert("mds_getDagStats".to_string());
        public_methods.insert("mds_getTps".to_string());
        
        Self {
            blockchain,
            network_manager: None,
            rate_limiter: None,
            shard_manager: None,
            metrics: None,
            security_scorer: None,
            mining_manager: None,
            forensic_analyzer: None,
            light_client: None,
            policy_manager: None,
            node_registry: None,
            reputation_manager: None,
            wallet_registry: None,
            multisig_manager: None,
            social_recovery_manager: None,
            api_key: None,
            public_methods,
        }
    }
    
    /// Create RPC server with API key authentication
    pub fn with_auth(blockchain: Arc<RwLock<Blockchain>>, api_key: String) -> Self {
        let mut server = Self::new(blockchain);
        server.api_key = Some(api_key);
        server
    }

    /// Create RPC server with rate limiting
    pub fn with_rate_limit(
        blockchain: Arc<RwLock<Blockchain>>,
        max_tokens: u32,
        tokens_per_second: f64,
    ) -> Self {
        let mut public_methods = HashSet::new();
        public_methods.insert("eth_blockNumber".to_string());
        public_methods.insert("net_version".to_string());
        public_methods.insert("eth_chainId".to_string());
        public_methods.insert("eth_syncing".to_string());
        public_methods.insert("mds_getDagStats".to_string());
        public_methods.insert("mds_getTps".to_string());
        
        Self {
            blockchain,
            network_manager: None,
            rate_limiter: Some(Arc::new(rate_limit::RateLimiter::new(
                max_tokens,
                tokens_per_second,
            ))),
            shard_manager: None,
            metrics: None,
            security_scorer: None,
            mining_manager: None,
            forensic_analyzer: None,
            light_client: None,
            policy_manager: None,
            node_registry: None,
            reputation_manager: None,
            wallet_registry: None,
            multisig_manager: None,
            social_recovery_manager: None,
            api_key: None,
            public_methods,
        }
    }
    
    /// Create RPC server with rate limiting and authentication
    pub fn with_rate_limit_and_auth(
        blockchain: Arc<RwLock<Blockchain>>,
        max_tokens: u32,
        tokens_per_second: f64,
        api_key: String,
    ) -> Self {
        let mut server = Self::with_rate_limit(blockchain, max_tokens, tokens_per_second);
        server.api_key = Some(api_key);
        server
    }

    /// Create RPC server with sharding
    pub fn with_sharding(
        blockchain: Arc<RwLock<Blockchain>>,
        shard_manager: Arc<crate::sharding::ShardManager>,
    ) -> Self {
        let mut public_methods = HashSet::new();
        public_methods.insert("eth_blockNumber".to_string());
        public_methods.insert("net_version".to_string());
        public_methods.insert("eth_chainId".to_string());
        public_methods.insert("eth_syncing".to_string());
        public_methods.insert("mds_getDagStats".to_string());
        public_methods.insert("mds_getTps".to_string());
        
        Self {
            blockchain,
            network_manager: None,
            rate_limiter: None,
            shard_manager: Some(shard_manager),
            metrics: None,
            security_scorer: None,
            mining_manager: None,
            forensic_analyzer: None,
            light_client: None,
            policy_manager: None,
            node_registry: None,
            reputation_manager: None,
            wallet_registry: None,
            multisig_manager: None,
            social_recovery_manager: None,
            api_key: None,
            public_methods,
        }
    }

    /// Create RPC server with both rate limiting and sharding
    pub fn with_rate_limit_and_sharding(
        blockchain: Arc<RwLock<Blockchain>>,
        max_tokens: u32,
        tokens_per_second: f64,
        shard_manager: Arc<crate::sharding::ShardManager>,
    ) -> Self {
        let mut public_methods = HashSet::new();
        public_methods.insert("eth_blockNumber".to_string());
        public_methods.insert("net_version".to_string());
        public_methods.insert("eth_chainId".to_string());
        public_methods.insert("eth_syncing".to_string());
        public_methods.insert("mds_getDagStats".to_string());
        public_methods.insert("mds_getTps".to_string());
        
        Self {
            blockchain,
            network_manager: None,
            rate_limiter: Some(Arc::new(rate_limit::RateLimiter::new(
                max_tokens,
                tokens_per_second,
            ))),
            shard_manager: Some(shard_manager),
            metrics: None,
            security_scorer: None,
            mining_manager: None,
            forensic_analyzer: None,
            light_client: None,
            policy_manager: None,
            node_registry: None,
            reputation_manager: None,
            wallet_registry: None,
            multisig_manager: None,
            social_recovery_manager: None,
            api_key: None,
            public_methods,
        }
    }
    
    /// Create RPC server with rate limiting, sharding, and authentication
    pub fn with_rate_limit_sharding_and_auth(
        blockchain: Arc<RwLock<Blockchain>>,
        max_tokens: u32,
        tokens_per_second: f64,
        shard_manager: Arc<crate::sharding::ShardManager>,
        api_key: String,
    ) -> Self {
        let mut server = Self::with_rate_limit_and_sharding(
            blockchain,
            max_tokens,
            tokens_per_second,
            shard_manager,
        );
        server.api_key = Some(api_key);
        server
    }
    
    /// Set API key for authentication
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }
    
    /// Add a method to the public methods list (no authentication required)
    pub fn add_public_method(&mut self, method: String) {
        self.public_methods.insert(method);
    }
    
    /// Check if authentication is required for a method
    fn requires_auth(&self, method: &str) -> bool {
        // If no API key is set, authentication is disabled
        if self.api_key.is_none() {
            return false;
        }
        
        // Public methods don't require authentication
        !self.public_methods.contains(method)
    }
    
    /// Verify API key from request
    /// Checks for API key in:
    /// 1. X-API-Key header (if available)
    /// 2. api_key parameter in JSON-RPC params
    fn verify_api_key(&self, request: &JsonRpcRequest, api_key_header: Option<&str>) -> bool {
        // If authentication is disabled, always return true
        let api_key = match &self.api_key {
            Some(key) => key,
            None => return true,
        };
        
        // Check header first
        if let Some(header_key) = api_key_header {
            if header_key == api_key {
                return true;
            }
        }
        
        // Check params for api_key field
        if let Some(ref params) = request.params {
            if let Some(obj) = params.as_object() {
                if let Some(key_value) = obj.get("api_key") {
                    if let Some(key_str) = key_value.as_str() {
                        if key_str == api_key {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }

    /// Set metrics handle
    pub fn set_metrics(&mut self, metrics: crate::metrics::MetricsHandle) {
        self.metrics = Some(metrics);
    }

    /// Handle JSON-RPC request
    /// 
    /// # Arguments
    /// * `request` - The JSON-RPC request
    /// * `api_key_header` - Optional API key from HTTP header (X-API-Key)
    pub async fn handle_request(&self, request: JsonRpcRequest, api_key_header: Option<&str>) -> JsonRpcResponse {
        // Check authentication if required
        if self.requires_auth(&request.method) {
            if !self.verify_api_key(&request, api_key_header) {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32001,
                        message: "Unauthorized: Invalid or missing API key".to_string(),
                        data: Some(Value::String(
                            "Provide API key via X-API-Key header or api_key parameter".to_string()
                        )),
                    }),
                    id: request.id,
                };
            }
        }
        
        // Check rate limit
        if let Some(ref limiter) = self.rate_limiter {
            if !limiter.try_acquire().await {
                return JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32005,
                        message: "Rate limit exceeded".to_string(),
                        data: None,
                    }),
                    id: request.id,
                };
            }
        }
        
        if request.jsonrpc != "2.0" {
            return JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32600,
                    message: "Invalid Request".to_string(),
                    data: None,
                }),
                id: request.id,
            };
        }

        let result = match request.method.as_str() {
            "eth_getBalance" => self.eth_get_balance(request.params).await,
            "eth_getTransactionCount" => self.eth_get_transaction_count(request.params).await,
            "eth_getBlockByNumber" => self.eth_get_block_by_number(request.params).await,
            "eth_getBlockByHash" => self.eth_get_block_by_hash(request.params).await,
            "eth_getTransactionByHash" => self.eth_get_transaction_by_hash(request.params).await,
            "eth_sendTransaction" => self.eth_send_transaction(request.params).await,
            "eth_blockNumber" => self.eth_block_number().await,
            "eth_getBlockTransactionCountByNumber" => self.eth_get_block_transaction_count_by_number(request.params).await,
            "net_peerCount" => self.net_peer_count().await,
            "net_version" => Ok(Value::String("1".to_string())),
            "eth_chainId" => Ok(Value::String("0x1".to_string())),
            "eth_syncing" => Ok(Value::Bool(false)),
            "mds_getDagStats" => self.mds_get_dag_stats().await,
            "mds_getBlueScore" => self.mds_get_blue_score(request.params).await,
            "mds_getTps" => self.mds_get_tps(request.params).await,
            "eth_getCode" => self.eth_get_code(request.params).await,
            "eth_estimateGas" => self.eth_estimate_gas(request.params).await,
            "mds_getShardStats" => self.mds_get_shard_stats(request.params).await,
            "mds_getShardForAddress" => self.mds_get_shard_for_address(request.params).await,
            "mds_getRiskScore" => self.mds_get_risk_score(request.params).await,
            "mds_getRiskLabels" => self.mds_get_risk_labels(request.params).await,
            "mds_getTransactionRisk" => self.mds_get_transaction_risk(request.params).await,
            "mds_getFairnessMetrics" => self.mds_get_fairness_metrics(request.params).await,
            "mds_getStateRoot" => self.mds_get_state_root().await,
            "mds_getStateProof" => self.mds_get_state_proof(request.params).await,
            "mds_verifyStateProof" => self.mds_verify_state_proof(request.params).await,
            "mds_getCrossShardTransaction" => self.mds_get_cross_shard_transaction(request.params).await,
            "mds_getCrossShardTransactions" => self.mds_get_cross_shard_transactions(request.params).await,
            "mds_getShardBlock" => self.mds_get_shard_block(request.params).await,
            "mds_getShardTransactions" => self.mds_get_shard_transactions(request.params).await,
            "mds_getShardBalance" => self.mds_get_shard_balance(request.params).await,
            "mds_getOrderingPolicy" => self.mds_get_ordering_policy().await,
            "mds_setOrderingPolicy" => self.mds_set_ordering_policy(request.params).await,
            "mds_getMevMetrics" => self.mds_get_mev_metrics(request.params).await,
            "mds_getBlockFairness" => self.mds_get_block_fairness(request.params).await,
            "mds_traceFunds" => self.mds_trace_funds(request.params).await,
            "mds_getAddressSummary" => self.mds_get_address_summary(request.params).await,
            "mds_getAddressTransactions" => self.mds_get_address_transactions(request.params).await,
            "mds_detectAnomalies" => self.mds_detect_anomalies(request.params).await,
            "mds_findRelatedAddresses" => self.mds_find_related_addresses(request.params).await,
            "mds_getStateRootHistory" => self.mds_get_state_root_history(request.params).await,
            "mds_getLightClientSyncStatus" => self.mds_get_light_client_sync_status().await,
            "mds_enableLightClientMode" => self.mds_enable_light_client_mode(request.params).await,
            "mds_generatePqAccount" => self.mds_generate_pq_account(request.params).await,
            "mds_getPqAccountType" => self.mds_get_pq_account_type(request.params).await,
            "mds_exportPqKey" => self.mds_export_pq_key(request.params).await,
            "mds_importPqKey" => self.mds_import_pq_key(request.params).await,
            "mds_createPqTransaction" => self.mds_create_pq_transaction(request.params).await,
            "mds_addSecurityPolicy" => self.mds_add_security_policy(request.params).await,
            "mds_removeSecurityPolicy" => self.mds_remove_security_policy(request.params).await,
            "mds_getSecurityPolicies" => self.mds_get_security_policies(request.params).await,
            "mds_setPolicyEnabled" => self.mds_set_policy_enabled(request.params).await,
            "mds_evaluateTransactionPolicy" => self.mds_evaluate_transaction_policy(request.params).await,
            "mds_addTestBlock" => self.mds_add_test_block(request.params).await,
            "mds_createTestTransaction" => self.mds_create_test_transaction(request.params).await,
            "mds_getNodeRegistry" => self.mds_get_node_registry().await,
            "mds_getNodeLongevity" => self.mds_get_node_longevity(request.params).await,
            "mds_registerNode" => self.mds_register_node(request.params).await,
            "mds_startMining" => self.mds_start_mining(request.params).await,
            "mds_stopMining" => self.mds_stop_mining(request.params).await,
            "mds_getMiningStatus" => self.mds_get_mining_status().await,
            "mds_getMiningDashboard" => self.mds_get_mining_dashboard(request.params).await,
            "mds_getNodeStatus" => self.mds_get_node_status().await,
            "mds_sendRawTransaction" => self.mds_send_raw_transaction(request.params).await,
            // Time-locked transactions
            "mds_createTimeLockedTransaction" => self.mds_create_time_locked_transaction(request.params).await,
            "mds_getTimeLockedTransactions" => self.mds_get_time_locked_transactions(request.params).await,
            // Gasless transactions
            "mds_createGaslessTransaction" => self.mds_create_gasless_transaction(request.params).await,
            "mds_getSponsoredTransactions" => self.mds_get_sponsored_transactions(request.params).await,
            // Reputation system
            "mds_getReputation" => self.mds_get_reputation(request.params).await,
            "mds_getReputationFactors" => self.mds_get_reputation_factors(request.params).await,
            // Account Abstraction
            "mds_createWallet" => self.mds_create_wallet(request.params).await,
            "mds_getWallet" => self.mds_get_wallet(request.params).await,
            "mds_getOwnerWallets" => self.mds_get_owner_wallets(request.params).await,
            "mds_isContractWallet" => self.mds_is_contract_wallet(request.params).await,
            // Multi-Signature Operations
            "mds_createMultisigTransaction" => self.mds_create_multisig_transaction(request.params).await,
            "mds_addMultisigSignature" => self.mds_add_multisig_signature(request.params).await,
            "mds_getPendingMultisigTransactions" => self.mds_get_pending_multisig_transactions(request.params).await,
            "mds_validateMultisigTransaction" => self.mds_validate_multisig_transaction(request.params).await,
            // Social Recovery Operations
            "mds_initiateRecovery" => self.mds_initiate_recovery(request.params).await,
            "mds_approveRecovery" => self.mds_approve_recovery(request.params).await,
            "mds_getRecoveryStatus" => self.mds_get_recovery_status(request.params).await,
            "mds_completeRecovery" => self.mds_complete_recovery(request.params).await,
            "mds_cancelRecovery" => self.mds_cancel_recovery(request.params).await,
            _ => Err(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
        };

        match result {
            Ok(value) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(value),
                error: None,
                id: request.id,
            },
            Err(error) => JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(error),
                id: request.id,
            },
        }
    }

    /// eth_getBalance - Get balance for an address
    async fn eth_get_balance(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;

        // Parse hex address (remove 0x prefix)
        let address = parse_address(address_str)?;

        let blockchain = self.blockchain.read().await;
        let balance = blockchain.get_balance(address);
        
        // Convert to hex string (Ethereum format)
        Ok(Value::String(format!("0x{:x}", balance)))
    }

    /// eth_getTransactionCount - Get nonce for an address
    async fn eth_get_transaction_count(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;

        let address = parse_address(address_str)?;

        let blockchain = self.blockchain.read().await;
        let nonce = blockchain.get_nonce(address);
        
        Ok(Value::String(format!("0x{:x}", nonce)))
    }

    /// eth_blockNumber - Get latest block number
    async fn eth_block_number(&self) -> Result<Value, JsonRpcError> {
        let blockchain = self.blockchain.read().await;
        let block_number = blockchain.latest_block_number();
        Ok(Value::String(format!("0x{:x}", block_number)))
    }

    /// eth_getBlockByNumber - Get block by number
    async fn eth_get_block_by_number(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let block_num_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid block number parameter".to_string(),
                data: None,
            })?;

        let block_number = parse_hex_number(block_num_str)?;

        let blockchain = self.blockchain.read().await;
        let block = blockchain.get_block_by_number(block_number).cloned();

        Ok(block_to_json(block))
    }

    /// eth_getBlockByHash - Get block by hash
    async fn eth_get_block_by_hash(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid hash parameter".to_string(),
                data: None,
            })?;

        let hash = parse_hash(hash_str)?;

        let blockchain = self.blockchain.read().await;
        let block = blockchain.get_block_by_hash(&hash);

        Ok(block_to_json(block.as_ref().cloned()))
    }

    /// eth_getTransactionByHash - Get transaction by hash
    async fn eth_get_transaction_by_hash(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid hash parameter".to_string(),
                data: None,
            })?;

        let hash = parse_hash(hash_str)?;

        let blockchain = self.blockchain.read().await;
        
        // Search for transaction in all blocks
        for block in blockchain.get_blocks() {
            for tx in &block.transactions {
                if tx.hash == hash {
                    // Get shard information if available
                    let shard_info = if let Some(shard_manager) = &self.shard_manager {
                        shard_manager.get_transaction_shards(tx).await
                    } else {
                        None
                    };
                    
                    return Ok(tx_to_json_with_shard(tx, block.header.block_number, shard_info));
                }
            }
        }

        Ok(Value::Null)
    }

    /// eth_sendTransaction - Send a transaction
    async fn eth_send_transaction(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        // This would need to be integrated with the transaction pool
        // For now, return an error indicating it's not fully implemented
        Err(JsonRpcError {
            code: -32603,
            message: "Transaction submission not yet fully implemented".to_string(),
            data: None,
        })
    }

    /// eth_getBlockTransactionCountByNumber - Get transaction count in block
    async fn eth_get_block_transaction_count_by_number(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let block_num_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid block number parameter".to_string(),
                data: None,
            })?;

        let block_number = parse_hex_number(block_num_str)?;

        let blockchain = self.blockchain.read().await;
        let block = blockchain.get_block_by_number(block_number);

        let count = block.map(|b| b.transactions.len()).unwrap_or(0);
        Ok(Value::String(format!("0x{:x}", count)))
    }

    /// net_peerCount - Get connected peer count
    async fn net_peer_count(&self) -> Result<Value, JsonRpcError> {
        if let Some(network_mgr) = &self.network_manager {
            let peer_count = network_mgr.peer_count().await;
            Ok(Value::String(format!("0x{:x}", peer_count)))
        } else {
            // Fallback if network manager not set
            Ok(Value::String("0x0".to_string()))
        }
    }

    /// mds_getDagStats - Get GhostDAG statistics
    async fn mds_get_dag_stats(&self) -> Result<Value, JsonRpcError> {
        let blockchain = self.blockchain.read().await;
        let stats = blockchain.get_dag_stats();
        
        Ok(serde_json::json!({
            "total_blocks": stats.total_blocks,
            "blue_blocks": stats.blue_blocks,
            "red_blocks": stats.red_blocks,
            "total_transactions": stats.total_transactions,
            "total_size_bytes": stats.total_size_bytes,
            "avg_block_size": stats.avg_block_size,
            "avg_txs_per_block": stats.avg_txs_per_block,
        }))
    }

    /// mds_getBlueScore - Get blue score for a block
    async fn mds_get_blue_score(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid hash parameter".to_string(),
                data: None,
            })?;

        let hash = parse_hash(hash_str)?;

        let blockchain = self.blockchain.read().await;
        let blue_score = blockchain.ghostdag().get_blue_score(&hash)
            .unwrap_or(0);
        
        Ok(Value::String(format!("0x{:x}", blue_score)))
    }

    /// mds_getTps - Get transactions per second
    async fn mds_get_tps(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let duration_seconds = if let Some(params) = params {
            if let Some(arr) = params.as_array() {
                if let Some(v) = arr.get(0) {
                    if let Some(s) = v.as_str() {
                        parse_hex_number(s).unwrap_or(60)
                    } else if let Some(n) = v.as_u64() {
                        n
                    } else {
                        60
                    }
                } else {
                    60
                }
            } else {
                60
            }
        } else {
            60
        };

        let blockchain = self.blockchain.read().await;
        let tps = blockchain.get_tps(duration_seconds);
        
        Ok(Value::String(format!("{:.2}", tps)))
    }

    /// eth_getCode - Get contract code at address
    async fn eth_get_code(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;

        let address = parse_address(address_str)?;

        let blockchain = self.blockchain.read().await;
        if let Some(executor) = blockchain.evm_executor() {
            if let Some(code) = executor.state().get_contract_code(address) {
                return Ok(Value::String(format!("0x{}", hex::encode(code))));
            }
        }
        
        Ok(Value::String("0x".to_string()))
    }

    /// eth_estimateGas - Estimate gas for transaction
    async fn eth_estimate_gas(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        // For now, return a basic estimate
        // In production, this would actually simulate the transaction
        Ok(Value::String("0x5208".to_string())) // 21,000 base gas
    }

    /// mds_getShardStats - Get statistics for all shards
    async fn mds_get_shard_stats(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        if let Some(shard_manager) = &self.shard_manager {
            let stats = shard_manager.get_all_shard_stats().await;
            let shards_json: Vec<Value> = stats.iter().map(|s| {
                serde_json::json!({
                    "shard_id": s.shard_id,
                    "block_count": s.block_count,
                    "transaction_pool_size": s.transaction_pool_size,
                    "cross_shard_outgoing": s.cross_shard_outgoing,
                    "cross_shard_incoming": s.cross_shard_incoming,
                })
            }).collect();
            
            Ok(serde_json::json!({
                "shard_count": stats.len(),
                "shards": shards_json
            }))
        } else {
            Ok(serde_json::json!({
                "shard_count": 0,
                "shards": []
            }))
        }
    }

    /// mds_getShardForAddress - Get shard ID for an address
    async fn mds_get_shard_for_address(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;

        let address = parse_address(address_str)?;

        if let Some(shard_manager) = &self.shard_manager {
            let shard_id = shard_manager.get_shard_for_address(&address);
            Ok(Value::String(format!("0x{:x}", shard_id)))
        } else {
            Ok(Value::String("0x0".to_string()))
        }
    }
    
    /// mds_getRiskScore - Get risk score for an address
    async fn mds_get_risk_score(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let scorer = self.security_scorer.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Security scoring not enabled".to_string(),
            data: None,
        })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        
        let address = parse_address(address_str)?;
        
        let scorer_guard = scorer.read().await;
        let risk_score = scorer_guard.score_address(&address);
        
        Ok(serde_json::json!({
            "score": risk_score.score,
            "confidence": risk_score.confidence,
            "labels": risk_score.labels,
        }))
    }
    
    /// mds_getRiskLabels - Get risk labels for an address
    async fn mds_get_risk_labels(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let scorer = self.security_scorer.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Security scoring not enabled".to_string(),
            data: None,
        })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        
        let address = parse_address(address_str)?;
        
        let scorer_guard = scorer.read().await;
        let risk_score = scorer_guard.score_address(&address);
        
        Ok(serde_json::json!({
            "labels": risk_score.labels,
        }))
    }
    
    /// mds_getTransactionRisk - Get risk score for a transaction
    async fn mds_get_transaction_risk(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let scorer = self.security_scorer.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Security scoring not enabled".to_string(),
            data: None,
        })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid transaction hash parameter".to_string(),
                data: None,
            })?;
        
        let tx_hash = parse_hash(hash_str)?;
        
        // Find transaction in blockchain
        let blockchain = self.blockchain.read().await;
        let mut found_tx: Option<Transaction> = None;
        
        for block in blockchain.get_blocks() {
            for tx in &block.transactions {
                if tx.hash == tx_hash {
                    found_tx = Some(tx.clone());
                    break;
                }
            }
            if found_tx.is_some() {
                break;
            }
        }
        
        drop(blockchain);
        
        let tx = found_tx.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Transaction not found".to_string(),
            data: None,
        })?;
        
        let scorer_guard = scorer.read().await;
        let risk_score = scorer_guard.score_transaction(&tx);
        
        Ok(serde_json::json!({
            "score": risk_score.score,
            "confidence": risk_score.confidence,
            "labels": risk_score.labels,
        }))
    }
    
    /// Set security scorer
    pub fn set_security_scorer(&mut self, scorer: Arc<tokio::sync::RwLock<crate::security::RiskScorer>>) {
        self.security_scorer = Some(scorer);
    }
    
    /// Set mining manager for fairness metrics
    pub fn set_mining_manager(&mut self, mining_manager: Arc<crate::mining::MiningManager>) {
        self.mining_manager = Some(mining_manager);
    }
    
    /// Set forensic analyzer for fund tracing
    pub fn set_forensic_analyzer(&mut self, forensic_analyzer: Arc<tokio::sync::RwLock<crate::security::ForensicAnalyzer>>) {
        self.forensic_analyzer = Some(forensic_analyzer);
    }
    
    /// Set light client for stateless mode
    pub fn set_light_client(&mut self, light_client: Arc<tokio::sync::RwLock<crate::light_client::LightClient>>) {
        self.light_client = Some(light_client);
    }
    
    /// Set network manager for peer info
    pub fn set_network_manager(&mut self, network_manager: Arc<crate::network::NetworkManager>) {
        self.network_manager = Some(network_manager);
    }
    
    /// Start TriStream mining via RPC
    async fn mds_start_mining(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        if let Some(mining_mgr) = &self.mining_manager {
            mining_mgr.start_mining().await;
            Ok(json!({
                "status": "started",
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Mining manager not available".to_string(),
                data: None,
            })
        }
    }
    
    /// Stop TriStream mining via RPC
    async fn mds_stop_mining(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        if let Some(mining_mgr) = &self.mining_manager {
            mining_mgr.stop_mining().await;
            Ok(json!({
                "status": "stopped",
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Mining manager not available".to_string(),
                data: None,
            })
        }
    }
    
    /// Get mining status and basic TriStream configuration
    async fn mds_get_mining_status(&self) -> Result<Value, JsonRpcError> {
        if let Some(mining_mgr) = &self.mining_manager {
            let is_mining = *mining_mgr.is_mining().read().await;
            let pending_txs = mining_mgr.pending_count().await;
            
            // Use constants from mining module for stream configuration
            let stream_a_block_time_ms = crate::mining::STREAM_A_BLOCK_TIME.as_millis();
            let stream_b_block_time_ms = crate::mining::STREAM_B_BLOCK_TIME.as_millis();
            let stream_c_block_time_ms = crate::mining::STREAM_C_BLOCK_TIME.as_millis();
            
            Ok(json!({
                "is_mining": is_mining,
                "pending_txs": pending_txs,
                "streams": {
                    "streamA": {
                        "block_time_ms": stream_a_block_time_ms,
                        "max_txs": crate::mining::STREAM_A_MAX_TXS,
                        "reward": format!("0x{:x}", crate::mining::STREAM_A_REWARD),
                    },
                    "streamB": {
                        "block_time_ms": stream_b_block_time_ms,
                        "max_txs": crate::mining::STREAM_B_MAX_TXS,
                        "reward": format!("0x{:x}", crate::mining::STREAM_B_REWARD),
                    },
                    "streamC": {
                        "block_time_ms": stream_c_block_time_ms,
                        "max_txs": crate::mining::STREAM_C_MAX_TXS,
                        "reward": format!("0x{:x}", crate::mining::STREAM_C_REWARD),
                    },
                }
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Mining manager not available".to_string(),
                data: None,
            })
        }
    }

    /// Get detailed mining dashboard statistics including hashrate and earnings
    async fn mds_get_mining_dashboard(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        let blockchain = self.blockchain.read().await;
        let blocks = blockchain.get_blocks();
        let total_blocks = blocks.len() as u64;

        // Calculate stream-specific metrics over last 100 blocks
        let mut stream_a_blocks = 0u64;
        let mut stream_b_blocks = 0u64;
        let mut stream_c_blocks = 0u64;
        let mut stream_a_earnings = 0u128;
        let mut stream_b_earnings = 0u128;
        let mut stream_c_earnings = 0u128;
        let mut total_fees_collected = 0u128;

        let recent_blocks = blocks.iter().rev().take(100);
        for block in recent_blocks {
            match block.header.stream_type {
                crate::types::StreamType::StreamA => {
                    stream_a_blocks += 1;
                    stream_a_earnings += crate::mining::STREAM_A_REWARD;
                }
                crate::types::StreamType::StreamB => {
                    stream_b_blocks += 1;
                    stream_b_earnings += crate::mining::STREAM_B_REWARD;
                }
                crate::types::StreamType::StreamC => {
                    stream_c_blocks += 1;
                    let block_fees: u128 = block.transactions.iter().map(|tx| tx.fee).sum();
                    stream_c_earnings += block_fees;
                    total_fees_collected += block_fees;
                }
            }
        }

        let total_earnings = stream_a_earnings + stream_b_earnings + stream_c_earnings;

        // Calculate hashrate estimates (blocks per hour from 100 block sample)
        let stream_a_hashrate = stream_a_blocks as f64 * 36.0;
        let stream_b_hashrate = stream_b_blocks as f64 * 36.0;
        let stream_c_hashrate = stream_c_blocks as f64 * 36.0;

        drop(blockchain);

        Ok(json!({
            "total_blocks": total_blocks,
            "recent_sample_size": 100,
            "streams": {
                "stream_a": {
                    "blocks_mined": stream_a_blocks,
                    "earnings": format!("0x{:x}", stream_a_earnings),
                    "hashrate_estimate_blocks_per_hour": stream_a_hashrate,
                    "block_time_seconds": 10,
                    "reward_per_block": format!("0x{:x}", crate::mining::STREAM_A_REWARD),
                },
                "stream_b": {
                    "blocks_mined": stream_b_blocks,
                    "earnings": format!("0x{:x}", stream_b_earnings),
                    "hashrate_estimate_blocks_per_hour": stream_b_hashrate,
                    "block_time_seconds": 1,
                    "reward_per_block": format!("0x{:x}", crate::mining::STREAM_B_REWARD),
                },
                "stream_c": {
                    "blocks_mined": stream_c_blocks,
                    "earnings": format!("0x{:x}", stream_c_earnings),
                    "hashrate_estimate_blocks_per_hour": stream_c_hashrate,
                    "block_time_seconds": 0.1,
                    "fees_collected": format!("0x{:x}", total_fees_collected),
                },
            },
            "total_earnings_recent": format!("0x{:x}", total_earnings),
            "fees_collected": format!("0x{:x}", total_fees_collected),
        }))
    }
    
    /// Send a signed transaction to the mining pool
    async fn mds_send_raw_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;

        // Expect a single parameter: the transaction object
        let tx_value = params.as_array()
            .and_then(|arr| arr.get(0))
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing transaction parameter".to_string(),
                data: None,
            })?;

        // Deserialize the transaction
        let tx: crate::blockchain::Transaction = serde_json::from_value(tx_value.clone())
            .map_err(|e| JsonRpcError {
                code: -32602,
                message: format!("Invalid transaction format: {}", e),
                data: None,
            })?;

        // Verify signature
        if !tx.verify_signature() {
            return Err(JsonRpcError {
                code: -32000,
                message: "Invalid transaction signature".to_string(),
                data: None,
            });
        }

        // Verify nonce (check against blockchain state)
        let blockchain = self.blockchain.read().await;
        let from_addr = tx.from;  // Copy the address
        let current_nonce = blockchain.get_nonce(from_addr);
        drop(blockchain);

        if tx.nonce != current_nonce {
            return Err(JsonRpcError {
                code: -32000,
                message: format!("Invalid nonce: expected {}, got {}", current_nonce, tx.nonce),
                data: None,
            });
        }

        // Verify balance (sender must have enough for value + fee)
        let blockchain = self.blockchain.read().await;
        let balance = blockchain.get_balance(from_addr);
        drop(blockchain);

        let total_cost = tx.value.checked_add(tx.fee).ok_or_else(|| JsonRpcError {
            code: -32000,
            message: "Transaction value + fee overflow".to_string(),
            data: None,
        })?;

        if balance < total_cost {
            return Err(JsonRpcError {
                code: -32000,
                message: format!("Insufficient balance: have {}, need {}", balance, total_cost),
                data: None,
            });
        }

        // Submit to mining manager
        if let Some(mining_mgr) = &self.mining_manager {
            mining_mgr.add_transaction(tx.clone()).await
                .map_err(|e| JsonRpcError {
                    code: -32603,
                    message: format!("Failed to add transaction: {}", e),
                    data: None,
                })?;
        } else {
            return Err(JsonRpcError {
                code: -32603,
                message: "Mining manager not available".to_string(),
                data: None,
            });
        }

        // Return the transaction hash
        Ok(json!({ "hash": format!("0x{}", hex::encode(tx.hash)) }))
    }

    /// Get aggregated node status for desktop and monitoring clients
    async fn mds_get_node_status(&self) -> Result<Value, JsonRpcError> {
        // Blockchain stats
        let blockchain = self.blockchain.read().await;
        let latest_block = blockchain.latest_block_number();
        let tx_count = blockchain.transaction_count();
        drop(blockchain);
        
        // Peer count
        let peer_count = if let Some(network_mgr) = &self.network_manager {
            network_mgr.peer_count().await
        } else {
            0
        };
        
        // Mining status
        let is_mining = if let Some(mining_mgr) = &self.mining_manager {
            *mining_mgr.is_mining().read().await
        } else {
            false
        };
        
        Ok(json!({
            "height": latest_block,
            "tx_count": tx_count,
            "peer_count": peer_count,
            "is_mining": is_mining,
        }))
    }
    
    /// mds_getFairnessMetrics - Get fairness metrics for a block
    async fn mds_get_fairness_metrics(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid block hash parameter".to_string(),
                data: None,
            })?;
        
        let block_hash = parse_hash(hash_str)?;
        
        // Find block in blockchain
        let blockchain = self.blockchain.read().await;
        let block = blockchain.get_block_by_hash(&block_hash);
        let block = block.as_ref().cloned();
        drop(blockchain);
        
        if let Some(block) = block {
            // Get fairness metrics from mining manager if available
            if let Some(mining_mgr) = &self.mining_manager {
                let metrics = mining_mgr.get_fairness_metrics(&block).await;
                Ok(serde_json::json!({
                    "reordering_distance": metrics.reordering_distance,
                    "sandwich_detections": metrics.sandwich_detections,
                    "backrun_detections": metrics.backrun_detections,
                    "frontrun_detections": metrics.frontrun_detections,
                    "estimated_mev_value": format!("0x{:x}", metrics.estimated_mev_value),
                    "fairness_score": metrics.fairness_score,
                    "transaction_count": metrics.transaction_count,
                    "avg_transaction_age": metrics.avg_transaction_age,
                    "fee_concentration": metrics.fee_concentration,
                }))
            } else {
                // Return basic metrics if mining manager not available
                Ok(serde_json::json!({
                    "reordering_distance": 0.0,
                    "sandwich_detections": 0,
                    "backrun_detections": 0,
                    "frontrun_detections": 0,
                    "estimated_mev_value": "0x0",
                    "fairness_score": 1.0,
                    "transaction_count": block.transactions.len(),
                    "avg_transaction_age": 0.0,
                    "fee_concentration": 0.0,
                }))
            }
        } else {
            Err(JsonRpcError {
                code: -32602,
                message: "Block not found".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getStateRoot - Get current state root (Verkle tree root hash)
    async fn mds_get_state_root(&self) -> Result<Value, JsonRpcError> {
        let blockchain = self.blockchain.read().await;
        
        if !blockchain.is_verkle_enabled() {
            return Err(JsonRpcError {
                code: -32603,
                message: "Verkle tree not enabled".to_string(),
                data: None,
            });
        }
        
        let state_root = blockchain.state_root()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "State root not available".to_string(),
                data: None,
            })?;
        
        Ok(Value::String(format!("0x{}", hex::encode(state_root))))
    }
    
    /// mds_getStateProof - Get state proof for an address (balance + nonce)
    async fn mds_get_state_proof(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        
        let address = parse_address(address_str)?;
        
        let blockchain = self.blockchain.read().await;
        
        if !blockchain.is_verkle_enabled() {
            return Err(JsonRpcError {
                code: -32603,
                message: "Verkle tree not enabled".to_string(),
                data: None,
            });
        }
        
        let (balance, proof) = blockchain.get_balance_with_proof(address)
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Failed to generate state proof".to_string(),
                data: None,
            })?;
        
        let (nonce, _) = blockchain.get_nonce_with_proof(address)
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Failed to generate nonce proof".to_string(),
                data: None,
            })?;
        
        // Serialize proof
        let proof_bytes = proof.to_bytes();
        
        Ok(serde_json::json!({
            "address": format!("0x{}", hex::encode(address)),
            "balance": format!("0x{:x}", balance),
            "nonce": format!("0x{:x}", nonce),
            "state_root": format!("0x{}", hex::encode(proof.state_root)),
            "proof": format!("0x{}", hex::encode(&proof_bytes)),
            "proof_path": proof.proof.iter().map(|h| format!("0x{}", hex::encode(h))).collect::<Vec<_>>(),
        }))
    }
    
    /// mds_verifyStateProof - Verify a state proof (for light clients)
    async fn mds_verify_state_proof(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        // Parse address
        let address_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        let address = parse_address(address_str)?;
        
        // Parse balance
        let balance_str = params_array.get(1)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid balance parameter".to_string(),
                data: None,
            })?;
        let balance = parse_hex_number(balance_str)? as u128;
        
        // Parse proof
        let proof_str = params_array.get(2)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid proof parameter".to_string(),
                data: None,
            })?;
        let proof_bytes = hex::decode(proof_str.strip_prefix("0x").unwrap_or(proof_str))
            .map_err(|_| JsonRpcError {
                code: -32602,
                message: "Invalid proof format".to_string(),
                data: None,
            })?;
        
        let proof = crate::verkle::StateProof::from_bytes(&proof_bytes)
            .map_err(|_| JsonRpcError {
                code: -32602,
                message: "Failed to deserialize proof".to_string(),
                data: None,
            })?;
        
        // Verify proof
        let is_valid = crate::verkle::ProofVerifier::verify_balance_proof(address, balance, &proof);
        
        Ok(serde_json::json!({
            "valid": is_valid,
            "address": format!("0x{}", hex::encode(address)),
            "balance": format!("0x{:x}", balance),
            "state_root": format!("0x{}", hex::encode(proof.state_root)),
        }))
    }
    
    /// mds_getStateRootHistory - Get state root history for a block range
    async fn mds_get_state_root_history(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let light_client = self.light_client.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Light client not available".to_string(),
            data: None,
        })?;
        
        let (start_block, end_block) = if let Some(params) = params {
            let arr = params.as_array().ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid params format".to_string(),
                data: None,
            })?;
            let start = arr.get(0).and_then(|v| v.as_u64()).unwrap_or(0);
            let end = arr.get(1).and_then(|v| v.as_u64()).unwrap_or(u64::MAX);
            (start, end)
        } else {
            (0, u64::MAX)
        };
        
        let client = light_client.read().await;
        let mut history = Vec::new();
        
        // Get state roots from light client (simplified - in real implementation, 
        // light client would store history)
        if let Some(state_root) = client.current_state_root() {
            history.push(serde_json::json!({
                "block_number": client.latest_verified_block(),
                "state_root": format!("0x{}", hex::encode(state_root)),
            }));
        }
        
        Ok(serde_json::json!({
            "history": history,
            "count": history.len(),
        }))
    }
    
    /// mds_getLightClientSyncStatus - Get light client sync status
    async fn mds_get_light_client_sync_status(&self) -> Result<Value, JsonRpcError> {
        let light_client = self.light_client.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Light client not available".to_string(),
            data: None,
        })?;
        
        let client = light_client.read().await;
        let status = client.sync_status();
        
        Ok(serde_json::json!({
            "is_synced": status.is_synced,
            "latest_block": status.latest_block,
            "current_state_root": status.current_state_root.map(|r| format!("0x{}", hex::encode(r))),
            "state_root_count": status.state_root_count,
        }))
    }
    
    /// mds_enableLightClientMode - Enable or disable light client mode
    async fn mds_enable_light_client_mode(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let _enabled = if let Some(p) = params {
            if let Some(arr) = p.as_array() {
                arr.get(0).and_then(|v| v.as_bool()).unwrap_or(true)
            } else {
                true
            }
        } else {
            true
        };
        
        // Light client mode is always enabled if light client is available
        // This is a placeholder for future implementation
        Ok(serde_json::json!({
            "enabled": self.light_client.is_some(),
            "message": "Light client mode status"
        }))
    }
    
    /// mds_generatePqAccount - Generate a new PQ account
    async fn mds_generate_pq_account(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        let algorithm = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid algorithm parameter".to_string(),
                data: None,
            })?;
        
        let account = crate::pqc::tooling::generate_pq_account(algorithm)
            .map_err(|e| JsonRpcError {
                code: -32603,
                message: format!("Failed to generate PQ account: {}", e),
                data: None,
            })?;
        Ok(serde_json::json!({
            "address": format!("0x{}", hex::encode(account.address())),
            "public_key": format!("0x{}", hex::encode(account.public_key())),
            "account_type": format!("{:?}", account.account_type()),
            "secret_key": format!("0x{}", hex::encode(account.secret_key())), // WARNING: Exposing secret key via RPC is INSECURE for production. For tooling only.
        }))
    }
    
    /// mds_getPqAccountType - Get PQ account type from a transaction
    async fn mds_get_pq_account_type(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        let tx_hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid transaction hash parameter".to_string(),
                data: None,
            })?;
        let tx_hash = parse_hash(tx_hash_str)?;
        
        let blockchain = self.blockchain.read().await;
        // This is a simplified check. In a real scenario, you'd need to retrieve the full transaction
        // and then use `detect_pq_account_type_from_transaction`.
        // For now, we'll simulate by checking if a transaction with this hash exists and has a PQ signature.
        for block in blockchain.get_blocks() {
            if let Some(tx) = block.transactions.iter().find(|t| t.hash == tx_hash) {
                if let Some(pq_sig) = &tx.pq_signature {
                    return Ok(Value::String(format!("{:?}", pq_sig.account_type)));
                }
            }
        }
        Ok(Value::Null)
    }
    
    /// mds_exportPqKey - Export PQ account keys (disabled for security)
    async fn mds_export_pq_key(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        Err(JsonRpcError {
            code: -32603,
            message: "Key export disabled for security reasons".to_string(),
            data: None,
        })
    }
    
    /// mds_importPqKey - Import PQ account keys (disabled for security)
    async fn mds_import_pq_key(&self, _params: Option<Value>) -> Result<Value, JsonRpcError> {
        Err(JsonRpcError {
            code: -32603,
            message: "Key import disabled for security reasons".to_string(),
            data: None,
        })
    }
    
    /// mds_createPqTransaction - Create a PQ-signed transaction
    async fn mds_create_pq_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        // Parse transaction parameters
        let from_str = params_array.get(0).and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid from address".to_string(),
                data: None,
            })?;
        let from = parse_address(from_str)?;
        
        let to_str = params_array.get(1).and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid to address".to_string(),
                data: None,
            })?;
        let to = parse_address(to_str)?;
        
        let value_str = params_array.get(2).and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid value".to_string(),
                data: None,
            })?;
        let value = parse_hex_number(value_str)? as u128;
        
        let fee_str = params_array.get(3).and_then(|v| v.as_str())
            .unwrap_or("0x0");
        let fee = parse_hex_number(fee_str)? as u128;
        
        let nonce = params_array.get(4).and_then(|v| v.as_u64()).unwrap_or(0);
        
        let algorithm = params_array.get(5).and_then(|v| v.as_str()).unwrap_or("Dilithium3");
        
        // Get current nonce from blockchain
        let blockchain = self.blockchain.read().await;
        let current_nonce = blockchain.get_nonce(from);
        let final_nonce = if current_nonce > nonce { current_nonce } else { nonce };
        
        // Generate PQ account for signing (in production, account would be provided)
        let account = crate::pqc::tooling::generate_pq_account(algorithm)
            .map_err(|e| JsonRpcError {
                code: -32603,
                message: format!("Failed to generate PQ account: {}", e),
                data: None,
            })?;
        
        // Create transaction (note: this creates a new account, not using 'from' address)
        // In production, you'd need to provide the actual account for 'from'
        let tx = crate::pqc::tooling::create_pq_transaction(
            &account,
            to,
            value,
            fee,
            final_nonce,
            vec![], // Empty data
        )
        .map_err(|e| JsonRpcError {
            code: -32603,
            message: format!("Failed to create PQ transaction: {}", e),
            data: None,
        })?;
        
        Ok(serde_json::json!({
            "hash": format!("0x{}", hex::encode(tx.hash)),
            "from": format!("0x{}", hex::encode(tx.from)),
            "to": format!("0x{}", hex::encode(tx.to)),
            "value": format!("0x{:x}", tx.value),
            "fee": format!("0x{:x}", tx.fee),
            "nonce": format!("0x{:x}", tx.nonce),
            "has_pq_signature": tx.pq_signature.is_some(),
        }))
    }
    
    /// mds_getCrossShardTransaction - Get cross-shard transaction details
    async fn mds_get_cross_shard_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid transaction hash parameter".to_string(),
                data: None,
            })?;
        
        let tx_hash = parse_hash(hash_str)?;
        
            if let Some(shard_manager) = &self.shard_manager {
                if let Some(cross_tx) = shard_manager.get_cross_shard_transaction(tx_hash).await {
                    Ok(serde_json::json!({
                        "transaction_hash": format!("0x{}", hex::encode(tx_hash)),
                        "source_shard": cross_tx.source_shard,
                        "target_shard": cross_tx.target_shard,
                        "status": format!("{:?}", cross_tx.status),
                        "from": format!("0x{}", hex::encode(cross_tx.tx.from)),
                        "to": format!("0x{}", hex::encode(cross_tx.tx.to)),
                        "value": format!("0x{:x}", cross_tx.tx.value),
                        "is_cross_shard": true,
                    }))
                } else {
                    // Not a cross-shard transaction
                    Ok(serde_json::json!({
                        "transaction_hash": format!("0x{}", hex::encode(tx_hash)),
                        "is_cross_shard": false,
                    }))
                }
        } else {
            Ok(serde_json::json!({
                "transaction_hash": format!("0x{}", hex::encode(tx_hash)),
                "is_cross_shard": false,
                "sharding_disabled": true,
            }))
        }
    }
    
    /// mds_getCrossShardTransactions - Get all cross-shard transactions (with optional filters)
    async fn mds_get_cross_shard_transactions(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        if let Some(shard_manager) = &self.shard_manager {
            let cross_txs = shard_manager.get_all_cross_shard_transactions().await;
            let mut results = Vec::new();
            
            for cross_tx in cross_txs {
                results.push(serde_json::json!({
                    "transaction_hash": format!("0x{}", hex::encode(cross_tx.id)),
                    "source_shard": cross_tx.source_shard,
                    "target_shard": cross_tx.target_shard,
                    "status": format!("{:?}", cross_tx.status),
                    "from": format!("0x{}", hex::encode(cross_tx.tx.from)),
                    "to": format!("0x{}", hex::encode(cross_tx.tx.to)),
                    "value": format!("0x{:x}", cross_tx.tx.value),
                }));
            }
            
            Ok(serde_json::json!({
                "count": results.len(),
                "transactions": results,
            }))
        } else {
            Ok(serde_json::json!({
                "count": 0,
                "transactions": [],
                "sharding_disabled": true,
            }))
        }
    }
    
    /// mds_getShardBlock - Get block from a specific shard
    async fn mds_get_shard_block(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        let shard_id_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid shard_id parameter".to_string(),
                data: None,
            })?;
        
        let shard_id = parse_hex_number(shard_id_str)? as usize;
        
        let block_number_str = params_array.get(1)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid block_number parameter".to_string(),
                data: None,
            })?;
        
        let block_number = parse_hex_number(block_number_str)?;
        
        if let Some(shard_manager) = &self.shard_manager {
            if let Some(shard) = shard_manager.get_shard(shard_id) {
                let shard_guard = shard.read().await;
                let blockchain = shard_guard.blockchain.read().await;
                
                if let Some(block) = blockchain.get_block_by_number(block_number) {
                    Ok(serde_json::json!({
                        "shard_id": shard_id,
                        "block": block_to_json(Some(block.clone())),
                    }))
                } else {
                    Err(JsonRpcError {
                        code: -32602,
                        message: "Block not found in shard".to_string(),
                        data: None,
                    })
                }
            } else {
                Err(JsonRpcError {
                    code: -32602,
                    message: format!("Invalid shard_id: {}", shard_id),
                    data: None,
                })
            }
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Sharding not enabled".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getShardTransactions - Get transactions from a specific shard's pool
    async fn mds_get_shard_transactions(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        let shard_id_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid shard_id parameter".to_string(),
                data: None,
            })?;
        
        let shard_id = parse_hex_number(shard_id_str)? as usize;
        
        let limit = params_array.get(1)
            .and_then(|v| v.as_u64())
            .unwrap_or(100) as usize;
        
        if let Some(shard_manager) = &self.shard_manager {
            let transactions = shard_manager.get_shard_transactions(shard_id, limit).await;
            
            let txs_json: Vec<Value> = transactions.iter().map(|tx| {
                tx_to_json(tx, 0) // Block number not available for pool transactions
            }).collect();
            
            Ok(serde_json::json!({
                "shard_id": shard_id,
                "count": transactions.len(),
                "transactions": txs_json,
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Sharding not enabled".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getShardBalance - Get balance for an address in a specific shard
    async fn mds_get_shard_balance(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        let shard_id_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid shard_id parameter".to_string(),
                data: None,
            })?;
        
        let shard_id = parse_hex_number(shard_id_str)? as usize;
        
        let address_str = params_array.get(1)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        
        let address = parse_address(address_str)?;
        
        if let Some(shard_manager) = &self.shard_manager {
            if let Some(shard) = shard_manager.get_shard(shard_id) {
                let shard_guard = shard.read().await;
                let blockchain = shard_guard.blockchain.read().await;
                
                let balance = blockchain.get_balance(address);
                
                Ok(serde_json::json!({
                    "shard_id": shard_id,
                    "address": format!("0x{}", hex::encode(address)),
                    "balance": format!("0x{:x}", balance),
                }))
            } else {
                Err(JsonRpcError {
                    code: -32602,
                    message: format!("Invalid shard_id: {}", shard_id),
                    data: None,
                })
            }
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Sharding not enabled".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getOrderingPolicy - Get current transaction ordering policy
    async fn mds_get_ordering_policy(&self) -> Result<Value, JsonRpcError> {
        if let Some(mining_mgr) = &self.mining_manager {
            let policy = mining_mgr.get_ordering_policy().await;
            Ok(serde_json::json!({
                "policy": policy.name(),
                "description": match policy {
                    crate::mining::ordering::OrderingPolicy::Fifo => "First-In-First-Out (most fair)",
                    crate::mining::ordering::OrderingPolicy::Random => "Random ordering (prevents front-running)",
                    crate::mining::ordering::OrderingPolicy::FeeBased => "Fee-based ordering (maximizes miner revenue)",
                    crate::mining::ordering::OrderingPolicy::Hybrid => "Hybrid: FIFO with fee boost",
                    crate::mining::ordering::OrderingPolicy::TimeWeighted => "Time-weighted fairness",
                }
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Mining manager not available".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_setOrderingPolicy - Set transaction ordering policy
    async fn mds_set_ordering_policy(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let policy_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid policy parameter".to_string(),
                data: None,
            })?;
        
        let policy = match policy_str.to_lowercase().as_str() {
            "fifo" => crate::mining::ordering::OrderingPolicy::Fifo,
            "random" => crate::mining::ordering::OrderingPolicy::Random,
            "feebased" | "fee-based" => crate::mining::ordering::OrderingPolicy::FeeBased,
            "hybrid" => crate::mining::ordering::OrderingPolicy::Hybrid,
            "timeweighted" | "time-weighted" => crate::mining::ordering::OrderingPolicy::TimeWeighted,
            _ => {
                return Err(JsonRpcError {
                    code: -32602,
                    message: format!("Unknown policy: {}. Valid options: fifo, random, feebased, hybrid, timeweighted", policy_str),
                    data: None,
                });
            }
        };
        
        if let Some(mining_mgr) = &self.mining_manager {
            mining_mgr.set_ordering_policy(policy).await;
            Ok(serde_json::json!({
                "success": true,
                "policy": policy.name(),
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Mining manager not available".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getMevMetrics - Get MEV metrics for recent blocks
    async fn mds_get_mev_metrics(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let block_count = if let Some(params) = params {
            params.as_array()
                .and_then(|arr| arr.get(0))
                .and_then(|v| v.as_u64())
                .unwrap_or(10) as usize
        } else {
            10
        };
        
        let recent_blocks: Vec<Block> = {
            let blockchain = self.blockchain.read().await;
            let blocks = blockchain.get_blocks();
            blocks.iter().rev().take(block_count).cloned().collect()
        };
        
        if let Some(mining_mgr) = &self.mining_manager {
            let mut total_sandwich = 0u64;
            let mut total_backrun = 0u64;
            let mut total_frontrun = 0u64;
            let mut total_mev_value = 0u128;
            let mut total_fairness = 0.0;
            let mut block_count_actual = 0;
            
            for block in recent_blocks {
                let metrics = mining_mgr.get_fairness_metrics(&block).await;
                total_sandwich += metrics.sandwich_detections;
                total_backrun += metrics.backrun_detections;
                total_frontrun += metrics.frontrun_detections;
                total_mev_value += metrics.estimated_mev_value;
                total_fairness += metrics.fairness_score;
                block_count_actual += 1;
            }
            
            let avg_fairness = if block_count_actual > 0 {
                total_fairness / block_count_actual as f64
            } else {
                0.0
            };
            
            Ok(serde_json::json!({
                "blocks_analyzed": block_count_actual,
                "total_sandwich_attacks": total_sandwich,
                "total_backrun_attacks": total_backrun,
                "total_frontrun_attacks": total_frontrun,
                "total_mev_value": format!("0x{:x}", total_mev_value),
                "average_fairness_score": avg_fairness,
                "mev_detected": total_sandwich > 0 || total_backrun > 0 || total_frontrun > 0,
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Mining manager not available".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getBlockFairness - Get detailed fairness metrics for a specific block
    async fn mds_get_block_fairness(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid block hash parameter".to_string(),
                data: None,
            })?;
        
        let block_hash = parse_hash(hash_str)?;
        
        let blockchain = self.blockchain.read().await;
        let block = blockchain.get_block_by_hash(&block_hash);
        let block = block.as_ref().cloned();
        drop(blockchain);
        
        if let Some(block) = block {
            if let Some(mining_mgr) = &self.mining_manager {
                let metrics = mining_mgr.get_fairness_metrics(&block).await;
                Ok(serde_json::json!({
                    "block_hash": format!("0x{}", hex::encode(block_hash)),
                    "block_number": block.header.block_number,
                    "reordering_distance": metrics.reordering_distance,
                    "sandwich_detections": metrics.sandwich_detections,
                    "backrun_detections": metrics.backrun_detections,
                    "frontrun_detections": metrics.frontrun_detections,
                    "estimated_mev_value": format!("0x{:x}", metrics.estimated_mev_value),
                    "fairness_score": metrics.fairness_score,
                    "transaction_count": metrics.transaction_count,
                    "avg_transaction_age": metrics.avg_transaction_age,
                    "fee_concentration": metrics.fee_concentration,
                }))
            } else {
                Err(JsonRpcError {
                    code: -32603,
                    message: "Mining manager not available".to_string(),
                    data: None,
                })
            }
        } else {
            Err(JsonRpcError {
                code: -32602,
                message: "Block not found".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_traceFunds - Trace funds from a source address
    async fn mds_trace_funds(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        let address_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        let source = parse_address(address_str)?;
        
        let max_hops = params_array.get(1)
            .and_then(|v| v.as_u64())
            .unwrap_or(5) as usize;
        
        let max_paths = params_array.get(2)
            .and_then(|v| v.as_u64())
            .unwrap_or(10) as usize;
        
        if let Some(forensic) = &self.forensic_analyzer {
            let analyzer = forensic.read().await;
            let flows = analyzer.trace_funds(source, max_hops, max_paths);
            
            let flows_json: Vec<Value> = flows.iter().map(|flow| {
                serde_json::json!({
                    "path": flow.path.iter().map(|a| format!("0x{}", hex::encode(a))).collect::<Vec<_>>(),
                    "transactions": flow.transactions.iter().map(|h| format!("0x{}", hex::encode(h))).collect::<Vec<_>>(),
                    "total_value": format!("0x{:x}", flow.total_value),
                    "hop_count": flow.hop_count,
                })
            }).collect();
            
            Ok(serde_json::json!({
                "source": format!("0x{}", hex::encode(source)),
                "max_hops": max_hops,
                "flows_found": flows.len(),
                "flows": flows_json,
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Forensic analyzer not available".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getAddressSummary - Get comprehensive address summary
    async fn mds_get_address_summary(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        
        let address = parse_address(address_str)?;
        
        if let Some(forensic) = &self.forensic_analyzer {
            let analyzer = forensic.read().await;
            let summary = analyzer.generate_address_summary(address);
            
            Ok(serde_json::json!({
                "address": format!("0x{}", hex::encode(address)),
                "total_received": format!("0x{:x}", summary.total_received),
                "total_sent": format!("0x{:x}", summary.total_sent),
                "net_balance": format!("0x{:x}", summary.net_balance.max(0) as u128),
                "incoming_tx_count": summary.incoming_tx_count,
                "outgoing_tx_count": summary.outgoing_tx_count,
                "unique_contacts": summary.unique_contacts,
                "first_seen": summary.first_seen,
                "last_seen": summary.last_seen,
                "suspicious_patterns": summary.suspicious_patterns,
                "risk_indicators": summary.risk_indicators,
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Forensic analyzer not available".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getAddressTransactions - Get transaction history for an address
    async fn mds_get_address_transactions(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        let address_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        
        let address = parse_address(address_str)?;
        
        // Optional limit parameter (default 50)
        let limit = params_array.get(1)
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as usize;
        
        let blockchain = self.blockchain.read().await;
        let mut transactions = Vec::new();
        
        // Iterate through all blocks in reverse to get most recent first
        let blocks: Vec<_> = blockchain.get_blocks().iter().rev().take(1000).cloned().collect();
        
        for block in blocks {
            for tx in &block.transactions {
                // Check if address is involved (sender or receiver)
                if tx.from == address || tx.to == address {
                    transactions.push(serde_json::json!({
                        "hash": format!("0x{}", hex::encode(tx.hash)),
                        "from": format!("0x{}", hex::encode(tx.from)),
                        "to": format!("0x{}", hex::encode(tx.to)),
                        "value": format!("0x{:x}", tx.value),
                        "fee": format!("0x{:x}", tx.fee),
                        "nonce": format!("0x{:x}", tx.nonce),
                        "block_number": format!("0x{:x}", block.header.block_number),
                        "block_hash": format!("0x{}", hex::encode(block.hash)),
                        "timestamp": format!("0x{:x}", block.header.timestamp),
                        "direction": if tx.from == address { "outgoing" } else { "incoming" },
                    }));
                    
                    if transactions.len() >= limit {
                        break;
                    }
                }
            }
            if transactions.len() >= limit {
                break;
            }
        }
        
        Ok(serde_json::json!({
            "address": format!("0x{}", hex::encode(address)),
            "total": transactions.len(),
            "limit": limit,
            "transactions": transactions,
        }))
    }
    
    /// mds_detectAnomalies - Detect anomalies for an address
    async fn mds_detect_anomalies(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        
        let address = parse_address(address_str)?;
        
        if let Some(forensic) = &self.forensic_analyzer {
            let analyzer = forensic.read().await;
            let detection = analyzer.detect_anomalies(address);
            
            let anomalies_json: Vec<Value> = detection.anomalies.iter().map(|anomaly| {
                serde_json::json!({
                    "type": format!("{:?}", anomaly.anomaly_type),
                    "description": anomaly.description,
                    "severity": anomaly.severity,
                    "related_addresses": anomaly.related_addresses.iter().map(|a| format!("0x{}", hex::encode(a))).collect::<Vec<_>>(),
                })
            }).collect();
            
            Ok(serde_json::json!({
                "address": format!("0x{}", hex::encode(address)),
                "anomaly_score": detection.anomaly_score,
                "confidence": detection.confidence,
                "anomalies": anomalies_json,
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Forensic analyzer not available".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_findRelatedAddresses - Find addresses that interacted with the target
    async fn mds_find_related_addresses(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        let address_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid address parameter".to_string(),
                data: None,
            })?;
        let address = parse_address(address_str)?;
        
        let max_results = params_array.get(1)
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as usize;
        
        if let Some(forensic) = &self.forensic_analyzer {
            let analyzer = forensic.read().await;
            let related = analyzer.find_related_addresses(address, max_results);
            
            Ok(serde_json::json!({
                "address": format!("0x{}", hex::encode(address)),
                "related_count": related.len(),
                "related_addresses": related.iter().map(|a| format!("0x{}", hex::encode(a))).collect::<Vec<_>>(),
            }))
        } else {
            Err(JsonRpcError {
                code: -32603,
                message: "Forensic analyzer not available".to_string(),
                data: None,
            })
        }
    }
    
    /// Set policy manager
    pub fn set_policy_manager(&mut self, policy_manager: Arc<tokio::sync::RwLock<crate::security::SecurityPolicyManager>>) {
        self.policy_manager = Some(policy_manager);
    }
    
    /// mds_addSecurityPolicy - Add a new security policy
    async fn mds_add_security_policy(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let policy_manager = self.policy_manager.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Policy manager not available".to_string(),
            data: None,
        })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        // Parse policy from JSON
        let policy_json = params.as_array()
            .and_then(|arr| arr.get(0))
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid policy parameter".to_string(),
                data: None,
            })?;
        
        let policy: crate::security::SecurityPolicy = serde_json::from_value(policy_json.clone())
            .map_err(|e| JsonRpcError {
                code: -32602,
                message: format!("Invalid policy format: {}", e),
                data: None,
            })?;
        
        let mut manager = policy_manager.write().await;
        match manager.add_policy(policy.clone()) {
            Ok(policy_id) => {
                Ok(serde_json::json!({
                    "policy_id": policy_id,
                    "message": "Policy added successfully",
                    "policy": serde_json::to_value(&policy).unwrap_or(Value::Null),
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: format!("Failed to add policy: {}", e),
                data: None,
            })
        }
    }
    
    /// mds_removeSecurityPolicy - Remove a security policy
    async fn mds_remove_security_policy(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let policy_manager = self.policy_manager.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Policy manager not available".to_string(),
            data: None,
        })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let owner_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid owner address parameter".to_string(),
                data: None,
            })?;
        let owner = parse_address(owner_str)?;
        
        let policy_id = params.as_array()
            .and_then(|arr| arr.get(1))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid policy_id parameter".to_string(),
                data: None,
            })?;
        
        let mut manager = policy_manager.write().await;
        match manager.remove_policy(owner, policy_id) {
            Ok(_) => Ok(serde_json::json!({
                "message": "Policy removed successfully"
            })),
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: format!("Failed to remove policy: {}", e),
                data: None,
            })
        }
    }
    
    /// mds_getSecurityPolicies - Get all security policies for an owner
    async fn mds_get_security_policies(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let policy_manager = self.policy_manager.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Policy manager not available".to_string(),
            data: None,
        })?;
        
        let owner = if let Some(p) = params {
            if let Some(arr) = p.as_array() {
                if let Some(v) = arr.get(0) {
                    if let Some(s) = v.as_str() {
                        parse_address(s).map_err(|e| JsonRpcError {
                            code: -32602,
                            message: format!("Invalid owner address: {}", e.message),
                            data: None,
                        })?
                    } else {
                        [0u8; 20]
                    }
                } else {
                    [0u8; 20]
                }
            } else {
                [0u8; 20]
            }
        } else {
            [0u8; 20]
        };
        
        let manager = policy_manager.read().await;
        let policies = manager.get_policies(owner);
        
        Ok(serde_json::json!({
            "owner": format!("0x{}", hex::encode(owner)),
            "policy_count": policies.len(),
            "policies": policies.iter().map(|p| serde_json::to_value(p).unwrap_or(Value::Null)).collect::<Vec<_>>(),
        }))
    }
    
    /// mds_setPolicyEnabled - Enable or disable a policy
    async fn mds_set_policy_enabled(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let policy_manager = self.policy_manager.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Policy manager not available".to_string(),
            data: None,
        })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let owner_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid owner address parameter".to_string(),
                data: None,
            })?;
        let owner = parse_address(owner_str)?;
        
        let policy_id = params.as_array()
            .and_then(|arr| arr.get(1))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid policy_id parameter".to_string(),
                data: None,
            })?;
        
        let enabled = params.as_array()
            .and_then(|arr| arr.get(2))
            .and_then(|v| v.as_bool())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid enabled parameter".to_string(),
                data: None,
            })?;
        
        let mut manager = policy_manager.write().await;
        match manager.set_policy_enabled(owner, policy_id, enabled) {
            Ok(_) => Ok(serde_json::json!({
                "message": format!("Policy {} {}", policy_id, if enabled { "enabled" } else { "disabled" })
            })),
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: format!("Failed to update policy: {}", e),
                data: None,
            })
        }
    }
    
    /// mds_evaluateTransactionPolicy - Evaluate a transaction against policies
    async fn mds_evaluate_transaction_policy(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let policy_manager = self.policy_manager.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Policy manager not available".to_string(),
            data: None,
        })?;
        
        let security_scorer = self.security_scorer.as_ref().ok_or_else(|| JsonRpcError {
            code: -32603,
            message: "Security scorer not available".to_string(),
            data: None,
        })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        // Parse transaction hash
        let tx_hash_str = params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid transaction hash parameter".to_string(),
                data: None,
            })?;
        let tx_hash = parse_hash(tx_hash_str)?;
        
        // Get owner address (optional)
        let owner_str = params.as_array()
            .and_then(|arr| arr.get(1))
            .and_then(|v| v.as_str())
            .map(|s| parse_address(s))
            .transpose()
            .map_err(|e| JsonRpcError {
                code: -32602,
                message: format!("Invalid owner address: {}", e.message),
                data: None,
            })?;
        let owner = owner_str.unwrap_or([0u8; 20]);
        
        // Find transaction
        let blockchain = self.blockchain.read().await;
        let mut tx: Option<&Transaction> = None;
        for block in blockchain.get_blocks() {
            for transaction in &block.transactions {
                if transaction.hash == tx_hash {
                    tx = Some(transaction);
                    break;
                }
            }
            if tx.is_some() {
                break;
            }
        }
        
        let tx = tx.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Transaction not found".to_string(),
            data: None,
        })?;
        
        // Get risk score
        let scorer = security_scorer.read().await;
        let risk_score = scorer.score_transaction(tx);
        drop(scorer);
        
        // Evaluate policies
        let manager = policy_manager.read().await;
        let evaluation = manager.evaluate_transaction(tx, &risk_score, owner);
        
        Ok(serde_json::json!({
            "triggered": evaluation.triggered,
            "message": evaluation.message,
            "action": evaluation.action.map(|a| serde_json::to_value(&a).unwrap_or(Value::Null)),
            "policy": evaluation.policy.map(|p| serde_json::to_value(&p).unwrap_or(Value::Null)),
            "risk_score": {
                "score": risk_score.score,
                "confidence": risk_score.confidence,
                "labels": risk_score.labels,
            }
        }))
    }
    
    /// mds_addTestBlock - Manually add a test block (for demo purposes, bypasses mining)
    async fn mds_add_test_block(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        // Parse block number
        let block_number = params_array.get(0)
            .and_then(|v| v.as_u64())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid block_number parameter".to_string(),
                data: None,
            })?;
        
        // Parse transactions (optional - array of transaction objects or simplified format)
        // We'll need blockchain access for nonces, so get read lock first
        let blockchain_read = self.blockchain.read().await;
        let transactions: Vec<crate::blockchain::Transaction> = if let Some(txs_value) = params_array.get(1) {
            if let Some(tx_array) = txs_value.as_array() {
                let mut txs = Vec::new();
                for tx_value in tx_array {
                    // Try to parse as full Transaction struct first
                    if let Ok(tx) = serde_json::from_value::<crate::blockchain::Transaction>(tx_value.clone()) {
                        txs.push(tx);
                    } else if let Some(tx_obj) = tx_value.as_object() {
                        // Try to parse as simplified format (from mds_createTestTransaction)
                        let from_str = tx_obj.get("from")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| JsonRpcError {
                                code: -32602,
                                message: "Missing 'from' field in transaction".to_string(),
                                data: None,
                            })?;
                        let from = parse_address(from_str)?;
                        
                        let to_str = tx_obj.get("to")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| JsonRpcError {
                                code: -32602,
                                message: "Missing 'to' field in transaction".to_string(),
                                data: None,
                            })?;
                        let to = parse_address(to_str)?;
                        
                        let value_str = tx_obj.get("value")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| JsonRpcError {
                                code: -32602,
                                message: "Missing 'value' field in transaction".to_string(),
                                data: None,
                            })?;
                        let value = parse_hex_number(value_str)? as u128;
                        
                        let fee_str = tx_obj.get("fee")
                            .and_then(|v| v.as_str())
                            .unwrap_or("0x0");
                        let fee = parse_hex_number(fee_str)? as u128;
                        
                        // Get nonce from blockchain if not provided in simplified format
                        let nonce = if let Some(nonce_val) = tx_obj.get("nonce") {
                            if let Some(nonce_str) = nonce_val.as_str() {
                                parse_hex_number(nonce_str)? as u64
                            } else if let Some(nonce_u64) = nonce_val.as_u64() {
                                nonce_u64
                            } else {
                                blockchain_read.get_nonce(from)
                            }
                        } else {
                            blockchain_read.get_nonce(from)
                        };
                        
                        // Create transaction (unsigned, for demo)
                        let tx = crate::blockchain::Transaction::new(from, to, value, fee, nonce);
                        txs.push(tx);
                    } else {
                        // Try to get more info about what we received
                        let tx_type = if tx_value.is_null() { "null" }
                            else if tx_value.is_string() { "string" }
                            else if tx_value.is_number() { "number" }
                            else if tx_value.is_boolean() { "boolean" }
                            else if tx_value.is_array() { "array" }
                            else { "unknown" };
                        
                        return Err(JsonRpcError {
                            code: -32602,
                            message: format!("Invalid transaction format. Expected transaction object, got: {}. Transaction value: {}", tx_type, tx_value),
                            data: None,
                        });
                    }
                }
                txs
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // Get parent hashes (optional - defaults to latest block)
        // Do this before dropping blockchain_read
        let parent_hashes: Vec<crate::types::Hash> = if let Some(parents_value) = params_array.get(2) {
            if let Some(parent_array) = parents_value.as_array() {
                parent_array.iter()
                    .filter_map(|v| v.as_str())
                    .filter_map(|s| parse_hash(s).ok())
                    .collect()
            } else {
                Vec::new()
            }
        } else {
            // Default to latest block hash (use blockchain_read we already have)
            if let Some(latest_block) = blockchain_read.get_blocks().last() {
                vec![latest_block.hash]
            } else {
                vec![] // Genesis block
            }
        };
        
        // Release read lock before write lock
        drop(blockchain_read);
        
        // Create block header
        let header = crate::blockchain::BlockHeader::new(
            parent_hashes.clone(),
            block_number,
            crate::types::StreamType::StreamA, // Default to StreamA
            4, // Default difficulty
        );
        
        // Create block
        let block = crate::blockchain::Block::new(header, transactions, parent_hashes);
        
        // Add block to blockchain
        let mut blockchain = self.blockchain.write().await;
        match blockchain.add_block(block.clone()) {
            Ok(_) => {
                // Update light client if available
                if let Some(light_client) = &self.light_client {
                    if let Some(state_root) = blockchain.state_root() {
                        let mut client = light_client.write().await;
                        client.update_state_root(block_number, state_root);
                    }
                }
                
                // Update forensic analyzer if available
                if let Some(forensic) = &self.forensic_analyzer {
                    let mut analyzer = forensic.write().await;
                    for tx in &block.transactions {
                        analyzer.index_transaction(tx, block_number);
                    }
                }
                
                Ok(serde_json::json!({
                    "success": true,
                    "block_hash": format!("0x{}", hex::encode(block.hash)),
                    "block_number": block_number,
                    "transaction_count": block.transactions.len(),
                    "message": "Block added successfully"
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: format!("Failed to add block: {}", e),
                data: None,
            })
        }
    }
    
    /// mds_createTestTransaction - Create a test transaction (unsigned, for demo)
    async fn mds_create_test_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let params_array = params.as_array().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params format".to_string(),
            data: None,
        })?;
        
        // Parse from address
        let from_str = params_array.get(0).and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid from address".to_string(),
                data: None,
            })?;
        let from = parse_address(from_str)?;
        
        // Parse to address
        let to_str = params_array.get(1).and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid to address".to_string(),
                data: None,
            })?;
        let to = parse_address(to_str)?;
        
        // Parse value
        let value_str = params_array.get(2).and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid value".to_string(),
                data: None,
            })?;
        let value = parse_hex_number(value_str)? as u128;
        
        // Parse fee (optional)
        let fee_str = params_array.get(3).and_then(|v| v.as_str()).unwrap_or("0x0");
        let fee = parse_hex_number(fee_str)? as u128;
        
        // Get nonce from blockchain
        let blockchain = self.blockchain.read().await;
        let nonce = blockchain.get_nonce(from);
        
        // Create unsigned transaction (for demo - in production would need signing)
        let tx = crate::blockchain::Transaction::new(from, to, value, fee, nonce);
        
        Ok(serde_json::json!({
            "hash": format!("0x{}", hex::encode(tx.hash)),
            "from": format!("0x{}", hex::encode(tx.from)),
            "to": format!("0x{}", hex::encode(tx.to)),
            "value": format!("0x{:x}", tx.value),
            "fee": format!("0x{:x}", tx.fee),
            "nonce": format!("0x{:x}", tx.nonce),
            "note": "This is an unsigned transaction. For production, transactions must be signed."
        }))
    }
    
    /// mds_getNodeRegistry - Get node registry statistics
    async fn mds_get_node_registry(&self) -> Result<Value, JsonRpcError> {
        let registry = self.node_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Node registry not available".to_string(),
                data: None,
            })?;
        
        let registry = registry.read().await;
        let total_nodes = registry.total_nodes();
        let active_nodes = registry.active_nodes();
        
        Ok(json!({
            "total_nodes": total_nodes,
            "active_nodes": active_nodes,
            "nodes": registry.get_all_nodes().iter().map(|node| {
                json!({
                    "public_key": hex::encode(&node.public_key),
                    "ip_address": node.ip_address.map(|ip| ip.to_string()),
                    "created_at": node.created_at,
                })
            }).collect::<Vec<_>>()
        }))
    }
    
    /// mds_getNodeLongevity - Get longevity stats for a node
    async fn mds_get_node_longevity(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let registry = self.node_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Node registry not available".to_string(),
                data: None,
            })?;
        
        let params_array = params.and_then(|p| p.as_array().cloned())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid parameters".to_string(),
                data: None,
            })?;
        
        let public_key_str = params_array.get(0)
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing public_key parameter".to_string(),
                data: None,
            })?;
        
        let public_key_bytes = hex::decode(public_key_str.strip_prefix("0x").unwrap_or(public_key_str))
            .map_err(|_| JsonRpcError {
                code: -32602,
                message: "Invalid public_key format".to_string(),
                data: None,
            })?;
        
        if public_key_bytes.len() != 32 {
            return Err(JsonRpcError {
                code: -32602,
                message: "Invalid public_key length".to_string(),
                data: None,
            });
        }
        
        let mut public_key = [0u8; 32];
        public_key.copy_from_slice(&public_key_bytes);
        
        let registry = registry.read().await;
        let all_nodes = registry.get_all_nodes();
        let node_identity = all_nodes
            .iter()
            .find(|node| node.public_key == public_key)
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Node not found".to_string(),
                data: None,
            })?;
        
        let stats = registry.get_node_stats(node_identity)
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Node stats not found".to_string(),
                data: None,
            })?;
        
        let network_age = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() - stats.network_age_at_join;
        let network_age_days = network_age / 86400;
        
        Ok(json!({
            "public_key": hex::encode(&node_identity.public_key),
            "active_days": stats.active_days,
            "blocks_mined": stats.blocks_mined,
            "uptime_index": stats.uptime_index,
            "last_seen": stats.last_seen,
            "network_age_at_join": stats.network_age_at_join,
            "consecutive_offline_days": stats.consecutive_offline_days,
            "longevity_weight": stats.calculate_weight(network_age_days),
            "activity_snapshots_count": stats.activity_snapshots.len(),
        }))
    }
    
    /// mds_registerNode - Register a new node
    async fn mds_register_node(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let registry = self.node_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Node registry not available".to_string(),
                data: None,
            })?;
        
        // For now, create a placeholder node identity
        // In production, this would parse the full node identity from params
        let public_key = [1u8; 32]; // Placeholder
        let private_key = [2u8; 32]; // Placeholder
        
        let hardware_fingerprint = crate::governance::HardwareFingerprint::generate(&private_key);
        let node_identity = crate::governance::NodeIdentity {
            public_key,
            ip_address: None,
            hardware_fingerprint,
            zk_uniqueness_proof: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        let mut registry = registry.write().await;
        match registry.register_node(node_identity.clone()) {
            Ok(_) => Ok(json!({
                "success": true,
                "public_key": hex::encode(&node_identity.public_key),
                "message": "Node registered successfully"
            })),
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: format!("Failed to register node: {}", e),
                data: None,
            }),
        }
    }
    
    /// Set reputation manager
    pub fn with_reputation_manager(&mut self, reputation_manager: Arc<tokio::sync::RwLock<crate::reputation::ReputationManager>>) {
        self.reputation_manager = Some(reputation_manager);
    }
    
    /// mds_createTimeLockedTransaction - Create a time-locked transaction
    async fn mds_create_time_locked_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let obj = params.as_object().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Params must be an object".to_string(),
            data: None,
        })?;
        
        let from = parse_address(obj.get("from").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'from' address".to_string(),
            data: None,
        })?)?;
        
        let to = parse_address(obj.get("to").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'to' address".to_string(),
            data: None,
        })?)?;
        
        let value = parse_hex_number(obj.get("value").and_then(|v| v.as_str()).unwrap_or("0x0"))? as u128;
        let fee = parse_hex_number(obj.get("fee").and_then(|v| v.as_str()).unwrap_or("0x0"))? as u128;
        
        let blockchain = self.blockchain.read().await;
        let nonce = blockchain.get_nonce(from);
        
        let mut tx = Transaction::new(from, to, value, fee, nonce);
        
        // Set time-lock if provided
        if let Some(block_str) = obj.get("executeAtBlock").and_then(|v| v.as_str()) {
            let block = parse_hex_number(block_str)?;
            tx = tx.with_execute_at_block(block);
        }
        
        if let Some(timestamp_str) = obj.get("executeAtTimestamp").and_then(|v| v.as_str()) {
            let timestamp = parse_hex_number(timestamp_str)?;
            tx = tx.with_execute_at_timestamp(timestamp);
        }
        
        // Note: Transaction would need to be signed by the caller
        // This just creates the transaction structure
        
        Ok(json!({
            "transaction": {
                "hash": format!("0x{}", hex::encode(tx.hash)),
                "from": format!("0x{}", hex::encode(tx.from)),
                "to": format!("0x{}", hex::encode(tx.to)),
                "value": format!("0x{:x}", tx.value),
                "fee": format!("0x{:x}", tx.fee),
                "nonce": format!("0x{:x}", tx.nonce),
                "executeAtBlock": tx.execute_at_block.map(|b| format!("0x{:x}", b)),
                "executeAtTimestamp": tx.execute_at_timestamp.map(|t| format!("0x{:x}", t)),
            },
            "message": "Transaction created. Must be signed before sending."
        }))
    }
    
    /// mds_getTimeLockedTransactions - Get pending time-locked transactions
    async fn mds_get_time_locked_transactions(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let blockchain = self.blockchain.read().await;
        let current_block = blockchain.latest_block_number();
        let current_timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Get all blocks and find time-locked transactions
        let mut time_locked = Vec::new();
        for block in blockchain.get_blocks() {
            for tx in &block.transactions {
                if tx.execute_at_block.is_some() || tx.execute_at_timestamp.is_some() {
                    let is_ready = tx.is_ready_to_execute(current_block, current_timestamp);
                    time_locked.push(json!({
                        "hash": format!("0x{}", hex::encode(tx.hash)),
                        "from": format!("0x{}", hex::encode(tx.from)),
                        "to": format!("0x{}", hex::encode(tx.to)),
                        "value": format!("0x{:x}", tx.value),
                        "executeAtBlock": tx.execute_at_block.map(|b| format!("0x{:x}", b)),
                        "executeAtTimestamp": tx.execute_at_timestamp.map(|t| format!("0x{:x}", t)),
                        "isReady": is_ready,
                        "currentBlock": format!("0x{:x}", current_block),
                        "currentTimestamp": format!("0x{:x}", current_timestamp),
                    }));
                }
            }
        }
        
        Ok(json!({
            "timeLockedTransactions": time_locked,
            "count": time_locked.len(),
        }))
    }
    
    /// mds_createGaslessTransaction - Create a gasless (sponsored) transaction
    async fn mds_create_gasless_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let obj = params.as_object().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Params must be an object".to_string(),
            data: None,
        })?;
        
        let from = parse_address(obj.get("from").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'from' address".to_string(),
            data: None,
        })?)?;
        
        let to = parse_address(obj.get("to").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'to' address".to_string(),
            data: None,
        })?)?;
        
        let sponsor = parse_address(obj.get("sponsor").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'sponsor' address".to_string(),
            data: None,
        })?)?;
        
        let value = parse_hex_number(obj.get("value").and_then(|v| v.as_str()).unwrap_or("0x0"))? as u128;
        let fee = parse_hex_number(obj.get("fee").and_then(|v| v.as_str()).unwrap_or("0x0"))? as u128;
        
        let blockchain = self.blockchain.read().await;
        let nonce = blockchain.get_nonce(from);
        
        let tx = Transaction::new(from, to, value, fee, nonce)
            .with_sponsor(sponsor);
        
        // Check sponsor balance
        let sponsor_balance = blockchain.get_balance(sponsor);
        if sponsor_balance < fee {
            return Err(JsonRpcError {
                code: -32603,
                message: format!("Insufficient sponsor balance: has {}, needs {}", sponsor_balance, fee),
                data: None,
            });
        }
        
        Ok(json!({
            "transaction": {
                "hash": format!("0x{}", hex::encode(tx.hash)),
                "from": format!("0x{}", hex::encode(tx.from)),
                "to": format!("0x{}", hex::encode(tx.to)),
                "value": format!("0x{:x}", tx.value),
                "fee": format!("0x{:x}", tx.fee),
                "sponsor": format!("0x{}", hex::encode(sponsor)),
                "nonce": format!("0x{:x}", tx.nonce),
            },
            "sponsorBalance": format!("0x{:x}", sponsor_balance),
            "message": "Transaction created. Must be signed before sending."
        }))
    }
    
    /// mds_getSponsoredTransactions - Get transactions sponsored by an address
    async fn mds_get_sponsored_transactions(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let sponsor = parse_address(params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing sponsor address".to_string(),
                data: None,
            })?)?;
        
        let blockchain = self.blockchain.read().await;
        let mut sponsored = Vec::new();
        
        for block in blockchain.get_blocks() {
            for tx in &block.transactions {
                if let Some(tx_sponsor) = tx.sponsor {
                    if tx_sponsor == sponsor {
                        sponsored.push(json!({
                            "hash": format!("0x{}", hex::encode(tx.hash)),
                            "from": format!("0x{}", hex::encode(tx.from)),
                            "to": format!("0x{}", hex::encode(tx.to)),
                            "value": format!("0x{:x}", tx.value),
                            "fee": format!("0x{:x}", tx.fee),
                            "sponsor": format!("0x{}", hex::encode(sponsor)),
                            "blockNumber": format!("0x{:x}", block.header.block_number),
                        }));
                    }
                }
            }
        }
        
        Ok(json!({
            "sponsoredTransactions": sponsored,
            "count": sponsored.len(),
            "sponsor": format!("0x{}", hex::encode(sponsor)),
        }))
    }
    
    /// mds_getReputation - Get reputation score for an address
    async fn mds_get_reputation(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let reputation_manager = self.reputation_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Reputation manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address = parse_address(params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing address".to_string(),
                data: None,
            })?)?;
        
        let mut manager = reputation_manager.write().await;
        let reputation = manager.get_reputation(&address);
        
        Ok(json!({
            "address": format!("0x{}", hex::encode(address)),
            "reputation": reputation.value(),
            "isHigh": reputation.is_high(),
            "isMedium": reputation.is_medium(),
            "isLow": reputation.is_low(),
        }))
    }
    
    /// mds_getReputationFactors - Get detailed reputation factors for an address
    async fn mds_get_reputation_factors(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let reputation_manager = self.reputation_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Reputation manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address = parse_address(params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing address".to_string(),
                data: None,
            })?)?;
        
        let mut manager = reputation_manager.write().await;
        let reputation = manager.get_reputation(&address);
        
        // Get factors before dropping the write lock
        let factors = manager.get_factors(&address).cloned();
        drop(manager);
        
        if let Some(factors) = factors {
            Ok(json!({
                "address": format!("0x{}", hex::encode(address)),
                "reputation": reputation.value(),
                "factors": {
                    "successfulTxs": factors.successful_txs,
                    "failedTxs": factors.failed_txs,
                    "blocksMined": factors.blocks_mined,
                    "nodeLongevity": factors.node_longevity,
                    "accountAgeDays": factors.account_age_days,
                    "totalValueTransacted": format!("0x{:x}", factors.total_value_transacted),
                    "uniqueContacts": factors.unique_contacts,
                    "suspiciousActivities": factors.suspicious_activities,
                }
            }))
        } else {
            Ok(json!({
                "address": format!("0x{}", hex::encode(address)),
                "reputation": reputation.value(),
                "factors": null,
            }))
        }
    }
    
    /// Set wallet registry
    pub fn with_wallet_registry(&mut self, wallet_registry: Arc<tokio::sync::RwLock<crate::account_abstraction::WalletRegistry>>) {
        self.wallet_registry = Some(wallet_registry);
    }
    
    /// Set multi-signature manager
    pub fn with_multisig_manager(&mut self, multisig_manager: Arc<tokio::sync::RwLock<crate::account_abstraction::MultiSigManager>>) {
        self.multisig_manager = Some(multisig_manager);
    }
    
    /// Set social recovery manager
    pub fn with_social_recovery_manager(&mut self, social_recovery_manager: Arc<tokio::sync::RwLock<crate::account_abstraction::SocialRecoveryManager>>) {
        self.social_recovery_manager = Some(social_recovery_manager);
    }
    
    /// mds_createWallet - Create a new smart contract wallet
    async fn mds_create_wallet(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let wallet_registry = self.wallet_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Wallet registry not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let obj = params.as_object().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Params must be an object".to_string(),
            data: None,
        })?;
        
        let owner = parse_address(obj.get("owner").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'owner' address".to_string(),
            data: None,
        })?)?;
        
        let wallet_type_str = obj.get("walletType").and_then(|v| v.as_str()).unwrap_or("basic");
        let salt = obj.get("salt").and_then(|v| v.as_str())
            .map(|s| parse_hex_number(s))
            .transpose()?
            .unwrap_or(0);
        
        use crate::account_abstraction::{WalletFactory, WalletType};
        
        let wallet = match wallet_type_str {
            "basic" => {
                WalletFactory::create_basic_wallet(owner, salt)
            }
            "multisig" => {
                let signers_arr = obj.get("signers").and_then(|v| v.as_array())
                    .ok_or_else(|| JsonRpcError {
                        code: -32602,
                        message: "Missing 'signers' array for multisig wallet".to_string(),
                        data: None,
                    })?;
                
                let signers: Result<Vec<Address>, _> = signers_arr
                    .iter()
                    .map(|v| v.as_str().ok_or_else(|| JsonRpcError {
                        code: -32602,
                        message: "Invalid signer address".to_string(),
                        data: None,
                    }).and_then(|s| parse_address(s)))
                    .collect();
                
                let signers = signers?;
                let threshold = obj.get("threshold").and_then(|v| v.as_u64())
                    .ok_or_else(|| JsonRpcError {
                        code: -32602,
                        message: "Missing 'threshold' for multisig wallet".to_string(),
                        data: None,
                    })? as u8;
                
                WalletFactory::create_multisig_wallet(owner, salt, signers, threshold)
                    .map_err(|e| JsonRpcError {
                        code: -32603,
                        message: format!("Failed to create multisig wallet: {}", e),
                        data: None,
                    })?
            }
            "socialRecovery" => {
                let guardians_arr = obj.get("guardians").and_then(|v| v.as_array())
                    .ok_or_else(|| JsonRpcError {
                        code: -32602,
                        message: "Missing 'guardians' array for social recovery wallet".to_string(),
                        data: None,
                    })?;
                
                let guardians: Result<Vec<Address>, _> = guardians_arr
                    .iter()
                    .map(|v| v.as_str().ok_or_else(|| JsonRpcError {
                        code: -32602,
                        message: "Invalid guardian address".to_string(),
                        data: None,
                    }).and_then(|s| parse_address(s)))
                    .collect();
                
                let guardians = guardians?;
                let recovery_threshold = obj.get("recoveryThreshold").and_then(|v| v.as_u64())
                    .ok_or_else(|| JsonRpcError {
                        code: -32602,
                        message: "Missing 'recoveryThreshold' for social recovery wallet".to_string(),
                        data: None,
                    })? as u8;
                
                let time_delay = obj.get("timeDelay").and_then(|v| v.as_u64())
                    .unwrap_or(604800); // Default 7 days
                
                WalletFactory::create_social_recovery_wallet(owner, salt, guardians, recovery_threshold, time_delay)
                    .map_err(|e| JsonRpcError {
                        code: -32603,
                        message: format!("Failed to create social recovery wallet: {}", e),
                        data: None,
                    })?
            }
            "spendingLimit" => {
                let daily_limit = obj.get("dailyLimit").and_then(|v| v.as_str())
                    .map(|s| parse_hex_number(s))
                    .transpose()?
                    .unwrap_or(0) as u128;
                let weekly_limit = obj.get("weeklyLimit").and_then(|v| v.as_str())
                    .map(|s| parse_hex_number(s))
                    .transpose()?
                    .unwrap_or(0) as u128;
                let monthly_limit = obj.get("monthlyLimit").and_then(|v| v.as_str())
                    .map(|s| parse_hex_number(s))
                    .transpose()?
                    .unwrap_or(0) as u128;
                
                WalletFactory::create_spending_limit_wallet(owner, salt, daily_limit, weekly_limit, monthly_limit)
            }
            _ => {
                return Err(JsonRpcError {
                    code: -32602,
                    message: format!("Unknown wallet type: {}", wallet_type_str),
                    data: None,
                });
            }
        };
        
        // Register wallet
        let mut registry = wallet_registry.write().await;
        match registry.register_wallet(wallet.clone()) {
            Ok(_) => {
                Ok(json!({
                    "walletAddress": format!("0x{}", hex::encode(wallet.address)),
                    "owner": format!("0x{}", hex::encode(wallet.owner)),
                    "walletType": wallet_type_str,
                    "nonce": format!("0x{:x}", wallet.nonce),
                    "createdAt": wallet.created_at,
                    "message": "Wallet created successfully"
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: format!("Failed to register wallet: {}", e),
                data: None,
            }),
        }
    }
    
    /// mds_getWallet - Get wallet information by address
    async fn mds_get_wallet(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let wallet_registry = self.wallet_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Wallet registry not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address = parse_address(params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing wallet address".to_string(),
                data: None,
            })?)?;
        
        let registry = wallet_registry.read().await;
        if let Some(wallet) = registry.get_wallet(&address) {
            let wallet_type_str = match &wallet.wallet_type {
                crate::account_abstraction::WalletType::Basic => "basic",
                crate::account_abstraction::WalletType::MultiSig { .. } => "multisig",
                crate::account_abstraction::WalletType::SocialRecovery { .. } => "socialRecovery",
                crate::account_abstraction::WalletType::SpendingLimit { .. } => "spendingLimit",
                crate::account_abstraction::WalletType::Combined { .. } => "combined",
            };
            
            Ok(json!({
                "walletAddress": format!("0x{}", hex::encode(wallet.address)),
                "owner": format!("0x{}", hex::encode(wallet.owner)),
                "walletType": wallet_type_str,
                "nonce": format!("0x{:x}", wallet.nonce),
                "createdAt": wallet.created_at,
            }))
        } else {
            Err(JsonRpcError {
                code: -32602,
                message: "Wallet not found".to_string(),
                data: None,
            })
        }
    }
    
    /// mds_getOwnerWallets - Get all wallets for an owner
    async fn mds_get_owner_wallets(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let wallet_registry = self.wallet_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Wallet registry not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let owner = parse_address(params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing owner address".to_string(),
                data: None,
            })?)?;
        
        let registry = wallet_registry.read().await;
        let wallets = registry.get_owner_wallets(&owner);
        
        let wallets_json: Vec<Value> = wallets.iter().map(|wallet| {
            let wallet_type_str = match &wallet.wallet_type {
                crate::account_abstraction::WalletType::Basic => "basic",
                crate::account_abstraction::WalletType::MultiSig { .. } => "multisig",
                crate::account_abstraction::WalletType::SocialRecovery { .. } => "socialRecovery",
                crate::account_abstraction::WalletType::SpendingLimit { .. } => "spendingLimit",
                crate::account_abstraction::WalletType::Combined { .. } => "combined",
            };
            
            json!({
                "walletAddress": format!("0x{}", hex::encode(wallet.address)),
                "walletType": wallet_type_str,
                "nonce": format!("0x{:x}", wallet.nonce),
                "createdAt": wallet.created_at,
            })
        }).collect();
        
        Ok(json!({
            "owner": format!("0x{}", hex::encode(owner)),
            "wallets": wallets_json,
            "count": wallets_json.len(),
        }))
    }
    
    /// mds_isContractWallet - Check if an address is a contract wallet
    async fn mds_is_contract_wallet(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let wallet_registry = self.wallet_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Wallet registry not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let address = parse_address(params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing address".to_string(),
                data: None,
            })?)?;
        
        let registry = wallet_registry.read().await;
        let is_wallet = registry.is_contract_wallet(&address);
        
        Ok(json!({
            "address": format!("0x{}", hex::encode(address)),
            "isContractWallet": is_wallet,
        }))
    }
    
    /// mds_createMultisigTransaction - Create a new multi-signature transaction
    async fn mds_create_multisig_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let wallet_registry = self.wallet_registry.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Wallet registry not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let obj = params.as_object().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Params must be an object".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(obj.get("walletAddress").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'walletAddress'".to_string(),
            data: None,
        })?)?;
        
        // Verify wallet exists and is multi-sig
        let registry = wallet_registry.read().await;
        let wallet = registry.get_wallet(&wallet_address)
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Wallet not found".to_string(),
                data: None,
            })?;
        
        if !wallet.is_multisig() {
            return Err(JsonRpcError {
                code: -32602,
                message: "Wallet is not a multi-signature wallet".to_string(),
                data: None,
            });
        }
        
        // Get signers and threshold from wallet
        let (signers, threshold) = match &wallet.wallet_type {
            crate::account_abstraction::WalletType::MultiSig { signers, threshold } => {
                (signers.clone(), *threshold)
            }
            crate::account_abstraction::WalletType::Combined { signers, threshold, .. } => {
                (signers.clone(), *threshold)
            }
            _ => {
                return Err(JsonRpcError {
                    code: -32602,
                    message: "Wallet is not a multi-signature wallet".to_string(),
                    data: None,
                });
            }
        };
        
        // Parse transaction fields
        let to = parse_address(obj.get("to").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'to' address".to_string(),
            data: None,
        })?)?;
        
        let value = obj.get("value").and_then(|v| v.as_str())
            .map(|s| parse_hex_u128(s))
            .transpose()?
            .unwrap_or(0);
        
        let fee = obj.get("fee").and_then(|v| v.as_str())
            .map(|s| parse_hex_u128(s))
            .transpose()?
            .unwrap_or(0);
        
        let nonce = wallet.get_nonce();
        
        // Create transaction
        let tx = crate::blockchain::Transaction::new(wallet_address, to, value, fee, nonce);
        
        // Clone signers for JSON response (before moving into MultiSigTransaction)
        let signers_for_json: Vec<String> = signers.iter().map(|s| format!("0x{}", hex::encode(s))).collect();
        
        // Create multi-sig transaction
        use crate::account_abstraction::MultiSigTransaction;
        let multisig_tx = MultiSigTransaction::new(wallet_address, tx, signers, threshold)
            .map_err(|e| JsonRpcError {
                code: -32603,
                message: format!("Failed to create multi-sig transaction: {}", e),
                data: None,
            })?;
        
        Ok(json!({
            "walletAddress": format!("0x{}", hex::encode(wallet_address)),
            "transactionHash": format!("0x{}", hex::encode(multisig_tx.transaction.hash)),
            "threshold": threshold,
            "signaturesRequired": threshold,
            "signaturesCollected": 0,
            "expectedSigners": signers_for_json,
            "message": "Multi-sig transaction created. Add signatures using mds_addMultisigSignature"
        }))
    }
    
    /// mds_addMultisigSignature - Add a signature to a multi-sig transaction
    async fn mds_add_multisig_signature(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let multisig_manager = self.multisig_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Multi-sig manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let obj = params.as_object().ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Params must be an object".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(obj.get("walletAddress").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'walletAddress'".to_string(),
            data: None,
        })?)?;
        
        let tx_hash = parse_hash(obj.get("transactionHash").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'transactionHash'".to_string(),
            data: None,
        })?)?;
        
        let signer = parse_address(obj.get("signer").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'signer' address".to_string(),
            data: None,
        })?)?;
        
        let signature_hex = obj.get("signature").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'signature'".to_string(),
            data: None,
        })?;
        
        let signature = hex::decode(signature_hex.strip_prefix("0x").unwrap_or(signature_hex))
            .map_err(|_| JsonRpcError {
                code: -32602,
                message: "Invalid signature format".to_string(),
                data: None,
            })?;
        
        let public_key_hex = obj.get("publicKey").and_then(|v| v.as_str()).ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing 'publicKey'".to_string(),
            data: None,
        })?;
        
        let public_key = hex::decode(public_key_hex.strip_prefix("0x").unwrap_or(public_key_hex))
            .map_err(|_| JsonRpcError {
                code: -32602,
                message: "Invalid public key format".to_string(),
                data: None,
            })?;
        
        // Add signature to pending transaction
        let mut manager = multisig_manager.write().await;
        match manager.add_signature_to_pending(&wallet_address, &tx_hash, signer, signature, public_key) {
            Ok(_) => {
                // Get updated transaction
                let pending = manager.get_pending_transactions(&wallet_address);
                let tx = pending.iter()
                    .find(|t| t.transaction.hash == tx_hash)
                    .ok_or_else(|| JsonRpcError {
                        code: -32602,
                        message: "Transaction not found".to_string(),
                        data: None,
                    })?;
                
                Ok(json!({
                    "walletAddress": format!("0x{}", hex::encode(wallet_address)),
                    "transactionHash": format!("0x{}", hex::encode(tx_hash)),
                    "signaturesCollected": tx.signature_count(),
                    "signaturesRequired": tx.threshold,
                    "isReady": tx.is_ready(),
                    "signedBy": tx.signed_by().iter().map(|s| format!("0x{}", hex::encode(s))).collect::<Vec<_>>(),
                    "pendingSigners": tx.pending_signers().iter().map(|s| format!("0x{}", hex::encode(s))).collect::<Vec<_>>(),
                    "message": if tx.is_ready() { "Transaction ready to execute" } else { "More signatures needed" }
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: format!("Failed to add signature: {}", e),
                data: None,
            }),
        }
    }
    
    /// mds_getPendingMultisigTransactions - Get pending multi-sig transactions for a wallet
    async fn mds_get_pending_multisig_transactions(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let multisig_manager = self.multisig_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Multi-sig manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(params.as_array()
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing wallet address".to_string(),
                data: None,
            })?)?;
        
        let manager = multisig_manager.read().await;
        let pending = manager.get_pending_transactions(&wallet_address);
        
        let transactions_json: Vec<Value> = pending.iter().map(|tx| {
            json!({
                "transactionHash": format!("0x{}", hex::encode(tx.transaction.hash)),
                "to": format!("0x{}", hex::encode(tx.transaction.to)),
                "value": format!("0x{:x}", tx.transaction.value),
                "fee": format!("0x{:x}", tx.transaction.fee),
                "nonce": format!("0x{:x}", tx.transaction.nonce),
                "signaturesCollected": tx.signature_count(),
                "signaturesRequired": tx.threshold,
                "isReady": tx.is_ready(),
                "signedBy": tx.signed_by().iter().map(|s| format!("0x{}", hex::encode(s))).collect::<Vec<_>>(),
                "pendingSigners": tx.pending_signers().iter().map(|s| format!("0x{}", hex::encode(s))).collect::<Vec<_>>(),
            })
        }).collect();
        
        Ok(json!({
            "walletAddress": format!("0x{}", hex::encode(wallet_address)),
            "pendingTransactions": transactions_json,
            "count": transactions_json.len(),
        }))
    }
    
    /// mds_validateMultisigTransaction - Validate a multi-sig transaction
    async fn mds_validate_multisig_transaction(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Invalid params".to_string(),
            data: None,
        })?;
        
        // This would parse and validate a complete multi-sig transaction
        // For now, return a placeholder
        
        Ok(json!({
            "valid": false,
            "message": "Multi-sig transaction validation - full implementation in progress"
        }))
    }
    
    /// mds_initiateRecovery - Initiate wallet recovery process
    async fn mds_initiate_recovery(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let social_recovery_manager = self.social_recovery_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Social recovery manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing parameters".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(params.get("walletAddress")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'walletAddress'".to_string(),
                data: None,
            })?)?;
        
        let new_owner = parse_address(params.get("newOwner")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'newOwner'".to_string(),
                data: None,
            })?)?;
        
        let guardians: Vec<Address> = params.get("guardians")
            .and_then(|v| v.as_array())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'guardians' array".to_string(),
                data: None,
            })?
            .iter()
            .map(|v| v.as_str().and_then(|s| parse_address(s).ok()))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Invalid guardian addresses".to_string(),
                data: None,
            })?;
        
        let recovery_threshold = params.get("recoveryThreshold")
            .and_then(|v| v.as_u64())
            .map(|v| v as u8)
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'recoveryThreshold'".to_string(),
                data: None,
            })?;
        
        let time_delay = params.get("timeDelay")
            .and_then(|v| v.as_u64());
        
        // Get current timestamp from blockchain
        let blockchain = self.blockchain.read().await;
        let current_timestamp = blockchain.get_blocks()
            .last()
            .map(|b| b.header.timestamp)
            .unwrap_or(0);
        drop(blockchain);
        
        let mut manager = social_recovery_manager.write().await;
        match manager.initiate_recovery(
            wallet_address,
            new_owner,
            guardians.clone(),
            recovery_threshold,
            time_delay,
            current_timestamp,
        ) {
            Ok(request) => {
                Ok(json!({
                    "walletAddress": format!("0x{}", hex::encode(wallet_address)),
                    "newOwner": format!("0x{}", hex::encode(new_owner)),
                    "guardians": guardians.iter().map(|g| format!("0x{}", hex::encode(g))).collect::<Vec<_>>(),
                    "recoveryThreshold": recovery_threshold,
                    "timeDelay": request.time_delay,
                    "initiatedAt": request.initiated_at,
                    "status": match request.status {
                        crate::account_abstraction::RecoveryStatus::Pending => "pending",
                        crate::account_abstraction::RecoveryStatus::Approved => "approved",
                        crate::account_abstraction::RecoveryStatus::Ready => "ready",
                        crate::account_abstraction::RecoveryStatus::Completed => "completed",
                        crate::account_abstraction::RecoveryStatus::Cancelled => "cancelled",
                    },
                    "approvalCount": request.approval_count(),
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: e,
                data: None,
            }),
        }
    }
    
    /// mds_approveRecovery - Approve a recovery request (guardian)
    async fn mds_approve_recovery(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let social_recovery_manager = self.social_recovery_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Social recovery manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing parameters".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(params.get("walletAddress")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'walletAddress'".to_string(),
                data: None,
            })?)?;
        
        let guardian = parse_address(params.get("guardian")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'guardian'".to_string(),
                data: None,
            })?)?;
        
        // Get current timestamp
        let blockchain = self.blockchain.read().await;
        let current_timestamp = blockchain.get_blocks()
            .last()
            .map(|b| b.header.timestamp)
            .unwrap_or(0);
        drop(blockchain);
        
        let mut manager = social_recovery_manager.write().await;
        match manager.approve_recovery(wallet_address, guardian, current_timestamp) {
            Ok(_) => {
                // Get updated status
                let status = manager.get_recovery_status(&wallet_address)
                    .ok_or_else(|| JsonRpcError {
                        code: -32603,
                        message: "Recovery request not found".to_string(),
                        data: None,
                    })?;
                
                Ok(json!({
                    "walletAddress": format!("0x{}", hex::encode(wallet_address)),
                    "status": match status.status {
                        crate::account_abstraction::RecoveryStatus::Pending => "pending",
                        crate::account_abstraction::RecoveryStatus::Approved => "approved",
                        crate::account_abstraction::RecoveryStatus::Ready => "ready",
                        crate::account_abstraction::RecoveryStatus::Completed => "completed",
                        crate::account_abstraction::RecoveryStatus::Cancelled => "cancelled",
                    },
                    "approvalCount": status.approval_count(),
                    "thresholdMet": status.threshold_met(),
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: e,
                data: None,
            }),
        }
    }
    
    /// mds_getRecoveryStatus - Get recovery request status
    async fn mds_get_recovery_status(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let social_recovery_manager = self.social_recovery_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Social recovery manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing parameters".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(params.get("walletAddress")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'walletAddress'".to_string(),
                data: None,
            })?)?;
        
        // Get current timestamp
        let blockchain = self.blockchain.read().await;
        let current_timestamp = blockchain.get_blocks()
            .last()
            .map(|b| b.header.timestamp)
            .unwrap_or(0);
        drop(blockchain);
        
        let mut manager = social_recovery_manager.write().await;
        manager.update_all_statuses(current_timestamp);
        
        let status = manager.get_recovery_status(&wallet_address)
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Recovery request not found".to_string(),
                data: None,
            })?;
        
        let approvals: Vec<Value> = status.approvals.iter()
            .map(|(guardian, timestamp)| {
                json!({
                    "guardian": format!("0x{}", hex::encode(guardian)),
                    "approvedAt": timestamp,
                })
            })
            .collect();
        
        Ok(json!({
            "walletAddress": format!("0x{}", hex::encode(wallet_address)),
            "newOwner": format!("0x{}", hex::encode(status.new_owner)),
            "guardians": status.guardians.iter().map(|g| format!("0x{}", hex::encode(g))).collect::<Vec<_>>(),
            "recoveryThreshold": status.recovery_threshold,
            "timeDelay": status.time_delay,
            "initiatedAt": status.initiated_at,
            "status": match status.status {
                crate::account_abstraction::RecoveryStatus::Pending => "pending",
                crate::account_abstraction::RecoveryStatus::Approved => "approved",
                crate::account_abstraction::RecoveryStatus::Ready => "ready",
                crate::account_abstraction::RecoveryStatus::Completed => "completed",
                crate::account_abstraction::RecoveryStatus::Cancelled => "cancelled",
            },
            "approvalCount": status.approval_count(),
            "thresholdMet": status.threshold_met(),
            "approvals": approvals,
            "isReady": status.is_ready(current_timestamp),
        }))
    }
    
    /// mds_completeRecovery - Complete recovery and transfer wallet ownership
    async fn mds_complete_recovery(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let social_recovery_manager = self.social_recovery_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Social recovery manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing parameters".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(params.get("walletAddress")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'walletAddress'".to_string(),
                data: None,
            })?)?;
        
        // Get current timestamp
        let blockchain = self.blockchain.read().await;
        let current_timestamp = blockchain.get_blocks()
            .last()
            .map(|b| b.header.timestamp)
            .unwrap_or(0);
        drop(blockchain);
        
        let mut manager = social_recovery_manager.write().await;
        match manager.complete_recovery(wallet_address, current_timestamp) {
            Ok(new_owner) => {
                Ok(json!({
                    "walletAddress": format!("0x{}", hex::encode(wallet_address)),
                    "newOwner": format!("0x{}", hex::encode(new_owner)),
                    "status": "completed",
                    "message": "Recovery completed successfully",
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: e,
                data: None,
            }),
        }
    }
    
    /// mds_cancelRecovery - Cancel a recovery request
    async fn mds_cancel_recovery(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let social_recovery_manager = self.social_recovery_manager.as_ref()
            .ok_or_else(|| JsonRpcError {
                code: -32603,
                message: "Social recovery manager not available".to_string(),
                data: None,
            })?;
        
        let params = params.ok_or_else(|| JsonRpcError {
            code: -32602,
            message: "Missing parameters".to_string(),
            data: None,
        })?;
        
        let wallet_address = parse_address(params.get("walletAddress")
            .and_then(|v| v.as_str())
            .ok_or_else(|| JsonRpcError {
                code: -32602,
                message: "Missing 'walletAddress'".to_string(),
                data: None,
            })?)?;
        
        let mut manager = social_recovery_manager.write().await;
        match manager.cancel_recovery(wallet_address) {
            Ok(_) => {
                Ok(json!({
                    "walletAddress": format!("0x{}", hex::encode(wallet_address)),
                    "status": "cancelled",
                    "message": "Recovery request cancelled",
                }))
            }
            Err(e) => Err(JsonRpcError {
                code: -32603,
                message: e,
                data: None,
            }),
        }
    }
}

/// Parse hex address string to Address
fn parse_address(s: &str) -> Result<Address, JsonRpcError> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.len() != 40 {
        return Err(JsonRpcError {
            code: -32602,
            message: "Invalid address length".to_string(),
            data: None,
        });
    }

    let mut address = [0u8; 20];
    hex::decode_to_slice(s, &mut address)
        .map_err(|_| JsonRpcError {
            code: -32602,
            message: "Invalid address format".to_string(),
            data: None,
        })?;

    Ok(address)
}

/// Parse hex hash string to Hash
fn parse_hash(s: &str) -> Result<crate::types::Hash, JsonRpcError> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s.len() != 64 {
        return Err(JsonRpcError {
            code: -32602,
            message: "Invalid hash length".to_string(),
            data: None,
        });
    }

    let mut hash = [0u8; 32];
    hex::decode_to_slice(s, &mut hash)
        .map_err(|_| JsonRpcError {
            code: -32602,
            message: "Invalid hash format".to_string(),
            data: None,
        })?;

    Ok(hash)
}

/// Parse hex number string to u64
fn parse_hex_number(s: &str) -> Result<u64, JsonRpcError> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s == "latest" || s == "pending" {
        // Would need blockchain access - for now return error
        return Err(JsonRpcError {
            code: -32602,
            message: "latest/pending not yet supported".to_string(),
            data: None,
        });
    }

    u64::from_str_radix(s, 16)
        .map_err(|_| JsonRpcError {
            code: -32602,
            message: "Invalid number format".to_string(),
            data: None,
        })
}

/// Parse hex number string to u128
fn parse_hex_u128(s: &str) -> Result<u128, JsonRpcError> {
    let s = s.strip_prefix("0x").unwrap_or(s);
    if s == "latest" || s == "pending" {
        return Err(JsonRpcError {
            code: -32602,
            message: "latest/pending not yet supported".to_string(),
            data: None,
        });
    }

    u128::from_str_radix(s, 16)
        .map_err(|_| JsonRpcError {
            code: -32602,
            message: "Invalid number format".to_string(),
            data: None,
        })
}

/// Convert block to JSON (with optional shard information)
fn block_to_json(block: Option<Block>) -> Value {
    block_to_json_with_shard(block, None)
}

/// Convert block to JSON with shard information
fn block_to_json_with_shard(block: Option<Block>, shard_id: Option<usize>) -> Value {
    match block {
        Some(b) => {
            let mut json = serde_json::json!({
                "number": format!("0x{:x}", b.header.block_number),
                "hash": format!("0x{}", hex::encode(b.hash)),
                "parentHash": b.header.parent_hashes.first()
                    .map(|h| format!("0x{}", hex::encode(h)))
                    .unwrap_or_else(|| "0x0".to_string()),
                "timestamp": format!("0x{:x}", b.header.timestamp),
                "transactions": b.transactions.iter().map(|tx| format!("0x{}", hex::encode(tx.hash))).collect::<Vec<_>>(),
                "transactionCount": b.transactions.len(),
            });
            
            // Add shard information if available
            if let Some(shard) = shard_id {
                json["shardId"] = Value::Number(shard.into());
            }
            
            json
        }
        None => Value::Null,
    }
}

/// Convert transaction to JSON (with optional shard information)
fn tx_to_json(tx: &Transaction, block_number: u64) -> Value {
    tx_to_json_with_shard(tx, block_number, None)
}

/// Convert transaction to JSON with shard information
fn tx_to_json_with_shard(tx: &Transaction, block_number: u64, shard_info: Option<(usize, usize)>) -> Value {
    let mut json = serde_json::json!({
        "hash": format!("0x{}", hex::encode(tx.hash)),
        "from": format!("0x{}", hex::encode(tx.from)),
        "to": format!("0x{}", hex::encode(tx.to)),
        "value": format!("0x{:x}", tx.value),
        "gas": format!("0x{:x}", tx.gas_limit),
        "gasPrice": format!("0x{:x}", tx.fee),
        "nonce": format!("0x{:x}", tx.nonce),
        "blockNumber": format!("0x{:x}", block_number),
        "input": format!("0x{}", hex::encode(&tx.data)),
    });
    
    // Add shard information if available
    if let Some((from_shard, to_shard)) = shard_info {
        json["fromShard"] = Value::Number(from_shard.into());
        json["toShard"] = Value::Number(to_shard.into());
        json["isCrossShard"] = Value::Bool(from_shard != to_shard);
    }
    
    // Add time-lock information if available
    if let Some(execute_at_block) = tx.execute_at_block {
        json["executeAtBlock"] = Value::String(format!("0x{:x}", execute_at_block));
        json["isTimeLocked"] = Value::Bool(true);
    }
    if let Some(execute_at_timestamp) = tx.execute_at_timestamp {
        json["executeAtTimestamp"] = Value::String(format!("0x{:x}", execute_at_timestamp));
        json["isTimeLocked"] = Value::Bool(true);
    }
    
    // Add sponsor information if gasless transaction
    if let Some(sponsor) = tx.sponsor {
        json["sponsor"] = Value::String(format!("0x{}", hex::encode(sponsor)));
        json["isGasless"] = Value::Bool(true);
    }
    
    json
}

