use crate::api;
use common::{Task, TaskStatus, UpdateTaskRequest};
use leptos::*;

#[component]
pub fn TaskItem<F>(task: Task, on_update: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let (is_updating, set_is_updating) = create_signal(false);
    let (is_deleting, set_is_deleting) = create_signal(false);

    let update_task = create_action(move |(id, request): &(uuid::Uuid, UpdateTaskRequest)| {
        let id = *id;
        let request = request.clone();
        async move { api::update_task(id, request).await }
    });

    let delete_task = create_action(move |id: &uuid::Uuid| {
        let id = *id;
        async move { api::delete_task(id).await }
    });

    let toggle_status = move |_| {
        let new_status = match task.status {
            TaskStatus::Todo => TaskStatus::InProgress,
            TaskStatus::InProgress => TaskStatus::Completed,
            TaskStatus::Completed => TaskStatus::Todo,
        };

        let request = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(new_status),
            category: None,
            due_date: None,
        };

        set_is_updating.set(true);
        update_task.dispatch((task.id, request));
    };

    let delete_task_handler = move |_| {
        set_is_deleting.set(true);
        delete_task.dispatch(task.id);
    };

    create_effect(move |_| {
        if let Some(result) = update_task.value().get() {
            set_is_updating.set(false);
            if result.is_ok() {
                on_update();
            }
        }
    });

    create_effect(move |_| {
        if let Some(result) = delete_task.value().get() {
            set_is_deleting.set(false);
            if result.is_ok() {
                on_update();
            }
        }
    });

    let status_color = match task.status {
        TaskStatus::Todo => "bg-gray-100 text-gray-800",
        TaskStatus::InProgress => "bg-blue-100 text-blue-800",
        TaskStatus::Completed => "bg-green-100 text-green-800",
    };

    let category_color = match task.category {
        common::TaskCategory::Work => "bg-purple-100 text-purple-800",
        common::TaskCategory::Personal => "bg-indigo-100 text-indigo-800",
        common::TaskCategory::Shopping => "bg-pink-100 text-pink-800",
        common::TaskCategory::Health => "bg-red-100 text-red-800",
        common::TaskCategory::Other => "bg-gray-100 text-gray-800",
    };

    view! {
        <div class="bg-white rounded-lg shadow-sm border p-3 hover:shadow-md transition-shadow">
            <div class="flex items-start justify-between">
                <div class="flex-1">
                    <div class="flex items-center space-x-2 mb-1">
                        <h3 class="font-medium text-gray-900">{&task.title}</h3>
                        <span class={format!("px-2 py-1 text-xs font-medium rounded-full {status_color}")}>
                            {format!("{:?}", task.status)}
                        </span>
                        <span class={format!("px-2 py-1 text-xs font-medium rounded-full {category_color}")}>
                            {format!("{:?}", task.category)}
                        </span>
                    </div>

                    {task.description.as_ref().map(|desc| view! {
                        <p class="text-sm text-gray-600 mb-1">{desc}</p>
                    })}

                    <div class="flex items-center space-x-4 text-xs text-gray-500">
                        <span>
                            "Created: " {task.created_at.format("%m/%d/%Y").to_string()}
                        </span>
                        {task.due_date.map(|due| view! {
                            <span>
                                "Due: " {due.format("%m/%d/%Y").to_string()}
                            </span>
                        })}
                    </div>
                </div>

                <div class="flex items-center space-x-2 ml-4">
                    <button
                        on:click=toggle_status
                        disabled=move || is_updating.get()
                        class="px-3 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded hover:bg-blue-200 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-1 disabled:opacity-50 transition-colors"
                    >
                        {move || if is_updating.get() { "..." } else { "Toggle" }}
                    </button>

                    <button
                        on:click=delete_task_handler
                        disabled=move || is_deleting.get()
                        class="px-3 py-1 text-xs font-medium bg-red-100 text-red-800 rounded hover:bg-red-200 focus:outline-none focus:ring-2 focus:ring-red-500 focus:ring-offset-1 disabled:opacity-50 transition-colors"
                    >
                        {move || if is_deleting.get() { "..." } else { "Delete" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
