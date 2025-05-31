use std::collections::HashMap;

use reqwest::{Error, Response};
use serde::Serialize;

const REAL_SERVER: &'static str = "https://api.spacetraders.io/v2";

#[derive(Debug, Clone)]
pub struct Sdk {
    client: reqwest::Client,
    url: String,
    account_token: String,
    agent_tokens: HashMap<String, String>,
}

impl Sdk {
    pub fn new(account_token: String) -> Self {
        Sdk {
            client: reqwest::Client::new(),
            url: REAL_SERVER.to_string(),
            account_token,
            agent_tokens: HashMap::new(),
        }
    }

    pub fn with_url(url: String, account_token: String) -> Self {
        Sdk {
            client: reqwest::Client::new(),
            url,
            account_token,
            agent_tokens: HashMap::new(),
        }
    }

    pub fn add_agent_token(&mut self, callsign: String, token: String) {
        self.agent_tokens.insert(callsign, token);
    }

    async fn post(&self, endpoint: &str, token: &str) -> Result<Response, Error> {
        self.client
            .post(&format!("{}/{}", self.url, endpoint))
            .bearer_auth(token)
            .header("Accept", "application/json")
            .send()
            .await
    }

    async fn post_with_body<T: Serialize + ?Sized>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<Response, Error> {
        self.client
            .post(&format!("{}/{}", self.url, endpoint))
            .bearer_auth(&self.account_token)
            .header("Accept", "application/json")
            .json(body)
            .send()
            .await
    }
}

pub mod agents;
pub mod login;
