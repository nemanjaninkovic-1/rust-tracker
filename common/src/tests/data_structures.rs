#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{DateTime, TimeZone, Utc};
    use serde_json;
    use uuid::Uuid;

    #[test]
    fn test_task_status_default() {
        let status = TaskStatus::default();
        assert_eq!(status, TaskStatus::Todo);
    }

    #[test]
    fn test_task_status_serialization() {
        assert_eq!(
            serde_json::to_string(&TaskStatus::Todo).unwrap(),
            "\"Todo\""
        );
        assert_eq!(
            serde_json::to_string(&TaskStatus::InProgress).unwrap(),
            "\"InProgress\""
        );
        assert_eq!(
            serde_json::to_string(&TaskStatus::Completed).unwrap(),
            "\"Completed\""
        );
    }

    #[test]
    fn test_task_status_deserialization() {
        assert_eq!(
            serde_json::from_str::<TaskStatus>("\"Todo\"").unwrap(),
            TaskStatus::Todo
        );
        assert_eq!(
            serde_json::from_str::<TaskStatus>("\"InProgress\"").unwrap(),
            TaskStatus::InProgress
        );
        assert_eq!(
            serde_json::from_str::<TaskStatus>("\"Completed\"").unwrap(),
            TaskStatus::Completed
        );
    }

    #[test]
    fn test_task_status_invalid_deserialization() {
        assert!(serde_json::from_str::<TaskStatus>("\"Invalid\"").is_err());
        assert!(serde_json::from_str::<TaskStatus>("\"todo\"").is_err()); // case sensitive
    }

    #[test]
    fn test_task_serialization() {
        let task_id = Uuid::new_v4();
        let created_at = Utc.with_ymd_and_hms(2024, 1, 1, 12, 0, 0).unwrap();
        let updated_at = Utc.with_ymd_and_hms(2024, 1, 2, 12, 0, 0).unwrap();
        let due_date = Utc.with_ymd_and_hms(2024, 1, 10, 12, 0, 0).unwrap();

        let task = Task {
            id: task_id,
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            status: TaskStatus::InProgress,
            priority: TaskPriority::High,
            due_date: Some(due_date),
            created_at,
            updated_at,
        };

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.id, task.id);
        assert_eq!(deserialized.title, task.title);
        assert_eq!(deserialized.description, task.description);
        assert_eq!(deserialized.status, task.status);
        assert_eq!(deserialized.priority, task.priority);
        assert_eq!(deserialized.due_date, task.due_date);
        assert_eq!(deserialized.created_at, task.created_at);
        assert_eq!(deserialized.updated_at, task.updated_at);
    }

    #[test]
    fn test_task_with_none_values() {
        let task_id = Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let task = Task {
            id: task_id,
            title: "Minimal Task".to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Low,
            due_date: None,
            created_at,
            updated_at,
        };

        let json = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.due_date, None);
    }

    #[test]
    fn test_create_task_request() {
        let request = CreateTaskRequest {
            title: "New Task".to_string(),
            description: Some("Task description".to_string()),
            priority: TaskPriority::High,
            due_date: Some(Utc::now() + chrono::Duration::days(7)),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, request.title);
        assert_eq!(deserialized.description, request.description);
        assert_eq!(deserialized.priority, request.priority);
        assert_eq!(deserialized.due_date, request.due_date);
    }

    #[test]
    fn test_create_task_request_minimal() {
        let request = CreateTaskRequest {
            title: "Minimal Task".to_string(),
            description: None,
            priority: TaskPriority::Medium,
            due_date: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CreateTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, "Minimal Task");
        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.priority, TaskPriority::Medium);
        assert_eq!(deserialized.due_date, None);
    }

    #[test]
    fn test_update_task_request() {
        let request = UpdateTaskRequest {
            title: Some("Updated Title".to_string()),
            description: Some("Updated Description".to_string()),
            status: Some(TaskStatus::Completed),
            priority: Some(TaskPriority::Urgent),
            due_date: Some(Utc::now() + chrono::Duration::days(3)),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, request.title);
        assert_eq!(deserialized.description, request.description);
        assert_eq!(deserialized.status, request.status);
        assert_eq!(deserialized.priority, request.priority);
        assert_eq!(deserialized.due_date, request.due_date);
    }

    #[test]
    fn test_update_task_request_all_none() {
        let request = UpdateTaskRequest {
            title: None,
            description: None,
            status: None,
            priority: None,
            due_date: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: UpdateTaskRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.title, None);
        assert_eq!(deserialized.description, None);
        assert_eq!(deserialized.status, None);
        assert_eq!(deserialized.priority, None);
        assert_eq!(deserialized.due_date, None);
    }

    #[test]
    fn test_task_filter() {
        let filter = TaskFilter {
            status: Some(TaskStatus::InProgress),
            priority: Some(TaskPriority::High),
            due_before: Some(Utc::now() + chrono::Duration::days(7)),
            due_after: Some(Utc::now() - chrono::Duration::days(1)),
        };

        let json = serde_json::to_string(&filter).unwrap();
        let deserialized: TaskFilter = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.status, filter.status);
        assert_eq!(deserialized.priority, filter.priority);
        assert_eq!(deserialized.due_before, filter.due_before);
        assert_eq!(deserialized.due_after, filter.due_after);
    }

    #[test]
    fn test_task_filter_empty() {
        let filter = TaskFilter {
            status: None,
            priority: None,
            due_before: None,
            due_after: None,
        };

        let json = serde_json::to_string(&filter).unwrap();
        let deserialized: TaskFilter = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.status, None);
        assert_eq!(deserialized.priority, None);
        assert_eq!(deserialized.due_before, None);
        assert_eq!(deserialized.due_after, None);
    }

    #[test]
    fn test_task_equality() {
        let task_id = Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let task1 = Task {
            id: task_id,
            title: "Task".to_string(),
            description: Some("Description".to_string()),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            due_date: None,
            created_at,
            updated_at,
        };

        let task2 = Task {
            id: task_id,
            title: "Task".to_string(),
            description: Some("Description".to_string()),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            due_date: None,
            created_at,
            updated_at,
        };

        assert_eq!(task1, task2);
    }

    #[test]
    fn test_task_inequality() {
        let task_id1 = Uuid::new_v4();
        let task_id2 = Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let task1 = Task {
            id: task_id1,
            title: "Task".to_string(),
            description: Some("Description".to_string()),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            due_date: None,
            created_at,
            updated_at,
        };

        let task2 = Task {
            id: task_id2,
            title: "Task".to_string(),
            description: Some("Description".to_string()),
            status: TaskStatus::Todo,
            priority: TaskPriority::Medium,
            due_date: None,
            created_at,
            updated_at,
        };

        assert_ne!(task1, task2);
    }

    #[test]
    fn test_enum_derive_traits() {
        // Test Clone/Copy
        let status = TaskStatus::InProgress;
        let status_clone = status; // Use copy instead of clone for Copy types
        assert_eq!(status, status_clone);

        let priority = TaskPriority::High;
        let priority_clone = priority; // Use copy instead of clone for Copy types
        assert_eq!(priority, priority_clone);

        // Test Copy
        let status_copy = status;
        assert_eq!(status, status_copy);

        let priority_copy = priority;
        assert_eq!(priority, priority_copy);

        // Test Debug
        let debug_str = format!("{:?}", TaskStatus::Completed);
        assert_eq!(debug_str, "Completed");

        let debug_str = format!("{:?}", TaskPriority::Urgent);
        assert_eq!(debug_str, "Urgent");
    }

    #[test]
    fn test_struct_derive_traits() {
        let task_id = Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = Utc::now();

        let task = Task {
            id: task_id,
            title: "Debug Test".to_string(),
            description: None,
            status: TaskStatus::Todo,
            priority: TaskPriority::Low,
            due_date: None,
            created_at,
            updated_at,
        };

        // Test Debug
        let debug_str = format!("{task:?}");
        assert!(debug_str.contains("Debug Test"));
        assert!(debug_str.contains("Todo"));
        assert!(debug_str.contains("Low"));

        // Test Clone
        let task_clone = task.clone();
        assert_eq!(task, task_clone);
    }

    #[test]
    fn test_invalid_json_handling() {
        // Test invalid JSON for enums
        assert!(serde_json::from_str::<TaskStatus>("\"InvalidStatus\"").is_err());
        assert!(serde_json::from_str::<TaskPriority>("\"InvalidPriority\"").is_err());

        // Test malformed JSON
        assert!(serde_json::from_str::<Task>("{invalid json}").is_err());
        assert!(serde_json::from_str::<CreateTaskRequest>("{\"title\":}").is_err());
    }

    #[test]
    fn test_datetime_serialization_format() {
        let datetime = Utc.with_ymd_and_hms(2024, 3, 15, 14, 30, 0).unwrap();
        let serialized = serde_json::to_string(&datetime).unwrap();
        let deserialized: DateTime<Utc> = serde_json::from_str(&serialized).unwrap();

        assert_eq!(datetime, deserialized);
    }

    #[test]
    fn test_uuid_serialization() {
        let uuid = Uuid::new_v4();
        let serialized = serde_json::to_string(&uuid).unwrap();
        let deserialized: Uuid = serde_json::from_str(&serialized).unwrap();

        assert_eq!(uuid, deserialized);
    }
}
