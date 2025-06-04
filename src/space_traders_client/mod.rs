use derivative::Derivative;
use reqwest::{Error, Response};
use serde::Serialize;

use crate::utils;

#[derive(Debug, Derivative)]
#[derivative(PartialEq)]
pub struct SpaceTradersClient {
    #[derivative(PartialEq = "ignore")]
    client: reqwest::Client,
    pub url: String,
    token: String,
}

impl SpaceTradersClient {
    pub fn new(url: &str, token: &str) -> Self {
        SpaceTradersClient {
            client: reqwest::Client::new(),
            url: url.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn post(&self, endpoint: &str) -> Result<Response, Error> {
        utils::post::<serde_json::Value>(&self.client, &self.url, endpoint, &self.token, None).await
    }

    pub async fn post_with_body<T: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response, Error> {
        utils::post(&self.client, &self.url, endpoint, &self.token, Some(body)).await
    }
}
