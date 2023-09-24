use reqwest::{Error, Response};
use serde::Serialize;

const REAL_SERVER: &'static str = "https://api.spacetraders.io/v2";

pub struct Api {
    client: reqwest::Client,
    url: String,
    token: Option<String>,
}

impl Api {
    pub fn new() -> Self {
        Api {
            client: reqwest::Client::new(),
            url: REAL_SERVER.to_string(),
            token: None,
        }
    }

    #[cfg(test)]
    pub fn test(url: String) -> Self {
        Api {
            client: reqwest::Client::new(),
            url: url,
            token: None,
        }
    }

    async fn post<T: Serialize + ?Sized>(
        &self,
        endpoint: &String,
        body: &T,
    ) -> Result<Response, Error> {
        self.client
            .post(&format!("{}/{}", &self.url, endpoint))
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await
    }
}

pub mod login;
