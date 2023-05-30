use std::{collections::HashMap, fs};

use scoresaber::PlayerScore;

use crate::data::{LevelsStatsDatum, BeatmapCharacteristic};

mod scoresaber;
mod data;

async fn get_page(player_id: &u64, page: &u32) -> reqwest::Result<scoresaber::ScoresPage> {
    Ok(reqwest::get(format!(
        "https://scoresaber.com/api/player/{}/scores?page={}&sort=recent",
        player_id, page
    ))
    .await?
    .json()
    .await?)
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct LevelId (String, BeatmapCharacteristic, isize);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "./test.json";
    println!("Loading user data");
    let data_file = std::fs::File::open(&path)?;
    let mut userdata: data::PlayerData = serde_json::from_reader(&data_file)?;

    if userdata.local_players.len() == 0 { panic!("No local players"); };

    let mut existing_scores = {
        let mut hm = HashMap::<LevelId, LevelsStatsDatum>::new();

        for score in &userdata.local_players[0].levels_stats_data {
            hm.insert(LevelId(score.level_id.clone(), score.beatmap_characteristic_name.clone(), score.difficulty.clone()), score.clone());
        }

        hm
    };

    println!("{:#?}", &existing_scores);
    println!("Loaded {} existing scores from PlayerData.dat", existing_scores.len());

    let player_id: u64 = 76561198101382389;
    let mut all_scores = Vec::<PlayerScore>::new();

    let mut first_page = get_page(&player_id, &1).await?;
    all_scores.append(&mut first_page.scores);

    let pages: u32 = first_page.metadata.total / first_page.metadata.items_per_page
        + match first_page.metadata.total % first_page.metadata.items_per_page {
            0 => 0,
            _ => 1,
        };

    for page in 2..pages {
        println!("fetching score page {}", &page);
        let mut page_data = get_page(&player_id, &page).await?;
        all_scores.append(&mut page_data.scores);
    }

    for score in all_scores {
        let game_mode = match serde_json::from_str::<BeatmapCharacteristic>(&score.leaderboard.difficulty.game_mode.as_str()[4..]) {
            Ok(v) => v,
            _ => BeatmapCharacteristic::Standard
        };
        let level_id = LevelId(
            format!("custom_level_{}", score.leaderboard.song_hash.to_ascii_uppercase()),
            game_mode.clone(),
            score.leaderboard.difficulty.difficulty_int.clone()
        );
        let mut new_score = match existing_scores.get(&level_id) {
            Some(v) => v.clone(),
            None => LevelsStatsDatum {
                level_id: level_id.0.clone(),
                difficulty: level_id.2,
                beatmap_characteristic_name: game_mode.clone(),
                valid_score: true,
                play_count: 1,
                ..Default::default()
            }
        };

        new_score.high_score = score.score.modified_score;
        new_score.full_combo = score.score.full_combo;

        existing_scores.remove(&level_id);

        existing_scores.insert(level_id, new_score);
    }

    userdata.local_players[0].levels_stats_data = existing_scores.into_iter().map(|(_, v)| v).collect();

    fs::write(path, serde_json::to_string_pretty(&userdata)?)?;

    Ok(())
}
