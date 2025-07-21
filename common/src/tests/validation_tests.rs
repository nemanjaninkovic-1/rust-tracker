#[cfg(test)]
mod validation_tests {
    use crate::*;
    use chrono::{TimeZone, Utc};
    use serde_json;
    use uuid::Uuid;

    #[test]
    fn test_task_builder_pattern() {
        let task = Task {
            id: Uuid::new_v4(),
            title: "Builder Test".to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Low,
            due_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(task.title, "Builder Test");
        assert_eq!(task.status, TaskStatus::Todo);
        assert_eq!(task.priority, TaskPriority::Low);
    }

    #[test]
    fn test_task_with_all_fields() {
        let due_date = Utc.with_ymd_and_hms(2024, 12, 31, 23, 59, 59).unwrap();
        let task = Task {
            id: Uuid::new_v4(),
            title: "Complete Task".to_string(),
            description: Some("Full description".to_string()),
            status: TaskStatus::InProgress,
            priority: TaskPriority::Urgent,
            due_date: Some(due_date),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(!task.title.is_empty());
        assert!(task.description.is_some());
        assert_eq!(task.status, TaskStatus::InProgress);
        assert_eq!(task.priority, TaskPriority::Urgent);
        assert!(task.due_date.is_some());
    }

    #[test]
    fn test_create_task_request_validation() {
        let valid_request = CreateTaskRequest {
            title: "Valid Task".to_string(),
            description: Some("Valid description".to_string()),
            priority: TaskPriority::High,
            due_date: None,
        };

        assert!(!valid_request.title.is_empty());
        assert!(valid_request.description.is_some());
        assert_eq!(valid_request.priority, TaskPriority::High);
    }

    #[test]
    fn test_create_task_request_minimal() {
        let minimal_request = CreateTaskRequest {
            title: "Minimal".to_string(),
            description: None,
            priority: TaskPriority::Medium,
            due_date: None,
        };

        assert_eq!(minimal_request.title, "Minimal");
        assert!(minimal_request.description.is_none());
        assert!(minimal_request.due_date.is_none());
    }

    #[test]
    fn test_update_task_request_partial() {
        let partial_update = UpdateTaskRequest {
            title: Some("Updated Title".to_string()),
            description: None,
            status: None,
            priority: Some(TaskPriority::Low),
            due_date: None,
        };

        assert_eq!(partial_update.title, Some("Updated Title".to_string()));
        assert!(partial_update.description.is_none());
        assert!(partial_update.status.is_none());
        assert_eq!(partial_update.priority, Some(TaskPriority::Low));
    }

    #[test]
    fn test_update_task_request_complete() {
        let due_date = Utc.with_ymd_and_hms(2024, 6, 15, 10, 0, 0).unwrap();
        let complete_update = UpdateTaskRequest {
            title: Some("Completely Updated".to_string()),
            description: Some("New description".to_string()),
            status: Some(TaskStatus::Completed),
            priority: Some(TaskPriority::Urgent),
            due_date: Some(due_date),
        };

        assert!(complete_update.title.is_some());
        assert!(complete_update.description.is_some());
        assert!(complete_update.status.is_some());
        assert!(complete_update.priority.is_some());
        assert!(complete_update.due_date.is_some());
    }

    #[test]
    fn test_task_filter_all_fields() {
        let due_before = Utc.with_ymd_and_hms(2024, 12, 31, 0, 0, 0).unwrap();
        let due_after = Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap();

        let filter = TaskFilter {
            status: Some(TaskStatus::InProgress),
            priority: Some(TaskPriority::High),
            due_before: Some(due_before),
            due_after: Some(due_after),
        };

        assert_eq!(filter.status, Some(TaskStatus::InProgress));
        assert_eq!(filter.priority, Some(TaskPriority::High));
        assert!(filter.due_before.is_some());
        assert!(filter.due_after.is_some());
    }

    #[test]
    fn test_task_filter_single_field() {
        let status_filter = TaskFilter {
            status: Some(TaskStatus::Todo),
            priority: None,
            due_before: None,
            due_after: None,
        };

        assert_eq!(status_filter.status, Some(TaskStatus::Todo));
        assert!(status_filter.priority.is_none());
        assert!(status_filter.due_before.is_none());
        assert!(status_filter.due_after.is_none());
    }

    #[test]
    fn test_task_priority_ordering() {
        let mut priorities = vec![
            TaskPriority::Urgent,
            TaskPriority::Low,
            TaskPriority::High,
            TaskPriority::Medium,
        ];

        // Test individual comparisons
        assert!(TaskPriority::Urgent > TaskPriority::High);
        assert!(TaskPriority::High > TaskPriority::Medium);
        assert!(TaskPriority::Medium > TaskPriority::Low);
        assert!(TaskPriority::Urgent > TaskPriority::Low);

        // Test that sorting works correctly
        priorities.sort();
        let expected = vec![
            TaskPriority::Low,
            TaskPriority::Medium,
            TaskPriority::High,
            TaskPriority::Urgent,
        ];
        assert_eq!(priorities, expected);
    }

    #[test]
    fn test_task_status_equality() {
        let status1 = TaskStatus::Todo;
        let status2 = TaskStatus::Todo;
        let status3 = TaskStatus::InProgress;

        assert_eq!(status1, status2);
        assert_ne!(status1, status3);
    }

    #[test]
    fn test_task_json_serialization_roundtrip() {
        let original_task = Task {
            id: Uuid::new_v4(),
            title: "JSON Test".to_string(),
            description: Some("Test serialization".to_string()),
            status: TaskStatus::InProgress,
            priority: TaskPriority::High,
            due_date: Some(Utc::now()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let json = serde_json::to_string(&original_task).unwrap();
        let deserialized_task: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(original_task.id, deserialized_task.id);
        assert_eq!(original_task.title, deserialized_task.title);
        assert_eq!(original_task.status, deserialized_task.status);
        assert_eq!(original_task.priority, deserialized_task.priority);
    }

    #[test]
    fn test_create_request_json_serialization() {
        let request = CreateTaskRequest {
            title: "JSON Create Test".to_string(),
            description: Some("Create via JSON".to_string()),
            priority: TaskPriority::Medium,
            due_date: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.priority, deserialized.priority);
    }

    #[test]
    fn test_update_request_json_serialization() {
        let request = UpdateTaskRequest {
            title: Some("Updated via JSON".to_string()),
            description: None,
            status: Some(TaskStatus::Completed),
            priority: None,
            due_date: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(request.title, deserialized.title);
        assert_eq!(request.status, deserialized.status);
    }

    #[test]
    fn test_task_filter_json_serialization() {
        let filter = TaskFilter {
            status: Some(TaskStatus::Todo),
            priority: Some(TaskPriority::Urgent),
            due_before: None,
            due_after: None,
        };

        let json = serde_json::to_string(&filter).unwrap();
        let deserialized: TaskFilter = serde_json::from_str(&json).unwrap();

        assert_eq!(filter.status, deserialized.status);
        assert_eq!(filter.priority, deserialized.priority);
    }

    #[test]
    fn test_enum_string_representation() {
        // Test TaskStatus string representation
        assert_eq!(format!("{:?}", TaskStatus::Todo), "Todo");
        assert_eq!(format!("{:?}", TaskStatus::InProgress), "InProgress");
        assert_eq!(format!("{:?}", TaskStatus::Completed), "Completed");
        assert_eq!(format!("{:?}", TaskStatus::Backlog), "Backlog");

        // Test TaskPriority string representation
        assert_eq!(format!("{:?}", TaskPriority::Low), "Low");
        assert_eq!(format!("{:?}", TaskPriority::Medium), "Medium");
        assert_eq!(format!("{:?}", TaskPriority::High), "High");
        assert_eq!(format!("{:?}", TaskPriority::Urgent), "Urgent");
    }

    #[test]
    fn test_task_clone() {
        let original = Task {
            id: Uuid::new_v4(),
            title: "Clone Test".to_string(),
            description: Some("Test cloning".to_string()),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            due_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let cloned = original.clone();

        assert_eq!(original.id, cloned.id);
        assert_eq!(original.title, cloned.title);
        assert_eq!(original.description, cloned.description);
        assert_eq!(original.status, cloned.status);
        assert_eq!(original.priority, cloned.priority);
    }

    #[test]
    fn test_partial_eq_implementations() {
        let task1 = Task {
            id: Uuid::new_v4(),
            title: "Test".to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Low,
            due_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let task2 = task1.clone();
        let mut task3 = task1.clone();
        task3.title = "Different".to_string();

        assert_eq!(task1, task2);
        assert_ne!(task1, task3);
    }

    #[test]
    fn test_date_handling_edge_cases() {
        let far_future = Utc.with_ymd_and_hms(2099, 12, 31, 23, 59, 59).unwrap();
        let far_past = Utc.with_ymd_and_hms(1970, 1, 1, 0, 0, 0).unwrap();

        let future_task = Task {
            id: Uuid::new_v4(),
            title: "Future Task".to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Low,
            due_date: Some(far_future),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let past_task = Task {
            id: Uuid::new_v4(),
            title: "Past Task".to_string(),
            description: None,
            status: TaskStatus::Completed,
            priority: TaskPriority::Low,
            due_date: Some(far_past),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(future_task.due_date.unwrap() > Utc::now());
        assert!(past_task.due_date.unwrap() < Utc::now());
    }
}
