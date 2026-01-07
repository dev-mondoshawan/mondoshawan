//! Post-Quantum Cryptography Tooling
//! 
//! Provides CLI and RPC helpers for generating PQ accounts and building PQ transactions

use crate::pqc::accounts::{PqAccount, PqAccountType};
use crate::blockchain::Transaction;
use crate::types::Address;

/// Generate a new PQ account
pub fn generate_pq_account(algorithm: &str) -> Result<PqAccount, String> {
    match algorithm.to_lowercase().as_str() {
        "dilithium3" | "dilithium" => Ok(PqAccount::new_dilithium3()),
        "sphincsplus" | "sphincs" => Ok(PqAccount::new_sphincsplus()),
        _ => Err(format!("Unknown PQ algorithm: {}. Use 'dilithium3' or 'sphincsplus'", algorithm))
    }
}

/// Derive address from PQ account
pub fn derive_address_from_pq_account(account: &PqAccount) -> Address {
    account.address()
}

/// Create a PQ-signed transaction
pub fn create_pq_transaction(
    account: &PqAccount,
    to: Address,
    value: u128,
    fee: u128,
    nonce: u64,
    data: Vec<u8>,
) -> Result<Transaction, String> {
    let from = account.address();
    
    // Create unsigned transaction
    let tx = if data.is_empty() {
        Transaction::new(from, to, value, fee, nonce)
    } else {
        let gas_limit = 21_000 + (data.len() as u64 * 16);
        Transaction::with_data(from, to, value, fee, nonce, data, gas_limit)
    };
    
    // Sign with PQ account
    let signed_tx = tx.sign_pq(account);
    
    Ok(signed_tx)
}

/// Export PQ account keys (for backup/import)
pub fn export_pq_account(account: &PqAccount) -> PqAccountExport {
    PqAccountExport {
        account_type: account.account_type(),
        secret_key: account.secret_key().to_vec(),
        public_key: account.public_key().to_vec(),
        address: account.address(),
    }
}

/// Import PQ account from exported keys
pub fn import_pq_account(export: PqAccountExport) -> Result<PqAccount, String> {
    PqAccount::from_keypair(export.account_type, export.secret_key, export.public_key)
}

/// Get PQ account type from address (requires checking transaction signatures)
/// This is a simplified check - in production, would use on-chain registry
pub fn detect_pq_account_type_from_transaction(tx: &Transaction) -> Option<PqAccountType> {
    tx.pq_signature.as_ref().map(|sig| sig.account_type)
}

/// Format PQ account for display
pub fn format_pq_account(account: &PqAccount) -> String {
    format!("{:?} Account: {}", account.account_type(), hex::encode(account.address()))
}

/// Exported PQ account data (for backup/import)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PqAccountExport {
    pub account_type: PqAccountType,
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: Address,
}

impl PqAccountExport {
    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    /// Deserialize from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    /// Serialize to hex (for CLI)
    pub fn to_hex(&self) -> String {
        hex::encode(bincode::serialize(self).unwrap_or_default())
    }
    
    /// Deserialize from hex
    pub fn from_hex(hex_str: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| format!("Invalid hex: {}", e))?;
        bincode::deserialize(&bytes)
            .map_err(|e| format!("Invalid account data: {}", e))
    }
}

use serde::{Serialize, Deserialize};
