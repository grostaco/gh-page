use rocket::{get, serde::json::Json, State};
use services::steam;

#[get("/steam/owned")]
pub async fn get_owned(
    steam: &State<steam::Client>,
    steam_id: &State<u64>,
) -> Json<steam::OwnedGames> {
    Json(steam.owned_games(*steam_id.inner()).await.unwrap())
}

#[get("/steam/recent")]
pub async fn get_recent(
    steam: &State<steam::Client>,
    steam_id: &State<u64>,
) -> Json<steam::RecentGames> {
    Json(steam.recent_games(*steam_id.inner(), None).await.unwrap())
}

#[get("/steam/info/<id>")]
pub async fn get_info(steam: &State<steam::Client>, id: u64) -> Json<steam::GameDetail> {
    Json(steam.info(id).await.unwrap())
}
