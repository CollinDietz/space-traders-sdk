use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
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

    pub fn some_cooldown(symbol: String) -> Cooldown {
        Cooldown {
            ship_symbol: symbol,
            total_seconds: 0,
            remaining_seconds: 0,
            expiration: None,
        }
    }

    #[test]
    pub fn some_cooldown_should_be_deserializable() {
        let json_str = r#"
        {
            "shipSymbol": "BADGER-1",
            "totalSeconds": 0,
            "remainingSeconds": 0
        }"#;

        let actual: Cooldown = serde_json::from_str(json_str).unwrap();
        let expected = some_cooldown(string!("BADGER-1"));

        assert_eq!(expected, actual);
    }
}
