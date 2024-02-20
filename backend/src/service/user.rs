use axum::{
    extract::{Path, State}, Form, Json
};
use crate::{
    auth, db::model::UserModel, error::AppError, AppState
};
use serde::{Deserialize, Serialize};

const MAX_USERNAME_LEN: usize = 32;
const MIN_PASSWORD_LEN: usize = 5;
const MAX_PASSWORD_LEN: usize = 64;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    access_token: Option<String>,
}

/// Simple Login API
pub async fn login(
    State(state): State<AppState>,
    Form(req): Form<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state.db_client.get_user_by_username(&req.username).await?;

    let valid = bcrypt::verify(&req.password, &user.password)?;

    if valid {
        let token = auth::generate_access_token(user.id, &user.username, &state.settings.general.token_secret);

        Ok(Json(LoginResponse{
            message: "login success".into(),
            access_token: Some(token),
        }))
    } else {
        Ok(Json(LoginResponse{
            message: "wrong credentials".into(),
            access_token: None,
        }))
    }
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest{
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse{
    message: String,
}

/// Simple register API
pub async fn register(
    State(state): State<AppState>,
    Form(req): Form<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    if req.username.len() < 1 || req.username.len() > MAX_USERNAME_LEN {
        return Err(AppError::BadRequest(format!("username must have 1 to {} characters", MAX_USERNAME_LEN)));
    }

    if req.password.len() < MIN_PASSWORD_LEN || req.password.len() > MAX_PASSWORD_LEN {
        return Err(AppError::BadRequest(format!("password must have {} to {} characters", MIN_PASSWORD_LEN, MAX_PASSWORD_LEN)));
    }

    let hashed_password = auth::hash_password(&req.password)?;

    _ = state.db_client.create_user(req.username, hashed_password).await?;

    Ok(Json(RegisterResponse{
        message: "register success".into(),
    }))
}

#[derive(Debug, Serialize)]
pub struct GetUserByUsernameResponse {
    user: UserModel,
}

/// Get user API
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