use axum::{
    extract::{Query, State}, Form, Json
};
use crate::{
    db::UserModel, error::AppError, AppState
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct GetUserByUsernameRequest {
    username: String,
}

pub async fn get_user_by_username(
    State(state): State<AppState>,
    Query(params): Query<GetUserByUsernameRequest>,
) -> Result<axum::Json<UserModel>, AppError> {
    let user = state.db_client.get_user_by_username(&params.username).await?;

    tracing::debug!("{:?}", params);

    Ok(Json(user))
}