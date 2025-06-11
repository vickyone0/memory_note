use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Note {
    pub note_id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewNote {
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateNote {
    pub note_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
}