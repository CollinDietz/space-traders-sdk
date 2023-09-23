#[macro_use]
mod utils;

mod tests {
    mod ship {
        use space_traders_api::ship::Ship;

        use self::{
            cargo::some_cargo,
            cooldown::some_cooldown,
            crew::some_crew,
            engine::some_engine,
            frame::some_frame,
            fuel::some_fuel,
            module::{
                some_cargo_hold, some_crew_quarters, some_jump_drive, some_mineral_processor,
                some_warp_drive,
            },
            mounts::{some_mining_laser, some_sensor_array, some_surveyor},
            nav::some_nav,
            reactor::some_reactor,
            registration::some_registration,
        };

        mod registration {
            use space_traders_api::ship::*;

            pub fn some_registration() -> Registration {
                Registration {
                    name: string!("THISISATEST888-1"),
                    faction_symbol: space_traders_api::faction::Factions::Cosmic,
                    role: ShipRole::Command,
                }
            }

            #[test]
            fn should_be_deserializable() {
                let json_str = r#"
             {
                "name": "THISISATEST888-1",
                "factionSymbol": "COSMIC",
                "role": "COMMAND"
             }
             "#;

                let actual: Registration = serde_json::from_str(json_str).unwrap();
                let expected = some_registration();

                assert_eq!(expected, actual);
            }
        }

        mod nav {
            use space_traders_api::ship::{FlightMode, Nav, ShipStatus};

            use self::route::some_route;
            mod route {
                use self::location::some_location;
                use space_traders_api::ship::Route;

                mod location {
                    use space_traders_api::ship::{Location, LocationType};

                    pub fn some_location() -> Location {
                        Location {
                            symbol: string!("X1-GM20-33220C"),
                            location_type: LocationType::Planet,
                            system_symbol: string!("X1-GM20"),
                            x: 5,
                            y: -21,
                        }
                    }

                    #[test]
                    fn should_be_deserializable() {
                        let json_str = r#"
                        {
                           "symbol": "X1-GM20-33220C",
                           "type": "PLANET",
                           "systemSymbol": "X1-GM20",
                           "x": 5,
                           "y": -21
                        }
                        "#;

                        let actual: Location = serde_json::from_str(json_str).unwrap();
                        let expected = some_location();

                        assert_eq!(expected, actual);
                    }
                }

                pub fn some_route() -> Route {
                    Route {
                        destination: some_location(),
                        departure: some_location(),
                        origin: some_location(),
                        departure_time: string!("2023-09-23T01:48:20.204Z"),
                        arrival: string!("2023-09-23T01:48:20.204Z"),
                    }
                }

                #[test]
                fn should_be_deserializable() {
                    let json_str = r#"
                    {
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
                     }"#;

                    let actual: Route = serde_json::from_str(json_str).unwrap();
                    let expected = some_route();

                    assert_eq!(expected, actual);
                }
            }

            pub fn some_nav() -> Nav {
                Nav {
                    system_symbol: string!("X1-GM20"),
                    waypoint_symbol: string!("X1-GM20-33220C"),
                    route: some_route(),
                    status: ShipStatus::Docked,
                    flight_mode: FlightMode::Cruise,
                }
            }

            #[test]
            fn should_be_deserializable() {
                let json_str = r#"
               {
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
               }"#;

                let actual: Nav = serde_json::from_str(json_str).unwrap();
                let expected = some_nav();

                assert_eq!(expected, actual);
            }
        }

        mod crew {
            use space_traders_api::ship::*;

            pub fn some_crew() -> Crew {
                Crew {
                    current: 59,
                    required: 59,
                    capacity: 80,
                    rotation: RotationMode::Strict,
                    morale: 100,
                    wages: 0,
                }
            }

            #[test]
            fn should_be_deserializable() {
                let json_str = r#"
               {
                  "current": 59,
                  "capacity": 80,
                  "required": 59,
                  "rotation": "STRICT",
                  "morale": 100,
                  "wages": 0
               }"#;

                let actual: Crew = serde_json::from_str(json_str).unwrap();
                let expected = some_crew();

                assert_eq!(expected, actual);
            }
        }

        mod frame {
            use space_traders_api::ship::{Frame, FrameType, Requirements};

            pub fn some_frame() -> Frame {
                Frame {
                    symbol: FrameType::Frigate,
                    name: string!("Frame Frigate"),
                    description: string!("A medium-sized, multi-purpose spacecraft, often used for combat, transport, or support operations."),
                    condition: Some(100),
                    module_slots: 8,
                    mounting_points: 5,
                    fuel_capacity: 1200,
                    requirements: Requirements {
                        power: Some(8),
                        crew: Some(25),
                        slots: None,
                    },
                }
            }

            #[test]
            fn should_be_deserializable() {
                let json_str = r#"
                    {
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
                    }"#;

                let actual: Frame = serde_json::from_str(json_str).unwrap();
                let expected = some_frame();

                assert_eq!(expected, actual);
            }
        }

        mod reactor {
            use space_traders_api::ship::Reactor;

            pub fn some_reactor() -> Reactor {
                Reactor {
                    symbol: space_traders_api::ship::ReactorType::FissionI,
                    name: string!("Fission Reactor I"),
                    description: string!("A basic fission power reactor, used to generate electricity from nuclear fission reactions."),
                    condition: Some(100),
                    power_output: 31,
                    requirements: space_traders_api::ship::Requirements {
                        power: None,
                        crew: Some(8),
                        slots: None,
                    },
                }
            }

            #[test]
            fn should_be_deserializable() {
                let json_str = r#"
                    {
                        "symbol": "REACTOR_FISSION_I",
                        "name": "Fission Reactor I",
                        "description": "A basic fission power reactor, used to generate electricity from nuclear fission reactions.",
                        "condition": 100,
                        "powerOutput": 31,
                        "requirements": {
                            "crew": 8
                        }
                    }"#;

                let actual: Reactor = serde_json::from_str(json_str).unwrap();
                let expected = some_reactor();

                assert_eq!(expected, actual);
            }
        }

        mod engine {
            use space_traders_api::ship::{Engine, Requirements};

            pub fn some_engine() -> Engine {
                Engine {
                    symbol: space_traders_api::ship::EngineType::IonDriveII,
                    name: string!("Ion Drive II"),
                    description: string!("An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance."),
                    condition: Some(100),
                    speed: 30,
                    requirements: Requirements {
                        power: Some(6),
                        crew: Some(8),
                        slots: None,
                    },
                }
            }

            #[test]
            fn should_be_deserializable() {
                let json_str = r#"
                    {
                        "symbol": "ENGINE_ION_DRIVE_II",
                        "name": "Ion Drive II",
                        "description": "An advanced propulsion system that uses ionized particles to generate high-speed, low-thrust acceleration, with improved efficiency and performance.",
                        "condition": 100,
                        "speed": 30,
                        "requirements": {
                        "power": 6,
                        "crew": 8
                        }
                    }"#;

                let actual: Engine = serde_json::from_str(json_str).unwrap();
                let expected = some_engine();

                assert_eq!(expected, actual);
            }
        }

        mod cooldown {
            use space_traders_api::ship::Cooldown;

            pub fn some_cooldown() -> Cooldown {
                Cooldown {
                    ship_symbol: string!("THISISATEST888-1"),
                    total_seconds: 0,
                    remaining_seconds: 0,
                    expiration: None,
                }
            }

            #[test]
            pub fn should_be_deserializable() {
                let json_str = r#"
                {
                    "shipSymbol": "THISISATEST888-1",
                    "totalSeconds": 0,
                    "remainingSeconds": 0
                }"#;

                let actual: Cooldown = serde_json::from_str(json_str).unwrap();
                let expected = some_cooldown();

                assert_eq!(expected, actual);
            }
        }

        mod module {
            use space_traders_api::ship::{Module, ModuleType, Requirements};

            pub fn some_cargo_hold() -> Module {
                Module {
                    symbol: ModuleType::CargoHoldI,
                    capacity: Some(30),
                    range: None,
                    name: string!("Cargo Hold"),
                    description: string!("A module that increases a ship's cargo capacity."),
                    requirements: Requirements {
                        power: Some(1),
                        crew: Some(0),
                        slots: Some(1),
                    },
                }
            }

            #[test]
            fn cargo_hold_should_be_deserializable() {
                let json_str = r#"
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
                    }"#;

                let actual: Module = serde_json::from_str(json_str).unwrap();
                let expected = some_cargo_hold();

                assert_eq!(expected, actual);
            }

            pub fn some_crew_quarters() -> Module {
                Module {
                    symbol: ModuleType::CrewQuartersI,
                    capacity: Some(40),
                    range: None,
                    name: string!("Crew Quarters"),
                    description: string!(
                        "A module that provides living space and amenities for the crew."
                    ),
                    requirements: Requirements {
                        power: Some(1),
                        crew: Some(2),
                        slots: Some(1),
                    },
                }
            }

            #[test]
            fn crew_quarters_should_be_deserializable() {
                let json_str = r#"
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
                 }"#;

                let actual: Module = serde_json::from_str(json_str).unwrap();
                let expected = some_crew_quarters();

                assert_eq!(expected, actual);
            }

            pub fn some_mineral_processor() -> Module {
                Module {
                    symbol: ModuleType::MineralProcessorI,
                    capacity: None,
                    range: None,
                    name: string!("Mineral Processor"),
                    description: string!("Crushes and processes extracted minerals and ores into their component parts, filters out impurities, and containerizes them into raw storage units."),
                    requirements: Requirements {
                        power: Some(1),
                        crew: Some(0),
                        slots: Some(2),
                    },
                }
            }

            #[test]
            fn mineral_processor_should_be_deserializable() {
                let json_str = r#"
                    {
                        "symbol": "MODULE_MINERAL_PROCESSOR_I",
                        "name": "Mineral Processor",
                        "description": "Crushes and processes extracted minerals and ores into their component parts, filters out impurities, and containerizes them into raw storage units.",
                        "requirements": {
                            "crew": 0,
                            "power": 1,
                            "slots": 2
                        }
                    }"#;

                let actual: Module = serde_json::from_str(json_str).unwrap();
                let expected = some_mineral_processor();

                assert_eq!(expected, actual);
            }

            pub fn some_jump_drive() -> Module {
                Module {
                    symbol: ModuleType::JumpDriveI,
                    capacity: None,
                    range: Some(500),
                    name: string!("Jump Drive I"),
                    description: string!("A basic antimatter jump drive that allows for instantaneous short-range interdimensional travel."),
                    requirements: Requirements {
                        power: Some(4),
                        crew: Some(10),
                        slots: Some(1),
                    },
                }
            }

            #[test]
            fn jump_drive_should_be_deserializable() {
                let json_str = r#"
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
                    }"#;

                let actual: Module = serde_json::from_str(json_str).unwrap();
                let expected = some_jump_drive();

                assert_eq!(expected, actual);
            }

            pub fn some_warp_drive() -> Module {
                Module {
                    symbol: ModuleType::WarpDriveI,
                    capacity: None,
                    range: Some(2000),
                    name: string!("Warp Drive I"),
                    description: string!(
                        "A basic warp drive that allows for short-range interstellar travel."
                    ),
                    requirements: Requirements {
                        power: Some(3),
                        crew: Some(2),
                        slots: Some(1),
                    },
                }
            }

            #[test]
            fn warp_drive_should_be_deserializable() {
                let json_str = r#"
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
                    }"#;

                let actual: Module = serde_json::from_str(json_str).unwrap();
                let expected = some_warp_drive();

                assert_eq!(expected, actual);
            }
        }

        mod mounts {
            use space_traders_api::ship::{Mount, MountType, Requirements, ResourceType};

            pub fn some_sensor_array() -> Mount {
                Mount {
                    symbol: MountType::SensorArrayI,
                    name: string!("Sensor Array I"),
                    description: Some(string!("A basic sensor array that improves a ship's ability to detect and track other objects in space.")),
                    strength: Some(1),
                    deposits: None,
                    requirements: Requirements {
                        power: Some(1),
                        crew: Some(0),
                        slots: None,
                    },
                }
            }

            #[test]
            fn sensor_array_should_be_deserializable() {
                let json_str = r#"
                {
                    "symbol": "MOUNT_SENSOR_ARRAY_I",
                    "name": "Sensor Array I",
                    "description": "A basic sensor array that improves a ship's ability to detect and track other objects in space.",
                    "strength": 1,
                    "requirements": {
                       "crew": 0,
                       "power": 1
                    }
                 }"#;

                let actual: Mount = serde_json::from_str(json_str).unwrap();
                let expected = some_sensor_array();

                assert_eq!(expected, actual);
            }

            pub fn some_mining_laser() -> Mount {
                Mount {
                    symbol: MountType::MiningLaserI,
                    name: string!("Mining Laser I"),
                    description: Some(string!("A basic mining laser that can be used to extract valuable minerals from asteroids and other space objects.")),
                    strength: Some(10),
                    deposits: None,
                    requirements: Requirements {
                        power: Some(1),
                        crew: Some(0),
                        slots: None,
                    },
                }
            }

            #[test]
            fn mining_laser_should_be_deserializable() {
                let json_str = r#"
                {
                    "symbol": "MOUNT_MINING_LASER_I",
                    "name": "Mining Laser I",
                    "description": "A basic mining laser that can be used to extract valuable minerals from asteroids and other space objects.",
                    "strength": 10,
                    "requirements": {
                       "crew": 0,
                       "power": 1
                    }
                 }"#;

                let actual: Mount = serde_json::from_str(json_str).unwrap();
                let expected = some_mining_laser();

                assert_eq!(expected, actual);
            }

            pub fn some_surveyor() -> Mount {
                Mount {
                    symbol: MountType::SurveyorI,
                    name: string!("Surveyor I"),
                    description: Some(string!("A basic survey probe that can be used to gather information about a mineral deposit.")),
                    strength: Some(1),
                    deposits: Some(vec![
                        ResourceType::QuartzSand,
                        ResourceType::SiliconCrystals,
                        ResourceType::PreciousStones,
                        ResourceType::IceWater,
                        ResourceType::AmmoniaIce,
                        ResourceType::IronOre,
                        ResourceType::CopperOre,
                        ResourceType::SilverOre,
                        ResourceType::AluminumOre,
                        ResourceType::GoldOre,
                        ResourceType::PlatinumOre,
                    ]),
                    requirements: Requirements {
                        power: Some(1),
                        crew: Some(2),
                        slots: None,
                    },
                }
            }

            #[test]
            fn surveyor_should_be_deserializable() {
                let json_str = r#"
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
                    }"#;

                let actual: Mount = serde_json::from_str(json_str).unwrap();
                let expected = some_surveyor();

                assert_eq!(expected, actual);
            }
        }

        mod cargo {
            use space_traders_api::ship::Cargo;

            pub fn some_cargo() -> Cargo {
                Cargo {
                    capacity: 60,
                    units: 0,
                    inventory: vec![],
                }
            }

            #[test]
            fn should_be_deserializable() {
                let json_str = r#"
                    {
                        "capacity": 60,
                        "units": 0,
                        "inventory": []
                    }"#;

                let actual: Cargo = serde_json::from_str(json_str).unwrap();
                let expected = some_cargo();

                assert_eq!(expected, actual);
            }
        }

        mod fuel {
            use space_traders_api::ship::{Fuel, FuelConsumed};

            pub fn some_fuel() -> Fuel {
                Fuel {
                    current: 1200,
                    capacity: 1200,
                    consumed: Some(FuelConsumed {
                        amount: 0,
                        timestamp: string!("2023-09-23T01:48:20.204Z"),
                    }),
                }
            }

            #[test]
            pub fn should_be_deserializable() {
                let json_str = r#"
                {
                    "current": 1200,
                    "capacity": 1200,
                    "consumed": {
                    "amount": 0,
                    "timestamp": "2023-09-23T01:48:20.204Z"
                    }
                }"#;

                let actual: Fuel = serde_json::from_str(json_str).unwrap();
                let expected = some_fuel();

                assert_eq!(expected, actual);
            }
        }

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
                    some_mineral_processor(),
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
}
