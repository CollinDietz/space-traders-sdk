use reqwest::Error;
use serde_derive::{Deserialize, Serialize};

use crate::agent::Agent;
use crate::api::Api;
use crate::contract::Contract;
use crate::faction::{Faction, Factions};
use crate::ship::Ship;

const REAL_SERVER: &'static str = "https://api.spacetraders.io/v2";

#[derive(Debug, PartialEq, Serialize)]
pub struct RegistrationRequest {
    #[serde(rename = "symbol")]
    pub callsign: String,
    pub faction: Factions,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LoginData {
    pub agent: Agent,
    pub contract: Contract,
    pub faction: Faction,
    pub ship: Ship,
    pub token: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LoginResponse {
    pub data: LoginData,
}

impl Api {
    async fn register_internal(
        url: String,
        request: RegistrationRequest,
    ) -> Result<(Self, LoginData), Error> {
        let api = Api {
            client: reqwest::Client::new(),
            url: url,
            token: "".to_string(),
        };

        let response: LoginResponse = api.post("register", &request).await?.json().await?;

        Ok((
            Api {
                token: response.data.token.clone(),
                ..api.clone()
            },
            response.data,
        ))
    }

    pub async fn register(request: RegistrationRequest) -> Result<(Self, LoginData), Error> {
        Api::register_internal(REAL_SERVER.to_string(), request).await
    }

    pub async fn login_internal(url: String, token: String) -> Result<(Self, LoginData), Error> {
        let _api = Api {
            client: reqwest::Client::new(),
            url: url,
            token: token,
        };

        let _login_data: LoginData = LoginData {
            agent: todo!(),
            contract: todo!(),
            faction: todo!(),
            ship: todo!(),
            token: token,
        };

        Ok((_api, _login_data))
    }

    #[cfg(test)]
    pub async fn register_test(
        url: String,
        request: RegistrationRequest,
    ) -> Result<(Self, LoginData), Error> {
        Api::register_internal(url, request).await
    }
}

#[cfg(test)]
pub mod tests {
    extern crate mock_server;
    use mock_server::*;

    use super::super::*;
    use super::*;
    use crate::agent::tests::some_agent;
    use crate::contract::tests::some_contract;
    use crate::faction::tests::some_faction;
    use crate::ordered_json;
    use crate::ship::tests::some_ship;
    use crate::string;

    fn some_login_data() -> LoginData {
        LoginData {
                agent: some_agent(),
                contract: some_contract(),
                faction: some_faction(),
                ship: some_ship(),
                token: string!("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiVEhJU0lTQVRFU1Q4ODgiLCJ2ZXJzaW9uIjoidjIiLCJyZXNldF9kYXRlIjoiMjAyMy0wOS0xNiIsImlhdCI6MTY5NTQzMzcwMCwic3ViIjoiYWdlbnQtdG9rZW4ifQ.Myu64GjY0OGWwyqn9q9nrKvJigGFHRvWKCboqror_iV3l-uJAUYAaMs7Dz2YhxhnGSlAEJ_B1uzbFRuE2YL1zVmpwQG2Jqou_YQk97vHwPhKyN7A7Ot_OcIbYTxLMR4GtVaSCSsGaRA0QQy-ive9YeZUWCkRf3tDSfSCEbhNMjbqeNxM9Mpy5WFiJIRvf9f9RvjApVBYGM4FikoXsCLeSskO8bntiYkEGYEhIH7EwGYQCKLXSzetrhVLF2YMzgDHM9nHB8OI2cth-3bqExmltLCmgJl_17b0ee0HhnY3BgIZ4D0qHRZFCoI1eGx4u8LhE14TpHXewkW4SCedlzVuMg"),
            }
    }

    fn some_login_response() -> LoginResponse {
        LoginResponse {
            data: some_login_data(),
        }
    }

    #[tokio::test]
    async fn request_should_be_sent_parsed_and_returned() {
        let mock_server = mock_response(RequestMethod::Post, "register", 201);

        let request: RegistrationRequest = RegistrationRequest {
            callsign: string!("SOMEPLAYER"),
            faction: Factions::Aegis,
        };

        let (_, actual) = Api::register_test(mock_server.url(), request)
            .await
            .unwrap();

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
        let json_str = r#"
        {
            "data": {
                "token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZGVudGlmaWVyIjoiVEhJU0lTQVRFU1Q4ODgiLCJ2ZXJzaW9uIjoidjIiLCJyZXNldF9kYXRlIjoiMjAyMy0wOS0xNiIsImlhdCI6MTY5NTQzMzcwMCwic3ViIjoiYWdlbnQtdG9rZW4ifQ.Myu64GjY0OGWwyqn9q9nrKvJigGFHRvWKCboqror_iV3l-uJAUYAaMs7Dz2YhxhnGSlAEJ_B1uzbFRuE2YL1zVmpwQG2Jqou_YQk97vHwPhKyN7A7Ot_OcIbYTxLMR4GtVaSCSsGaRA0QQy-ive9YeZUWCkRf3tDSfSCEbhNMjbqeNxM9Mpy5WFiJIRvf9f9RvjApVBYGM4FikoXsCLeSskO8bntiYkEGYEhIH7EwGYQCKLXSzetrhVLF2YMzgDHM9nHB8OI2cth-3bqExmltLCmgJl_17b0ee0HhnY3BgIZ4D0qHRZFCoI1eGx4u8LhE14TpHXewkW4SCedlzVuMg",
                "agent": {
                    "accountId": "clmvdg7i7dap7s60cmpyn9tnz",
                    "symbol": "THISISATEST888",
                    "headquarters": "X1-GM20-33220C",
                    "credits": 150000,
                    "startingFaction": "COSMIC"
                },
                "contract": {
                    "id": "clmvdg7k7dap9s60coav75shp",
                    "factionSymbol": "COSMIC",
                    "type": "PROCUREMENT",
                    "terms": {
                        "deadline": "2023-09-30T01:48:20.149Z",
                        "payment": {
                            "onAccepted": 23229,
                            "onFulfilled": 113742
                        },
                        "deliver": [
                            {
                            "tradeSymbol": "ALUMINUM_ORE",
                            "destinationSymbol": "X1-GM20-97222X",
                            "unitsRequired": 8900,
                            "unitsFulfilled": 0
                            }
                        ]
                    },
                    "accepted": false,
                    "fulfilled": false,
                    "expiration": "2023-09-24T01:48:20.149Z",
                    "deadlineToAccept": "2023-09-24T01:48:20.149Z"
                },
                "faction": {
                    "symbol": "COSMIC",
                    "name": "Cosmic Engineers",
                    "description": "The Cosmic Engineers are a group of highly advanced scientists and engineers who seek to terraform and colonize new worlds, pushing the boundaries of technology and exploration.",
                    "headquarters": "X1-GM20-33220C",
                    "traits": [
                        {
                            "symbol": "INNOVATIVE",
                            "name": "Innovative",
                            "description": "Willing to try new and untested ideas. Sometimes able to come up with creative and original solutions to problems, and may be able to think outside the box. Sometimes at the forefront of technological or social change, and may be willing to take risks in order to advance the boundaries of human knowledge and understanding."
                        },
                        {
                            "symbol": "BOLD",
                            "name": "Bold",
                            "description": "Unafraid to take risks and challenge the status quo. Sometimes willing to do things that others would not dare, and may be able to overcome obstacles and challenges that would be insurmountable for others. Sometimes able to inspire and motivate others to take bold action as well."
                        },
                        {
                            "symbol": "VISIONARY",
                            "name": "Visionary",
                            "description": "Possessing a clear and compelling vision for the future. Sometimes able to see beyond the present and anticipate the needs and challenges of tomorrow. Sometimes able to inspire and guide others towards a better and brighter future, and may be willing to take bold and decisive action to make their vision a reality."
                        },
                        {
                            "symbol": "CURIOUS",
                            "name": "Curious",
                            "description": "Possessing a strong desire to learn and explore. Sometimes interested in a wide range of topics and may be willing to take risks in order to satisfy their curiosity. Sometimes able to think outside the box and come up with creative solutions to challenges."
                        }
                    ],
                    "isRecruiting": true
                },
                "ship": {
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
                }
            }
        }"#;

        let actual: LoginResponse = serde_json::from_str(json_str).unwrap();

        let expected = some_login_response();

        assert_eq!(expected, actual);
    }
}
