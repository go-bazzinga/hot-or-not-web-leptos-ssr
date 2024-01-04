#[cfg(feature = "ssr")]
mod handlers {
    use axum::{
        body::Body as AxumBody,
        extract::{Path, RawQuery, State},
        http::{header::HeaderMap, Request},
        response::{IntoResponse, Response},
    };
    use leptos::provide_context;
    use leptos_axum::handle_server_fns_with_context;
    use leptos_ssr::{app::App, state::server::AppState};

    pub async fn server_fn_handler(
        State(app_state): State<AppState>,
        path: Path<String>,
        headers: HeaderMap,
        raw_query: RawQuery,
        request: Request<AxumBody>,
    ) -> impl IntoResponse {
        handle_server_fns_with_context(
            path,
            headers,
            raw_query,
            move || {
                provide_context(app_state.canisters.clone());
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
            },
            App,
        );
        handler(req).await.into_response()
    }
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        routing::{get, post},
        Router,
    };
    use handlers::*;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_ssr::app::*;
    use leptos_ssr::fileserv::file_and_error_handler;
    use leptos_ssr::state::canisters::Canisters;
    use leptos_ssr::state::server::AppState;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        canisters: Canisters::default(),
        routes: routes.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
