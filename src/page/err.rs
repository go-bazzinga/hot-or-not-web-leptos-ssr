use leptos::*;
use leptos_router::*;

#[derive(Params, PartialEq)]
struct ServerErrParams {
    err: String,
}

#[component]
pub fn ServerErrorPage() -> impl IntoView {
    let params = use_query::<ServerErrParams>();
    let error = move || {
        params.with(|p| {
            p.as_ref()
                .map(|p| p.err.clone())
                .unwrap_or("Server Error".to_string())
        })
    };
    view! {
        <div>
            <h1>Server Error</h1>
            <h3>{error()}</h3>
        </div>
    }
}
