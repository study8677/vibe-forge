use stevessr_core::error::{Error, Result};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// In-memory rate limiter. In production, this should use Redis for
/// distributed rate limiting across multiple instances.
pub struct RateLimiter {
    buckets: Mutex<HashMap<String, Vec<Instant>>>,
}

pub struct RateLimit {
    pub max_requests: usize,
    pub window: Duration,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            buckets: Mutex::new(HashMap::new()),
        }
    }

    /// Check if an action is rate-limited. Returns Ok(()) if allowed, Err if limited.
    pub fn check(&self, key: &str, limit: &RateLimit) -> Result<()> {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Instant::now();
        let cutoff = now - limit.window;

        let timestamps = buckets.entry(key.to_string()).or_insert_with(Vec::new);

        // Remove expired entries
        timestamps.retain(|t| *t > cutoff);

        if timestamps.len() >= limit.max_requests {
            return Err(Error::RateLimited {
                retry_after_secs: limit.window.as_secs(),
            });
        }

        timestamps.push(now);
        Ok(())
    }

    /// Rate limit for login attempts (per IP).
    pub fn check_login(&self, ip: &str) -> Result<()> {
        self.check(
            &format!("login:{}", ip),
            &RateLimit {
                max_requests: 10,
                window: Duration::from_secs(300), // 10 attempts per 5 minutes
            },
        )
    }

    /// Rate limit for creating topics (per user).
    pub fn check_create_topic(&self, user_id: i64) -> Result<()> {
        self.check(
            &format!("create_topic:{}", user_id),
            &RateLimit {
                max_requests: 5,
                window: Duration::from_secs(60), // 5 topics per minute
            },
        )
    }

    /// Rate limit for creating posts (per user).
    pub fn check_create_post(&self, user_id: i64) -> Result<()> {
        self.check(
            &format!("create_post:{}", user_id),
            &RateLimit {
                max_requests: 20,
                window: Duration::from_secs(60), // 20 posts per minute
            },
        )
    }

    /// Rate limit for likes (per user).
    pub fn check_like(&self, user_id: i64) -> Result<()> {
        self.check(
            &format!("like:{}", user_id),
            &RateLimit {
                max_requests: 50,
                window: Duration::from_secs(86400), // 50 likes per day
            },
        )
    }

    /// Rate limit for flag actions (per user).
    pub fn check_flag(&self, user_id: i64) -> Result<()> {
        self.check(
            &format!("flag:{}", user_id),
            &RateLimit {
                max_requests: 10,
                window: Duration::from_secs(86400), // 10 flags per day
            },
        )
    }

    /// Rate limit for password reset requests.
    pub fn check_password_reset(&self, ip: &str) -> Result<()> {
        self.check(
            &format!("password_reset:{}", ip),
            &RateLimit {
                max_requests: 3,
                window: Duration::from_secs(3600), // 3 per hour
            },
        )
    }

    /// Rate limit for email sending.
    pub fn check_email(&self, user_id: i64) -> Result<()> {
        self.check(
            &format!("email:{}", user_id),
            &RateLimit {
                max_requests: 20,
                window: Duration::from_secs(3600), // 20 emails per hour
            },
        )
    }

    /// Periodically clean up old entries to prevent memory growth.
    pub fn cleanup(&self) {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Instant::now();
        let max_age = Duration::from_secs(86400); // Remove entries older than 1 day

        buckets.retain(|_, timestamps| {
            timestamps.retain(|t| now.duration_since(*t) < max_age);
            !timestamps.is_empty()
        });
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}
