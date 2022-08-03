use std::env;

use rocket::{self, fs::FileServer, launch, routes, tokio::sync::Mutex};
use routes::{
    spotify::get_tracks,
    steam::{get_info, get_owned, get_recent},
};
use services::spotify::{types::Inner, Spotify};

mod routes;

#[launch]
async fn rocket() -> _ {
    let spotify = match env::var("SPOTIFY") {
        Ok(val) => {
            let inner: Inner = serde_json::from_str(&val).unwrap();
            Spotify::from_inner(inner, "spotify.json").await.unwrap()
        }
        Err(_) => Spotify::new("spotify.json").await.unwrap(),
    };
    let steam = services::steam::Client::new(env::var("STEAM").unwrap());
    let steam_id: u64 = env::var("STEAM_ID").unwrap().parse().unwrap();

    // TODO: separate the api routes
    rocket::build()
        .mount("/", FileServer::from("target/release/static/"))
        .mount(
            "/api/",
            routes![get_tracks, get_owned, get_recent, get_info],
        )
        .manage(Mutex::new(spotify))
        .manage(steam)
        .manage(steam_id)
}

#[cfg(test)]
mod test {
    use services::steam::Client;

    #[rocket::async_test]
    async fn test() {
        let client = Client::new("111");
        let info = client.info(1172470).await;
        println!("{:#?}", info);
    }
}
