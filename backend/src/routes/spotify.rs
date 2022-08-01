use rocket::{get, serde::json::Json, State};
use spotify::{types::Tracks, Spotify};

// #[get("/tracks")]
// pub async fn get_tracks(spotify: &State<Spotify>) -> Json<Tracks> {
//     let tracks = spotify.tracks().await.unwrap();
//     Json(tracks)
// }
