use skillratings::glicko::GlickoRating;

use crate::error::AppError;

// functions in this file must take into account backward compatibility

const CURRENT_RATING_ALGORITHM: &str = "glicko1";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ContentData {
    pub words: Vec<ContentWordData>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ContentWordData {
    pub original: String,
    pub translated_1: String,
    pub translated_2: String,
    pub should_swap: bool,
}

pub fn decode_content_data(content_data: serde_json::Value) -> Result<ContentData, AppError> {
    let content_data = serde_json::from_value::<ContentData>(content_data)?;

    Ok(content_data)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RatingData {
    pub alg: String,
    pub global: GlickoRating,
    pub comfort: GlickoRating
}

// TODO: maybe change these function so that they become methods for rating models, using traits.
pub fn decode_rating_data(rating_data_option: Option<serde_json::Value>, rating: i32, rating_comfort: i32) -> Result<RatingData, AppError> {
    match rating_data_option {
        Some(rating_data) => Ok(serde_json::from_value::<RatingData>(rating_data)?),
        None => Ok(RatingData{ // create default values
            alg: String::from(CURRENT_RATING_ALGORITHM),
            global: GlickoRating{
                rating: rating as f64,
                deviation: 350.0,
            },
            comfort: GlickoRating{
                rating: rating_comfort as f64,
                deviation: 350.0,
            },
        }),
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResultData {
    pub words: Vec<ResultWordData>,
    pub score: i32,
    pub comfort_score: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResultWordData {
    pub original: String,
    pub translated_1: String,
    pub translated_2: String,
    pub time_1: i64,
    pub time_2: i64,
    pub comfort_choice: i32,
}
