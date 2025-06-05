use std::sync::Arc;

use derivative::Derivative;
use reqwest::{Error, Response};
use serde::Serialize;

use crate::utils;

const REAL_SERVER: &'static str = "https://api.spacetraders.io/v2";

#[derive(Debug, Derivative, Clone)]
#[derivative(PartialEq)]
pub struct SpaceTradersClient {
    #[derivative(PartialEq = "ignore")]
    client: Arc<reqwest::Client>,
    pub url: String,
    token: String,
}

impl SpaceTradersClient {
    pub fn new(token: &str) -> Self {
        SpaceTradersClient {
            client: Arc::new(reqwest::Client::new()),
            url: REAL_SERVER.to_string(),
            token: token.to_string(),
        }
    }

    pub fn clone_with_token(client: &SpaceTradersClient, new_token: &str) -> Self {
        SpaceTradersClient {
            token: new_token.to_string(),
            ..client.clone()
        }
    }

    pub fn with_url(url: &str, token: &str) -> Self {
        SpaceTradersClient {
            client: Arc::new(reqwest::Client::new()),
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
