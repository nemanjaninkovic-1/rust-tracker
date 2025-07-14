#[cfg(test)]
mod tests {
    use common::{CreateTaskRequest, TaskPriority, TaskStatus};

    #[test]
    fn test_task_creation_request_validation() {
        let request = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            priority: common::TaskPriority::Low,
            due_date: None,
        };

        assert_eq!(request.title, "Test Task");
        assert_eq!(request.priority, TaskPriority::Low);
        assert!(request.description.is_some());
    }

    #[test]
    fn test_task_status_default() {
        let status = TaskStatus::default();
        assert_eq!(status, TaskStatus::Todo);
    }

    #[test]
    fn test_task_category_default() {
        let category = TaskPriority::default();
        assert_eq!(category, TaskPriority::Medium);
    }
}

// Include all test modules
mod benchmarks;
mod database_tests;
mod error_tests;
mod handler_tests;
mod integration_tests;
