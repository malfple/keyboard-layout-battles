use axum::{
    debug_handler, extract::State, Extension, Form, Json
};
use crate::{
    auth, error::AppError, middleware::Identity, AppState
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct CreateBattleRequest {
    base_layout_data: String,
    is_personal: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateBattleResponse {

}

/// Creates battle API
/// 
/// Will not use layout_data from typer's data but will use it from base_layout_data.
pub async fn create_battle(
    State(state): State<AppState>,
    Extension(identity): Extension<Option<Identity>>,
    Json(req): Json<CreateBattleRequest>,
) {
    tracing::debug!("req {:?}", req);
    tracing::debug!("ext {:?}", identity);
}
