use diesel::{prelude::*};
use super::schema::{layout_tab, user_tab};

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
