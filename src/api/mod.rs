use reqwest::{Error, Response};
use serde::Serialize;

const REAL_SERVER: &'static str = "https://api.spacetraders.io/v2";

#[derive(Debug, Clone)]
pub struct Api {
    client: reqwest::Client,
    url: String,
    token: String,
}

impl Api {
    pub fn new(token: String) -> Self {
        Api {
            client: reqwest::Client::new(),
            url: REAL_SERVER.to_string(),
            token,
        }
    }

    pub fn with_url(url: String, token: String) -> Self {
        Api {
            client: reqwest::Client::new(),
            url,
            token,
        }
    }

    async fn post<T: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response, Error> {
        self.client
            .post(&format!("{}/{}", self.url, endpoint))
            .bearer_auth(&self.token)
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await
    }
}

pub mod login;
