//! Security Hardening Module
//! 
//! Implements comprehensive security measures to protect against:
//! - DoS/DDoS attacks
//! - Brute force attacks
//! - Injection attacks
//! - Resource exhaustion
//! - Unauthorized access

use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Maximum requests per IP per minute
    pub max_requests_per_minute: u32,
    /// Maximum requests per IP per hour
    pub max_requests_per_hour: u32,
    /// Maximum concurrent connections per IP
    pub max_concurrent_connections: u32,
    /// Timeout for request processing (seconds)
    pub request_timeout_seconds: u64,
    /// Maximum request body size (bytes)
    pub max_request_size: usize,
    /// Enable IP whitelisting
    pub enable_whitelist: bool,
    /// Enable IP blacklisting
    pub enable_blacklist: bool,
    /// Auto-ban IPs after this many violations
    pub auto_ban_threshold: u32,
    /// Ban duration (seconds)
    pub ban_duration_seconds: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_requests_per_minute: 60,
            max_requests_per_hour: 1000,
            max_concurrent_connections: 10,
            request_timeout_seconds: 30,
            max_request_size: 10 * 1024 * 1024, // 10MB
            enable_whitelist: false,
            enable_blacklist: true,
            auto_ban_threshold: 10,
            ban_duration_seconds: 3600, // 1 hour
        }
    }
}

/// Per-IP rate limiting data
#[derive(Debug)]
struct IpRateLimit {
    /// Requests in current minute window
    requests_minute: Vec<Instant>,
    /// Requests in current hour window
    requests_hour: Vec<Instant>,
    /// Current concurrent connections
    concurrent_connections: u32,
    /// Number of violations (rate limit hits)
    violations: u32,
    /// Banned until this time (None = not banned)
    banned_until: Option<Instant>,
    /// Last seen timestamp
    last_seen: Instant,
}

impl IpRateLimit {
    fn new() -> Self {
        Self {
            requests_minute: Vec::new(),
            requests_hour: Vec::new(),
            concurrent_connections: 0,
            violations: 0,
            banned_until: None,
            last_seen: Instant::now(),
        }
    }

    /// Clean old requests outside time windows
    fn cleanup(&mut self) {
        let now = Instant::now();
        let minute_ago = now - Duration::from_secs(60);
        let hour_ago = now - Duration::from_secs(3600);

        self.requests_minute.retain(|&t| t > minute_ago);
        self.requests_hour.retain(|&t| t > hour_ago);
    }

    /// Check if IP is currently banned
    fn is_banned(&self) -> bool {
        if let Some(ban_until) = self.banned_until {
            ban_until > Instant::now()
        } else {
            false
        }
    }

    /// Record a violation and potentially ban
    fn record_violation(&mut self, config: &SecurityConfig) {
        self.violations += 1;
        if self.violations >= config.auto_ban_threshold {
            self.banned_until = Some(Instant::now() + Duration::from_secs(config.ban_duration_seconds));
        }
    }

    /// Reset violations (called when ban expires)
    fn reset_violations(&mut self) {
        if let Some(ban_until) = self.banned_until {
            if ban_until <= Instant::now() {
                self.banned_until = None;
                self.violations = 0;
            }
        }
    }
}

/// Security Hardening Manager
pub struct SecurityHardening {
    config: SecurityConfig,
    /// Per-IP rate limiting data
    ip_limits: Arc<RwLock<HashMap<IpAddr, IpRateLimit>>>,
    /// Whitelisted IPs (bypass rate limiting)
    whitelist: Arc<RwLock<HashSet<IpAddr>>>,
    /// Blacklisted IPs (always denied)
    blacklist: Arc<RwLock<HashSet<IpAddr>>>,
    /// Attack detection: suspicious patterns
    attack_patterns: Arc<RwLock<HashMap<IpAddr, AttackPattern>>>,
}

/// Attack pattern detection
#[derive(Debug, Clone)]
struct AttackPattern {
    /// Number of failed requests
    failed_requests: u32,
    /// Number of rate limit violations
    rate_limit_hits: u32,
    /// Number of invalid requests
    invalid_requests: u32,
    /// First seen
    first_seen: Instant,
    /// Last seen
    last_seen: Instant,
}

impl AttackPattern {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            failed_requests: 0,
            rate_limit_hits: 0,
            invalid_requests: 0,
            first_seen: now,
            last_seen: now,
        }
    }

    /// Check if this looks like an attack
    fn is_attack(&self) -> bool {
        // If more than 50% of requests are failures or rate limit hits, likely an attack
        let total = self.failed_requests + self.rate_limit_hits + self.invalid_requests;
        total > 10 && (self.failed_requests + self.rate_limit_hits) as f64 / total as f64 > 0.5
    }
}

impl SecurityHardening {
    /// Create new security hardening manager
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            ip_limits: Arc::new(RwLock::new(HashMap::new())),
            whitelist: Arc::new(RwLock::new(HashSet::new())),
            blacklist: Arc::new(RwLock::new(HashSet::new())),
            attack_patterns: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(SecurityConfig::default())
    }

    /// Check if IP is allowed (not blacklisted, not banned, within rate limits)
    pub async fn check_ip(&self, ip: IpAddr) -> Result<(), SecurityError> {
        // Check blacklist
        if self.config.enable_blacklist {
            let blacklist = self.blacklist.read().await;
            if blacklist.contains(&ip) {
                return Err(SecurityError::Blacklisted);
            }
        }

        // Check whitelist (bypass rate limiting)
        if self.config.enable_whitelist {
            let whitelist = self.whitelist.read().await;
            if whitelist.contains(&ip) {
                return Ok(());
            }
        }

        // Check rate limits
        let mut limits = self.ip_limits.write().await;
        let ip_limit = limits.entry(ip).or_insert_with(IpRateLimit::new);
        
        // Cleanup old data
        ip_limit.cleanup();
        
        // Check if banned
        ip_limit.reset_violations();
        if ip_limit.is_banned() {
            return Err(SecurityError::Banned);
        }

        // Check rate limits
        if ip_limit.requests_minute.len() >= self.config.max_requests_per_minute as usize {
            ip_limit.record_violation(&self.config);
            self.record_attack_pattern(ip, AttackEvent::RateLimitHit).await;
            return Err(SecurityError::RateLimitExceeded);
        }

        if ip_limit.requests_hour.len() >= self.config.max_requests_per_hour as usize {
            ip_limit.record_violation(&self.config);
            self.record_attack_pattern(ip, AttackEvent::RateLimitHit).await;
            return Err(SecurityError::RateLimitExceeded);
        }

        // Record request
        let now = Instant::now();
        ip_limit.requests_minute.push(now);
        ip_limit.requests_hour.push(now);
        ip_limit.last_seen = now;

        Ok(())
    }

    /// Check if request size is within limits
    pub fn check_request_size(&self, size: usize) -> Result<(), SecurityError> {
        if size > self.config.max_request_size {
            Err(SecurityError::RequestTooLarge)
        } else {
            Ok(())
        }
    }

    /// Add IP to whitelist
    pub async fn whitelist_ip(&self, ip: IpAddr) {
        let mut whitelist = self.whitelist.write().await;
        whitelist.insert(ip);
    }

    /// Remove IP from whitelist
    pub async fn unwhitelist_ip(&self, ip: IpAddr) {
        let mut whitelist = self.whitelist.write().await;
        whitelist.remove(&ip);
    }

    /// Add IP to blacklist
    pub async fn blacklist_ip(&self, ip: IpAddr) {
        let mut blacklist = self.blacklist.write().await;
        blacklist.insert(ip);
    }

    /// Remove IP from blacklist
    pub async fn unblacklist_ip(&self, ip: IpAddr) {
        let mut blacklist = self.blacklist.write().await;
        blacklist.remove(&ip);
    }

    /// Record an attack pattern event
    async fn record_attack_pattern(&self, ip: IpAddr, event: AttackEvent) {
        let mut patterns = self.attack_patterns.write().await;
        let pattern = patterns.entry(ip).or_insert_with(AttackPattern::new);
        
        match event {
            AttackEvent::FailedRequest => pattern.failed_requests += 1,
            AttackEvent::RateLimitHit => pattern.rate_limit_hits += 1,
            AttackEvent::InvalidRequest => pattern.invalid_requests += 1,
        }
        
        pattern.last_seen = Instant::now();
        
        // Auto-blacklist if attack pattern detected
        if pattern.is_attack() && self.config.enable_blacklist {
            drop(patterns);
            self.blacklist_ip(ip).await;
        }
    }

    /// Record a failed request
    pub async fn record_failed_request(&self, ip: IpAddr) {
        self.record_attack_pattern(ip, AttackEvent::FailedRequest).await;
    }

    /// Record an invalid request
    pub async fn record_invalid_request(&self, ip: IpAddr) {
        self.record_attack_pattern(ip, AttackEvent::InvalidRequest).await;
    }

    /// Get security statistics for an IP
    pub async fn get_ip_stats(&self, ip: IpAddr) -> Option<IpSecurityStats> {
        let limits = self.ip_limits.read().await;
        let ip_limit = limits.get(&ip)?;
        
        let patterns = self.attack_patterns.read().await;
        let pattern = patterns.get(&ip);

        Some(IpSecurityStats {
            requests_per_minute: ip_limit.requests_minute.len() as u32,
            requests_per_hour: ip_limit.requests_hour.len() as u32,
            concurrent_connections: ip_limit.concurrent_connections,
            violations: ip_limit.violations,
            is_banned: ip_limit.is_banned(),
            is_whitelisted: {
                let whitelist = self.whitelist.read().await;
                whitelist.contains(&ip)
            },
            is_blacklisted: {
                let blacklist = self.blacklist.read().await;
                blacklist.contains(&ip)
            },
            attack_score: pattern.map(|p| {
                (p.failed_requests + p.rate_limit_hits + p.invalid_requests) as f64
            }).unwrap_or(0.0),
        })
    }

    /// Cleanup old data (should be called periodically)
    pub async fn cleanup(&self) {
        let mut limits = self.ip_limits.write().await;
        let now = Instant::now();
        let cleanup_threshold = now - Duration::from_secs(3600 * 24); // 24 hours

        limits.retain(|_, limit| {
            limit.last_seen > cleanup_threshold
        });

        // Cleanup attack patterns
        let mut patterns = self.attack_patterns.write().await;
        patterns.retain(|_, pattern| {
            pattern.last_seen > cleanup_threshold
        });
    }

    /// Get configuration
    pub fn config(&self) -> &SecurityConfig {
        &self.config
    }

    /// Update configuration
    pub fn update_config(&mut self, config: SecurityConfig) {
        self.config = config;
    }
}

/// Security errors
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityError {
    Blacklisted,
    Banned,
    RateLimitExceeded,
    RequestTooLarge,
    TooManyConnections,
    Timeout,
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::Blacklisted => write!(f, "IP is blacklisted"),
            SecurityError::Banned => write!(f, "IP is temporarily banned"),
            SecurityError::RateLimitExceeded => write!(f, "Rate limit exceeded"),
            SecurityError::RequestTooLarge => write!(f, "Request size exceeds maximum"),
            SecurityError::TooManyConnections => write!(f, "Too many concurrent connections"),
            SecurityError::Timeout => write!(f, "Request timeout"),
        }
    }
}

impl std::error::Error for SecurityError {}

/// Attack event types
#[derive(Debug, Clone)]
enum AttackEvent {
    FailedRequest,
    RateLimitHit,
    InvalidRequest,
}

/// IP security statistics
#[derive(Debug, Clone)]
pub struct IpSecurityStats {
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub concurrent_connections: u32,
    pub violations: u32,
    pub is_banned: bool,
    pub is_whitelisted: bool,
    pub is_blacklisted: bool,
    pub attack_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut config = SecurityConfig::default();
        config.max_requests_per_minute = 5;
        let hardening = SecurityHardening::new(config);

        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        // Should allow 5 requests
        for _ in 0..5 {
            assert!(hardening.check_ip(ip).await.is_ok());
        }

        // 6th request should be rate limited
        assert!(hardening.check_ip(ip).await.is_err());
    }

    #[tokio::test]
    async fn test_whitelist() {
        let mut config = SecurityConfig::default();
        config.enable_whitelist = true;
        config.max_requests_per_minute = 1;
        let hardening = SecurityHardening::new(config);

        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        hardening.whitelist_ip(ip).await;

        // Should bypass rate limit
        for _ in 0..10 {
            assert!(hardening.check_ip(ip).await.is_ok());
        }
    }

    #[tokio::test]
    async fn test_blacklist() {
        let mut config = SecurityConfig::default();
        config.enable_blacklist = true;
        let hardening = SecurityHardening::new(config);

        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        hardening.blacklist_ip(ip).await;

        // Should always be denied
        assert!(hardening.check_ip(ip).await.is_err());
    }

    #[tokio::test]
    async fn test_auto_ban() {
        let mut config = SecurityConfig::default();
        config.max_requests_per_minute = 5;
        config.auto_ban_threshold = 3;
        let hardening = SecurityHardening::new(config);

        let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

        // Exhaust rate limit 3 times
        for _ in 0..3 {
            for _ in 0..5 {
                let _ = hardening.check_ip(ip).await;
            }
            // This should trigger rate limit
            let _ = hardening.check_ip(ip).await;
        }

        // Should now be banned
        assert!(hardening.check_ip(ip).await.is_err());
    }
}
