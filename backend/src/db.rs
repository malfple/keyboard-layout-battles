use diesel::prelude::*;
use diesel_async::{
    pooled_connection::{
        deadpool::Pool, AsyncDieselConnectionManager
    },
    RunQueryDsl,
};
use crate::{
    error::AppError, schema::user_tab, settings::AppSettings
};

pub struct DBClient {
    pub pool: Pool<diesel_async::AsyncMysqlConnection>,
}

impl DBClient {
    pub fn new(settings: &AppSettings) -> DBClient {
        let config = AsyncDieselConnectionManager::<diesel_async::AsyncMysqlConnection>::new(&settings.database.url);
        let pool = Pool::builder(config).build().unwrap();

        DBClient {
            pool,
        }
    }

    pub async fn create_user(&self, username: &str, hashed_password: &str) -> Result<usize, AppError> {
        let mut conn = self.pool.get().await?;

        let result = diesel::insert_into(user_tab::table)
            .values((
                user_tab::username.eq(username),
                user_tab::password.eq(hashed_password)))
            .execute(&mut conn)
            .await?;

        Ok(result)
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<UserModel, AppError> {
        let mut conn = self.pool.get().await?;

        let result = user_tab::table
            .filter(user_tab::username.eq(username))
            .select(UserModel::as_select())
            .first(&mut conn)
            .await?;
        
        Ok(result)
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