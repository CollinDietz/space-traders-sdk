use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

use crate::{
    space_traders_client::{Error, SpaceTradersClient},
    system::waypoint::{Waypoint, WaypointData, WaypointTraitSymbol, WaypointType},
};

pub mod waypoint;

pub struct System {
    symbol: String,
    client: Arc<SpaceTradersClient>,
}

#[derive(Deserialize)]
pub struct WaypointResponse {
    data: Vec<WaypointData>,
}

#[derive(Serialize)]
struct ListWayPointsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<WaypointType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#trait: Option<WaypointTraitSymbol>,
}

impl System {
    pub fn new(client: Arc<SpaceTradersClient>, symbol: &str) -> Self {
        System {
            client: client,
            symbol: symbol.into(),
        }
    }

    pub async fn list_waypoints(
        &self,
        waypoint_type: Option<WaypointType>,
        waypoint_trait: Option<WaypointTraitSymbol>,
    ) -> Result<Vec<Waypoint>, Error> {
        let query_params = ListWayPointsParams {
            r#type: waypoint_type,
            r#trait: waypoint_trait,
        };

        let response: WaypointResponse = self
            .client
            .get(
                &format!("systems/{}/waypoints", self.symbol),
                Some(&query_params),
                reqwest::StatusCode::OK,
            )
            .await?;

        Ok(response
            .data
            .into_iter()
            .map(|data| Waypoint::new(self.client.clone(), data))
            .collect())
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use mock_server::{mock_response, RequestMethod};

    use crate::{
        space_traders_client::SpaceTradersClient,
        system::{
            waypoint::{
                tests::{
                    some_asteroid, some_asteroid_base, some_engineered_asteroid, some_fuel_station,
                    some_moon, some_planet,
                },
                Waypoint, WaypointTraitSymbol, WaypointType,
            },
            System,
        },
    };

    #[tokio::test]
    async fn list_way_points_request_should_be_sent_parsed_and_returned() {
        let mock_server = mock_response::<serde_json::Value>(
            RequestMethod::Get,
            "systems/X1-MH3/waypoints",
            200,
            None,
            None,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let system = System::new(client.clone(), "X1-MH3");

        let actual = system.list_waypoints(None, None).await.unwrap();

        let expected = vec![
            Waypoint::new(client.clone(), some_planet()),
            Waypoint::new(client.clone(), some_engineered_asteroid()),
            Waypoint::new(client.clone(), some_fuel_station()),
            Waypoint::new(client.clone(), some_asteroid_base()),
            Waypoint::new(client.clone(), some_asteroid()),
        ];

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn list_way_points_with_planet_type_request_should_be_sent_parsed_and_returned() {
        let mock_server = mock_response(
            RequestMethod::Get,
            "systems/X1-MH3/waypoints",
            200,
            None,
            Some(&[("type", "PLANET")]),
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let system = System::new(client.clone(), "X1-MH3");

        let actual = system
            .list_waypoints(Some(WaypointType::Planet), None)
            .await
            .unwrap();

        let expected = vec![Waypoint::new(client.clone(), some_planet())];

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn list_way_points_with_trait_shipyard_request_should_be_sent_parsed_and_returned() {
        let mock_server = mock_response(
            RequestMethod::Get,
            "systems/X1-MH3/waypoints",
            200,
            None,
            Some(&[("trait", "SHIPYARD")]),
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let system = System::new(client.clone(), "X1-MH3");

        let actual = system
            .list_waypoints(None, Some(WaypointTraitSymbol::Shipyard))
            .await
            .unwrap();

        let expected = vec![Waypoint::new(client.clone(), some_moon())];

        assert_eq!(expected, actual)
    }
}
