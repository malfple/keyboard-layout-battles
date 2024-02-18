#[derive(serde::Serialize, serde::Deserialize)]
pub struct ContentData {
    pub words: Vec<WordData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WordData {
    pub original: String,
    pub translated_1: String,
    pub translated_2: String,
    pub should_swap: bool,
}
