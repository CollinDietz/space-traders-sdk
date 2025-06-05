use std::sync::Arc;

use reqwest::StatusCode;
use serde_derive::Deserialize;

use crate::{
    agent::AgentData,
    faction::Factions,
    space_traders_client::{Error, SpaceTradersClient},
};

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContractData {
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContractType {
    Procurement,
    Transport,
    Shuttle,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Terms {
    pub deadline: String, // date time?
    pub payment: Payment,
    pub deliver: Option<Vec<Deliver>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub on_accepted: i32,
    pub on_fulfilled: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deliver {
    pub trade_symbol: String,
    pub destination_symbol: String,
    pub units_required: i32,
    pub units_fulfilled: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ContractAcceptResponseData {
    pub contract: ContractData,
    pub agent: AgentData,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ContractAcceptResponse {
    pub data: ContractAcceptResponseData,
}

#[derive(Debug, PartialEq)]
pub struct Contract {
    client: Arc<SpaceTradersClient>,
    pub data: ContractData,
}

impl Contract {
    pub fn new(client: Arc<SpaceTradersClient>, data: ContractData) -> Self {
        Contract { client, data }
    }

    pub fn is_accepted(&self) -> bool {
        self.data.accepted
    }

    pub async fn accept(&self) -> Result<Contract, Error> {
        let response: ContractAcceptResponse = self
            .client
            .post(
                &format!("my/contracts/{}/accept", &self.data.id),
                StatusCode::OK,
            )
            .await?;

        Ok(Contract::new(self.client.clone(), response.data.contract))
    }
}

#[cfg(test)]
pub mod tests {
    pub mod contract_data {
        use super::super::*;
        use crate::string;

        pub fn some_contract_data() -> ContractData {
            ContractData {
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
        pub fn some_accepted_contract_data() -> ContractData {
            let contract_data = some_contract_data();
            ContractData {
                accepted: true,
                ..contract_data
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

            let actual: ContractData = serde_json::from_str(json_str).unwrap();

            let expected = some_contract_data();

            assert_eq!(expected, actual);
        }
    }

    pub mod contract {
        use std::sync::Arc;

        use mock_server::{mock_response, RequestMethod};

        use crate::{
            contract::{
                tests::contract_data::{some_accepted_contract_data, some_contract_data},
                Contract,
            },
            space_traders_client::SpaceTradersClient,
            string,
        };

        fn some_token() -> String {
            string!("some_token")
        }

        #[test]
        fn should_be_constructable_from_contract_data() {
            let contract =
                Contract::new(Arc::new(SpaceTradersClient::new("")), some_contract_data());
            assert!(!contract.is_accepted())
        }

        #[tokio::test]
        async fn when_made_with_contract_data_should_be_acceptable() {
            let data = some_contract_data();

            let mock_server = mock_response::<serde_json::Value>(
                RequestMethod::Post,
                &format!("my/contracts/{}/accept", &data.id),
                200,
                Some(some_token()),
                None,
            )
            .await;

            let space_traders_client =
                SpaceTradersClient::with_url(&mock_server.url(), &some_token());
            let contract = Contract::new(Arc::new(space_traders_client), data);
            let accepted_contract = contract.accept().await.unwrap();

            assert!(!contract.is_accepted());
            assert_eq!(accepted_contract.data, some_accepted_contract_data());
            assert!(accepted_contract.is_accepted());
        }
    }
}
