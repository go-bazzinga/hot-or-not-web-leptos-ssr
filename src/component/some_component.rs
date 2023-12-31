use leptos::*;
use stylers::style;

#[component]
pub fn SomeComponent() -> impl IntoView {
    let styles = style! {
        h2 {
            color: var(--red-8);
            background-color: var(--green-1);
            padding: var(--size-7);
        }
    };

    view! { class=styles, <h2>"Hello World"</h2> }
}
