use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{info, warn};

use crate::AppState;

// Allow dead code for rate limiter implementation that will be used later
#[allow(dead_code)]
// Define the rate limiter struct
#[derive(Debug, Clone)]
pub struct RateLimiter {
    // Map client IPs to their last request times and counts
    clients: Arc<Mutex<HashMap<String, (Instant, u32)>>>,
    // Maximum number of requests allowed per time window
    max_requests: u32,
    // Time window for rate limiting in milliseconds
    window_ms: u64,
}

impl RateLimiter {
    pub fn new() -> Self {
        // Get rate limit configuration from environment variables
        let max_requests = std::env::var("MAX_REQUESTS_PER_MINUTE")
            .unwrap_or_else(|_| "100".to_string())
            .parse::<u32>()
            .unwrap_or(100);

        let window_ms = std::env::var("RATE_LIMIT_WINDOW_MS")
            .unwrap_or_else(|_| "60000".to_string())
            .parse::<u64>()
            .unwrap_or(60000);

        info!(
            "Rate limiter initialized: {} requests per {} ms",
            max_requests, window_ms
        );

        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window_ms,
        }
    }

    // Check if a client is allowed to make a request
    #[allow(dead_code)]
    pub async fn is_allowed(&self, client_id: &str) -> bool {
        let mut clients = self.clients.lock().await;
        let now = Instant::now();
        let window = Duration::from_millis(self.window_ms);

        if let Some((last_request_time, count)) = clients.get_mut(client_id) {
            // If the time window has passed, reset the counter
            if now.duration_since(*last_request_time) > window {
                *last_request_time = now;
                *count = 1;
                true
            } else {
                // If we're still in the time window, increment the counter
                if *count >= self.max_requests {
                    warn!("Rate limit exceeded for client {}", client_id);
                    false
                } else {
                    *count += 1;
                    true
                }
            }
        } else {
            // First request from this client
            clients.insert(client_id.to_string(), (now, 1));
            true
        }
    }
}

// Middleware for rate limiting
#[allow(dead_code)]
pub async fn rate_limit_middleware(
    State(app_state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get the client IP address
    let client_id = addr.ip().to_string();

    // Check if the client is allowed to make a request
    if app_state.rate_limiter.is_allowed(&client_id).await {
        Ok(next.run(request).await)
    } else {
        // Return 429 Too Many Requests if rate limit is exceeded
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}

// Simplified rate limit function - for now just a placeholder
// TODO: Implement proper middleware when needed
#[allow(dead_code)]
pub fn rate_limit() {
    // Rate limiting is disabled for now
    // The RateLimiter struct is available in AppState for future use
}
