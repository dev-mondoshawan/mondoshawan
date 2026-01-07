//! Post-Quantum Cryptography Module
//! 
//! Provides PQ account types (Dilithium/SPHINCS+) and Kyber key exchange
//! for quantum-resistant blockchain operations.

pub mod accounts;
pub mod kyber;
pub mod encryption;
pub mod tooling;

pub use accounts::{PqAccount, PqAccountType, PqSignature};
pub use kyber::{KyberKeyExchange, SessionKey};
pub use encryption::{PqEncryption, EncryptedMessage};
pub use tooling::{generate_pq_account, derive_address_from_pq_account, create_pq_transaction, format_pq_account};