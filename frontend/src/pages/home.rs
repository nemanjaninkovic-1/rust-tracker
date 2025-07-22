use crate::{
    api,
    components::{Modal, Navbar, TaskForm, TaskList},
};
use common::{Task, TaskPriority};
use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn HomePage() -> impl IntoView {
    let (tasks, set_tasks) = create_signal(Vec::<Task>::new());
    let (is_loading, set_is_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);
    let (show_modal, set_show_modal) = create_signal(false);
    let (filter_priority, set_filter_priority) = create_signal(None::<TaskPriority>);
    let (refresh_debounce, set_refresh_debounce) = create_signal(false);

    let load_tasks = create_action(move |_: &()| async move { api::fetch_tasks(None).await });

    let refresh_tasks = move || {
        if !refresh_debounce.get() {
            set_refresh_debounce.set(true);
            set_is_loading.set(true);
            set_error.set(None);
            load_tasks.dispatch(());

            // Reset debounce after a delay
            set_timeout(
                move || {
                    set_refresh_debounce.set(false);
                },
                std::time::Duration::from_millis(500),
            );
        }
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
        <div class="min-h-screen bg-gray-900">
            <Navbar
                filter_priority=filter_priority
                set_filter_priority=set_filter_priority
                on_add_task=move || set_show_modal.set(true)
            />

            <div class="max-w-7xl mx-auto px-6">
                {move || {
                    if is_loading.get() {
                        view! {
                            <div class="text-center py-12">
                                <div class="inline-block animate-spin rounded-full h-8 w-8 border-b-2 border-blue-400"></div>
                                <p class="mt-2 text-gray-300">"Loading tasks..."</p>
                            </div>
                        }.into_view()
                    } else if let Some(err) = error.get() {
                        view! {
                            <div class="bg-red-900 border border-red-700 rounded-lg p-4 mb-6">
                                <div class="flex items-center">
                                    <div class="text-red-400 mr-3">
                                        "✗"
                                    </div>
                                    <div>
                                        <h3 class="text-sm font-medium text-red-200">"Error loading tasks"</h3>
                                        <p class="text-sm text-red-300 mt-1">{err}</p>
                                    </div>
                                </div>
                                <button
                                    on:click=move |_| refresh_tasks()
                                    class="mt-3 bg-red-800 text-red-200 px-3 py-1 rounded text-sm hover:bg-red-700 transition-colors"
                                >
                                    "Retry"
                                </button>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <TaskList tasks=tasks set_tasks=set_tasks _refresh_tasks=refresh_tasks filter_priority=filter_priority />
                        }.into_view()
                    }
                }}

                <Modal
                    show=show_modal
                    on_close=move || set_show_modal.set(false)
                    title="Add New Task".to_string()
                >
                    <TaskForm
                        on_submit=refresh_tasks
                        on_close=Some(move || set_show_modal.set(false))
                    />
                </Modal>
            </div>
        </div>
    }
}
