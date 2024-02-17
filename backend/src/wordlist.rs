use std::fs;

use rand::{seq::SliceRandom, Rng};
use serde::Deserialize;


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let ws = Wordlist::new();
        assert_eq!(ws.words.get(0).unwrap(), "ability");
    }

    #[test]
    fn test_random_words() {
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
    fn test_random_words_with_limit() {
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
}
