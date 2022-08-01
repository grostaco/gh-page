pub mod errors;
pub mod types;

use std::{
    fs::{read_to_string, OpenOptions},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use reqwest::{Client, StatusCode};
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
    inner: Arc<Mutex<Inner>>,
    path: PathBuf,
    client: Client,
}

impl Spotify {
    pub async fn new<P: AsRef<Path> + Into<PathBuf>>(path: P) -> Result<Self, SpotifyError> {
        let inner: Inner = serde_json::from_str(&read_to_string(&path).unwrap()).unwrap();
        let spotify = Self {
            inner: Arc::new(Mutex::new(inner)),
            path: path.into(),
            client: Client::new(),
        };

        spotify.refresh_token().await?;

        Ok(spotify)
    }

    pub async fn update_file(&mut self) {}

    pub async fn refresh_token(&self) -> Result<(), SpotifyError> {
        let mut inner = self.inner.lock().unwrap();
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

        serde_json::to_writer_pretty(
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&self.path)
                .unwrap(),
            &*inner,
        )
        .unwrap();

        Ok(())
    }

    pub async fn tracks(&self) -> Result<Tracks, SpotifyError> {
        self.refresh_token().await?;

        let response = self
            .client
            .get(spotify_url!("/me/tracks"))
            .header(
                "Authorization",
                format!("Bearer {}", self.inner.lock().unwrap().access_token),
            )
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let status = response.status();
        let content = response.text().await?;

        let result: Tracks = handle_res_error!(status, &content)?;

        Ok(result)
    }
}

// #[cfg(test)]
// mod test {
//     use super::Spotify;

//     #[rocket::async_test]
//     async fn test() {
//         println!("{:#?}", std::env::current_dir());
//         let spotify = Spotify::new("../spotify.json").await.unwrap();

//         println!("{:#?}", spotify.tracks().await);
//     }
// }
