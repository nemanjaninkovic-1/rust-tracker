#[cfg(test)]
#[allow(dead_code)]
mod component_tests {
    use chrono::Utc;
    use common::{Task, TaskPriority, TaskStatus};
    use uuid::Uuid;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    fn create_test_task() -> Task {
        Task {
            id: Uuid::new_v4(),
            title: "Test Task".to_string(),
            description: Some("Test Description".to_string()),
            status: TaskStatus::Todo,
            priority: TaskPriority::High,
            due_date: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[wasm_bindgen_test]
    fn test_task_status_display() {
        assert_eq!(format!("{:?}", TaskStatus::Todo), "Todo");
        assert_eq!(format!("{:?}", TaskStatus::InProgress), "InProgress");
        assert_eq!(format!("{:?}", TaskStatus::Completed), "Completed");
    }

    #[wasm_bindgen_test]
    fn test_task_priority_display() {
        assert_eq!(format!("{:?}", TaskPriority::Low), "Low");
        assert_eq!(format!("{:?}", TaskPriority::Medium), "Medium");
        assert_eq!(format!("{:?}", TaskPriority::High), "High");
        assert_eq!(format!("{:?}", TaskPriority::Urgent), "Urgent");
    }

    #[wasm_bindgen_test]
    fn test_task_creation() {
        let task = create_test_task();

        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, Some("Test Description".to_string()));
        assert_eq!(task.status, TaskStatus::Todo);
        assert_eq!(task.priority, TaskPriority::High);
        assert!(task.due_date.is_none());
        assert!(task.id != Uuid::nil());
    }

    #[wasm_bindgen_test]
    fn test_task_with_due_date() {
        let due_date = Utc::now() + chrono::Duration::days(7);
        let mut task = create_test_task();
        task.due_date = Some(due_date);

        assert!(task.due_date.is_some());
        assert_eq!(task.due_date.unwrap(), due_date);
    }

    #[wasm_bindgen_test]
    fn test_task_status_progression() {
        let mut task = create_test_task();

        // Start as Todo
        assert_eq!(task.status, TaskStatus::Todo);

        // Move to InProgress
        task.status = TaskStatus::InProgress;
        assert_eq!(task.status, TaskStatus::InProgress);

        // Complete the task
        task.status = TaskStatus::Completed;
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[wasm_bindgen_test]
    fn test_task_priorities() {
        let priorities = vec![
            TaskPriority::Low,
            TaskPriority::Medium,
            TaskPriority::High,
            TaskPriority::Urgent,
        ];

        assert_eq!(priorities.len(), 4);

        // Test default
        assert_eq!(TaskPriority::default(), TaskPriority::Medium);
    }

    #[wasm_bindgen_test]
    fn test_task_without_description() {
        let mut task = create_test_task();
        task.description = None;

        assert!(task.description.is_none());
    }

    #[wasm_bindgen_test]
    fn test_task_equality() {
        let task1 = create_test_task();
        let mut task2 = task1.clone();

        assert_eq!(task1, task2);

        // Change title
        task2.title = "Different Title".to_string();
        assert_ne!(task1, task2);
    }

    #[wasm_bindgen_test]
    fn test_overdue_task_logic() {
        let past_date = Utc::now() - chrono::Duration::days(1);
        let future_date = Utc::now() + chrono::Duration::days(1);

        let mut overdue_task = create_test_task();
        overdue_task.due_date = Some(past_date);

        let future_task = {
            let mut task = create_test_task();
            task.due_date = Some(future_date);
            task
        };

        // Test overdue logic
        if let Some(due) = overdue_task.due_date {
            assert!(due < Utc::now());
        }

        if let Some(due) = future_task.due_date {
            assert!(due > Utc::now());
        }
    }

    #[wasm_bindgen_test]
    fn test_task_filtering_logic() {
        let work_task = {
            let mut task = create_test_task();
            task.priority = TaskPriority::High;
            task
        };

        let personal_task = {
            let mut task = create_test_task();
            task.priority = TaskPriority::Low;
            task
        };

        let completed_task = {
            let mut task = create_test_task();
            task.status = TaskStatus::Completed;
            task
        };

        let tasks = vec![work_task, personal_task, completed_task];

        // Filter by category
        let work_tasks: Vec<_> = tasks
            .iter()
            .filter(|task| task.priority == TaskPriority::High)
            .collect();
        assert_eq!(work_tasks.len(), 1);

        // Filter by status
        let completed_tasks: Vec<_> = tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Completed)
            .collect();
        assert_eq!(completed_tasks.len(), 1);

        // Filter by multiple criteria
        let todo_work_tasks: Vec<_> = tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Todo && task.priority == TaskPriority::High)
            .collect();
        assert_eq!(todo_work_tasks.len(), 1);
    }

    #[wasm_bindgen_test]
    fn test_task_sorting_logic() {
        let older_task = {
            let mut task = create_test_task();
            task.created_at = Utc::now() - chrono::Duration::hours(2);
            task.title = "Older Task".to_string();
            task
        };

        let newer_task = {
            let mut task = create_test_task();
            task.created_at = Utc::now() - chrono::Duration::hours(1);
            task.title = "Newer Task".to_string();
            task
        };

        let mut tasks = vec![older_task.clone(), newer_task.clone()];

        // Sort by creation time (newest first)
        tasks.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        assert_eq!(tasks[0].title, "Newer Task");
        assert_eq!(tasks[1].title, "Older Task");
    }

    #[wasm_bindgen_test]
    fn test_task_update_timestamps() {
        let mut task = create_test_task();
        let original_updated_at = task.updated_at;

        // Simulate update
        std::thread::sleep(std::time::Duration::from_millis(1));
        task.updated_at = Utc::now();

        assert!(task.updated_at >= original_updated_at);
    }

    #[wasm_bindgen_test]
    fn test_task_validation_logic() {
        let task = create_test_task();

        // Title should not be empty
        assert!(!task.title.trim().is_empty());

        // ID should be valid
        assert!(task.id != Uuid::nil());

        // Timestamps should be valid
        assert!(task.created_at <= Utc::now());
        assert!(task.updated_at <= Utc::now());
        assert!(task.updated_at >= task.created_at);
    }

    #[wasm_bindgen_test]
    fn test_task_priority_logic() {
        let high_priority_task = {
            let mut task = create_test_task();
            task.due_date = Some(Utc::now() + chrono::Duration::hours(1)); // Due very soon
            task
        };

        let low_priority_task = {
            let mut task = create_test_task();
            task.due_date = Some(Utc::now() + chrono::Duration::days(30)); // Due in a month
            task
        };

        let no_due_date_task = create_test_task(); // No due date

        // Logic for determining priority based on due date
        fn is_urgent(task: &Task) -> bool {
            if let Some(due_date) = task.due_date {
                let hours_until_due = (due_date - Utc::now()).num_hours();
                hours_until_due < 24 // Urgent if due within 24 hours
            } else {
                false
            }
        }

        assert!(is_urgent(&high_priority_task));
        assert!(!is_urgent(&low_priority_task));
        assert!(!is_urgent(&no_due_date_task));
    }
}
