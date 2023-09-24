use reqwest::{Error, Response};
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct Api {
    client: reqwest::Client,
    url: String,
    token: String,
}

impl Api {
    async fn post<T: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response, Error> {
        self.client
            .post(&format!("{}/{}", self.url, endpoint))
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await
    }
}

pub mod login;
