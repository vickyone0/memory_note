use chrono::{NaiveDateTime};
use serde::Serialize;


#[derive(Serialize)]
pub struct Note{
    pub note_id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub created_at: NaiveDateTime,
}