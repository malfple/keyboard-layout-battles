use axum::extract::State;
use crate::AppState;

mod user;
mod layout;
mod battle;
mod battle_history;

pub use user::*;
pub use layout::*;
pub use battle::*;
pub use battle_history::*;

pub async fn ping(State(_state): State<AppState>) -> &'static str {
    "Hello, World!"
}
