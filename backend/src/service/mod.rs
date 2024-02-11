use axum::{
    extract::State, Form, Json
};
use crate::{
    auth, error::AppError, AppState
};
use serde::{Serialize, Deserialize};

pub mod user;
pub mod layout;

pub async fn ping(State(_state): State<AppState>) -> &'static str {
    "Hello, World!"
}

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

pub async fn login(
    State(state): State<AppState>,
    Form(req): Form<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state.db_client.get_user_by_username(&req.username).await?;

    let valid = bcrypt::verify(&req.password, &user.password)?;

    if valid {
        let token = auth::generate_access_token(&user.username, &state.settings.general.token_secret);

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

pub async fn register(
    State(state): State<AppState>,
    Form(req): Form<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    let hashed_password = auth::hash_password(&req.password)?;

    _ = state.db_client.create_user(&req.username, &hashed_password).await?;

    Ok(Json(RegisterResponse{
        message: "register success".into(),
    }))
}
