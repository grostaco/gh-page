use rocket::{self, fs::FileServer, launch, routes, tokio::sync::Mutex};
use routes::spotify::get_tracks;
use spotify::Spotify;

mod routes;

#[launch]
async fn rocket() -> _ {
    let spotify = Spotify::new("spotify.json").await.unwrap();
    rocket::build()
        .mount("/", FileServer::from("backend/static/"))
        .mount("/api/spotify", routes![get_tracks])
        .manage(Mutex::new(spotify))
}
