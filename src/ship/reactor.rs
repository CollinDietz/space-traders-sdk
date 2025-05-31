use serde_derive::Deserialize;

use super::Requirements;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
    pub symbol: ReactorType,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub power_output: i32,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum ReactorType {
    #[serde(rename = "REACTOR_SOLAR_I")]
    SolarI,
    #[serde(rename = "REACTOR_FUSION_I")]
    FusionI,
    #[serde(rename = "REACTOR_FISSION_I")]
    FissionI,
    #[serde(rename = "REACTOR_CHEMICAL_I")]
    ChemicalI,
    #[serde(rename = "REACTOR_ANTIMATTER_I")]
    AntimatterI,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_fission_reactor() -> Reactor {
        Reactor {
          symbol: ReactorType::FissionI,
          name: string!("Fission Reactor I"),
          description: string!("A basic fission power reactor, used to generate electricity from nuclear fission reactions."),
          condition: Some(1),
          power_output: 31,
          requirements: Requirements {
              power: None,
              crew: Some(8),
              slots: None,
          },
      }
    }

    #[test]
    fn some_fission_reactor_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "REACTOR_FISSION_I",
            "name": "Fission Reactor I",
            "condition": 1,
            "integrity": 1,
            "description": "A basic fission power reactor, used to generate electricity from nuclear fission reactions.",
            "powerOutput": 31,
            "requirements": {
            "crew": 8
            },
            "quality": 5
        }"#;

        let actual: Reactor = serde_json::from_str(json_str).unwrap();
        let expected = some_fission_reactor();

        assert_eq!(expected, actual);
    }

    pub fn some_solar_reactor() -> Reactor {
        Reactor {
            symbol: ReactorType::SolarI,
            name: string!("Solar Reactor I"),
            description: string!(
                "A basic solar power reactor, used to generate electricity from solar energy."
            ),
            condition: Some(1),
            power_output: 3,
            requirements: Requirements {
                power: None,
                crew: Some(0),
                slots: None,
            },
        }
    }

    #[test]
    fn some_solar_reactor_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "REACTOR_SOLAR_I",
            "name": "Solar Reactor I",
            "condition": 1,
            "integrity": 1,
            "description": "A basic solar power reactor, used to generate electricity from solar energy.",
            "powerOutput": 3,
            "requirements": {
                "crew": 0
            },
            "quality": 1
        }"#;

        let actual: Reactor = serde_json::from_str(json_str).unwrap();
        let expected = some_solar_reactor();

        assert_eq!(expected, actual);
    }
}
