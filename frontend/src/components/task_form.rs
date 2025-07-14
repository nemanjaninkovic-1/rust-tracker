use crate::api;
use common::{CreateTaskRequest, TaskCategory};
use leptos::*;

#[component]
pub fn TaskForm<F>(on_submit: F) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    let (title, set_title) = create_signal(String::new());
    let (description, set_description) = create_signal(String::new());
    let (category, set_category) = create_signal(TaskCategory::Other);
    let (is_submitting, set_is_submitting) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let submit_task = create_action(move |request: &CreateTaskRequest| {
        let request = request.clone();
        async move { api::create_task(request).await }
    });

    let on_form_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let title_value = title.get().trim().to_string();
        if title_value.is_empty() {
            set_error.set(Some("Title is required".to_string()));
            return;
        }

        let description_value = description.get();
        let description_value = description_value.trim();
        let description_value = if description_value.is_empty() {
            None
        } else {
            Some(description_value.to_string())
        };

        let request = CreateTaskRequest {
            title: title_value,
            description: description_value,
            category: category.get(),
            due_date: None, // TODO: Add date picker
        };

        set_error.set(None);
        set_is_submitting.set(true);
        submit_task.dispatch(request);
    };

    create_effect(move |_| {
        if let Some(result) = submit_task.value().get() {
            set_is_submitting.set(false);
            match result {
                Ok(_) => {
                    set_title.set(String::new());
                    set_description.set(String::new());
                    set_category.set(TaskCategory::Other);
                    on_submit();
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
        }
    });

    view! {
        <div class="bg-white rounded-lg shadow-sm border p-6 mb-6">
            <h2 class="text-lg font-semibold text-gray-800 mb-4">"Add New Task"</h2>

            <form on:submit=on_form_submit class="space-y-4">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Title"
                    </label>
                    <input
                        type="text"
                        prop:value=title
                        on:input=move |ev| set_title.set(event_target_value(&ev))
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                        placeholder="Enter task title..."
                        required
                    />
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Description"
                    </label>
                    <textarea
                        prop:value=description
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
                        rows="3"
                        placeholder="Enter task description (optional)..."
                    ></textarea>
                </div>

                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-2">
                        "Category"
                    </label>
                    <select
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            let cat = match value.as_str() {
                                "Work" => TaskCategory::Work,
                                "Personal" => TaskCategory::Personal,
                                "Shopping" => TaskCategory::Shopping,
                                "Health" => TaskCategory::Health,
                                _ => TaskCategory::Other,
                            };
                            set_category.set(cat);
                        }
                        class="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                    >
                        <option value="Other">"Other"</option>
                        <option value="Work">"Work"</option>
                        <option value="Personal">"Personal"</option>
                        <option value="Shopping">"Shopping"</option>
                        <option value="Health">"Health"</option>
                    </select>
                </div>

                {move || error.get().map(|err| view! {
                    <div class="text-red-600 text-sm bg-red-50 border border-red-200 rounded-md p-3">
                        {err}
                    </div>
                })}

                <button
                    type="submit"
                    disabled=move || is_submitting.get()
                    class="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                >
                    {move || if is_submitting.get() { "Creating..." } else { "Create Task" }}
                </button>
            </form>
        </div>
    }
}
