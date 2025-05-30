use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crew {
    pub current: i32,
    pub required: i32,
    pub capacity: i32,
    pub rotation: RotationMode,
    pub morale: i32,
    pub wages: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RotationMode {
    Strict,
    Relaxed,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn some_crew() -> Crew {
        Crew {
            current: 57,
            required: 57,
            capacity: 80,
            rotation: RotationMode::Strict,
            morale: 100,
            wages: 0,
        }
    }

    #[test]
    fn some_crew_should_be_deserializable() {
        let json_str = r#"
        {
            "current": 57,
            "required": 57,
            "capacity": 80,
            "rotation": "STRICT",
            "morale": 100,
            "wages": 0
        }"#;

        let actual: Crew = serde_json::from_str(json_str).unwrap();
        let expected = some_crew();

        assert_eq!(expected, actual);
    }

    pub fn empty_crew() -> Crew {
        Crew {
            current: 0,
            required: 0,
            capacity: 0,
            rotation: RotationMode::Strict,
            morale: 100,
            wages: 0,
        }
    }

    #[test]
    fn empty_crew_should_be_deserializable() {
        let json_str = r#"
        {
          "current": 0,
          "required": 0,
          "capacity": 0,
          "rotation": "STRICT",
          "morale": 100,
          "wages": 0
        }"#;

        let actual: Crew = serde_json::from_str(json_str).unwrap();
        let expected = empty_crew();

        assert_eq!(expected, actual);
    }
}
