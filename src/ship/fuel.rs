use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fuel {
    pub current: i32,
    pub capacity: i32,
    pub consumed: Option<FuelConsumed>,
}

#[derive(Debug, PartialEq, Deserialize)]
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
            current: 1200,
            capacity: 1200,
            consumed: Some(FuelConsumed {
                amount: 0,
                timestamp: string!("2023-09-23T01:48:20.204Z"),
            }),
        }
    }

    #[test]
    pub fn should_be_deserializable() {
        let json_str = r#"
      {
          "current": 1200,
          "capacity": 1200,
          "consumed": {
          "amount": 0,
          "timestamp": "2023-09-23T01:48:20.204Z"
          }
      }"#;

        let actual: Fuel = serde_json::from_str(json_str).unwrap();
        let expected = some_fuel();

        assert_eq!(expected, actual);
    }
}
