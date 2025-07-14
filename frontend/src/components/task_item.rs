use crate::api;
use common::{Task, TaskStatus, UpdateTaskRequest};
use leptos::*;

#[component]
pub fn TaskItem<F>(
    task: Task,
    on_update: F,
    #[prop(optional)] set_dragging_task_id: Option<WriteSignal<Option<uuid::Uuid>>>,
) -> impl IntoView
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
            TaskStatus::Backlog => TaskStatus::Todo, // Backlog tasks go to Todo when toggled
        };

        let request = UpdateTaskRequest {
            title: None,
            description: None,
            status: Some(new_status),
            priority: None,
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
        TaskStatus::Backlog => "bg-purple-100 text-purple-800",
    };

    let priority_color = match task.priority {
        common::TaskPriority::Low => "bg-gray-100 text-gray-800",
        common::TaskPriority::Medium => "bg-blue-100 text-blue-800",
        common::TaskPriority::High => "bg-yellow-100 text-yellow-800",
        common::TaskPriority::Urgent => "bg-red-100 text-red-800",
    };

    view! {
        <div
            class="bg-white rounded-lg shadow-sm border hover:shadow-md transition-shadow h-32 w-full max-w-md mx-auto flex flex-row cursor-grab active:cursor-grabbing"
            draggable="true"
            on:dragstart=move |_| {
                if let Some(setter) = set_dragging_task_id {
                    setter.set(Some(task.id));
                }
            }
            on:dragend=move |_| {
                if let Some(setter) = set_dragging_task_id {
                    setter.set(None);
                }
            }
        >
            // Left content area with title and description
            <div class="flex-1 p-3 flex flex-col justify-between min-w-0">
                <div>
                    <h3 class="font-medium text-gray-900 text-sm line-clamp-2 leading-tight mb-2">
                        {&task.title}
                    </h3>
                    {task.description.as_ref().map(|desc| view! {
                        <p class="text-xs text-gray-600 line-clamp-2">{desc}</p>
                    })}
                </div>

                {task.due_date.map(|due| view! {
                    <div class="text-xs text-orange-600 font-medium mt-2">
                        "Due: " {due.format("%m/%d/%Y").to_string()}
                    </div>
                })}
            </div>

            // Right sidebar with badges and actions
            <div class="w-24 p-3 flex flex-col justify-between items-center border-l border-gray-100 bg-gray-50">
                <div class="flex flex-col gap-1 items-center">
                    <span class={format!("px-2 py-1 text-xs font-medium rounded-full {status_color}")}>
                        {format!("{:?}", task.status)}
                    </span>
                    <span class={format!("px-2 py-1 text-xs font-medium rounded-full {priority_color}")}>
                        {format!("{:?}", task.priority)}
                    </span>
                </div>

                <div class="flex flex-col gap-1 w-full">
                    <button
                        on:click=toggle_status
                        disabled=move || is_updating.get()
                        class="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded hover:bg-blue-200 focus:outline-none focus:ring-1 focus:ring-blue-500 disabled:opacity-50 transition-colors w-full"
                    >
                        {move || if is_updating.get() { "..." } else { "Toggle" }}
                    </button>

                    <button
                        on:click=delete_task_handler
                        disabled=move || is_deleting.get()
                        class="px-2 py-1 text-xs font-medium bg-red-100 text-red-800 rounded hover:bg-red-200 focus:outline-none focus:ring-1 focus:ring-red-500 disabled:opacity-50 transition-colors w-full"
                    >
                        {move || if is_deleting.get() { "..." } else { "Delete" }}
                    </button>
                </div>
            </div>
        </div>
    }
}
