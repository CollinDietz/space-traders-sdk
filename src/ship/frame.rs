use serde_derive::Deserialize;

use super::Requirements;

#[derive(Debug, PartialEq, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
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

    pub fn some_frame() -> Frame {
        Frame {
            symbol: FrameType::Frigate,
            name: string!("Frame Frigate"),
            description: string!("A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations."),
            condition: Some(100),
            module_slots: 8,
            mounting_points: 5,
            fuel_capacity: 1200,
            requirements: Requirements {
                power: Some(8),
                crew: Some(25),
                slots: None,
            },
        }
    }

    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
            {
                "symbol": "FRAME_FRIGATE",
                "name": "Frame Frigate",
                "description": "A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations.",
                "moduleSlots": 8,
                "mountingPoints": 5,
                "fuelCapacity": 1200,
                "condition": 100,
                "requirements": {
                "power": 8,
                "crew": 25
                }
            }"#;

        let actual: Frame = serde_json::from_str(json_str).unwrap();
        let expected = some_frame();

        assert_eq!(expected, actual);
    }
}
