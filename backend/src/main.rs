use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

mod auth;
mod database;
mod error;
mod handlers;
mod rate_limit;

#[cfg(test)]
mod tests;

use database::Database;
use rate_limit::RateLimiter;

// Extended AppState with more configuration from environment
#[allow(dead_code)]
pub struct AppStateData {
    database: Database,
    rate_limiter: RateLimiter,
    app_env: String,
}

pub type AppState = Arc<AppStateData>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/rusttracker".to_string());

    info!("Connecting to database: {}", database_url);

    // Create database connection pool
    let pool = PgPool::connect(&database_url).await?;

    // Run database migrations
    sqlx::migrate!().run(&pool).await?;

    // Create application state
    let database = Database::new(pool);
    let rate_limiter = RateLimiter::new();
    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".to_string());

    let app_state = Arc::new(AppStateData {
        database,
        rate_limiter,
        app_env,
    });

    // Build our application with routes
    let app = Router::new()
        .route("/api/tasks", get(handlers::list_tasks))
        .route("/api/tasks", post(handlers::create_task))
        .route("/api/tasks/:id", put(handlers::update_task))
        .route("/api/tasks/:id", delete(handlers::delete_task))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Run the server
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Server running on http://0.0.0.0:{}", port);

    axum::serve(listener, app).await?;

    Ok(())
}

// Health check endpoint handler
async fn health_check() -> &'static str {
    "OK"
}
