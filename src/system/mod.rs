use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

use crate::{
    faction::Factions,
    space_traders_client::{Error, SpaceTradersClient},
    system::waypoint::{
        Waypoint, WaypointData, WaypointOrbital, WaypointTraitSymbol, WaypointType,
    },
};

pub mod waypoint;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemData {
    pub symbol: String,
    pub sector_symbol: String,
    pub constellation: Option<String>,
    pub name: Option<String>,
    pub r#type: SystemType,
    pub x: i32,
    pub y: i32,
    pub waypoints: Vec<SystemWaypoint>,
    pub factions: Vec<SystemFaction>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemType {
    NeutronStar,
    RedStar,
    OrangeStar,
    BlueStar,
    YoungStar,
    WhiteDwarf,
    BlackHole,
    Hypergiant,
    Nebula,
    Unstable,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SystemWaypoint {
    pub symbol: String,
    pub r#type: WaypointType,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<WaypointOrbital>,
    pub orbits: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SystemFaction {
    symbol: Factions,
}

#[derive(Debug, Clone, PartialEq)]
pub struct System {
    symbol: String,
    data: Option<SystemData>,
    client: Arc<SpaceTradersClient>,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct SystemResponse {
    pub data: SystemData,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ListSystemResponse {
    pub data: Vec<SystemData>,
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

#[derive(Serialize)]
struct ListSystemParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u8>,
}

impl System {
    pub async fn list_systems(
        client: &Arc<SpaceTradersClient>,
        page: Option<u8>,
        limit: Option<u8>,
    ) -> Result<Vec<System>, Error> {
        let params = ListSystemParams { page, limit };
        let response: ListSystemResponse = client
            .get("systems", Some(&params), reqwest::StatusCode::OK)
            .await?;

        Ok(response
            .data
            .into_iter()
            .map(|data| System::with_data(client.clone(), data))
            .collect())
    }

    pub async fn get_system_data(
        client: &SpaceTradersClient,
        symbol: &str,
    ) -> Result<SystemData, Error> {
        let response: SystemResponse = client
            .get(
                &format!("systems/{}", symbol),
                None::<&()>,
                reqwest::StatusCode::OK,
            )
            .await?;

        Ok(response.data)
    }

    pub async fn get_system(
        client: Arc<SpaceTradersClient>,
        symbol: &str,
    ) -> Result<System, Error> {
        let data = System::get_system_data(&client, symbol).await?;
        Ok(System::with_data(client, data))
    }

    pub async fn list_system_waypoints(
        client: &Arc<SpaceTradersClient>,
        symbol: &str,
        waypoint_type: Option<WaypointType>,
        waypoint_trait: Option<WaypointTraitSymbol>,
    ) -> Result<Vec<Waypoint>, Error> {
        let query_params = ListWayPointsParams {
            r#type: waypoint_type,
            r#trait: waypoint_trait,
        };

        let response: WaypointResponse = client
            .get(
                &format!("systems/{}/waypoints", symbol),
                Some(&query_params),
                reqwest::StatusCode::OK,
            )
            .await?;

        Ok(response
            .data
            .into_iter()
            .map(|data| Waypoint::with_data(client.clone(), data))
            .collect())
    }
}

impl System {
    pub fn new(client: Arc<SpaceTradersClient>, symbol: &str) -> Self {
        System {
            client: client,
            data: None,
            symbol: symbol.into(),
        }
    }

    pub fn with_data(client: Arc<SpaceTradersClient>, data: SystemData) -> Self {
        System {
            symbol: data.symbol.clone(),
            data: Some(data),
            client,
        }
    }

    pub async fn get_data(&mut self) -> Result<SystemData, Error> {
        let data = match &self.data {
            Some(cached) => cached.clone(),
            None => {
                let fetched = System::get_system_data(&self.client, &self.symbol).await?;
                self.data = Some(fetched.clone());
                fetched
            }
        };

        Ok(data)
    }

    pub async fn list_waypoints(
        &self,
        waypoint_type: Option<WaypointType>,
        waypoint_trait: Option<WaypointTraitSymbol>,
    ) -> Result<Vec<Waypoint>, Error> {
        System::list_system_waypoints(&self.client, &self.symbol, waypoint_type, waypoint_trait)
            .await
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use mock_server::{MockServerBuilder, RequestMethod};

    use crate::{
        space_traders_client::SpaceTradersClient,
        string,
        system::{
            waypoint::{
                tests::{
                    some_asteroid, some_asteroid_base, some_engineered_asteroid, some_fuel_station,
                    some_moon, some_planet,
                },
                Waypoint, WaypointOrbital, WaypointTraitSymbol, WaypointType,
            },
            System, SystemData, SystemResponse, SystemType, SystemWaypoint,
        },
    };

    fn some_system_data() -> SystemData {
        SystemData {
            symbol: string!("X1-AG18"),
            sector_symbol: string!("X1"),
            constellation: Some(string!("Lacerta")),
            name: Some(string!("Borealis")),
            r#type: SystemType::OrangeStar,
            x: 20324,
            y: 7157,
            waypoints: vec![
                SystemWaypoint {
                    symbol: string!("X1-AG18-A1"),
                    r#type: WaypointType::Planet,
                    x: -10,
                    y: -24,
                    orbitals: vec![
                        WaypointOrbital {
                            symbol: string!("X1-AG18-A2"),
                        },
                        WaypointOrbital {
                            symbol: string!("X1-AG18-A3"),
                        },
                        WaypointOrbital {
                            symbol: string!("X1-AG18-A4"),
                        },
                    ],
                    orbits: None,
                },
                SystemWaypoint {
                    symbol: string!("X1-AG18-XX5D"),
                    r#type: WaypointType::EngineeredAsteroid,
                    x: 29,
                    y: -1,
                    orbitals: vec![],
                    orbits: None,
                },
                SystemWaypoint {
                    symbol: string!("X1-AG18-A2"),
                    r#type: WaypointType::Moon,
                    x: -10,
                    y: -24,
                    orbitals: vec![],
                    orbits: Some(string!("X1-AG18-A1")),
                },
            ],
            factions: vec![],
        }
    }

    fn some_system_data_response() -> SystemResponse {
        SystemResponse {
            data: some_system_data(),
        }
    }

    fn some_neutron_star() -> SystemData {
        SystemData {
            symbol: string!("X1-HX62"),
            sector_symbol: string!("X1"),
            constellation: Some(string!("Zhang")),
            name: Some(string!("Kappa")),
            r#type: SystemType::NeutronStar,
            x: 1185,
            y: 234,
            waypoints: vec![],
            factions: vec![],
        }
    }

    fn some_blue_star() -> SystemData {
        SystemData {
            symbol: string!("X1-NC38"),
            sector_symbol: string!("X1"),
            constellation: Some(string!("Zhang")),
            name: Some(string!("Psi")),
            r#type: SystemType::BlueStar,
            x: 1115,
            y: 496,
            waypoints: vec![
                SystemWaypoint {
                    symbol: string!("X1-NC38-FC1B"),
                    r#type: WaypointType::Planet,
                    x: -10,
                    y: -7,
                    orbitals: vec![],
                    orbits: None,
                },
                SystemWaypoint {
                    symbol: string!("X1-NC38-AD2C"),
                    r#type: WaypointType::GasGiant,
                    x: 15,
                    y: 23,
                    orbitals: vec![
                        WaypointOrbital {
                            symbol: string!("X1-NC38-CF3D"),
                        },
                        WaypointOrbital {
                            symbol: string!("X1-NC38-BB4X"),
                        },
                    ],
                    orbits: None,
                },
            ],
            factions: vec![],
        }
    }

    #[tokio::test]
    async fn should_list_systems() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "systems",
            200,
            None,
            Some(&[("page", 1), ("limit", 2)]),
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let actual = System::list_systems(&client, Some(1), Some(2))
            .await
            .unwrap();

        let expected = vec![
            System::with_data(client.clone(), some_neutron_star()),
            System::with_data(client.clone(), some_blue_star()),
        ];

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_get_system_data() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "systems/X1-AG18",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = SpaceTradersClient::with_url(&mock_server.url(), None);

        let actual = System::get_system_data(&client, "X1-AG18").await.unwrap();

        let expected = some_system_data();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_get_system() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "systems/X1-AG18",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let actual = System::get_system(client.clone(), "X1-AG18").await.unwrap();

        let expected = System::with_data(client.clone(), some_system_data());

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_get_system_data_with_object() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "systems/X1-AG18",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let mut system = System::new(client.clone(), "X1-AG18");

        let actual = system.get_data().await.unwrap();

        let expected = some_system_data();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_list_waypoints() {
        let mock_server = MockServerBuilder::mock_once::<serde_json::Value>(
            RequestMethod::Get,
            "systems/X1-MH3/waypoints",
            200,
            None,
            None,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let actual = System::list_system_waypoints(&client, "X1-MH3", None, None)
            .await
            .unwrap();

        let expected = vec![
            Waypoint::with_data(client.clone(), some_planet()),
            Waypoint::with_data(client.clone(), some_engineered_asteroid()),
            Waypoint::with_data(client.clone(), some_fuel_station()),
            Waypoint::with_data(client.clone(), some_asteroid_base()),
            Waypoint::with_data(client.clone(), some_asteroid()),
        ];

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn should_list_waypoints_with_an_object() {
        let mock_server = MockServerBuilder::mock_once::<serde_json::Value>(
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
            Waypoint::with_data(client.clone(), some_planet()),
            Waypoint::with_data(client.clone(), some_engineered_asteroid()),
            Waypoint::with_data(client.clone(), some_fuel_station()),
            Waypoint::with_data(client.clone(), some_asteroid_base()),
            Waypoint::with_data(client.clone(), some_asteroid()),
        ];

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn should_list_waypoints_with_a_certain_type_with_an_object() {
        let mock_server = MockServerBuilder::mock_once(
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

        let expected = vec![Waypoint::with_data(client.clone(), some_planet())];

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn should_list_waypoints_with_a_certain_trait_with_an_object() {
        let mock_server = MockServerBuilder::mock_once(
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

        let expected = vec![Waypoint::with_data(client.clone(), some_moon())];

        assert_eq!(expected, actual)
    }

    #[test]
    pub fn system_data_should_be_deserializable() {
        let json_str = r#"
        {
          "data": {
            "symbol": "X1-AG18",
            "sectorSymbol": "X1",
            "type": "ORANGE_STAR",
            "x": 20324,
            "y": 7157,
            "waypoints": [
              {
                "symbol": "X1-AG18-A1",
                "type": "PLANET",
                "x": -10,
                "y": -24,
                "orbitals": [
                  {
                    "symbol": "X1-AG18-A2"
                  },
                  {
                    "symbol": "X1-AG18-A3"
                  },
                  {
                    "symbol": "X1-AG18-A4"
                  }
                ]
              },
              {
                "symbol": "X1-AG18-XX5D",
                "type": "ENGINEERED_ASTEROID",
                "x": 29,
                "y": -1,
                "orbitals": []
              },
              {
                "symbol": "X1-AG18-A2",
                "type": "MOON",
                "x": -10,
                "y": -24,
                "orbitals": [],
                "orbits": "X1-AG18-A1"
              }
            ],
            "factions": [],
            "constellation": "Lacerta",
            "name": "Borealis"
          }
        }"#;

        let actual: SystemResponse = serde_json::from_str(json_str).unwrap();
        let expected = some_system_data_response();

        assert_eq!(expected, actual);
    }
}
