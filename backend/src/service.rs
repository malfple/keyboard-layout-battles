use axum::{
    extract::{Query, State}, response
};
use crate::{
    db::UserModel, error::AppError, AppState
};

pub async fn ping(State(_state): State<AppState>) -> &'static str {
    "Hello, World!"
}

#[derive(Debug, serde::Deserialize)]
pub struct GetUserByUsernameRequest {
    username: String,
}

pub async fn get_user_by_username(
    State(state): State<AppState>,
    Query(params): Query<GetUserByUsernameRequest>,
) -> Result<axum::Json<UserModel>, AppError> {
    let user = state.db_client.get_user_by_username(&params.username).await?;

    print!("{:?}", params);

    Ok(response::Json(user))
}
