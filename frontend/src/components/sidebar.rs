use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="w-64 fixed top-0 left-0 h-full bg-gray-800 border-r border-gray-700 flex flex-col z-20">
            <div class="p-4 font-semibold text-lg border-b border-gray-700 text-white">RustTracker</div>
            <nav class="flex-1 p-4 space-y-2">
                <a href="/" class="block px-4 py-2 rounded hover:bg-gray-700 text-gray-200 hover:text-white font-medium transition-colors">Dashboard</a>
                <a href="/tasks" class="block px-4 py-2 rounded hover:bg-gray-700 text-gray-200 hover:text-white font-medium transition-colors">Tasks</a>
                <a href="/kanban" class="block px-4 py-2 rounded hover:bg-gray-700 text-gray-200 hover:text-white font-medium transition-colors">Kanban</a>
                <a href="/reports" class="block px-4 py-2 rounded hover:bg-gray-700 text-gray-200 hover:text-white font-medium transition-colors">Reports</a>
                <a href="/settings" class="block px-4 py-2 rounded hover:bg-gray-700 text-gray-200 hover:text-white font-medium transition-colors">Settings</a>
            </nav>
            <div class="p-4 border-t border-gray-700 text-sm text-gray-400">Sidebar navigation</div>
        </aside>
    }
}
