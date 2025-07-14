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
        <div class="space-y-6">
            <div class="bg-white rounded-lg shadow-sm border p-4">
                <h3 class="text-lg font-semibold text-gray-800 mb-4">"Filter Tasks"</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Status"
                        </label>
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
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        >
                            <option value="">"All Status"</option>
                            <option value="Todo">"Todo"</option>
                            <option value="InProgress">"In Progress"</option>
                            <option value="Completed">"Completed"</option>
                        </select>
                    </div>

                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">
                            "Category"
                        </label>
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
                            class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        >
                            <option value="">"All Categories"</option>
                            <option value="Work">"Work"</option>
                            <option value="Personal">"Personal"</option>
                            <option value="Shopping">"Shopping"</option>
                            <option value="Health">"Health"</option>
                            <option value="Other">"Other"</option>
                        </select>
                    </div>
                </div>
            </div>

            <div class="space-y-4">
                {move || {
                    let tasks = filtered_tasks.get();
                    if tasks.is_empty() {
                        view! {
                            <div class="text-center py-12 bg-white rounded-lg shadow-sm border">
                                <div class="text-gray-400 text-6xl mb-4">"📝"</div>
                                <h3 class="text-lg font-medium text-gray-900 mb-2">"No tasks found"</h3>
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
