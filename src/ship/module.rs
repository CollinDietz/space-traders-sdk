use serde_derive::Deserialize;

use super::Requirements;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub symbol: ModuleType,
    pub capacity: Option<i32>,
    pub range: Option<i32>,
    pub name: String,
    pub description: String,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum ModuleType {
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    MineralProcessorI,
    #[serde(rename = "MODULE_GAS_PROCESSOR_I")]
    GasProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    CargoHoldI,
    #[serde(rename = "MODULE_CARGO_HOLD_II")]
    CargoHoldII,
    #[serde(rename = "MODULE_CARGO_HOLD_III")]
    CargoHoldIII,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    CrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    EnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    PassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    MicroRefineryI,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    OreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    FuelRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    JumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    JumpDriveII,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    JumpDriveIII,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    WarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    WarpDriveII,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    WarpDriveIII,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ShieldGeneratorII,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_cargo_hold() -> Module {
        Module {
            symbol: ModuleType::CargoHoldI,
            capacity: Some(30),
            range: None,
            name: string!("Cargo Hold"),
            description: string!("A module that increases a ship's cargo capacity."),
            requirements: Requirements {
                power: Some(1),
                crew: Some(0),
                slots: Some(1),
            },
        }
    }

    #[test]
    fn cargo_hold_should_be_deserializable() {
        let json_str = r#"
          {
              "symbol": "MODULE_CARGO_HOLD_I",
              "name": "Cargo Hold",
              "description": "A module that increases a ship's cargo capacity.",
              "capacity": 30,
              "requirements": {
                  "crew": 0,
                  "power": 1,
                  "slots": 1
              }
          }"#;

        let actual: Module = serde_json::from_str(json_str).unwrap();
        let expected = some_cargo_hold();

        assert_eq!(expected, actual);
    }

    pub fn some_cargo_hold_2() -> Module {
        Module {
            symbol: ModuleType::CargoHoldII,
            capacity: Some(40),
            range: None,
            name: string!("Expanded Cargo Hold"),
            description: string!("An expanded cargo hold module that provides more efficient storage space for a ship's cargo."),
            requirements: Requirements {
                power: Some(2),
                crew: Some(2),
                slots: Some(2),
            },
        }
    }

    #[test]
    fn cargo_hold_2_should_be_deserializable() {
        let json_str = r#"
          {
            "symbol": "MODULE_CARGO_HOLD_II",
            "name": "Expanded Cargo Hold",
            "description": "An expanded cargo hold module that provides more efficient storage space for a ship's cargo.",
            "requirements": {
              "power": 2,
              "crew": 2,
              "slots": 2
            },
            "capacity": 40
          }"#;

        let actual: Module = serde_json::from_str(json_str).unwrap();
        let expected = some_cargo_hold_2();

        assert_eq!(expected, actual);
    }

    pub fn some_crew_quarters() -> Module {
        Module {
            symbol: ModuleType::CrewQuartersI,
            capacity: Some(40),
            range: None,
            name: string!("Crew Quarters"),
            description: string!("A module that provides living space and amenities for the crew."),
            requirements: Requirements {
                power: Some(1),
                crew: Some(2),
                slots: Some(1),
            },
        }
    }

    #[test]
    fn crew_quarters_should_be_deserializable() {
        let json_str = r#"
      {
          "symbol": "MODULE_CREW_QUARTERS_I",
          "name": "Crew Quarters",
          "description": "A module that provides living space and amenities for the crew.",
          "capacity": 40,
          "requirements": {
             "crew": 2,
             "power": 1,
             "slots": 1
          }
       }"#;

        let actual: Module = serde_json::from_str(json_str).unwrap();
        let expected = some_crew_quarters();

        assert_eq!(expected, actual);
    }

    pub fn some_mineral_processor() -> Module {
        Module {
          symbol: ModuleType::MineralProcessorI,
          capacity: None,
          range: None,
          name: string!("Mineral Processor"),
          description: string!("Crushes and processes extracted minerals and ores into their component parts, filters out impurities, and containerizes them into raw storage units."),
          requirements: Requirements {
              power: Some(1),
              crew: Some(0),
              slots: Some(2),
          },
      }
    }

    #[test]
    fn mineral_processor_should_be_deserializable() {
        let json_str = r#"
          {
              "symbol": "MODULE_MINERAL_PROCESSOR_I",
              "name": "Mineral Processor",
              "description": "Crushes and processes extracted minerals and ores into their component parts, filters out impurities, and containerizes them into raw storage units.",
              "requirements": {
                  "crew": 0,
                  "power": 1,
                  "slots": 2
              }
          }"#;

        let actual: Module = serde_json::from_str(json_str).unwrap();
        let expected = some_mineral_processor();

        assert_eq!(expected, actual);
    }

    pub fn some_jump_drive() -> Module {
        Module {
          symbol: ModuleType::JumpDriveI,
          capacity: None,
          range: Some(500),
          name: string!("Jump Drive I"),
          description: string!("A basic antimatter jump drive that allows for instantaneous short-range interdimensional travel."),
          requirements: Requirements {
              power: Some(4),
              crew: Some(10),
              slots: Some(1),
          },
      }
    }

    #[test]
    fn jump_drive_should_be_deserializable() {
        let json_str = r#"
          {
              "symbol": "MODULE_JUMP_DRIVE_I",
              "name": "Jump Drive I",
              "description": "A basic antimatter jump drive that allows for instantaneous short-range interdimensional travel.",
              "range": 500,
              "requirements": {
              "crew": 10,
              "power": 4,
              "slots": 1
              }
          }"#;

        let actual: Module = serde_json::from_str(json_str).unwrap();
        let expected = some_jump_drive();

        assert_eq!(expected, actual);
    }

    pub fn some_warp_drive() -> Module {
        Module {
            symbol: ModuleType::WarpDriveI,
            capacity: None,
            range: Some(2000),
            name: string!("Warp Drive I"),
            description: string!(
                "A basic warp drive that allows for short-range interstellar travel."
            ),
            requirements: Requirements {
                power: Some(3),
                crew: Some(2),
                slots: Some(1),
            },
        }
    }

    #[test]
    fn warp_drive_should_be_deserializable() {
        let json_str = r#"
          {
              "symbol": "MODULE_WARP_DRIVE_I",
              "name": "Warp Drive I",
              "description": "A basic warp drive that allows for short-range interstellar travel.",
              "range": 2000,
              "requirements": {
              "crew": 2,
              "power": 3,
              "slots": 1
              }
          }"#;

        let actual: Module = serde_json::from_str(json_str).unwrap();
        let expected = some_warp_drive();

        assert_eq!(expected, actual);
    }

    pub fn some_gas_processor() -> Module {
        Module {
            symbol: ModuleType::GasProcessorI,
            capacity: None,
            range: None,
            name: string!("Gas Processor"),
            description: string!(
                "Filters and processes extracted gases into their component parts, filters out impurities, and containerizes them into raw storage units."
            ),
            requirements: Requirements {
                power: Some(1),
                crew: Some(0),
                slots: Some(2),
            },
        }
    }

    #[test]
    fn gas_processor_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "MODULE_GAS_PROCESSOR_I",
            "name": "Gas Processor",
            "description": "Filters and processes extracted gases into their component parts, filters out impurities, and containerizes them into raw storage units.",
            "requirements": {
                "power": 1,
                "crew": 0,
                "slots": 2
            }
        }"#;

        let actual: Module = serde_json::from_str(json_str).unwrap();
        let expected = some_gas_processor();

        assert_eq!(expected, actual);
    }
}
