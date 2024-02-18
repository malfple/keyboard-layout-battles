use axum::{
    debug_handler, extract::State, Extension, Form, Json
};
use rand::Rng;
use crate::{
    auth, db::{model::LayoutModel, DBClient}, error::{bad_request_error_response, AppError}, middleware::Identity, AppState
};
use serde::{Serialize, Deserialize};

const MIN_LAYOUT_COUNT: u64 = 5;
const WORD_COUNT: usize = 3;
const MAX_WORD_LEN: usize = 7;

#[derive(Debug, Deserialize)]
pub struct CreateBattleRequest {
    base_layout_data: String,
    is_personal: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateBattleResponse {

}

/// Creates battle API
/// 
/// Will not use layout_data from typer's data but will use it from base_layout_data.
pub async fn create_battle(
    State(state): State<AppState>,
    Extension(identity): Extension<Option<Identity>>,
    Json(req): Json<CreateBattleRequest>,
) -> Result<Json<CreateBattleResponse>, AppError> {
    if req.is_personal {
        return Err(AppError::BadRequest(String::from("is_personal is not yet supported")));
    }

    // pick 2 random layouts
    let (layout_1, layout_2) = random_two_layouts(&state.db_client).await?;

    // generate content
    let words = state.wordlist.random_words_with_limit(WORD_COUNT, MAX_WORD_LEN);

    // for word in words {
    //     battle.words.push(Word{
    //         original: word.to_owned(),
    //         translated_1: translate_word(word, &base_layout_data, &layout_1.layout_data)?,
    //         translated_2: translate_word(word, &base_layout_data, &layout_2.layout_data)?,
    //         should_swap: rand::thread_rng().gen_bool(0.5),
    //     })
    // }


    // insert battle

    Err(AppError::BadRequest("sntc".to_owned()))
}

async fn random_two_layouts(db_client: &DBClient) -> Result<(LayoutModel, LayoutModel), AppError> {
    let max_seq_id = db_client.get_layout_max_sequence_id().await?;

    if max_seq_id < MIN_LAYOUT_COUNT {
        return Err(AppError::NotEnoughLayoutsForBattle);
    }

    let random_seq_1 = rand::thread_rng().gen_range(1..=max_seq_id);
    let random_seq_2 = loop {
        let seq = rand::thread_rng().gen_range(1..=max_seq_id);
        if seq != random_seq_1 {
            break seq;
        }
    };

    let layout_1 = db_client.get_active_layout_by_sequence_id(random_seq_1).await?;
    let layout_2 = db_client.get_active_layout_by_sequence_id(random_seq_2).await?;

    Ok((layout_1, layout_2))
}
