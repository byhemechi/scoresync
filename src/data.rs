use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
    pub version: String,
    pub local_players: Vec<Player>,
    pub guest_players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
    pub should_show_tutorial_prompt: bool,
    pub should_show360_warning: bool,
    pub agreed_to_eula: bool,
    pub did_select_language: bool,
    pub agreed_to_multiplayer_disclaimer: bool,
    pub avatar_created: bool,
    pub did_select_region_version: i64,
    pub player_agreements: PlayerAgreements,
    pub last_selected_beatmap_difficulty: i64,
    pub last_selected_beatmap_characteristic_name: BeatmapCharacteristic,
    pub gameplay_modifiers: GameplayModifiers,
    pub player_specific_settings: PlayerSpecificSettings,
    pub practice_settings: PracticeSettings,
    pub player_all_overall_stats_data: PlayerAllOverallStatsData,
    pub levels_stats_data: Vec<LevelsStatsDatum>,
    pub missions_stats_data: Vec<MissionsStatsDatum>,
    pub showed_mission_help_ids: Vec<Option<serde_json::Value>>,
    pub color_schemes_settings: ColorSchemesSettings,
    pub override_environment_settings: OverrideEnvironmentSettings,
    pub favorites_level_ids: Vec<String>,
    pub multiplayer_mode_settings: MultiplayerModeSettings,
    pub current_dlc_promo_display_count: i64,
    pub current_dlc_promo_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorSchemesSettings {
    pub override_default_colors: bool,
    pub selected_color_scheme_id: String,
    pub color_schemes: Vec<ColorScheme>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorScheme {
    pub color_scheme_id: String,
    pub saber_a_color: Colour,
    pub saber_b_color: Colour,
    pub environment_color0: Colour,
    pub environment_color1: Colour,
    pub obstacles_color: Colour,
    pub environment_color0_boost: Colour,
    pub environment_color1_boost: Colour,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameplayModifiers {
    pub energy_type: i64,
    pub insta_fail: bool,
    pub fail_on_saber_clash: bool,
    pub enabled_obstacle_type: i64,
    pub fast_notes: bool,
    pub strict_angles: bool,
    pub disappearing_arrows: bool,
    pub ghost_notes: bool,
    pub no_bombs: bool,
    pub song_speed: i64,
    pub no_arrows: bool,
    pub no_fail_on0_energy: bool,
    pub pro_mode: bool,
    pub zen_mode: bool,
    pub small_cubes: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LevelsStatsDatum {
    pub level_id: String,
    pub difficulty: isize,
    pub beatmap_characteristic_name: BeatmapCharacteristic,
    pub high_score: i64,
    pub max_combo: i64,
    pub full_combo: bool,
    pub max_rank: i64,
    pub valid_score: bool,
    pub play_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionsStatsDatum {
    pub mission_id: String,
    pub cleared: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultiplayerModeSettings {
    pub create_server_number_of_players: i64,
    pub quick_play_difficulty: String,
    pub quick_play_song_pack_mask: Vec<Option<serde_json::Value>>,
    pub quick_play_song_pack_mask_serialized_name: String,
    pub quick_play_enable_level_selection: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OverrideEnvironmentSettings {
    pub override_environments: bool,
    pub override_normal_environment_name: String,
    pub override360_environment_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAgreements {
    pub eula_version: i64,
    pub privacy_policy_version: i64,
    pub health_and_safety_version: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAllOverallStatsData {
    pub campaign_overall_stats_data: HashMap<String, f64>,
    pub solo_free_play_overall_stats_data: HashMap<String, f64>,
    pub party_free_play_overall_stats_data: HashMap<String, f64>,
    pub online_play_overall_stats_data: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSpecificSettings {
    pub static_lights: bool,
    pub left_handed: bool,
    pub player_height: f64,
    pub automatic_player_height: bool,
    pub sfx_volume: f64,
    pub reduce_debris: bool,
    pub no_texts_and_huds: bool,
    pub advanced_hud: bool,
    pub saber_trail_intensity: f64,
    #[serde(rename = "_noteJumpDurationTypeSettingsSaveData")]
    pub note_jump_duration_type_settings_save_data: i64,
    pub note_jump_fixed_duration: f64,
    pub auto_restart: bool,
    pub no_fail_effects: bool,
    pub note_jump_beat_offset: f64,
    pub hide_note_spawn_effect: bool,
    pub adaptive_sfx: bool,
    pub arcs_haptic_feedback: Option<bool>,
    pub arcs_visible_save_data: Option<i64>,
    pub environment_effects_filter_default_preset: i64,
    pub environment_effects_filter_expert_plus_preset: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PracticeSettings {
    pub start_song_time: f64,
    pub song_speed_mul: f64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Hash, PartialEq, Eq)]
pub enum BeatmapCharacteristic {
    #[default]
    Standard,
    Lawless,
    Lightshow,
    NoArrows,
    OneSaber,
    #[serde(rename = "360Degree")]
    Degrees360,
    #[serde(rename = "90Degree")]
    Degrees90,
    MissingCharacteristic,
}
