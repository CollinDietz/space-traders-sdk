use reqwest::Error;
use serde_json::{json, Value};

use crate::api::Api;
use crate::faction::Factions;

impl Api {
    pub async fn register(&self, callsign: String, faction: Factions) -> Result<String, Error> {
        // let request_body = json!({
        //     "faction": faction.to_string(),
        //     "symbol": callsign,
        // });

        // let data: String = self
        //     .client
        //     .post("https://stoplight.io/mocks/spacetraders/spacetraders/96627693/register")
        //     .header("Accept", "application/json")
        //     .json(&request_body)
        //     .send()
        //     .await?
        //     .text()
        //     .await?
        //     .to_string();

        // let json: Value = serde_json::from_str(data.as_str()).unwrap();
        // // extract_keys(&json);
        // // let agent: Agent = serde_json::from_str(json["data"]["agent"].to_string().as_str()).unwrap();
        // println!("{:?}", json["data"]["agent"].to_string());

        // let token = &json["data"]["token"];
        // println!("{}", token);

        Ok("Hi".to_string())
    }
}
