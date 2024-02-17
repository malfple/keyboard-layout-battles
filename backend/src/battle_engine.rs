use dashmap::DashMap;
use dashmap::mapref::entry::Entry::{Occupied, Vacant};
use nanoid::nanoid;
use rand::Rng;

use crate::db::model::{LayoutModel, UserModel};
use crate::error::AppError;
use crate::wordlist::Wordlist;

const WORD_COUNT: usize = 3;

pub enum ComfortChoice {
    First,
    Second,
}
pub struct BattleEngine {
    store: DashMap<String, Battle>,
    pub wordlist: Wordlist,
}

impl BattleEngine {
    pub fn new() -> BattleEngine {
        BattleEngine{
            store: DashMap::new(),
            wordlist: Wordlist::new(),
        }
    }

    fn insert_battle(&self, battle: Battle) -> String {
        loop {
            let id = nanoid!(10);
            match self.store.entry(id.clone()) {
                Occupied(_) => (),
                Vacant(entry) => {
                    entry.insert(battle);
                    break id;
                },
            }
        }
    }

    fn get_battle_cloned(&self, id: &str) -> Option<Battle> {
        match self.store.get(id) {
            Some(battle_ref) => Some(battle_ref.clone()),
            None => None,
        }
    }

    fn retrieve_battle(&self, id: &str) -> Option<Battle> {
        match self.store.remove(id) {
            Some((_, battle)) => Some(battle),
            None => None,
        }
    }

    /// Creates a battle
    /// 
    /// # Arguments
    /// 
    /// Will not use layout_data from typer but will use it from base_layout_data. Callers should adjust.
    pub fn create_battle(
        &self, base_layout_data: &str,
        layout_1: &LayoutModel,
        layout_2: &LayoutModel,
        typer: Option<&UserModel>,
        is_personal: bool
    ) -> Result<(String, Battle), AppError> {
        if layout_1.id == layout_2.id {
            return Err(AppError::BattleIdenticalLayout);
        }

        let mut battle = Battle{
            layout_id_1: layout_1.id,
            layout_id_2: layout_2.id,
            words: Vec::new(),
            user_id_typer: 0,
            is_personal,
        };

        if let Some(user) = typer {
            battle.user_id_typer = user.id;
        }

        // generate content
        let words = self.wordlist.random_words_with_limit(WORD_COUNT, 7);

        for word in words {
            battle.words.push(Word{
                original: word.to_owned(),
                translated_1: translate_word(word, &base_layout_data, &layout_1.layout_data)?,
                translated_2: translate_word(word, &base_layout_data, &layout_2.layout_data)?,
                should_swap: rand::thread_rng().gen_bool(0.5),
            })
        }

        // store ongoing battle
        let id = self.insert_battle(battle.clone());

        Ok((id, battle))
    }

    /// Finalizes the battle and do score calculations
    /// 
    /// # Arguments
    /// 
    /// * `times` - List of times in millisecond.
    /// The tuple's first and second value should correspond to what is displayed (taking into account spaws), not what's beings stored.
    /// * `comfort_choices` - The choice that the typer makes for comfort.
    /// Same as `times`, First or Second should take into account whether the words are swapped
    pub fn finalize_battle(
        &self, id: &str,
        times: [(i64, i64); WORD_COUNT],
        comfort_choices: [ComfortChoice; WORD_COUNT],
    ) -> Result<Battle, AppError> {
        let battle = self.retrieve_battle(id).ok_or(AppError::BattleNotFound)?;

        // calculate score and rating
        let mut score = 0; // positive = first wins, negative = second wins
        for i in 0..WORD_COUNT {
            let (time_1, time_2) = if !battle.words[i].should_swap {
                (times[i].0, times[i].1)
            } else {
                (times[i].1, times[i].0)
            };

            if time_1 < time_2 {
                score += 1;
            } else if time_1 > time_2 {
                score -= 1;
            } // if draw = do nothing
        }

        let mut comfort_score = 0;
        for choice in comfort_choices {
            match choice {
                ComfortChoice::First => comfort_score += 1,
                ComfortChoice::Second => comfort_score -= 1
            }
        }

        // 

        Ok(battle)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Word {
    original: String,
    translated_1: String, // translated from layout_1
    translated_2: String, // translated from layout_2
    should_swap: bool, // should the display be swapped? To hide the 2 layouts
}

#[derive(Debug, PartialEq, Clone)]
pub struct Battle {
    layout_id_1: u64,
    layout_id_2: u64,
    words: Vec<Word>,
    user_id_typer: u64,
    is_personal: bool,
}

fn translate_word(word: &str, base_layout_data: &str, target_layout_data: &str) -> Result<String, AppError> {
    // works regardless of the layout format, but both has to be the same
    if base_layout_data.len() != target_layout_data.len() {
        return Err(AppError::LayoutFormat(format!(
            "length of layout {0} and {1} is not the same", base_layout_data, target_layout_data
        )));
    }

    let mut res = String::with_capacity(word.len());
    for c in word.chars() {
        match target_layout_data.find(c) {
            Some(idx) => res.push(base_layout_data.as_bytes()[idx] as char),
            None => return Err(AppError::LayoutFormat(format!("incomplete charset {0}", target_layout_data))),
        }
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_layouts() -> (LayoutModel, LayoutModel, LayoutModel, LayoutModel) {
        let base_layout = LayoutModel{
            id: 1,
            name: "Qwerty".into(),
            layout_data: "qwertyuiopasdfghjkl;''zxcvbnm,./".into(),
            description: None,
            rating: 1500,
            rating_comfort: 1500,
            rating_data: None,
            time_created: 0,
            time_modified: 0,
        };
        let target_layout = LayoutModel{
            id: 2,
            name: "Colemak-DH".into(),
            layout_data: "qwfpbjluy;arstgmneio''zxcdvkh,./".into(),
            description: None,
            rating: 1500,
            rating_comfort: 1500,
            rating_data: None,
            time_created: 0,
            time_modified: 0,
        };
        let target_layout_2 = LayoutModel{
            id: 3,
            name: "MTGAP".into(),
            layout_data: "ypoujkdlcwinea,mhtsr''qz/.:bfgvx".into(),
            description: None,
            rating: 1500,
            rating_comfort: 1500,
            rating_data: None,
            time_created: 0,
            time_modified: 0,
        };
        let invalid_layout = LayoutModel{
            id: 4,
            name: "missing string".into(),
            layout_data: "qwfpbjluy;arstgmneio''zxcdvkh,.".into(),
            description: None,
            rating: 1500,
            rating_comfort: 1500,
            rating_data: None,
            time_created: 0,
            time_modified: 0,
        };
        (base_layout, target_layout, target_layout_2, invalid_layout)
    }

    #[test]
    fn test_insert_get_retrieve_battle() {
        let engine = BattleEngine::new();
        assert_eq!(engine.store.len(), 0);

        let battle = Battle{
            layout_id_1: 1,
            layout_id_2: 2,
            words: Vec::new(),
            user_id_typer: 1,
            is_personal: false,
        };
        let id = engine.insert_battle(battle.clone());
        assert!(engine.store.contains_key(&id));
        assert_eq!(engine.store.len(), 1);

        assert_eq!(engine.get_battle_cloned(&id), Some(battle.clone()));
        assert_eq!(engine.store.len(), 1);

        assert_eq!(engine.retrieve_battle("random_id"), None);
        assert_eq!(engine.retrieve_battle(&id), Some(battle.clone()));
        assert_eq!(engine.retrieve_battle(&id), None);
        assert_eq!(engine.store.len(), 0);
    }

    #[test]
    fn test_translate_word() {
        let (base_layout, target_layout, _, invalid_layout) = setup_layouts();
        
        assert_eq!(translate_word("arena", &base_layout.layout_data, &target_layout.layout_data).unwrap(), "askja".to_owned());
        assert_eq!(translate_word("work", &base_layout.layout_data, &target_layout.layout_data).unwrap(), "w;sn".to_owned());
        assert!(matches!(translate_word("arena", &base_layout.layout_data, &invalid_layout.layout_data).unwrap_err(), AppError::LayoutFormat(..)));
    }

    #[test]
    fn test_create_finalize_battle() {
        let (base_layout, target_layout, target_layout_2, invalid_layout) = setup_layouts();
        
        let engine = BattleEngine::new();

        assert!(matches!(engine.create_battle(&base_layout.layout_data, &target_layout, &target_layout, Some(&UserModel{
            id: 1,
            username: String::from("baba"),
            password: String::new(),
            layout_data: String::new(),
            time_created: 0,
            time_modified: 0,
        }), false).unwrap_err(), AppError::BattleIdenticalLayout));
        assert!(matches!(engine.create_battle(&base_layout.layout_data, &target_layout, &invalid_layout, Some(&UserModel{
            id: 1,
            username: String::from("baba"),
            password: String::new(),
            layout_data: String::new(),
            time_created: 0,
            time_modified: 0,
        }), false).unwrap_err(), AppError::LayoutFormat(..)));
        assert_eq!(engine.create_battle(&base_layout.layout_data, &target_layout, &target_layout_2, Some(&UserModel{
            id: 1,
            username: String::from("baba"),
            password: String::new(),
            layout_data: String::new(),
            time_created: 0,
            time_modified: 0,
        }), false).unwrap().1.layout_id_1, target_layout.id);
    }
}