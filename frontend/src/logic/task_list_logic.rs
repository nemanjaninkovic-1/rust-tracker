use common::{Task, TaskPriority, TaskStatus};
use std::collections::HashMap;

#[must_use]
pub fn filter_and_group_tasks(
    tasks: &[Task],
    priority_filter: Option<TaskPriority>,
) -> HashMap<TaskStatus, Vec<Task>> {
    let filtered: Vec<Task> = tasks
        .iter()
        .filter(|&task| {
            let priority_match =
                priority_filter.is_none() || priority_filter == Some(task.priority);
            let not_backlog = task.status != TaskStatus::Backlog;
            priority_match && not_backlog
        })
        .cloned()
        .collect();

    let mut grouped: HashMap<TaskStatus, Vec<Task>> = HashMap::new();
    for task in filtered {
        grouped.entry(task.status).or_default().push(task);
    }
    for tasks_list in grouped.values_mut() {
        tasks_list.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    for status in [
        TaskStatus::Todo,
        TaskStatus::InProgress,
        TaskStatus::Completed,
    ] {
        grouped.entry(status).or_default();
    }
    grouped
}
