//! Security policies for opt-in behavior gating
//! 
//! Allows wallets, nodes, and contracts to define security policies
//! that gate behavior based on risk scores and other criteria

use crate::blockchain::block::Transaction;
use crate::types::Address;
use crate::security::risk_scoring::RiskScore;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Security policy that gates behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy ID (unique identifier)
    pub id: String,
    /// Policy name (human-readable)
    pub name: String,
    /// Policy owner (address that created the policy)
    pub owner: Address,
    /// Policy type
    pub policy_type: PolicyType,
    /// Policy action when triggered
    pub action: PolicyAction,
    /// Whether the policy is currently enabled
    pub enabled: bool,
    /// Policy metadata (for extensibility)
    pub metadata: HashMap<String, String>,
}

/// Type of security policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyType {
    /// Reject transactions above a risk score threshold
    MaxRiskScore {
        threshold: f64,
    },
    /// Require risk summary before executing contract
    RequireRiskSummary {
        contract_address: Address,
    },
    /// Block specific addresses
    BlockAddress {
        addresses: Vec<Address>,
    },
    /// Block addresses with specific risk labels
    BlockRiskLabels {
        labels: Vec<String>,
    },
    /// Require minimum confidence for risk scores
    MinConfidence {
        threshold: f64,
    },
    /// Custom policy (for future extensibility)
    Custom {
        policy_id: String,
    },
}

/// Action to take when policy is triggered
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PolicyAction {
    /// Reject the transaction/operation
    Reject {
        reason: String,
    },
    /// Warn but allow (with logging)
    Warn {
        message: String,
    },
    /// Require additional confirmation
    RequireConfirmation {
        message: String,
    },
    /// Log and continue
    Log {
        message: String,
    },
}

/// Policy evaluation result
#[derive(Debug, Clone)]
pub struct PolicyEvaluation {
    /// Whether the policy was triggered
    pub triggered: bool,
    /// Policy that was triggered (if any)
    pub policy: Option<SecurityPolicy>,
    /// Action to take
    pub action: Option<PolicyAction>,
    /// Evaluation message
    pub message: String,
}

/// Security policy manager
pub struct SecurityPolicyManager {
    /// Active policies (indexed by owner)
    policies_by_owner: HashMap<Address, Vec<SecurityPolicy>>,
    /// Global policies (applied to all)
    global_policies: Vec<SecurityPolicy>,
    /// Policy counter for unique IDs
    policy_counter: u64,
}

impl SecurityPolicyManager {
    /// Create a new policy manager
    pub fn new() -> Self {
        Self {
            policies_by_owner: HashMap::new(),
            global_policies: Vec::new(),
            policy_counter: 0,
        }
    }
    
    /// Add a new policy
    pub fn add_policy(&mut self, policy: SecurityPolicy) -> Result<String, String> {
        // Validate policy
        self.validate_policy(&policy)?;
        
        let policy_id = policy.id.clone();
        
        // Add to appropriate collection
        if policy.owner == [0u8; 20] {
            // Global policy
            self.global_policies.push(policy);
        } else {
            // Owner-specific policy
            self.policies_by_owner
                .entry(policy.owner)
                .or_insert_with(Vec::new)
                .push(policy);
        }
        
        Ok(policy_id)
    }
    
    /// Remove a policy
    pub fn remove_policy(&mut self, owner: Address, policy_id: &str) -> Result<(), String> {
        if owner == [0u8; 20] {
            // Remove from global policies
            self.global_policies.retain(|p| p.id != policy_id);
        } else {
            // Remove from owner policies
            if let Some(policies) = self.policies_by_owner.get_mut(&owner) {
                policies.retain(|p| p.id != policy_id);
            }
        }
        Ok(())
    }
    
    /// Enable or disable a policy
    pub fn set_policy_enabled(&mut self, owner: Address, policy_id: &str, enabled: bool) -> Result<(), String> {
        if owner == [0u8; 20] {
            // Update global policy
            if let Some(policy) = self.global_policies.iter_mut().find(|p| p.id == policy_id) {
                policy.enabled = enabled;
                return Ok(());
            }
        } else {
            // Update owner policy
            if let Some(policies) = self.policies_by_owner.get_mut(&owner) {
                if let Some(policy) = policies.iter_mut().find(|p| p.id == policy_id) {
                    policy.enabled = enabled;
                    return Ok(());
                }
            }
        }
        
        Err(format!("Policy not found: {}", policy_id))
    }
    
    /// Get all policies for an owner
    pub fn get_policies(&self, owner: Address) -> Vec<SecurityPolicy> {
        let mut policies = Vec::new();
        
        // Add global policies
        policies.extend(self.global_policies.clone());
        
        // Add owner-specific policies
        if let Some(owner_policies) = self.policies_by_owner.get(&owner) {
            policies.extend(owner_policies.clone());
        }
        
        policies
    }
    
    /// Evaluate a transaction against all applicable policies
    pub fn evaluate_transaction(
        &self,
        tx: &Transaction,
        risk_score: &RiskScore,
        owner: Address,
    ) -> PolicyEvaluation {
        // Get all applicable policies
        let policies = self.get_policies(owner);
        
        // Evaluate each enabled policy
        for policy in policies {
            if !policy.enabled {
                continue;
            }
            
            if self.policy_matches(&policy, tx, risk_score) {
                return PolicyEvaluation {
                    triggered: true,
                    policy: Some(policy.clone()),
                    action: Some(policy.action.clone()),
                    message: format!("Policy '{}' triggered", policy.name),
                };
            }
        }
        
        PolicyEvaluation {
            triggered: false,
            policy: None,
            action: None,
            message: "No policies triggered".to_string(),
        }
    }
    
    /// Check if a policy matches a transaction
    fn policy_matches(&self, policy: &SecurityPolicy, tx: &Transaction, risk_score: &RiskScore) -> bool {
        match &policy.policy_type {
            PolicyType::MaxRiskScore { threshold } => {
                risk_score.score > *threshold
            }
            PolicyType::RequireRiskSummary { contract_address } => {
                tx.to == *contract_address && risk_score.confidence < 0.5
            }
            PolicyType::BlockAddress { addresses } => {
                addresses.contains(&tx.from) || addresses.contains(&tx.to)
            }
            PolicyType::BlockRiskLabels { labels } => {
                labels.iter().any(|label| risk_score.labels.contains(label))
            }
            PolicyType::MinConfidence { threshold } => {
                risk_score.confidence < *threshold
            }
            PolicyType::Custom { .. } => {
                // Custom policies not yet implemented
                false
            }
        }
    }
    
    /// Validate a policy before adding it
    fn validate_policy(&self, policy: &SecurityPolicy) -> Result<(), String> {
        // Validate threshold values
        match &policy.policy_type {
            PolicyType::MaxRiskScore { threshold } => {
                if *threshold < 0.0 || *threshold > 1.0 {
                    return Err("Risk score threshold must be between 0.0 and 1.0".to_string());
                }
            }
            PolicyType::MinConfidence { threshold } => {
                if *threshold < 0.0 || *threshold > 1.0 {
                    return Err("Confidence threshold must be between 0.0 and 1.0".to_string());
                }
            }
            PolicyType::BlockAddress { addresses } => {
                if addresses.is_empty() {
                    return Err("Block address list cannot be empty".to_string());
                }
            }
            PolicyType::BlockRiskLabels { labels } => {
                if labels.is_empty() {
                    return Err("Block risk labels list cannot be empty".to_string());
                }
            }
            _ => {}
        }
        
        // Validate policy ID is unique
        if policy.owner == [0u8; 20] {
            if self.global_policies.iter().any(|p| p.id == policy.id) {
                return Err(format!("Policy ID already exists: {}", policy.id));
            }
        } else {
            if let Some(policies) = self.policies_by_owner.get(&policy.owner) {
                if policies.iter().any(|p| p.id == policy.id) {
                    return Err(format!("Policy ID already exists: {}", policy.id));
                }
            }
        }
        
        Ok(())
    }
    
    /// Generate a unique policy ID
    pub fn generate_policy_id(&mut self) -> String {
        self.policy_counter += 1;
        format!("policy_{}", self.policy_counter)
    }
}

impl Default for SecurityPolicyManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_max_risk_score_policy() {
        let mut manager = SecurityPolicyManager::new();
        
        let policy = SecurityPolicy {
            id: "test_1".to_string(),
            name: "Reject High Risk".to_string(),
            owner: [1u8; 20],
            policy_type: PolicyType::MaxRiskScore { threshold: 0.7 },
            action: PolicyAction::Reject {
                reason: "Risk score too high".to_string(),
            },
            enabled: true,
            metadata: HashMap::new(),
        };
        
        manager.add_policy(policy.clone()).unwrap();
        
        let risk_score = RiskScore {
            score: 0.8,
            confidence: 0.9,
            labels: vec!["high_risk".to_string()],
        };
        
        let tx = Transaction::new([1u8; 20], [2u8; 20], 1000, 100, 0);
        let evaluation = manager.evaluate_transaction(&tx, &risk_score, [1u8; 20]);
        
        assert!(evaluation.triggered);
        assert_eq!(evaluation.policy.as_ref().unwrap().id, "test_1");
    }
    
    #[test]
    fn test_block_address_policy() {
        let mut manager = SecurityPolicyManager::new();
        
        let blocked_address = [9u8; 20];
        
        let policy = SecurityPolicy {
            id: "test_2".to_string(),
            name: "Block Address".to_string(),
            owner: [1u8; 20],
            policy_type: PolicyType::BlockAddress {
                addresses: vec![blocked_address],
            },
            action: PolicyAction::Reject {
                reason: "Address is blocked".to_string(),
            },
            enabled: true,
            metadata: HashMap::new(),
        };
        
        manager.add_policy(policy.clone()).unwrap();
        
        let risk_score = RiskScore {
            score: 0.1,
            confidence: 0.5,
            labels: vec![],
        };
        
        let tx = Transaction::new([1u8; 20], blocked_address, 1000, 100, 0);
        let evaluation = manager.evaluate_transaction(&tx, &risk_score, [1u8; 20]);
        
        assert!(evaluation.triggered);
    }
}
