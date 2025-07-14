use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use common::{CreateTaskRequest, Task, TaskFilter, UpdateTaskRequest};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::AppError, AppState};

pub async fn list_tasks(
    State(app_state): State<AppState>,
    Query(filter): Query<TaskFilterQuery>,
) -> Result<Json<Vec<Task>>, AppError> {
    let filter = TaskFilter {
        status: filter.status,
        category: filter.category,
        due_before: filter.due_before,
        due_after: filter.due_after,
    };

    let tasks = app_state.database.get_tasks(Some(filter)).await?;
    Ok(Json(tasks))
}

pub async fn create_task(
    State(app_state): State<AppState>,
    Json(request): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    if request.title.trim().is_empty() {
        return Err(AppError::InvalidInput("Title cannot be empty".to_string()));
    }

    let task = app_state.database.create_task(request).await?;
    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn update_task(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, AppError> {
    let task = app_state.database.update_task(id, request).await?;
    Ok(Json(task))
}

pub async fn delete_task(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    app_state.database.delete_task(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct TaskFilterQuery {
    pub status: Option<common::TaskStatus>,
    pub category: Option<common::TaskCategory>,
    pub due_before: Option<chrono::DateTime<chrono::Utc>>,
    pub due_after: Option<chrono::DateTime<chrono::Utc>>,
}
