use axum::{
    extract::{Path, State}, Form, Json
};
use crate::{
    db::model::UserModel, error::AppError, AppState
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct GetUserByUsernameResponse {
    user: UserModel,
}

pub async fn get_user_by_username(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<GetUserByUsernameResponse>, AppError> {
    let user = state.db_client.get_user_by_username(&username).await?;

    tracing::debug!("{:?}", username);

    Ok(Json(GetUserByUsernameResponse{
        user,
    }))
}