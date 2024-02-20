use std::time::{SystemTime, UNIX_EPOCH};

use diesel::{prelude::*, result::Error};
use diesel_async::{
    pooled_connection::{
        deadpool::{Object, Pool}, AsyncDieselConnectionManager
    }, scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl
};
use crate::{
    error::AppError, settings::AppSettings
};
use schema::*;
use model::*;

use self::json::{decode_rating_data, ContentData, RatingData, ResultData};

pub mod schema;
pub mod model;
pub mod json;

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

    // Users

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

    // Layouts

    pub async fn get_layout_by_id(&self, id: u64) -> Result<LayoutModel, AppError> {
        let mut conn = self.pool.get().await?;

        let result = layout_tab::table
            .filter(layout_tab::id.eq(id))
            .select(LayoutModel::as_select())
            .first(&mut conn)
            .await?;

        Ok(result)
    }

    async fn get_layouts_by_ids_for_update_with_conn(
        &self,
        conn: &mut Object<diesel_async::AsyncMysqlConnection>,
        ids: Vec<u64>,
    ) -> Result<Vec<LayoutModel>, Error> {
        let result = layout_tab::table
            .filter(layout_tab::id.eq_any(ids))
            .select(LayoutModel::as_select())
            .for_update()
            .load(conn)
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

    pub async fn get_layout_max_sequence_id(&self) -> Result<u64, AppError> {
        let mut conn = self.pool.get().await?;

        let result: Option<u64> = layout_tab::table
            .select(diesel::dsl::max(layout_tab::sequence_id))
            .first(&mut conn)
            .await?;

        Ok(result.unwrap_or(0)) // if no id found, then set to 0
    }

    pub async fn get_layout_by_sequence_id(&self, sequence_id: u64) -> Result<LayoutModel, AppError> {
        let mut conn = self.pool.get().await?;

        let result = layout_tab::table
            .filter(layout_tab::sequence_id.eq(sequence_id)) 
            .select(LayoutModel::as_select())
            .first(&mut conn)
            .await?;

        Ok(result)
    }

    async fn update_layout_rating_with_conn(
        &self,
        conn: &mut Object<diesel_async::AsyncMysqlConnection>,
        id: u64,
        rating: i32,
        rating_comfort: i32,
        rating_data: serde_json::Value,
    ) -> Result<usize, Error> {
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_millis() as i64;

        let result = diesel::update(layout_tab::table)
            .filter(layout_tab::id.eq(id))
            .set((
                layout_tab::rating.eq(rating),
                layout_tab::rating_comfort.eq(rating_comfort),
                layout_tab::rating_data.eq(rating_data),
                layout_tab::time_modified.eq(time_now),
            ))
            .execute(conn)
            .await?;

        Ok(result)
    }

    // battle

    pub async fn create_battle(
        &self,
        id: String,
        layout_id_1: u64,
        layout_id_2: u64,
        base_layout_data: String,
        user_id_typer: Option<u64>,
        content_data: ContentData,
        is_personal: bool,
    ) -> Result<usize, AppError> {
        let mut conn = self.pool.get().await?;

        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_millis() as i64;
        let battle = BattleModel{
            id,
            layout_id_1,
            layout_id_2,
            base_layout_data,
            user_id_typer,
            content_data: serde_json::to_value(content_data)?,
            is_personal,
            time_created: time_now,
            time_modified: time_now,
        };

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

    async fn delete_battle_with_conn(
        &self,
        conn: &mut Object<diesel_async::AsyncMysqlConnection>,
        id: &str
    ) -> Result<usize, Error> {
        let result = diesel::delete(battle_tab::table)
            .filter(battle_tab::id.eq(id))
            .execute(conn)
            .await?;

        Ok(result)
    }

    // battle history
    async fn create_battle_history_with_conn(
        &self,
        conn: &mut Object<diesel_async::AsyncMysqlConnection>,
        layout_id_1: u64,
        layout_id_2: u64,
        base_layout_data: String,
        user_id_typer: Option<u64>,
        layout_1_rating: i32,
        layout_2_rating: i32,
        rating_1_gain: i32,
        rating_2_gain: i32,
        result_data: serde_json::Value,
        is_personal: bool,
    ) -> Result<usize, Error> {
        let time_now = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards").as_millis() as i64;
        let battle_history = BattleHistoryModelForInsert{
            layout_id_1,
            layout_id_2,
            base_layout_data,
            user_id_typer,
            layout_1_rating,
            layout_2_rating,
            rating_1_gain,
            rating_2_gain,
            result_data,
            is_personal,
            time_created: time_now,
        };

        let result = diesel::insert_into(battle_history_tab::table)
            .values(&battle_history)
            .execute(conn)
            .await?;

        Ok(result)
    }

    pub async fn get_battle_history_lite_list_ordered_by_time(&self, limit: i64) -> Result<Vec<BattleHistoryLiteModel>, AppError> {
        let mut conn = self.pool.get().await?;

        let result = battle_history_tab::table
            .select(BattleHistoryLiteModel::as_select())
            .order(battle_history_tab::id.desc())
            .limit(limit)
            .load(&mut conn)
            .await?;

        Ok(result)
    }

    pub async fn get_battle_history_by_id(&self, id: u64) -> Result<BattleHistoryModel, AppError> {
        let mut conn = self.pool.get().await?;

        let result = battle_history_tab::table
            .filter(battle_history_tab::id.eq(id))
            .select(BattleHistoryModel::as_select())
            .first(&mut conn)
            .await?;

        Ok(result)
    }

    // compound

    /// Does a lot of things in a transaction
    /// 
    /// # Arguments
    /// 
    /// * rating_func: A func with these parameters (rating_1, rating_2, score, comfort_score)
    pub async fn make_battle_history_and_update_ratings(
        &self,
        battle_id: &str,
        layout_id_1: u64,
        layout_id_2: u64,
        base_layout_data: String,
        user_id_typer: Option<u64>,
        result_data: ResultData,
        is_personal: bool,
        update_rating_func: fn(&mut RatingData, &mut RatingData, i32, i32),
    ) -> Result<usize, AppError> {
        let mut conn = self.pool.get().await?;
        
        let result = conn.transaction::<usize, Error, _>(
            |conn| async move {
                // table lock order: layout_tab > battle_tab

                let mut rows_affected = 0;

                // read layouts and lock.
                let mut layouts =
                    self.get_layouts_by_ids_for_update_with_conn(conn, vec![layout_id_1, layout_id_2])
                    .await?;

                if layouts.len() != 2 {
                    tracing::error!("queried layouts should be exactly 2. Instead it's {}", layouts.len());
                    return Err(Error::RollbackTransaction);
                }

                if layouts[0].id != layout_id_1 { // it's flipped
                    layouts.swap(0, 1);
                }

                let layout_2 = layouts.remove(1);
                let layout_1 = layouts.remove(0);

                // delete battle
                rows_affected += self.delete_battle_with_conn(conn, battle_id).await?;

                // calc ratings
                let mut rating_1 = decode_rating_data(
                    layout_1.rating_data, layout_1.rating, layout_1.rating_comfort
                ).map_err(|_| Error::RollbackTransaction)?;
                let mut rating_2 = decode_rating_data(
                    layout_2.rating_data, layout_2.rating, layout_2.rating_comfort
                ).map_err(|_| Error::RollbackTransaction)?;
                
                update_rating_func(&mut rating_1, &mut rating_2, result_data.score, result_data.comfort_score);

                let new_rating_1 = rating_1.global.rating.round() as i32;
                let new_rating_comfort_1 = rating_1.comfort.rating.round() as i32;
                let new_rating_2 = rating_2.global.rating.round() as i32;
                let new_rating_comfort_2 = rating_2.comfort.rating.round() as i32;
                let rating_1_gain = new_rating_1 - layout_1.rating;
                let rating_2_gain = new_rating_2 - layout_2.rating;

                // update layouts
                rows_affected += self.update_layout_rating_with_conn(
                    conn,
                    layout_1.id,
                    new_rating_1,
                    new_rating_comfort_1,
                    serde_json::to_value(rating_1).map_err(|_| Error::RollbackTransaction)?,
                ).await?;
                rows_affected += self.update_layout_rating_with_conn(
                    conn,
                    layout_2.id,
                    new_rating_2,
                    new_rating_comfort_2,
                    serde_json::to_value(rating_2).map_err(|_| Error::RollbackTransaction)?,
                ).await?;

                // insert history
                rows_affected += self.create_battle_history_with_conn(
                    conn,
                    layout_id_1,
                    layout_id_2,
                    base_layout_data,
                    user_id_typer,
                    new_rating_1,
                    new_rating_2,
                    rating_1_gain,
                    rating_2_gain,
                    serde_json::to_value(result_data).map_err(|_| Error::RollbackTransaction)?,
                    is_personal).await?;

                Ok(rows_affected)
            }.scope_boxed()
        ).await?;

        Ok(result)
    }
}