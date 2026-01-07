//! Kyber Key Exchange
//! 
//! Implements CRYSTALS-Kyber for P2P handshake and session key derivation

use serde::{Deserialize, Serialize};

/// Session key derived from Kyber key exchange (32 bytes)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionKey(pub [u8; 32]);

impl SessionKey {
    /// Create a new session key from bytes
    pub fn new(key: [u8; 32]) -> Self {
        Self(key)
    }
    
    /// Get key bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// Kyber key exchange for P2P handshake
pub struct KyberKeyExchange {
    /// Kyber public key
    public_key: Vec<u8>,
    /// Kyber secret key
    secret_key: Vec<u8>,
}

impl KyberKeyExchange {
    /// Generate a new Kyber keypair
    /// NOTE: Kyber is currently disabled due to Windows/MSVC build issues
    /// This returns a stub implementation that will not work for actual encryption
    pub fn generate() -> Self {
        // Stub implementation - Kyber disabled
        // TODO: Re-enable when pqcrypto-kyber Windows build issues are resolved
        Self {
            public_key: vec![0u8; 1568],  // Kyber1024 public key size
            secret_key: vec![0u8; 3168], // Kyber1024 secret key size
        }
    }
    
    /// Create from existing keypair
    pub fn from_keypair(public_key: Vec<u8>, secret_key: Vec<u8>) -> Result<Self, String> {
        // Verify key sizes (Kyber1024)
        if public_key.len() != 1568 {
            return Err(format!("Invalid public key size: expected 1568, got {}", public_key.len()));
        }
        if secret_key.len() != 3168 {
            return Err(format!("Invalid secret key size: expected 3168, got {}", secret_key.len()));
        }
        
        Ok(Self {
            public_key,
            secret_key,
        })
    }
    
    /// Encapsulate a shared secret (client side)
    /// Returns (ciphertext, shared_secret)
    /// NOTE: Kyber is currently disabled - this returns an error
    pub fn encapsulate(&self, _peer_public_key: &[u8]) -> Result<(Vec<u8>, SessionKey), String> {
        Err("Kyber key exchange is currently disabled due to Windows/MSVC build issues".to_string())
    }
    
    /// Decapsulate a shared secret (server side)
    /// Returns the shared secret
    /// NOTE: Kyber is currently disabled - this returns an error
    pub fn decapsulate(&self, _ciphertext: &[u8]) -> Result<SessionKey, String> {
        Err("Kyber key exchange is currently disabled due to Windows/MSVC build issues".to_string())
    }
    
    /// Get public key
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
    
    /// Get public key as bytes (for serialization)
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public_key.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kyber_key_exchange() {
        // NOTE: Kyber is currently disabled, so this test is skipped
        // Generate two keypairs
        let _alice = KyberKeyExchange::generate();
        let _bob = KyberKeyExchange::generate();
        
        // Test skipped - Kyber functionality disabled
        // When re-enabled, uncomment:
        // let (ciphertext, alice_session) = alice.encapsulate(bob.public_key()).unwrap();
        // let bob_session = bob.decapsulate(&ciphertext).unwrap();
        // assert_eq!(alice_session, bob_session);
    }
}
