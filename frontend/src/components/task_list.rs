use crate::api;
use crate::components::TaskItem;
use common::{Task, TaskPriority, TaskStatus, UpdateTaskRequest};
use leptos::*;
use std::collections::HashMap;

#[component]
#[allow(non_snake_case)]
pub fn TaskList<F>(tasks: ReadSignal<Vec<Task>>, on_task_change: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let (filter_priority, set_filter_priority) = create_signal(None::<TaskPriority>);
    let (drag_over_status, set_drag_over_status) = create_signal(None::<TaskStatus>);
    let (dragging_task_id, set_dragging_task_id) = create_signal(None::<uuid::Uuid>);

    let update_task_action =
        create_action(move |(id, request): &(uuid::Uuid, UpdateTaskRequest)| {
            let id = *id;
            let request = request.clone();
            async move { api::update_task(id, request).await }
        });

    create_effect(move |_| {
        if let Some(result) = update_task_action.value().get() {
            if result.is_ok() {
                on_task_change();
            }
        }
    });

    let filtered_and_grouped_tasks = create_memo(move |_| {
        let tasks = tasks.get();
        let priority_filter = filter_priority.get();

        let filtered: Vec<Task> = tasks
            .into_iter()
            .filter(|task| {
                // Filter by priority and exclude Backlog tasks from main view
                let priority_match = priority_filter.is_none_or(|p| task.priority == p);
                let not_backlog = task.status != TaskStatus::Backlog;
                priority_match && not_backlog
            })
            .collect();

        // Group tasks by status
        let mut grouped: HashMap<TaskStatus, Vec<Task>> = HashMap::new();
        for task in filtered {
            grouped.entry(task.status).or_default().push(task);
        }

        // Sort tasks within each status by priority (Urgent -> High -> Medium -> Low)
        for tasks_list in grouped.values_mut() {
            tasks_list.sort_by(|a, b| b.priority.cmp(&a.priority));
        }

        // Ensure all statuses are present (excluding Backlog for main view)
        for status in [
            TaskStatus::Todo,
            TaskStatus::InProgress,
            TaskStatus::Completed,
        ] {
            grouped.entry(status).or_default();
        }

        grouped
    });

    let handle_drop = move |status: TaskStatus, task_id: uuid::Uuid| {
        let request = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(status),
            priority: None,
            due_date: None,
        };
        update_task_action.dispatch((task_id, request));
    };

    let render_status_column = move |status: TaskStatus| {
        let status_name = match status {
            TaskStatus::Todo => "Todo",
            TaskStatus::InProgress => "In Progress",
            TaskStatus::Completed => "Completed",
            TaskStatus::Backlog => "Backlog",
        };

        let status_color = match status {
            TaskStatus::Todo => "border-gray-300 bg-gray-50",
            TaskStatus::InProgress => "border-blue-300 bg-blue-50",
            TaskStatus::Completed => "border-green-300 bg-green-50",
            TaskStatus::Backlog => "border-purple-300 bg-purple-50",
        };

        let is_drag_over = move || drag_over_status.get() == Some(status);

        view! {
            <div
                class={move || format!("min-h-96 p-4 rounded-lg border-2 transition-colors {}",
                    if is_drag_over() {
                        match status {
                            TaskStatus::Todo => "border-gray-400 bg-gray-100",
                            TaskStatus::InProgress => "border-blue-400 bg-blue-100",
                            TaskStatus::Completed => "border-green-400 bg-green-100",
                            TaskStatus::Backlog => "border-purple-400 bg-purple-100",
                        }
                    } else {
                        status_color
                    }
                )}
                on:dragover=move |ev| {
                    ev.prevent_default();
                    set_drag_over_status.set(Some(status));
                }
                on:dragleave=move |_| {
                    set_drag_over_status.set(None);
                }
                on:drop=move |ev| {
                    ev.prevent_default();
                    set_drag_over_status.set(None);
                    if let Some(dragged_id) = dragging_task_id.get() {
                        handle_drop(status, dragged_id);
                        set_dragging_task_id.set(None);
                    }
                }
            >
                <h3 class="text-lg font-semibold mb-4 text-center">{status_name}</h3>
                <div class="space-y-3">
                    {move || {
                        let grouped = filtered_and_grouped_tasks.get();
                        if let Some(status_tasks) = grouped.get(&status) {
                            if status_tasks.is_empty() {
                                view! {
                                    <div class="text-center py-8 text-gray-400">
                                        <div class="text-2xl mb-2">"+"</div>
                                        <p class="text-sm">"Drop tasks here"</p>
                                    </div>
                                }.into_view()
                            } else {
                                status_tasks.iter().map(|task| {
                                    view! {
                                        <TaskItem
                                            task=task.clone()
                                            on_update=on_task_change
                                            set_dragging_task_id=set_dragging_task_id
                                        />
                                    }
                                }).collect_view()
                            }
                        } else {
                            view! {
                                <div class="text-center py-8 text-gray-400">
                                    <div class="text-2xl mb-2">"+"</div>
                                    <p class="text-sm">"Drop tasks here"</p>
                                </div>
                            }.into_view()
                        }
                    }}
                </div>
            </div>
        }
    };

    view! {
        <div class="space-y-6">
            // Filter bar - only category filter now
            <div class="flex flex-wrap items-center gap-3 pb-3 border-b-2 border-blue-200 bg-blue-50 px-3 py-2 rounded">
                <span class="text-sm font-medium text-blue-700">"Filter:"</span>
                <select
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let priority = match value.as_str() {
                            "Low" => Some(TaskPriority::Low),
                            "Medium" => Some(TaskPriority::Medium),
                            "High" => Some(TaskPriority::High),
                            "Urgent" => Some(TaskPriority::Urgent),
                            _ => None,
                        };
                        set_filter_priority.set(priority);
                    }
                    class="text-sm px-2 py-1 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                    <option value="">"All Priorities"</option>
                    <option value="Low">"Low"</option>
                    <option value="Medium">"Medium"</option>
                    <option value="High">"High"</option>
                    <option value="Urgent">"Urgent"</option>
                </select>
                <div class="ml-auto text-sm text-gray-500">
                    {move || {
                        let grouped = filtered_and_grouped_tasks.get();
                        let total_count: usize = grouped.values().map(|tasks| tasks.len()).sum();
                        if total_count == 1 {
                            "1 task".to_string()
                        } else {
                            format!("{total_count} tasks")
                        }
                    }}
                </div>
            </div>

            // Status columns layout - 3 columns without Backlog
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                {render_status_column(TaskStatus::Todo)}
                {render_status_column(TaskStatus::InProgress)}
                {render_status_column(TaskStatus::Completed)}
            </div>
        </div>
    }
}
