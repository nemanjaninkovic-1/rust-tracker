use leptos::*;

#[component]
#[allow(non_snake_case)]
pub fn Modal<F>(
    show: ReadSignal<bool>,
    on_close: F,
    title: String,
    children: Children,
) -> impl IntoView
where
    F: Fn() + 'static + Copy,
{
    view! {
        <div
            class=move || if show.get() { "fixed inset-0 z-50 overflow-y-auto" } else { "hidden" }
        >
            <div class="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
                // Background overlay
                <div
                    class="fixed inset-0 bg-gray-900 bg-opacity-75 transition-opacity"
                    on:click=move |_| on_close()
                ></div>

                // Modal content
                <div class="inline-block align-bottom bg-gray-800 dark:bg-gray-800 rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full">
                    <div class="bg-gray-800 dark:bg-gray-800 px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
                        <div class="flex justify-between items-center mb-4">
                            <h3 class="text-lg leading-6 font-medium text-gray-100 dark:text-gray-100">
                                {title}
                            </h3>
                            <button
                                on:click=move |_| on_close()
                                class="text-gray-400 hover:text-gray-300 dark:hover:text-gray-300 focus:outline-none focus:text-gray-300 dark:focus:text-gray-300 transition ease-in-out duration-150"
                            >
                                <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                </svg>
                            </button>
                        </div>
                        <div>
                            {children()}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
