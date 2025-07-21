#[cfg(test)]
mod error_test_suite {
    use crate::error::AppError;
    use axum::{http::StatusCode, response::IntoResponse};
    use serde_json::Value;

    #[test]
    fn test_app_error_task_not_found() {
        let error = AppError::TaskNotFound;
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_app_error_invalid_input() {
        let error = AppError::InvalidInput("Test error message".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_app_error_internal_error() {
        let error = AppError::InternalError;
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_app_error_database_error() {
        // Create a mock SQLx error
        let db_error = sqlx::Error::RowNotFound;
        let error = AppError::Database(db_error);
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn test_error_response_format() {
        let error = AppError::InvalidInput("Test validation error".to_string());
        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Extract body and verify JSON format
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
        let json: Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(json["error"], "Test validation error");
    }

    #[test]
    fn test_error_display() {
        let error = AppError::TaskNotFound;
        assert_eq!(format!("{error}"), "Task not found");

        let error = AppError::InvalidInput("Custom message".to_string());
        assert_eq!(format!("{error}"), "Invalid input: Custom message");

        let error = AppError::InternalError;
        assert_eq!(format!("{error}"), "Internal server error");
    }

    #[test]
    fn test_error_debug() {
        let error = AppError::TaskNotFound;
        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("TaskNotFound"));
    }

    #[test]
    fn test_error_from_sqlx() {
        let sqlx_error = sqlx::Error::RowNotFound;
        let app_error: AppError = sqlx_error.into();

        match app_error {
            AppError::Database(_) => {} // Expected
            _ => panic!("Expected Database error variant"),
        }
    }
}
