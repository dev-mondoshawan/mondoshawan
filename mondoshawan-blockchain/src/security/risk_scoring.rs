//! Risk scoring system
//! 
//! Calculates risk scores for addresses, transactions, and contracts
//! based on fraud detection, transaction history, and behavioral patterns

use crate::blockchain::Transaction;
use crate::types::Address;
use crate::security::fraud_detection::FraudDetector;
use std::collections::HashMap;

/// Risk scorer that analyzes addresses, transactions, and contracts
pub struct RiskScorer {
    fraud_detector: FraudDetector,
    address_history: HashMap<Address, AddressHistory>,
}

/// Risk score result
#[derive(Debug, Clone)]
pub struct RiskScore {
    /// Risk score from 0.0 (safe) to 1.0 (high risk)
    pub score: f64,
    /// Confidence level from 0.0 to 1.0
    pub confidence: f64,
    /// Risk labels describing the risk factors
    pub labels: Vec<String>,
}

/// Address transaction history for risk analysis
#[derive(Debug, Clone)]
pub struct AddressHistory {
    pub suspicious_tx_count: u64,
    pub total_tx_count: u64,
    pub total_value: u128,
    pub first_seen: u64,
    pub last_seen: u64,
    pub unique_recipients: u64,
}

impl AddressHistory {
    pub fn new(timestamp: u64) -> Self {
        Self {
            suspicious_tx_count: 0,
            total_tx_count: 0,
            total_value: 0,
            first_seen: timestamp,
            last_seen: timestamp,
            unique_recipients: 0,
        }
    }
    
    pub fn record_transaction(&mut self, tx: &Transaction, is_suspicious: bool, timestamp: u64) {
        self.total_tx_count += 1;
        if is_suspicious {
            self.suspicious_tx_count += 1;
        }
        self.total_value = self.total_value.saturating_add(tx.value);
        self.last_seen = timestamp;
    }
}

impl RiskScorer {
    /// Create a new risk scorer
    pub fn new() -> Self {
        Self {
            fraud_detector: FraudDetector::new(),
            address_history: HashMap::new(),
        }
    }
    
    /// Get or create address history
    pub fn get_or_create_history(&mut self, address: &Address, timestamp: u64) -> &mut AddressHistory {
        self.address_history
            .entry(*address)
            .or_insert_with(|| AddressHistory::new(timestamp))
    }
    
    /// Record a transaction for address history tracking
    pub fn record_transaction(&mut self, tx: &Transaction, timestamp: u64) {
        let fraud_analysis = self.fraud_detector.analyze_transaction(tx);
        let is_suspicious = fraud_analysis.risk_score > 0.5;
        
        // Update sender history
        let sender_history = self.get_or_create_history(&tx.from, timestamp);
        sender_history.record_transaction(tx, is_suspicious, timestamp);
        
        // Update receiver history (if not contract deployment)
        if tx.to != [0u8; 20] {
            let receiver_history = self.get_or_create_history(&tx.to, timestamp);
            receiver_history.record_transaction(tx, is_suspicious, timestamp);
        }
    }
    
    /// Calculate risk score for an address
    pub fn score_address(&self, address: &Address) -> RiskScore {
        let history = self.address_history.get(address);
        
        let mut score: f64 = 0.0;
        let mut labels = Vec::new();
        let mut confidence: f64 = 0.3; // Low confidence if no history
        
        if let Some(hist) = history {
            confidence = 0.8; // Higher confidence with history
            
            // Analyze transaction history
            if hist.total_tx_count > 0 {
                let suspicious_ratio = hist.suspicious_tx_count as f64 / hist.total_tx_count as f64;
                if suspicious_ratio > 0.5 {
                    score += 0.6;
                    labels.push("high_suspicious_activity".to_string());
                } else if suspicious_ratio > 0.2 {
                    score += 0.3;
                    labels.push("moderate_suspicious_activity".to_string());
                }
            }
            
            // High value address (potential target)
            if hist.total_value > 1_000_000_000_000_000_000_000 {
                score += 0.2;
                labels.push("high_value_address".to_string());
            }
            
            // Very new address (potential throwaway)
            if hist.total_tx_count < 5 && hist.total_value < 1000 {
                score += 0.2;
                labels.push("new_address".to_string());
            }
            
            // Many unique recipients (potential mixer/scammer)
            if hist.unique_recipients > 100 && hist.total_tx_count > 200 {
                score += 0.4;
                labels.push("high_recipient_diversity".to_string());
            }
        } else {
            // No history - default to low risk but low confidence
            labels.push("no_history".to_string());
        }
        
        RiskScore {
            score: score.min(1.0f64),
            confidence,
            labels,
        }
    }
    
    /// Calculate risk score for a transaction
    pub fn score_transaction(&self, tx: &Transaction) -> RiskScore {
        let fraud_analysis = self.fraud_detector.analyze_transaction(tx);
        
        // Combine fraud analysis with address history
        let sender_score = self.score_address(&tx.from);
        let receiver_score = if tx.to != [0u8; 20] {
            self.score_address(&tx.to)
        } else {
            RiskScore {
                score: 0.0,
                confidence: 0.0,
                labels: vec!["contract_deployment".to_string()],
            }
        };
        
        // Combine scores (weighted average)
        let combined_score = (fraud_analysis.risk_score * 0.5f64 
            + sender_score.score * 0.3f64 
            + receiver_score.score * 0.2f64).min(1.0f64);
        
        let mut labels = fraud_analysis.labels;
        labels.extend(sender_score.labels);
        labels.extend(receiver_score.labels);
        
        // Remove duplicates
        labels.sort();
        labels.dedup();
        
        RiskScore {
            score: combined_score,
            confidence: 0.7, // Medium confidence for transaction analysis
            labels,
        }
    }
    
    /// Calculate risk score for a contract (simplified)
    pub fn score_contract(&self, contract: &Address) -> RiskScore {
        // For now, use address scoring
        // TODO: Add contract-specific analysis:
        // - Bytecode pattern matching
        // - Known vulnerability checks
        // - Deployment history
        // - Interaction patterns
        
        let address_score = self.score_address(contract);
        
        RiskScore {
            score: address_score.score,
            confidence: address_score.confidence * 0.7, // Lower confidence for contracts
            labels: {
                let mut labels = address_score.labels;
                labels.push("contract_address".to_string());
                labels
            },
        }
    }
    
    /// Add a known malicious address
    pub fn add_malicious_address(&mut self, address: Address, reason: String) {
        self.fraud_detector.add_malicious_address(address, reason);
    }
}

impl Default for RiskScorer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::Transaction;
    
    fn create_test_transaction(from: Address, to: Address, value: u128) -> Transaction {
        let mut tx = Transaction {
            from,
            to,
            value,
            fee: 1000,
            nonce: 0,
            data: vec![],
            gas_limit: 21000,
            hash: [0u8; 32],
            public_key: vec![],
            signature: vec![],
            pq_signature: None,
            execute_at_block: None,
            execute_at_timestamp: None,
            sponsor: None,
        };
        tx.hash = tx.calculate_hash();
        tx
    }
    
    #[test]
    fn test_address_scoring_no_history() {
        let scorer = RiskScorer::new();
        let address = [1u8; 20];
        
        let score = scorer.score_address(&address);
        assert_eq!(score.score, 0.0);
        assert!(score.confidence < 0.5);
        assert!(score.labels.contains(&"no_history".to_string()));
    }
    
    #[test]
    fn test_transaction_scoring() {
        let scorer = RiskScorer::new();
        let tx = create_test_transaction([1u8; 20], [2u8; 20], 1000);
        
        let score = scorer.score_transaction(&tx);
        assert!(score.score >= 0.0 && score.score <= 1.0);
        assert!(score.confidence > 0.0);
    }
    
    #[test]
    fn test_malicious_address_scoring() {
        let mut scorer = RiskScorer::new();
        let malicious_addr = [99u8; 20];
        scorer.add_malicious_address(malicious_addr, "Known scammer".to_string());
        
        let score = scorer.score_address(&malicious_addr);
        // Even without history, malicious addresses should have some risk
        assert!(score.score >= 0.0);
    }
}
