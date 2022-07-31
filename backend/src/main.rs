use rocket::{self, launch};

mod routes;
use routes::spotify::Spotify;

#[launch]
fn rocket() -> _ {
    rocket::build()
}
