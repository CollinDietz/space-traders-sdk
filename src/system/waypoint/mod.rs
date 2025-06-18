use std::sync::Arc;

use serde_derive::{Deserialize, Serialize};

use crate::{
    faction::Factions,
    space_traders_client::{Error, SpaceTradersClient},
    system::waypoint::{
        market::{Market, MarketResponse},
        shipyard::{Shipyard, ShipyardResponse},
    },
};

pub mod market;
pub mod shipyard;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaypointData {
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: WaypointType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<WaypointOrbital>,
    pub orbits: Option<String>,
    pub faction: Option<WaypointFaction>,
    pub traits: Vec<WaypointTrait>,
    pub modifiers: Option<Vec<WaypointModifier>>,
    pub chart: Option<Chart>,
    pub is_under_construction: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct WaypointOrbital {
    pub symbol: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WaypointFaction {
    pub symbol: Factions,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WaypointTrait {
    pub symbol: WaypointTraitSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WaypointModifier {
    pub symbol: WaypointModifierSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub waypoint_symbol: String,
    pub submitted_by: Option<String>,
    pub submitted_on: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Asteroid,
    EngineeredAsteroid,
    AsteroidBase,
    Nebula,
    DebrisField,
    GravityWell,
    ArtificialGravityWell,
    FuelStation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointTraitSymbol {
    Uncharted,
    UnderConstruction,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    PirateBase,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    ThinAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Radioactive,
    MicroGravityAnomalies,
    DebrisCluster,
    DeepCraters,
    ShallowCraters,
    UnstableComposition,
    HollowedInterior,
    Stripped,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointModifierSymbol {
    Stripped,
    Unstable,
    RadiationLeak,
    CriticalLimit,
    CivilUnrest,
}

#[derive(Debug, PartialEq)]
pub struct Waypoint {
    pub data: WaypointData,
    client: Arc<SpaceTradersClient>,
}

impl Waypoint {
    pub fn new(client: Arc<SpaceTradersClient>, data: WaypointData) -> Self {
        Waypoint {
            client: client,
            data,
        }
    }

    pub async fn get_shipyard(&self) -> Result<Shipyard, Error> {
        let response: ShipyardResponse = self
            .client
            .get(
                &format!(
                    "systems/{}/waypoints/{}/shipyard",
                    self.data.system_symbol, self.data.symbol
                ),
                None::<&()>,
                reqwest::StatusCode::OK,
            )
            .await?;

        Ok(response.data)
    }

    pub async fn get_market(&self) -> Result<Market, Error> {
        let response: MarketResponse = self
            .client
            .get(
                &format!(
                    "systems/{}/waypoints/{}/market",
                    self.data.system_symbol, self.data.symbol
                ),
                None::<&()>,
                reqwest::StatusCode::OK,
            )
            .await?;

        Ok(response.data)
    }
}

#[cfg(test)]
pub mod tests {
    use std::vec;

    use mock_server::{MockServerBuilder, RequestMethod};

    use crate::{
        faction::Factions,
        string,
        system::waypoint::{market::tests::some_market, shipyard::tests::some_shipyard, *},
    };

    fn some_chart() -> Chart {
        Chart {
            waypoint_symbol: string!("X1-AB31-A2"),
            submitted_by: Some(string!("COSMIC")),
            submitted_on: Some(string!("2025-06-01T13:01:39.041Z")),
        }
    }

    pub fn some_planet() -> WaypointData {
        WaypointData {
            symbol: string!("X1-MH3-A1"),
            waypoint_type: WaypointType::Planet,
            system_symbol: string!("X1-MH3"),
            x: 15,
            y: -19,
            orbitals: vec![
                WaypointOrbital { symbol: string!("X1-MH3-A2") },
                WaypointOrbital { symbol: string!("X1-MH3-A3") },
                WaypointOrbital { symbol: string!("X1-MH3-A4") },
            ],
            orbits: None,
            faction: Some(WaypointFaction { symbol: Factions::Cosmic }),
            traits: vec![
                WaypointTrait {
                    symbol: WaypointTraitSymbol::Rocky,
                    name: string!("Rocky"),
                    description: string!("A world with a rugged, rocky landscape, rich in minerals and other resources, providing a variety of opportunities for mining, research, and exploration."),
                },
                WaypointTrait {
                    symbol: WaypointTraitSymbol::Outpost,
                    name: string!("Outpost"),
                    description: string!("A small, remote settlement providing essential services and a safe haven for travelers passing through."),
                },
                WaypointTrait {
                    symbol: WaypointTraitSymbol::ThinAtmosphere,
                    name: string!("Thin Atmosphere"),
                    description: string!("A location with a sparse atmosphere, making it difficult to support life without specialized life-support systems."),
                },
                WaypointTrait {
                    symbol: WaypointTraitSymbol::DrySeabeds,
                    name: string!("Dry Seabeds"),
                    description: string!("Vast, desolate landscapes that once held oceans, now exposing the remnants of ancient marine life and providing opportunities for the discovery of valuable resources."),
                },
                WaypointTrait {
                    symbol: WaypointTraitSymbol::Marketplace,
                    name: string!("Marketplace"),
                    description: string!("A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."),
                },
            ],
            modifiers: Some(vec![]),
            chart: Some(Chart {
                waypoint_symbol: string!("X1-MH3-A1"),
                submitted_by: Some(string!("COSMIC")),
                submitted_on: Some(string!("2025-06-08T13:01:40.136Z")),
            }),
            is_under_construction: false,
        }
    }

    pub fn some_engineered_asteroid() -> WaypointData {
        WaypointData {
            symbol: string!("X1-MH3-FE5Z"),
            waypoint_type: WaypointType::EngineeredAsteroid,
            system_symbol: string!("X1-MH3"),
            x: 27,
            y: 2,
            orbitals: vec![],
            orbits: None,
            faction: Some(WaypointFaction { symbol: Factions::Cosmic }),
            traits: vec![
            WaypointTrait {
                symbol: WaypointTraitSymbol::CommonMetalDeposits,
                name: string!("Common Metal Deposits"),
                description: string!("A waypoint rich in common metal ores like iron, copper, and aluminum, essential for construction and manufacturing."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Stripped,
                name: string!("Stripped"),
                description: string!("A location that has been over-mined or over-harvested, resulting in depleted resources and barren landscapes."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Marketplace,
                name: string!("Marketplace"),
                description: string!("A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."),
            },
            ],
            modifiers: Some(vec![]),
            chart: Some(Chart {
            waypoint_symbol: string!("X1-MH3-FE5Z"),
            submitted_by: Some(string!("COSMIC")),
            submitted_on: Some(string!("2025-06-08T13:01:40.136Z")),
            }),
            is_under_construction: false,
      }
    }

    pub fn some_fuel_station() -> WaypointData {
        WaypointData {
            symbol: string!("X1-MH3-B6"),
            waypoint_type: WaypointType::FuelStation,
            system_symbol: string!("X1-MH3"),
            x: 37,
            y: -186,
            orbitals: vec![],
            orbits: None,
            faction: Some(WaypointFaction { symbol: Factions::Cosmic }),
            traits: vec![
            WaypointTrait {
                symbol: WaypointTraitSymbol::Marketplace,
                name: string!("Marketplace"),
                description: string!("A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."),
            }
            ],
            modifiers: Some(vec![]),
            chart: Some(Chart {
            waypoint_symbol: string!("X1-MH3-B6"),
            submitted_by: Some(string!("COSMIC")),
            submitted_on: Some(string!("2025-06-08T13:01:40.136Z")),
            }),
            is_under_construction: false,
      }
    }

    pub fn some_asteroid_base() -> WaypointData {
        WaypointData {
            symbol: string!("X1-MH3-B7"),
            waypoint_type: WaypointType::AsteroidBase,
            system_symbol: string!("X1-MH3"),
            x: 41,
            y: -342,
            orbitals: vec![],
            orbits: None,
            faction: Some(WaypointFaction { symbol: Factions::Cosmic }),
            traits: vec![
            WaypointTrait {
                symbol: WaypointTraitSymbol::HollowedInterior,
                name: string!("Hollowed Interior"),
                description: string!("A location with large hollow spaces beneath its surface, providing unique opportunities for subterranean construction and resource extraction, but also posing risks of structural instability."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Outpost,
                name: string!("Outpost"),
                description: string!("A small, remote settlement providing essential services and a safe haven for travelers passing through."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Marketplace,
                name: string!("Marketplace"),
                description: string!("A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."),
            },
            ],
            modifiers: Some(vec![]),
            chart: Some(Chart {
            waypoint_symbol: string!("X1-MH3-B7"),
            submitted_by: Some(string!("COSMIC")),
            submitted_on: Some(string!("2025-06-08T13:01:40.136Z")),
            }),
            is_under_construction: false,
      }
    }

    pub fn some_asteroid() -> WaypointData {
        WaypointData {
            symbol: string!("X1-MH3-B8"),
            waypoint_type: WaypointType::Asteroid,
            system_symbol: string!("X1-MH3"),
            x: 334,
            y: -145,
            orbitals: vec![],
            orbits: None,
            faction: Some(WaypointFaction { symbol: Factions::Cosmic }),
            traits: vec![
            WaypointTrait {
                symbol: WaypointTraitSymbol::CommonMetalDeposits,
                name: string!("Common Metal Deposits"),
                description: string!("A waypoint rich in common metal ores like iron, copper, and aluminum, essential for construction and manufacturing."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Radioactive,
                name: string!("Radioactive"),
                description: string!("A hazardous location with elevated levels of radiation, requiring specialized equipment and shielding for safe habitation and exploration."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::ShallowCraters,
                name: string!("Shallow Craters"),
                description: string!("Numerous shallow craters, offering easier access to sub-surface resources but also creating an uneven terrain that can complicate land-based activities."),
            },
            ],
            modifiers: Some(vec![]),
            chart: Some(Chart {
            waypoint_symbol: string!("X1-MH3-B8"),
            submitted_by: Some(string!("COSMIC")),
            submitted_on: Some(string!("2025-06-08T13:01:40.136Z")),
            }),
            is_under_construction: false,
      }
    }

    pub fn some_moon() -> WaypointData {
        WaypointData {
          symbol: string!("X1-MH3-A2"),
          waypoint_type: WaypointType::Moon,
          system_symbol: string!("X1-MH3"),
          x: 15,
          y: -19,
          orbitals: vec![],
          orbits: Some(string!("X1-MH3-A1")),
          faction: Some(WaypointFaction { symbol: Factions::Cosmic }),
          traits: vec![
        WaypointTrait {
            symbol: WaypointTraitSymbol::Volcanic,
            name: string!("Volcanic"),
            description: string!("A volatile world marked by intense volcanic activity, creating a hazardous environment with the potential for valuable resource extraction, such as rare metals and geothermal energy."),
        },
        WaypointTrait {
            symbol: WaypointTraitSymbol::Outpost,
            name: string!("Outpost"),
            description: string!("A small, remote settlement providing essential services and a safe haven for travelers passing through."),
        },
        WaypointTrait {
            symbol: WaypointTraitSymbol::ThinAtmosphere,
            name: string!("Thin Atmosphere"),
            description: string!("A location with a sparse atmosphere, making it difficult to support life without specialized life-support systems."),
        },
        WaypointTrait {
            symbol: WaypointTraitSymbol::CorrosiveAtmosphere,
            name: string!("Corrosive Atmosphere"),
            description: string!("A hostile environment with an atmosphere that can rapidly degrade materials and equipment, requiring advanced engineering solutions to ensure the safety and longevity of structures and vehicles."),
        },
        WaypointTrait {
            symbol: WaypointTraitSymbol::Marketplace,
            name: string!("Marketplace"),
            description: string!("A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."),
        },
        WaypointTrait {
            symbol: WaypointTraitSymbol::Shipyard,
            name: string!("Shipyard"),
            description: string!("A bustling hub for the construction, repair, and sale of various spacecraft, from humble shuttles to mighty warships."),
        },
          ],
          modifiers: Some(vec![]),
          chart: Some(Chart {
        waypoint_symbol: string!("X1-MH3-A2"),
        submitted_by: Some(string!("COSMIC")),
        submitted_on: Some(string!("2025-06-08T13:01:40.176Z")),
          }),
          is_under_construction: false,
      }
    }

    #[tokio::test]
    async fn get_shipyard_request_should_be_sent_parsed_and_returned_for_waypoint_that_has_shipyard(
    ) {
        let mock_server = MockServerBuilder::mock_once::<serde_json::Value>(
            RequestMethod::Get,
            "systems/X1-MH3/waypoints/X1-MH3-A2/shipyard",
            200,
            None,
            None,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let waypoint = Waypoint::new(client.clone(), some_moon());
        let actual = waypoint.get_shipyard().await.unwrap();

        let expected = some_shipyard();

        assert_eq!(expected, actual)
    }

    #[tokio::test]
    async fn get_market_request_should_be_sent_parsed_and_returned_for_waypoint_that_has_market() {
        let mock_server = MockServerBuilder::mock_once::<serde_json::Value>(
            RequestMethod::Get,
            "systems/X1-MH3/waypoints/X1-MH3-A2/market",
            200,
            None,
            None,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let waypoint = Waypoint::new(client.clone(), some_moon());
        let actual = waypoint.get_market().await.unwrap();

        let expected = some_market();

        assert_eq!(expected, actual)
    }

    #[test]
    fn chart_should_be_deserializable() {
        let json_str = r#"
        {
            "waypointSymbol": "X1-AB31-A2",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-01T13:01:39.041Z"
        }"#;

        let actual: Chart = serde_json::from_str(json_str).unwrap();

        let expected = some_chart();

        assert_eq!(expected, actual);
    }

    #[test]
    fn some_planet_should_be_deserializable() {
        let json_str = r#"
        {
          "symbol": "X1-MH3-A1",
          "type": "PLANET",
          "systemSymbol": "X1-MH3",
          "x": 15,
          "y": -19,
          "orbitals": [
            {
              "symbol": "X1-MH3-A2"
            },
            {
              "symbol": "X1-MH3-A3"
            },
            {
              "symbol": "X1-MH3-A4"
            }
          ],
          "traits": [
            {
              "symbol": "ROCKY",
              "name": "Rocky",
              "description": "A world with a rugged, rocky landscape, rich in minerals and other resources, providing a variety of opportunities for mining, research, and exploration."
            },
            {
              "symbol": "OUTPOST",
              "name": "Outpost",
              "description": "A small, remote settlement providing essential services and a safe haven for travelers passing through."
            },
            {
              "symbol": "THIN_ATMOSPHERE",
              "name": "Thin Atmosphere",
              "description": "A location with a sparse atmosphere, making it difficult to support life without specialized life-support systems."
            },
            {
              "symbol": "DRY_SEABEDS",
              "name": "Dry Seabeds",
              "description": "Vast, desolate landscapes that once held oceans, now exposing the remnants of ancient marine life and providing opportunities for the discovery of valuable resources."
            },
            {
              "symbol": "MARKETPLACE",
              "name": "Marketplace",
              "description": "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."
            }
          ],
          "isUnderConstruction": false,
          "faction": {
            "symbol": "COSMIC"
          },
          "modifiers": [],
          "chart": {
            "waypointSymbol": "X1-MH3-A1",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-08T13:01:40.136Z"
          }
        }"#;

        let expected: WaypointData = serde_json::from_str(json_str).unwrap();

        let actual = some_planet();

        assert_eq!(expected, actual);
    }

    #[test]
    fn some_engineered_asteroid_should_be_deserializable() {
        let json_str = r#"
        {
          "symbol": "X1-MH3-FE5Z",
          "type": "ENGINEERED_ASTEROID",
          "systemSymbol": "X1-MH3",
          "x": 27,
          "y": 2,
          "orbitals": [],
          "traits": [
            {
              "symbol": "COMMON_METAL_DEPOSITS",
              "name": "Common Metal Deposits",
              "description": "A waypoint rich in common metal ores like iron, copper, and aluminum, essential for construction and manufacturing."
            },
            {
              "symbol": "STRIPPED",
              "name": "Stripped",
              "description": "A location that has been over-mined or over-harvested, resulting in depleted resources and barren landscapes."
            },
            {
              "symbol": "MARKETPLACE",
              "name": "Marketplace",
              "description": "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."
            }
          ],
          "isUnderConstruction": false,
          "faction": {
            "symbol": "COSMIC"
          },
          "modifiers": [],
          "chart": {
            "waypointSymbol": "X1-MH3-FE5Z",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-08T13:01:40.136Z"
          }
        }"#;

        let actual: WaypointData = serde_json::from_str(json_str).unwrap();

        let expected = some_engineered_asteroid();

        assert_eq!(expected, actual);
    }

    #[test]
    fn some_fuel_station_should_be_deserializable() {
        let json_str = r#"
        {
          "symbol": "X1-MH3-B6",
          "type": "FUEL_STATION",
          "systemSymbol": "X1-MH3",
          "x": 37,
          "y": -186,
          "orbitals": [],
          "traits": [
            {
              "symbol": "MARKETPLACE",
              "name": "Marketplace",
              "description": "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."
            }
          ],
          "isUnderConstruction": false,
          "faction": {
            "symbol": "COSMIC"
          },
          "modifiers": [],
          "chart": {
            "waypointSymbol": "X1-MH3-B6",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-08T13:01:40.136Z"
          }
        }"#;

        let actual: WaypointData = serde_json::from_str(json_str).unwrap();

        let expected = some_fuel_station();

        assert_eq!(expected, actual);
    }

    #[test]
    fn some_asteroid_base_should_be_deserializable() {
        let json_str = r#"
        {
          "symbol": "X1-MH3-B7",
          "type": "ASTEROID_BASE",
          "systemSymbol": "X1-MH3",
          "x": 41,
          "y": -342,
          "orbitals": [],
          "traits": [
            {
              "symbol": "HOLLOWED_INTERIOR",
              "name": "Hollowed Interior",
              "description": "A location with large hollow spaces beneath its surface, providing unique opportunities for subterranean construction and resource extraction, but also posing risks of structural instability."
            },
            {
              "symbol": "OUTPOST",
              "name": "Outpost",
              "description": "A small, remote settlement providing essential services and a safe haven for travelers passing through."
            },
            {
              "symbol": "MARKETPLACE",
              "name": "Marketplace",
              "description": "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."
            }
          ],
          "isUnderConstruction": false,
          "faction": {
            "symbol": "COSMIC"
          },
          "modifiers": [],
          "chart": {
            "waypointSymbol": "X1-MH3-B7",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-08T13:01:40.136Z"
          }
        }"#;

        let actual: WaypointData = serde_json::from_str(json_str).unwrap();

        let expected = some_asteroid_base();

        assert_eq!(expected, actual);
    }

    #[test]
    fn some_asteroid_should_be_deserializable() {
        let json_str = r#"
        {
          "symbol": "X1-MH3-B8",
          "type": "ASTEROID",
          "systemSymbol": "X1-MH3",
          "x": 334,
          "y": -145,
          "orbitals": [],
          "traits": [
            {
              "symbol": "COMMON_METAL_DEPOSITS",
              "name": "Common Metal Deposits",
              "description": "A waypoint rich in common metal ores like iron, copper, and aluminum, essential for construction and manufacturing."
            },
            {
              "symbol": "RADIOACTIVE",
              "name": "Radioactive",
              "description": "A hazardous location with elevated levels of radiation, requiring specialized equipment and shielding for safe habitation and exploration."
            },
            {
              "symbol": "SHALLOW_CRATERS",
              "name": "Shallow Craters",
              "description": "Numerous shallow craters, offering easier access to sub-surface resources but also creating an uneven terrain that can complicate land-based activities."
            }
          ],
          "isUnderConstruction": false,
          "faction": {
            "symbol": "COSMIC"
          },
          "modifiers": [],
          "chart": {
            "waypointSymbol": "X1-MH3-B8",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-08T13:01:40.136Z"
          }
        }"#;

        let actual: WaypointData = serde_json::from_str(json_str).unwrap();

        let expected = some_asteroid();

        assert_eq!(expected, actual);
    }

    #[test]
    fn some_moon_should_be_deserializable() {
        let json_str = r#"
        {
          "symbol": "X1-MH3-A2",
          "type": "MOON",
          "systemSymbol": "X1-MH3",
          "x": 15,
          "y": -19,
          "orbitals": [],
          "traits": [
            {
              "symbol": "VOLCANIC",
              "name": "Volcanic",
              "description": "A volatile world marked by intense volcanic activity, creating a hazardous environment with the potential for valuable resource extraction, such as rare metals and geothermal energy."
            },
            {
              "symbol": "OUTPOST",
              "name": "Outpost",
              "description": "A small, remote settlement providing essential services and a safe haven for travelers passing through."
            },
            {
              "symbol": "THIN_ATMOSPHERE",
              "name": "Thin Atmosphere",
              "description": "A location with a sparse atmosphere, making it difficult to support life without specialized life-support systems."
            },
            {
              "symbol": "CORROSIVE_ATMOSPHERE",
              "name": "Corrosive Atmosphere",
              "description": "A hostile environment with an atmosphere that can rapidly degrade materials and equipment, requiring advanced engineering solutions to ensure the safety and longevity of structures and vehicles."
            },
            {
              "symbol": "MARKETPLACE",
              "name": "Marketplace",
              "description": "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."
            },
            {
              "symbol": "SHIPYARD",
              "name": "Shipyard",
              "description": "A bustling hub for the construction, repair, and sale of various spacecraft, from humble shuttles to mighty warships."
            }
          ],
          "isUnderConstruction": false,
          "orbits": "X1-MH3-A1",
          "faction": {
            "symbol": "COSMIC"
          },
          "modifiers": [],
          "chart": {
            "waypointSymbol": "X1-MH3-A2",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-08T13:01:40.176Z"
          }
        }"#;

        let actual: WaypointData = serde_json::from_str(json_str).unwrap();

        let expected = some_moon();

        assert_eq!(expected, actual);
    }
}
