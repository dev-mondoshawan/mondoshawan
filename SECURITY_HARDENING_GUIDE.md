# Security Hardening Guide

**Mondoshawan Blockchain - Production Security Hardening**

---

## 🛡️ Overview

This guide documents the comprehensive security hardening measures implemented in the Mondoshawan blockchain to protect against:

- **DoS/DDoS Attacks**: Rate limiting, IP filtering, request size limits
- **Brute Force Attacks**: Auto-banning, violation tracking
- **Injection Attacks**: Input validation, sanitization
- **Resource Exhaustion**: Transaction pool limits, block size limits
- **Unauthorized Access**: API key authentication, IP whitelisting/blacklisting
- **Network Attacks**: Message size limits, per-peer rate limiting

---

## 🔒 Security Features

### 1. **Per-IP Rate Limiting**

**Purpose**: Prevent a single IP from overwhelming the RPC server.

**Configuration**:
```rust
use mondoshawan_blockchain::security::{SecurityHardening, SecurityConfig};

let mut config = SecurityConfig::default();
config.max_requests_per_minute = 60;  // 60 requests per minute per IP
config.max_requests_per_hour = 1000;   // 1000 requests per hour per IP
config.max_concurrent_connections = 10; // Max 10 concurrent connections per IP

let hardening = SecurityHardening::new(config);
```

**Default Limits**:
- **60 requests/minute** per IP
- **1000 requests/hour** per IP
- **10 concurrent connections** per IP

**How It Works**:
- Tracks requests per IP using sliding time windows
- Automatically cleans up old request data
- Returns `RateLimitExceeded` error when limit is hit

---

### 2. **IP Whitelisting/Blacklisting**

**Purpose**: Allow trusted IPs to bypass rate limits or block malicious IPs.

**Whitelisting** (Bypass Rate Limits):
```rust
// Add trusted IP to whitelist
hardening.whitelist_ip("192.168.1.100".parse().unwrap()).await;

// Remove from whitelist
hardening.unwhitelist_ip("192.168.1.100".parse().unwrap()).await;
```

**Blacklisting** (Always Deny):
```rust
// Block malicious IP
hardening.blacklist_ip("10.0.0.1".parse().unwrap()).await;

// Unblock IP
hardening.unblacklist_ip("10.0.0.1".parse().unwrap()).await;
```

**Use Cases**:
- **Whitelist**: Trusted partners, internal services, monitoring tools
- **Blacklist**: Known attackers, compromised IPs, botnet IPs

---

### 3. **Auto-Banning System**

**Purpose**: Automatically ban IPs that repeatedly violate rate limits.

**Configuration**:
```rust
let mut config = SecurityConfig::default();
config.auto_ban_threshold = 10;        // Ban after 10 violations
config.ban_duration_seconds = 3600;     // Ban for 1 hour
```

**How It Works**:
1. Tracks violations per IP (rate limit hits, failed requests)
2. When threshold is reached, IP is automatically banned
3. Ban expires after configured duration
4. Violations reset after ban expires

**Attack Pattern Detection**:
- Monitors failed requests, rate limit hits, invalid requests
- If >50% of requests are failures, auto-blacklists IP
- Prevents sophisticated attacks that try to evade simple rate limits

---

### 4. **Request Size Limits**

**Purpose**: Prevent DoS via oversized requests.

**Configuration**:
```rust
let mut config = SecurityConfig::default();
config.max_request_size = 10 * 1024 * 1024; // 10MB max request size
```

**Enforcement**:
- Checks request body size before processing
- Returns `RequestTooLarge` error if exceeded
- Records invalid request for attack pattern detection

---

### 5. **Input Validation**

**Purpose**: Prevent injection attacks and malformed data.

**Block Size Limits**:
```rust
// Defined in blockchain/mod.rs
pub const MAX_BLOCK_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const MAX_PARENT_HASHES: usize = 10; // Max 10 parent hashes
```

**Transaction Limits**:
```rust
pub const MAX_TX_DATA_SIZE: usize = 128 * 1024; // 128KB transaction data
```

**Transaction Pool Limits**:
```rust
// Defined in mining.rs
pub const MAX_TX_POOL_SIZE: usize = 100_000; // 100k transactions max
```

**Network Message Limits**:
```rust
// Defined in network.rs
pub const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB
```

---

### 6. **API Key Authentication**

**Purpose**: Restrict access to sensitive RPC methods.

**Setup**:
```rust
use mondoshawan_blockchain::rpc::RpcServer;

let mut server = RpcServer::new(blockchain);
server.set_api_key("your-secret-api-key".to_string());
```

**Public Methods** (No Authentication Required):
- `eth_blockNumber`
- `net_version`
- `eth_chainId`
- `eth_syncing`
- `mds_getDagStats`
- `mds_getTps`

**Protected Methods** (Require API Key):
- All other methods require `X-API-Key` header or `api_key` parameter

**Usage**:
```bash
# With header
curl -H "X-API-Key: your-secret-api-key" \
  -X POST http://localhost:8545 \
  -d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0x..."],"id":1}'

# With parameter
curl -X POST http://localhost:8545 \
  -d '{"jsonrpc":"2.0","method":"eth_getBalance","params":["0x...","api_key":"your-secret-api-key"],"id":1}'
```

---

### 7. **Security Monitoring**

**Purpose**: Track and analyze security events.

**Get IP Statistics**:
```rust
let stats = hardening.get_ip_stats(ip).await;
if let Some(stats) = stats {
    println!("Requests/min: {}", stats.requests_per_minute);
    println!("Violations: {}", stats.violations);
    println!("Is banned: {}", stats.is_banned);
    println!("Attack score: {}", stats.attack_score);
}
```

**Attack Pattern Detection**:
- Tracks failed requests, rate limit hits, invalid requests per IP
- Calculates attack score based on suspicious activity
- Auto-blacklists IPs with high attack scores

---

## 🚀 Integration

### **Basic Setup**:

```rust
use mondoshawan_blockchain::security::{SecurityHardening, SecurityConfig};
use mondoshawan_blockchain::rpc::RpcServer;
use std::sync::Arc;
use tokio::sync::RwLock;

// Create security config
let config = SecurityConfig::default();

// Create security hardening
let hardening = Arc::new(RwLock::new(SecurityHardening::new(config)));

// Create RPC server with security hardening
let mut server = RpcServer::with_security_hardening(blockchain, config);

// Or add to existing server
server.set_security_hardening(hardening.clone());
```

### **In HTTP Handler**:

```rust
use std::net::IpAddr;

async fn handle_http_request(
    request: JsonRpcRequest,
    client_ip: IpAddr,
    rpc_server: &RpcServer,
) -> JsonRpcResponse {
    // Pass client IP to RPC server for security checks
    rpc_server.handle_request(request, None, Some(client_ip)).await
}
```

---

## 📊 Security Configuration Reference

### **SecurityConfig Defaults**:

```rust
SecurityConfig {
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
```

### **Recommended Production Settings**:

```rust
let mut config = SecurityConfig::default();
config.max_requests_per_minute = 120;  // Higher for production
config.max_requests_per_hour = 5000;   // Higher for production
config.max_concurrent_connections = 20; // Higher for production
config.enable_whitelist = true;         // Enable whitelisting
config.auto_ban_threshold = 5;          // Stricter (ban after 5 violations)
config.ban_duration_seconds = 7200;      // 2 hour ban
```

---

## 🔍 Monitoring & Alerts

### **Key Metrics to Monitor**:

1. **Rate Limit Hits**: Number of requests blocked by rate limiting
2. **Banned IPs**: Number of IPs currently banned
3. **Attack Scores**: IPs with high attack scores
4. **Request Sizes**: Monitor for unusually large requests
5. **Failed Requests**: Track authentication failures

### **Alert Thresholds**:

- **>100 rate limit hits/minute**: Possible DDoS attack
- **>10 IPs banned in 5 minutes**: Coordinated attack
- **Attack score >50**: Highly suspicious IP
- **Request size >8MB**: Possible DoS attempt

---

## 🛠️ Maintenance

### **Periodic Cleanup**:

```rust
// Cleanup old data (should be called periodically, e.g., every hour)
hardening.cleanup().await;
```

**What Gets Cleaned**:
- IP rate limit data older than 24 hours
- Attack pattern data older than 24 hours
- Expired bans are automatically reset

### **Manual IP Management**:

```rust
// Whitelist trusted IPs
hardening.whitelist_ip("192.168.1.100".parse().unwrap()).await;

// Blacklist known attackers
hardening.blacklist_ip("10.0.0.1".parse().unwrap()).await;

// Check IP status
let stats = hardening.get_ip_stats(ip).await;
```

---

## ⚠️ Security Best Practices

### **1. Always Enable Security Hardening in Production**

```rust
// ✅ Good: Enable security hardening
let server = RpcServer::with_security_hardening(blockchain, config);

// ❌ Bad: No security hardening
let server = RpcServer::new(blockchain);
```

### **2. Use Strong API Keys**

```rust
// ✅ Good: Generate strong random API key
let api_key = generate_secure_random_key(32);

// ❌ Bad: Weak or hardcoded API key
let api_key = "password123".to_string();
```

### **3. Whitelist Trusted IPs**

```rust
// ✅ Good: Whitelist internal services
hardening.whitelist_ip("192.168.1.100".parse().unwrap()).await;

// ❌ Bad: No whitelisting, all IPs rate limited
```

### **4. Monitor Attack Patterns**

```rust
// ✅ Good: Regular monitoring
let stats = hardening.get_ip_stats(suspicious_ip).await;
if let Some(stats) = stats {
    if stats.attack_score > 50.0 {
        // Alert security team
    }
}
```

### **5. Regular Cleanup**

```rust
// ✅ Good: Periodic cleanup
tokio::spawn(async move {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));
    loop {
        interval.tick().await;
        hardening.cleanup().await;
    }
});
```

---

## 🚨 Attack Scenarios & Mitigations

### **Scenario 1: DDoS Attack**

**Attack**: Thousands of requests from multiple IPs

**Mitigation**:
- Per-IP rate limiting (60 req/min per IP)
- Auto-banning after violations
- Attack pattern detection

**Response**:
```rust
// Monitor for high rate limit hits
if rate_limit_hits > 100 {
    // Increase ban duration
    config.ban_duration_seconds = 7200; // 2 hours
}
```

### **Scenario 2: Brute Force API Key Attack**

**Attack**: Repeated attempts to guess API key

**Mitigation**:
- Failed authentication tracked as attack pattern
- Auto-blacklist after threshold
- Rate limiting prevents rapid attempts

**Response**:
```rust
// Track failed authentications
hardening.record_failed_request(ip).await;

// Auto-blacklist after 10 failures
if violations >= 10 {
    hardening.blacklist_ip(ip).await;
}
```

### **Scenario 3: Resource Exhaustion**

**Attack**: Large requests or transaction spam

**Mitigation**:
- Request size limits (10MB)
- Transaction pool limits (100k)
- Block size limits (10MB)

**Response**:
```rust
// Reject oversized requests
if request_size > config.max_request_size {
    return Err(SecurityError::RequestTooLarge);
}
```

### **Scenario 4: Injection Attack**

**Attack**: Malformed JSON or malicious data

**Mitigation**:
- Input validation
- Transaction data size limits (128KB)
- Type checking

**Response**:
```rust
// Validate all inputs
validate_transaction(&tx)?;
validate_block(&block)?;
```

---

## 📝 Summary

The Mondoshawan blockchain includes comprehensive security hardening to protect against:

✅ **DoS/DDoS Protection**: Per-IP rate limiting, request size limits  
✅ **Brute Force Protection**: Auto-banning, violation tracking  
✅ **Resource Protection**: Transaction pool limits, block size limits  
✅ **Access Control**: API key authentication, IP whitelisting/blacklisting  
✅ **Attack Detection**: Pattern recognition, automatic response  
✅ **Monitoring**: IP statistics, attack scores, violation tracking  

**Production Ready**: ✅ Yes, with proper configuration

---

**Last Updated**: January 2026  
**Version**: 1.0
