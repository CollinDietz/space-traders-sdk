use serde_derive::Deserialize;

use super::Requirements;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    pub symbol: EngineType,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub speed: i32,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum EngineType {
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    ImpulseDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_I")]
    IonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    IonDriveII,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    HyperDriveI,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_engine() -> Engine {
        Engine {
          symbol: EngineType::IonDriveII,
          name: string!("Ion Drive II"),
          description: string!("An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance."),
          condition: Some(100),
          speed: 30,
          requirements: Requirements {
              power: Some(6),
              crew: Some(8),
              slots: None,
          },
      }
    }

    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
          {
              "symbol": "ENGINE_ION_DRIVE_II",
              "name": "Ion Drive II",
              "description": "An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance.",
              "condition": 100,
              "speed": 30,
              "requirements": {
              "power": 6,
              "crew": 8
              }
          }"#;

        let actual: Engine = serde_json::from_str(json_str).unwrap();
        let expected = some_engine();

        assert_eq!(expected, actual);
    }
}
