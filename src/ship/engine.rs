use serde_derive::Deserialize;

use super::Requirements;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    pub symbol: EngineType,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub speed: i32,
    pub requirements: Requirements,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

    pub fn some_ion_drive_2() -> Engine {
        Engine {
          symbol: EngineType::IonDriveII,
          name: string!("Ion Drive II"),
          description: string!("An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance."),
          condition: Some(1),
          speed: 36,
          requirements: Requirements {
              power: Some(6),
              crew: Some(8),
              slots: None,
          },
      }
    }

    #[test]
    fn some_ion_drive_2_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "ENGINE_ION_DRIVE_II",
            "name": "Ion Drive II",
            "condition": 1,
            "integrity": 1,
            "description": "An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance.",
            "speed": 36,
            "requirements": {
            "power": 6,
            "crew": 8
            },
            "quality": 4
        }"#;

        let actual: Engine = serde_json::from_str(json_str).unwrap();
        let expected = some_ion_drive_2();

        assert_eq!(expected, actual);
    }

    pub fn some_impluse_drive() -> Engine {
        Engine {
          symbol: EngineType::ImpulseDriveI,
          name: string!("Impulse Drive I"),
          description: string!("A basic low-energy propulsion system that generates thrust for interplanetary travel."),
          condition: Some(1),
          speed: 9,
          requirements: Requirements {
              power: Some(1),
              crew: Some(0),
              slots: None,
          },
      }
    }

    #[test]
    fn some_impluse_drive_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "ENGINE_IMPULSE_DRIVE_I",
            "name": "Impulse Drive I",
            "condition": 1,
            "integrity": 1,
            "description": "A basic low-energy propulsion system that generates thrust for interplanetary travel.",
            "speed": 9,
            "requirements": {
                "power": 1,
                "crew": 0
            },
            "quality": 1
        }"#;

        let actual: Engine = serde_json::from_str(json_str).unwrap();
        let expected = some_impluse_drive();

        assert_eq!(expected, actual);
    }
}
