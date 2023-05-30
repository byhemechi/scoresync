use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoresPage {
    #[serde(rename = "playerScores")]
    pub scores: Vec<PlayerScore>,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub total: u32,
    pub page: u32,
    pub items_per_page: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScore {
    pub score: Score,
    pub leaderboard: Leaderboard,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaderboard {
    pub id: i64,
    pub song_hash: String,
    pub song_name: String,
    pub song_sub_name: String,
    pub song_author_name: String,
    pub level_author_name: String,
    pub difficulty: Difficulty,
    pub max_score: Option<i64>,
    pub created_date: String,
    pub ranked_date: Option<String>,
    pub qualified_date: Option<String>,
    pub loved_date: Option<String>,
    pub ranked: bool,
    pub qualified: bool,
    pub loved: bool,
    pub max_pp: Option<i64>,
    pub stars: Option<f64>,
    pub plays: i64,
    pub daily_plays: i64,
    pub positive_modifiers: bool,
    pub player_score: Option<serde_json::Value>,
    pub cover_image: String,
    pub difficulties: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub leaderboard_id: i64,
    #[serde(rename = "difficulty")]
    pub difficulty_int: isize,
    pub game_mode: String,
    pub difficulty_raw: DifficultyRaw,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub id: i64,
    pub rank: i64,
    pub base_score: i64,
    pub modified_score: i64,
    pub pp: f64,
    pub weight: f64,
    pub modifiers: String,
    pub multiplier: f64,
    pub bad_cuts: i64,
    pub missed_notes: i64,
    pub max_combo: i64,
    pub full_combo: bool,
    pub hmd: i64,
    pub time_set: String,
    pub has_replay: bool,
}

#[derive(Debug, serde_enum_str::Serialize_enum_str, serde_enum_str::Deserialize_enum_str)]
pub enum DifficultyRaw {
    #[serde(rename = "_Easy_SoloStandard")]
    Easy,
    #[serde(rename = "_Normal_SoloStandard")]
    Normal,
    #[serde(rename = "_Hard_SoloStandard")]
    Hard,
    #[serde(rename = "_Expert_SoloStandard")]
    Expert,
    #[serde(rename = "_ExpertPlus_SoloStandard")]
    ExpertPlus,
    #[serde(other)]
    Other(String),
}
