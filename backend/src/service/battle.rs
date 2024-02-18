use axum::{
    extract::State, Extension, Json
};
use rand::Rng;
use crate::{
    db::{json::{ContentData, WordData},
    model::LayoutModel, DBClient}, error::AppError, layout_validation::validate_layout_data, middleware::Identity, words::translate_word, AppState
};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;

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
    id: String,
    words: Vec<[String; 2]>,
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

    if !validate_layout_data(&req.base_layout_data) {
        return Err(AppError::InvalidParameter(String::from("base_layout_data")))
    }

    // TODO: create limiter: either rate limit for public or count limiter for users

    // pick 2 random layouts
    let (layout_1, layout_2) = random_two_layouts(&state.db_client).await?;
    tracing::debug!("random 2 layouts {:?}, {:?}", layout_1, layout_2);

    // generate content
    let mut content_data = ContentData{
        words: Vec::new(),
    };
    let words = state.wordlist.random_words_with_limit(WORD_COUNT, MAX_WORD_LEN);

    for word in words {
        content_data.words.push(WordData{
            original: word.to_owned(),
            translated_1: translate_word(word, &req.base_layout_data, &layout_1.layout_data)?,
            translated_2: translate_word(word, &req.base_layout_data, &layout_2.layout_data)?,
            should_swap: rand::thread_rng().gen_bool(0.5),
        })
    }

    let mut words_for_resp = Vec::new();
    for content_word in content_data.words.iter() {
        words_for_resp.push(
            if !content_word.should_swap {
                [content_word.translated_1.clone(), content_word.translated_2.clone()]
            } else {
                [content_word.translated_2.clone(), content_word.translated_1.clone()]
            }
        )
    }

    // insert battle
    let user_id_typer = if let Some(user) = identity {
        Some(user.user_id)
    } else {
        None
    };

    let battle_id = nanoid!();

    state.db_client.create_battle(
        battle_id.clone(),
        layout_1.id, 
        layout_2.id, 
        req.base_layout_data,
        user_id_typer,
        content_data,
        req.is_personal).await?;

    // construct resp
    Ok(Json(CreateBattleResponse{
        id: battle_id,
        words: words_for_resp,
    }))
}

async fn random_two_layouts(db_client: &DBClient) -> Result<(LayoutModel, LayoutModel), AppError> {
    let max_seq_id = db_client.get_layout_max_sequence_id().await?;

    tracing::debug!("max seq id {}", max_seq_id);
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

    tracing::debug!("random 2 seq id {} {}", random_seq_1, random_seq_2);

    let layout_1 = db_client.get_layout_by_sequence_id(random_seq_1).await?;
    let layout_2 = db_client.get_layout_by_sequence_id(random_seq_2).await?;

    if layout_1.id == layout_2.id {
        return Err(AppError::NotEnoughLayoutsForBattle);
    }

    Ok((layout_1, layout_2))
}
