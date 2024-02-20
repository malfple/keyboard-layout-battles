use axum::{
    extract::State, Extension, Json,
};
use rand::Rng;
use skillratings::{glicko::{glicko, GlickoConfig}, Outcomes};
use crate::{
    db::{json::{decode_content_data, ContentData, ContentWordData, RatingData, ResultData, ResultWordData},
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
    words: Vec<(String, String)>,
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
    let mut content = ContentData{
        words: Vec::new(),
    };
    let words = state.wordlist.random_words_with_limit(WORD_COUNT, MAX_WORD_LEN);

    for word in words {
        content.words.push(ContentWordData{
            original: word.to_owned(),
            translated_1: translate_word(word, &req.base_layout_data, &layout_1.layout_data)?,
            translated_2: translate_word(word, &req.base_layout_data, &layout_2.layout_data)?,
            should_swap: rand::thread_rng().gen_bool(0.5),
        })
    }

    let mut words_for_resp = Vec::new();
    for content_word in content.words.iter() {
        words_for_resp.push(
            if !content_word.should_swap {
                (content_word.translated_1.clone(), content_word.translated_2.clone())
            } else {
                (content_word.translated_2.clone(), content_word.translated_1.clone())
            }
        )
    }

    // insert battle
    let user_id_typer = if let Some(identity) = identity {
        Some(identity.user_id)
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
        content,
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

#[derive(Debug, Deserialize)]
pub struct FinalizeBattleRequest {
    id: String,
    times: Vec<(i64, i64)>,
    comfort_choice: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct FinalizeBattleResponse {
    layout_id_1: u64,
    layout_id_2: u64,
    result_data: ResultData,
}

/// Finalize battle API
pub async fn finalize_battle(
    State(state): State<AppState>,
    Extension(identity): Extension<Option<Identity>>,
    Json(req): Json<FinalizeBattleRequest>,
) -> Result<Json<FinalizeBattleResponse>, AppError> {
    if req.times.len() != WORD_COUNT {
        return Err(AppError::InvalidParameter(String::from("times")));
    }

    // TODO: validate realistic time range

    if req.comfort_choice.len() != WORD_COUNT {
        return Err(AppError::InvalidParameter(String::from("comfort_choice")));
    }

    for choice in req.comfort_choice.iter() {
        if !validate_comfort_choice(*choice) {
            return Err(AppError::InvalidParameter(String::from("comfort_choice")));
        }
    }

    let battle = state.db_client.get_battle(&req.id).await?;

    if let Some(user_id_typer) = battle.user_id_typer { // if typer exist, then only typer can update
        match identity {
            Some(identity) => if user_id_typer != identity.user_id {
                return Err(AppError::Unauthorized); // incorrect auth
            },
            None => return Err(AppError::Unauthorized), // no auth
        }
    }

    let content = decode_content_data(battle.content_data)?;

    // calculate score
    let mut result = ResultData{
        words: Vec::new(),
        score: 0, // positive = first wins, negative = second wins
        comfort_score: 0, // same as above
    };
    for i in 0..WORD_COUNT {
        let content_word = &content.words[i];
        // time
        let (time_1, time_2) = if !content_word.should_swap {
            (req.times[i].0, req.times[i].1)
        } else {
            (req.times[i].1, req.times[i].0)
        };

        if time_1 < time_2 {
            result.score += 1;
        } else if time_1 > time_2 {
            result.score -= 1;
        } // if draw = do nothing

        // comfort
        let comfort = if !content_word.should_swap {
            req.comfort_choice[i]
        } else {
            if req.comfort_choice[i] == 1 {2} else {1}
        };
        match comfort {
            1 => result.comfort_score += 1,
            2 => result.comfort_score -= 1,
            _ => tracing::error!("comfort choice error"),
        }
        // push to result
        result.words.push(ResultWordData{
            original: content_word.original.clone(), // TODO: optimize
            translated_1: content_word.translated_1.clone(),
            translated_2: content_word.translated_2.clone(),
            time_1,
            time_2,
            comfort_choice: comfort,
        })
    }

    // commit to history
    let rows_affected = state.db_client.make_battle_history_and_update_ratings(
        &req.id,
        battle.layout_id_1,
        battle.layout_id_2,
        battle.base_layout_data,
        battle.user_id_typer,
        result.clone(),
        battle.is_personal,
        update_rating
    ).await?;
    
    tracing::debug!("Create battle history, rows affected {}", rows_affected);

    // construct resp
    Ok(Json(FinalizeBattleResponse{
        layout_id_1: battle.layout_id_1,
        layout_id_2: battle.layout_id_2,
        result_data: result,
    }))
}

fn validate_comfort_choice(comfort: i32) -> bool {
    comfort == 1 || comfort == 2
}

fn calc_outcome(score: i32) -> Outcomes {
    if score > 0 {
        Outcomes::WIN
    } else if score < 0 {
        Outcomes::LOSS
    } else {
        Outcomes::DRAW
    }
}

/// Updates rating in-place
fn update_rating(rating_1: &mut RatingData, rating_2: &mut RatingData, score: i32, comfort_score: i32) {
    tracing::debug!("old rating data {:?} {:?}", rating_1, rating_2);

    let config = GlickoConfig::new();
    (rating_1.global, rating_2.global) = glicko(
        &rating_1.global,
        &rating_2.global,
        &calc_outcome(score),
        &config);
    (rating_1.comfort, rating_2.comfort) = glicko(
        &rating_1.comfort,
        &rating_2.comfort,
        &calc_outcome(comfort_score),
        &config);

    tracing::debug!("new rating data {:?} {:?}", rating_1, rating_2);
}
