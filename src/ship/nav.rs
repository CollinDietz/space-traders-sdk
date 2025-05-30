use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nav {
    pub system_symbol: String,
    pub waypoint_symbol: String,
    pub route: Route,
    pub status: ShipStatus,
    pub flight_mode: FlightMode,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub destination: Location,
    pub origin: Location,
    pub departure_time: String,
    pub arrival: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub symbol: String,
    #[serde(rename = "type")]
    pub location_type: LocationType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ShipStatus {
    InTransit,
    InOrbit,
    Docked,
}

#[derive(Debug, PartialEq, Deserialize)]
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
        use self::route::some_route;
        use super::super::*;
        use crate::string;

        pub mod route {
            use self::location::some_location;
            use super::super::super::*;
            use crate::string;

            pub mod location {
                use super::super::super::super::*;
                use crate::string;

                pub fn some_location() -> Location {
                    Location {
                        symbol: string!("X1-RC42-A1"),
                        location_type: LocationType::Planet,
                        system_symbol: string!("X1-RC42"),
                        x: -22,
                        y: -3,
                    }
                }

                #[test]
                fn should_be_deserializable() {
                    let json_str = r#"
                    {
                        "symbol": "X1-RC42-A1",
                        "type": "PLANET",
                        "systemSymbol": "X1-RC42",
                        "x": -22,
                        "y": -3
                    }"#;

                    let actual: Location = serde_json::from_str(json_str).unwrap();
                    let expected = some_location();

                    assert_eq!(expected, actual);
                }
            }

            pub fn some_route() -> Route {
                Route {
                    destination: some_location(),
                    origin: some_location(),
                    departure_time: string!("2025-05-29T22:47:42.914Z"),
                    arrival: string!("2025-05-29T22:47:42.914Z"),
                }
            }

            #[test]
            fn should_be_deserializable() {
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
        fn should_be_deserializable() {
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
    }
}
