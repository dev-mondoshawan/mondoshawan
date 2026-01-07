//! Rule-based fraud detection
//! 
//! Initial implementation using heuristics and pattern matching
//! for detecting common attack patterns and suspicious behavior

use crate::blockchain::Transaction;
use crate::types::Address;
use std::collections::HashMap;

/// Fraud detector using rule-based heuristics
pub struct FraudDetector {
    known_malicious_addresses: HashMap<Address, String>,
    pattern_rules: Vec<PatternRule>,
}

impl FraudDetector {
    /// Create a new fraud detector
    pub fn new() -> Self {
        Self {
            known_malicious_addresses: HashMap::new(),
            pattern_rules: vec![
                PatternRule::HoneypotPattern,
                PatternRule::MixerPattern,
                PatternRule::PhishingPattern,
                PatternRule::SuspiciousValuePattern,
            ],
        }
    }
    
    /// Add a known malicious address to the blacklist
    pub fn add_malicious_address(&mut self, address: Address, reason: String) {
        self.known_malicious_addresses.insert(address, reason);
    }
    
    /// Analyze a transaction for fraud patterns
    pub fn analyze_transaction(&self, tx: &Transaction) -> FraudAnalysis {
        let mut risk_score = 0.0;
        let mut labels = Vec::new();
        
        // Check known malicious addresses
        if self.known_malicious_addresses.contains_key(&tx.from) {
            risk_score += 0.8;
            labels.push("known_malicious_address".to_string());
        }
        
        if self.known_malicious_addresses.contains_key(&tx.to) {
            risk_score += 0.7;
            labels.push("sending_to_malicious_address".to_string());
        }
        
        // Pattern matching
        for rule in &self.pattern_rules {
            if rule.matches(tx) {
                risk_score += rule.risk_weight();
                labels.push(rule.label().to_string());
            }
        }
        
        // Normalize risk score to 0.0-1.0
        FraudAnalysis {
            risk_score: risk_score.min(1.0f64),
            labels,
        }
    }
}

/// Fraud analysis result
#[derive(Debug, Clone)]
pub struct FraudAnalysis {
    pub risk_score: f64,
    pub labels: Vec<String>,
}

/// Pattern rules for fraud detection
#[derive(Debug, Clone)]
pub enum PatternRule {
    HoneypotPattern,
    MixerPattern,
    PhishingPattern,
    SuspiciousValuePattern,
}

impl PatternRule {
    /// Check if a transaction matches this pattern
    fn matches(&self, tx: &Transaction) -> bool {
        match self {
            PatternRule::HoneypotPattern => {
                // Detect honeypot: contract deployment with small value and no recipient
                tx.to == [0u8; 20] && tx.value < 1000 && !tx.data.is_empty()
            }
            PatternRule::MixerPattern => {
                // Detect mixer: equal value transactions to multiple addresses
                // This is a simplified check - real mixer detection needs transaction graph analysis
                tx.value > 0 && tx.value % 1000 == 0 && tx.data.is_empty()
            }
            PatternRule::PhishingPattern => {
                // Detect phishing: suspicious data patterns
                // Check for common phishing indicators in transaction data
                !tx.data.is_empty() && tx.data.len() < 100 && tx.value == 0
            }
            PatternRule::SuspiciousValuePattern => {
                // Detect suspicious value patterns (e.g., round numbers, very small/large)
                tx.value == 0 || tx.value > 1_000_000_000_000_000_000_000_000_000
            }
        }
    }
    
    /// Get the risk weight for this pattern
    fn risk_weight(&self) -> f64 {
        match self {
            PatternRule::HoneypotPattern => 0.6,
            PatternRule::MixerPattern => 0.7,
            PatternRule::PhishingPattern => 0.9,
            PatternRule::SuspiciousValuePattern => 0.3,
        }
    }
    
    /// Get the label for this pattern
    fn label(&self) -> &str {
        match self {
            PatternRule::HoneypotPattern => "honeypot_pattern",
            PatternRule::MixerPattern => "mixer_pattern",
            PatternRule::PhishingPattern => "phishing_pattern",
            PatternRule::SuspiciousValuePattern => "suspicious_value_pattern",
        }
    }
}

impl Default for FraudDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::block::Transaction;
    use crate::types::Hash;
    
    fn create_test_transaction(from: Address, to: Address, value: u128, data: Vec<u8>) -> Transaction {
        let mut tx = Transaction {
            from,
            to,
            value,
            fee: 1000,
            nonce: 0,
            data,
            gas_limit: 21000,
            hash: [0u8; 32],
            public_key: vec![],
            signature: vec![],
            pq_signature: None,
        };
        tx.hash = tx.calculate_hash();
        tx
    }
    
    #[test]
    fn test_honeypot_detection() {
        let detector = FraudDetector::new();
        let tx = create_test_transaction(
            [1u8; 20],
            [0u8; 20],  // Contract deployment
            100,  // Small value
            vec![1, 2, 3],  // Has data
        );
        
        let analysis = detector.analyze_transaction(&tx);
        assert!(analysis.risk_score > 0.5);
        assert!(analysis.labels.contains(&"honeypot_pattern".to_string()));
    }
    
    #[test]
    fn test_mixer_detection() {
        let detector = FraudDetector::new();
        let tx = create_test_transaction(
            [1u8; 20],
            [2u8; 20],
            10000,  // Round number
            vec![],  // No data
        );
        
        let analysis = detector.analyze_transaction(&tx);
        assert!(analysis.risk_score > 0.0);
    }
    
    #[test]
    fn test_malicious_address_detection() {
        let mut detector = FraudDetector::new();
        let malicious_addr = [99u8; 20];
        detector.add_malicious_address(malicious_addr, "Known scammer".to_string());
        
        let tx = create_test_transaction(
            malicious_addr,
            [2u8; 20],
            1000,
            vec![],
        );
        
        let analysis = detector.analyze_transaction(&tx);
        assert!(analysis.risk_score >= 0.8);
        assert!(analysis.labels.contains(&"known_malicious_address".to_string()));
    }
}
