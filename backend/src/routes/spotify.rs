use rocket::{get, serde::json::Json, tokio::sync::Mutex, State};
use services::spotify::{types::Tracks, Spotify};

#[get("/spotify/tracks")]
pub async fn get_tracks(spotify: &State<Mutex<Spotify>>) -> Json<Tracks> {
    let mut spotify = spotify.lock().await;
    spotify.refresh_token().await.unwrap();
    let tracks = spotify.tracks().await.unwrap();
    Json(tracks)
}
