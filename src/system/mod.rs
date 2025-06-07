use std::sync::Arc;

use crate::space_traders_client::SpaceTradersClient;

pub mod waypoint;

pub struct System {
    symbol: String,
    client: Arc<SpaceTradersClient>,
}

impl System {
    pub fn new(client: Arc<SpaceTradersClient>, symbol: &str) -> Self {
        System {
            client: client.clone(),
            symbol: symbol.into(),
        }
    }

    // pub async fn list_waypoints() -> Vec<Waypoint> {}
}
