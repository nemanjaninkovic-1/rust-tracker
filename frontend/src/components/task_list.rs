use crate::components::TaskItem;
use common::{Task, TaskCategory, TaskStatus};
use leptos::*;

#[component]
pub fn TaskList<F>(tasks: ReadSignal<Vec<Task>>, on_task_change: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let (filter_status, set_filter_status) = create_signal(None::<TaskStatus>);
    let (filter_category, set_filter_category) = create_signal(None::<TaskCategory>);

    let filtered_tasks = create_memo(move |_| {
        let tasks = tasks.get();
        let status_filter = filter_status.get();
        let category_filter = filter_category.get();

        tasks
            .into_iter()
            .filter(|task| {
                let status_match = status_filter.is_none_or(|s| task.status == s);
                let category_match = category_filter.is_none_or(|c| task.category == c);
                status_match && category_match
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="space-y-3">
            // Compact filter bar - UPDATED LAYOUT
            <div class="flex flex-wrap items-center gap-3 pb-3 border-b-2 border-blue-200 bg-blue-50 px-3 py-2 rounded">
                <span class="text-sm font-medium text-blue-700">"Filter:"</span>
                <select
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let status = match value.as_str() {
                            "Todo" => Some(TaskStatus::Todo),
                            "InProgress" => Some(TaskStatus::InProgress),
                            "Completed" => Some(TaskStatus::Completed),
                            _ => None,
                        };
                        set_filter_status.set(status);
                    }
                    class="text-sm px-2 py-1 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                    <option value="">"All Status"</option>
                    <option value="Todo">"Todo"</option>
                    <option value="InProgress">"In Progress"</option>
                    <option value="Completed">"Completed"</option>
                </select>
                <select
                    on:change=move |ev| {
                        let value = event_target_value(&ev);
                        let category = match value.as_str() {
                            "Work" => Some(TaskCategory::Work),
                            "Personal" => Some(TaskCategory::Personal),
                            "Shopping" => Some(TaskCategory::Shopping),
                            "Health" => Some(TaskCategory::Health),
                            "Other" => Some(TaskCategory::Other),
                            _ => None,
                        };
                        set_filter_category.set(category);
                    }
                    class="text-sm px-2 py-1 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                >
                    <option value="">"All Categories"</option>
                    <option value="Work">"Work"</option>
                    <option value="Personal">"Personal"</option>
                    <option value="Shopping">"Shopping"</option>
                    <option value="Health">"Health"</option>
                    <option value="Other">"Other"</option>
                </select>
                <div class="ml-auto text-sm text-gray-500">
                    {move || {
                        let count = filtered_tasks.get().len();
                        if count == 1 {
                            "1 task".to_string()
                        } else {
                            format!("{} tasks", count)
                        }
                    }}
                </div>
            </div>

            // Tasks section - now the main focus
            <div class="space-y-2">
                {move || {
                    let tasks = filtered_tasks.get();
                    if tasks.is_empty() {
                        view! {
                            <div class="text-center py-8 bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
                                <div class="text-gray-400 text-4xl mb-3">"📝"</div>
                                <h3 class="text-lg font-medium text-gray-900 mb-1">"No tasks found"</h3>
                                <p class="text-gray-500">"Create your first task to get started!"</p>
                            </div>
                        }.into_view()
                    } else {
                        tasks.into_iter().map(|task| {
                            view! {
                                <TaskItem task=task on_update=on_task_change />
                            }
                        }).collect_view()
                    }
                }}
            </div>
        </div>
    }
}
