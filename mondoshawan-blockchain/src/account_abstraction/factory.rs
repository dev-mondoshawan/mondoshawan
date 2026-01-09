//! Wallet Factory - Create and Deploy Smart Contract Wallets

use crate::account_abstraction::wallet::{SmartContractWallet, WalletType, WalletConfig, SpendingLimits, AuthMethod};
use crate::types::Address;
use sha3::{Digest, Keccak256};

/// Wallet Factory for creating and deploying smart contract wallets
pub struct WalletFactory;

impl WalletFactory {
    /// Create a new wallet factory
    pub fn new() -> Self {
        Self
    }

    /// Derive wallet address from owner and salt
    /// Uses CREATE2-style address derivation for deterministic addresses
    pub fn derive_wallet_address(owner: Address, salt: u64, wallet_type: &WalletType) -> Address {
        let mut hasher = Keccak256::new();
        hasher.update(b"\x19\x01"); // EIP-191 prefix
        hasher.update(&owner);
        hasher.update(&salt.to_le_bytes());
        
        // Include wallet type in hash for uniqueness
        match wallet_type {
            WalletType::Basic => hasher.update(b"basic"),
            WalletType::MultiSig { .. } => hasher.update(b"multisig"),
            WalletType::SocialRecovery { .. } => hasher.update(b"social"),
            WalletType::SpendingLimit { .. } => hasher.update(b"limit"),
            WalletType::Combined { .. } => hasher.update(b"combined"),
        }
        
        let hash = hasher.finalize();
        let mut address = [0u8; 20];
        address.copy_from_slice(&hash[12..32]); // Last 20 bytes
        address
    }

    /// Create a basic wallet
    pub fn create_basic_wallet(owner: Address, salt: u64) -> SmartContractWallet {
        let wallet_type = WalletType::Basic;
        let address = Self::derive_wallet_address(owner, salt, &wallet_type);
        SmartContractWallet::new_basic(address, owner)
    }

    /// Create a multi-signature wallet
    pub fn create_multisig_wallet(
        owner: Address,
        salt: u64,
        signers: Vec<Address>,
        threshold: u8,
    ) -> Result<SmartContractWallet, String> {
        let wallet_type = WalletType::MultiSig {
            signers: signers.clone(),
            threshold,
        };
        let address = Self::derive_wallet_address(owner, salt, &wallet_type);
        SmartContractWallet::new_multisig(address, owner, signers, threshold)
    }

    /// Create a social recovery wallet
    pub fn create_social_recovery_wallet(
        owner: Address,
        salt: u64,
        guardians: Vec<Address>,
        recovery_threshold: u8,
        time_delay: u64,
    ) -> Result<SmartContractWallet, String> {
        let wallet_type = WalletType::SocialRecovery {
            guardians: guardians.clone(),
            recovery_threshold,
        };
        let address = Self::derive_wallet_address(owner, salt, &wallet_type);
        SmartContractWallet::new_social_recovery(address, owner, guardians, recovery_threshold, time_delay)
    }

    /// Create a wallet with spending limits
    pub fn create_spending_limit_wallet(
        owner: Address,
        salt: u64,
        daily_limit: u128,
        weekly_limit: u128,
        monthly_limit: u128,
    ) -> SmartContractWallet {
        let wallet_type = WalletType::SpendingLimit {
            limits: SpendingLimits::new(daily_limit, weekly_limit, monthly_limit),
        };
        let address = Self::derive_wallet_address(owner, salt, &wallet_type);
        
        SmartContractWallet {
            address,
            owner,
            wallet_type,
            config: WalletConfig {
                auth_method: crate::account_abstraction::wallet::AuthMethod::SingleSignature,
                spending_limits: Some(SpendingLimits::new(daily_limit, weekly_limit, monthly_limit)),
                recovery_config: None,
            },
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            nonce: 0,
        }
    }
}

impl Default for WalletFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wallet_address_derivation() {
        let owner = [1u8; 20];
        let salt = 12345u64;
        
        let address1 = WalletFactory::derive_wallet_address(owner, salt, &WalletType::Basic);
        let address2 = WalletFactory::derive_wallet_address(owner, salt, &WalletType::Basic);
        
        // Same owner + salt + type = same address
        assert_eq!(address1, address2);
        
        // Different salt = different address
        let address3 = WalletFactory::derive_wallet_address(owner, 12346, &WalletType::Basic);
        assert_ne!(address1, address3);
        
        // Different type = different address
        let address4 = WalletFactory::derive_wallet_address(owner, salt, &WalletType::MultiSig {
            signers: vec![],
            threshold: 1,
        });
        assert_ne!(address1, address4);
    }

    #[test]
    fn test_create_basic_wallet() {
        let owner = [1u8; 20];
        let wallet = WalletFactory::create_basic_wallet(owner, 0);
        
        assert_eq!(wallet.owner, owner);
        assert_eq!(wallet.wallet_type, WalletType::Basic);
    }

    #[test]
    fn test_create_multisig_wallet() {
        let owner = [1u8; 20];
        let signers = vec![[2u8; 20], [3u8; 20], [4u8; 20]];
        
        let wallet = WalletFactory::create_multisig_wallet(owner, 0, signers, 2).unwrap();
        
        assert_eq!(wallet.owner, owner);
        assert!(wallet.is_multisig());
    }
}
