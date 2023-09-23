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
            account_id: Some(string!("clmvdg7i7dap7s60cmpyn9tnz")),
            symbol: string!("THISISATEST888"),
            headquarters: string!("X1-GM20-33220C"),
            credits: 150000,
            starting_faction: Factions::Cosmic,
            ship_count: None,
        }
    }

    #[test]
    fn registration_response_should_be_deserializable() {
        let json_str = r#"
        {
            "accountId": "clmvdg7i7dap7s60cmpyn9tnz",
            "symbol": "THISISATEST888",
            "headquarters": "X1-GM20-33220C",
            "credits": 150000,
            "startingFaction": "COSMIC"
        }"#;

        let actual: Agent = serde_json::from_str(json_str).unwrap();

        let expected = some_agent();

        assert_eq!(expected, actual);
    }
}
