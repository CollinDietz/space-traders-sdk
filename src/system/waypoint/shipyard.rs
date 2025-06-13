use serde_derive::Deserialize;

use crate::{
    ship::{Crew, Engine, Frame, Module, Mount, Reactor},
    system::waypoint::market::{ActivityLevel, SupplyLevel},
};

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShipyardResponse {
    pub data: Shipyard,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Shipyard {
    pub symbol: String,
    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<ShipTypeWrapper>,
    pub transactions: Option<Vec<ShipyardTransaction>>,
    pub ships: Option<Vec<ShipyardShip>>,
    #[serde(rename = "modificationsFee")]
    pub modifications_fee: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShipyardShip {
    pub r#type: ShipType,
    pub name: String,
    pub description: String,
    pub supply: SupplyLevel,
    pub activity: ActivityLevel,
    pub purchase_price: i32,
    pub frame: Frame,
    pub reactor: Reactor,
    pub engine: Engine,
    pub modules: Vec<Module>,
    pub mounts: Vec<Mount>,
    pub crew: Crew,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShipType {
    #[serde(rename = "SHIP_PROBE")]
    Probe,
    #[serde(rename = "SHIP_MINING_DRONE")]
    MiningDrone,
    #[serde(rename = "SHIP_SIPHON_DRONE")]
    SiphonDrone,
    #[serde(rename = "SHIP_INTERCEPTOR")]
    Interceptor,
    #[serde(rename = "SHIP_LIGHT_HAULER")]
    LightHauler,
    #[serde(rename = "SHIP_COMMAND_FRIGATE")]
    CommandFrigate,
    #[serde(rename = "SHIP_EXPLORER")]
    Explorer,
    #[serde(rename = "SHIP_HEAVY_FREIGHTER")]
    HeavyFreighter,
    #[serde(rename = "SHIP_LIGHT_SHUTTLE")]
    LightShuttle,
    #[serde(rename = "SHIP_ORE_HOUND")]
    OreHound,
    #[serde(rename = "SHIP_REFINING_FREIGHTER")]
    RefiningFreighter,
    #[serde(rename = "SHIP_SURVEYOR")]
    Surveyor,
    #[serde(rename = "SHIP_BULK_FREIGHTER")]
    BulkFreighter,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShipTypeWrapper {
    pub r#type: ShipType,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ShipyardTransaction {
    pub waypoint_symbol: String,
    pub ship_symbol: Option<String>,
    pub ship_type: ShipType,
    pub price: i32,
    pub agent_symbol: String,
    pub timestamp: String,
}

#[cfg(test)]
pub mod tests {
    use crate::{
        string,
        system::waypoint::shipyard::{ShipType, ShipTypeWrapper, Shipyard, ShipyardResponse},
    };

    pub fn some_shipyard() -> Shipyard {
        Shipyard {
            symbol: string!("X1-MH3-A2"),
            ship_types: vec![
                ShipTypeWrapper {
                    r#type: ShipType::Probe,
                },
                ShipTypeWrapper {
                    r#type: ShipType::LightShuttle,
                },
                ShipTypeWrapper {
                    r#type: ShipType::LightHauler,
                },
            ],
            transactions: None,
            ships: None,
            modifications_fee: 100,
        }
    }

    pub fn some_shipyard_response() -> ShipyardResponse {
        ShipyardResponse {
            data: some_shipyard(),
        }
    }

    #[test]
    pub fn ship_yard_data_should_be_deserializable() {
        let json_str = r#"
        {
          "data": {
            "symbol": "X1-MH3-A2",
            "shipTypes": [
              {
                "type": "SHIP_PROBE"
              },
              {
                "type": "SHIP_LIGHT_SHUTTLE"
              },
              {
                "type": "SHIP_LIGHT_HAULER"
              }
            ],
            "modificationsFee": 100
          }
        }"#;

        let actual: ShipyardResponse = serde_json::from_str(json_str).unwrap();
        let expected = some_shipyard_response();

        assert_eq!(expected, actual);
    }
}
