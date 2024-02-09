use core::time;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Query, State}, response, Form, Json
};
use crate::{
    db::UserModel, error::AppError, AppState
};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};

const BCRYPT_COST: u32 = 8;
const TOKEN_ISSUER: &str = "klb-backend";
const TOKEN_EXP: u64 = time::Duration::from_secs(7 * 24 * 60 * 60).as_millis() as u64;

pub async fn ping(State(_state): State<AppState>) -> &'static str {
    "Hello, World!"
}

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

    Ok(response::Json(user))
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: String,
    access_token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Form(req): Form<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = state.db_client.get_user_by_username(&req.username).await?;

    let valid = bcrypt::verify(&req.password, &user.password)?;

    if valid {
        let token = generate_access_token(&user.username, &state.settings.general.token_secret);

        Ok(Json(LoginResponse{
            message: "login success".into(),
            access_token: token,
        }))
    } else {
        Ok(Json(LoginResponse{
            message: "wrong credentials".into(),
            access_token: "".into(),
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
    let hashed_password = bcrypt::hash(&req.password, BCRYPT_COST)?;

    _ = state.db_client.create_user(&req.username, &hashed_password).await?;

    Ok(Json(RegisterResponse{
        message: "register success".into(),
    }))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: u64,
    iss: String,
    sub: String,
}

fn generate_access_token(username: &str, token_secret: &str) -> String {
    let claims = Claims{
        exp: SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_millis() as u64 + TOKEN_EXP,
        iss: TOKEN_ISSUER.into(),
        sub: username.into(),
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(token_secret.as_ref())).expect("should be no error in generating jwt");

    token
}
