use serde::{ Deserialize, Serialize };
use chrono::{ DateTime, Utc };

#[derive(Serialize, Deserialize)]
pub struct VideoEntry {
    pub path: String,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub duration: f64,
}
