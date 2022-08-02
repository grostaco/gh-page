use rocket::{self, figment::Figment, fs::FileServer, launch, routes, tokio::sync::Mutex, Config};
use routes::spotify::get_tracks;
use spotify::{types::Inner, Spotify};

mod routes;

#[launch]
async fn rocket() -> _ {
    let spotify = match std::env::var("SPOTIFY") {
        Ok(val) => {
            let inner: Inner = serde_json::from_str(&val).unwrap();
            Spotify::from_inner(inner, "spotify.json").await.unwrap()
        }
        Err(_) => Spotify::new("spotify.json").await.unwrap(),
    };
    rocket::build()
        .mount("/", FileServer::from("backend/static/"))
        .mount("/api/spotify", routes![get_tracks])
        .manage(Mutex::new(spotify))
}
