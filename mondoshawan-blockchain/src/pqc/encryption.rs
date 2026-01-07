//! Post-Quantum Encryption
//! 
//! Provides encrypted P2P communication using session keys derived from Kyber

use crate::pqc::kyber::SessionKey;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use serde::{Deserialize, Serialize};

/// Encrypted message for P2P communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Nonce used for encryption
    pub nonce: Vec<u8>,
    /// Encrypted ciphertext
    pub ciphertext: Vec<u8>,
}

impl EncryptedMessage {
    /// Create a new encrypted message
    pub fn new(nonce: Vec<u8>, ciphertext: Vec<u8>) -> Self {
        Self { nonce, ciphertext }
    }
}

/// Post-Quantum Encryption handler
pub struct PqEncryption;

impl PqEncryption {
    /// Encrypt a message using a session key
    pub fn encrypt(message: &[u8], session_key: &SessionKey) -> Result<EncryptedMessage, String> {
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(session_key.as_bytes());
        let cipher = Aes256Gcm::new(key);
        
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = cipher.encrypt(&nonce, message)
            .map_err(|e| format!("Encryption failed: {:?}", e))?;
        
        Ok(EncryptedMessage::new(nonce.to_vec(), ciphertext))
    }
    
    /// Decrypt a message using a session key
    pub fn decrypt(encrypted: &EncryptedMessage, session_key: &SessionKey) -> Result<Vec<u8>, String> {
        let key = aes_gcm::Key::<Aes256Gcm>::from_slice(session_key.as_bytes());
        let cipher = Aes256Gcm::new(key);
        
        let nonce = Nonce::from_slice(&encrypted.nonce);
        let plaintext = cipher.decrypt(nonce, encrypted.ciphertext.as_ref())
            .map_err(|e| format!("Decryption failed: {:?}", e))?;
        
        Ok(plaintext)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_decryption() {
        let session_key = SessionKey::new([42u8; 32]);
        let message = b"Hello, quantum-resistant world!";
        
        let encrypted = PqEncryption::encrypt(message, &session_key).unwrap();
        let decrypted = PqEncryption::decrypt(&encrypted, &session_key).unwrap();
        
        assert_eq!(message, decrypted.as_slice());
    }
}
