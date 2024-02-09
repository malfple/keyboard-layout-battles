use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use diesel_async::pooled_connection::deadpool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("connection pool failure")]
    DeadPool(#[from] deadpool::PoolError),
    #[error("database failure")]
    DieselResult(#[from] diesel::result::Error),
    #[error("bcrypt error")]
    BcryptError(#[from] bcrypt::BcryptError),
}

#[derive(Debug, serde::Serialize, Clone)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("error_response: {:?}", self);

        if let AppError::DieselResult(err) = &self {
            if *err == diesel::result::Error::NotFound {
                return (
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse{
                        error: "not_found".into(),
                    }),
                ).into_response()
            }
        }

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse{
                error: "internal_error".into(),
            }),
        ).into_response()
    }
}
