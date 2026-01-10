//! Security module for AI-driven fraud detection and risk scoring
//! 
//! Provides protocol-level security services including:
//! - Fraud and anomaly detection
//! - Risk scoring for addresses, transactions, and contracts
//! - Security labels and threat classification
//! - Forensic analysis and fund tracing
//! - Security hardening (DoS protection, rate limiting, IP filtering)

pub mod fraud_detection;
pub mod risk_scoring;
pub mod forensics;
pub mod policies;
pub mod hardening;

pub use fraud_detection::{FraudDetector, FraudAnalysis, PatternRule};
pub use risk_scoring::{RiskScorer, RiskScore, AddressHistory};
pub use forensics::{ForensicAnalyzer, FundFlow, AddressSummary, AnomalyDetection, Anomaly, AnomalyType};
pub use policies::{SecurityPolicyManager, SecurityPolicy, PolicyType, PolicyAction, PolicyEvaluation};
pub use hardening::{SecurityHardening, SecurityConfig, SecurityError, IpSecurityStats};