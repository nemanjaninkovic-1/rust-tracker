use crate::{
    api,
    components::{TaskForm, TaskList},
};
use common::Task;
use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let (tasks, set_tasks) = create_signal(Vec::<Task>::new());
    let (is_loading, set_is_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    let load_tasks = create_action(move |_: &()| async move { api::fetch_tasks(None).await });

    let refresh_tasks = move || {
        set_is_loading.set(true);
        set_error.set(None);
        load_tasks.dispatch(());
    };

    // Load tasks on mount
    create_effect(move |_| {
        load_tasks.dispatch(());
    });

    // Handle task loading results
    create_effect(move |_| {
        if let Some(result) = load_tasks.value().get() {
            set_is_loading.set(false);
            match result {
                Ok(loaded_tasks) => {
                    set_tasks.set(loaded_tasks);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
        }
    });

    view! {
        <div class="max-w-4xl mx-auto">
            <div class="mb-8">
                <h1 class="text-3xl font-bold text-gray-900 mb-2">"Task Management"</h1>
                <p class="text-gray-600">"Organize your tasks and boost your productivity."</p>
            </div>

            <TaskForm on_submit=refresh_tasks />

            {move || {
                if is_loading.get() {
                    view! {
                        <div class="text-center py-12">
                            <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"></div>
                            <p class="mt-2 text-gray-600">"Loading tasks..."</p>
                        </div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class="bg-red-50 border border-red-200 rounded-lg p-4 mb-6">
                            <div class="flex items-center">
                                <div class="text-red-400 mr-3">
                                    "⚠️"
                                </div>
                                <div>
                                    <h3 class="text-sm font-medium text-red-800">"Error loading tasks"</h3>
                                    <p class="text-sm text-red-700 mt-1">{err}</p>
                                </div>
                            </div>
                            <button
                                on:click=move |_| refresh_tasks()
                                class="mt-3 bg-red-100 text-red-800 px-3 py-1 rounded text-sm hover:bg-red-200 transition-colors"
                            >
                                "Retry"
                            </button>
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <TaskList tasks=tasks on_task_change=refresh_tasks />
                    }.into_view()
                }
            }}
        </div>
    }
}
