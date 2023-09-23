use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub ship_symbol: String,
    pub total_seconds: i32,
    pub remaining_seconds: i32,
    pub expiration: Option<String>,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_cooldown() -> Cooldown {
        Cooldown {
            ship_symbol: string!("THISISATEST888-1"),
            total_seconds: 0,
            remaining_seconds: 0,
            expiration: None,
        }
    }

    #[test]
    pub fn should_be_deserializable() {
        let json_str = r#"
      {
          "shipSymbol": "THISISATEST888-1",
          "totalSeconds": 0,
          "remainingSeconds": 0
      }"#;

        let actual: Cooldown = serde_json::from_str(json_str).unwrap();
        let expected = some_cooldown();

        assert_eq!(expected, actual);
    }
}
