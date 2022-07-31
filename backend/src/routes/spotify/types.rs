use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tracks {
    href: String,
    items: Vec<TrackItems>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackItems {
    added_at: String,
    track: Track,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenRes {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub expires_in: u64,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    height: u32,
    url: String,
    width: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    album: Album,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    disc_number: u64,
    duration_ms: u64,
    explicit: bool,
    external_ids: Value,
    external_urls: Value,
    href: String,
    id: String,
    is_local: bool,
    name: String,
    popularity: u64,
    preview_url: Option<String>,
    track_number: u64,
    #[serde(rename = "type")]
    ty: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    name: String,
    #[serde(rename = "type")]
    ty: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    album_type: String,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    external_urls: HashMap<String, String>,
    href: String,
    id: String,
    images: Vec<Image>,
    name: String,
    release_date: String,
    release_date_precision: String,
    total_tracks: u64,
    #[serde(rename = "type")]
    ty: String,
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Albums {
    href: String,
    items: Value,
    limit: u64,
    next: Option<String>,
    offset: u64,
    previous: Option<String>,
    total: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inner {
    pub client_id: String,
    pub client_secret: String,
    pub access_token: String,
    pub refresh_token: String,
    pub last_requested: u64,
}
