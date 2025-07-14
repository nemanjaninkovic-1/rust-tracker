use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white shadow-sm border-b">
            <div class="container mx-auto px-4 py-4">
                <div class="flex items-center justify-between">
                    <div class="flex items-center space-x-3">
                        <div class="w-8 h-8 bg-blue-600 rounded-lg flex items-center justify-center">
                            <span class="text-white font-bold text-sm">"RT"</span>
                        </div>
                        <h1 class="text-2xl font-bold text-gray-800">"RustTracker"</h1>
                    </div>
                    <nav class="flex items-center space-x-6">
                        <a href="/" class="text-gray-600 hover:text-blue-600 transition-colors">
                            "Tasks"
                        </a>
                        <div class="flex items-center space-x-2 text-sm text-gray-500">
                            <span class="w-2 h-2 bg-green-500 rounded-full"></span>
                            <span>"Online"</span>
                        </div>
                    </nav>
                </div>
            </div>
        </header>
    }
}
