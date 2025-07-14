#[cfg(test)]
mod integration_tests {
    use axum_test::TestServer;
    use chrono::Utc;
    use common::{CreateTaskRequest, TaskPriority, TaskStatus, UpdateTaskRequest};
    use serial_test::serial;
    use sqlx::PgPool;
    use std::{env, sync::Arc};
    use uuid::Uuid;

    async fn setup_integration_server() -> TestServer {
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

        let app = axum::Router::new()
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
            .layer(tower_http::cors::CorsLayer::permissive())
            .with_state(app_state);

        TestServer::new(app).unwrap()
    }

    #[tokio::test]
    #[serial]
    async fn test_full_task_lifecycle() {
        let server = setup_integration_server().await;

        // Step 1: Verify empty state
        let response = server.get("/api/tasks").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let tasks: Vec<common::Task> = response.json();
        assert!(tasks.is_empty());

        // Step 2: Create multiple tasks
        let tasks_to_create = vec![
            CreateTaskRequest {
                title: "Work Task 1".to_string(),
                description: Some("Low important task".to_string()),
                priority: TaskPriority::Low,
                due_date: Some(Utc::now() + chrono::Duration::days(7)),
            },
            CreateTaskRequest {
                title: "Personal Task 1".to_string(),
                description: None,
                priority: TaskPriority::Low,
                due_date: Some(Utc::now() + chrono::Duration::days(3)),
            },
            CreateTaskRequest {
                title: "Shopping List".to_string(),
                description: Some("Buy groceries".to_string()),
                priority: TaskPriority::Medium,
                due_date: Some(Utc::now() + chrono::Duration::days(1)),
            },
        ];

        let mut created_tasks = Vec::new();
        for task_request in tasks_to_create {
            let response = server.post("/api/tasks").json(&task_request).await;
            assert_eq!(response.status_code(), axum::http::StatusCode::CREATED);
            let task: common::Task = response.json();
            created_tasks.push(task);
        }

        // Step 3: Verify all tasks exist
        let response = server.get("/api/tasks").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let tasks: Vec<common::Task> = response.json();
        assert_eq!(tasks.len(), 3);

        // Step 4: Test filtering by category
        let response = server.get("/api/tasks?category=Work").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let work_tasks: Vec<common::Task> = response.json();
        assert_eq!(work_tasks.len(), 1);
        assert_eq!(work_tasks[0].priority, TaskPriority::High);

        // Step 5: Update task status workflow
        let work_task = &created_tasks[0];

        // Move to InProgress
        let update_to_progress = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(TaskStatus::InProgress),
            priority: None,
            due_date: None,
        };

        let response = server
            .put(&format!("/api/tasks/{}", work_task.id))
            .json(&update_to_progress)
            .await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let updated_task: common::Task = response.json();
        assert_eq!(updated_task.status, TaskStatus::InProgress);

        // Complete the task
        let update_to_complete = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(TaskStatus::Completed),
            priority: None,
            due_date: None,
        };

        let response = server
            .put(&format!("/api/tasks/{}", work_task.id))
            .json(&update_to_complete)
            .await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let completed_task: common::Task = response.json();
        assert_eq!(completed_task.status, TaskStatus::Completed);

        // Step 6: Test filtering by status
        let response = server.get("/api/tasks?status=Completed").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let completed_tasks: Vec<common::Task> = response.json();
        assert_eq!(completed_tasks.len(), 1);
        assert_eq!(completed_tasks[0].status, TaskStatus::Completed);

        // Step 7: Delete completed task
        let response = server.delete(&format!("/api/tasks/{}", work_task.id)).await;
        assert_eq!(response.status_code(), axum::http::StatusCode::NO_CONTENT);

        // Step 8: Verify task is deleted
        let response = server.get("/api/tasks").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let remaining_tasks: Vec<common::Task> = response.json();
        assert_eq!(remaining_tasks.len(), 2);

        // Step 9: Clean up remaining tasks
        for task in &created_tasks[1..] {
            let response = server.delete(&format!("/api/tasks/{}", task.id)).await;
            assert_eq!(response.status_code(), axum::http::StatusCode::NO_CONTENT);
        }

        // Step 10: Verify all tasks are gone
        let response = server.get("/api/tasks").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let final_tasks: Vec<common::Task> = response.json();
        assert!(final_tasks.is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn test_sequential_task_operations() {
        let server = setup_integration_server().await;

        // Create multiple tasks sequentially
        let tasks = vec![
            CreateTaskRequest {
                title: "Sequential Task 1".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: None,
            },
            CreateTaskRequest {
                title: "Sequential Task 2".to_string(),
                description: None,
                priority: TaskPriority::Low,
                due_date: None,
            },
            CreateTaskRequest {
                title: "Sequential Task 3".to_string(),
                description: None,
                priority: TaskPriority::Medium,
                due_date: None,
            },
        ];

        let mut created_tasks = Vec::new();
        for task in tasks {
            let response = server.post("/api/tasks").json(&task).await;
            assert_eq!(response.status_code(), axum::http::StatusCode::CREATED);
            let created_task: common::Task = response.json();
            created_tasks.push(created_task);
        }

        // Verify all tasks were created
        let response = server.get("/api/tasks").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let tasks: Vec<common::Task> = response.json();
        assert_eq!(tasks.len(), 3);

        // Update tasks sequentially
        for task in &created_tasks {
            let update_request = UpdateTaskRequest {
                title: None,
                description: Some("Updated sequentially".to_string()),
                status: Some(TaskStatus::InProgress),
                priority: None,
                due_date: None,
            };

            let response = server
                .put(&format!("/api/tasks/{}", task.id))
                .json(&update_request)
                .await;
            assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        }

        // Verify all tasks were updated
        let response = server.get("/api/tasks?status=InProgress").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let updated_tasks: Vec<common::Task> = response.json();
        assert_eq!(updated_tasks.len(), 3);
    }

    #[tokio::test]
    #[serial]
    async fn test_error_scenarios() {
        let server = setup_integration_server().await;

        // Test 404 scenarios
        let non_existent_id = Uuid::new_v4();

        // Update non-existent task
        let update_request = UpdateTaskRequest {
            title: Some("This won't work".to_string()),
            description: None,
            status: None,
            priority: None,
            due_date: None,
        };

        let response = server
            .put(&format!("/api/tasks/{}", non_existent_id))
            .json(&update_request)
            .await;
        assert_eq!(response.status_code(), axum::http::StatusCode::NOT_FOUND);

        // Delete non-existent task
        let response = server
            .delete(&format!("/api/tasks/{}", non_existent_id))
            .await;
        assert_eq!(response.status_code(), axum::http::StatusCode::NOT_FOUND);

        // Test validation errors
        let invalid_task = CreateTaskRequest {
            title: "".to_string(), // Empty title should fail
            description: None,
            priority: TaskPriority::High,
            due_date: None,
        };

        let response = server.post("/api/tasks").json(&invalid_task).await;
        assert_eq!(response.status_code(), axum::http::StatusCode::BAD_REQUEST);

        // Test invalid UUID in URL
        let response = server
            .put("/api/tasks/not-a-uuid")
            .json(&update_request)
            .await;
        assert_eq!(response.status_code(), axum::http::StatusCode::BAD_REQUEST);

        let response = server.delete("/api/tasks/not-a-uuid").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    #[serial]
    async fn test_date_filtering() {
        let server = setup_integration_server().await;

        let now = Utc::now();
        let yesterday = now - chrono::Duration::days(1);
        let tomorrow = now + chrono::Duration::days(1);
        let next_week = now + chrono::Duration::days(7);

        // Create tasks with different due dates
        let tasks = vec![
            CreateTaskRequest {
                title: "Overdue Task".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: Some(yesterday),
            },
            CreateTaskRequest {
                title: "Due Tomorrow".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: Some(tomorrow),
            },
            CreateTaskRequest {
                title: "Due Next Week".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: Some(next_week),
            },
            CreateTaskRequest {
                title: "No Due Date".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: None,
            },
        ];

        for task in tasks {
            let response = server.post("/api/tasks").json(&task).await;
            assert_eq!(response.status_code(), axum::http::StatusCode::CREATED);
        }

        // Test due_before filter (should get overdue and tomorrow tasks)
        let due_before = (now + chrono::Duration::days(2)).to_rfc3339();
        let response = server
            .get(&format!(
                "/api/tasks?due_before={}",
                due_before.replace("+", "%2B")
            ))
            .await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let tasks: Vec<common::Task> = response.json();
        assert_eq!(tasks.len(), 2);

        // Test due_after filter (should get tomorrow and next week tasks)
        let due_after = now.to_rfc3339();
        let response = server
            .get(&format!(
                "/api/tasks?due_after={}",
                due_after.replace("+", "%2B")
            ))
            .await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        let tasks: Vec<common::Task> = response.json();
        assert_eq!(tasks.len(), 2);
    }

    #[tokio::test]
    #[serial]
    async fn test_health_endpoint() {
        let server = setup_integration_server().await;

        let response = server.get("/health").await;
        assert_eq!(response.status_code(), axum::http::StatusCode::OK);
        assert_eq!(response.text(), "OK");
    }
}
