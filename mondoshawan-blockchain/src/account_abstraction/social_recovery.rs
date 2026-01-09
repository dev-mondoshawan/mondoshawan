//! Social Recovery System for Smart Contract Wallets
//!
//! Enables wallet recovery via trusted guardians with time-delayed security.

use crate::types::Address;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Recovery request status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecoveryStatus {
    /// Recovery initiated, waiting for guardian approvals
    Pending,
    /// Sufficient approvals received, waiting for time delay
    Approved,
    /// Time delay expired, recovery can be completed
    Ready,
    /// Recovery completed
    Completed,
    /// Recovery cancelled or expired
    Cancelled,
}

/// Recovery request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryRequest {
    /// Wallet address being recovered
    pub wallet_address: Address,
    /// New owner address (who will control the wallet after recovery)
    pub new_owner: Address,
    /// List of guardian addresses
    pub guardians: Vec<Address>,
    /// Number of guardian approvals required
    pub recovery_threshold: u8,
    /// Guardian approvals received (guardian -> approval timestamp)
    pub approvals: HashMap<Address, u64>,
    /// Recovery initiation timestamp
    pub initiated_at: u64,
    /// Time delay in seconds (e.g., 7 days = 604800)
    pub time_delay: u64,
    /// Current status
    pub status: RecoveryStatus,
}

impl RecoveryRequest {
    /// Create a new recovery request
    pub fn new(
        wallet_address: Address,
        new_owner: Address,
        guardians: Vec<Address>,
        recovery_threshold: u8,
        time_delay: u64,
        current_timestamp: u64,
    ) -> Self {
        Self {
            wallet_address,
            new_owner,
            guardians,
            recovery_threshold,
            approvals: HashMap::new(),
            initiated_at: current_timestamp,
            time_delay,
            status: RecoveryStatus::Pending,
        }
    }

    /// Add a guardian approval
    pub fn add_approval(&mut self, guardian: Address, timestamp: u64) -> Result<(), String> {
        // Check if guardian is valid
        if !self.guardians.contains(&guardian) {
            return Err("Guardian not in guardian list".to_string());
        }

        // Check if already approved
        if self.approvals.contains_key(&guardian) {
            return Err("Guardian already approved".to_string());
        }

        // Check if recovery is still pending
        if self.status != RecoveryStatus::Pending {
            return Err("Recovery is not in pending status".to_string());
        }

        // Add approval
        self.approvals.insert(guardian, timestamp);

        // Check if threshold is met
        if self.approvals.len() >= self.recovery_threshold as usize {
            self.status = RecoveryStatus::Approved;
        }

        Ok(())
    }

    /// Check if recovery is ready to complete (time delay expired)
    pub fn is_ready(&self, current_timestamp: u64) -> bool {
        match self.status {
            RecoveryStatus::Approved => {
                let elapsed = current_timestamp.saturating_sub(self.initiated_at);
                elapsed >= self.time_delay
            }
            RecoveryStatus::Ready => true,
            _ => false,
        }
    }

    /// Update status based on current timestamp
    pub fn update_status(&mut self, current_timestamp: u64) {
        if self.status == RecoveryStatus::Approved {
            if self.is_ready(current_timestamp) {
                self.status = RecoveryStatus::Ready;
            }
        }
    }

    /// Get number of approvals received
    pub fn approval_count(&self) -> usize {
        self.approvals.len()
    }

    /// Check if threshold is met
    pub fn threshold_met(&self) -> bool {
        self.approvals.len() >= self.recovery_threshold as usize
    }

    /// Cancel the recovery request
    pub fn cancel(&mut self) {
        self.status = RecoveryStatus::Cancelled;
    }
}

/// Social Recovery Manager
pub struct SocialRecoveryManager {
    /// Active recovery requests (wallet_address -> RecoveryRequest)
    requests: HashMap<Address, RecoveryRequest>,
    /// Default time delay (7 days in seconds)
    default_time_delay: u64,
}

impl SocialRecoveryManager {
    /// Create a new social recovery manager
    pub fn new() -> Self {
        Self {
            requests: HashMap::new(),
            default_time_delay: 7 * 24 * 60 * 60, // 7 days
        }
    }

    /// Create a new recovery request
    pub fn initiate_recovery(
        &mut self,
        wallet_address: Address,
        new_owner: Address,
        guardians: Vec<Address>,
        recovery_threshold: u8,
        time_delay: Option<u64>,
        current_timestamp: u64,
    ) -> Result<RecoveryRequest, String> {
        // Check if recovery already exists
        if self.requests.contains_key(&wallet_address) {
            return Err("Recovery request already exists for this wallet".to_string());
        }

        // Validate threshold
        if recovery_threshold == 0 || recovery_threshold > guardians.len() as u8 {
            return Err("Invalid recovery threshold".to_string());
        }

        // Validate guardians
        if guardians.is_empty() {
            return Err("Guardians list cannot be empty".to_string());
        }

        // Use default time delay if not provided
        let delay = time_delay.unwrap_or(self.default_time_delay);

        // Create recovery request
        let request = RecoveryRequest::new(
            wallet_address,
            new_owner,
            guardians,
            recovery_threshold,
            delay,
            current_timestamp,
        );

        // Store request
        self.requests.insert(wallet_address, request.clone());

        Ok(request)
    }

    /// Add a guardian approval to a recovery request
    pub fn approve_recovery(
        &mut self,
        wallet_address: Address,
        guardian: Address,
        current_timestamp: u64,
    ) -> Result<(), String> {
        let request = self
            .requests
            .get_mut(&wallet_address)
            .ok_or("Recovery request not found")?;

        request.add_approval(guardian, current_timestamp)?;

        // Update status
        request.update_status(current_timestamp);

        Ok(())
    }

    /// Get recovery request status
    pub fn get_recovery_status(&self, wallet_address: &Address) -> Option<&RecoveryRequest> {
        self.requests.get(wallet_address)
    }

    /// Complete recovery (transfer wallet ownership)
    pub fn complete_recovery(
        &mut self,
        wallet_address: Address,
        current_timestamp: u64,
    ) -> Result<Address, String> {
        let request = self
            .requests
            .get_mut(&wallet_address)
            .ok_or("Recovery request not found")?;

        // Update status
        request.update_status(current_timestamp);

        // Check if ready
        if request.status != RecoveryStatus::Ready {
            return Err("Recovery is not ready to complete".to_string());
        }

        // Get new owner
        let new_owner = request.new_owner;

        // Mark as completed
        request.status = RecoveryStatus::Completed;

        Ok(new_owner)
    }

    /// Cancel a recovery request
    pub fn cancel_recovery(&mut self, wallet_address: Address) -> Result<(), String> {
        let request = self
            .requests
            .get_mut(&wallet_address)
            .ok_or("Recovery request not found")?;

        request.cancel();
        Ok(())
    }

    /// Remove completed or cancelled recovery requests (cleanup)
    pub fn cleanup(&mut self) {
        self.requests.retain(|_, req| {
            matches!(
                req.status,
                RecoveryStatus::Pending | RecoveryStatus::Approved | RecoveryStatus::Ready
            )
        });
    }

    /// Get all active recovery requests
    pub fn get_all_requests(&self) -> Vec<&RecoveryRequest> {
        self.requests.values().collect()
    }

    /// Update all recovery request statuses based on current timestamp
    pub fn update_all_statuses(&mut self, current_timestamp: u64) {
        for request in self.requests.values_mut() {
            request.update_status(current_timestamp);
        }
    }
}

impl Default for SocialRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recovery_request_creation() {
        let wallet = Address::from([1; 20]);
        let new_owner = Address::from([2; 20]);
        let guardians = vec![Address::from([3; 20]), Address::from([4; 20]), Address::from([5; 20])];
        let timestamp = 1000;

        let request = RecoveryRequest::new(
            wallet,
            new_owner,
            guardians.clone(),
            2,
            604800, // 7 days
            timestamp,
        );

        assert_eq!(request.wallet_address, wallet);
        assert_eq!(request.new_owner, new_owner);
        assert_eq!(request.guardians, guardians);
        assert_eq!(request.recovery_threshold, 2);
        assert_eq!(request.status, RecoveryStatus::Pending);
        assert_eq!(request.approval_count(), 0);
    }

    #[test]
    fn test_guardian_approval() {
        let wallet = Address::from([1; 20]);
        let new_owner = Address::from([2; 20]);
        let guardian1 = Address::from([3; 20]);
        let guardian2 = Address::from([4; 20]);
        let guardians = vec![guardian1, guardian2];
        let timestamp = 1000;

        let mut request = RecoveryRequest::new(
            wallet,
            new_owner,
            guardians,
            2,
            604800,
            timestamp,
        );

        // Add first approval
        assert!(request.add_approval(Address::from([3; 20]), timestamp + 100).is_ok());
        assert_eq!(request.approval_count(), 1);
        assert_eq!(request.status, RecoveryStatus::Pending);

        // Add second approval (threshold met)
        assert!(request.add_approval(Address::from([4; 20]), timestamp + 200).is_ok());
        assert_eq!(request.approval_count(), 2);
        assert_eq!(request.status, RecoveryStatus::Approved);

        // Try to add duplicate approval
        assert!(request.add_approval(Address::from([3; 20]), timestamp + 300).is_err());
    }

    #[test]
    fn test_recovery_time_delay() {
        let wallet = Address::from([1; 20]);
        let new_owner = Address::from([2; 20]);
        let guardians = vec![Address::from([3; 20]), Address::from([4; 20])];
        let timestamp = 1000;
        let time_delay = 604800; // 7 days

        let mut request = RecoveryRequest::new(
            wallet,
            new_owner,
            guardians,
            2,
            time_delay,
            timestamp,
        );

        // Add approvals
        request.add_approval(Address::from([3; 20]), timestamp + 100).unwrap();
        request.add_approval(Address::from([4; 20]), timestamp + 200).unwrap();

        // Check immediately (not ready)
        assert!(!request.is_ready(timestamp + 1000));

        // Check after time delay (ready)
        assert!(request.is_ready(timestamp + time_delay + 1));
    }

    #[test]
    fn test_social_recovery_manager() {
        let mut manager = SocialRecoveryManager::new();
        let wallet = Address::from([1; 20]);
        let new_owner = Address::from([2; 20]);
        let guardians = vec![Address::from([3; 20]), Address::from([4; 20]), Address::from([5; 20])];
        let timestamp = 1000;

        // Initiate recovery
        let request = manager
            .initiate_recovery(wallet, new_owner, guardians.clone(), 2, None, timestamp)
            .unwrap();

        assert_eq!(request.status, RecoveryStatus::Pending);

        // Add approvals
        manager
            .approve_recovery(wallet, Address::from([3; 20]), timestamp + 100)
            .unwrap();
        manager
            .approve_recovery(wallet, Address::from([4; 20]), timestamp + 200)
            .unwrap();

        // Check status
        let status = manager.get_recovery_status(&wallet).unwrap();
        assert_eq!(status.status, RecoveryStatus::Approved);

        // Complete recovery after time delay
        let final_timestamp = timestamp + manager.default_time_delay + 1;
        let completed_owner = manager.complete_recovery(wallet, final_timestamp).unwrap();
        assert_eq!(completed_owner, new_owner);

        // Verify status
        let status = manager.get_recovery_status(&wallet).unwrap();
        assert_eq!(status.status, RecoveryStatus::Completed);
    }

    #[test]
    fn test_invalid_guardian_approval() {
        let mut manager = SocialRecoveryManager::new();
        let wallet = Address::from([1; 20]);
        let new_owner = Address::from([2; 20]);
        let guardians = vec![Address::from([3; 20]), Address::from([4; 20])];
        let timestamp = 1000;

        manager
            .initiate_recovery(wallet, new_owner, guardians, 2, None, timestamp)
            .unwrap();

        // Try to approve with invalid guardian
        let result = manager.approve_recovery(wallet, Address::from([99; 20]), timestamp + 100);
        assert!(result.is_err());
    }
}
