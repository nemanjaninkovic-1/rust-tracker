use common::{TaskPriority, TaskStatus, UpdateTaskRequest};
use leptos::*;

pub struct TaskListSignals {
    pub filter_priority: RwSignal<Option<TaskPriority>>,
    pub drag_over_status: RwSignal<Option<TaskStatus>>,
    pub dragging_task_id: RwSignal<Option<uuid::Uuid>>,
}

impl Default for TaskListSignals {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskListSignals {
    pub fn new() -> Self {
        Self {
            filter_priority: create_rw_signal(None::<TaskPriority>),
            drag_over_status: create_rw_signal(None::<TaskStatus>),
            dragging_task_id: create_rw_signal(None::<uuid::Uuid>),
        }
    }
}

type UpdateTaskAction = Action<(uuid::Uuid, UpdateTaskRequest), Result<common::Task, String>>;

pub fn use_update_task_action() -> (UpdateTaskAction, Box<dyn Fn()>) {
    let update_task_action = create_action(|(id, request): &(uuid::Uuid, UpdateTaskRequest)| {
        let id = *id;
        let request = request.clone();
        async move {
            leptos::logging::log!("Updating task {} with request: {:?}", id, request);
            let result = crate::api::update_task(id, request).await;
            leptos::logging::log!("Update result: {:?}", result);
            result
        }
    });
    let on_success = move || {
        if let Some(result) = update_task_action.value().get() {
            match result {
                Ok(_) => {
                    leptos::logging::log!("Task update successful");
                }
                Err(e) => {
                    leptos::logging::log!("Task update failed: {:?}", e);
                }
            }
        }
    };
    (update_task_action, Box::new(on_success))
}
