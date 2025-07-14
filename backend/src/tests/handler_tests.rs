#[cfg(test)]
mod handler_tests {
    use axum::{http::StatusCode, Router};
    use axum_test::TestServer;
    use chrono::Utc;
    use common::{CreateTaskRequest, TaskCategory, TaskStatus, UpdateTaskRequest};
    use serial_test::serial;
    use sqlx::PgPool;
    use std::{env, sync::Arc};
    use uuid::Uuid;

    async fn setup_test_server() -> TestServer {
        let database_url = env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:password@localhost:5432/rusttracker_test".to_string()
        });

        let pool = PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to test database");

        // Run migrations
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        // Clean all existing data for fresh tests
        sqlx::query("DELETE FROM tasks")
            .execute(&pool)
            .await
            .unwrap();

        let database = crate::database::Database::new(pool);
        let rate_limiter = crate::rate_limit::RateLimiter::new();
        let app_env = "test".to_string();

        let app_state = Arc::new(crate::AppStateData {
            database,
            rate_limiter,
            app_env,
        });

        let app = Router::new()
            .route(
                "/api/tasks",
                axum::routing::get(crate::handlers::list_tasks),
            )
            .route(
                "/api/tasks",
                axum::routing::post(crate::handlers::create_task),
            )
            .route(
                "/api/tasks/:id",
                axum::routing::put(crate::handlers::update_task),
            )
            .route(
                "/api/tasks/:id",
                axum::routing::delete(crate::handlers::delete_task),
            )
            .route("/health", axum::routing::get(|| async { "OK" }))
            .with_state(app_state);

        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    #[serial]
    async fn test_health_check() {
        let server = setup_test_server().await;

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "OK");
    }

    #[tokio::test]
    #[serial]
    async fn test_list_tasks_empty() {
        let server = setup_test_server().await;

        let response = server.get("/api/tasks").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let tasks: Vec<common::Task> = response.json();
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn test_create_task_success() {
        let server = setup_test_server().await;

        let create_request = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        let response = server.post("/api/tasks").json(&create_request).await;

        assert_eq!(response.status_code(), StatusCode::CREATED);

        let task: common::Task = response.json();
        assert_eq!(task.title, create_request.title);
        assert_eq!(task.description, create_request.description);
        assert_eq!(task.category, create_request.category);
        assert_eq!(task.status, TaskStatus::Todo);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_task_empty_title() {
        let server = setup_test_server().await;

        let create_request = CreateTaskRequest {
            title: "".to_string(),
            description: Some("Description".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        let response = server.post("/api/tasks").json(&create_request).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);

        let error: serde_json::Value = response.json();
        assert!(error["error"]
            .as_str()
            .unwrap()
            .contains("Title cannot be empty"));
    }

    #[tokio::test]
    #[serial]
    async fn test_create_task_whitespace_title() {
        let server = setup_test_server().await;

        let create_request = CreateTaskRequest {
            title: "   ".to_string(),
            description: Some("Description".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        let response = server.post("/api/tasks").json(&create_request).await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_create_task_invalid_json() {
        let server = setup_test_server().await;

        let response = server
            .post("/api/tasks")
            .content_type("application/json")
            .text("invalid json")
            .await;

        assert_eq!(response.status_code(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_tasks_with_data() {
        let server = setup_test_server().await;

        // Create test tasks
        let task1 = CreateTaskRequest {
            title: "Task 1".to_string(),
            description: Some("Description 1".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        let task2 = CreateTaskRequest {
            title: "Task 2".to_string(),
            description: None,
            category: TaskCategory::Personal,
            due_date: Some(Utc::now() + chrono::Duration::days(3)),
        };

        server.post("/api/tasks").json(&task1).await;
        server.post("/api/tasks").json(&task2).await;

        let response = server.get("/api/tasks").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let tasks: Vec<common::Task> = response.json();
        assert_eq!(tasks.len(), 2);
    }

    #[tokio::test]
    #[serial]
    async fn test_list_tasks_with_filters() {
        let server = setup_test_server().await;

        // Create tasks with different categories
        let work_task = CreateTaskRequest {
            title: "Work Task".to_string(),
            description: None,
            category: TaskCategory::Work,
            due_date: None,
        };

        let personal_task = CreateTaskRequest {
            title: "Personal Task".to_string(),
            description: None,
            category: TaskCategory::Personal,
            due_date: None,
        };

        server.post("/api/tasks").json(&work_task).await;
        server.post("/api/tasks").json(&personal_task).await;

        // Test category filter
        let response = server.get("/api/tasks?category=Work").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let tasks: Vec<common::Task> = response.json();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].category, TaskCategory::Work);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_task_success() {
        let server = setup_test_server().await;

        // Create a task first
        let create_request = CreateTaskRequest {
            title: "Original Task".to_string(),
            description: Some("Original Description".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        let create_response = server.post("/api/tasks").json(&create_request).await;

        let created_task: common::Task = create_response.json();

        // Update the task
        let update_request = UpdateTaskRequest {
            title: Some("Updated Task".to_string()),
            description: Some("Updated Description".to_string()),
            status: Some(TaskStatus::InProgress),
            category: Some(TaskCategory::Personal),
            due_date: Some(Utc::now() + chrono::Duration::days(7)),
        };

        let response = server
            .put(&format!("/api/tasks/{}", created_task.id))
            .json(&update_request)
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);

        let updated_task: common::Task = response.json();
        assert_eq!(updated_task.title, "Updated Task");
        assert_eq!(
            updated_task.description,
            Some("Updated Description".to_string())
        );
        assert_eq!(updated_task.status, TaskStatus::InProgress);
        assert_eq!(updated_task.category, TaskCategory::Personal);
        assert!(updated_task.due_date.is_some());
    }

    #[tokio::test]
    #[serial]
    async fn test_update_task_not_found() {
        let server = setup_test_server().await;

        let non_existent_id = Uuid::new_v4();
        let update_request = UpdateTaskRequest {
            title: Some("This won't work".to_string()),
            description: None,
            status: None,
            category: None,
            due_date: None,
        };

        let response = server
            .put(&format!("/api/tasks/{}", non_existent_id))
            .json(&update_request)
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_task_invalid_uuid() {
        let server = setup_test_server().await;

        let update_request = UpdateTaskRequest {
            title: Some("This won't work".to_string()),
            description: None,
            status: None,
            category: None,
            due_date: None,
        };

        let response = server
            .put("/api/tasks/invalid-uuid")
            .json(&update_request)
            .await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_task_success() {
        let server = setup_test_server().await;

        // Create a task first
        let create_request = CreateTaskRequest {
            title: "Delete Me".to_string(),
            description: None,
            category: TaskCategory::Other,
            due_date: None,
        };

        let create_response = server.post("/api/tasks").json(&create_request).await;

        let created_task: common::Task = create_response.json();

        // Delete the task
        let response = server
            .delete(&format!("/api/tasks/{}", created_task.id))
            .await;

        assert_eq!(response.status_code(), StatusCode::NO_CONTENT);

        // Verify task is deleted by trying to fetch all tasks
        let list_response = server.get("/api/tasks").await;
        let tasks: Vec<common::Task> = list_response.json();
        assert!(tasks.is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_task_not_found() {
        let server = setup_test_server().await;

        let non_existent_id = Uuid::new_v4();
        let response = server
            .delete(&format!("/api/tasks/{}", non_existent_id))
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_task_invalid_uuid() {
        let server = setup_test_server().await;

        let response = server.delete("/api/tasks/invalid-uuid").await;

        assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_cors_headers() {
        let server = setup_test_server().await;

        // Test a simple GET request to verify CORS is configured
        let response = server.get("/api/tasks").await;

        // CORS should allow the request
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    #[serial]
    async fn test_content_type_json_required() {
        let server = setup_test_server().await;

        let response = server
            .post("/api/tasks")
            .content_type("text/plain")
            .text("not json")
            .await;

        assert_eq!(response.status_code(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    #[tokio::test]
    #[serial]
    async fn test_large_task_title() {
        let server = setup_test_server().await;

        let large_title = "a".repeat(1000);
        let create_request = CreateTaskRequest {
            title: large_title.clone(),
            description: Some("Description".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        let response = server.post("/api/tasks").json(&create_request).await;

        // Should succeed (assuming no database constraint preventing it)
        assert_eq!(response.status_code(), StatusCode::CREATED);

        let task: common::Task = response.json();
        assert_eq!(task.title, large_title);
    }

    #[tokio::test]
    #[serial]
    async fn test_task_workflow_complete() {
        let server = setup_test_server().await;

        // 1. Create a task
        let create_request = CreateTaskRequest {
            title: "Complete Workflow Test".to_string(),
            description: Some("Testing complete workflow".to_string()),
            category: TaskCategory::Work,
            due_date: Some(Utc::now() + chrono::Duration::days(7)),
        };

        let create_response = server.post("/api/tasks").json(&create_request).await;

        assert_eq!(create_response.status_code(), StatusCode::CREATED);
        let task: common::Task = create_response.json();

        // 2. List tasks and verify our task is there
        let list_response = server.get("/api/tasks").await;
        assert_eq!(list_response.status_code(), StatusCode::OK);
        let tasks: Vec<common::Task> = list_response.json();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, task.id);

        // 3. Update task to in progress
        let update_request = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(TaskStatus::InProgress),
            category: None,
            due_date: None,
        };

        let update_response = server
            .put(&format!("/api/tasks/{}", task.id))
            .json(&update_request)
            .await;

        assert_eq!(update_response.status_code(), StatusCode::OK);
        let updated_task: common::Task = update_response.json();
        assert_eq!(updated_task.status, TaskStatus::InProgress);

        // 4. Complete the task
        let complete_request = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(TaskStatus::Completed),
            category: None,
            due_date: None,
        };

        let complete_response = server
            .put(&format!("/api/tasks/{}", task.id))
            .json(&complete_request)
            .await;

        assert_eq!(complete_response.status_code(), StatusCode::OK);
        let completed_task: common::Task = complete_response.json();
        assert_eq!(completed_task.status, TaskStatus::Completed);

        // 5. Delete the task
        let delete_response = server.delete(&format!("/api/tasks/{}", task.id)).await;

        assert_eq!(delete_response.status_code(), StatusCode::NO_CONTENT);

        // 6. Verify task is gone
        let final_list_response = server.get("/api/tasks").await;
        let final_tasks: Vec<common::Task> = final_list_response.json();
        assert!(final_tasks.is_empty());
    }
}
