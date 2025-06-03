use reqwest::Error;
use serde_derive::{Deserialize, Serialize};

use crate::agent::AgentData;
use crate::contract::ContractData;
use crate::faction::{Faction, Factions};
use crate::sdk::Sdk;
use crate::ship::Ship;

#[derive(Debug, PartialEq, Serialize)]
pub struct RegistrationRequest {
    #[serde(rename = "symbol")]
    pub callsign: String,
    pub faction: Factions,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LoginData {
    pub agent: AgentData,
    pub contract: ContractData,
    pub faction: Faction,
    pub ships: Vec<Ship>,
    pub token: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LoginResponse {
    pub data: LoginData,
}

impl Sdk {
    // pub async fn login_internal(url: String, token: String) -> Result<(Self, LoginData), Error> {
    //     let _sdk = Sdk {
    //         client: reqwest::Client::new(),
    //         url: url,
    //         token: token,
    //     };

    //     let _login_data: LoginData = LoginData {
    //         agent: todo!(),
    //         contract: todo!(),
    //         faction: todo!(),
    //         ships: todo!(),
    //         token: token,
    //     };

    //     Ok((_sdk, _login_data))
    // }

    pub async fn register(&mut self, request: RegistrationRequest) -> Result<LoginData, Error> {
        let response = self.post("register", &request).await?;

        if response.status() == 201 {
            let text = response.text().await?;
            // println!("{}", text);
            let data: LoginResponse = serde_json::from_str(text.as_str()).unwrap();
            self.add_agent_token(data.data.agent.symbol.clone(), data.data.token.clone());
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
    extern crate mock_server;
    use mock_server::*;

    use super::super::*;
    use super::*;
    use crate::agent::tests::some_agent_data;
    use crate::contract::tests::contract_data::some_contract_data;
    use crate::faction::tests::some_faction;
    use crate::ordered_json;
    use crate::ship::tests::some_other_ship;
    use crate::ship::tests::some_ship;
    use crate::string;

    fn some_token() -> String {
        "some_token".to_string()
    }

    fn some_login_data() -> LoginData {
        LoginData {
                agent: some_agent_data(),
                contract: some_contract_data(),
                faction: some_faction(),
                ships: vec![some_ship(), some_other_ship()],
                token: string!("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiQkFER0VSIiwidmVyc2lvbiI6InYyLjMuMCIsInJlc2V0X2RhdGUiOiIyMDI1LTA1LTI1IiwiaWF0IjoxNzQ4NTU4ODYyLCJzdWIiOiJhZ2VudC10b2tlbiJ9.HMkoU-6j8OI_dibjJ7tvgE15t0XFAkDraPf_r4JgTpeX9Joc6kvjljci5hvZULsHVc7R-R9DwxUZidbQhlaYkbqEUFVsZd-Ywh58l9Gn2Hc3qfmF-NjwPCVCPSfCY7AiyxSJS_8jP57Q7HS_HkzKdY7Z5AoYZ4v-ep6aiWpe6u9kPyHczbfn-1UYw3ylzgxreSSfQUDYqeLfaj95WCJg9OUYlc4TG2zSxE4Qd6NAQ_wfsEiJsV4G8YgrH6XAACBo0zTgwy4xoRMDb4zaOWGbqTQKI8WsyTeZuEuaDrzQL81tYqChQ1WhHkjKpFPNsAe501Sw2gTGyjG8elzt5ErA_yGMswZs0M4KePD9B1tjeDvyHAgZ2U6jNfh6IKyR1OK6jeFimVFBE2ffLpIRnJD_wRsTgofBx3HI8cKx15XzDGjU82p5tr1SuwyOwQQpxhcMhsoB8WNabtj1ntWX55ODWqQ7PANlfdMWfkY7hLdOTy6rfvBwMkCKZrT7hWzxJ1wzbUhOoD3XD81rMt_xBP_KlVhldRhmNYiboxhwFoVZZlkmGr347XhBi9G3k1lYUiBBVOD8-k9TMk2gROC5VXMW7KdCHD2OtY3RLe6P19audvT5r8Og3pJw_1HMF7Xnz2_PFySPHfqgvZzjiFERgkuR4v472jofcliL-bwOhuEOHu0"),
            }
    }

    fn some_login_response() -> LoginResponse {
        LoginResponse {
            data: some_login_data(),
        }
    }

    #[tokio::test]
    async fn request_should_be_sent_parsed_and_returned() {
        let request: RegistrationRequest = RegistrationRequest {
            callsign: string!("SOMEPLAYER"),
            faction: Factions::Aegis,
        };

        let mock_server = mock_response(
            RequestMethod::Post,
            "register",
            201,
            some_token(),
            Some(&request),
        )
        .await;

        let mut sdk = Sdk::with_url(mock_server.url(), some_token());

        let actual = sdk.register(request).await.unwrap();

        let expected = some_login_data();

        assert_eq!(expected, actual)
    }

    #[test]
    fn request_should_be_serializable() {
        let request = RegistrationRequest {
            callsign: string!("THISISATEST888"),
            faction: Factions::Cosmic,
        };

        let actual = serde_json::to_string(&request).unwrap();

        let expected = ordered_json! {
            "symbol" => "THISISATEST888",
            "faction" => "COSMIC"
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn response_should_be_deserializable() {
        let json_str = &std::fs::read_to_string("mock_server/responses/register_201.json").unwrap();
        let actual: LoginResponse = serde_json::from_str(json_str).unwrap();
        let expected = some_login_response();

        assert_eq!(expected, actual);
    }
}
