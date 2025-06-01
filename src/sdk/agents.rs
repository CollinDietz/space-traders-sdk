use reqwest::Error;
use serde_derive::Deserialize;

use crate::agent::Agent;

use super::Sdk;

#[derive(Debug, PartialEq, Deserialize)]
pub struct MyAgentResponse {
    pub data: Agent,
}

impl Sdk {
    pub async fn get_agent(&self, callsign: String) -> Result<Agent, Error> {
        let response = self
            .get("my/agent", self.agent_tokens.get(&callsign).unwrap())
            .await?;

        if response.status() == 200 {
            let text = response.text().await?;
            // println!("{}", text);
            let data: MyAgentResponse = serde_json::from_str(text.as_str()).unwrap();
            Ok(data.data)
        } else {
            println!("{}", response.status());
            let error_text = response.text().await?;
            println!("{}", error_text);
            todo!()
        }
    }
}

#[cfg(test)]
pub mod tests {
    use mock_server::{mock_response, RequestMethod};

    use crate::{agent::tests::some_agent, sdk::Sdk, string};

    use super::MyAgentResponse;

    fn some_my_agent_response() -> MyAgentResponse {
        MyAgentResponse { data: some_agent() }
    }

    fn some_account_token() -> String {
        "some_account_token".to_string()
    }

    fn some_agent_token() -> String {
        "some_agent_token".to_string()
    }

    #[tokio::test]
    async fn request_should_be_sent_parsed_and_returned() {
        let mock_server =
            mock_response(RequestMethod::Get, "my/agent", 200, some_agent_token()).await;

        let mut sdk = Sdk::with_url(mock_server.url(), some_account_token());
        sdk.add_agent_token(string!("BADGER"), some_agent_token());

        let actual = sdk.get_agent(string!("BADGER")).await.unwrap();

        let expected = some_agent();

        assert_eq!(expected, actual)
    }

    #[test]
    fn response_should_be_deserializable() {
        let json_str = &std::fs::read_to_string("mock_server/responses/my_agent_200.json").unwrap();
        let actual: MyAgentResponse = serde_json::from_str(json_str).unwrap();
        let expected = some_my_agent_response();

        assert_eq!(expected, actual);
    }
}
