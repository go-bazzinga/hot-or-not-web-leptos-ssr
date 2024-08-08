use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
};
use axum::{routing::get, Router};
use hot_or_not_web_leptos_ssr::fallback::file_and_error_handler;
use hot_or_not_web_leptos_ssr::{app::App, init::AppStateBuilder, state::server::AppState};
use leptos::{get_configuration, logging::log, provide_context};
use leptos_axum::handle_server_fns_with_context;
use leptos_axum::{generate_route_list, LeptosRoutes};

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        move || {
            provide_context(app_state.canisters.clone());
            #[cfg(feature = "backend-admin")]
            provide_context(app_state.admin_canisters.clone());
            #[cfg(feature = "cloudflare")]
            provide_context(app_state.cloudflare.clone());
            provide_context(app_state.kv.clone());
            provide_context(app_state.cookie_key.clone());
            #[cfg(feature = "oauth-ssr")]
            provide_context(app_state.google_oauth.clone());

            #[cfg(not(clippy))]
            #[cfg(feature = "ga4")]
            provide_context(app_state.grpc_offchain_channel.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.canisters.clone());
            #[cfg(feature = "backend-admin")]
            provide_context(app_state.admin_canisters.clone());
            #[cfg(feature = "cloudflare")]
            provide_context(app_state.cloudflare.clone());
            provide_context(app_state.kv.clone());
            provide_context(app_state.cookie_key.clone());
            #[cfg(feature = "oauth-ssr")]
            provide_context(app_state.google_oauth.clone());

            #[cfg(not(clippy))]
            #[cfg(feature = "ga4")]
            provide_context(app_state.grpc_offchain_channel.clone());
        },
        App,
    );
    handler(req).await.into_response()
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
    dotenv::dotenv().ok();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let res = AppStateBuilder::new(leptos_options, routes.clone())
        .build()
        .await;
    let terminate = {
        use tokio::signal;

        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            use tokio::signal;
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        async {
            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }
            log::info!("stopping...");

            #[cfg(feature = "local-bin")]
            std::mem::drop(res.containers);
        }
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(res.app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(terminate)
        .await
        .unwrap();
}
