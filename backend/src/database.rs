use chrono::Utc;
use common::{CreateTaskRequest, Task, TaskFilter, TaskStatus, UpdateTaskRequest};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::error::AppError;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<Task, AppError> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let status = TaskStatus::Todo;

        let row = sqlx::query(
            r#"
            INSERT INTO tasks (id, title, description, status, priority, due_date, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, title, description, status, priority, due_date, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&request.title)
        .bind(&request.description)
        .bind(status)
        .bind(request.priority)
        .bind(request.due_date)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            priority: row.get("priority"),
            due_date: row.get("due_date"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn get_tasks(&self, filter: Option<TaskFilter>) -> Result<Vec<Task>, AppError> {
        let rows = sqlx::query(
            r#"
            SELECT id, title, description, status, priority, due_date, created_at, updated_at
            FROM tasks
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let tasks: Vec<Task> = rows
            .into_iter()
            .map(|row| Task {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                status: row.get("status"),
                priority: row.get("priority"),
                due_date: row.get("due_date"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect();

        // Apply filters in Rust for now
        let filtered_tasks = if let Some(filter) = filter {
            tasks
                .into_iter()
                .filter(|task| {
                    let status_match = filter.status.is_none_or(|s| task.status == s);
                    let priority_match = filter.priority.is_none_or(|p| task.priority == p);
                    let due_before_match = filter
                        .due_before
                        .is_none_or(|d| task.due_date.is_some_and(|due| due < d));
                    let due_after_match = filter
                        .due_after
                        .is_none_or(|d| task.due_date.is_some_and(|due| due > d));

                    status_match && priority_match && due_before_match && due_after_match
                })
                .collect()
        } else {
            tasks
        };

        Ok(filtered_tasks)
    }

    pub async fn get_task_by_id(&self, id: Uuid) -> Result<Task, AppError> {
        let row = sqlx::query(
            r#"
            SELECT id, title, description, status, priority, due_date, created_at, updated_at
            FROM tasks WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Task {
                id: row.get("id"),
                title: row.get("title"),
                description: row.get("description"),
                status: row.get("status"),
                priority: row.get("priority"),
                due_date: row.get("due_date"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(AppError::TaskNotFound),
        }
    }

    pub async fn update_task(
        &self,
        id: Uuid,
        request: UpdateTaskRequest,
    ) -> Result<Task, AppError> {
        // First check if task exists
        let _existing_task = self.get_task_by_id(id).await?;
        let now = Utc::now();

        // Simple update approach using COALESCE
        let row = sqlx::query(
            r#"
            UPDATE tasks 
            SET title = COALESCE($2, title),
                description = COALESCE($3, description),
                status = COALESCE($4, status),
                priority = COALESCE($5, priority),
                due_date = COALESCE($6, due_date),
                updated_at = $7
            WHERE id = $1
            RETURNING id, title, description, status, priority, due_date, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(&request.title)
        .bind(&request.description)
        .bind(request.status)
        .bind(request.priority)
        .bind(request.due_date)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
            priority: row.get("priority"),
            due_date: row.get("due_date"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }

    pub async fn delete_task(&self, id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::TaskNotFound);
        }

        Ok(())
    }
}
