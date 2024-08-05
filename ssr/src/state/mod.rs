#[cfg(feature = "backend-admin")]
pub mod admin_canisters;
pub mod auth;
pub mod canisters;
pub mod content_seed_client;
pub mod history;
pub mod local_storage;

#[cfg(feature = "ssr")]
pub mod server {
    use crate::auth::server_impl::store::KVStoreImpl;

    use super::canisters::Canisters;
    use axum::extract::FromRef;
    use axum_extra::extract::cookie::Key;
    use leptos::LeptosOptions;
    use leptos_router::RouteListing;
    use tonic::transport::Channel;

    #[derive(FromRef, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub canisters: Canisters<false>,
        #[cfg(feature = "backend-admin")]
        pub admin_canisters: super::admin_canisters::AdminCanisters,
        #[cfg(feature = "cloudflare")]
        pub cloudflare: gob_cloudflare::CloudflareAuth,
        pub kv: KVStoreImpl,
        pub routes: Vec<RouteListing>,
        pub cookie_key: Key,
        #[cfg(feature = "oauth-ssr")]
        pub google_oauth: openidconnect::core::CoreClient,
        #[cfg(feature = "ga4")]
        pub grpc_offchain_channel: Channel,
    }
}
