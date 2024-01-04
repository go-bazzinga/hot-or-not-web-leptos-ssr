pub mod canisters;

#[cfg(feature = "ssr")]
pub mod server {
    use leptos::LeptosOptions;
    use leptos_router::RouteListing;
    use super::canisters::Canisters;
    use axum::extract::FromRef;

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub canisters: Canisters,
        pub routes: Vec<RouteListing>,
    }
}
