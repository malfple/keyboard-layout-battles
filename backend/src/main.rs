use std::{sync::Arc, time::Duration};
use axum::{
    extract::MatchedPath, http::{Request, Response}, middleware::{from_fn, from_fn_with_state}, routing::{get, post}, Router
};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tower_http::trace::TraceLayer;

pub mod auth;
pub mod service;
pub mod db;
pub mod error;
pub mod settings;
pub mod words;
pub mod middleware;

#[derive(Clone)]
pub struct AppState {
    pub db_client: Arc<db::DBClient>,
    pub settings: Arc<settings::AppSettings>,
    pub wordlist: Arc<words::Wordlist>
}

#[tokio::main]
async fn main() {
    let settings = settings::AppSettings::new().expect("settings cannot be initialized");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "backend=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = AppState{
        db_client: Arc::new(db::DBClient::new(&settings)),
        settings: Arc::new(settings),
        wordlist: Arc::new(words::Wordlist::new()),
    };

    let app = root_router(&state)
        .layer(TraceLayer::new_for_http()
            .make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                tracing::info_span!(
                    "http_request_logger",
                    method = ?request.method(),
                    matched_path
                )
            }).on_request(|request: &Request<_>, _span: &Span| {
                tracing::info!("request_data: {:?}", request)
            }).on_response(|response: &Response<_>, latency: Duration, _span: &Span| {
                tracing::info!("response_data: {:?} | latency: {:?}", response, latency)
            }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn root_router(state: &AppState) -> Router<AppState> {
    let app = Router::new()
        .route("/", get(service::ping))
        .route("/user/:username", get(service::user::get_user_by_username))
        .route("/login", post(service::login))
        .route("/register", post(service::register))
        .route("/layouts", get(service::layout::get_layout_list))
        .route("/layout/:id", get(service::layout::get_layout))
        .merge(battle_router(&state));

    app
}

fn battle_router(state: &AppState) -> Router<AppState> {
    let app = Router::new()
        .route("/battle", post(service::battle::create_battle))
        .layer(from_fn_with_state(state.clone(), middleware::relaxed_auth_middleware));

    app
}
