use serde_derive::Deserialize;

use crate::faction::Factions;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub account_id: Option<String>,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    pub starting_faction: Factions,
    pub ship_count: Option<i32>,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::string;

    pub fn some_agent() -> Agent {
        Agent {
            account_id: Some(string!("cmb9x37zu005atm16tqkta71c")),
            symbol: string!("BADGER"),
            headquarters: string!("X1-RC42-A1"),
            credits: 175000,
            starting_faction: Factions::Cosmic,
            ship_count: Some(2),
        }
    }

    #[test]
    fn registration_response_should_be_deserializable() {
        let json_str = r#"
        {
            "accountId": "cmb9x37zu005atm16tqkta71c",
            "symbol": "BADGER",
            "headquarters": "X1-RC42-A1",
            "credits": 175000,
            "startingFaction": "COSMIC",
            "shipCount": 2
        }"#;

        let actual: Agent = serde_json::from_str(json_str).unwrap();

        let expected = some_agent();

        assert_eq!(expected, actual);
    }
}
