use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod api;
mod components;
mod pages;

#[cfg(test)]
mod tests;

use components::*;
use pages::*;

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Title text="RustTracker - Task Management"/>

        <Router>
            <main class="min-h-screen bg-gray-50">
                <Header/>
                <div class="container mx-auto px-4 py-8">
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </div>
            </main>
        </Router>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="text-center py-16">
            <h1 class="text-4xl font-bold text-gray-800 mb-4">"Page Not Found"</h1>
            <p class="text-gray-600 mb-8">"The page you're looking for doesn't exist."</p>
            <a href="/" class="bg-blue-600 text-white px-6 py-3 rounded-lg hover:bg-blue-700 transition-colors">
                "Go Home"
            </a>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn start() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    mount_to_body(|| view! { <App/> })
}
