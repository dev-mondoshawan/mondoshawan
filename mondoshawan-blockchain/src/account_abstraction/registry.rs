//! Wallet Registry - Track and Manage Smart Contract Wallets

use crate::account_abstraction::wallet::SmartContractWallet;
use crate::types::Address;
use std::collections::HashMap;

/// Wallet registry for tracking all smart contract wallets
pub struct WalletRegistry {
    /// Map of wallet address to wallet data
    wallets: HashMap<Address, SmartContractWallet>,
    /// Map of owner address to their wallets
    owner_wallets: HashMap<Address, Vec<Address>>,
}

impl WalletRegistry {
    /// Create new wallet registry
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            owner_wallets: HashMap::new(),
        }
    }

    /// Register a new wallet
    pub fn register_wallet(&mut self, wallet: SmartContractWallet) -> Result<(), String> {
        // Check if wallet address already exists
        if self.wallets.contains_key(&wallet.address) {
            return Err("Wallet address already registered".to_string());
        }

        // Add wallet
        self.wallets.insert(wallet.address, wallet.clone());

        // Track owner's wallets
        self.owner_wallets
            .entry(wallet.owner)
            .or_insert_with(Vec::new)
            .push(wallet.address);

        Ok(())
    }

    /// Get wallet by address
    pub fn get_wallet(&self, address: &Address) -> Option<&SmartContractWallet> {
        self.wallets.get(address)
    }

    /// Get wallet by address (mutable)
    pub fn get_wallet_mut(&mut self, address: &Address) -> Option<&mut SmartContractWallet> {
        self.wallets.get_mut(address)
    }

    /// Check if address is a contract wallet
    pub fn is_contract_wallet(&self, address: &Address) -> bool {
        self.wallets.contains_key(address)
    }

    /// Get all wallets for an owner
    pub fn get_owner_wallets(&self, owner: &Address) -> Vec<&SmartContractWallet> {
        self.owner_wallets
            .get(owner)
            .map(|addresses| {
                addresses
                    .iter()
                    .filter_map(|addr| self.wallets.get(addr))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get total wallet count
    pub fn total_wallets(&self) -> usize {
        self.wallets.len()
    }

    /// Update wallet nonce
    pub fn update_wallet_nonce(&mut self, address: &Address) -> Result<(), String> {
        if let Some(wallet) = self.wallets.get_mut(address) {
            wallet.increment_nonce();
            Ok(())
        } else {
            Err("Wallet not found".to_string())
        }
    }
}

impl Default for WalletRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account_abstraction::wallet::SmartContractWallet;

    #[test]
    fn test_wallet_registry() {
        let mut registry = WalletRegistry::new();
        let owner = [1u8; 20];
        let wallet = SmartContractWallet::new_basic([2u8; 20], owner);

        // Register wallet
        assert!(registry.register_wallet(wallet.clone()).is_ok());

        // Check if it's a contract wallet
        assert!(registry.is_contract_wallet(&wallet.address));

        // Get wallet
        let retrieved = registry.get_wallet(&wallet.address);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().address, wallet.address);

        // Get owner's wallets
        let owner_wallets = registry.get_owner_wallets(&owner);
        assert_eq!(owner_wallets.len(), 1);
    }
}
