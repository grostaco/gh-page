use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct OwnedGames {
    pub game_count: u64,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RecentGames {
    pub total_count: u64,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub appid: u64,
    pub name: String,
    #[serde(default)]
    pub playtime_2weeks: u64,
    pub playtime_forever: u64,
    #[serde(default)]
    pub img_icon_url: String,
    #[serde(default)]
    pub img_logo_url: String,
    #[serde(default)]
    pub has_community_visible_stats: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GameDetail {
    #[serde(rename = "type")]
    pub ty: String,
    pub name: String,
    pub steam_appid: u64,
    pub required_age: u64,
    #[serde(default)]
    pub is_free: bool,
    pub controller_support: String,
    #[serde(default)]
    pub dlc: Vec<u64>,
    pub detailed_description: String,
    pub about_the_game: String,
    pub short_description: String,
    pub header_image: String,
    pub website: String,
    pub pc_requirements: Value,
    pub mac_requirements: Value,
    pub linux_requirements: Value,
    pub supported_languages: String,
    // #[serde(default)]
    // fullgame: Value,

    // #[serde(default)]
    // legal_notice: Vec<String>,
    // #[serde(default)]
    // developers: Vec<String>,
    // publishers: Vec<String>,
    // #[serde(default)]
    // demos: Value,
    // #[serde(default)]
    // price_overview: Value,
}
