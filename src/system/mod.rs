use std::sync::Arc;

use serde_derive::Deserialize;

use crate::{
    space_traders_client::{Error, SpaceTradersClient},
    system::waypoint::Waypoint,
};

pub mod waypoint;

pub struct System {
    symbol: String,
    client: Arc<SpaceTradersClient>,
}

#[derive(Deserialize)]
pub struct WaypointResponse {
    data: Vec<Waypoint>,
}

impl System {
    pub fn new(client: &Arc<SpaceTradersClient>, symbol: &str) -> Self {
        System {
            client: client.clone(),
            symbol: symbol.into(),
        }
    }

    pub async fn list_waypoints(&self) -> Result<Vec<Waypoint>, Error> {
        let response: WaypointResponse = self
            .client
            .get(
                &format!("systems/{}/waypoints", self.symbol),
                reqwest::StatusCode::OK,
            )
            .await?;

        Ok(response.data)
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use mock_server::{mock_response, RequestMethod};

    use crate::{
        space_traders_client::SpaceTradersClient,
        system::{
            waypoint::tests::{
                some_asteroid, some_asteroid_base, some_engineered_asteroid, some_fuel_station,
                some_planet,
            },
            System,
        },
    };

    #[tokio::test]
    async fn request_should_be_sent_parsed_and_returned() {
        let mock_server = mock_response::<serde_json::Value>(
            RequestMethod::Get,
            "systems/X1-MH3/waypoints",
            200,
            None,
            None,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), "".into()));

        let system = System::new(&client, "X1-MH3");

        let actual = system.list_waypoints().await.unwrap();

        let expected = vec![
            some_planet(),
            some_engineered_asteroid(),
            some_fuel_station(),
            some_asteroid_base(),
            some_asteroid(),
        ];

        assert_eq!(expected, actual)
    }
}
