use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::debug;

/// A rate limiter using the token bucket algorithm
#[derive(Debug, Clone)]
pub struct RateLimiter {
    /// Maximum number of tokens in the bucket
    capacity: u32,
    /// Current number of tokens in the bucket
    tokens: u32,
    /// Rate at which tokens are added to the bucket (tokens per second)
    rate: f64,
    /// Last time the bucket was updated
    last_update: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(capacity: u32, rate: f64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            rate,
            last_update: Instant::now(),
        }
    }

    /// Wait until a token is available
    pub async fn acquire(&mut self) {
        self.update_tokens();
        if self.tokens == 0 {
            let wait_time = self.calculate_wait_time();
            debug!("Rate limit reached, waiting for {:?}", wait_time);
            sleep(wait_time).await;
            self.update_tokens();
        }
        self.tokens -= 1;
    }

    /// Update the number of tokens in the bucket
    fn update_tokens(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        let new_tokens = (elapsed * self.rate) as u32;
        
        if new_tokens > 0 {
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_update = now;
        }
    }

    /// Calculate how long to wait for the next token
    fn calculate_wait_time(&self) -> Duration {
        let tokens_needed = 1.0;
        let wait_seconds = tokens_needed / self.rate;
        Duration::from_secs_f64(wait_seconds)
    }

    /// Get the current number of tokens in the bucket
    pub fn tokens(&self) -> u32 {
        self.tokens
    }

    /// Get the capacity of the bucket
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Get the rate at which tokens are added to the bucket
    pub fn rate(&self) -> f64 {
        self.rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_rate_limiter() {
        let mut limiter = RateLimiter::new(2, 1.0); // 2 tokens, 1 token per second

        // First two requests should be immediate
        limiter.acquire().await;
        assert_eq!(limiter.tokens(), 1);
        limiter.acquire().await;
        assert_eq!(limiter.tokens(), 0);

        // Third request should wait
        let start = Instant::now();
        limiter.acquire().await;
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_secs(1));
        assert_eq!(limiter.tokens(), 0);
    }

    #[tokio::test]
    async fn test_rate_limiter_burst() {
        let mut limiter = RateLimiter::new(5, 1.0); // 5 tokens, 1 token per second

        // Use all tokens immediately
        for _ in 0..5 {
            limiter.acquire().await;
        }
        assert_eq!(limiter.tokens(), 0);

        // Wait for tokens to replenish
        sleep(Duration::from_secs(3)).await;
        limiter.update_tokens();
        assert_eq!(limiter.tokens(), 3);
    }
} 