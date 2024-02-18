use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use diesel_async::pooled_connection::deadpool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // general errors
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),
    #[error("Unauthorized")]
    Unauthorized,
    // self defined errors
    #[error("Something is wrong with the layout format or it is invalid: {0}")]
    LayoutFormat(String),
    #[error("cannot have a battle with 2 identical layouts")]
    BattleIdenticalLayout,
    #[error("battle not found")]
    BattleNotFound,
    #[error("Not enough active layouts to start battle")]
    NotEnoughLayoutsForBattle,
    // Library Errors
    #[error("connection pool failure")]
    DeadPool(#[from] deadpool::PoolError),
    #[error("database failure")]
    DieselResult(#[from] diesel::result::Error),
    #[error("bcrypt error")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("jsonwebtoken error")]
    JsonWebToken(#[from] jsonwebtoken::errors::Error),
    #[error("serde json error")]
    SerdeJsonError(#[from] serde_json::error::Error),
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct ErrorResponse {
    error: String,
    error_message: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("error_response: {:?}", self);

        match self {
            AppError::BadRequest(_) => bad_request_error_response(self.to_string()),
            AppError::InvalidParameter(_) => bad_request_error_response(self.to_string()),
            AppError::Unauthorized => unauthorized_error_response(),
            AppError::LayoutFormat(_) => bad_request_error_response(self.to_string()),
            AppError::BattleIdenticalLayout => bad_request_error_response(self.to_string()),
            AppError::BattleNotFound => not_found_error_response(),
            AppError::NotEnoughLayoutsForBattle => bad_request_error_response(self.to_string()),
            AppError::DeadPool(_) => internal_server_error_response(),
            AppError::DieselResult(err) => {
                if err == diesel::result::Error::NotFound {
                    not_found_error_response()
                } else {
                    internal_server_error_response()
                }
            },
            AppError::BcryptError(_) => internal_server_error_response(),
            AppError::JsonWebToken(_) => bad_request_error_response(self.to_string()),
            AppError::SerdeJsonError(_) => internal_server_error_response(),
        }
    }
}

pub fn bad_request_error_response(msg: String) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse{
            error: String::from("bad_request"),
            error_message: Some(msg),
        })
    ).into_response()
}

fn unauthorized_error_response() -> Response {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse{
            error: String::from("unauthorized"),
            error_message: None,
        })
    ).into_response()
}

fn not_found_error_response() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(ErrorResponse{
            error: String::from("not_found"),
            error_message: None,
        }),
    ).into_response()
}

fn internal_server_error_response() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse{
            error: String::from("internal_error"),
            error_message: None,
        }),
    ).into_response()
}
