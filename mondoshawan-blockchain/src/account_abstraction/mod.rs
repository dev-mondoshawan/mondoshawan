//! Account Abstraction - Smart Contract Wallets as First-Class Accounts

pub mod wallet;
pub mod factory;
pub mod registry;
pub mod multisig;
pub mod social_recovery;
pub mod batch;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod integration_tests_phase5;

#[cfg(test)]
mod multisig_integration_tests;

pub use wallet::{SmartContractWallet, WalletType, WalletConfig, AuthMethod, SpendingLimits, RecoveryConfig};
pub use factory::WalletFactory;
pub use registry::WalletRegistry;
pub use multisig::{MultiSigTransaction, MultiSigSignature, MultiSigManager, MultiSigValidationResult};
pub use social_recovery::{SocialRecoveryManager, RecoveryRequest, RecoveryStatus};
pub use batch::{BatchTransaction, BatchOperation, BatchManager, BatchStatus, BatchOperationResult, GasEstimate};