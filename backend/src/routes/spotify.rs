use rocket::{get, serde::json::Json, tokio::sync::Mutex, State};
use spotify::{types::Tracks, Spotify};

#[get("/tracks")]
pub async fn get_tracks(spotify: &State<Mutex<Spotify>>) -> Json<Tracks> {
    let mut spotify = spotify.lock().await;
    spotify.refresh_token().await.unwrap();
    let tracks = spotify.tracks().await.unwrap();
    Json(tracks)
}
