#[cfg(test)]
mod benchmarks {
    use chrono::Utc;
    use common::{CreateTaskRequest, TaskCategory, TaskFilter, TaskStatus, UpdateTaskRequest};
    use serial_test::serial;
    use sqlx::PgPool;
    use std::{env, time::Instant};

    async fn setup_benchmark_db() -> PgPool {
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

        // Clean all existing data for fresh benchmarks
        sqlx::query("DELETE FROM tasks")
            .execute(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    #[serial]
    async fn bench_create_single_task() {
        let pool = setup_benchmark_db().await;
        let database = crate::database::Database::new(pool);

        let request = CreateTaskRequest {
            title: "Benchmark Task".to_string(),
            description: Some("Testing performance".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        let start = Instant::now();
        let result = database.create_task(request).await;
        let duration = start.elapsed();

        assert!(result.is_ok());
        println!("Create single task took: {:?}", duration);

        // Should complete within reasonable time
        assert!(duration.as_millis() < 500);
    }

    #[tokio::test]
    #[serial]
    async fn bench_create_multiple_tasks() {
        let pool = setup_benchmark_db().await;
        let database = crate::database::Database::new(pool);

        let task_count = 10;
        let start = Instant::now();

        for i in 0..task_count {
            let request = CreateTaskRequest {
                title: format!("Benchmark Task {}", i),
                description: Some(format!("Testing performance {}", i)),
                category: TaskCategory::Work,
                due_date: Some(Utc::now() + chrono::Duration::days(i % 30)),
            };

            let result = database.create_task(request).await;
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let avg_per_task = duration.as_millis() / task_count as u128;

        println!("Create {} tasks took: {:?}", task_count, duration);
        println!("Average per task: {} ms", avg_per_task);

        // Should complete within reasonable time (less than 10ms per task on average)
        assert!(avg_per_task < 200);
    }

    #[tokio::test]
    #[serial]
    async fn bench_fetch_large_task_list() {
        let pool = setup_benchmark_db().await;
        let database = crate::database::Database::new(pool);

        // Create 50 tasks for testing
        let task_count = 50;
        for i in 0..task_count {
            let request = CreateTaskRequest {
                title: format!("Task {}", i),
                description: if i % 2 == 0 {
                    Some(format!("Description {}", i))
                } else {
                    None
                },
                category: match i % 5 {
                    0 => TaskCategory::Work,
                    1 => TaskCategory::Personal,
                    2 => TaskCategory::Shopping,
                    3 => TaskCategory::Health,
                    _ => TaskCategory::Other,
                },
                due_date: if i % 3 == 0 {
                    Some(Utc::now() + chrono::Duration::days(i % 30))
                } else {
                    None
                },
            };

            database.create_task(request).await.unwrap();
        }

        // Benchmark fetching all tasks
        let start = Instant::now();
        let result = database.get_tasks(None).await;
        let duration = start.elapsed();

        assert!(result.is_ok());
        let tasks = result.unwrap();
        assert_eq!(tasks.len(), task_count as usize);

        println!("Fetch {} tasks took: {:?}", task_count, duration);

        // Should complete within reasonable time
        assert!(duration.as_millis() < 1000);
    }

    #[tokio::test]
    #[serial]
    async fn bench_filtering_performance() {
        let pool = setup_benchmark_db().await;
        let database = crate::database::Database::new(pool);

        // Create diverse set of tasks
        let task_count = 50;
        for i in 0..task_count {
            let request = CreateTaskRequest {
                title: format!("Filter Test Task {}", i),
                description: Some(format!("Description {}", i)),
                category: match i % 5 {
                    0 => TaskCategory::Work,
                    1 => TaskCategory::Personal,
                    2 => TaskCategory::Shopping,
                    3 => TaskCategory::Health,
                    _ => TaskCategory::Other,
                },
                due_date: if i % 3 == 0 {
                    Some(Utc::now() + chrono::Duration::days((i % 30) as i64))
                } else {
                    None
                },
            };

            let task = database.create_task(request).await.unwrap();

            // Update some tasks to different statuses
            if i % 4 == 1 {
                database
                    .update_task(
                        task.id,
                        UpdateTaskRequest {
                            title: None,
                            description: None,
                            status: Some(TaskStatus::InProgress),
                            category: None,
                            due_date: None,
                        },
                    )
                    .await
                    .unwrap();
            } else if i % 4 == 2 {
                database
                    .update_task(
                        task.id,
                        UpdateTaskRequest {
                            title: None,
                            description: None,
                            status: Some(TaskStatus::Completed),
                            category: None,
                            due_date: None,
                        },
                    )
                    .await
                    .unwrap();
            }
        }

        // Benchmark different filtering scenarios
        let filters = vec![
            TaskFilter {
                status: Some(TaskStatus::Todo),
                category: None,
                due_before: None,
                due_after: None,
            },
            TaskFilter {
                status: None,
                category: Some(TaskCategory::Work),
                due_before: None,
                due_after: None,
            },
            TaskFilter {
                status: Some(TaskStatus::InProgress),
                category: Some(TaskCategory::Personal),
                due_before: None,
                due_after: None,
            },
            TaskFilter {
                status: None,
                category: None,
                due_before: Some(Utc::now() + chrono::Duration::days(15)),
                due_after: Some(Utc::now()),
            },
        ];

        for (i, filter) in filters.iter().enumerate() {
            let start = Instant::now();
            let result = database.get_tasks(Some(filter.clone())).await;
            let duration = start.elapsed();

            assert!(result.is_ok());
            let tasks = result.unwrap();

            println!(
                "Filter scenario {} returned {} tasks in {:?}",
                i + 1,
                tasks.len(),
                duration
            );

            // Should complete within reasonable time
            assert!(duration.as_millis() < 500);
        }
    }

    #[tokio::test]
    #[serial]
    async fn bench_update_operations() {
        let pool = setup_benchmark_db().await;
        let database = crate::database::Database::new(pool);

        // Create test tasks
        let task_count = 10;
        let mut task_ids = Vec::new();

        for i in 0..task_count {
            let request = CreateTaskRequest {
                title: format!("Update Test Task {}", i),
                description: Some("Original description".to_string()),
                category: TaskCategory::Work,
                due_date: None,
            };

            let task = database.create_task(request).await.unwrap();
            task_ids.push(task.id);
        }

        // Benchmark updates
        let start = Instant::now();

        for (i, task_id) in task_ids.iter().enumerate() {
            let update_request = UpdateTaskRequest {
                title: Some(format!("Updated Task {}", i)),
                description: Some("Updated description".to_string()),
                status: Some(TaskStatus::InProgress),
                category: Some(TaskCategory::Personal),
                due_date: Some(Utc::now() + chrono::Duration::days(7)),
            };

            let result = database.update_task(*task_id, update_request).await;
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let avg_per_update = duration.as_millis() / task_count as u128;

        println!("Update {} tasks took: {:?}", task_count, duration);
        println!("Average per update: {} ms", avg_per_update);

        // Should complete within reasonable time
        assert!(avg_per_update < 500);
    }

    #[tokio::test]
    #[serial]
    async fn bench_delete_operations() {
        let pool = setup_benchmark_db().await;
        let database = crate::database::Database::new(pool);

        // Create test tasks
        let task_count = 10;
        let mut task_ids = Vec::new();

        for i in 0..task_count {
            let request = CreateTaskRequest {
                title: format!("Delete Test Task {}", i),
                description: Some("To be deleted".to_string()),
                category: TaskCategory::Other,
                due_date: None,
            };

            let task = database.create_task(request).await.unwrap();
            task_ids.push(task.id);
        }

        // Benchmark deletions
        let start = Instant::now();

        for task_id in task_ids {
            let result = database.delete_task(task_id).await;
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let avg_per_delete = duration.as_millis() / task_count as u128;

        println!("Delete {} tasks took: {:?}", task_count, duration);
        println!("Average per delete: {} ms", avg_per_delete);

        // Should complete within reasonable time
        assert!(avg_per_delete < 200);
    }

    #[tokio::test]
    #[serial]
    async fn bench_concurrent_operations() {
        let pool = setup_benchmark_db().await;
        let database = std::sync::Arc::new(crate::database::Database::new(pool));

        let concurrent_operations = 5;
        let start = Instant::now();

        let mut handles = Vec::new();

        // Spawn concurrent create operations
        for i in 0..concurrent_operations {
            let db = database.clone();
            let handle = tokio::spawn(async move {
                let request = CreateTaskRequest {
                    title: format!("Concurrent Task {}", i),
                    description: Some(format!("Concurrent description {}", i)),
                    category: TaskCategory::Work,
                    due_date: None,
                };

                db.create_task(request).await
            });
            handles.push(handle);
        }

        // Wait for all operations to complete
        for handle in handles {
            let result = handle.await.unwrap();
            assert!(result.is_ok());
        }

        let duration = start.elapsed();
        let avg_per_operation = duration.as_millis() / concurrent_operations;

        println!(
            "Concurrent {} operations took: {:?}",
            concurrent_operations, duration
        );
        println!("Average per operation: {} ms", avg_per_operation);

        // Should handle concurrency well
        assert!(avg_per_operation < 1000);
    }

    #[tokio::test]
    #[serial]
    async fn bench_memory_usage() {
        let pool = setup_benchmark_db().await;
        let database = crate::database::Database::new(pool);

        // Create a large number of tasks to test memory usage
        let task_count = 50;

        println!("Creating {} tasks to test memory usage...", task_count);

        for i in 0..task_count {
            let request = CreateTaskRequest {
                title: format!("Memory Test Task {}", i),
                description: Some("A".repeat(100)), // Larger description to use more memory
                category: TaskCategory::Work,
                due_date: Some(Utc::now() + chrono::Duration::days(i % 365)),
            };

            let result = database.create_task(request).await;
            assert!(result.is_ok());
        }

        // Fetch all tasks multiple times to test memory usage patterns
        for iteration in 0..3 {
            let start = Instant::now();
            let result = database.get_tasks(None).await;
            let duration = start.elapsed();

            assert!(result.is_ok());
            let tasks = result.unwrap();
            assert_eq!(tasks.len(), task_count as usize);

            println!(
                "Iteration {}: Fetched {} tasks in {:?}",
                iteration + 1,
                tasks.len(),
                duration
            );

            // Memory usage should be consistent across iterations
            assert!(duration.as_millis() < 2000);
        }
    }
}
