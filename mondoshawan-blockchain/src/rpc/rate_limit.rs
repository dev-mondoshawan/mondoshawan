//! Rate limiting for RPC API
//! 
//! Implements token bucket rate limiting to prevent abuse.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Rate limiter using token bucket algorithm
pub struct RateLimiter {
    /// Maximum number of tokens (requests)
    max_tokens: u32,
    /// Current number of tokens
    tokens: Arc<RwLock<u32>>,
    /// Token refill rate (tokens per second)
    refill_rate: f64,
    /// Last refill time
    last_refill: Arc<RwLock<Instant>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    /// 
    /// # Arguments
    /// * `max_tokens` - Maximum number of tokens (burst capacity)
    /// * `tokens_per_second` - Token refill rate
    pub fn new(max_tokens: u32, tokens_per_second: f64) -> Self {
        Self {
            max_tokens,
            tokens: Arc::new(RwLock::new(max_tokens)),
            refill_rate: tokens_per_second,
            last_refill: Arc::new(RwLock::new(Instant::now())),
        }
    }

    /// Try to acquire a token (allow a request)
    /// 
    /// Returns `true` if request is allowed, `false` if rate limited
    pub async fn try_acquire(&self) -> bool {
        let mut tokens = self.tokens.write().await;
        let mut last_refill = self.last_refill.write().await;
        
        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill);
        let tokens_to_add = (elapsed.as_secs_f64() * self.refill_rate) as u32;
        
        if tokens_to_add > 0 {
            *tokens = (*tokens + tokens_to_add).min(self.max_tokens);
            *last_refill = now;
        }
        
        // Try to consume a token
        if *tokens > 0 {
            *tokens -= 1;
            true
        } else {
            false
        }
    }

    /// Get current token count (for monitoring)
    pub async fn current_tokens(&self) -> u32 {
        *self.tokens.read().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(10, 1.0); // 10 tokens, 1 per second
        
        // Should allow initial requests
        for _ in 0..10 {
            assert!(limiter.try_acquire().await);
        }
        
        // Should rate limit after tokens exhausted
        assert!(!limiter.try_acquire().await);
        
        // Should refill after time passes
        sleep(Duration::from_secs(2)).await;
        assert!(limiter.try_acquire().await);
    }
}
