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
    pub departure: Location,
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
}
