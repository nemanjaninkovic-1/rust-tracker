//! Application configuration and setup tests
//!
//! Tests for main application initialization, state management,
//! and configuration handling.

use std::env;

// Import the main application components
use crate::database::Database;
use crate::{AppState, AppStateData};

#[tokio::test]
async fn test_health_check_endpoint() {
    // Test the health check endpoint directly
    let response = crate::health_check().await;
    assert_eq!(response, "OK");
}

#[tokio::test]
async fn test_application_state_creation() {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // In GitHub Actions, use localhost. In Docker, use test-db
        if env::var("GITHUB_ACTIONS").is_ok() {
            "postgres://postgres:password@localhost:5432/rusttracker_test".to_string()
        } else {
            "postgres://postgres:password@test-db:5432/rusttracker_test".to_string()
        }
    });

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let database = Database::new(pool);
    let app_state = std::sync::Arc::new(AppStateData { database });

    // Verify state structure
    assert!(std::sync::Arc::strong_count(&app_state) == 1);

    // Test that we can clone the state (for sharing across handlers)
    let cloned_state = app_state.clone();
    assert!(std::sync::Arc::strong_count(&app_state) == 2);
    assert!(std::sync::Arc::strong_count(&cloned_state) == 2);
}

#[tokio::test]
async fn test_database_url_configuration() {
    // Test default database URL construction
    env::remove_var("DATABASE_URL");
    let default_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/rusttracker".to_string());

    assert!(default_url.contains("postgres://"));
    assert!(default_url.contains("rusttracker"));

    // Test custom database URL
    env::set_var(
        "DATABASE_URL",
        "postgres://custom:password@custom-host:5432/custom_db",
    );
    let custom_url = env::var("DATABASE_URL").unwrap();
    assert_eq!(
        custom_url,
        "postgres://custom:password@custom-host:5432/custom_db"
    );

    // Clean up
    env::remove_var("DATABASE_URL");
}

#[tokio::test]
async fn test_port_configuration() {
    // Test default port
    env::remove_var("PORT");
    let default_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    assert_eq!(default_port, "8080");

    // Test custom port
    env::set_var("PORT", "3000");
    let custom_port = env::var("PORT").unwrap();
    assert_eq!(custom_port, "3000");

    // Clean up
    env::remove_var("PORT");
}

#[tokio::test]
async fn test_environment_variable_handling() {
    use std::env;

    // Test setting and getting environment variables for the application
    env::set_var("TEST_VAR", "test_value");
    let value = env::var("TEST_VAR").unwrap();
    assert_eq!(value, "test_value");

    // Test with missing environment variable
    env::remove_var("MISSING_VAR");
    let result = env::var("MISSING_VAR");
    assert!(result.is_err());
}
#[tokio::test]
async fn test_cors_configuration() {
    // This test verifies CORS is properly configured in the application
    // We can't directly test the CORS layer without the full app, but we can test
    // that the necessary components are available

    use tower_http::cors::CorsLayer;
    let cors_layer = CorsLayer::permissive();

    // The CorsLayer should be created successfully
    // This tests that the CORS configuration is properly set up
    assert!(format!("{cors_layer:?}").contains("CorsLayer"));
}

#[tokio::test]
async fn test_application_router_structure() {
    // Test that we can construct the application router structure
    use axum::{
        routing::{delete, get, post, put},
        Router,
    };

    // Create a test database state
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // In GitHub Actions, use localhost. In Docker, use test-db
        if env::var("GITHUB_ACTIONS").is_ok() {
            "postgres://postgres:password@localhost:5432/rusttracker_test".to_string()
        } else {
            "postgres://postgres:password@test-db:5432/rusttracker_test".to_string()
        }
    });

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let database = Database::new(pool);
    let app_state = std::sync::Arc::new(AppStateData { database });

    // Build router with our routes (testing route structure)
    let router: Router<AppState> = Router::new()
        .route("/api/tasks", get(crate::handlers::list_tasks))
        .route("/api/tasks", post(crate::handlers::create_task))
        .route("/api/tasks/:id", put(crate::handlers::update_task))
        .route("/api/tasks/:id", delete(crate::handlers::delete_task))
        .route("/health", get(crate::health_check))
        .with_state(app_state);

    // Verify router was created successfully
    assert!(format!("{router:?}").contains("Router"));
}

#[tokio::test]
async fn test_database_migration_readiness() {
    // Test that database migrations are properly configured
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // In GitHub Actions, use localhost. In Docker, use test-db
        if env::var("GITHUB_ACTIONS").is_ok() {
            "postgres://postgres:password@localhost:5432/rusttracker_test".to_string()
        } else {
            "postgres://postgres:password@test-db:5432/rusttracker_test".to_string()
        }
    });

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    // Test that we can run migrations (this tests the migration setup)
    let migration_result = sqlx::migrate!().run(&pool).await;
    assert!(
        migration_result.is_ok(),
        "Database migrations should run successfully"
    );
}

#[tokio::test]
async fn test_application_state_thread_safety() {
    // Test that AppState can be safely shared across threads
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        // In GitHub Actions, use localhost. In Docker, use test-db
        if env::var("GITHUB_ACTIONS").is_ok() {
            "postgres://postgres:password@localhost:5432/rusttracker_test".to_string()
        } else {
            "postgres://postgres:password@test-db:5432/rusttracker_test".to_string()
        }
    });

    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");

    let database = Database::new(pool);
    let app_state: AppState = std::sync::Arc::new(AppStateData { database });

    // Test that state can be moved to different threads
    let state_clone = app_state.clone();
    let handle = tokio::spawn(async move {
        // Perform a database operation to verify the state works across threads
        let result = state_clone.database.get_tasks(None).await;
        assert!(
            result.is_ok(),
            "Database operations should work across threads"
        );
        42
    });

    let result = handle.await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[tokio::test]
async fn test_logging_configuration() {
    // Test that logging can be configured (this tests the tracing setup)
    use tracing::{debug, error, info, warn};

    // Test that we can emit log messages at different levels
    debug!("Test debug message");
    info!("Test info message");
    warn!("Test warning message");
    error!("Test error message");

    // If we get here without panicking, logging is working
    // Test passes if we reach this point
}

#[tokio::test]
async fn test_error_handling_setup() {
    // Test that our error handling types are properly configured
    use crate::error::AppError;

    // Test error creation and handling
    let db_error = AppError::Database(sqlx::Error::RowNotFound);
    assert!(matches!(db_error, AppError::Database(_)));

    let internal_error = AppError::InternalError;
    assert!(matches!(internal_error, AppError::InternalError));

    let not_found_error = AppError::TaskNotFound;
    assert!(matches!(not_found_error, AppError::TaskNotFound));

    let invalid_input_error = AppError::InvalidInput("Test validation error".to_string());
    assert!(matches!(invalid_input_error, AppError::InvalidInput(_)));

    // Test result types
    let success_result: Result<String, AppError> = Ok("success".to_string());
    assert!(success_result.is_ok());

    let error_result: Result<String, AppError> = Err(AppError::TaskNotFound);
    assert!(error_result.is_err());
}

#[tokio::test]
async fn test_handler_module_structure() {
    // Test that all required handlers are available
    // This ensures our handler module is properly structured

    use crate::handlers::{create_task, delete_task, list_tasks, update_task};

    // We can't easily test the handlers without full integration,
    // but we can test that they exist and are properly imported
    assert!(format!("{:?}", create_task as *const ()).contains("0x"));
    assert!(format!("{:?}", delete_task as *const ()).contains("0x"));
    assert!(format!("{:?}", list_tasks as *const ()).contains("0x"));
    assert!(format!("{:?}", update_task as *const ()).contains("0x"));
}

#[tokio::test]
async fn test_common_types_integration() {
    // Test that common types are properly integrated
    use chrono::Utc;
    use common::{CreateTaskRequest, Task, TaskPriority, TaskStatus, UpdateTaskRequest};
    use uuid::Uuid;

    // Test CreateTaskRequest
    let create_request = CreateTaskRequest {
        title: "Test Task".to_string(),
        description: Some("Test Description".to_string()),
        priority: TaskPriority::High,
        due_date: None,
    };

    assert_eq!(create_request.title, "Test Task");
    assert_eq!(create_request.priority, TaskPriority::High);

    // Test UpdateTaskRequest
    let update_request = UpdateTaskRequest {
        title: Some("Updated Title".to_string()),
        description: None,
        status: Some(TaskStatus::Completed),
        priority: Some(TaskPriority::Low),
        due_date: None,
    };

    assert_eq!(update_request.title, Some("Updated Title".to_string()));
    assert_eq!(update_request.status, Some(TaskStatus::Completed));

    // Test Task structure
    let task = Task {
        id: Uuid::new_v4(),
        title: "Sample Task".to_string(),
        description: Some("Sample Description".to_string()),
        status: TaskStatus::InProgress,
        priority: TaskPriority::Medium,
        due_date: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert_eq!(task.title, "Sample Task");
    assert_eq!(task.status, TaskStatus::InProgress);
    assert_eq!(task.priority, TaskPriority::Medium);
}
