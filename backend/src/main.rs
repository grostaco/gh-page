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
    let port: u64 = std::env::var("PORT")
        .map(|p| p.parse().unwrap())
        .unwrap_or(8080);

    let figment = Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));
    rocket::custom(figment)
        .mount("/", FileServer::from("backend/static/"))
        .mount("/api/spotify", routes![get_tracks])
        .manage(Mutex::new(spotify))
}
