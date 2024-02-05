use diesel::prelude::*;
use diesel_async::{
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::Pool,
    },
    RunQueryDsl,
};
use crate::{
    schema::user_tab,
    error::AppError,
};

const DATABASE_URL : &str = "mysql://malfple_test:Password1@127.0.0.1:3306/klb";

#[derive(Clone)]
pub struct DB {
    pub pool: Pool<diesel_async::AsyncMysqlConnection>,
}

impl DB {
    pub fn new() -> DB {
        let config = AsyncDieselConnectionManager::<diesel_async::AsyncMysqlConnection>::new(DATABASE_URL);
        let pool = Pool::builder(config).build().unwrap();

        DB {
            pool,
        }
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<UserModel, AppError> {
        let mut conn = self.pool.get().await?;

        let result = user_tab::table
            .filter(user_tab::username.eq(username))
            .select(UserModel::as_select())
            .first(&mut conn)
            .await?;
        
        return Ok(result)
    }
}

#[derive(Queryable, Selectable, serde::Serialize)]
#[diesel(table_name = user_tab)]
pub struct UserModel {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub layout_data: String,
    pub time_created: i64,
    pub time_modified: i64,
}