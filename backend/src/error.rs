use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Task not found")]
    TaskNotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error")]
    #[allow(dead_code)]
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);

                // Check if this is a constraint violation (user input error)
                if let Some(db_err) = err.as_database_error() {
                    if let Some(constraint) = db_err.constraint() {
                        // Handle specific constraint violations
                        if constraint.contains("check") || constraint.contains("length") {
                            return (
                                StatusCode::BAD_REQUEST,
                                Json(json!({
                                    "error": "Invalid input: constraint violation"
                                })),
                            )
                                .into_response();
                        }
                    }

                    // Check for value too long errors
                    if db_err.message().contains("value too long") {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(json!({
                                "error": "Input value exceeds maximum length"
                            })),
                        )
                            .into_response();
                    }
                }

                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found"),
            AppError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
