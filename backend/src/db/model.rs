use diesel::prelude::*;
use super::schema::*;

#[derive(Debug, Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = battle_history_tab)]
pub struct BattleHistoryModel {
    pub id: u64,
    pub layout_id_1: u64,
    pub layout_id_2: u64,
    pub base_layout_data: String,
    pub user_id_typer: Option<u64>,
    pub layout_1_rating: i32,
    pub layout_2_rating: i32,
    pub rating_1_gain: i32,
    pub rating_2_gain: i32,
    pub result_data: serde_json::Value,
    pub is_personal: bool,
    pub time_created: i64,
}

#[derive(Debug, Queryable, Insertable, serde::Serialize)]
#[diesel(table_name = battle_history_tab)]
pub struct BattleHistoryModelForInsert {
    pub layout_id_1: u64,
    pub layout_id_2: u64,
    pub base_layout_data: String,
    pub user_id_typer: Option<u64>,
    pub layout_1_rating: i32,
    pub layout_2_rating: i32,
    pub rating_1_gain: i32,
    pub rating_2_gain: i32,
    pub result_data: serde_json::Value,
    pub is_personal: bool,
    pub time_created: i64,
}

#[derive(Debug, Queryable, Selectable, Insertable, serde::Serialize)]
#[diesel(table_name = battle_tab)]
pub struct BattleModel {
    pub id: String,
    pub layout_id_1: u64,
    pub layout_id_2: u64,
    pub base_layout_data: String,
    pub user_id_typer: Option<u64>,
    pub content_data: serde_json::Value,
    pub is_personal: bool,
    pub time_created: i64,
    pub time_modified: i64,
}

#[derive(Debug, Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = layout_tab)]
pub struct LayoutModel {
    pub id: u64,
    pub sequence_id: Option<u64>,
    pub name: String,
    pub layout_data: String,
    pub description: Option<String>,
    pub rating: i32,
    pub rating_comfort: i32,
    pub rating_data: Option<serde_json::Value>,
    pub time_created: i64,
    pub time_modified: i64,
}

#[derive(Debug, Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = layout_tab)]
pub struct LayoutLiteModel {
    pub id: u64,
    pub name: String,
    pub layout_data: String,
    pub rating: i32,
    pub rating_comfort: i32,
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
