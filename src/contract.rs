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
            id: string!("clmvdg7k7dap9s60coav75shp"),
            faction: Factions::Cosmic,
            contract_type: ContractType::Procurement,
            accepted: false,
            fulfilled: false,
            _deprecated: string!("2023-09-24T01:48:20.149Z"),
            deadline_to_accept: Some(string!("2023-09-24T01:48:20.149Z")),
            terms: Terms {
                deadline: string!("2023-09-30T01:48:20.149Z"),
                payment: Payment {
                    on_accepted: 23229,
                    on_fulfilled: 113742,
                },
                deliver: Some(vec![Deliver {
                    trade_symbol: string!("ALUMINUM_ORE"),
                    destination_symbol: string!("X1-GM20-97222X"),
                    units_required: 8900,
                    units_fulfilled: 0,
                }]),
            },
        }
    }

    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
        {
            "id": "clmvdg7k7dap9s60coav75shp",
            "factionSymbol": "COSMIC",
            "type": "PROCUREMENT",
            "terms": {
            "deadline": "2023-09-30T01:48:20.149Z",
            "payment": {
                "onAccepted": 23229,
                "onFulfilled": 113742
            },
            "deliver": [
                {
                    "tradeSymbol": "ALUMINUM_ORE",
                    "destinationSymbol": "X1-GM20-97222X",
                    "unitsRequired": 8900,
                    "unitsFulfilled": 0
                }
            ]
            },
            "accepted": false,
            "fulfilled": false,
            "expiration": "2023-09-24T01:48:20.149Z",
            "deadlineToAccept": "2023-09-24T01:48:20.149Z"
        }"#;

        let actual: Contract = serde_json::from_str(json_str).unwrap();

        let expected = some_contract();

        assert_eq!(expected, actual);
    }
}
