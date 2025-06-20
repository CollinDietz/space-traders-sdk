use serde_derive::Deserialize;

use super::Requirements;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    pub symbol: MountType,
    pub name: String,
    pub description: Option<String>,
    pub strength: Option<i32>,
    pub deposits: Option<Vec<ResourceType>>,
    pub requirements: Requirements,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum MountType {
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    GasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    GasSiphonII,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    GasSiphonIII,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    SurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    SurveyorII,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    SurveyorIII,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    SensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    SensorArrayII,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    SensorArrayIII,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MiningLaserII,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MiningLaserIII,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    LaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    TurretI,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    QuartzSand,
    SiliconCrystals,
    PreciousStones,
    IceWater,
    AmmoniaIce,
    IronOre,
    CopperOre,
    SilverOre,
    AluminumOre,
    GoldOre,
    PlatinumOre,
    Diamonds,
    UraniteOre,
    MeritiumOre,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_sensor_array() -> Mount {
        Mount {
          symbol: MountType::SensorArrayI,
          name: string!("Sensor Array I"),
          description: Some(string!("A basic sensor array that improves a ship's ability to detect and track other objects in space.")),
          strength: Some(1),
          deposits: None,
          requirements: Requirements {
              power: Some(1),
              crew: Some(0),
              slots: None,
          },
      }
    }

    #[test]
    fn sensor_array_should_be_deserializable() {
        let json_str = r#"
      {
          "symbol": "MOUNT_SENSOR_ARRAY_I",
          "name": "Sensor Array I",
          "description": "A basic sensor array that improves a ship's ability to detect and track other objects in space.",
          "strength": 1,
          "requirements": {
             "crew": 0,
             "power": 1
          }
       }"#;

        let actual: Mount = serde_json::from_str(json_str).unwrap();
        let expected = some_sensor_array();

        assert_eq!(expected, actual);
    }

    pub fn some_sensor_array_2() -> Mount {
        Mount {
          symbol: MountType::SensorArrayII,
          name: string!("Sensor Array II"),
          description: Some(string!("An advanced sensor array that improves a ship's ability to detect and track other objects in space with greater accuracy and range.")),
          strength: Some(4),
          deposits: None,
          requirements: Requirements {
              power: Some(2),
              crew: Some(2),
              slots: None,
          },
      }
    }

    #[test]
    fn sensor_array_2_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "MOUNT_SENSOR_ARRAY_II",
            "name": "Sensor Array II",
            "description": "An advanced sensor array that improves a ship's ability to detect and track other objects in space with greater accuracy and range.",
            "requirements": {
                "power": 2,
                "crew": 2
            },
            "strength": 4
        }"#;

        let actual: Mount = serde_json::from_str(json_str).unwrap();
        let expected = some_sensor_array_2();

        assert_eq!(expected, actual);
    }

    pub fn some_mining_laser() -> Mount {
        Mount {
          symbol: MountType::MiningLaserI,
          name: string!("Mining Laser I"),
          description: Some(string!("A basic mining laser that can be used to extract valuable minerals from asteroids and other space objects.")),
          strength: Some(10),
          deposits: None,
          requirements: Requirements {
              power: Some(1),
              crew: Some(0),
              slots: None,
          },
      }
    }

    #[test]
    fn mining_laser_should_be_deserializable() {
        let json_str = r#"
      {
          "symbol": "MOUNT_MINING_LASER_I",
          "name": "Mining Laser I",
          "description": "A basic mining laser that can be used to extract valuable minerals from asteroids and other space objects.",
          "strength": 10,
          "requirements": {
             "crew": 0,
             "power": 1
          }
       }"#;

        let actual: Mount = serde_json::from_str(json_str).unwrap();
        let expected = some_mining_laser();

        assert_eq!(expected, actual);
    }

    pub fn some_mining_laser_2() -> Mount {
        Mount {
          symbol: MountType::MiningLaserII,
          name: string!("Mining Laser II"),
          description: Some(string!("An advanced mining laser that is more efficient and effective at extracting valuable minerals from asteroids and other space objects.")),
          strength: Some(5),
          deposits: None,
          requirements: Requirements {
              power: Some(2),
              crew: Some(2),
              slots: None,
          },
      }
    }

    #[test]
    fn mining_laser_2_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "MOUNT_MINING_LASER_II",
            "name": "Mining Laser II",
            "description": "An advanced mining laser that is more efficient and effective at extracting valuable minerals from asteroids and other space objects.",
            "requirements": {
                "power": 2,
                "crew": 2
            },
            "strength": 5
        }"#;

        let actual: Mount = serde_json::from_str(json_str).unwrap();
        let expected = some_mining_laser_2();

        assert_eq!(expected, actual);
    }

    pub fn some_surveyor() -> Mount {
        Mount {
          symbol: MountType::SurveyorI,
          name: string!("Surveyor I"),
          description: Some(string!("A basic survey probe that can be used to gather information about a mineral deposit.")),
          strength: Some(1),
          deposits: Some(vec![
              ResourceType::QuartzSand,
              ResourceType::SiliconCrystals,
              ResourceType::PreciousStones,
              ResourceType::IceWater,
              ResourceType::AmmoniaIce,
              ResourceType::IronOre,
              ResourceType::CopperOre,
              ResourceType::SilverOre,
              ResourceType::AluminumOre,
              ResourceType::GoldOre,
              ResourceType::PlatinumOre,
          ]),
          requirements: Requirements {
              power: Some(1),
              crew: Some(2),
              slots: None,
          },
      }
    }

    #[test]
    fn surveyor_should_be_deserializable() {
        let json_str = r#"
          {
              "symbol": "MOUNT_SURVEYOR_I",
              "name": "Surveyor I",
              "description": "A basic survey probe that can be used to gather information about a mineral deposit.",
              "strength": 1,
              "deposits": [
                  "QUARTZ_SAND",
                  "SILICON_CRYSTALS",
                  "PRECIOUS_STONES",
                  "ICE_WATER",
                  "AMMONIA_ICE",
                  "IRON_ORE",
                  "COPPER_ORE",
                  "SILVER_ORE",
                  "ALUMINUM_ORE",
                  "GOLD_ORE",
                  "PLATINUM_ORE"
              ],
              "requirements": {
                  "crew": 2,
                  "power": 1
              }
          }"#;

        let actual: Mount = serde_json::from_str(json_str).unwrap();
        let expected = some_surveyor();

        assert_eq!(expected, actual);
    }

    pub fn some_surveyor_2() -> Mount {
        Mount {
          symbol: MountType::SurveyorII,
          name: string!("Surveyor II"),
          description: Some(string!("An advanced survey probe that can be used to gather information about a mineral deposit with greater accuracy.")),
          strength: Some(2),
          deposits: Some(vec![
              ResourceType::QuartzSand,
              ResourceType::SiliconCrystals,
              ResourceType::PreciousStones,
              ResourceType::IceWater,
              ResourceType::AmmoniaIce,
              ResourceType::IronOre,
              ResourceType::CopperOre,
              ResourceType::SilverOre,
              ResourceType::AluminumOre,
              ResourceType::GoldOre,
              ResourceType::PlatinumOre,
              ResourceType::Diamonds,
              ResourceType::UraniteOre,
          ]),
          requirements: Requirements {
              power: Some(3),
              crew: Some(4),
              slots: None,
          },
      }
    }

    #[test]
    fn surveyor_2_should_be_deserializable() {
        let json_str = r#"
          {
            "symbol": "MOUNT_SURVEYOR_II",
            "name": "Surveyor II",
            "description": "An advanced survey probe that can be used to gather information about a mineral deposit with greater accuracy.",
            "requirements": {
              "power": 3,
              "crew": 4
            },
            "strength": 2,
            "deposits": [
              "QUARTZ_SAND",
              "SILICON_CRYSTALS",
              "PRECIOUS_STONES",
              "ICE_WATER",
              "AMMONIA_ICE",
              "IRON_ORE",
              "COPPER_ORE",
              "SILVER_ORE",
              "ALUMINUM_ORE",
              "GOLD_ORE",
              "PLATINUM_ORE",
              "DIAMONDS",
              "URANITE_ORE"
            ]
          }"#;

        let actual: Mount = serde_json::from_str(json_str).unwrap();
        let expected = some_surveyor_2();

        assert_eq!(expected, actual);
    }

    pub fn some_gas_siphon_2() -> Mount {
        Mount {
          symbol: MountType::GasSiphonII,
          name: string!("Gas Siphon II"),
          description: Some(string!("An advanced gas siphon that can extract gas and other resources from gas giants and other gas-rich bodies more efficiently and at a higher rate.")),
          strength: Some(20),
          deposits: None,
          requirements: Requirements {
              power: Some(2),
              crew: Some(2),
              slots: None,
          },
      }
    }

    #[test]
    fn gas_siphon_2_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "MOUNT_GAS_SIPHON_II",
            "name": "Gas Siphon II",
            "description": "An advanced gas siphon that can extract gas and other resources from gas giants and other gas-rich bodies more efficiently and at a higher rate.",
            "requirements": {
                "power": 2,
                "crew": 2
            },
            "strength": 20
        }"#;

        let actual: Mount = serde_json::from_str(json_str).unwrap();
        let expected = some_gas_siphon_2();

        assert_eq!(expected, actual);
    }
}
