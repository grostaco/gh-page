use log::info;
use serde_json::json;
use spotify::Tracks;

use crate::error::Error;

pub async fn get_tracks() -> Result<Tracks, Error> {
    let res = reqwest::get("http://127.0.0.1:8000/api/spotify/tracks")
        .await
        .map_err(|_| Error::ReqestError)?;
    let tracks: Tracks =
        serde_json::from_str(&res.text().await.map_err(|_| Error::ReqestError)?).unwrap();
    Ok(tracks)
}
