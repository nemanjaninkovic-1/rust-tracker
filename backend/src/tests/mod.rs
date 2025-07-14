#[cfg(test)]
mod tests {
    use common::{CreateTaskRequest, TaskCategory, TaskStatus};

    #[test]
    fn test_task_creation_request_validation() {
        let request = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            category: TaskCategory::Work,
            due_date: None,
        };

        assert_eq!(request.title, "Test Task");
        assert_eq!(request.category, TaskCategory::Work);
        assert!(request.description.is_some());
    }

    #[test]
    fn test_task_status_default() {
        let status = TaskStatus::default();
        assert_eq!(status, TaskStatus::Todo);
    }

    #[test]
    fn test_task_category_default() {
        let category = TaskCategory::default();
        assert_eq!(category, TaskCategory::Other);
    }
}

// Include all test modules
mod benchmarks;
mod database_tests;
mod error_tests;
mod handler_tests;
mod integration_tests;
