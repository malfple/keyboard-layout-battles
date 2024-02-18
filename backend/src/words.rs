use std::fs;

use rand::{seq::SliceRandom, Rng};
use serde::Deserialize;

use crate::error::AppError;


#[derive(Debug, Deserialize)]
pub struct Wordlist {
    words: Vec<String>,
}

const WORDLIST_FILE_PATH: &str = "wordlist.json";

impl Wordlist {
    pub fn new() -> Wordlist {
        let data = fs::read_to_string(WORDLIST_FILE_PATH).expect("wordlist file missing");
        let res: Wordlist = serde_json::from_str(&data).expect("unable to parse wordlist file");
        res
    }

    pub fn random_words(&self, count : usize) -> Vec<&String> {
        let words = self.words
            .choose_multiple(&mut rand::thread_rng(), count)
            .collect();

        words
    }

    pub fn random_words_with_limit(&self, count: usize, limit : usize) -> Vec<&str> {
        let words = self.random_words(count);
        let mut result = Vec::new();

        for word in words {
            result.push(
                if word.len() <= limit {
                    word
                } else {
                    let extra = word.len() - limit;
                    let idx = rand::thread_rng().gen_range(0..=extra);
                    &word[idx..idx+limit]
                }
            );
        }
        
        result
    }
}

pub fn translate_word(word: &str, base_layout_data: &str, target_layout_data: &str) -> Result<String, AppError> {
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

    #[test]
    fn test_wordlist_init() {
        let ws = Wordlist::new();
        assert_eq!(ws.words.get(0).unwrap(), "ability");
    }

    #[test]
    fn test_wordlist_random_words() {
        let ws = Wordlist::new();

        for _ in 0..10 {
            let w = ws.random_words(1);
            assert!(ws.words.contains(w[0]));

            let w = ws.random_words(5);
            assert_eq!(5, w.len());
            for x in w {
                assert!(ws.words.contains(x));
            }
        }
    }

    #[test]
    fn test_wordlist_random_words_with_limit() {
        let ws = Wordlist::new();

        for i in 0..10 {
            let w = ws.random_words_with_limit(1, i);
            assert!(w[0].len() <= i);

            let w = ws.random_words_with_limit(5, i);
            assert_eq!(5, w.len());
            for x in w {
                assert!(x.len() <= i);
            }
        }
    }

    #[test]
    fn test_translate_word() {
        let base_layout_data = "qwertyuiopasdfghjkl;''zxcvbnm,./"; // qwerty
        let target_layout_data = "qwfpbjluy;arstgmneio''zxcdvkh,./"; // colemak-dh
        let invalid_layout_data = "qwfpbjluy;arstgmneio''zxcdvkh,."; // invalid
        
        assert_eq!(translate_word("arena", base_layout_data, target_layout_data).unwrap(), "askja".to_owned());
        assert_eq!(translate_word("work", base_layout_data, target_layout_data).unwrap(), "w;sn".to_owned());
        assert!(matches!(translate_word("arena", base_layout_data, invalid_layout_data).unwrap_err(), AppError::LayoutFormat(..)));
    }
}
