use core::str;
use std::{
    collections::{hash_map::Keys, HashMap},
    sync::Arc,
};

use serde_derive::Deserialize;

use crate::{
    account::RegistrationResponseData,
    contract::{Contract, ContractData},
    faction::Factions,
    space_traders_client::{Error, SpaceTradersClient},
};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AgentData {
    pub account_id: Option<String>,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    pub starting_faction: Factions,
    pub ship_count: Option<i32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AgentDataResponse {
    data: AgentData,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct ContractDataResponse {
    data: Vec<ContractData>,
}

#[derive(Debug, PartialEq)]
pub struct Agent {
    client: Arc<SpaceTradersClient>,
    pub data: AgentData,
    contracts: HashMap<String, Contract>,
}

impl Agent {
    pub async fn new(client: Arc<SpaceTradersClient>) -> Result<Self, Error> {
        let agent_data = Agent::get_agent_data(&client).await?;
        let contract_data = Agent::get_contracts_data(&client).await?;

        let contracts: HashMap<String, Contract> = contract_data
            .into_iter()
            .map(|data| (data.id.clone(), Contract::new(client.clone(), data)))
            .collect();

        Ok(Agent {
            client: client,
            data: agent_data,
            contracts: contracts,
        })
    }

    pub fn from_registration_data(
        origin_client: &SpaceTradersClient,
        data: RegistrationResponseData,
    ) -> Self {
        let client = Arc::new(SpaceTradersClient::clone_with_token(
            origin_client,
            &data.token,
        ));

        Agent {
            client: client.clone(),
            contracts: HashMap::from([(
                data.contract.id.clone(),
                Contract::new(client.clone(), data.contract),
            )]),
            data: data.agent,
        }
    }

    pub fn list_contracts(&self) -> Keys<'_, String, Contract> {
        self.contracts.keys().clone()
    }

    pub fn edit_contract(&mut self, id: &str) -> &mut Contract {
        self.contracts.get_mut(id).unwrap()
    }

    pub async fn get_agent_data(client: &SpaceTradersClient) -> Result<AgentData, Error> {
        let response: AgentDataResponse = client
            .get("my/agent", None::<&()>, reqwest::StatusCode::OK)
            .await?;

        Ok(response.data)
    }

    pub async fn get_contracts_data(
        client: &SpaceTradersClient,
    ) -> Result<Vec<ContractData>, Error> {
        let response: ContractDataResponse = client
            .get("my/contracts", None::<&()>, reqwest::StatusCode::OK)
            .await?;

        Ok(response.data)
    }
}

#[cfg(test)]
pub mod tests {
    use mock_server::{MockServerBuilder, RequestMethod};

    use super::*;

    use crate::{
        account::tests::some_registration_response_data,
        contract::tests::contract_data::some_contract_data, string,
    };

    fn some_token() -> Option<String> {
        Some(string!("token"))
    }

    pub fn some_agent_data() -> AgentData {
        AgentData {
            account_id: Some(string!("cmb9x37zu005atm16tqkta71c")),
            symbol: string!("BADGER"),
            headquarters: string!("X1-RC42-A1"),
            credits: 175000,
            starting_faction: Factions::Cosmic,
            ship_count: Some(2),
        }
    }

    #[tokio::test]
    async fn should_get_agent_data_with_just_a_client() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "my/agent",
            200,
            some_token(),
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(
            &mock_server.url(),
            some_token(),
        ));

        let actual = Agent::get_agent_data(&client).await.unwrap();

        let expected = some_agent_data();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_get_contract_data_with_just_a_client() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "my/contracts",
            200,
            some_token(),
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(
            &mock_server.url(),
            some_token(),
        ));

        let actual = Agent::get_contracts_data(&client).await.unwrap();

        let expected = vec![some_contract_data()];

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn agent_should_be_constructable_with_only_a_client() {
        let mock_server = MockServerBuilder::new()
            .await
            .mock_response(
                RequestMethod::Get,
                "my/agent",
                200,
                some_token(),
                None::<&()>,
            )
            .await
            .mock_response(
                RequestMethod::Get,
                "my/contracts",
                200,
                some_token(),
                None::<&()>,
            )
            .await
            .build();

        let client = Arc::new(SpaceTradersClient::with_url(
            &mock_server.url(),
            some_token(),
        ));

        let actual = Agent::new(client.clone()).await.unwrap();

        let expected = Agent {
            client: client.clone(),
            data: some_agent_data(),
            contracts: HashMap::from([(
                some_contract_data().id,
                Contract::new(client.clone(), some_contract_data()),
            )]),
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn agent_data_should_be_deserializable() {
        let json_str = r#"
        {
            "accountId": "cmb9x37zu005atm16tqkta71c",
            "symbol": "BADGER",
            "headquarters": "X1-RC42-A1",
            "credits": 175000,
            "startingFaction": "COSMIC",
            "shipCount": 2
        }"#;

        let actual: AgentData = serde_json::from_str(json_str).unwrap();

        let expected = some_agent_data();

        assert_eq!(expected, actual);
    }

    #[test]
    fn agent_should_be_constructable_with_registration_data() {
        let _ = Agent::from_registration_data(
            &SpaceTradersClient::new(Some("".to_string())),
            some_registration_response_data(),
        );
    }

    #[test]
    fn agent_should_be_constructable_with_agent_data_and_be_able_to_list_contracts() {
        let data = some_registration_response_data();

        let expected = vec![&data.contract.id];

        let agent = Agent::from_registration_data(
            &SpaceTradersClient::new(Some("".to_string())),
            some_registration_response_data(),
        );

        assert_eq!(expected, agent.list_contracts().collect::<Vec<&String>>());
    }
}
