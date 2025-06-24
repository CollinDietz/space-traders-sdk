use serde_derive::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    pub capacity: i32,
    pub units: i32,
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub symbol: TradeSymbol,
    pub name: String,
    pub description: String,
    pub units: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
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
    use crate::string;
    use pretty_assertions::assert_eq;

    use super::*;

    pub fn some_cargo() -> Cargo {
        Cargo {
            capacity: 40,
            units: 0,
            inventory: vec![],
        }
    }

    #[test]
    fn some_cargo_should_be_deserializable() {
        let json_str = r#"
          {
              "capacity": 40,
              "units": 0,
              "inventory": []
          }"#;

        let actual: Cargo = serde_json::from_str(json_str).unwrap();
        let expected = some_cargo();

        assert_eq!(expected, actual);
    }

    pub fn no_capacity_cargo() -> Cargo {
        Cargo {
            capacity: 0,
            units: 0,
            inventory: vec![],
        }
    }

    fn cargo_with_something_in_it() -> Cargo {
        Cargo {
            capacity: 40,
            units: 4,
            inventory: vec![InventoryItem {
                symbol: TradeSymbol::IceWater,
                name: string!("Fresh Water"),
                description: string!("High-quality fresh water, essential for life support and hydroponic agriculture."),
                units: 4,
            }],
        }
    }

    #[test]
    fn no_capacity_cargo_should_be_deserializable() {
        let json_str = r#"
          {
              "capacity": 0,
              "units": 0,
              "inventory": []
          }"#;

        let actual: Cargo = serde_json::from_str(json_str).unwrap();
        let expected = no_capacity_cargo();

        assert_eq!(expected, actual);
    }

    #[test]
    fn cargo_with_contents_should_be_deserializable() {
        let json_str = r#"
        {
          "capacity": 40,
          "units": 4,
          "inventory": [
            {
              "symbol": "ICE_WATER",
              "name": "Fresh Water",
              "description": "High-quality fresh water, essential for life support and hydroponic agriculture.",
              "units": 4
            }
          ]
        }"#;

        let actual: Cargo = serde_json::from_str(json_str).unwrap();
        let expected = cargo_with_something_in_it();

        assert_eq!(expected, actual);
    }
}
