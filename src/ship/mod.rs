use std::sync::Arc;

use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipData {
    pub symbol: String,
    pub registration: Registration,
    pub nav: Nav,
    pub crew: Crew,
    pub frame: Frame,
    pub reactor: Reactor,
    pub engine: Engine,
    pub cooldown: Cooldown,
    pub modules: Vec<Module>,
    pub mounts: Vec<Mount>,
    pub cargo: Cargo,
    pub fuel: Fuel,
}

mod nav;
pub use nav::*;

mod crew;
pub use crew::*;

mod frame;
pub use frame::*;

mod registration;
pub use registration::*;

mod requirements;
pub use requirements::*;

mod engine;
pub use engine::*;

mod reactor;
pub use reactor::*;

mod cooldown;
pub use cooldown::*;

mod module;
pub use module::*;

mod cargo;
pub use cargo::*;

mod mount;
pub use mount::*;

mod fuel;
pub use fuel::*;

use crate::space_traders_client::{Error, SpaceTradersClient};

#[derive(Debug, PartialEq, Deserialize)]
struct ShipDataResponse {
    data: ShipData,
}

#[derive(Debug, PartialEq, Deserialize)]
struct NavChangeData {
    nav: Nav,
}

#[derive(Debug, PartialEq, Deserialize)]
struct NavChangeResponse {
    data: NavChangeData,
}

#[derive(Debug, PartialEq)]
pub struct Ship {
    symbol: String,
    client: Arc<SpaceTradersClient>,
    data: Option<ShipData>,
}

impl Ship {
    pub async fn get_ship_data(
        client: &SpaceTradersClient,
        symbol: &str,
    ) -> Result<ShipData, Error> {
        let response: ShipDataResponse = client
            .get(
                &format!("my/ships/{}", symbol),
                None::<&()>,
                reqwest::StatusCode::OK,
            )
            .await?;

        Ok(response.data)
    }

    pub async fn get_ship(client: Arc<SpaceTradersClient>, symbol: &str) -> Result<Self, Error> {
        Ok(Ship {
            data: Some(Ship::get_ship_data(&client, symbol).await?),
            symbol: symbol.to_string(),
            client: client,
        })
    }
}

impl Ship {
    pub fn new(client: Arc<SpaceTradersClient>, symbol: &str) -> Self {
        Ship {
            client: client.clone(),
            symbol: symbol.to_string(),
            data: None,
        }
    }

    pub fn with_data(client: Arc<SpaceTradersClient>, data: ShipData) -> Self {
        Ship {
            client: client.clone(),
            symbol: data.symbol.to_string(),
            data: Some(data),
        }
    }

    pub async fn get_data(&mut self) -> Result<ShipData, Error> {
        let data = match &self.data {
            Some(cached) => cached.clone(),
            None => {
                let fetched = Ship::get_ship_data(&self.client, &self.symbol).await?;
                self.data = Some(fetched.clone());
                fetched
            }
        };

        Ok(data)
    }

    pub async fn go_to_orbit(&mut self) -> Result<(), Error> {
        let response: NavChangeResponse = self
            .client
            .post(
                &format!("my/ships/{}/orbit", self.symbol),
                reqwest::StatusCode::OK,
            )
            .await?;

        self.data.as_mut().unwrap().nav = response.data.nav;

        Ok(())
    }

    pub async fn dock(&mut self) -> Result<(), Error> {
        let response: NavChangeResponse = self
            .client
            .post(
                &format!("my/ships/{}/dock", self.symbol),
                reqwest::StatusCode::OK,
            )
            .await?;

        self.data.as_mut().unwrap().nav = response.data.nav;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use mock_server::{MockServerBuilder, RequestMethod};

    use super::cargo::tests::*;
    use super::cooldown::tests::*;
    use super::crew::tests::*;
    use super::engine::tests::*;
    use super::frame::tests::*;
    use super::fuel::tests::*;
    use super::module::tests::*;
    use super::mount::tests::*;
    use super::nav::tests::nav::*;
    use super::reactor::tests::*;
    use super::*;
    use crate::faction::Factions;
    use crate::ship::registration::tests::*;
    use crate::string;

    pub fn some_ship() -> ShipData {
        ShipData {
            symbol: string!("BADGER-1"),
            registration: some_registration(),
            nav: some_nav(),
            crew: some_crew(),
            frame: some_frigate_frame(),
            reactor: some_fission_reactor(),
            engine: some_ion_drive_2(),
            cooldown: some_cooldown(string!("BADGER-1")),
            modules: vec![
                some_cargo_hold_2(),
                some_crew_quarters(),
                some_crew_quarters(),
                some_mineral_processor(),
                some_gas_processor(),
            ],
            mounts: vec![
                some_sensor_array_2(),
                some_gas_siphon_2(),
                some_mining_laser_2(),
                some_surveyor_2(),
            ],
            cargo: some_cargo(),
            fuel: some_fuel(),
        }
    }

    pub fn some_other_ship() -> ShipData {
        ShipData {
            symbol: string!("BADGER-2"),
            registration: some_other_registration(),
            nav: some_other_nav(),
            crew: empty_crew(),
            frame: some_probe_frame(),
            reactor: some_solar_reactor(),
            engine: some_impluse_drive(),
            cooldown: some_cooldown(string!("BADGER-2")),
            modules: vec![],
            mounts: vec![],
            cargo: no_capacity_cargo(),
            fuel: empty_fuel(),
        }
    }

    fn snake_ship() -> ShipData {
        ShipData {
            symbol: string!("SNAKE-1"),
            registration: Registration {
                name: string!("SNAKE-1"),
                faction_symbol: Factions::Cosmic,
                role: ShipRole::Command,
            },
            nav: Nav {
                system_symbol: string!("X1-CB91"),
                waypoint_symbol: string!("X1-CB91-A1"),
                route: Route {
                    destination: Location {
                        symbol: string!("X1-CB91-A1"),
                        location_type: LocationType::Planet,
                        system_symbol: string!("X1-CB91"),
                        x: -18,
                        y: 15,
                    },
                    origin: Location {
                        symbol: string!("X1-CB91-A1"),
                        location_type: LocationType::Planet,
                        system_symbol: string!("X1-CB91"),
                        x: -18,
                        y: 15,
                    },
                    departure_time: string!("2025-06-23T02:20:47.405Z"),
                    arrival: string!("2025-06-23T02:20:47.405Z"),
                },
                status: ShipStatus::Docked,
                flight_mode: FlightMode::Cruise,
            },
            crew: some_crew(),
            frame: some_frigate_frame(),
            reactor: some_fission_reactor(),
            engine: some_ion_drive_2(),
            cooldown: some_cooldown(string!("SNAKE-1")),
            modules: vec![
                some_cargo_hold_2(),
                some_crew_quarters(),
                some_crew_quarters(),
                some_mineral_processor(),
                some_gas_processor(),
            ],
            mounts: vec![
                some_sensor_array_2(),
                some_gas_siphon_2(),
                some_mining_laser_2(),
                some_surveyor_2(),
            ],
            cargo: some_cargo(),
            fuel: Fuel {
                current: 400,
                capacity: 400,
                consumed: Some(FuelConsumed {
                    amount: 0,
                    timestamp: string!("2025-06-23T02:20:47.405Z"),
                }),
            },
        }
    }

    #[tokio::test]
    async fn should_get_ship_data() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "my/ships/BADGER-1",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = SpaceTradersClient::with_url(&mock_server.url(), None);

        let actual = Ship::get_ship_data(&client, "BADGER-1").await.unwrap();

        let expected = some_ship();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_get_ship() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "my/ships/BADGER-1",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let actual = Ship::get_ship(client.clone(), "BADGER-1").await.unwrap();

        let expected = Ship::with_data(client.clone(), some_ship());

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_get_ship_data_with_object() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Get,
            "my/ships/BADGER-1",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let mut ship = Ship::new(client.clone(), "BADGER-1");

        let actual = ship.get_data().await.unwrap();

        let expected = some_ship();

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_send_ship_to_orbit() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Post,
            "my/ships/SNAKE-1/orbit",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let mut ship = Ship::with_data(client.clone(), snake_ship());

        let _ = ship.go_to_orbit().await;

        let actual = ship;

        let mut data = snake_ship();

        data.nav.status = ShipStatus::InOrbit;

        let expected = Ship::with_data(client.clone(), data);

        assert_eq!(expected, actual);
    }

    #[tokio::test]
    async fn should_dock_ship() {
        let mock_server = MockServerBuilder::mock_once(
            RequestMethod::Post,
            "my/ships/SNAKE-1/dock",
            200,
            None,
            None::<&()>,
        )
        .await;

        let client = Arc::new(SpaceTradersClient::with_url(&mock_server.url(), None));

        let mut data = snake_ship();
        data.nav.status = ShipStatus::InOrbit;

        let mut ship = Ship::with_data(client.clone(), data);

        let _ = ship.dock().await;

        let actual = ship;

        let expected = Ship::with_data(client.clone(), snake_ship());

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
         {
            "symbol": "BADGER-1",
            "registration": {
               "name": "BADGER-1",
               "factionSymbol": "COSMIC",
               "role": "COMMAND"
            },
            "nav": {
               "systemSymbol": "X1-RC42",
               "waypointSymbol": "X1-RC42-A1",
               "route": {
               "destination": {
                  "symbol": "X1-RC42-A1",
                  "type": "PLANET",
                  "systemSymbol": "X1-RC42",
                  "x": -22,
                  "y": -3
               },
               "origin": {
                  "symbol": "X1-RC42-A1",
                  "type": "PLANET",
                  "systemSymbol": "X1-RC42",
                  "x": -22,
                  "y": -3
               },
               "departureTime": "2025-05-29T22:47:42.914Z",
               "arrival": "2025-05-29T22:47:42.914Z"
               },
               "status": "DOCKED",
               "flightMode": "CRUISE"
            },
            "crew": {
               "current": 57,
               "required": 57,
               "capacity": 80,
               "rotation": "STRICT",
               "morale": 100,
               "wages": 0
            },
            "frame": {
               "symbol": "FRAME_FRIGATE",
               "name": "Frigate",
               "condition": 1,
               "integrity": 1,
               "description": "A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations.",
               "moduleSlots": 8,
               "mountingPoints": 5,
               "fuelCapacity": 400,
               "requirements": {
               "power": 8,
               "crew": 25
               },
               "quality": 4
            },
            "reactor": {
               "symbol": "REACTOR_FISSION_I",
               "name": "Fission Reactor I",
               "condition": 1,
               "integrity": 1,
               "description": "A basic fission power reactor, used to generate electricity from nuclear fission reactions.",
               "powerOutput": 31,
               "requirements": {
               "crew": 8
               },
               "quality": 5
            },
            "engine": {
               "symbol": "ENGINE_ION_DRIVE_II",
               "name": "Ion Drive II",
               "condition": 1,
               "integrity": 1,
               "description": "An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance.",
               "speed": 36,
               "requirements": {
               "power": 6,
               "crew": 8
               },
               "quality": 4
            },
            "modules": [
               {
               "symbol": "MODULE_CARGO_HOLD_II",
               "name": "Expanded Cargo Hold",
               "description": "An expanded cargo hold module that provides more efficient storage space for a ship's cargo.",
               "requirements": {
                  "power": 2,
                  "crew": 2,
                  "slots": 2
               },
               "capacity": 40
               },
               {
               "symbol": "MODULE_CREW_QUARTERS_I",
               "name": "Crew Quarters",
               "description": "A module that provides living space and amenities for the crew.",
               "requirements": {
                  "power": 1,
                  "crew": 2,
                  "slots": 1
               },
               "capacity": 40
               },
               {
               "symbol": "MODULE_CREW_QUARTERS_I",
               "name": "Crew Quarters",
               "description": "A module that provides living space and amenities for the crew.",
               "requirements": {
                  "power": 1,
                  "crew": 2,
                  "slots": 1
               },
               "capacity": 40
               },
               {
               "symbol": "MODULE_MINERAL_PROCESSOR_I",
               "name": "Mineral Processor",
               "description": "Crushes and processes extracted minerals and ores into their component parts, filters out impurities, and containerizes them into raw storage units.",
               "requirements": {
                  "power": 1,
                  "crew": 0,
                  "slots": 2
               }
               },
               {
               "symbol": "MODULE_GAS_PROCESSOR_I",
               "name": "Gas Processor",
               "description": "Filters and processes extracted gases into their component parts, filters out impurities, and containerizes them into raw storage units.",
               "requirements": {
                  "power": 1,
                  "crew": 0,
                  "slots": 2
               }
               }
            ],
            "mounts": [
               {
               "symbol": "MOUNT_SENSOR_ARRAY_II",
               "name": "Sensor Array II",
               "description": "An advanced sensor array that improves a ship's ability to detect and track other objects in space with greater accuracy and range.",
               "requirements": {
                  "power": 2,
                  "crew": 2
               },
               "strength": 4
               },
               {
               "symbol": "MOUNT_GAS_SIPHON_II",
               "name": "Gas Siphon II",
               "description": "An advanced gas siphon that can extract gas and other resources from gas giants and other gas-rich bodies more efficiently and at a higher rate.",
               "requirements": {
                  "power": 2,
                  "crew": 2
               },
               "strength": 20
               },
               {
               "symbol": "MOUNT_MINING_LASER_II",
               "name": "Mining Laser II",
               "description": "An advanced mining laser that is more efficient and effective at extracting valuable minerals from asteroids and other space objects.",
               "requirements": {
                  "power": 2,
                  "crew": 2
               },
               "strength": 5
               },
               {
               "symbol": "MOUNT_SURVEYOR_II",
               "name": "Surveyor II",
               "description": "An advanced survey probe that can be used to gather information about a mineral deposit with greater accuracy.",
               "requirements": {
                  "power": 3,
                  "crew": 4
               },
               "strength": 2,
               "deposits": [
                  "QUARTZ_SAND",
                  "SILICON_CRYSTALS",
                  "PRECIOUS_STONES",
                  "ICE_WATER",
                  "AMMONIA_ICE",
                  "IRON_ORE",
                  "COPPER_ORE",
                  "SILVER_ORE",
                  "ALUMINUM_ORE",
                  "GOLD_ORE",
                  "PLATINUM_ORE",
                  "DIAMONDS",
                  "URANITE_ORE"
               ]
               }
            ],
            "cargo": {
               "capacity": 40,
               "units": 0,
               "inventory": []
            },
            "fuel": {
               "current": 400,
               "capacity": 400,
               "consumed": {
               "amount": 0,
               "timestamp": "2025-05-29T22:47:42.914Z"
               }
            },
            "cooldown": {
               "shipSymbol": "BADGER-1",
               "totalSeconds": 0,
               "remainingSeconds": 0
            }
         }"#;

        let actual: ShipData = serde_json::from_str(json_str).unwrap();
        let expected = some_ship();

        assert_eq!(expected, actual);
    }

    #[test]
    fn other_ships_should_be_deserializable() {
        let json_str = r#"
      {
        "symbol": "BADGER-2",
        "registration": {
          "name": "BADGER-2",
          "factionSymbol": "COSMIC",
          "role": "SATELLITE"
        },
        "nav": {
          "systemSymbol": "X1-RC42",
          "waypointSymbol": "X1-RC42-H53",
          "route": {
            "destination": {
              "symbol": "X1-RC42-H53",
              "type": "MOON",
              "systemSymbol": "X1-RC42",
              "x": -9,
              "y": -45
            },
            "origin": {
              "symbol": "X1-RC42-H53",
              "type": "MOON",
              "systemSymbol": "X1-RC42",
              "x": -9,
              "y": -45
            },
            "departureTime": "2025-05-29T22:47:42.923Z",
            "arrival": "2025-05-29T22:47:42.923Z"
          },
          "status": "DOCKED",
          "flightMode": "CRUISE"
        },
        "crew": {
          "current": 0,
          "required": 0,
          "capacity": 0,
          "rotation": "STRICT",
          "morale": 100,
          "wages": 0
        },
        "frame": {
          "symbol": "FRAME_PROBE",
          "name": "Probe",
          "condition": 1,
          "integrity": 1,
          "description": "A small, unmanned spacecraft used for exploration, reconnaissance, and scientific research.",
          "moduleSlots": 0,
          "mountingPoints": 0,
          "fuelCapacity": 0,
          "requirements": {
            "power": 1,
            "crew": 0
          },
          "quality": 1
        },
        "reactor": {
          "symbol": "REACTOR_SOLAR_I",
          "name": "Solar Reactor I",
          "condition": 1,
          "integrity": 1,
          "description": "A basic solar power reactor, used to generate electricity from solar energy.",
          "powerOutput": 3,
          "requirements": {
            "crew": 0
          },
          "quality": 1
        },
        "engine": {
          "symbol": "ENGINE_IMPULSE_DRIVE_I",
          "name": "Impulse Drive I",
          "condition": 1,
          "integrity": 1,
          "description": "A basic low-energy propulsion system that generates thrust for interplanetary travel.",
          "speed": 9,
          "requirements": {
            "power": 1,
            "crew": 0
          },
          "quality": 1
        },
        "modules": [],
        "mounts": [],
        "cargo": {
          "capacity": 0,
          "units": 0,
          "inventory": []
        },
        "fuel": {
          "current": 0,
          "capacity": 0,
          "consumed": {
            "amount": 0,
            "timestamp": "2025-05-29T22:47:42.923Z"
          }
        },
        "cooldown": {
          "shipSymbol": "BADGER-2",
          "totalSeconds": 0,
          "remainingSeconds": 0
        }
      }"#;

        let actual: ShipData = serde_json::from_str(json_str).unwrap();
        let expected = some_other_ship();

        assert_eq!(expected, actual);
    }
}
