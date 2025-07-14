use common::{CreateTaskRequest, Task, TaskFilter, UpdateTaskRequest};
use gloo_net::http::Request;
use uuid::Uuid;

// API base URL - function to get it dynamically
fn api_base() -> String {
    // In a real app, this would come from environment variables or build-time configuration
    // For now, we'll use a hardcoded value that matches our backend
    "http://localhost:8080/api".to_string()
}

pub async fn fetch_tasks(filter: Option<TaskFilter>) -> Result<Vec<Task>, String> {
    let mut url = format!("{}/tasks", api_base());

    if let Some(filter) = filter {
        let mut params = Vec::new();

        if let Some(status) = filter.status {
            params.push(format!("status={status:?}"));
        }

        if let Some(category) = filter.category {
            params.push(format!("category={category:?}"));
        }

        if let Some(due_before) = filter.due_before {
            params.push(format!("due_before={}", due_before.to_rfc3339()));
        }

        if let Some(due_after) = filter.due_after {
            params.push(format!("due_after={}", due_after.to_rfc3339()));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }
    }

    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    response
        .json::<Vec<Task>>()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn create_task(request: CreateTaskRequest) -> Result<Task, String> {
    let response = Request::post(&format!("{}/tasks", api_base()))
        .json(&request)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    response
        .json::<Task>()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn update_task(id: Uuid, request: UpdateTaskRequest) -> Result<Task, String> {
    let response = Request::put(&format!("{}/tasks/{id}", api_base()))
        .json(&request)
        .map_err(|e| format!("Failed to serialize request: {e}"))?
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    response
        .json::<Task>()
        .await
        .map_err(|e| format!("Failed to parse response: {e}"))
}

pub async fn delete_task(id: Uuid) -> Result<(), String> {
    let response = Request::delete(&format!("{}/tasks/{id}", api_base()))
        .send()
        .await
        .map_err(|e| format!("Request failed: {e}"))?;

    if !response.ok() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    Ok(())
}
