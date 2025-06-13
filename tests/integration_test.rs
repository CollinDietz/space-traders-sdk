use std::sync::Arc;

use dotenv::dotenv;
use space_traders_sdk::account::Account;
use space_traders_sdk::account::RegistrationRequest;
use space_traders_sdk::faction::Factions;
use space_traders_sdk::space_traders_client::SpaceTradersClient;
use space_traders_sdk::string;
use space_traders_sdk::system::waypoint::WaypointTraitSymbol;
use space_traders_sdk::system::waypoint::WaypointType;
use space_traders_sdk::system::System;

#[ignore]
#[tokio::test]
async fn can_register_agent_and_accept_contract() {
    dotenv().ok();
    let account_token = std::env::var("ACCOUNT_TOKEN").expect("ACCOUNT_TOKEN must be set.");
    let client = Arc::new(SpaceTradersClient::new(Some(account_token)));
    let account = Account::new(client);
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

#[ignore]
#[tokio::test]
async fn can_get_waypoints_from_a_system_and_get_info_for_those_way_points() {
    let client = Arc::new(SpaceTradersClient::new(None));

    let system = System::new(client, "X1-MH3");

    let waypoints_that_are_moons = system
        .list_waypoints(Some(WaypointType::Moon), None)
        .await
        .unwrap();

    println!("A Moon: {:?}\n", waypoints_that_are_moons);

    let waypoints_with_shipyards = system
        .list_waypoints(None, Some(WaypointTraitSymbol::Shipyard))
        .await
        .unwrap();

    let shipyard = waypoints_with_shipyards
        .first()
        .unwrap()
        .get_market()
        .await
        .unwrap();

    println!("A Shipyard: {:?}\n", shipyard);

    let waypoints_with_markets = system
        .list_waypoints(None, Some(WaypointTraitSymbol::Marketplace))
        .await
        .unwrap();

    let market = waypoints_with_markets
        .first()
        .unwrap()
        .get_market()
        .await
        .unwrap();

    println!("A Market: {:?}\n", market);
}
