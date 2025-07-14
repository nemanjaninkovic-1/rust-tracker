#[cfg(test)]
mod api_tests {
    use chrono::Utc;
    use common::{CreateTaskRequest, TaskFilter, TaskPriority, TaskStatus, UpdateTaskRequest};
    use uuid::Uuid;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    const API_BASE: &str = "http://localhost:8080/api";

    #[test]
    fn test_api_base_url() {
        assert_eq!(API_BASE, "http://localhost:8080/api");
    }

    #[test]
    fn test_task_filter_url_generation() {
        // Test with no filter
        let url = format!("{}/tasks", API_BASE);
        assert_eq!(url, "http://localhost:8080/api/tasks");
    }

    #[test]
    fn test_task_filter_status_url() {
        let filter = TaskFilter {
            status: Some(TaskStatus::InProgress),
            priority: None,
            due_before: None,
            due_after: None,
        };

        let mut url = format!("{}/tasks", API_BASE);
        let mut params = Vec::new();

        if let Some(status) = filter.status {
            params.push(format!("status={:?}", status));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        assert_eq!(url, "http://localhost:8080/api/tasks?status=InProgress");
    }

    #[test]
    fn test_task_filter_category_url() {
        let filter = TaskFilter {
            status: None,
            priority: Some(TaskPriority::High),
            due_before: None,
            due_after: None,
        };

        let mut url = format!("{}/tasks", API_BASE);
        let mut params = Vec::new();

        if let Some(priority) = filter.priority {
            params.push(format!("category={:?}", priority));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        assert_eq!(url, "http://localhost:8080/api/tasks?category=Work");
    }

    #[test]
    fn test_task_filter_multiple_params_url() {
        let due_before = Utc::now() + chrono::Duration::days(7);
        let due_after = Utc::now() - chrono::Duration::days(1);

        let filter = TaskFilter {
            status: Some(TaskStatus::Todo),
            priority: Some(TaskPriority::Low),
            due_before: Some(due_before),
            due_after: Some(due_after),
        };

        let mut url = format!("{}/tasks", API_BASE);
        let mut params = Vec::new();

        if let Some(status) = filter.status {
            params.push(format!("status={:?}", status));
        }

        if let Some(category) = filter.priority {
            params.push(format!("category={:?}", category));
        }

        if let Some(due_before) = filter.due_before {
            params.push(format!("due_before={}", due_before.to_rfc3339()));
        }

        if let Some(due_after) = filter.due_after {
            params.push(format!("due_after={}", due_after.to_rfc3339()));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        assert!(url.contains("status=Todo"));
        assert!(url.contains("category=Personal"));
        assert!(url.contains("due_before="));
        assert!(url.contains("due_after="));
        assert!(url.contains("&"));
    }

    #[test]
    fn test_create_task_url() {
        let url = format!("{}/tasks", API_BASE);
        assert_eq!(url, "http://localhost:8080/api/tasks");
    }

    #[test]
    fn test_update_task_url() {
        let task_id = Uuid::new_v4();
        let url = format!("{}/tasks/{}", API_BASE, task_id);
        assert_eq!(url, format!("http://localhost:8080/api/tasks/{}", task_id));
    }

    #[test]
    fn test_delete_task_url() {
        let task_id = Uuid::new_v4();
        let url = format!("{}/tasks/{}", API_BASE, task_id);
        assert_eq!(url, format!("http://localhost:8080/api/tasks/{}", task_id));
    }

    #[test]
    fn test_create_task_request_structure() {
        let request = CreateTaskRequest {
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            priority: TaskPriority::High,
            due_date: None,
        };

        assert_eq!(request.title, "Test Task");
        assert_eq!(request.description, Some("Test Description".to_string()));
        assert_eq!(request.priority, TaskPriority::High);
        assert_eq!(request.due_date, None);
    }

    #[test]
    fn test_update_task_request_structure() {
        let request = UpdateTaskRequest {
            title: Some("Updated Title".to_string()),
            description: None,
            status: Some(TaskStatus::Completed),
            priority: Some(TaskPriority::Low),
            due_date: None,
        };

        assert_eq!(request.title, Some("Updated Title".to_string()));
        assert_eq!(request.description, None);
        assert_eq!(request.status, Some(TaskStatus::Completed));
        assert_eq!(request.priority, Some(TaskPriority::Low));
        assert_eq!(request.due_date, None);
    }

    // Note: The following tests would require a mock server to actually test the HTTP requests
    // In a real-world scenario, you would set up a mock HTTP server or use dependency injection
    // to test the actual API functions. For now, we test the URL generation and request structure.

    #[test]
    fn test_error_message_format() {
        let error_msg = format!("Request failed: {}", "Network error");
        assert_eq!(error_msg, "Request failed: Network error");

        let error_msg = format!("HTTP error: {}", 404);
        assert_eq!(error_msg, "HTTP error: 404");

        let error_msg = format!("Failed to parse response: {}", "Invalid JSON");
        assert_eq!(error_msg, "Failed to parse response: Invalid JSON");
    }

    #[test]
    fn test_date_rfc3339_formatting() {
        let date = Utc::now();
        let formatted = date.to_rfc3339();

        // Should be in RFC3339 format
        assert!(formatted.contains("T"));
        assert!(formatted.contains("Z") || formatted.contains("+"));
        assert!(formatted.len() >= 19); // Basic length check for YYYY-MM-DDTHH:MM:SS
    }
}
