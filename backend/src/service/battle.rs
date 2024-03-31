use axum::{
    extract::{Path, State}, Extension, Json,
};
use rand::Rng;
use skillratings::{glicko::{glicko, GlickoConfig}, Outcomes};
use crate::{
    db::{json::{decode_content_data, ContentData, ContentWordData, RatingData, ResultData, ResultWordData},
    model::{BattleModel, LayoutModel}, DBClient}, error::AppError, layout_validation::{calc_layout_difference, validate_base_layout_data}, middleware::Identity, words::{translate_word, Wordlist}, AppState
};
use serde::{Serialize, Deserialize};
use nanoid::nanoid;

const MIN_LAYOUT_COUNT: u64 = 5;
const WORD_COUNT: usize = 5;
const MAX_WORD_LEN: usize = 7;
const GLICKO_C_VALUE: f64 = 20.0;
const TIME_PERCENT_FOR_DRAW: f64 = 10.0;
const MIN_TIME_FOR_DRAW: i64 = 25;

const MAX_RANDOM_LAYOUT_ATTEMPTS: i32 = 3;
const MIN_RANDOM_LAYOUT_DIFF: i32 = 4;
const MAX_RANDOM_WORD_ATTEMPTS: i32 = 10;

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

    if !validate_base_layout_data(&req.base_layout_data) {
        return Err(AppError::InvalidParameter(String::from("base_layout_data")))
    }

    // TODO: create limiter: either rate limit for public or count limiter for users

    // pick 2 random layouts
    let (layout_1, layout_2) = random_two_layouts(&state.db_client).await?;
    tracing::debug!("random 2 layouts {:?}, {:?}", layout_1, layout_2);

    // generate content
    let content = generate_content(
        &state.wordlist, &req.base_layout_data, &layout_1.layout_data, &layout_2.layout_data
    )?;

    let words_for_resp = construct_words_for_resp(&content);

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

    let mut best_pair: Option<(LayoutModel, LayoutModel)> = None;
    let mut diff = -1;
    for _ in 0..MAX_RANDOM_LAYOUT_ATTEMPTS {
        let random_seq_1 = rand::thread_rng().gen_range(1..=max_seq_id);
        let random_seq_2 = loop {
            let seq = rand::thread_rng().gen_range(1..=max_seq_id);
            if seq != random_seq_1 {
                break seq;
            }
        };

        let layout_1 = db_client.get_layout_by_sequence_id(random_seq_1).await?;
        let layout_2 = db_client.get_layout_by_sequence_id(random_seq_2).await?;

        if layout_1.id == layout_2.id {
            return Err(AppError::NotEnoughLayoutsForBattle);
        }

        let t_diff = calc_layout_difference(&layout_1.layout_data, &layout_2.layout_data);
        
        tracing::debug!("random 2 layout with seq {} {}, diff {}", random_seq_1, random_seq_2, t_diff);

        if t_diff >= MIN_RANDOM_LAYOUT_DIFF {
            return Ok((layout_1, layout_2));
        }
        
        if t_diff > diff {
            diff = t_diff;
            best_pair = Some((layout_1, layout_2));
        }
    }

    if let Some(best_pair) = best_pair {
        Ok(best_pair)
    } else {
        Err(AppError::NotEnoughLayoutsForBattle)
    }
}

fn generate_content(wordlist: &Wordlist, base_layout_data: &str, layout_1_data: &str, layout_2_data: &str) -> Result<ContentData, AppError> {
    let mut content = ContentData{
        words: Vec::new(),
    };

    for _ in 0..WORD_COUNT {
        let word = loop { // retry if word already chosen
            let w = wordlist.random_words_with_limit(1, MAX_WORD_LEN)[0];
            let mut is_new = true;
            for c in content.words.iter() {
                if w == c.original {
                    is_new = false;
                    break;
                }
            }
            if is_new {
                break w;
            }
        };

        let mut random_word_attempt = 0;
        let (translated_1, translated_2) = loop { // retry while translated word is the same
            random_word_attempt += 1;
            let translated_1 = translate_word(word, base_layout_data, layout_1_data)?;
            let translated_2 = translate_word(word, base_layout_data, layout_2_data)?;

            if random_word_attempt >= MAX_RANDOM_WORD_ATTEMPTS || translated_1 != translated_2 {
                break (translated_1, translated_2);
            }
        };

        content.words.push(ContentWordData{
            original: word.to_owned(),
            translated_1,
            translated_2,
            should_swap: rand::thread_rng().gen_bool(0.5),
        })
    }

    Ok(content)
}

fn construct_words_for_resp(content: &ContentData) -> Vec<(String, String)> {
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

    words_for_resp
}

#[derive(Debug, Serialize)]
pub struct GetBattleResponse {
    words: Vec<(String, String)>,
}

/// Get Battle API
pub async fn get_battle(
    State(state): State<AppState>,
    Extension(identity): Extension<Option<Identity>>,
    Path(id): Path<String>,
) -> Result<Json<GetBattleResponse>, AppError> {
    let battle = state.db_client.get_battle(&id).await?;

    _ = authenticate_typer(&battle, &identity)?;

    let content = decode_content_data(battle.content_data)?;

    let words_for_resp = construct_words_for_resp(&content);

    Ok(Json(GetBattleResponse{
       words: words_for_resp,
    }))
}

fn authenticate_typer(battle: &BattleModel, identity: &Option<Identity>) -> Result<(), AppError> {
    if let Some(user_id_typer) = battle.user_id_typer { // if typer exist, then only typer can access
        match identity {
            Some(identity) => if user_id_typer != identity.user_id {
                return Err(AppError::Unauthorized); // incorrect auth
            },
            None => return Err(AppError::Unauthorized), // no auth
        }
    }

    Ok(())
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

    _ = authenticate_typer(&battle, &identity)?;

    let content = decode_content_data(battle.content_data)?;

    let result = calc_result(&content, req.times, req.comfort_choice);

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
    comfort == 0 || comfort == 1 || comfort == 2
}

fn calc_result(content: &ContentData, times: Vec<(i64, i64)>, comfort_choice: Vec<i32>) -> ResultData {
    // calculate score
    let mut result = ResultData{
        words: Vec::new(),
        score: 0, // positive = first wins, negative = second wins
        comfort_score: 0, // same as above
    };
    for i in 0..content.words.len() {
        let content_word = &content.words[i];
        // time
        let (time_1, time_2) = if !content_word.should_swap {
            (times[i].0, times[i].1)
        } else {
            (times[i].1, times[i].0)
        };

        let mut score = 0;
        let min_diff = std::cmp::max(
            ((std::cmp::min(time_1, time_2) as f64) * TIME_PERCENT_FOR_DRAW / 100.0).round() as i64,
            MIN_TIME_FOR_DRAW
        );
        if time_1 + min_diff < time_2 { // time_2 is atleast min_diff higher than time_1
            result.score += 1;
            score = 1;
        } else if time_1 > time_2 + min_diff { // vice verca
            result.score -= 1;
            score = -1;
        } // if draw = do nothing

        // comfort
        let comfort = if !content_word.should_swap {
            comfort_choice[i]
        } else {
            if comfort_choice[i] == 1 {2} else if comfort_choice[i] == 2 {1} else {0}
        };
        match comfort {
            1 => result.comfort_score += 1,
            2 => result.comfort_score -= 1,
            0 => result.comfort_score += 0, // draw, do nothing
            _ => tracing::error!("comfort choice error"),
        }
        // push to result
        result.words.push(ResultWordData{
            original: content_word.original.clone(), // TODO: optimize
            translated_1: content_word.translated_1.clone(),
            translated_2: content_word.translated_2.clone(),
            time_1,
            time_2,
            score,
            comfort_choice: comfort,
        })
    }

    result
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

    let config = GlickoConfig { c: GLICKO_C_VALUE };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_result() {
        let no_swap: ContentWordData = ContentWordData{
            original: "some_word".to_owned(),
            translated_1: "some_word".to_owned(),
            translated_2: "some_word".to_owned(),
            should_swap: false,
        };
    
        let with_swap: ContentWordData = ContentWordData{
            original: "some_word".to_owned(),
            translated_1: "some_word".to_owned(),
            translated_2: "some_word".to_owned(),
            should_swap: true,
        };
    
        let result_1: ResultWordData = ResultWordData{
            original: "some_word".to_owned(),
            translated_1: "some_word".to_owned(),
            translated_2: "some_word".to_owned(),
            time_1: 100,
            time_2: 200,
            score: 1,
            comfort_choice: 1,
        };
    
        let result_2: ResultWordData = ResultWordData{
            original: "some_word".to_owned(),
            translated_1: "some_word".to_owned(),
            translated_2: "some_word".to_owned(),
            time_1: 200,
            time_2: 100,
            score: -1,
            comfort_choice: 2,
        };

        // test swaps
        let result = calc_result(&ContentData{
            words: vec![no_swap.clone(), with_swap.clone(), with_swap.clone()],
        }, vec![(100, 200), (100, 200), (100, 200)], vec![1, 1, 1]);

        assert_eq!(result, ResultData{
            words: vec![result_1.clone(), result_2.clone(), result_2.clone()],
            score: -1,
            comfort_score: -1,
        });

        // test draw
        let result = calc_result(&ContentData{
            words: vec![no_swap.clone(), with_swap.clone()],
        }, vec![(100, 200), (100, 200)], vec![1, 1]);

        assert_eq!(result, ResultData{
            words: vec![result_1.clone(), result_2.clone()],
            score: 0,
            comfort_score: 0,
        });

        // test draw in word
        let result = calc_result(&ContentData{
            words: vec![no_swap.clone(), no_swap.clone()],
        }, vec![(920, 1000), (100, 200)], vec![0, 1]);

        assert_eq!(result, ResultData{
            words: vec![ResultWordData{
                original: "some_word".to_owned(),
                translated_1: "some_word".to_owned(),
                translated_2: "some_word".to_owned(),
                time_1: 920,
                time_2: 1000,
                score: 0,
                comfort_choice: 0,
            }, result_1.clone()],
            score: 1,
            comfort_score: 1,
        });

        let result = calc_result(&ContentData{
            words: vec![no_swap.clone(), no_swap.clone()],
        }, vec![(880, 1000), (100, 200)], vec![0, 1]);

        assert_eq!(result, ResultData{
            words: vec![ResultWordData{
                original: "some_word".to_owned(),
                translated_1: "some_word".to_owned(),
                translated_2: "some_word".to_owned(),
                time_1: 880,
                time_2: 1000,
                score: 1,
                comfort_choice: 0,
            }, result_1.clone()],
            score: 2,
            comfort_score: 1,
        });

        // test draw time too low
        let result = calc_result(&ContentData{
            words: vec![no_swap.clone(), no_swap.clone()],
        }, vec![(88, 100), (100, 200)], vec![0, 1]);

        assert_eq!(result, ResultData{
            words: vec![ResultWordData{
                original: "some_word".to_owned(),
                translated_1: "some_word".to_owned(),
                translated_2: "some_word".to_owned(),
                time_1: 88,
                time_2: 100,
                score: 0,
                comfort_choice: 0,
            }, result_1.clone()],
            score: 1,
            comfort_score: 1,
        });
    }
}
