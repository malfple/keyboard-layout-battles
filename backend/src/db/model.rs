use diesel::{prelude::*};
use super::schema::{battle_history_tab, layout_tab, user_tab};

#[derive(Debug, Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = battle_history_tab)]
pub struct BattleHistoryModel {
    pub id: u64,
    pub layout_id_1: u64,
    pub layout_id_2: u64,
    pub layout_data_typer: String,
    pub user_id_typer: u64,
    pub content_data: serde_json::Value,
    pub layout_1_rating: i32,
    pub layout_2_rating: i32,
    pub rating_1_gain: i32,
    pub rating_2_gain: i32,
    pub result_data: serde_json::Value,
    pub is_personal: bool,
    pub time_created: i64,
}

#[derive(Debug, Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = layout_tab)]
pub struct LayoutModel {
    pub id: u64,
    pub name: String,
    pub layout_data: String,
    pub description: Option<String>,
    pub rating: i32,
    pub rating_comfort: i32,
    pub rating_data: Option<serde_json::Value>,
    pub is_active: bool,
    pub time_created: i64,
    pub time_modified: i64,
}

#[derive(Debug, Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = user_tab)]
pub struct UserModel {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub layout_data: String,
    pub time_created: i64,
    pub time_modified: i64,
}
