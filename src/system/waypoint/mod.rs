use serde_derive::Deserialize;

use crate::faction::Factions;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: WaypointType,
    pub system_symbol: String,
    pub x: i32,
    pub y: i32,
    pub orbitals: Vec<WaypointOrbital>,
    pub orbits: Option<String>,
    pub faction: Option<WaypointFaction>,
    pub traits: Vec<WaypointTrait>,
    pub modifiers: Option<Vec<WaypointModifier>>,
    pub chart: Option<Chart>,
    pub is_under_construction: bool,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WaypointOrbital {
    pub symbol: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WaypointFaction {
    pub symbol: Factions,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WaypointTrait {
    pub symbol: WaypointTraitSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct WaypointModifier {
    pub symbol: WaypointModifierSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub waypoint_symbol: String,
    pub submitted_by: Option<String>,
    pub submitted_on: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointType {
    Planet,
    GasGiant,
    Moon,
    OrbitalStation,
    JumpGate,
    AsteroidField,
    Asteroid,
    EngineeredAsteroid,
    AsteroidBase,
    Nebula,
    DebrisField,
    GravityWell,
    ArtificialGravityWell,
    FuelStation,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointTraitSymbol {
    Uncharted,
    UnderConstruction,
    Marketplace,
    Shipyard,
    Outpost,
    ScatteredSettlements,
    SprawlingCities,
    MegaStructures,
    PirateBase,
    Overcrowded,
    HighTech,
    Corrupt,
    Bureaucratic,
    TradingHub,
    Industrial,
    BlackMarket,
    ResearchFacility,
    MilitaryBase,
    SurveillanceOutpost,
    ExplorationOutpost,
    MineralDeposits,
    CommonMetalDeposits,
    PreciousMetalDeposits,
    RareMetalDeposits,
    MethanePools,
    IceCrystals,
    ExplosiveGases,
    StrongMagnetosphere,
    VibrantAuroras,
    SaltFlats,
    Canyons,
    PerpetualDaylight,
    PerpetualOvercast,
    DrySeabeds,
    MagmaSeas,
    Supervolcanoes,
    AshClouds,
    VastRuins,
    MutatedFlora,
    Terraformed,
    ExtremeTemperatures,
    ExtremePressure,
    DiverseLife,
    ScarceLife,
    Fossils,
    WeakGravity,
    StrongGravity,
    CrushingGravity,
    ToxicAtmosphere,
    CorrosiveAtmosphere,
    BreathableAtmosphere,
    ThinAtmosphere,
    Jovian,
    Rocky,
    Volcanic,
    Frozen,
    Swamp,
    Barren,
    Temperate,
    Jungle,
    Ocean,
    Radioactive,
    MicroGravityAnomalies,
    DebrisCluster,
    DeepCraters,
    ShallowCraters,
    UnstableComposition,
    HollowedInterior,
    Stripped,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WaypointModifierSymbol {
    Stripped,
    Unstable,
    RadiationLeak,
    CriticalLimit,
    CivilUnrest,
}

#[cfg(test)]
pub mod tests {
    use std::vec;

    use crate::{
        faction::Factions,
        string,
        system::waypoint::{
            Chart, Waypoint, WaypointFaction, WaypointTrait, WaypointTraitSymbol, WaypointType,
        },
    };

    fn some_chart() -> Chart {
        Chart {
            waypoint_symbol: string!("X1-AB31-A2"),
            submitted_by: Some(string!("COSMIC")),
            submitted_on: Some(string!("2025-06-01T13:01:39.041Z")),
        }
    }

    fn some_waypoint() -> Waypoint {
        Waypoint {
            symbol: string!("X1-AB31-A2"),
            waypoint_type: WaypointType::Moon,
            system_symbol: string!("X1-AB31"),
            x: -15,
            y: 21,
            orbitals: vec![],
            orbits: Some(string!("X1-AB31-A1")),
            faction: Some(WaypointFaction {
                symbol: Factions::Cosmic,
            }),
            traits: vec![
            WaypointTrait {
                symbol: WaypointTraitSymbol::Volcanic,
                name: string!("Volcanic"),
                description: string!("A volatile world marked by intense volcanic activity, creating a hazardous environment with the potential for valuable resource extraction, such as rare metals and geothermal energy."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::SprawlingCities,
                name: string!("Sprawling Cities"),
                description: string!("Expansive urban centers that stretch across the landscape, boasting advanced infrastructure and diverse populations."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Terraformed,
                name: string!("Terraformed"),
                description: string!("A waypoint that has been artificially transformed to support life, showcasing the engineering prowess of its inhabitants and providing a hospitable environment for colonization."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Marketplace,
                name: string!("Marketplace"),
                description: string!("A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."),
            },
            WaypointTrait {
                symbol: WaypointTraitSymbol::Shipyard,
                name: string!("Shipyard"),
                description: string!("A bustling hub for the construction, repair, and sale of various spacecraft, from humble shuttles to mighty warships."),
            }],
            modifiers: Some(vec![]),
            chart: Some(some_chart()),
            is_under_construction: false,
        }
    }

    #[test]
    fn chart_should_be_deserializable() {
        let json_str = r#"
        {
            "waypointSymbol": "X1-AB31-A2",
            "submittedBy": "COSMIC",
            "submittedOn": "2025-06-01T13:01:39.041Z"
        }"#;

        let actual: Chart = serde_json::from_str(json_str).unwrap();

        let expected = some_chart();

        assert_eq!(expected, actual);
    }
    #[test]
    fn waypoint_should_be_deserializable() {
        let json_str = r#"
        {
            "symbol": "X1-AB31-A2",
            "type": "MOON",
            "systemSymbol": "X1-AB31",
            "x": -15,
            "y": 21,
            "orbitals": [],
            "traits": [
                {
                    "symbol": "VOLCANIC",
                    "name": "Volcanic",
                    "description": "A volatile world marked by intense volcanic activity, creating a hazardous environment with the potential for valuable resource extraction, such as rare metals and geothermal energy."
                },
                {
                    "symbol": "SPRAWLING_CITIES",
                    "name": "Sprawling Cities",
                    "description": "Expansive urban centers that stretch across the landscape, boasting advanced infrastructure and diverse populations."
                },
                {
                    "symbol": "TERRAFORMED",
                    "name": "Terraformed",
                    "description": "A waypoint that has been artificially transformed to support life, showcasing the engineering prowess of its inhabitants and providing a hospitable environment for colonization."
                },
                {
                    "symbol": "MARKETPLACE",
                    "name": "Marketplace",
                    "description": "A thriving center of commerce where traders from across the galaxy gather to buy, sell, and exchange goods."
                },
                {
                    "symbol": "SHIPYARD",
                    "name": "Shipyard",
                    "description": "A bustling hub for the construction, repair, and sale of various spacecraft, from humble shuttles to mighty warships."
                }
            ],
            "isUnderConstruction": false,
            "orbits": "X1-AB31-A1",
            "faction": { "symbol": "COSMIC" },
            "modifiers": [],
            "chart": {
                "waypointSymbol": "X1-AB31-A2",
                "submittedBy": "COSMIC",
                "submittedOn": "2025-06-01T13:01:39.041Z"
            }
        }"#;

        let actual: Waypoint = serde_json::from_str(json_str).unwrap();

        let expected = some_waypoint();

        assert_eq!(expected, actual);
    }
}
