use std::sync::Arc;

use derivative::Derivative;
use reqwest::{Error, Response, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

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

    async fn _get(
        client: &reqwest::Client,
        url: &str,
        endpoint: &str,
        token: &str,
    ) -> Result<Response, Error> {
        client
            .get(&format!("{}/{}", url, endpoint))
            .bearer_auth(token)
            .send()
            .await
    }

    async fn internal_post<T: Serialize + ?Sized, R: DeserializeOwned>(
        client: &reqwest::Client,
        url: &str,
        endpoint: &str,
        token: &str,
        body: Option<&T>,
        success_status: StatusCode,
    ) -> Result<R, Error> {
        let mut request = client
            .post(&format!("{}/{}", url, endpoint))
            .bearer_auth(token)
            .header("Accept", "application/json");

        if let Some(body) = body {
            request = request.json(body);
        };

        let result = request.send().await;

        match result {
            Ok(response) => {
                if response.status() == success_status {
                    Ok(response.json().await?)
                } else {
                    todo!()
                }
            }
            Err(_error) => {
                todo!()
            }
        }
    }

    pub async fn post<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        success_status: StatusCode,
    ) -> Result<R, Error> {
        SpaceTradersClient::internal_post::<serde_json::Value, R>(
            &self.client,
            &self.url,
            endpoint,
            &self.token,
            None,
            success_status,
        )
        .await
    }

    pub async fn post_with_body<T: Serialize + ?Sized, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &T,
        success_status: StatusCode,
    ) -> Result<R, Error> {
        SpaceTradersClient::internal_post(
            &self.client,
            &self.url,
            endpoint,
            &self.token,
            Some(body),
            success_status,
        )
        .await
    }
}

// TODO: Add tests
