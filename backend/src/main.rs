use axum::{
    routing::get,
    Router,
};

pub mod service;
pub mod db;
pub mod schema;
pub mod error;

#[derive(Clone)]
pub struct AppState {
    pub db: db::DB,
}

#[tokio::main]
async fn main() {
    let state = AppState{
        db: db::DB::new(),
    };

    let app = root_router()
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn root_router() -> Router<AppState> {
    let app = Router::new()
        .route("/", get(service::ping))
        .route("/user", get(service::get_user_by_username));

    app
}
