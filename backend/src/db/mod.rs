use diesel::{prelude::*, result::Error};
use diesel_async::{
    pooled_connection::{
        deadpool::Pool, AsyncDieselConnectionManager
    }, AsyncConnection, RunQueryDsl
};
use crate::{
    error::AppError, settings::AppSettings
};
use schema::*;
use model::*;

pub mod schema;
pub mod model;

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

    pub async fn get_layout_by_id(&self, id: u64) -> Result<LayoutModel, AppError> {
        let mut conn = self.pool.get().await?;

        let result = layout_tab::table
            .filter(layout_tab::id.eq(id))
            .select(LayoutModel::as_select())
            .first(&mut conn)
            .await?;

        Ok(result)
    }

    pub async fn get_layout_lite_list(&self) -> Result<Vec<LayoutLiteModel>, AppError> {
        let mut conn = self.pool.get().await?;

        let result = layout_tab::table
            .select(LayoutLiteModel::as_select())
            .load(&mut conn)
            .await?;

        Ok(result)
    }

    pub async fn create_battle(
        &self,
        battle: BattleModel,
    ) -> Result<usize, AppError> {
        let mut conn = self.pool.get().await?;

        let result = diesel::insert_into(battle_tab::table)
            .values(&battle)
            .execute(&mut conn)
            .await?;

        Ok(result)
    }

    pub async fn get_battle(&self, id: &str) -> Result<BattleModel, AppError> {
        let mut conn = self.pool.get().await?;

        let result = battle_tab::table
            .filter(battle_tab::id.eq(id))
            .select(BattleModel::as_select())
            .first(&mut conn)
            .await?;

        Ok(result)
    }

    // pub async fn update_layout_ratings_and_create_battle_history<F>(
    //     &self,
    // ) -> Result<(), AppError> {
    //     let mut conn = self.pool.get().await?;

    //     let res = conn.transaction::<_, Error, _>(|conn| {


    //         Ok(())
    //     });

    //     Ok(())
    // }
}