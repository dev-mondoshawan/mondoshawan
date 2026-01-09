//! Forensic analysis and fund tracing
//! 
//! Provides tools for analyzing transaction flows, tracing funds,
//! and generating address summaries for security investigations

use crate::blockchain::{Block, Transaction};
use crate::types::{Address, Hash};
use std::collections::{HashMap, HashSet, VecDeque};

/// Forensic analyzer for transaction flow analysis
pub struct ForensicAnalyzer {
    /// Transaction index: hash -> transaction
    transaction_index: HashMap<Hash, Transaction>,
    /// Address transaction history: address -> list of transaction hashes
    address_tx_history: HashMap<Address, Vec<Hash>>,
    /// Transaction graph: from -> to -> list of transaction hashes
    transaction_graph: HashMap<Address, HashMap<Address, Vec<Hash>>>,
}

/// Fund flow path for tracing
#[derive(Debug, Clone)]
pub struct FundFlow {
    /// Path of addresses the funds moved through
    pub path: Vec<Address>,
    /// Transaction hashes in the path
    pub transactions: Vec<Hash>,
    /// Total value moved
    pub total_value: u128,
    /// Number of hops
    pub hop_count: usize,
}

/// Address summary with forensic information
#[derive(Debug, Clone)]
pub struct AddressSummary {
    /// Address being analyzed
    pub address: Address,
    /// Total value received
    pub total_received: u128,
    /// Total value sent
    pub total_sent: u128,
    /// Net balance change
    pub net_balance: i128,
    /// Number of incoming transactions
    pub incoming_tx_count: u64,
    /// Number of outgoing transactions
    pub outgoing_tx_count: u64,
    /// Unique addresses interacted with
    pub unique_contacts: usize,
    /// First seen timestamp
    pub first_seen: Option<u64>,
    /// Last seen timestamp
    pub last_seen: Option<u64>,
    /// Suspicious patterns detected
    pub suspicious_patterns: Vec<String>,
    /// Risk indicators
    pub risk_indicators: Vec<String>,
}

/// Anomaly detection result
#[derive(Debug, Clone)]
pub struct AnomalyDetection {
    /// Anomaly score (0.0 = normal, 1.0 = highly anomalous)
    pub anomaly_score: f64,
    /// Detected anomalies
    pub anomalies: Vec<Anomaly>,
    /// Confidence level
    pub confidence: f64,
}

/// Detected anomaly
#[derive(Debug, Clone)]
pub struct Anomaly {
    /// Anomaly type
    pub anomaly_type: AnomalyType,
    /// Description
    pub description: String,
    /// Severity (0.0 to 1.0)
    pub severity: f64,
    /// Related addresses
    pub related_addresses: Vec<Address>,
}

/// Types of anomalies
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalyType {
    /// Rapid movement of funds (potential laundering)
    RapidFundMovement,
    /// Circular transactions (potential mixer)
    CircularTransactions,
    /// High-frequency small transactions (potential spam)
    HighFrequencySmallTxs,
    /// Sudden large value transfer (potential theft)
    SuddenLargeTransfer,
    /// Many small inputs to one output (potential mixer)
    ManyToOnePattern,
    /// One input to many outputs (potential distribution)
    OneToManyPattern,
    /// Unusual transaction timing
    UnusualTiming,
    /// Address clustering (potential coordinated activity)
    AddressClustering,
}

impl ForensicAnalyzer {
    /// Create a new forensic analyzer
    pub fn new() -> Self {
        Self {
            transaction_index: HashMap::new(),
            address_tx_history: HashMap::new(),
            transaction_graph: HashMap::new(),
        }
    }
    
    /// Index a transaction for forensic analysis
    pub fn index_transaction(&mut self, tx: &Transaction, block_timestamp: u64) {
        let tx_hash = tx.hash;
        
        // Add to transaction index
        self.transaction_index.insert(tx_hash, tx.clone());
        
        // Update address transaction history
        self.address_tx_history
            .entry(tx.from)
            .or_insert_with(Vec::new)
            .push(tx_hash);
        
        if tx.to != [0u8; 20] {
            self.address_tx_history
                .entry(tx.to)
                .or_insert_with(Vec::new)
                .push(tx_hash);
        }
        
        // Update transaction graph
        if tx.to != [0u8; 20] {
            self.transaction_graph
                .entry(tx.from)
                .or_insert_with(HashMap::new)
                .entry(tx.to)
                .or_insert_with(Vec::new)
                .push(tx_hash);
        }
    }
    
    /// Trace funds from a source address
    pub fn trace_funds(&self, source: Address, max_hops: usize, max_paths: usize) -> Vec<FundFlow> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        // Start with source address
        queue.push_back((vec![source], vec![], 0u128, 0));
        
        while !queue.is_empty() && paths.len() < max_paths {
            let (current_path, current_txs, current_value, hops) = queue.pop_front().unwrap();
            
            if hops >= max_hops {
                continue;
            }
            
            let current_addr = *current_path.last().unwrap();
            
            // Get outgoing transactions from current address
            if let Some(targets) = self.transaction_graph.get(&current_addr) {
                for (target, tx_hashes) in targets {
                    // Avoid cycles (simple check)
                    if current_path.contains(target) {
                        continue;
                    }
                    
                    // Process each transaction
                    for &tx_hash in tx_hashes {
                        if let Some(tx) = self.transaction_index.get(&tx_hash) {
                            let path_key = format!("{:?}-{:?}", current_path, target);
                            if visited.contains(&path_key) {
                                continue;
                            }
                            visited.insert(path_key);
                            
                            let mut new_path = current_path.clone();
                            new_path.push(*target);
                            let mut new_txs = current_txs.clone();
                            new_txs.push(tx_hash);
                            let new_value = current_value + tx.value;
                            
                            // Add to paths if we've reached max hops or found a significant path
                            if hops + 1 >= max_hops || new_value > 1_000_000_000_000_000_000 {
                                paths.push(FundFlow {
                                    path: new_path.clone(),
                                    transactions: new_txs.clone(),
                                    total_value: new_value,
                                    hop_count: hops + 1,
                                });
                            } else {
                                queue.push_back((new_path, new_txs, new_value, hops + 1));
                            }
                        }
                    }
                }
            }
        }
        
        // Sort by value (descending)
        paths.sort_by(|a, b| b.total_value.cmp(&a.total_value));
        paths.truncate(max_paths);
        paths
    }
    
    /// Generate address summary
    pub fn generate_address_summary(&self, address: Address) -> AddressSummary {
        let mut total_received = 0u128;
        let mut total_sent = 0u128;
        let mut incoming_count = 0u64;
        let mut outgoing_count = 0u64;
        let mut contacts = HashSet::new();
        let mut first_seen: Option<u64> = None;
        let mut last_seen: Option<u64> = None;
        let mut suspicious_patterns = Vec::new();
        let mut risk_indicators = Vec::new();
        
        // Analyze all transactions involving this address
        if let Some(tx_hashes) = self.address_tx_history.get(&address) {
            for &tx_hash in tx_hashes {
                if let Some(tx) = self.transaction_index.get(&tx_hash) {
                    // Determine if this is incoming or outgoing
                    if tx.to == address {
                        // Incoming transaction
                        total_received = total_received.saturating_add(tx.value);
                        incoming_count += 1;
                        contacts.insert(tx.from);
                    } else if tx.from == address {
                        // Outgoing transaction
                        total_sent = total_sent.saturating_add(tx.value);
                        outgoing_count += 1;
                        if tx.to != [0u8; 20] {
                            contacts.insert(tx.to);
                        }
                    }
                    
                    // Update timestamps (simplified - would need block timestamps)
                    // For now, we'll use transaction count as a proxy
                }
            }
        }
        
        // Detect suspicious patterns
        if incoming_count > 0 && outgoing_count > 0 {
            let ratio = incoming_count as f64 / outgoing_count as f64;
            if ratio > 10.0 {
                suspicious_patterns.push("high_incoming_to_outgoing_ratio".to_string());
                risk_indicators.push("Potential mixer or laundering".to_string());
            }
        }
        
        if contacts.len() > 100 {
            suspicious_patterns.push("high_contact_count".to_string());
            risk_indicators.push("Interacted with many addresses".to_string());
        }
        
        if total_received > 0 && total_sent > 0 {
            let velocity = (total_received + total_sent) as f64 / (incoming_count + outgoing_count) as f64;
            if velocity > 1_000_000_000_000_000_000_000.0 {
                suspicious_patterns.push("high_transaction_velocity".to_string());
                risk_indicators.push("Very high value per transaction".to_string());
            }
        }
        
        AddressSummary {
            address,
            total_received,
            total_sent,
            net_balance: total_received as i128 - total_sent as i128,
            incoming_tx_count: incoming_count,
            outgoing_tx_count: outgoing_count,
            unique_contacts: contacts.len(),
            first_seen,
            last_seen,
            suspicious_patterns,
            risk_indicators,
        }
    }
    
    /// Detect anomalies for an address
    pub fn detect_anomalies(&self, address: Address) -> AnomalyDetection {
        let mut anomalies = Vec::new();
        let mut anomaly_score: f64 = 0.0;
        
        let summary = self.generate_address_summary(address);
        
        // Check for rapid fund movement
        if summary.incoming_tx_count > 50 && summary.outgoing_tx_count > 50 {
            let avg_time_between = 1.0; // Simplified - would need actual timestamps
            if avg_time_between < 60.0 {
                anomalies.push(Anomaly {
                    anomaly_type: AnomalyType::RapidFundMovement,
                    description: "Rapid movement of funds detected".to_string(),
                    severity: 0.7,
                    related_addresses: vec![address],
                });
                anomaly_score += 0.7;
            }
        }
        
        // Check for many-to-one pattern (mixer)
        if summary.incoming_tx_count > 20 && summary.outgoing_tx_count == 1 {
            anomalies.push(Anomaly {
                anomaly_type: AnomalyType::ManyToOnePattern,
                description: "Many inputs to single output (potential mixer)".to_string(),
                severity: 0.8,
                related_addresses: vec![address],
            });
            anomaly_score += 0.8;
        }
        
        // Check for one-to-many pattern (distribution)
        if summary.incoming_tx_count == 1 && summary.outgoing_tx_count > 20 {
            anomalies.push(Anomaly {
                anomaly_type: AnomalyType::OneToManyPattern,
                description: "Single input to many outputs (potential distribution)".to_string(),
                severity: 0.6,
                related_addresses: vec![address],
            });
            anomaly_score += 0.6;
        }
        
        // Check for high frequency small transactions
        let avg_received = summary.total_received / (summary.incoming_tx_count as u128).max(1);
        if summary.incoming_tx_count > 100 && avg_received < 1000 {
            anomalies.push(Anomaly {
                anomaly_type: AnomalyType::HighFrequencySmallTxs,
                description: "High frequency of small transactions (potential spam)".to_string(),
                severity: 0.5,
                related_addresses: vec![address],
            });
            anomaly_score += 0.5;
        }
        
        // Check for circular transactions
        if let Some(targets) = self.transaction_graph.get(&address) {
            for target in targets.keys() {
                if let Some(reverse_targets) = self.transaction_graph.get(target) {
                    if reverse_targets.contains_key(&address) {
                        anomalies.push(Anomaly {
                            anomaly_type: AnomalyType::CircularTransactions,
                            description: "Circular transaction pattern detected".to_string(),
                            severity: 0.7,
                            related_addresses: vec![address, *target],
                        });
                        anomaly_score += 0.7;
                        break;
                    }
                }
            }
        }
        
        AnomalyDetection {
            anomaly_score: anomaly_score.min(1.0),
            anomalies,
            confidence: if summary.incoming_tx_count + summary.outgoing_tx_count > 10 {
                0.8
            } else {
                0.5
            },
        }
    }
    
    /// Find related addresses (addresses that interacted with the target)
    pub fn find_related_addresses(&self, address: Address, max_results: usize) -> Vec<Address> {
        let mut related = HashSet::new();
        
        if let Some(tx_hashes) = self.address_tx_history.get(&address) {
            for &tx_hash in tx_hashes {
                if let Some(tx) = self.transaction_index.get(&tx_hash) {
                    if tx.from == address && tx.to != [0u8; 20] {
                        related.insert(tx.to);
                    } else if tx.to == address {
                        related.insert(tx.from);
                    }
                }
            }
        }
        
        let mut result: Vec<Address> = related.into_iter().collect();
        result.truncate(max_results);
        result
    }
}

impl Default for ForensicAnalyzer {
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
    fn test_fund_tracing() {
        let mut analyzer = ForensicAnalyzer::new();
        
        let addr1 = [1u8; 20];
        let addr2 = [2u8; 20];
        let addr3 = [3u8; 20];
        
        let tx1 = create_test_transaction(addr1, addr2, 1000);
        let tx2 = create_test_transaction(addr2, addr3, 500);
        
        analyzer.index_transaction(&tx1, 1000);
        analyzer.index_transaction(&tx2, 1001);
        
        let flows = analyzer.trace_funds(addr1, 3, 10);
        assert!(!flows.is_empty());
    }
    
    #[test]
    fn test_address_summary() {
        let mut analyzer = ForensicAnalyzer::new();
        
        let addr1 = [1u8; 20];
        let addr2 = [2u8; 20];
        
        let tx1 = create_test_transaction(addr1, addr2, 1000);
        analyzer.index_transaction(&tx1, 1000);
        
        let summary = analyzer.generate_address_summary(addr1);
        assert_eq!(summary.total_sent, 1000);
        assert_eq!(summary.outgoing_tx_count, 1);
    }
}
