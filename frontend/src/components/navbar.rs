use common::TaskPriority;
use leptos::*;

#[component]
pub fn Navbar<F>(
    filter_priority: ReadSignal<Option<TaskPriority>>,
    set_filter_priority: WriteSignal<Option<TaskPriority>>,
    on_add_task: F,
) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    view! {
        <nav class="bg-gray-800 dark:bg-gray-900 border-b border-gray-700 dark:border-gray-600 px-4 py-1 mb-2">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                    // Clear filters button
                    <button
                        on:click=move |_| set_filter_priority.set(None)
                        class="bg-gray-600 hover:bg-gray-500 text-gray-200 px-2 py-1 rounded text-xs transition-colors"
                    >
                        "Clear Filters"
                    </button>

                    // Priority filter dropdown (no label, no arrow)
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
                        class="bg-gray-700 dark:bg-gray-800 text-gray-100 dark:text-gray-100 border border-gray-600 dark:border-gray-600 rounded px-2 py-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-32 appearance-none"
                        style="background-image: none;"
                    >
                        <option value="" selected=move || filter_priority.get().is_none()>"All Priorities"</option>
                        <option value="Low" selected=move || filter_priority.get() == Some(TaskPriority::Low)>"Low"</option>
                        <option value="Medium" selected=move || filter_priority.get() == Some(TaskPriority::Medium)>"Medium"</option>
                        <option value="High" selected=move || filter_priority.get() == Some(TaskPriority::High)>"High"</option>
                        <option value="Urgent" selected=move || filter_priority.get() == Some(TaskPriority::Urgent)>"Urgent"</option>
                    </select>

                    // Status dropdown placeholder (no label, no arrow)
                    <select
                        class="bg-gray-700 dark:bg-gray-800 text-gray-100 dark:text-gray-100 border border-gray-600 dark:border-gray-600 rounded px-2 py-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-32 appearance-none"
                        style="background-image: none;"
                    >
                        <option selected>"All Status"</option>
                        <option>"Todo"</option>
                        <option>"In Progress"</option>
                        <option>"Completed"</option>
                    </select>

                    // Assignee dropdown placeholder (no label, no arrow)
                    <select
                        class="bg-gray-700 dark:bg-gray-800 text-gray-100 dark:text-gray-100 border border-gray-600 dark:border-gray-600 rounded px-2 py-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-32 appearance-none"
                        style="background-image: none;"
                    >
                        <option selected>"All Users"</option>
                        <option>"Pratham"</option>
                        <option>"Marko"</option>
                        <option>"Sarah"</option>
                        <option>"John"</option>
                    </select>
                </div>

                <div class="flex items-center space-x-3">
                    // Search textbox (no label)
                    <input
                        type="text"
                        placeholder="Search tasks..."
                        class="bg-gray-700 dark:bg-gray-800 text-gray-100 dark:text-gray-100 border border-gray-600 dark:border-gray-600 rounded px-2 py-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-32"
                    />

                    // Additional textbox (no label)
                    <input
                        type="text"
                        placeholder="Filter by tags..."
                        class="bg-gray-700 dark:bg-gray-800 text-gray-100 dark:text-gray-100 border border-gray-600 dark:border-gray-600 rounded px-2 py-1 text-xs focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-32"
                    />

                    // Add Task Button
                    <button
                        on:click=move |_| on_add_task()
                        class="bg-blue-600 text-white px-2 py-1 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 focus:ring-offset-gray-800 transition-colors text-xs"
                    >
                        "Add Task"
                    </button>
                </div>
            </div>
        </nav>
    }
}
