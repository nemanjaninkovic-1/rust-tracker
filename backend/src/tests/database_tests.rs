#[cfg(test)]
mod database_tests {
    use crate::{database::Database, error::AppError};
    use chrono::Utc;
    use common::{CreateTaskRequest, TaskFilter, TaskPriority, TaskStatus, UpdateTaskRequest};
    use serial_test::serial;
    use sqlx::PgPool;
    use std::env;
    use uuid::Uuid;

    async fn setup_test_db() -> PgPool {
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

        pool
    }

    #[tokio::test]
    #[serial]
    async fn test_create_task_success() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let request = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            priority: common::TaskPriority::Low,
            due_date: None,
        };

        let result = database.create_task(request.clone()).await;
        assert!(result.is_ok());

        let task = result.unwrap();
        assert_eq!(task.title, request.title);
        assert_eq!(task.description, request.description);
        assert_eq!(task.priority, request.priority);
        assert_eq!(task.status, TaskStatus::Todo);
        assert!(task.id != Uuid::nil());
        assert!(task.created_at <= Utc::now());
        assert!(task.updated_at <= Utc::now());
    }

    #[tokio::test]
    #[serial]
    async fn test_create_task_with_due_date() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let due_date = Utc::now() + chrono::Duration::days(7);
        let request = CreateTaskRequest {
            title: "Task with due date".to_string(),
            description: None,
            priority: TaskPriority::Low,
            due_date: Some(due_date),
        };

        let result = database.create_task(request.clone()).await;
        assert!(result.is_ok());

        let task = result.unwrap();
        // Compare just the date parts since PostgreSQL may have different precision
        assert_eq!(
            task.due_date.unwrap().date_naive(),
            request.due_date.unwrap().date_naive()
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_get_tasks_empty() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let result = database.get_tasks(None).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    #[serial]
    async fn test_get_tasks_with_data() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        // Create test tasks
        let task1 = CreateTaskRequest {
            title: "Task 1".to_string(),
            description: Some("Description 1".to_string()),
            priority: TaskPriority::High,
            due_date: None,
        };

        let task2 = CreateTaskRequest {
            title: "Task 2".to_string(),
            description: None,
            priority: TaskPriority::Low,
            due_date: Some(Utc::now() + chrono::Duration::days(3)),
        };

        database.create_task(task1).await.unwrap();
        database.create_task(task2).await.unwrap();

        let result = database.get_tasks(None).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 2);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_tasks_with_status_filter() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        // Create tasks with different statuses
        let task = database
            .create_task(CreateTaskRequest {
                title: "Todo Task".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: None,
            })
            .await
            .unwrap();

        // Update one task to InProgress
        database
            .update_task(
                task.id,
                UpdateTaskRequest {
                    title: None,
                    description: None,
                    status: Some(TaskStatus::InProgress),
                    priority: None,
                    due_date: None,
                },
            )
            .await
            .unwrap();

        // Create another todo task
        database
            .create_task(CreateTaskRequest {
                title: "Another Todo".to_string(),
                description: None,
                priority: TaskPriority::Low,
                due_date: None,
            })
            .await
            .unwrap();

        // Test filter for Todo status
        let filter = TaskFilter {
            status: Some(TaskStatus::Todo),
            priority: None,
            due_before: None,
            due_after: None,
        };

        let result = database.get_tasks(Some(filter)).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].status, TaskStatus::Todo);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_tasks_with_priority_filter() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        database
            .create_task(CreateTaskRequest {
                title: "Work Task".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: None,
            })
            .await
            .unwrap();

        database
            .create_task(CreateTaskRequest {
                title: "Personal Task".to_string(),
                description: None,
                priority: TaskPriority::Low,
                due_date: None,
            })
            .await
            .unwrap();

        let filter = TaskFilter {
            status: None,
            priority: Some(TaskPriority::High),
            due_before: None,
            due_after: None,
        };

        let result = database.get_tasks(Some(filter)).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].priority, TaskPriority::High);
    }

    #[tokio::test]
    #[serial]
    async fn test_get_task_by_id_success() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let created_task = database
            .create_task(CreateTaskRequest {
                title: "Find Me".to_string(),
                description: Some("Found!".to_string()),
                priority: TaskPriority::Medium,
                due_date: None,
            })
            .await
            .unwrap();

        let result = database.get_task_by_id(created_task.id).await;
        assert!(result.is_ok());

        let found_task = result.unwrap();
        assert_eq!(found_task.id, created_task.id);
        assert_eq!(found_task.title, "Find Me");
    }

    #[tokio::test]
    #[serial]
    async fn test_get_task_by_id_not_found() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let non_existent_id = Uuid::new_v4();
        let result = database.get_task_by_id(non_existent_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::TaskNotFound));
    }

    #[tokio::test]
    #[serial]
    async fn test_update_task_success() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let created_task = database
            .create_task(CreateTaskRequest {
                title: "Original Title".to_string(),
                description: Some("Original Description".to_string()),
                priority: TaskPriority::High,
                due_date: None,
            })
            .await
            .unwrap();

        let update_request = UpdateTaskRequest {
            title: Some("Updated Title".to_string()),
            description: Some("Updated Description".to_string()),
            status: Some(TaskStatus::InProgress),
            priority: Some(TaskPriority::Low),
            due_date: Some(Utc::now() + chrono::Duration::days(5)),
        };

        let result = database
            .update_task(created_task.id, update_request.clone())
            .await;
        assert!(result.is_ok());

        let updated_task = result.unwrap();
        assert_eq!(updated_task.title, "Updated Title");
        assert_eq!(
            updated_task.description,
            Some("Updated Description".to_string())
        );
        assert_eq!(updated_task.status, TaskStatus::InProgress);
        assert_eq!(updated_task.priority, TaskPriority::Low);
        assert!(updated_task.due_date.is_some());
        assert!(updated_task.updated_at > created_task.updated_at);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_task_partial() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let created_task = database
            .create_task(CreateTaskRequest {
                title: "Original Title".to_string(),
                description: Some("Original Description".to_string()),
                priority: TaskPriority::High,
                due_date: None,
            })
            .await
            .unwrap();

        // Only update title
        let update_request = UpdateTaskRequest {
            title: Some("Only Title Updated".to_string()),
            description: None,
            status: None,
            priority: None,
            due_date: None,
        };

        let result = database.update_task(created_task.id, update_request).await;
        assert!(result.is_ok());

        let updated_task = result.unwrap();
        assert_eq!(updated_task.title, "Only Title Updated");
        assert_eq!(updated_task.description, created_task.description);
        assert_eq!(updated_task.status, created_task.status);
        assert_eq!(updated_task.priority, created_task.priority);
        assert_eq!(updated_task.due_date, created_task.due_date);
    }

    #[tokio::test]
    #[serial]
    async fn test_update_task_not_found() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let non_existent_id = Uuid::new_v4();
        let update_request = UpdateTaskRequest {
            title: Some("This won't work".to_string()),
            description: None,
            status: None,
            priority: None,
            due_date: None,
        };

        let result = database.update_task(non_existent_id, update_request).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::TaskNotFound));
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_task_success() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let created_task = database
            .create_task(CreateTaskRequest {
                title: "Delete Me".to_string(),
                description: None,
                priority: TaskPriority::Medium,
                due_date: None,
            })
            .await
            .unwrap();

        let result = database.delete_task(created_task.id).await;
        assert!(result.is_ok());

        // Verify task is deleted
        let get_result = database.get_task_by_id(created_task.id).await;
        assert!(get_result.is_err());
        assert!(matches!(get_result.unwrap_err(), AppError::TaskNotFound));
    }

    #[tokio::test]
    #[serial]
    async fn test_delete_task_not_found() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let non_existent_id = Uuid::new_v4();
        let result = database.delete_task(non_existent_id).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::TaskNotFound));
    }

    #[tokio::test]
    #[serial]
    async fn test_get_tasks_with_due_date_filters() {
        let pool = setup_test_db().await;
        let database = Database::new(pool);

        let now = Utc::now();
        let yesterday = now - chrono::Duration::days(1);
        let tomorrow = now + chrono::Duration::days(1);
        let next_week = now + chrono::Duration::days(7);

        // Create tasks with different due dates
        database
            .create_task(CreateTaskRequest {
                title: "Past Task".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: Some(yesterday),
            })
            .await
            .unwrap();

        database
            .create_task(CreateTaskRequest {
                title: "Future Task".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: Some(tomorrow),
            })
            .await
            .unwrap();

        database
            .create_task(CreateTaskRequest {
                title: "Far Future Task".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: Some(next_week),
            })
            .await
            .unwrap();

        database
            .create_task(CreateTaskRequest {
                title: "No Due Date".to_string(),
                description: None,
                priority: TaskPriority::High,
                due_date: None,
            })
            .await
            .unwrap();

        // Test due_before filter
        let filter = TaskFilter {
            status: None,
            priority: None,
            due_before: Some(now + chrono::Duration::days(2)),
            due_after: None,
        };

        let result = database.get_tasks(Some(filter)).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 2); // yesterday and tomorrow tasks

        // Test due_after filter
        let filter = TaskFilter {
            status: None,
            priority: None,
            due_before: None,
            due_after: Some(now),
        };

        let result = database.get_tasks(Some(filter)).await;
        assert!(result.is_ok());

        let tasks = result.unwrap();
        assert_eq!(tasks.len(), 2); // tomorrow and next_week tasks
    }
}
