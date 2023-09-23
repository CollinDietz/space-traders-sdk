use reqwest::Error;
use serde_derive::{Deserialize, Serialize};

use crate::agent::Agent;
use crate::api::Api;
use crate::contract::Contract;
use crate::faction::{Faction, Factions};
use crate::ship::Ship;

#[derive(Debug, PartialEq, Serialize)]
pub struct RegistrationRequest {
    #[serde(rename = "symbol")]
    pub callsign: String,
    pub faction: Factions,
}

#[derive(Debug, PartialEq, Deserialize)]

pub struct LoginData {
    pub agent: Agent,
    pub contract: Contract,
    pub faction: Faction,
    pub ship: Ship,
    pub token: String,
}

impl Api {
    pub async fn register(&self, request: RegistrationRequest) -> Result<LoginData, Error> {
        self.client
            .post("https://api.spacetraders.io/v2")
            .header("Accept", "application/json")
            .json(&serde_json::to_string(&request).unwrap())
            .send()
            .await?
            .json()
            .await
    }
}
