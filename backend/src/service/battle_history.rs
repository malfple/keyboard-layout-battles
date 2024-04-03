use axum::{extract::{Path, Query, State}, Json};
use serde::{Deserialize, Serialize};

use crate::{db::model::{BattleHistoryLiteModel, BattleHistoryModel}, error::AppError, AppState};

const MAX_LIMIT: i64 = 100;

#[derive(Debug, Deserialize)]
pub struct GetBattleHistoryListRequest {
    limit: i64,
    layout_id: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct GetBattleHistoryListResponse {
    battles: Vec<BattleHistoryLiteModel>,
}

/// Get Battle History List API
pub async fn get_battle_history_list(
    State(state): State<AppState>,
    Query(req): Query<GetBattleHistoryListRequest>,
) -> Result<Json<GetBattleHistoryListResponse>, AppError> {
    if req.limit > MAX_LIMIT {
        return Err(AppError::InvalidParameter(String::from("limit")));
    }

    let battles = if let Some(layout_id) = req.layout_id {
        state.db_client.get_battle_history_lite_list_with_layout_id(layout_id, req.limit).await?
    } else {
        state.db_client.get_battle_history_lite_list(req.limit).await?
    };

    Ok(Json(GetBattleHistoryListResponse{
        battles,
    }))
}

#[derive(Debug, Serialize)]
pub struct GetBattleHistoryResponse {
    battle: BattleHistoryModel,
}

/// Get Battle History API
pub async fn get_battle_history(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<GetBattleHistoryResponse>, AppError> {
    let battle = state.db_client.get_battle_history_by_id(id).await?;

    Ok(Json(GetBattleHistoryResponse{
        battle,
    }))
}