//! Block and transaction structures

use crate::types::{Address, Hash, StreamType};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};

/// Block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub parent_hashes: Vec<Hash>,
    pub block_number: u64,
    pub stream_type: StreamType,
    pub difficulty: u64,
    pub timestamp: u64,
}

impl BlockHeader {
    pub fn new(parent_hashes: Vec<Hash>, block_number: u64, stream_type: StreamType, difficulty: u64) -> Self {
        Self {
            parent_hashes,
            block_number,
            stream_type,
            difficulty,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Transaction signature (64 bytes for Ed25519)
/// Using Vec<u8> for serde compatibility
pub type TransactionSignature = Vec<u8>;

/// Public key (32 bytes for Ed25519)
/// Using Vec<u8> for serde compatibility
pub type PublicKey = Vec<u8>;

/// Transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub value: u128,
    pub fee: u128,
    pub nonce: u64,
    pub data: Vec<u8>,
    pub gas_limit: u64,
    pub hash: Hash,
    /// Ed25519 signature (64 bytes) - CRITICAL for security
    /// If empty, transaction is unsigned (only allowed for genesis/system transactions)
    /// For PQ accounts, this field is empty and pq_signature is used instead
    pub signature: TransactionSignature,
    /// Ed25519 public key (32 bytes) - required for signature verification
    /// If empty, transaction is unsigned (only allowed for genesis/system transactions)
    /// For PQ accounts, this field is empty and pq_signature is used instead
    pub public_key: PublicKey,
    /// Post-Quantum signature (optional, for PQ accounts)
    /// If present, this is used instead of Ed25519 signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pq_signature: Option<crate::pqc::PqSignature>,
    /// Time-locked transaction: Execute at this block number (0 = immediate)
    /// If set, transaction will only be processed when current block >= execute_at_block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_at_block: Option<u64>,
    /// Time-locked transaction: Execute at this Unix timestamp (0 = immediate)
    /// If set, transaction will only be processed when block timestamp >= execute_at_timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_at_timestamp: Option<u64>,
    /// Gasless transaction: Address that sponsors (pays for) this transaction's fee
    /// If set, the sponsor's balance is checked and debited instead of the sender's
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sponsor: Option<Address>,
    /// Multi-signature support (for contract wallets)
    /// If present, this transaction requires multiple signatures
    /// Format: Vec<(signer_address, signature_bytes, public_key_bytes)>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multisig_signatures: Option<Vec<(Address, Vec<u8>, Vec<u8>)>>,
    /// Privacy transaction: zk-SNARK proof and privacy data
    /// If present, this is a private transaction (hidden sender, receiver, amount)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_data: Option<crate::privacy::PrivacyTransaction>,
}

impl Transaction {
    pub fn new(from: Address, to: Address, value: u128, fee: u128, nonce: u64) -> Self {
        let mut tx = Self {
            from,
            to,
            value,
            fee,
            nonce,
            data: Vec::new(),
            gas_limit: 21_000,
            hash: [0; 32],
            signature: vec![0; 64], // Unsigned - must be signed before use
            public_key: vec![], // No public key for unsigned transactions
            pq_signature: None, // No PQ signature initially
            execute_at_block: None, // Immediate execution
            execute_at_timestamp: None, // Immediate execution
            sponsor: None, // No sponsor (sender pays fee)
            multisig_signatures: None, // No multi-sig initially
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    pub fn with_data(from: Address, to: Address, value: u128, fee: u128, nonce: u64, data: Vec<u8>, gas_limit: u64) -> Self {
        let mut tx = Self {
            from,
            to,
            value,
            fee,
            nonce,
            data,
            gas_limit,
            hash: [0; 32],
            signature: vec![0; 64], // Unsigned - must be signed before use
            public_key: vec![], // No public key for unsigned transactions
            pq_signature: None, // No PQ signature initially
            execute_at_block: None, // Immediate execution
            execute_at_timestamp: None, // Immediate execution
            sponsor: None, // No sponsor (sender pays fee)
            multisig_signatures: None, // No multi-sig initially
        };
        tx.hash = tx.calculate_hash();
        tx
    }

    /// Sign a transaction with Ed25519
    /// 
    /// # Arguments
    /// * `secret_key` - 32-byte Ed25519 secret key
    /// 
    /// # Returns
    /// The transaction with signature and public key set
    pub fn sign(mut self, secret_key: &[u8; 32]) -> Self {
        use ed25519_dalek::{SigningKey, Signer};
        
        // Create signing key from bytes
        let signing_key = SigningKey::from_bytes(secret_key);
        
        // Get the public key (verifying key)
        let verifying_key = signing_key.verifying_key();
        let public_key_bytes: [u8; 32] = verifying_key.to_bytes();
        
        // Store public key
        self.public_key = public_key_bytes.to_vec();
        
        // Sign the transaction hash (before signature is added)
        let message = &self.hash;
        let signature = signing_key.sign(message);
        
        // Store signature
        self.signature = signature.to_bytes().into();
        
        // Note: We don't recalculate hash after signing because the hash
        // should be calculated before signing (signature signs the hash)
        
        self
    }

    /// Sign a transaction with a PQ account
    pub fn sign_pq(mut self, pq_account: &crate::pqc::PqAccount) -> Self {
        let signature = pq_account.sign(&self.hash);
        self.pq_signature = Some(signature);
        // Clear Ed25519 fields for PQ transactions
        self.signature = vec![];
        self.public_key = vec![];
        self
    }

    /// Verify transaction signature
    /// 
    /// # Returns
    /// `true` if signature is valid, `false` otherwise
    /// 
    /// This implementation:
    /// 1. Allows unsigned transactions only for system/genesis transactions (from = zero address)
    /// 2. Verifies the signature using the stored public key
    /// 3. Optionally verifies that the address matches the public key hash (if implemented)
    pub fn verify_signature(&self) -> bool {
        // Check for PQ signature first
        if let Some(ref pq_sig) = self.pq_signature {
            return crate::pqc::PqAccount::verify_signature(&self.hash, pq_sig);
        }
        
        // Fall back to Ed25519 verification
        use ed25519_dalek::{VerifyingKey, Signature, Verifier};
        use sha3::{Digest, Keccak256};
        
        // Allow unsigned transactions only if from address is zero (system/genesis)
        if self.signature.is_empty() || self.signature.iter().all(|&b| b == 0) {
            return self.from == [0u8; 20] && self.public_key.is_empty();
        }
        
        // Must have both signature and public key for signed transactions
        if self.signature.len() != 64 {
            return false;
        }
        
        if self.public_key.len() != 32 {
            return false;
        }
        
        // Parse public key
        let pub_key_bytes: [u8; 32] = match self.public_key.as_slice().try_into() {
            Ok(b) => b,
            Err(_) => return false,
        };
        
        let verifying_key = match VerifyingKey::from_bytes(&pub_key_bytes) {
            Ok(key) => key,
            Err(_) => return false,
        };
        
        // Parse signature
        let sig_bytes: [u8; 64] = match self.signature.as_slice().try_into() {
            Ok(b) => b,
            Err(_) => return false,
        };
        
        let signature = match Signature::try_from(&sig_bytes[..]) {
            Ok(s) => s,
            Err(_) => return false,
        };
        
        // Verify signature against transaction hash
        match verifying_key.verify(&self.hash, &signature) {
            Ok(_) => {
                // Optional: Verify that address matches public key hash
                // This ensures the address was derived from the public key
                // For now, we'll do a simple check: address should be last 20 bytes of Keccak256(public_key)
                let mut hasher = Keccak256::new();
                hasher.update(&pub_key_bytes);
                let hash = hasher.finalize();
                let derived_address: [u8; 20] = {
                    let mut addr = [0u8; 20];
                    addr.copy_from_slice(&hash[12..32]); // Last 20 bytes (Ethereum-style)
                    addr
                };
                
                // Verify address matches (or allow if not enforced for backward compatibility)
                // For now, we'll verify it matches
                derived_address == self.from
            }
            Err(_) => false,
        }
    }

    /// Create a time-locked transaction that executes at a specific block
    pub fn with_execute_at_block(mut self, block_number: u64) -> Self {
        self.execute_at_block = Some(block_number);
        self.hash = self.calculate_hash();
        self
    }

    /// Create a time-locked transaction that executes at a specific timestamp
    pub fn with_execute_at_timestamp(mut self, timestamp: u64) -> Self {
        self.execute_at_timestamp = Some(timestamp);
        self.hash = self.calculate_hash();
        self
    }

    /// Create a gasless transaction sponsored by another address
    pub fn with_sponsor(mut self, sponsor: Address) -> Self {
        self.sponsor = Some(sponsor);
        self.hash = self.calculate_hash();
        self
    }

    /// Add multi-signature support to transaction
    pub fn with_multisig_signatures(mut self, signatures: Vec<(Address, Vec<u8>, Vec<u8>)>) -> Self {
        self.multisig_signatures = Some(signatures);
        self.hash = self.calculate_hash();
        self
    }

    /// Check if transaction is multi-signature
    pub fn is_multisig(&self) -> bool {
        self.multisig_signatures.is_some()
    }

    /// Check if transaction is ready to execute (time-lock conditions met)
    pub fn is_ready_to_execute(&self, current_block: u64, current_timestamp: u64) -> bool {
        // Check block-based time lock
        if let Some(execute_at_block) = self.execute_at_block {
            if current_block < execute_at_block {
                return false;
            }
        }
        
        // Check timestamp-based time lock
        if let Some(execute_at_timestamp) = self.execute_at_timestamp {
            if current_timestamp < execute_at_timestamp {
                return false;
            }
        }
        
        true
    }

    /// Check if transaction is gasless (has a sponsor)
    pub fn is_gasless(&self) -> bool {
        self.sponsor.is_some()
    }

    /// Calculate transaction hash (public for validation)
    /// Hash includes all fields except signature and public_key (signature signs this hash)
    pub fn calculate_hash(&self) -> Hash {
        let mut hasher = Keccak256::new();
        hasher.update(&self.from);
        hasher.update(&self.to);
        hasher.update(&self.value.to_le_bytes());
        hasher.update(&self.fee.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.data);
        hasher.update(&self.gas_limit.to_le_bytes());
        // Include time-lock fields in hash
        if let Some(block) = self.execute_at_block {
            hasher.update(&block.to_le_bytes());
        }
        if let Some(timestamp) = self.execute_at_timestamp {
            hasher.update(&timestamp.to_le_bytes());
        }
        // Include sponsor in hash
        if let Some(sponsor) = self.sponsor {
            hasher.update(&sponsor);
        }
        // Note: signature and public_key are NOT included in hash (signature signs this hash)
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
    
    /// Derive address from public key (Ethereum-style: Keccak256(public_key)[12:32])
    pub fn derive_address_from_public_key(public_key: &[u8; 32]) -> Address {
        use sha3::{Digest, Keccak256};
        let mut hasher = Keccak256::new();
        hasher.update(public_key);
        let hash = hasher.finalize();
        let mut addr = [0u8; 20];
        addr.copy_from_slice(&hash[12..32]); // Last 20 bytes
        addr
    }
}

/// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub hash: Hash,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>, parent_hashes: Vec<Hash>) -> Self {
        let mut block = Self {
            header,
            transactions,
            hash: [0; 32],
        };
        block.header.parent_hashes = parent_hashes;
        block.hash = block.calculate_hash();
        block
    }

    /// Calculate block hash (public for validation)
    pub fn calculate_hash(&self) -> Hash {
        let mut hasher = Keccak256::new();
        for parent in &self.header.parent_hashes {
            hasher.update(parent);
        }
        hasher.update(&self.header.block_number.to_le_bytes());
        hasher.update(&self.header.difficulty.to_le_bytes());
        hasher.update(&self.header.timestamp.to_le_bytes());
        for tx in &self.transactions {
            hasher.update(&tx.hash);
        }
        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

