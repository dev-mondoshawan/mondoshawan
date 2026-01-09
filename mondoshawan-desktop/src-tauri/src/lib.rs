use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::State;

// ----------------------------------------------------------------------------
// Key Management (simple in-memory keystore for MVP)
// ----------------------------------------------------------------------------

struct KeyStore {
    secret_key: Option<[u8; 32]>,
}

impl KeyStore {
    fn new() -> Self {
        Self { secret_key: None }
    }

    fn set_key(&mut self, key: [u8; 32]) {
        self.secret_key = Some(key);
    }

    fn get_key(&self) -> Option<[u8; 32]> {
        self.secret_key
    }

    fn has_key(&self) -> bool {
        self.secret_key.is_some()
    }

    /// Derive address from secret key (simplified: use public key hash)
    fn get_address(&self) -> Option<[u8; 20]> {
        use ed25519_dalek::SigningKey;
        use sha3::{Digest, Keccak256};

        let secret = self.secret_key?;
        let signing_key = SigningKey::from_bytes(&secret);
        let verifying_key = signing_key.verifying_key();
        let pub_bytes = verifying_key.to_bytes();

        // Hash public key with Keccak256 and take last 20 bytes as address
        let mut hasher = Keccak256::new();
        hasher.update(&pub_bytes);
        let result = hasher.finalize();
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&result[12..32]);
        Some(addr)
    }
}

// ----------------------------------------------------------------------------
// Address Book
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
struct Contact {
    name: String,
    address: String,
    notes: Option<String>,
}

struct AddressBook {
    contacts: HashMap<String, Contact>, // key = address
    storage_path: PathBuf,
}

impl AddressBook {
    fn new(storage_path: PathBuf) -> Self {
        let mut book = Self {
            contacts: HashMap::new(),
            storage_path,
        };
        book.load();
        book
    }

    fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.storage_path) {
            if let Ok(contacts) = serde_json::from_str(&data) {
                self.contacts = contacts;
            }
        }
    }

    fn save(&self) -> Result<(), String> {
        let data = serde_json::to_string_pretty(&self.contacts)
            .map_err(|e| format!("Serialize error: {}", e))?;
        fs::write(&self.storage_path, data)
            .map_err(|e| format!("Write error: {}", e))?;
        Ok(())
    }

    fn add_contact(&mut self, contact: Contact) -> Result<(), String> {
        self.contacts.insert(contact.address.clone(), contact);
        self.save()
    }

    fn remove_contact(&mut self, address: &str) -> Result<(), String> {
        self.contacts.remove(address);
        self.save()
    }

    fn get_contacts(&self) -> Vec<Contact> {
        self.contacts.values().cloned().collect()
    }
}

// ----------------------------------------------------------------------------
// Multi-Account Wallet
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
struct Account {
    name: String,
    address: String,
    // Note: We don't store private keys, only addresses. Keys stay in KeyStore.
}

struct Accounts {
    accounts: Vec<Account>,
    storage_path: PathBuf,
}

impl Accounts {
    fn new(storage_path: PathBuf) -> Self {
        let mut accts = Self {
            accounts: Vec::new(),
            storage_path,
        };
        accts.load();
        accts
    }

    fn load(&mut self) {
        if let Ok(data) = fs::read_to_string(&self.storage_path) {
            if let Ok(accounts) = serde_json::from_str(&data) {
                self.accounts = accounts;
            }
        }
    }

    fn save(&self) -> Result<(), String> {
        let data = serde_json::to_string_pretty(&self.accounts)
            .map_err(|e| format!("Serialize error: {}", e))?;
        fs::write(&self.storage_path, data)
            .map_err(|e| format!("Write error: {}", e))?;
        Ok(())
    }

    fn add_account(&mut self, account: Account) -> Result<(), String> {
        // Check for duplicates
        if !self.accounts.iter().any(|a| a.address == account.address) {
            self.accounts.push(account);
            self.save()?;
        }
        Ok(())
    }

    fn remove_account(&mut self, address: &str) -> Result<(), String> {
        self.accounts.retain(|a| a.address != address);
        self.save()
    }

    fn get_accounts(&self) -> Vec<Account> {
        self.accounts.clone()
    }
}

// ----------------------------------------------------------------------------
// RPC Configuration
// ----------------------------------------------------------------------------

#[derive(Clone)]
struct RpcConfig {
    url: String,
    api_key: Option<String>,
}

#[derive(Serialize)]
struct JsonRpcRequest<'a> {
    jsonrpc: &'a str,
    method: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Value>,
    id: u64,
}

#[derive(Deserialize)]
struct JsonRpcResponse {
    #[allow(dead_code)]
    jsonrpc: String,
    result: Option<Value>,
    error: Option<JsonRpcError>,
    #[allow(dead_code)]
    id: Value,
}

#[derive(Deserialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[allow(dead_code)]
    data: Option<Value>,
}

async fn call_rpc(
    cfg: &RpcConfig,
    method: &str,
    params: Option<Value>,
) -> Result<Value, String> {
    let client = reqwest::Client::new();
    let req = JsonRpcRequest {
        jsonrpc: "2.0",
        method,
        params,
        id: 1,
    };

    let mut http_req = client.post(&cfg.url).json(&req);
    if let Some(ref key) = cfg.api_key {
        http_req = http_req.header("X-API-Key", key);
    }

    let resp = http_req.send().await.map_err(|e| e.to_string())?;
    let body: JsonRpcResponse = resp.json().await.map_err(|e| e.to_string())?;

    if let Some(err) = body.error {
        return Err(format!("RPC error {}: {}", err.code, err.message));
    }

    Ok(body.result.unwrap_or(Value::Null))
}

// ----------------------------------------------------------------------------
// Tauri Commands
// ----------------------------------------------------------------------------

#[tauri::command]
async fn get_node_status(state: State<'_, RpcConfig>) -> Result<Value, String> {
    call_rpc(&state, "mds_getNodeStatus", None).await
}

#[tauri::command]
async fn get_mining_status(state: State<'_, RpcConfig>) -> Result<Value, String> {
    call_rpc(&state, "mds_getMiningStatus", None).await
}

#[tauri::command]
async fn start_mining(state: State<'_, RpcConfig>) -> Result<(), String> {
    let _ = call_rpc(&state, "mds_startMining", None).await?;
    Ok(())
}

#[tauri::command]
async fn stop_mining(state: State<'_, RpcConfig>) -> Result<(), String> {
    let _ = call_rpc(&state, "mds_stopMining", None).await?;
    Ok(())
}

#[tauri::command]
async fn get_balance(state: State<'_, RpcConfig>, address: String) -> Result<String, String> {
    let params = Some(serde_json::json!([address, "latest"]));
    let result = call_rpc(&state, "eth_getBalance", params).await?;
    if let Some(balance_str) = result.as_str() {
        Ok(balance_str.to_string())
    } else {
        Err("Unexpected balance format".to_string())
    }
}

#[tauri::command]
async fn get_nonce(state: State<'_, RpcConfig>, address: String) -> Result<String, String> {
    let params = Some(serde_json::json!([address, "latest"]));
    let result = call_rpc(&state, "eth_getTransactionCount", params).await?;
    if let Some(nonce_str) = result.as_str() {
        Ok(nonce_str.to_string())
    } else {
        Err("Unexpected nonce format".to_string())
    }
}

// ----------------------------------------------------------------------------
// Explorer Commands
// ----------------------------------------------------------------------------

#[tauri::command]
async fn get_latest_blocks(state: State<'_, RpcConfig>, count: u64) -> Result<Value, String> {
    // Get current block height
    let height_result = call_rpc(&state, "eth_blockNumber", None).await?;
    let height_str = height_result.as_str().ok_or("Invalid block height")?;
    let height = u64::from_str_radix(height_str.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Invalid height: {}", e))?;

    let mut blocks = Vec::new();
    let start = if height >= count { height - count + 1 } else { 0 };

    for block_num in (start..=height).rev().take(count as usize) {
        let params = Some(serde_json::json!([format!("0x{:x}", block_num), true]));
        if let Ok(block) = call_rpc(&state, "eth_getBlockByNumber", params).await {
            blocks.push(block);
        }
    }

    Ok(serde_json::json!(blocks))
}

#[tauri::command]
async fn get_dag_stats(state: State<'_, RpcConfig>) -> Result<Value, String> {
    call_rpc(&state, "mds_getDagStats", None).await
}

#[tauri::command]
async fn get_tps(state: State<'_, RpcConfig>) -> Result<Value, String> {
    let params = Some(serde_json::json!([60])); // 60 second window
    call_rpc(&state, "mds_getTps", params).await
}

#[tauri::command]
async fn get_shard_stats(state: State<'_, RpcConfig>) -> Result<Value, String> {
    call_rpc(&state, "mds_getShardStats", None).await
}

#[tauri::command]
async fn get_mining_dashboard(state: State<'_, RpcConfig>) -> Result<Value, String> {
    call_rpc(&state, "mds_getMiningDashboard", None).await
}

// ----------------------------------------------------------------------------
// Key Management Commands
// ----------------------------------------------------------------------------

#[tauri::command]
fn create_new_key(keystore: State<'_, Arc<Mutex<KeyStore>>>) -> Result<String, String> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let secret_key: [u8; 32] = rng.gen();

    let mut ks = keystore.lock().map_err(|e| e.to_string())?;
    ks.set_key(secret_key);

    let address = ks.get_address().ok_or("Failed to derive address")?;
    Ok(format!("0x{}", hex::encode(address)))
}

#[tauri::command]
fn import_key(
    keystore: State<'_, Arc<Mutex<KeyStore>>>,
    private_key_hex: String,
) -> Result<String, String> {
    let key_hex = private_key_hex.trim_start_matches("0x");
    let bytes = hex::decode(key_hex).map_err(|e| format!("Invalid hex: {}", e))?;
    if bytes.len() != 32 {
        return Err("Private key must be 32 bytes".to_string());
    }

    let mut secret_key = [0u8; 32];
    secret_key.copy_from_slice(&bytes);

    let mut ks = keystore.lock().map_err(|e| e.to_string())?;
    ks.set_key(secret_key);

    let address = ks.get_address().ok_or("Failed to derive address")?;
    Ok(format!("0x{}", hex::encode(address)))
}

#[tauri::command]
fn get_wallet_address(keystore: State<'_, Arc<Mutex<KeyStore>>>) -> Result<String, String> {
    let ks = keystore.lock().map_err(|e| e.to_string())?;
    if !ks.has_key() {
        return Err("No key loaded. Create or import a key first.".to_string());
    }
    let address = ks.get_address().ok_or("Failed to derive address")?;
    Ok(format!("0x{}", hex::encode(address)))
}

#[tauri::command]
fn export_private_key(keystore: State<'_, Arc<Mutex<KeyStore>>>) -> Result<String, String> {
    let ks = keystore.lock().map_err(|e| e.to_string())?;
    if let Some(key) = ks.get_key() {
        Ok(format!("0x{}", hex::encode(key)))
    } else {
        Err("No key loaded".to_string())
    }
}

// ----------------------------------------------------------------------------
// Transaction History
// ----------------------------------------------------------------------------

#[tauri::command]
async fn get_address_transactions(
    rpc: State<'_, RpcConfig>,
    address: String,
    limit: Option<u64>,
) -> Result<Value, String> {
    let params = Some(serde_json::json!([address, limit.unwrap_or(50)]));
    call_rpc(&rpc, "mds_getAddressTransactions", params).await
}

// ----------------------------------------------------------------------------
// Address Book Commands
// ----------------------------------------------------------------------------

#[tauri::command]
fn add_contact(
    address_book: State<'_, Arc<Mutex<AddressBook>>>,
    name: String,
    address: String,
    notes: Option<String>,
) -> Result<(), String> {
    let mut book = address_book.lock().map_err(|e| e.to_string())?;
    book.add_contact(Contact { name, address, notes })
}

#[tauri::command]
fn remove_contact(
    address_book: State<'_, Arc<Mutex<AddressBook>>>,
    address: String,
) -> Result<(), String> {
    let mut book = address_book.lock().map_err(|e| e.to_string())?;
    book.remove_contact(&address)
}

#[tauri::command]
fn get_contacts(address_book: State<'_, Arc<Mutex<AddressBook>>>) -> Result<Vec<Contact>, String> {
    let book = address_book.lock().map_err(|e| e.to_string())?;
    Ok(book.get_contacts())
}

// ----------------------------------------------------------------------------
// Multi-Account Commands
// ----------------------------------------------------------------------------

#[tauri::command]
fn add_account(
    accounts: State<'_, Arc<Mutex<Accounts>>>,
    name: String,
    address: String,
) -> Result<(), String> {
    let mut accts = accounts.lock().map_err(|e| e.to_string())?;
    accts.add_account(Account { name, address })
}

#[tauri::command]
fn remove_account(
    accounts: State<'_, Arc<Mutex<Accounts>>>,
    address: String,
) -> Result<(), String> {
    let mut accts = accounts.lock().map_err(|e| e.to_string())?;
    accts.remove_account(&address)
}

#[tauri::command]
fn get_accounts(accounts: State<'_, Arc<Mutex<Accounts>>>) -> Result<Vec<Account>, String> {
    let accts = accounts.lock().map_err(|e| e.to_string())?;
    Ok(accts.get_accounts())
}

// ----------------------------------------------------------------------------
// Transaction Signing & Sending
// ----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
struct Transaction {
    from: [u8; 20],
    to: [u8; 20],
    value: u128,
    fee: u128,
    nonce: u64,
    data: Vec<u8>,
    gas_limit: u64,
    hash: [u8; 32],
    signature: Vec<u8>,
    public_key: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pq_signature: Option<Value>, // placeholder for PQ
}

impl Transaction {
    fn calculate_hash(&self) -> [u8; 32] {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(&self.from);
        hasher.update(&self.to);
        hasher.update(&self.value.to_le_bytes());
        hasher.update(&self.fee.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.data);
        hasher.update(&self.gas_limit.to_le_bytes());
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result[..]);
        hash
    }

    fn sign(mut self, secret_key: &[u8; 32]) -> Self {
        use ed25519_dalek::{Signer, SigningKey};

        let signing_key = SigningKey::from_bytes(secret_key);
        let verifying_key = signing_key.verifying_key();
        let public_key_bytes: [u8; 32] = verifying_key.to_bytes();

        self.public_key = public_key_bytes.to_vec();

        let message = &self.hash;
        let signature = signing_key.sign(message);

        self.signature = signature.to_bytes().into();

        self
    }
}

#[tauri::command]
async fn send_transaction(
    rpc: State<'_, RpcConfig>,
    keystore: State<'_, Arc<Mutex<KeyStore>>>,
    to_address: String,
    value_hex: String,
    fee_hex: String,
) -> Result<String, String> {
    // Parse to address
    let to_hex = to_address.trim_start_matches("0x");
    let to_bytes = hex::decode(to_hex).map_err(|e| format!("Invalid to address: {}", e))?;
    if to_bytes.len() != 20 {
        return Err("To address must be 20 bytes".to_string());
    }
    let mut to = [0u8; 20];
    to.copy_from_slice(&to_bytes);

    // Parse value and fee
    let value = u128::from_str_radix(value_hex.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Invalid value: {}", e))?;
    let fee = u128::from_str_radix(fee_hex.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Invalid fee: {}", e))?;

    // Get secret key and from address (scope the lock)
    let (secret_key, from) = {
        let ks = keystore.lock().map_err(|e| e.to_string())?;
        let secret_key = ks.get_key().ok_or("No key loaded")?;
        let from = ks.get_address().ok_or("Failed to derive address")?;
        (secret_key, from)
    }; // Lock is dropped here

    // Get current nonce from node
    let from_str = format!("0x{}", hex::encode(from));
    let nonce_hex = get_nonce(rpc.clone(), from_str).await?;
    let nonce = u64::from_str_radix(nonce_hex.trim_start_matches("0x"), 16)
        .map_err(|e| format!("Invalid nonce: {}", e))?;

    // Build transaction
    let mut tx = Transaction {
        from,
        to,
        value,
        fee,
        nonce,
        data: vec![],
        gas_limit: 21_000,
        hash: [0; 32],
        signature: vec![],
        public_key: vec![],
        pq_signature: None,
    };
    tx.hash = tx.calculate_hash();
    tx = tx.sign(&secret_key);

    // Send via RPC
    let tx_json = serde_json::to_value(&tx).map_err(|e| format!("Failed to serialize: {}", e))?;
    let params = Some(serde_json::json!([tx_json]));
    let result = call_rpc(&rpc, "mds_sendRawTransaction", params).await?;

    // Extract tx hash from result
    if let Some(hash_str) = result.get("hash").and_then(|h| h.as_str()) {
        Ok(hash_str.to_string())
    } else {
        Err("Unexpected response from node".to_string())
    }
}

// ----------------------------------------------------------------------------
// Main Entry Point
// ----------------------------------------------------------------------------

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let keystore = Arc::new(Mutex::new(KeyStore::new()));
    
    // Get app data directory for storage (use current dir as fallback for MVP)
    let app_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    std::fs::create_dir_all(&app_dir).ok();
    
    let address_book = Arc::new(Mutex::new(AddressBook::new(
        app_dir.join("address_book.json")
    )));
    let accounts = Arc::new(Mutex::new(Accounts::new(
        app_dir.join("accounts.json")
    )));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(RpcConfig {
            url: "http://127.0.0.1:8545".to_string(),
            api_key: None,
        })
        .manage(keystore)
        .manage(address_book)
        .manage(accounts)
        .invoke_handler(tauri::generate_handler![
            get_node_status,
            get_mining_status,
            start_mining,
            stop_mining,
            get_balance,
            get_nonce,
            create_new_key,
            import_key,
            get_wallet_address,
            export_private_key,
            send_transaction,
            get_latest_blocks,
            get_dag_stats,
            get_tps,
            get_shard_stats,
            get_address_transactions,
            add_contact,
            remove_contact,
            get_contacts,
            add_account,
            remove_account,
            get_accounts,
            get_mining_dashboard,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Mondoshawan Desktop");
}
