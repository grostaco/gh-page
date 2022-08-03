use services::spotify::Tracks;

use crate::error::Error;

// https://grostaco.herokuapp.com/api/spotify/tracks
pub async fn get_tracks() -> Result<Tracks, Error> {
    let res = reqwest::get("https://grostaco.herokuapp.com/api/spotify/tracks")
        .await
        .map_err(|_| Error::ReqestError)?;
    let tracks: Tracks =
        serde_json::from_str(&res.text().await.map_err(|_| Error::ReqestError)?).unwrap();
    Ok(tracks)
}
