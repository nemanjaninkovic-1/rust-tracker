use common::{Task, TaskPriority};
use leptos::logging::log;
use leptos::*;

#[component]
pub fn Card(task: Task, set_dragging_task_id: WriteSignal<Option<uuid::Uuid>>) -> impl IntoView {
    // Card for displaying a task
    let task_name = task.title.clone();

    let task_description = task.description.clone().unwrap_or_default();

    // Limit description to 15 words for display
    let display_description = {
        let words: Vec<&str> = task_description.split_whitespace().collect();
        if words.len() > 15 {
            format!("{}...", words[..15].join(" "))
        } else {
            task_description.clone()
        }
    };

    let date = task
        .due_date
        .map(|d| d.format("%Y %b %d").to_string())
        .unwrap_or_default();
    let priority_label = match task.priority {
        TaskPriority::Low => "Low",
        TaskPriority::Medium => "Mid",
        TaskPriority::High => "Hi",
        TaskPriority::Urgent => "Top",
    };
    view! {
        <div
            class="bg-white dark:bg-gray-800 rounded-xl shadow-lg border-2 border-gray-300 dark:border-gray-600 p-4 mb-4 w-full max-w-sm h-48 flex flex-col select-none cursor-grab active:cursor-grabbing pointer-events-auto hover:border-gray-400 dark:hover:border-gray-500 transition-colors"
            draggable="true"
            on:dragstart=move |_ev| {
                log!("Drag started for task {}", task.id);
                set_dragging_task_id.set(Some(task.id));
                log!("Set dragging task ID to: {}", task.id);
            }
            on:dragend=move |ev| {
                // Prevent any default scrolling behavior on drag end
                ev.prevent_default();
                ev.stop_propagation();
                log!("Drag ended for task {}", task.id);

                // Use a slight delay before clearing to ensure drop events complete
                set_timeout(
                    move || {
                        set_dragging_task_id.set(None);
                        log!("Cleared dragging task ID after timeout");
                    },
                    std::time::Duration::from_millis(50)
                );
            }
            on:dragover=move |ev| {
                // Allow the card to be dragged over and let the event bubble up to the drop zone
                ev.prevent_default();
                // Don't stop propagation - let it bubble up to the column drop zone
            }
            on:drop=move |_ev| {
                // Allow drop events to bubble up to the parent drop zone
                // Don't prevent default or stop propagation
                log!("Drop event on card {}, bubbling up", task.id);
            }
        >
            <div class="flex items-start mb-2">
                <span class={format!("px-1 py-0.5 rounded text-xs font-bold mr-2 flex-shrink-0 w-8 text-center {}", match task.priority {
                    TaskPriority::Low => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300",
                    TaskPriority::Medium => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300",
                    TaskPriority::High => "bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300",
                    TaskPriority::Urgent => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
                })}>{priority_label}</span>
                <div class="flex-1 min-w-0 mr-2">
                    <div class="font-semibold text-gray-900 dark:text-gray-100 leading-tight text-sm overflow-hidden" style="display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical;">
                        {task_name}
                    </div>
                </div>
                <button class="flex-shrink-0 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300" title="Follow">
                    <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v6m3-3h-6" /></svg>
                </button>
            </div>
            <div class="bg-gray-50 dark:bg-gray-900 rounded-lg p-3 mb-3 border border-gray-100 dark:border-gray-800 flex-1 overflow-hidden">
                <div class="text-sm text-gray-600 dark:text-gray-300 overflow-hidden mb-2" style="display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical;">
                    {display_description}
                </div>
            </div>
            {
                if !date.is_empty() {
                    view! {
                        <div class="flex justify-between items-center text-xs text-gray-500 dark:text-gray-400 mt-auto">
                            <span>"Due: " {date}</span>
                        </div>
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }
            }
        </div>
    }
}
