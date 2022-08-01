use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Tracks {
    pub href: String,
    pub items: Vec<TrackItems>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TrackItems {
    pub added_at: String,
    pub track: Track,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct AccessTokenRes {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub height: u32,
    pub url: String,
    pub width: u32,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: u64,
    pub duration_ms: u64,
    pub explicit: bool,
    pub external_ids: Value,
    pub external_urls: Value,
    pub href: String,
    pub id: String,
    pub is_local: bool,
    pub name: String,
    pub popularity: u64,
    pub preview_url: Option<String>,
    pub track_number: u64,
    #[serde(rename = "type")]
    pub ty: String,
    pub uri: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub uri: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub album_type: String,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub external_urls: HashMap<String, String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub total_tracks: u64,
    #[serde(rename = "type")]
    pub ty: String,
    pub uri: String,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Albums {
    pub href: String,
    pub items: Value,
    pub limit: u64,
    pub next: Option<String>,
    pub offset: u64,
    pub previous: Option<String>,
    pub total: u64,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Inner {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
    pub last_requested: u64,
    pub cached_responses: Option<Value>,
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TrackResponse {
    name: String,
    added_at: u64,
    icon: String,
    artist: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CachedResponse<T>
where
    T: Serialize,
{
    response: T,
    time: u64,
}
