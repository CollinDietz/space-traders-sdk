use std::{
    collections::{hash_map::Keys, HashMap},
    sync::Arc,
};

use serde_derive::Deserialize;

use crate::{
    account::RegistrationResponseData, contract::Contract, faction::Factions,
    space_traders_client::SpaceTradersClient,
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

#[derive(Debug, PartialEq)]
pub struct Agent {
    client: Arc<SpaceTradersClient>,
    pub data: AgentData,
    contracts: HashMap<String, Contract>,
}

impl Agent {
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
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use crate::{account::tests::some_registration_response_data, string};

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
