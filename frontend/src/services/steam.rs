use services::steam::{GameDetail, OwnedGames, RecentGames};

use crate::{error::Error, route};

pub async fn get_owned_games() -> Result<OwnedGames, Error> {
    let res = reqwest::get(route!("/api/steam/owned"))
        .await
        .map_err(|_| Error::ReqestError)?;
    let games: OwnedGames =
        serde_json::from_str(&res.text().await.map_err(|_| Error::ReqestError)?).unwrap();
    Ok(games)
}

pub async fn get_recent_games() -> Result<RecentGames, Error> {
    let res = reqwest::get(route!("/api/steam/recent"))
        .await
        .map_err(|_| Error::ReqestError)?;
    let games: RecentGames =
        serde_json::from_str(&res.text().await.map_err(|_| Error::ReqestError)?).unwrap();
    Ok(games)
}

pub async fn get_game_infos(ids: Vec<u64>) -> Result<Vec<GameDetail>, Error> {
    let mut games = Vec::with_capacity(ids.len());
    for id in ids {
        let res = reqwest::get(format!("{}/{}", route!("/api/steam/info"), id))
            .await
            .map_err(|_| Error::ReqestError)?;
        games.push(
            serde_json::from_str(&res.text().await.map_err(|_| Error::ReqestError)?).unwrap(),
        );
    }

    Ok(games)
}
