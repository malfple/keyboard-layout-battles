use axum::{
    extract::{Path, State}, Json
};
use crate::{
    db::model::{LayoutModel, LayoutLiteModel}, error::AppError, AppState
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct GetLayoutListResponse {
    layouts: Vec<LayoutLiteModel>,
}

/// Get Layout list API
pub async fn get_layout_list(
    State(state): State<AppState>,
) -> Result<Json<GetLayoutListResponse>, AppError> {
    let layouts = state.db_client.get_layout_lite_list().await?;

    Ok(Json(GetLayoutListResponse{
        layouts,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetLayoutResponse {
    layout: LayoutModel,
}

/// Get layout API
pub async fn get_layout(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<GetLayoutResponse>, AppError> {
    let layout = state.db_client.get_layout_by_id(id).await?;

    Ok(Json(GetLayoutResponse{
        layout,
    }))
}
