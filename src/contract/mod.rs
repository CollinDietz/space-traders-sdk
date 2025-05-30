use serde_derive::Deserialize;

use crate::faction::Factions;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: String,
    #[serde(rename = "factionSymbol")]
    pub faction: Factions,
    #[serde(rename = "type")]
    pub contract_type: ContractType,
    pub terms: Terms,
    pub accepted: bool,
    pub fulfilled: bool,
    #[serde(rename = "expiration")]
    pub _deprecated: String, // date time? deprecated
    pub deadline_to_accept: Option<String>, // date time?
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms {
    pub deadline: String, // date time?
    pub payment: Payment,
    pub deliver: Option<Vec<Deliver>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub on_accepted: i32,
    pub on_fulfilled: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deliver {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i32,
    pub units_fulfilled: i32,
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::string;

    pub fn some_contract() -> Contract {
        Contract {
            id: string!("cmb9ysth4mqyfuo6x6jh4jk9w"),
            faction: Factions::Cosmic,
            contract_type: ContractType::Procurement,
            accepted: false,
            fulfilled: false,
            _deprecated: string!("2025-05-30T22:47:42.900Z"),
            deadline_to_accept: Some(string!("2025-05-30T22:47:42.900Z")),
            terms: Terms {
                deadline: string!("2025-06-05T22:47:42.900Z"),
                payment: Payment {
                    on_accepted: 1690,
                    on_fulfilled: 8276,
                },
                deliver: Some(vec![Deliver {
                    trade_symbol: string!("IRON_ORE"),
                    destination_symbol: string!("X1-RC42-H52"),
                    units_required: 62,
                    units_fulfilled: 0,
                }]),
            },
        }
    }

    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
        {
            "id": "cmb9ysth4mqyfuo6x6jh4jk9w",
            "factionSymbol": "COSMIC",
            "type": "PROCUREMENT",
            "terms": {
                "deadline": "2025-06-05T22:47:42.900Z",
                "payment": {
                "onAccepted": 1690,
                "onFulfilled": 8276
                },
                "deliver": [
                {
                    "tradeSymbol": "IRON_ORE",
                    "destinationSymbol": "X1-RC42-H52",
                    "unitsRequired": 62,
                    "unitsFulfilled": 0
                }
                ]
            },
            "accepted": false,
            "fulfilled": false,
            "expiration": "2025-05-30T22:47:42.900Z",
            "deadlineToAccept": "2025-05-30T22:47:42.900Z"
        }"#;

        let actual: Contract = serde_json::from_str(json_str).unwrap();

        let expected = some_contract();

        assert_eq!(expected.terms, actual.terms);
        assert_eq!(expected, actual);
    }
}
