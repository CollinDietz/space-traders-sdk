use dotenv::dotenv;
use space_traders_sdk::account::Account;
use space_traders_sdk::account::RegistrationRequest;
use space_traders_sdk::faction::Factions;
use space_traders_sdk::string;

#[ignore]
#[tokio::test]
async fn can_register_agent_and_accept_contract() {
    dotenv().ok();
    let account_token = std::env::var("ACCOUNT_TOKEN").expect("ACCOUNT_TOKEN must be set.");
    let account = Account::new(account_token);
    let registration_request = RegistrationRequest {
        callsign: string!("SHOOTTEST6"),
        faction: Factions::Aegis,
    };
    let mut agent = account.register_agent(registration_request).await.unwrap();
    let contract_id = agent.list_contracts().next().cloned().unwrap();
    let contract = agent.edit_contract(&contract_id);
    contract.accept().await.unwrap();
    assert!(contract.is_accepted());
}
