use axum::{response::{IntoResponse, Response}, http::StatusCode};
use diesel_async::pooled_connection::deadpool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("connection pool failure")]
    DeadPool(#[from] deadpool::PoolError),
    #[error("database failure")]
    DieselResult(#[from] diesel::result::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {:?}", self),
        ).into_response()
    }
}
