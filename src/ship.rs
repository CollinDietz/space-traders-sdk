use serde_derive::Deserialize;

use crate::faction::Factions;

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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    pub name: String,
    pub faction_symbol: Factions,
    pub role: ShipRole,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ShipRole {
    Fabricator,
    Harvester,
    Hauler,
    Interceptor,
    Excavator,
    Transport,
    Repair,
    Surveyor,
    Command,
    Carrier,
    Patrol,
    Satellite,
    Explorer,
    Refinery,
}

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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Crew {
    pub current: i32,
    pub required: i32,
    pub capacity: i32,
    pub rotation: RotationMode,
    pub morale: i32,
    pub wages: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RotationMode {
    Strict,
    Relaxed,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub symbol: FrameType,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub module_slots: i32,
    pub mounting_points: i32,
    pub fuel_capacity: i32,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum FrameType {
    #[serde(rename = "FRAME_PROBE")]
    Probe,
    #[serde(rename = "FRAME_DRONE")]
    Drone,
    #[serde(rename = "FRAME_INTERCEPTOR")]
    Interceptor,
    #[serde(rename = "FRAME_RACER")]
    Racer,
    #[serde(rename = "FRAME_FIGHTER")]
    Fighter,
    #[serde(rename = "FRAME_FRIGATE")]
    Frigate,
    #[serde(rename = "FRAME_SHUTTLE")]
    Shuttle,
    #[serde(rename = "FRAME_EXPLORER")]
    Explorer,
    #[serde(rename = "FRAME_MINER")]
    Miner,
    #[serde(rename = "FRAME_LIGHT_FREIGHTER")]
    LightFreighter,
    #[serde(rename = "FRAME_HEAVY_FREIGHTER")]
    HeavyFreighter,
    #[serde(rename = "FRAME_TRANSPORT")]
    Transport,
    #[serde(rename = "FRAME_DESTROYER")]
    Destroyer,
    #[serde(rename = "FRAME_CRUISER")]
    Cruiser,
    #[serde(rename = "FRAME_CARRIER")]
    Carrier,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reactor {
    pub symbol: ReactorType,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub power_output: i32,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum ReactorType {
    #[serde(rename = "REACTOR_SOLAR_I")]
    SolarI,
    #[serde(rename = "REACTOR_FUSION_I")]
    FusionI,
    #[serde(rename = "REACTOR_FISSION_I")]
    FissionI,
    #[serde(rename = "REACTOR_CHEMICAL_I")]
    ChemicalI,
    #[serde(rename = "REACTOR_ANTIMATTER_I")]
    AntimatterI,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Engine {
    pub symbol: EngineType,
    pub name: String,
    pub description: String,
    pub condition: Option<i32>,
    pub speed: i32,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum EngineType {
    #[serde(rename = "ENGINE_IMPULSE_DRIVE_I")]
    ImpulseDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_I")]
    IonDriveI,
    #[serde(rename = "ENGINE_ION_DRIVE_II")]
    IonDriveII,
    #[serde(rename = "ENGINE_HYPER_DRIVE_I")]
    HyperDriveI,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Requirements {
    pub power: Option<i32>,
    pub crew: Option<i32>,
    pub slots: Option<i32>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    pub ship_symbol: String,
    pub total_seconds: i32,
    pub remaining_seconds: i32,
    pub expiration: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    pub symbol: ModuleType,
    pub capacity: Option<i32>,
    pub range: Option<i32>,
    pub name: String,
    pub description: String,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum ModuleType {
    #[serde(rename = "MODULE_MINERAL_PROCESSOR_I")]
    MineralProcessorI,
    #[serde(rename = "MODULE_CARGO_HOLD_I")]
    CargoHoldI,
    #[serde(rename = "MODULE_CREW_QUARTERS_I")]
    CrewQuartersI,
    #[serde(rename = "MODULE_ENVOY_QUARTERS_I")]
    EnvoyQuartersI,
    #[serde(rename = "MODULE_PASSENGER_CABIN_I")]
    PassengerCabinI,
    #[serde(rename = "MODULE_MICRO_REFINERY_I")]
    MicroRefineryI,
    #[serde(rename = "MODULE_ORE_REFINERY_I")]
    OreRefineryI,
    #[serde(rename = "MODULE_FUEL_REFINERY_I")]
    FuelRefineryI,
    #[serde(rename = "MODULE_SCIENCE_LAB_I")]
    ScienceLabI,
    #[serde(rename = "MODULE_JUMP_DRIVE_I")]
    JumpDriveI,
    #[serde(rename = "MODULE_JUMP_DRIVE_II")]
    JumpDriveII,
    #[serde(rename = "MODULE_JUMP_DRIVE_III")]
    JumpDriveIII,
    #[serde(rename = "MODULE_WARP_DRIVE_I")]
    WarpDriveI,
    #[serde(rename = "MODULE_WARP_DRIVE_II")]
    WarpDriveII,
    #[serde(rename = "MODULE_WARP_DRIVE_III")]
    WarpDriveIII,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_I")]
    ShieldGeneratorI,
    #[serde(rename = "MODULE_SHIELD_GENERATOR_II")]
    ShieldGeneratorII,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    pub symbol: MountType,
    pub name: String,
    pub description: Option<String>,
    pub strength: Option<i32>,
    pub deposits: Option<Vec<ResourceType>>,
    pub requirements: Requirements,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum MountType {
    #[serde(rename = "MOUNT_GAS_SIPHON_I")]
    GasSiphonI,
    #[serde(rename = "MOUNT_GAS_SIPHON_II")]
    GasSiphonII,
    #[serde(rename = "MOUNT_GAS_SIPHON_III")]
    GasSiphonIII,
    #[serde(rename = "MOUNT_SURVEYOR_I")]
    SurveyorI,
    #[serde(rename = "MOUNT_SURVEYOR_II")]
    SurveyorII,
    #[serde(rename = "MOUNT_SURVEYOR_III")]
    SurveyorIII,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_I")]
    SensorArrayI,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_II")]
    SensorArrayII,
    #[serde(rename = "MOUNT_SENSOR_ARRAY_III")]
    SensorArrayIII,
    #[serde(rename = "MOUNT_MINING_LASER_I")]
    MiningLaserI,
    #[serde(rename = "MOUNT_MINING_LASER_II")]
    MiningLaserII,
    #[serde(rename = "MOUNT_MINING_LASER_III")]
    MiningLaserIII,
    #[serde(rename = "MOUNT_LASER_CANNON_I")]
    LaserCannonI,
    #[serde(rename = "MOUNT_MISSILE_LAUNCHER_I")]
    MissileLauncherI,
    #[serde(rename = "MOUNT_TURRET_I")]
    TurretI,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    QuartzSand,
    SiliconCrystals,
    PreciousStones,
    IceWater,
    AmmoniaIce,
    IronOre,
    CopperOre,
    SilverOre,
    AluminumOre,
    GoldOre,
    PlatinumOre,
    Diamond,
    UraniteOre,
    MeritiumOre,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cargo {
    pub capacity: i32,
    pub units: i32,
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub units: i32,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fuel {
    pub current: i32,
    pub capacity: i32,
    pub consumed: Option<FuelConsumed>,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuelConsumed {
    pub amount: i32,
    pub timestamp: String,
}
