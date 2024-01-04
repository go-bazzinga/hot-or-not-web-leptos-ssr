pub mod canisters;

#[cfg(feature = "ssr")]
pub mod server {
    use super::canisters::Canisters;
    use axum::extract::FromRef;
    use leptos::LeptosOptions;
    use leptos_router::RouteListing;

    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub canisters: Canisters,
        pub routes: Vec<RouteListing>,
    }
}
