use leptos::*;

#[component]
pub fn Spinner() -> impl IntoView {
    view! {
        <div class="animate-spin border-solid rounded-full border-t-transparent border-white border-8 h-32 w-32"></div>
    }
}
