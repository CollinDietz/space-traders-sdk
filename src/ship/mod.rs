use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ship {
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

#[cfg(test)]
pub mod tests {
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
    use crate::ship::registration::tests::*;
    use crate::string;

    pub fn some_ship() -> Ship {
        Ship {
            symbol: string!("THISISATEST888-1"),
            registration: some_registration(),
            nav: some_nav(),
            crew: some_crew(),
            frame: some_frame(),
            reactor: some_reactor(),
            engine: some_engine(),
            cooldown: some_cooldown(),
            modules: vec![
                some_cargo_hold(),
                some_cargo_hold(),
                some_crew_quarters(),
                some_crew_quarters(),
                some_mineral_processor(),
                some_jump_drive(),
                some_warp_drive(),
            ],
            mounts: vec![some_sensor_array(), some_mining_laser(), some_surveyor()],
            cargo: some_cargo(),
            fuel: some_fuel(),
        }
    }

    #[test]
    fn should_be_deserializable() {
        let json_str = r#"
      {
          "symbol": "THISISATEST888-1",
          "nav": {
             "systemSymbol": "X1-GM20",
             "waypointSymbol": "X1-GM20-33220C",
             "route": {
                "departure": {
                   "symbol": "X1-GM20-33220C",
                   "type": "PLANET",
                   "systemSymbol": "X1-GM20",
                   "x": 5,
                   "y": -21
                },
                "origin": {
                   "symbol": "X1-GM20-33220C",
                   "type": "PLANET",
                   "systemSymbol": "X1-GM20",
                   "x": 5,
                   "y": -21
                },
                "destination": {
                   "symbol": "X1-GM20-33220C",
                   "type": "PLANET",
                   "systemSymbol": "X1-GM20",
                   "x": 5,
                   "y": -21
                },
                "arrival": "2023-09-23T01:48:20.204Z",
                "departureTime": "2023-09-23T01:48:20.204Z"
             },
             "status": "DOCKED",
             "flightMode": "CRUISE"
          },
          "crew": {
             "current": 59,
             "capacity": 80,
             "required": 59,
             "rotation": "STRICT",
             "morale": 100,
             "wages": 0
          },
          "fuel": {
             "current": 1200,
             "capacity": 1200,
             "consumed": {
                "amount": 0,
                "timestamp": "2023-09-23T01:48:20.204Z"
             }
          },
          "cooldown": {
             "shipSymbol": "THISISATEST888-1",
             "totalSeconds": 0,
             "remainingSeconds": 0
          },
          "frame": {
             "symbol": "FRAME_FRIGATE",
             "name": "Frame Frigate",
             "description": "A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations.",
             "moduleSlots": 8,
             "mountingPoints": 5,
             "fuelCapacity": 1200,
             "condition": 100,
             "requirements": {
                "power": 8,
                "crew": 25
             }
          },
          "reactor": {
             "symbol": "REACTOR_FISSION_I",
             "name": "Fission Reactor I",
             "description": "A basic fission power reactor, used to generate electricity from nuclear fission reactions.",
             "condition": 100,
             "powerOutput": 31,
             "requirements": {
                "crew": 8
             }
          },
          "engine": {
             "symbol": "ENGINE_ION_DRIVE_II",
             "name": "Ion Drive II",
             "description": "An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance.",
             "condition": 100,
             "speed": 30,
             "requirements": {
                "power": 6,
                "crew": 8
             }
          },
          "modules": [
             {
                "symbol": "MODULE_CARGO_HOLD_I",
                "name": "Cargo Hold",
                "description": "A module that increases a ship's cargo capacity.",
                "capacity": 30,
                "requirements": {
                   "crew": 0,
                   "power": 1,
                   "slots": 1
                }
             },
             {
                "symbol": "MODULE_CARGO_HOLD_I",
                "name": "Cargo Hold",
                "description": "A module that increases a ship's cargo capacity.",
                "capacity": 30,
                "requirements": {
                   "crew": 0,
                   "power": 1,
                   "slots": 1
                }
             },
             {
                "symbol": "MODULE_CREW_QUARTERS_I",
                "name": "Crew Quarters",
                "description": "A module that provides living space and amenities for the crew.",
                "capacity": 40,
                "requirements": {
                   "crew": 2,
                   "power": 1,
                   "slots": 1
                }
             },
             {
                "symbol": "MODULE_CREW_QUARTERS_I",
                "name": "Crew Quarters",
                "description": "A module that provides living space and amenities for the crew.",
                "capacity": 40,
                "requirements": {
                   "crew": 2,
                   "power": 1,
                   "slots": 1
                }
             },
             {
                "symbol": "MODULE_MINERAL_PROCESSOR_I",
                "name": "Mineral Processor",
                "description": "Crushes and processes extracted minerals and ores into their component parts, filters out impurities, and containerizes them into raw storage units.",
                "requirements": {
                   "crew": 0,
                   "power": 1,
                   "slots": 2
                }
             },
             {
                "symbol": "MODULE_JUMP_DRIVE_I",
                "name": "Jump Drive I",
                "description": "A basic antimatter jump drive that allows for instantaneous short-range interdimensional travel.",
                "range": 500,
                "requirements": {
                   "crew": 10,
                   "power": 4,
                   "slots": 1
                }
             },
             {
                "symbol": "MODULE_WARP_DRIVE_I",
                "name": "Warp Drive I",
                "description": "A basic warp drive that allows for short-range interstellar travel.",
                "range": 2000,
                "requirements": {
                   "crew": 2,
                   "power": 3,
                   "slots": 1
                }
             }
          ],
          "mounts": [
             {
                "symbol": "MOUNT_SENSOR_ARRAY_I",
                "name": "Sensor Array I",
                "description": "A basic sensor array that improves a ship's ability to detect and track other objects in space.",
                "strength": 1,
                "requirements": {
                   "crew": 0,
                   "power": 1
                }
             },
             {
                "symbol": "MOUNT_MINING_LASER_I",
                "name": "Mining Laser I",
                "description": "A basic mining laser that can be used to extract valuable minerals from asteroids and other space objects.",
                "strength": 10,
                "requirements": {
                   "crew": 0,
                   "power": 1
                }
             },
             {
                "symbol": "MOUNT_SURVEYOR_I",
                "name": "Surveyor I",
                "description": "A basic survey probe that can be used to gather information about a mineral deposit.",
                "strength": 1,
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
                   "PLATINUM_ORE"
                ],
                "requirements": {
                   "crew": 2,
                   "power": 1
                }
             }
          ],
          "registration": {
             "name": "THISISATEST888-1",
             "factionSymbol": "COSMIC",
             "role": "COMMAND"
          },
          "cargo": {
             "capacity": 60,
             "units": 0,
             "inventory": []
          }
       }"#;

        let actual: Ship = serde_json::from_str(json_str).unwrap();
        let expected = some_ship();

        assert_eq!(expected, actual);
    }
}
