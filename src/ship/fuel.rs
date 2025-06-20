use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fuel {
    pub current: i32,
    pub capacity: i32,
    pub consumed: Option<FuelConsumed>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuelConsumed {
    pub amount: i32,
    pub timestamp: String,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_fuel() -> Fuel {
        Fuel {
            current: 400,
            capacity: 400,
            consumed: Some(FuelConsumed {
                amount: 0,
                timestamp: string!("2025-05-29T22:47:42.914Z"),
            }),
        }
    }

    #[test]
    pub fn some_fuel_should_be_deserializable() {
        let json_str = r#"
        {
            "current": 400,
            "capacity": 400,
            "consumed": {
                "amount": 0,
                "timestamp": "2025-05-29T22:47:42.914Z"
            }
        }"#;

        let actual: Fuel = serde_json::from_str(json_str).unwrap();
        let expected = some_fuel();

        assert_eq!(expected, actual);
    }

    pub fn empty_fuel() -> Fuel {
        Fuel {
            current: 0,
            capacity: 0,
            consumed: Some(FuelConsumed {
                amount: 0,
                timestamp: string!("2025-05-29T22:47:42.923Z"),
            }),
        }
    }

    #[test]
    pub fn empty_fuel_should_be_deserializable() {
        let json_str = r#"
        {
            "current": 0,
            "capacity": 0,
            "consumed": {
                "amount": 0,
                "timestamp": "2025-05-29T22:47:42.923Z"
            }
        }"#;

        let actual: Fuel = serde_json::from_str(json_str).unwrap();
        let expected = empty_fuel();

        assert_eq!(expected, actual);
    }
}
