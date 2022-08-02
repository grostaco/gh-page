pub mod errors;
pub mod types;

use std::{
    fs::{read_to_string, OpenOptions},
    path::{Path, PathBuf},
};

use reqwest::{Client, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use self::{
    errors::SpotifyError,
    types::{Credentials, Inner},
};

pub use chrono;
pub use types::Tracks;

macro_rules! spotify_url {
    ($path:expr) => {
        format!("https://api.spotify.com/v1{}", $path)
    };
}

const EXPIRE_TIME: u64 = 3600;
const INTERVAL_REFRESH: u64 = 300;

macro_rules! handle_res_error {
    ($status:expr, $body:expr) => {
        match $status {
            StatusCode::OK => Ok(serde_json::from_str($body).unwrap()),
            StatusCode::UNAUTHORIZED => Err(SpotifyError::Unauthorized(
                serde_json::from_str($body).unwrap(),
            )),
            StatusCode::FORBIDDEN => Err(SpotifyError::Forbidden(
                serde_json::from_str($body).unwrap(),
            )),
            StatusCode::TOO_MANY_REQUESTS => Err(SpotifyError::RateLimited(
                serde_json::from_str($body).unwrap(),
            )),
            code => panic!("Unexpected status code {}", code),
        }
    };
}

pub struct Spotify {
    inner: Inner,
    path: PathBuf,
    client: Client,
}

impl Spotify {
    pub async fn new<P: AsRef<Path> + Into<PathBuf>>(path: P) -> Result<Self, SpotifyError> {
        let inner: Inner = serde_json::from_str(&read_to_string(&path).unwrap()).unwrap();
        let mut spotify = Self {
            inner,
            path: path.into(),
            client: Client::new(),
        };

        spotify.refresh_token().await?;

        Ok(spotify)
    }

    pub async fn from_inner<P: Into<PathBuf>>(inner: Inner, path: P) -> Result<Self, SpotifyError> {
        let mut spotify = Self {
            inner,
            path: path.into(),
            client: Client::new(),
        };

        spotify.refresh_token().await?;

        Ok(spotify)
    }

    pub fn update_file(&mut self) {
        serde_json::to_writer_pretty(
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.path)
                .unwrap(),
            &self.inner,
        )
        .unwrap();
    }

    pub fn update_cache<T: Serialize>(&mut self, src: &str, response: T, update_file: bool) {
        self.inner.cached_responses.insert(
            src.to_string(),
            (
                chrono::Utc::now().timestamp() as u64,
                serde_json::to_value(response).unwrap(),
            ),
        );
        if update_file {
            self.update_file()
        }
    }

    pub fn get_cache<'de, T: DeserializeOwned>(&mut self, src: &str) -> Option<(bool, T)> {
        let (ts, val) = self.inner.cached_responses.get(src)?;
        let needs_refresh = chrono::Utc::now().timestamp() as u64 - *ts > INTERVAL_REFRESH;
        let val: T = serde_json::from_value(val.clone()).ok()?;
        Some((needs_refresh, val))
    }

    pub async fn refresh_token(&mut self) -> Result<(), SpotifyError> {
        let mut inner = &mut self.inner;
        let ts = chrono::Utc::now().timestamp() as u64;
        if ts - EXPIRE_TIME < inner.last_requested {
            return Ok(());
        }

        let params = json!({"grant_type": "refresh_token", "refresh_token": inner.refresh_token});
        let response = self
            .client
            .post("https://accounts.spotify.com/api/token")
            .header(
                "Authorization",
                format!(
                    "Basic {}",
                    base64::encode(format!("{}:{}", inner.client_id, inner.client_secret))
                ),
            )
            .form(&params)
            .send()
            .await?;
        let status = response.status();
        let content = response.text().await?;

        let cred: Credentials = handle_res_error!(status, &content)?;
        inner.access_token = cred.access_token;
        inner.last_requested = ts;

        self.update_file();

        Ok(())
    }

    pub async fn tracks(&mut self) -> Result<Tracks, SpotifyError> {
        self.refresh_token().await.unwrap();
        if let Some((refresh, tracks)) = self.get_cache::<Tracks>("tracks") {
            if !refresh {
                return Ok(tracks);
            }
        }

        let inner = &self.inner;
        let response = self
            .client
            .get(spotify_url!("/me/tracks"))
            .header("Authorization", format!("Bearer {}", inner.access_token))
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let status = response.status();
        let content = response.text().await?;

        let result: Tracks = handle_res_error!(status, &content)?;
        self.update_cache("tracks", &result, true);

        Ok(result)
    }
}

// #[cfg(test)]
// mod test {
//     use super::Spotify;

//     #[rocket::async_test]
//     async fn test() {
//         println!("{:#?}", std::env::current_dir());
//         let mut spotify = Spotify::new("../spotify.json").await.unwrap();

//         println!("{:#?}", spotify.tracks().await);
//     }
// }
