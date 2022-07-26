use std::str::FromStr;

use reqwest::ClientBuilder;
use serde_json::Value;

use super::{
    errors::Error,
    types::{OwnedGames, RecentGames},
    GameDetail,
};

pub struct Client {
    client: reqwest::Client,
    api_key: String,
}

#[cfg(feature = "brotli")]
#[allow(dead_code)]
#[inline]
fn enable_brotli(builder: ClientBuilder) -> ClientBuilder {
    builder.brotli(true).http2_prior_knowledge()
}

#[cfg(not(feature = "brotli"))]
#[inline]
fn enable_brotli(builder: ClientBuilder) -> ClientBuilder {
    builder
}

impl Client {
    pub fn new<S: ToString>(api_key: S) -> Self {
        let client = ClientBuilder::new();
        // let client = enable_brotli(client);
        Self {
            client: client.build().unwrap(),
            api_key: api_key.to_string(),
        }
    }
    pub async fn owned_games(&self, steam_id: u64) -> Result<OwnedGames, Error> {
        let response = self
            .client
            .get("http://api.steampowered.com/IPlayerService/GetOwnedGames/v0001/")
            .query(&[
                ("key", self.api_key.as_str()),
                ("steamid", &steam_id.to_string()),
                ("include_appinfo", "true"),
                ("include_played_free_games", "true"),
                ("format", &*"json"),
            ])
            .send()
            .await?;
        let (status, text) = (response.status(), response.text().await?);

        if !status.is_success() {
            return Err(Error::UnexpectedStatus(status.as_u16()));
        }
        let content = match Value::from_str(&text)? {
            Value::Object(mut obj) => obj.remove("response").unwrap(),
            _ => return Err(Error::MissingResponse),
        };

        Ok(serde_json::from_value(content)?)
    }

    pub async fn recent_games(
        &self,
        steam_id: u64,
        count: Option<u64>,
    ) -> Result<RecentGames, Error> {
        let mut request = self
            .client
            .get("http://api.steampowered.com/IPlayerService/GetRecentlyPlayedGames/v0001/")
            .query(&[
                ("key", self.api_key.as_str()),
                ("format", &*"json"),
                ("steamid", &steam_id.to_string()),
            ]);

        if let Some(count) = count {
            request = request.query(&[("count", count)]);
        }

        let response = request.send().await?;

        let (status, text) = (response.status(), response.text().await?);

        if !status.is_success() {
            return Err(Error::UnexpectedStatus(status.as_u16()));
        }
        let content = match Value::from_str(&text)? {
            Value::Object(mut obj) => obj.remove("response").unwrap(),
            _ => return Err(Error::MissingResponse),
        };
        Ok(serde_json::from_value(content)?)
    }

    pub async fn info(&self, appid: u64) -> Result<GameDetail, Error> {
        let request = self
            .client
            .get("https://store.steampowered.com/api/appdetails")
            .query(&[("filters", "basic"), ("appids", &appid.to_string())]);

        let response = request.send().await?;

        let (status, text) = (response.status(), response.text().await?);

        let content = match Value::from_str(&text)? {
            Value::Object(mut obj) => match obj.remove(&appid.to_string()).unwrap() {
                Value::Object(mut obj) => obj.remove("data").unwrap(),
                _ => return Err(Error::MissingResponse),
            },
            _ => return Err(Error::MissingResponse),
        };

        if !status.is_success() {
            return Err(Error::UnexpectedStatus(status.as_u16()));
        }
        Ok(serde_json::from_value(content)?)
    }
}
