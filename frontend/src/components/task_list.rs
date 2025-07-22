use crate::components::Card;
use crate::logic::task_list_logic::filter_and_group_tasks;
use crate::logic::task_list_signals::{use_update_task_action, TaskListSignals};
use common::{Task, TaskStatus, UpdateTaskRequest};
use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn TaskList(
    tasks: ReadSignal<Vec<Task>>,
    set_tasks: WriteSignal<Vec<Task>>,
    _refresh_tasks: impl Fn() + 'static + Copy,
    filter_priority: ReadSignal<Option<common::TaskPriority>>,
) -> impl IntoView {
    let signals = TaskListSignals::new();
    let (update_task_action, on_success) = use_update_task_action();

    // Create effect to handle server response and revert on failure
    create_effect(move |_| {
        on_success();
        if let Some(result) = update_task_action.value().get() {
            match result {
                Ok(_) => {
                    leptos::logging::log!(
                        "Task updated successfully on server - optimistic update confirmed"
                    );
                }
                Err(e) => {
                    leptos::logging::log!(
                        "Task update failed on server: {:?}, reverting optimistic update",
                        e
                    );
                    // Revert the optimistic update by refreshing from server
                    _refresh_tasks();
                }
            }
        }
    });
    let filtered_and_grouped_tasks = create_memo(move |_| {
        let tasks = tasks.get();
        let priority_filter = filter_priority.get();
        filter_and_group_tasks(&tasks, priority_filter)
    });

    let handle_drop = move |status: TaskStatus, task_id: uuid::Uuid| {
        leptos::logging::log!("Dropping task {} to status {:?}", task_id, status);

        // Optimistic update: immediately update the local task list
        set_tasks.update(|tasks| {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == task_id) {
                task.status = status;
                leptos::logging::log!(
                    "Optimistically updated task {} to status {:?}",
                    task_id,
                    status
                );
            }
        });

        // Then send the update to the server
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
            TaskStatus::Todo => "border-gray-600 bg-gray-800",
            TaskStatus::InProgress => "border-blue-600 bg-blue-900",
            TaskStatus::Completed => "border-green-600 bg-green-900",
            TaskStatus::Backlog => "border-purple-600 bg-purple-900",
        };

        let is_drag_over = move || signals.drag_over_status.get() == Some(status);

        view! {
            <div
                class={move || format!("min-h-96 p-4 rounded-lg border-2 transition-colors {}",
                    if is_drag_over() {
                        match status {
                            TaskStatus::Todo => "border-gray-400 bg-gray-700",
                            TaskStatus::InProgress => "border-blue-400 bg-blue-800",
                            TaskStatus::Completed => "border-green-400 bg-green-800",
                            TaskStatus::Backlog => "border-purple-400 bg-purple-800",
                        }
                    } else {
                        status_color
                    }
                )}
                on:dragover=move |ev| {
                    ev.prevent_default();
                    ev.stop_propagation();
                    leptos::logging::log!("Drag over status {:?}", status);
                    signals.drag_over_status.set(Some(status));
                }
                on:dragleave=move |ev| {
                    ev.stop_propagation();
                    leptos::logging::log!("Drag leave status {:?}", status);

                    // Use a longer timeout to prevent issues with fast dragging
                    set_timeout(
                        move || {
                            // Only clear if we're still on the same status
                            if signals.drag_over_status.get() == Some(status) {
                                signals.drag_over_status.set(None);
                            }
                        },
                        std::time::Duration::from_millis(100)
                    );
                }
                on:drop=move |ev| {
                    ev.prevent_default();
                    ev.stop_propagation();
                    leptos::logging::log!("Drop event on status {:?}", status);
                    signals.drag_over_status.set(None);
                    if let Some(dragged_id) = signals.dragging_task_id.get() {
                        leptos::logging::log!("Processing drop for task {} to status {:?}", dragged_id, status);
                        handle_drop(status, dragged_id);
                        signals.dragging_task_id.set(None);
                    } else {
                        leptos::logging::log!("No dragging task ID found during drop");
                    }
                }
            >
                <h3 class="text-lg font-semibold mb-4 text-center text-white">{status_name}</h3>
                <div class="space-y-3 min-h-80 relative"
                    on:dragover=move |ev| {
                        ev.prevent_default();
                        ev.stop_propagation();
                        leptos::logging::log!("Drag over task area for status {:?}", status);
                        signals.drag_over_status.set(Some(status));
                    }
                    on:dragleave=move |ev| {
                        ev.stop_propagation();
                        leptos::logging::log!("Drag leave task area for status {:?}", status);

                        // Use timeout for inner area as well
                        set_timeout(
                            move || {
                                if signals.drag_over_status.get() == Some(status) {
                                    signals.drag_over_status.set(None);
                                }
                            },
                            std::time::Duration::from_millis(100)
                        );
                    }
                    on:drop=move |ev| {
                        ev.prevent_default();
                        ev.stop_propagation();
                        leptos::logging::log!("Drop event on task area for status {:?}", status);
                        signals.drag_over_status.set(None);
                        if let Some(dragged_id) = signals.dragging_task_id.get() {
                            leptos::logging::log!("Processing drop in task area for task {} to status {:?}", dragged_id, status);
                            handle_drop(status, dragged_id);
                            signals.dragging_task_id.set(None);
                        } else {
                            leptos::logging::log!("No dragging task ID found during drop in task area");
                        }
                    }
                >
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
                                        <Card task=task.clone() set_dragging_task_id=signals.dragging_task_id.write_only() />
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
            // Status columns layout - 3 columns without Backlog
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                {render_status_column(TaskStatus::Todo)}
                {render_status_column(TaskStatus::InProgress)}
                {render_status_column(TaskStatus::Completed)}
            </div>
        </div>
    }
}
