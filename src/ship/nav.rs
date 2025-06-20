use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nav {
    pub system_symbol: String,
    pub waypoint_symbol: String,
    pub route: Route,
    pub status: ShipStatus,
    pub flight_mode: FlightMode,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub destination: Location,
    pub origin: Location,
    pub departure_time: String,
    pub arrival: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub symbol: String,
    #[serde(rename = "type")]
    pub location_type: LocationType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LocationType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Nebula,
    DebrisField,
    GravityWell,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ShipStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FlightMode {
    Drift,
    Stealth,
    Cruise,
    Burn,
}

#[cfg(test)]
pub mod tests {
    pub mod nav {
        use super::super::*;
        use crate::string;
        use route::some_other_route;
        use route::some_route;

        pub mod route {
            use super::super::super::*;
            use crate::string;
            use location::some_moon_location;
            use location::some_planet_location;

            pub mod location {
                use super::super::super::super::*;
                use crate::string;

                pub fn some_planet_location() -> Location {
                    Location {
                        symbol: string!("X1-RC42-A1"),
                        location_type: LocationType::Planet,
                        system_symbol: string!("X1-RC42"),
                        x: -22,
                        y: -3,
                    }
                }

                #[test]
                fn some_planet_location_should_be_deserializable() {
                    let json_str = r#"
                    {
                        "symbol": "X1-RC42-A1",
                        "type": "PLANET",
                        "systemSymbol": "X1-RC42",
                        "x": -22,
                        "y": -3
                    }"#;

                    let actual: Location = serde_json::from_str(json_str).unwrap();
                    let expected = some_planet_location();

                    assert_eq!(expected, actual);
                }

                pub fn some_moon_location() -> Location {
                    Location {
                        symbol: string!("X1-RC42-H53"),
                        location_type: LocationType::Moon,
                        system_symbol: string!("X1-RC42"),
                        x: -9,
                        y: -45,
                    }
                }

                #[test]
                fn some_moon_location_should_be_deserializable() {
                    let json_str = r#"
                    {
                        "symbol": "X1-RC42-H53",
                        "type": "MOON",
                        "systemSymbol": "X1-RC42",
                        "x": -9,
                        "y": -45
                    }"#;

                    let actual: Location = serde_json::from_str(json_str).unwrap();
                    let expected = some_moon_location();

                    assert_eq!(expected, actual);
                }
            }

            pub fn some_route() -> Route {
                Route {
                    destination: some_planet_location(),
                    origin: some_planet_location(),
                    departure_time: string!("2025-05-29T22:47:42.914Z"),
                    arrival: string!("2025-05-29T22:47:42.914Z"),
                }
            }

            #[test]
            fn some_route_should_be_deserializable() {
                let json_str = r#"
                {
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
                }"#;

                let actual: Route = serde_json::from_str(json_str).unwrap();
                let expected = some_route();

                assert_eq!(expected, actual);
            }

            pub fn some_other_route() -> Route {
                Route {
                    destination: some_moon_location(),
                    origin: some_moon_location(),
                    departure_time: string!("2025-05-29T22:47:42.923Z"),
                    arrival: string!("2025-05-29T22:47:42.923Z"),
                }
            }

            #[test]
            fn some_other_route_should_be_deserializable() {
                let json_str = r#"
                {
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
                }"#;

                let actual: Route = serde_json::from_str(json_str).unwrap();
                let expected = some_other_route();

                assert_eq!(expected, actual);
            }
        }

        pub fn some_nav() -> Nav {
            Nav {
                system_symbol: string!("X1-RC42"),
                waypoint_symbol: string!("X1-RC42-A1"),
                route: some_route(),
                status: ShipStatus::Docked,
                flight_mode: FlightMode::Cruise,
            }
        }

        #[test]
        fn some_nav_should_be_deserializable() {
            let json_str = r#"
            {
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
            }"#;

            let actual: Nav = serde_json::from_str(json_str).unwrap();
            let expected = some_nav();

            assert_eq!(expected, actual);
        }

        pub fn some_other_nav() -> Nav {
            Nav {
                system_symbol: string!("X1-RC42"),
                waypoint_symbol: string!("X1-RC42-H53"),
                route: some_other_route(),
                status: ShipStatus::Docked,
                flight_mode: FlightMode::Cruise,
            }
        }
        #[test]
        fn some_other_nav_should_be_deserializable() {
            let json_str = r#"
            {
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
            }"#;

            let actual: Nav = serde_json::from_str(json_str).unwrap();
            let expected = some_other_nav();

            assert_eq!(expected, actual);
        }
    }
}
