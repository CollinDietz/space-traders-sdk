use serde_derive::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct MarketResponse {
    data: Market,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Market {
    pub symbol: String,
    pub exports: Vec<TradeGood>,
    pub imports: Vec<TradeGood>,
    pub exchange: Vec<TradeGood>,
    pub transactions: Option<Vec<MarketTransaction>>,
    pub trade_goods: Option<Vec<MarketTradeGood>>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionType {
    Purchase,
    Sell,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeType {
    Export,
    Import,
    Exchange,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SupplyLevel {
    Scarce,
    Limited,
    Moderate,
    High,
    Abundant,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivityLevel {
    Weak,
    Growing,
    Strong,
    Restricted,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct TradeGood {
    pub symbol: TradeSymbol,
    pub name: String,
    pub description: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct MarketTransaction {
    pub waypoint_symbol: String,
    pub ship_symbol: String,
    pub trade_symbol: TradeSymbol,
    pub transaction_type: TransactionType,
    pub units: i32,
    pub price_per_unit: i32,
    pub total_price: i32,
    pub timestamp: String,
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct MarketTradeGood {
    pub symbol: TradeSymbol,
    pub trade_type: TradeType,
    pub trade_volume: i32,
    pub supply: SupplyLevel,
    pub activity: ActivityLevel,
    pub purchase_price: i32,
    pub sell_price: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TradeSymbol {
    PreciousStones,
    QuartzSand,
    SiliconCrystals,
    AmmoniaIce,
    LiquidHydrogen,
    LiquidNitrogen,
    IceWater,
    ExoticMatter,
    AdvancedCircuitry,
    GravitonEmitters,
    Iron,
    IronOre,
    Copper,
    CopperOre,
    Aluminum,
    AluminumOre,
    Silver,
    SilverOre,
    Gold,
    GoldOre,
    Platinum,
    PlatinumOre,
    Diamonds,
    Uranite,
    UraniteOre,
    Meritium,
    MeritiumOre,
    Hydrocarbon,
    Antimatter,
    FabMats,
    Fertilizers,
    Fabrics,
    Food,
    Jewelry,
    Machinery,
    Firearms,
    AssaultRifles,
    MilitaryEquipment,
    Explosives,
    LabInstruments,
    Ammunition,
    Electronics,
    ShipPlating,
    ShipParts,
    Equipment,
    Fuel,
    Medicine,
    Drugs,
    Clothing,
    Microprocessors,
    Plastics,
    Polynucleotides,
    Biocomposites,
    QuantumStabilizers,
    Nanobots,
    AiMainframes,
    QuantumDrives,
    RoboticDrones,
    CyberImplants,
    GeneTherapeutics,
    NeuralChips,
    MoodRegulators,
    ViralAgents,
    MicroFusionGenerators,
    Supergrains,
    LaserRifles,
    Holographics,
    ShipSalvage,
    RelicTech,
    NovelLifeforms,
    BotanicalSpecimens,
    CulturalArtifacts,
    FrameProbe,
    FrameDrone,
    FrameInterceptor,
    FrameRacer,
    FrameFighter,
    FrameFrigate,
    FrameShuttle,
    FrameExplorer,
    FrameMiner,
    FrameLightFreighter,
    FrameHeavyFreighter,
    FrameTransport,
    FrameDestroyer,
    FrameCruiser,
    FrameCarrier,
    FrameBulkFreighter,
    ReactorSolarI,
    ReactorFusionI,
    ReactorFissionI,
    ReactorChemicalI,
    ReactorAntimatterI,
    EngineImpulseDriveI,
    EngineIonDriveI,
    EngineIonDriveII,
    EngineHyperDriveI,
    ModuleMineralProcessorI,
    ModuleGasProcessorI,
    ModuleCargoHoldI,
    ModuleCargoHoldII,
    ModuleCargoHoldIII,
    ModuleCrewQuartersI,
    ModuleEnvoyQuartersI,
    ModulePassengerCabinI,
    ModuleMicroRefineryI,
    ModuleScienceLabI,
    ModuleJumpDriveI,
    ModuleJumpDriveII,
    ModuleJumpDriveIII,
    ModuleWarpDriveI,
    ModuleWarpDriveII,
    ModuleWarpDriveIII,
    ModuleShieldGeneratorI,
    ModuleShieldGeneratorII,
    ModuleOreRefineryI,
    ModuleFuelRefineryI,
    MountGasSiphonI,
    MountGasSiphonII,
    MountGasSiphonIII,
    MountSurveyorI,
    MountSurveyorII,
    MountSurveyorIII,
    MountSensorArrayI,
    MountSensorArrayII,
    MountSensorArrayIII,
    MountMiningLaserI,
    MountMiningLaserII,
    MountMiningLaserIII,
    MountLaserCannonI,
    MountMissileLauncherI,
    MountTurretI,
    ShipProbe,
    ShipMiningDrone,
    ShipSiphonDrone,
    ShipInterceptor,
    ShipLightHauler,
    ShipCommandFrigate,
    ShipExplorer,
    ShipHeavyFreighter,
    ShipLightShuttle,
    ShipOreHound,
    ShipRefiningFreighter,
    ShipSurveyor,
    ShipBulkFreighter,
}

#[cfg(test)]
pub mod tests {
    use crate::{
        string,
        system::waypoint::market::{Market, MarketResponse, TradeGood, TradeSymbol},
    };

    pub fn some_market() -> Market {
        Market {
            symbol: string!("X1-MH3-A2"),
            exports: vec![],
            imports: vec![
                TradeGood {
                symbol: TradeSymbol::ShipPlating,
                name: string!("Ship Plating"),
                description: string!("High-quality metal plating used in the construction of ship hulls and other structural components.")
            },
                TradeGood {
                symbol: TradeSymbol::ShipParts,
                name: string!("Ship Parts"),
                description: string!("Various components and hardware required for spacecraft maintenance, upgrades, and construction.")
            },
            ],
            exchange: vec![TradeGood {
                symbol: TradeSymbol::Fuel,
                name: string!("Fuel"),
                description: string!("High-energy fuel used in spacecraft propulsion systems to enable long-distance space travel.")
            }],
            transactions: None,
            trade_goods: None,
        }
    }

    pub fn some_market_response() -> MarketResponse {
        MarketResponse {
            data: some_market(),
        }
    }

    #[test]
    pub fn ship_yard_data_should_be_deserializable() {
        let json_str = r#"
        {
          "data": {
            "symbol": "X1-MH3-A2",
            "exports": [],
            "imports": [
              {
                "symbol": "SHIP_PLATING",
                "name": "Ship Plating",
                "description": "High-quality metal plating used in the construction of ship hulls and other structural components."
              },
              {
                "symbol": "SHIP_PARTS",
                "name": "Ship Parts",
                "description": "Various components and hardware required for spacecraft maintenance, upgrades, and construction."
              }
            ],
            "exchange": [
              {
                "symbol": "FUEL",
                "name": "Fuel",
                "description": "High-energy fuel used in spacecraft propulsion systems to enable long-distance space travel."
              }
            ]
          }
        }"#;

        let actual: MarketResponse = serde_json::from_str(json_str).unwrap();
        let expected = some_market_response();

        assert_eq!(expected, actual);
    }
}
