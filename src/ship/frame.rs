use serde_derive::Deserialize;

use super::Requirements;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub symbol: FrameType,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub fuel_capacity: i32,
    pub requirements: Requirements,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum FrameType {
    #[serde(rename = "FRAME_PROBE")]
    Probe,
    #[serde(rename = "FRAME_DRONE")]
    Drone,
    #[serde(rename = "FRAME_INTERCEPTOR")]
    Interceptor,
    #[serde(rename = "FRAME_RACER")]
    Racer,
    #[serde(rename = "FRAME_FIGHTER")]
    Fighter,
    #[serde(rename = "FRAME_FRIGATE")]
    Frigate,
    #[serde(rename = "FRAME_SHUTTLE")]
    Shuttle,
    #[serde(rename = "FRAME_EXPLORER")]
    Explorer,
    #[serde(rename = "FRAME_MINER")]
    Miner,
    #[serde(rename = "FRAME_LIGHT_FREIGHTER")]
    LightFreighter,
    #[serde(rename = "FRAME_HEAVY_FREIGHTER")]
    HeavyFreighter,
    #[serde(rename = "FRAME_TRANSPORT")]
    Transport,
    #[serde(rename = "FRAME_DESTROYER")]
    Destroyer,
    #[serde(rename = "FRAME_CRUISER")]
    Cruiser,
    #[serde(rename = "FRAME_CARRIER")]
    Carrier,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_frigate_frame() -> Frame {
        Frame {
            symbol: FrameType::Frigate,
            name: string!("Frigate"),
            description: string!("A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations."),
            condition: Some(1),
            module_slots: 8,
            mounting_points: 5,
            fuel_capacity: 400,
            requirements: Requirements {
                power: Some(8),
                crew: Some(25),
                slots: None,
            },
        }
    }

    #[test]
    fn some_frigate_frame_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "FRAME_FRIGATE",
            "name": "Frigate",
            "condition": 1,
            "integrity": 1,
            "description": "A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations.",
            "moduleSlots": 8,
            "mountingPoints": 5,
            "fuelCapacity": 400,
            "requirements": {
                "power": 8,
                "crew": 25
            },
            "quality": 4
        }"#;

        let actual: Frame = serde_json::from_str(json_str).unwrap();
        let expected = some_frigate_frame();

        assert_eq!(expected, actual);
    }

    pub fn some_probe_frame() -> Frame {
        Frame {
            symbol: FrameType::Probe,
            name: string!("Probe"),
            description: string!("A small, unmanned spacecraft used for exploration, reconnaissance, and scientific research."),
            condition: Some(1),
            module_slots: 0,
            mounting_points: 0,
            fuel_capacity: 0,
            requirements: Requirements {
                power: Some(1),
                crew: Some(0),
                slots: None,
            },
        }
    }

    #[test]
    fn some_probe_frame_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "FRAME_PROBE",
            "name": "Probe",
            "condition": 1,
            "integrity": 1,
            "description": "A small, unmanned spacecraft used for exploration, reconnaissance, and scientific research.",
            "moduleSlots": 0,
            "mountingPoints": 0,
            "fuelCapacity": 0,
            "requirements": {
                "power": 1,
                "crew": 0
            },
            "quality": 1
        }"#;

        let actual: Frame = serde_json::from_str(json_str).unwrap();
        let expected = some_probe_frame();

        assert_eq!(expected, actual);
    }
}
