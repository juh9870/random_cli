#![allow(clippy::unnecessary_cast)]
#![allow(dead_code)]

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/versions.xml
pub use crate::helpers::*;

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ActivationType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ActivationType {
    #[default]
    None,
    Manual,
    Mixed,
}
impl DatabaseItem for ActivationType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ActivationType"
    }
}
impl serde::Serialize for ActivationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Ai/AiDifficultyLevel.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum AiDifficultyLevel {
    #[default]
    ///Early-game enemy
    Easy = 0i32,
    ///Mid-game enemy
    Medium = 1i32,
    ///Late-game enemy
    Hard = 2i32,
}
impl DatabaseItem for AiDifficultyLevel {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "AiDifficultyLevel"
    }
}
impl serde::Serialize for AiDifficultyLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Ai/AiWeaponCategory.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum AiWeaponCategory {
    #[default]
    All = 0i32,
    Repair = 1i32,
    Damage = 2i32,
    CaptureDrone = 3i32,
}
impl DatabaseItem for AiWeaponCategory {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "AiWeaponCategory"
    }
}
impl serde::Serialize for AiWeaponCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Ai/BehaviorNodeType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BehaviorNodeType {
    #[default]
    ///Always returs SUCCESS
    Success = 0i32,
    ///Always returs FAILURE
    Failure = 9i32,
    SubTree = 1i32,
    ///Executes nodes sequentially, stops when the first node returns SUCCESS or RUNNING
    Selector = 2i32,
    ///Executes nodes sequentially, stops when the first node returns FAILURE or RUNNING
    Sequence = 3i32,
    ///Executes nodes in parallel. Returns SUCCESS if at least one succeeds, RUNNING if any still running, and FAILURE if all fail
    Parallel = 4i32,
    ///Executes random node, selects another after 'Cooldown' sec
    RandomSelector = 5i32,
    ///Executes node, swaps SUCCESS and FAILURE
    Invertor = 6i32,
    ///Executes node until condition met, then waits a specific amount of time, returning 'Result', repeats
    Cooldown = 7i32,
    ///Executes node until condition met, returns 'Result' after that
    Execute = 8i32,
    ///Executes nodes sequentially, stops when the first node returns FAILURE
    ParallelSequence = 10i32,
    ///Memorizes the primary target, runs the child node, then restores the original target. Returns the result of the node's execution
    PreserveTarget = 11i32,
    ///If the first node returns SUCCESS, executes the second node, executes the thirid node otherwise
    IfThenElse = 12i32,
    HasEnoughEnergy = 50i32,
    IsLowOnHp = 51i32,
    ///SUCCESS if player did any action this frame
    IsControledByPlayer = 52i32,
    ///Requires threat list. Returns SUCCESS if time to collision less than value
    HasIncomingThreat = 53i32,
    ///Returns SUCCESS if has any targets found by LookForAdditionalTargets node
    HasAdditionalTargets = 54i32,
    ///Returns SUCCESS if [engine power] > Multiplier * [target engine power]
    IsFasterThanTarget = 55i32,
    ///Returns SUCCESS if the main is selected and alive
    HasMainTarget = 56i32,
    MainTargetIsAlly = 57i32,
    MainTargetIsEnemy = 58i32,
    MainTargetLowHp = 59i32,
    MainTargetWithinAttackRange = 60i32,
    ///Returns SUCCESS if mothership exists and is alive
    HasMothership = 61i32,
    ///Returns SUCCESS if the distance to the target does not exceed the MaxDistance
    TargetDistance = 62i32,
    ///Returns SUCCESS if the attack range of any weapon surpasses [enemy attack range] * Multiplier
    HasLongerAttackRange = 63i32,
    ///Looks for nearest enemy. Changes target every MaxCooldown sec if it's defined. SUCCESS if found, FAILURE otherwise
    FindEnemy = 100i32,
    ///Moves towards target until inside attack radius. Requires main target. Uses attack range of selected weapons. SUCCESS if inside attack range, FAILURE if any error, RUNNING otherwise
    MoveToAttackRange = 101i32,
    ///Attacks main target with selected weapons, returning SUCCESS upon firing, RUNNING while aiming, and FAILURE for any inability to initiate the attack
    AttackMainTarget = 102i32,
    ///Returns SUCCESS if any weapon found, FAILURE otherwise. If no weapon selected all weapons will be used
    SelectWeapon = 103i32,
    ///Spawns drones and clones. Returns RUNNING until there are no more drones left to spawn, then returns SUCCESS if any drones active, FAILURE otherwise
    SpawnDrones = 104i32,
    ///Tries to ram the target, can use Afterburner and Fortification. Returns FAILURE if there is no target or it's moving too fast, RUNNING otherwise
    Ram = 105i32,
    DetonateShip = 106i32,
    ///Makes ship disappear completely, without a trace
    Vanish = 107i32,
    ///Keeps ship at required distance. Requires main target. Uses attack range of selected weapons. SUCCESS if inside valid range, FAILURE if any error, RUNNING otherwise
    MaintainAttackRange = 108i32,
    Wait = 109i32,
    LookAtTarget = 110i32,
    ///Updates the list of enemies other than the main target. Returns SUCCESS if any found, FAILURE otherwise
    LookForAdditionalTargets = 111i32,
    ///Updates threat list. Returns SUCCESS if any found, FAILURE otherwise
    LookForThreats = 112i32,
    ///Aligns ship's movement speed and direction to the target's. Returns SUCCESS on reaching, FAILURE on errors, and RUNNING otherwise
    MatchVelocityWithTarget = 113i32,
    ///Activates device. Returns SUCCESS if activated, FAILURE otherwise
    ActivateDevice = 114i32,
    ///If energy level drops below FailIfLess, enters recharging state and returns FAILURE until energy level reaches RestoreUntil, otherwise returns SUCCESS
    RechargeEnergy = 115i32,
    ///If any directional weapon active, maintain focus on target and returns RUNNING. Returns SUCCESS otherwise
    SustainAim = 116i32,
    ///Charges all weapons that require charging, reserves energy needed to fully charge. Returns FAILURE if not possible, SUCCESS if fully charged, RUNNING otherwise
    ChargeWeapons = 117i32,
    ///Follows the target. Returns FAILURE if there is no target, RUNNING otherwise
    Chase = 118i32,
    ///Changes the ship's trajectory to avoid collision. Requires threat list. Returns SUCCESS if there are no threats, RUNNING otherwise
    AvoidThreats = 119i32,
    ///Reduces ship's speed. Returns SUCCESS if it completely stopped, RUNNING otherwise
    SlowDown = 120i32,
    ///Activates weapons with high recoil to get impulse. Returns FAILURE if doesn't have such weapons, RUNNING otherwise
    UseRecoil = 121i32,
    ///Turns ship towards threats and activates shield. Returns FAILURE if there are no shields or threats, RUNNING otherwise
    DefendWithFronalShield = 122i32,
    ///Tracks controllable bullets. Detonates them if near the target. Returns FAILURE if there are no bullets left. RUNNING otherwise
    TrackControllableAmmo = 123i32,
    ///Keeps ship within a specified distance from main target. Returns SUCCESS if inside valid range, FAILURE if any error, RUNNING otherwise
    KeepDistance = 124i32,
    ///Removes main target. Returns SUCCESS
    ForgetMainTarget = 125i32,
    ///Flies away from target until outside its attack radius. Returns SUCCESS when safe, RUNNING otherwise
    EscapeTargetAttackRadius = 126i32,
    ///Attacks additional targets with selected weapons, returning SUCCESS upon firing, RUNNING while aiming, and FAILURE for any inability to initiate the attack
    AttackAdditionalTargets = 127i32,
    ///Targets an ally starbase. Returns SUCCESS if it exists and alive, otherwise FAILURE
    TargetAllyStarbase = 128i32,
    ///Targets an enemy starbase. Returns SUCCESS if it exists and alive, otherwise FAILURE
    TargetEnemyStarbase = 129i32,
    ///Tries to bypass obstacle if any. Returns FAILURE if there are no obstacles, RUNNING otherwise
    BypassObstacles = 130i32,
    ///Attacks targets that tracked by auto-aiming turrets, returning SUCCESS upon firing, FAILURE otherwise
    AttackTurretTargets = 131i32,
    ///Returns SUCCESS when [forward acceleration]/[max acceleration] > MinValue
    EnginePropulsionForce = 150i32,
    MotherShipRetreated = 200i32,
    MotherShipDestroyed = 201i32,
    FlyAroundMothership = 202i32,
    ///Detaches drone from the mothership. Starts treating all ships as enemies. Returns SUCCESS once activated, FAILURE after that
    GoBerserk = 203i32,
    ///Sets the mothership as a main target. Returns FAILURE if it's absent, otherwise SUCCESS
    TargetMothership = 204i32,
    MothershipLowHp = 205i32,
    ///Returns SUCCESS if ship is too far from the mothership
    MothershipDistanceExceeded = 206i32,
    ///Makes primary target a new mothership. Returns FAILURE if target is absent or already a mothership, otherwise SUCCESS
    MakeTargetMothership = 207i32,
    ///Spawns text over the ship sprite that disappears shortly. Retruns SUCCESS if spawned, RUNNING if on cooldown
    ShowMessage = 300i32,
    ///Writes message to the log file (https://docs.unity3d.com/Manual/LogFiles.html). Retruns SUCCESS
    DebugLog = 301i32,
    ///Sets a boolean variable for this ship. Returns SUCCESS if the variable's value changes, and FAILURE if the value remains unchanged
    SetValue = 302i32,
    ///Verifies the state of the variable. Returns SUCCESS if the variable is set, and FAILURE if not set or undefined
    GetValue = 303i32,
    ///Sends a message to all allies, returning SUCCESS if any ally receives it
    SendMessage = 304i32,
    ///Monitors the radio, waiting for a specific message. Remembers message sender. If received returns SUCCESS, otherwise FAILURE
    MessageReceived = 305i32,
    ///Targets the sender of the last received message. Returns SUCCESS if it's still alive, otherwise FAILURE
    TargetMessageSender = 306i32,
    ///Saves the main target for this ship. It returns SUCCESS if the target is successfully updated, and FAILURE if the new target is identical to the previously saved target
    SaveTarget = 307i32,
    ///Loads saved target. Returns SUCCESS if it exists and alive, otherwise FAILURE
    LoadTarget = 308i32,
    ///Returns SUCCESS if specific target exists and still alive
    HasSavedTarget = 309i32,
    ///Removes saved target. Returns SUCCESS
    ForgetSavedTarget = 310i32,
}
impl DatabaseItem for BehaviorNodeType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeType"
    }
}
impl serde::Serialize for BehaviorNodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Ai/BehaviorRequirementType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BehaviorRequirementType {
    #[default]
    Empty = 0i32,
    Any = 1i32,
    All = 2i32,
    None = 3i32,
    ///Condition met if AI level equals to this value
    AiLevel = 5i32,
    ///Condition met if AI level is equals or higher than this value
    MinAiLevel = 6i32,
    SizeClass = 7i32,
    HasDevice = 10i32,
    HasDrones = 12i32,
    HasAnyWeapon = 11i32,
    CanRepairAllies = 13i32,
    HasHighRecoilWeapon = 14i32,
    HasChargeableWeapon = 15i32,
    HasRemotelyControlledWeapon = 16i32,
    HasLongRangeWeapon = 17i32,
    HasEngine = 18i32,
    IsDrone = 50i32,
    HasKineticResistance = 100i32,
    ///Condition met when EnginePower/sqrt(Mass) > Value
    HasHighManeuverability = 101i32,
    HasHighRammingDamage = 102i32,
}
impl DatabaseItem for BehaviorRequirementType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorRequirementType"
    }
}
impl serde::Serialize for BehaviorRequirementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Ai/NodeExecutionMode.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum NodeExecutionMode {
    #[default]
    UntilSucceeds = 0i32,
    UntilFails = 1i32,
    UntilFinishes = 2i32,
    Infinitely = 3i32,
    OneTime = 4i32,
}
impl DatabaseItem for NodeExecutionMode {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "NodeExecutionMode"
    }
}
impl serde::Serialize for NodeExecutionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/AmmunitionClassObsolete.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum AmmunitionClassObsolete {
    #[default]
    Common,
    Acid,
    Aura,
    Carrier,
    Bomb,
    DamageOverTime,
    DroneControl,
    Emp,
    EmpMissile,
    EnergyBeam,
    EnergySiphon,
    Explosion,
    Fireworks,
    FragBomb,
    Fragment,
    HomingImmobilizer,
    HomingTorpedo,
    Immobilizer,
    LaserBeam,
    Rocket,
    Singularity,
    TractorBeam,
    VampiricRay,
    SmallVampiricRay,
    RepairRay,
    PlasmaWeb,
    UnguidedRocket,
    IonBeam,
    AcidRocket,
    BlackHole,
    ClusterMissile,
    HomingCarrier,
}
impl DatabaseItem for AmmunitionClassObsolete {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "AmmunitionClassObsolete"
    }
}
impl serde::Serialize for AmmunitionClassObsolete {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Availability.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum Availability {
    #[default]
    None = 0i32,
    Common = 1i32,
    Rare = 2i32,
    Special = 3i32,
    Hidden = 4i32,
    LootOnly = 5i32,
}
impl DatabaseItem for Availability {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "Availability"
    }
}
impl serde::Serialize for Availability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/CellType.xml
#[repr(u32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum CellType {
    #[default]
    ///'0'
    Empty = 48u32,
    ///'4'
    Weapon = 52u32,
    ///'1'
    Outer = 49u32,
    ///'2'
    Inner = 50u32,
    ///'3'
    InnerOuter = 51u32,
    ///'5'
    Engine = 53u32,
}
impl DatabaseItem for CellType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "CellType"
    }
}
impl serde::Serialize for CellType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl std::fmt::Display for CellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = *self as u32;
        if code == 0 {
            write!(f, "")
        } else {
            write!(f, "{}", char::from_u32(code).unwrap())
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ComponentCategory.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ComponentCategory {
    #[default]
    Undefined,
    Weapon,
    Defense,
    Energy,
    Engine,
    Drones,
    Special,
}
impl DatabaseItem for ComponentCategory {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ComponentCategory"
    }
}
impl serde::Serialize for ComponentCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ComponentStatsType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ComponentStatsType {
    #[default]
    PerComponent = 0i32,
    PerOneCell = 1i32,
}
impl DatabaseItem for ComponentStatsType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ComponentStatsType"
    }
}
impl serde::Serialize for ComponentStatsType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/DeviceClass.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum DeviceClass {
    #[default]
    Accelerator,
    Decoy,
    EnergyShield,
    Ghost,
    GravityGenerator,
    PartialShield,
    PointDefense,
    RepairBot,
    Detonator,
    Stealth,
    Teleporter,
    Brake,
    SuperStealth,
    Fortification,
    ToxicWaste,
    WormTail,
    ClonningCenter,
    TimeMachine,
    Jammer,
    DroneCamouflage,
    MissileCamouflage,
}
impl DatabaseItem for DeviceClass {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "DeviceClass"
    }
}
impl serde::Serialize for DeviceClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/DifficultyClass.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum DifficultyClass {
    #[default]
    Default = 0i32,
    Class1 = 1i32,
    Class2 = 2i32,
}
impl DatabaseItem for DifficultyClass {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "DifficultyClass"
    }
}
impl serde::Serialize for DifficultyClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ItemType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ItemType {
    #[default]
    Undefined = 0i32,
    Component = 1i32,
    Device = 2i32,
    Weapon = 3i32,
    AmmunitionObsolete = 4i32,
    DroneBay = 5i32,
    Ship = 6i32,
    Satellite = 7i32,
    ShipBuild = 8i32,
    SatelliteBuild = 9i32,
    Technology = 10i32,
    ComponentStats = 11i32,
    ComponentMod = 12i32,
    Skill = 13i32,
    Faction = 14i32,
    Quest = 15i32,
    Loot = 16i32,
    Fleet = 18i32,
    Character = 19i32,
    QuestItem = 20i32,
    Ammunition = 25i32,
    VisualEffect = 26i32,
    BulletPrefab = 27i32,
    BehaviorTree = 28i32,
    GameObjectPrefab = 29i32,
    CombatRules = 30i32,
    ShipSettings = 100i32,
    GalaxySettings = 101i32,
    DatabaseSettings = 102i32,
    ExplorationSettings = 103i32,
    FrontierSettings = 104i32,
    ShipModSettings = 105i32,
    SpecialEventSettings = 106i32,
    SkillSettings = 107i32,
    DebugSettings = 108i32,
    CombatSettings = 109i32,
    UiSettings = 110i32,
    FactionsSettings = 111i32,
    MusicPlaylist = 112i32,
}
impl DatabaseItem for ItemType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ItemType"
    }
}
impl serde::Serialize for ItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ModificationQuality.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ModificationQuality {
    #[default]
    N3 = 0i32,
    N2 = 1i32,
    N1 = 2i32,
    P1 = 3i32,
    P2 = 4i32,
    P3 = 5i32,
}
impl DatabaseItem for ModificationQuality {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ModificationQuality"
    }
}
impl serde::Serialize for ModificationQuality {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ObjectPrefabType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ObjectPrefabType {
    #[default]
    Undefined = 0i32,
    WormTailSegment = 1i32,
    CircularSpriteObject = 2i32,
    CircularOutlineObject = 3i32,
}
impl DatabaseItem for ObjectPrefabType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ObjectPrefabType"
    }
}
impl serde::Serialize for ObjectPrefabType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/FactionFilterType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum FactionFilterType {
    #[default]
    AllButList = 0i32,
    ListOnly = 1i32,
    StarOwnersAndList = 2i32,
    AllAvailable = 3i32,
}
impl DatabaseItem for FactionFilterType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "FactionFilterType"
    }
}
impl serde::Serialize for FactionFilterType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/LootItemType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum LootItemType {
    #[default]
    None = 0i32,
    SomeMoney = 1i32,
    Fuel = 2i32,
    Money = 3i32,
    Stars = 4i32,
    StarMap = 5i32,
    RandomComponents = 10i32,
    RandomItems = 20i32,
    AllItems = 21i32,
    ItemsWithChance = 22i32,
    QuestItem = 25i32,
    Ship = 30i32,
    EmptyShip = 31i32,
    Component = 35i32,
    Blueprint = 40i32,
    ResearchPoints = 41i32,
    Satellite = 45i32,
}
impl DatabaseItem for LootItemType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootItemType"
    }
}
impl serde::Serialize for LootItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/NodeType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum NodeType {
    #[default]
    Undefined = 0i32,
    ComingSoon = 1i32,
    ShowDialog = 10i32,
    OpenShipyard = 11i32,
    OpenWorkshop = 12i32,
    Switch = 15i32,
    Random = 16i32,
    Condition = 17i32,
    AttackFleet = 20i32,
    AttackOccupants = 21i32,
    AttackStarbase = 22i32,
    DestroyOccupants = 25i32,
    SuppressOccupants = 26i32,
    Retreat = 30i32,
    ReceiveItem = 35i32,
    RemoveItem = 36i32,
    Trade = 37i32,
    CompleteQuest = 40i32,
    FailQuest = 41i32,
    CancelQuest = 42i32,
    StartQuest = 43i32,
    SetCharacterRelations = 50i32,
    SetFactionRelations = 51i32,
    SetFactionStarbasePower = 52i32,
    ChangeCharacterRelations = 55i32,
    ChangeFactionRelations = 56i32,
    ChangeFactionStarbasePower = 57i32,
    CaptureStarBase = 60i32,
    LiberateStarBase = 61i32,
    ChangeFaction = 62i32,
}
impl DatabaseItem for NodeType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "NodeType"
    }
}
impl serde::Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/PlayerShipSelectionMode.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum PlayerShipSelectionMode {
    #[default]
    Default = 0i32,
    ///Player can select only one ship. When it dies, battle ends
    OnlyOneShip = 1i32,
    ///Ships enters battlefield by order. Selection is not allowed
    ByOrder = 2i32,
    ///Player can select ship but can't change it until it dies
    NoRetreats = 3i32,
}
impl DatabaseItem for PlayerShipSelectionMode {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "PlayerShipSelectionMode"
    }
}
impl serde::Serialize for PlayerShipSelectionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/QuestOriginType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum QuestOriginType {
    #[default]
    CurrentStar,
    CurrentFactionBase,
    RandomFactionBase,
    HomeStar,
    RandomStar,
}
impl DatabaseItem for QuestOriginType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "QuestOriginType"
    }
}
impl serde::Serialize for QuestOriginType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/QuestType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum QuestType {
    #[default]
    Common = 0i32,
    Singleton = 1i32,
    Storyline = 2i32,
    Temporary = 3i32,
    Urgent = 4i32,
}
impl DatabaseItem for QuestType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "QuestType"
    }
}
impl serde::Serialize for QuestType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/RequiredViewMode.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum RequiredViewMode {
    #[default]
    Any = 0i32,
    StarSystem = 1i32,
    StarMap = 2i32,
    GalaxyMap = 3i32,
}
impl DatabaseItem for RequiredViewMode {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequiredViewMode"
    }
}
impl serde::Serialize for RequiredViewMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/RequirementType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum RequirementType {
    #[default]
    Empty = 0i32,
    Any = 1i32,
    All = 2i32,
    None = 3i32,
    PlayerPosition = 6i32,
    RandomStarSystem = 7i32,
    AggressiveOccupants = 8i32,
    QuestCompleted = 9i32,
    QuestActive = 10i32,
    CharacterRelations = 15i32,
    FactionRelations = 16i32,
    StarbaseCaptured = 17i32,
    FactionStarbasePower = 18i32,
    IsHostileFaction = 19i32,
    Faction = 20i32,
    HaveQuestItem = 25i32,
    HaveItem = 26i32,
    HaveItemById = 27i32,
    ComeToOrigin = 30i32,
    TimeSinceQuestStart = 40i32,
    TimeSinceLastCompletion = 41i32,
}
impl DatabaseItem for RequirementType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementType"
    }
}
impl serde::Serialize for RequirementType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/RewardCondition.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum RewardCondition {
    #[default]
    Default = 0i32,
    Always = 1i32,
    Never = 2i32,
}
impl DatabaseItem for RewardCondition {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RewardCondition"
    }
}
impl serde::Serialize for RewardCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/StartCondition.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum StartCondition {
    #[default]
    Manual = 0i32,
    Beacon = 1i32,
    LocalEncounter = 2i32,
    FactionMission = 3i32,
    GameStart = 4i32,
    NewStarExplored = 5i32,
    ArrivedAtStar = 6i32,
    Daily = 7i32,
}
impl DatabaseItem for StartCondition {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "StartCondition"
    }
}
impl serde::Serialize for StartCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Quests/TimeOutMode.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum TimeOutMode {
    #[default]
    CallNextEnemy = 0i32,
    DrainPlayerHp = 1i32,
    ///Calls next enemy if any and resets the timer, draws the battle otherwise
    CallNextEnemyOrDraw = 2i32,
}
impl DatabaseItem for TimeOutMode {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "TimeOutMode"
    }
}
impl serde::Serialize for TimeOutMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ShipRarity.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ShipRarity {
    #[default]
    Normal,
    Rare,
    Hidden,
    Unique,
}
impl DatabaseItem for ShipRarity {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ShipRarity"
    }
}
impl serde::Serialize for ShipRarity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ShipType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ShipType {
    #[default]
    Common = 0i32,
    Drone = 1i32,
    Starbase = 2i32,
    Special = 3i32,
    Flagship = 4i32,
}
impl DatabaseItem for ShipType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ShipType"
    }
}
impl serde::Serialize for ShipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/SizeClass.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum SizeClass {
    #[default]
    Undefined = -1i32,
    Frigate = 0i32,
    Destroyer = 1i32,
    Cruiser = 2i32,
    Battleship = 3i32,
    Titan = 4i32,
    Starbase = 5i32,
}
impl DatabaseItem for SizeClass {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "SizeClass"
    }
}
impl serde::Serialize for SizeClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/SkillType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum SkillType {
    #[default]
    Undefined = 0i32,
    ShipAttack = 1i32,
    ShipDefense = 2i32,
    StarbaseAttack = 3i32,
    StarbaseDefense = 4i32,
    QuickLearning = 5i32,
    BetterPrices = 6i32,
    BetterLoot = 7i32,
    CommandPoints = 8i32,
    SalvageDrones = 9i32,
    Engineer = 10i32,
}
impl DatabaseItem for SkillType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "SkillType"
    }
}
impl serde::Serialize for SkillType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/StatModificationType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum StatModificationType {
    #[default]
    None = 0i32,
    WeaponDamage = 1i32,
    WeaponRange = 2i32,
    WeaponFireRate = 3i32,
    WeaponBulletSpeed = 4i32,
    WeaponBulletMass = 5i32,
    WeaponAoe = 6i32,
    DroneAttack = 10i32,
    DroneDefense = 11i32,
    DroneSpeed = 12i32,
    DroneRange = 13i32,
    EnergyCapacity = 20i32,
    EnergyRechargeRate = 21i32,
    ShieldPoints = 22i32,
    ShieldRechargeRate = 23i32,
    ArmorPoints = 24i32,
    ArmorRepairRate = 25i32,
    Resistance = 30i32,
    DeviceCooldown = 40i32,
    DeviceRange = 41i32,
    DevicePower = 42i32,
    EnginePower = 50i32,
    EngineTurnRate = 51i32,
    Mass = 60i32,
    EnergyCost = 61i32,
    ExtraHitPoints = 62i32,
}
impl DatabaseItem for StatModificationType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "StatModificationType"
    }
}
impl serde::Serialize for StatModificationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/TechType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum TechType {
    #[default]
    Component,
    Ship,
    Satellite,
}
impl DatabaseItem for TechType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "TechType"
    }
}
impl serde::Serialize for TechType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/AiBulletBehavior.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum AiBulletBehavior {
    #[default]
    Projectile,
    Homing,
    Beam,
    AreaOfEffect,
}
impl DatabaseItem for AiBulletBehavior {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "AiBulletBehavior"
    }
}
impl serde::Serialize for AiBulletBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/BulletControllerType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BulletControllerType {
    #[default]
    Projectile,
    Homing,
    Beam,
    Parametric,
}
impl DatabaseItem for BulletControllerType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletControllerType"
    }
}
impl serde::Serialize for BulletControllerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/BulletEffectType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BulletEffectType {
    #[default]
    None,
    PlaySfx,
    SpawnBullet,
    Detonate,
    SpawnStaticSfx,
    GravityField,
}
impl DatabaseItem for BulletEffectType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletEffectType"
    }
}
impl serde::Serialize for BulletEffectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/BulletImpactType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BulletImpactType {
    #[default]
    HitFirstTarget,
    HitAllTargets,
    DamageOverTime,
}
impl DatabaseItem for BulletImpactType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletImpactType"
    }
}
impl serde::Serialize for BulletImpactType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/BulletShape.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BulletShape {
    #[default]
    Projectile,
    Rocket,
    LaserBeam,
    LightningBolt,
    EnergyBeam,
    Spark,
    Mine,
    Wave,
    BlackHole,
}
impl DatabaseItem for BulletShape {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletShape"
    }
}
impl serde::Serialize for BulletShape {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/BulletTriggerCondition.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BulletTriggerCondition {
    #[default]
    Undefined,
    Created,
    Destroyed,
    Hit,
    Disarmed,
    Expired,
    Detonated,
    OutOfAmmo,
    Cooldown,
}
impl DatabaseItem for BulletTriggerCondition {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletTriggerCondition"
    }
}
impl serde::Serialize for BulletTriggerCondition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/BulletTypeObsolete.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum BulletTypeObsolete {
    #[default]
    Projectile,
    Homing,
    Static,
    Continuous,
    Magnetic,
}
impl DatabaseItem for BulletTypeObsolete {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletTypeObsolete"
    }
}
impl serde::Serialize for BulletTypeObsolete {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/ColorMode.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ColorMode {
    #[default]
    TakeFromOwner,
    UseMyOwn,
    Blend,
    Multiply,
}
impl DatabaseItem for ColorMode {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ColorMode"
    }
}
impl serde::Serialize for ColorMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/DamageType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum DamageType {
    #[default]
    Impact,
    Energy,
    Heat,
    Direct,
}
impl DatabaseItem for DamageType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "DamageType"
    }
}
impl serde::Serialize for DamageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/ImpactEffectType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ImpactEffectType {
    #[default]
    Damage,
    Push,
    Pull,
    DrainEnergy,
    SiphonHitPoints,
    SlowDown,
    CaptureDrones,
    Repair,
    RestoreLifetime,
    Devour,
    Teleport,
    DrainShield,
    DriveDronesCrazy,
}
impl DatabaseItem for ImpactEffectType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ImpactEffectType"
    }
}
impl serde::Serialize for ImpactEffectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/VisualEffectType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum VisualEffectType {
    #[default]
    Flash,
    FlashAdditive,
    Shockwave,
    Smoke,
    SmokeAdditive,
    Shake,
    Spark,
    Lightning,
    LightningStrike,
}
impl DatabaseItem for VisualEffectType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "VisualEffectType"
    }
}
impl serde::Serialize for VisualEffectType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/WeaponClass.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum WeaponClass {
    #[default]
    Common,
    Manageable,
    Continuous,
    MashineGun,
    MultiShot,
    RequiredCharging,
}
impl DatabaseItem for WeaponClass {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "WeaponClass"
    }
}
impl serde::Serialize for WeaponClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (*self as i32).serialize(serializer)
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/Weapon/WeaponSlotType.xml
#[repr(u32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum WeaponSlotType {
    #[default]
    Default,
    ///'C'
    Cannon = 67u32,
    ///'T'
    Torpedo = 84u32,
    ///'M'
    Missile = 77u32,
    ///'L'
    Laser = 76u32,
    ///'S'
    Special = 83u32,
}
impl DatabaseItem for WeaponSlotType {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "WeaponSlotType"
    }
}
impl serde::Serialize for WeaponSlotType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl std::fmt::Display for WeaponSlotType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = *self as u32;
        if code == 0 {
            write!(f, "")
        } else {
            write!(f, "{}", char::from_u32(code).unwrap())
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Expressions/FloatToFloat.xml

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Expressions/IntToFloat.xml

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Expressions/IntToInt.xml

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Expressions/SizeClassToInt.xml

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Ai/BehaviorNodeRequirement.xml
#[derive(Debug, Clone)]
pub enum BehaviorNodeRequirement {
    Empty(BehaviorNodeRequirementEmpty),
    Any(BehaviorNodeRequirementAny),
    All(BehaviorNodeRequirementAll),
    None(BehaviorNodeRequirementNone),
    AiLevel(BehaviorNodeRequirementAiLevel),
    MinAiLevel(BehaviorNodeRequirementMinAiLevel),
    SizeClass(BehaviorNodeRequirementSizeClass),
    HasDevice(BehaviorNodeRequirementHasDevice),
    HasDrones(BehaviorNodeRequirementHasDrones),
    HasAnyWeapon(BehaviorNodeRequirementHasAnyWeapon),
    CanRepairAllies(BehaviorNodeRequirementCanRepairAllies),
    HasHighRecoilWeapon(BehaviorNodeRequirementHasHighRecoilWeapon),
    HasChargeableWeapon(BehaviorNodeRequirementHasChargeableWeapon),
    HasRemotelyControlledWeapon(BehaviorNodeRequirementHasRemotelyControlledWeapon),
    HasLongRangeWeapon(BehaviorNodeRequirementHasLongRangeWeapon),
    HasEngine(BehaviorNodeRequirementHasEngine),
    IsDrone(BehaviorNodeRequirementIsDrone),
    HasKineticResistance(BehaviorNodeRequirementHasKineticResistance),
    HasHighManeuverability(BehaviorNodeRequirementHasHighManeuverability),
    HasHighRammingDamage(BehaviorNodeRequirementHasHighRammingDamage),
}
impl Default for BehaviorNodeRequirement {
    fn default() -> Self {
        Self::Empty(Default::default())
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementEmpty {}
impl BehaviorNodeRequirementEmpty {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementEmpty {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementEmpty"
    }
}
impl Default for BehaviorNodeRequirementEmpty {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementEmpty> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementEmpty) -> Self {
        Self::Empty(item)
    }
}
impl BehaviorNodeRequirementEmpty {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn empty() -> BehaviorNodeRequirementEmpty {
        BehaviorNodeRequirementEmpty::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementAny {
    pub r#requirements: Vec<BehaviorNodeRequirement>,
}
impl BehaviorNodeRequirementAny {
    pub fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(
        mut self,
        r#requirements: impl Into<Vec<BehaviorNodeRequirement>>,
    ) -> Self {
        self.r#requirements = r#requirements.into();
        self
    }
    pub fn set_requirements(
        &mut self,
        r#requirements: impl Into<Vec<BehaviorNodeRequirement>>,
    ) -> &mut Self {
        self.r#requirements = r#requirements.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementAny {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementAny"
    }
}
impl Default for BehaviorNodeRequirementAny {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementAny> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementAny) -> Self {
        Self::Any(item)
    }
}
impl BehaviorNodeRequirementAny {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn any() -> BehaviorNodeRequirementAny {
        BehaviorNodeRequirementAny::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementAll {
    pub r#requirements: Vec<BehaviorNodeRequirement>,
}
impl BehaviorNodeRequirementAll {
    pub fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(
        mut self,
        r#requirements: impl Into<Vec<BehaviorNodeRequirement>>,
    ) -> Self {
        self.r#requirements = r#requirements.into();
        self
    }
    pub fn set_requirements(
        &mut self,
        r#requirements: impl Into<Vec<BehaviorNodeRequirement>>,
    ) -> &mut Self {
        self.r#requirements = r#requirements.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementAll {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementAll"
    }
}
impl Default for BehaviorNodeRequirementAll {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementAll> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementAll) -> Self {
        Self::All(item)
    }
}
impl BehaviorNodeRequirementAll {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn all() -> BehaviorNodeRequirementAll {
        BehaviorNodeRequirementAll::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementNone {
    pub r#requirements: Vec<BehaviorNodeRequirement>,
}
impl BehaviorNodeRequirementNone {
    pub fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(
        mut self,
        r#requirements: impl Into<Vec<BehaviorNodeRequirement>>,
    ) -> Self {
        self.r#requirements = r#requirements.into();
        self
    }
    pub fn set_requirements(
        &mut self,
        r#requirements: impl Into<Vec<BehaviorNodeRequirement>>,
    ) -> &mut Self {
        self.r#requirements = r#requirements.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementNone {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementNone"
    }
}
impl Default for BehaviorNodeRequirementNone {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementNone> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementNone) -> Self {
        Self::None(item)
    }
}
impl BehaviorNodeRequirementNone {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn none() -> BehaviorNodeRequirementNone {
        BehaviorNodeRequirementNone::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementAiLevel {
    ///AiLevel rises with the level of enemies. Always High for drones and autopilot
    pub r#difficulty_level: AiDifficultyLevel,
}
impl BehaviorNodeRequirementAiLevel {
    pub fn new() -> Self {
        Self {
            r#difficulty_level: Default::default(),
        }
    }
    pub fn with_difficulty_level(
        mut self,
        r#difficulty_level: impl Into<AiDifficultyLevel>,
    ) -> Self {
        self.r#difficulty_level = r#difficulty_level.into();
        self
    }
    pub fn set_difficulty_level(
        &mut self,
        r#difficulty_level: impl Into<AiDifficultyLevel>,
    ) -> &mut Self {
        self.r#difficulty_level = r#difficulty_level.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementAiLevel {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementAiLevel"
    }
}
impl Default for BehaviorNodeRequirementAiLevel {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementAiLevel> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementAiLevel) -> Self {
        Self::AiLevel(item)
    }
}
impl BehaviorNodeRequirementAiLevel {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn ai_level() -> BehaviorNodeRequirementAiLevel {
        BehaviorNodeRequirementAiLevel::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementMinAiLevel {
    ///AiLevel rises with the level of enemies. Always High for drones and autopilot
    pub r#difficulty_level: AiDifficultyLevel,
}
impl BehaviorNodeRequirementMinAiLevel {
    pub fn new() -> Self {
        Self {
            r#difficulty_level: Default::default(),
        }
    }
    pub fn with_difficulty_level(
        mut self,
        r#difficulty_level: impl Into<AiDifficultyLevel>,
    ) -> Self {
        self.r#difficulty_level = r#difficulty_level.into();
        self
    }
    pub fn set_difficulty_level(
        &mut self,
        r#difficulty_level: impl Into<AiDifficultyLevel>,
    ) -> &mut Self {
        self.r#difficulty_level = r#difficulty_level.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementMinAiLevel {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementMinAiLevel"
    }
}
impl Default for BehaviorNodeRequirementMinAiLevel {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementMinAiLevel> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementMinAiLevel) -> Self {
        Self::MinAiLevel(item)
    }
}
impl BehaviorNodeRequirementMinAiLevel {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn min_ai_level() -> BehaviorNodeRequirementMinAiLevel {
        BehaviorNodeRequirementMinAiLevel::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementSizeClass {
    pub r#size_class: SizeClass,
}
impl BehaviorNodeRequirementSizeClass {
    pub fn new() -> Self {
        Self {
            r#size_class: Default::default(),
        }
    }
    pub fn with_size_class(mut self, r#size_class: impl Into<SizeClass>) -> Self {
        self.r#size_class = r#size_class.into();
        self
    }
    pub fn set_size_class(&mut self, r#size_class: impl Into<SizeClass>) -> &mut Self {
        self.r#size_class = r#size_class.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementSizeClass {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementSizeClass"
    }
}
impl Default for BehaviorNodeRequirementSizeClass {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementSizeClass> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementSizeClass) -> Self {
        Self::SizeClass(item)
    }
}
impl BehaviorNodeRequirementSizeClass {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn size_class() -> BehaviorNodeRequirementSizeClass {
        BehaviorNodeRequirementSizeClass::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasDevice {
    pub r#device_class: DeviceClass,
}
impl BehaviorNodeRequirementHasDevice {
    pub fn new() -> Self {
        Self {
            r#device_class: Default::default(),
        }
    }
    pub fn with_device_class(mut self, r#device_class: impl Into<DeviceClass>) -> Self {
        self.r#device_class = r#device_class.into();
        self
    }
    pub fn set_device_class(&mut self, r#device_class: impl Into<DeviceClass>) -> &mut Self {
        self.r#device_class = r#device_class.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasDevice {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasDevice"
    }
}
impl Default for BehaviorNodeRequirementHasDevice {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasDevice> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasDevice) -> Self {
        Self::HasDevice(item)
    }
}
impl BehaviorNodeRequirementHasDevice {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_device() -> BehaviorNodeRequirementHasDevice {
        BehaviorNodeRequirementHasDevice::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasDrones {}
impl BehaviorNodeRequirementHasDrones {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasDrones {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasDrones"
    }
}
impl Default for BehaviorNodeRequirementHasDrones {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasDrones> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasDrones) -> Self {
        Self::HasDrones(item)
    }
}
impl BehaviorNodeRequirementHasDrones {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_drones() -> BehaviorNodeRequirementHasDrones {
        BehaviorNodeRequirementHasDrones::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasAnyWeapon {}
impl BehaviorNodeRequirementHasAnyWeapon {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasAnyWeapon {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasAnyWeapon"
    }
}
impl Default for BehaviorNodeRequirementHasAnyWeapon {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasAnyWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasAnyWeapon) -> Self {
        Self::HasAnyWeapon(item)
    }
}
impl BehaviorNodeRequirementHasAnyWeapon {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_any_weapon() -> BehaviorNodeRequirementHasAnyWeapon {
        BehaviorNodeRequirementHasAnyWeapon::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementCanRepairAllies {}
impl BehaviorNodeRequirementCanRepairAllies {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementCanRepairAllies {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementCanRepairAllies"
    }
}
impl Default for BehaviorNodeRequirementCanRepairAllies {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementCanRepairAllies> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementCanRepairAllies) -> Self {
        Self::CanRepairAllies(item)
    }
}
impl BehaviorNodeRequirementCanRepairAllies {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn can_repair_allies() -> BehaviorNodeRequirementCanRepairAllies {
        BehaviorNodeRequirementCanRepairAllies::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasHighRecoilWeapon {}
impl BehaviorNodeRequirementHasHighRecoilWeapon {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasHighRecoilWeapon {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasHighRecoilWeapon"
    }
}
impl Default for BehaviorNodeRequirementHasHighRecoilWeapon {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasHighRecoilWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasHighRecoilWeapon) -> Self {
        Self::HasHighRecoilWeapon(item)
    }
}
impl BehaviorNodeRequirementHasHighRecoilWeapon {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_high_recoil_weapon() -> BehaviorNodeRequirementHasHighRecoilWeapon {
        BehaviorNodeRequirementHasHighRecoilWeapon::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasChargeableWeapon {}
impl BehaviorNodeRequirementHasChargeableWeapon {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasChargeableWeapon {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasChargeableWeapon"
    }
}
impl Default for BehaviorNodeRequirementHasChargeableWeapon {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasChargeableWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasChargeableWeapon) -> Self {
        Self::HasChargeableWeapon(item)
    }
}
impl BehaviorNodeRequirementHasChargeableWeapon {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_chargeable_weapon() -> BehaviorNodeRequirementHasChargeableWeapon {
        BehaviorNodeRequirementHasChargeableWeapon::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasRemotelyControlledWeapon {}
impl BehaviorNodeRequirementHasRemotelyControlledWeapon {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasRemotelyControlledWeapon {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasRemotelyControlledWeapon"
    }
}
impl Default for BehaviorNodeRequirementHasRemotelyControlledWeapon {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasRemotelyControlledWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasRemotelyControlledWeapon) -> Self {
        Self::HasRemotelyControlledWeapon(item)
    }
}
impl BehaviorNodeRequirementHasRemotelyControlledWeapon {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_remotely_controlled_weapon() -> BehaviorNodeRequirementHasRemotelyControlledWeapon {
        BehaviorNodeRequirementHasRemotelyControlledWeapon::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasLongRangeWeapon {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasLongRangeWeapon {
    pub fn new() -> Self {
        Self {
            r#value: Default::default(),
        }
    }
    pub fn with_value(mut self, r#value: impl Into<f32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<f32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasLongRangeWeapon {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasLongRangeWeapon"
    }
}
impl Default for BehaviorNodeRequirementHasLongRangeWeapon {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasLongRangeWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasLongRangeWeapon) -> Self {
        Self::HasLongRangeWeapon(item)
    }
}
impl BehaviorNodeRequirementHasLongRangeWeapon {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_long_range_weapon() -> BehaviorNodeRequirementHasLongRangeWeapon {
        BehaviorNodeRequirementHasLongRangeWeapon::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasEngine {}
impl BehaviorNodeRequirementHasEngine {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasEngine {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasEngine"
    }
}
impl Default for BehaviorNodeRequirementHasEngine {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasEngine> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasEngine) -> Self {
        Self::HasEngine(item)
    }
}
impl BehaviorNodeRequirementHasEngine {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_engine() -> BehaviorNodeRequirementHasEngine {
        BehaviorNodeRequirementHasEngine::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementIsDrone {}
impl BehaviorNodeRequirementIsDrone {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementIsDrone {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementIsDrone"
    }
}
impl Default for BehaviorNodeRequirementIsDrone {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementIsDrone> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementIsDrone) -> Self {
        Self::IsDrone(item)
    }
}
impl BehaviorNodeRequirementIsDrone {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn is_drone() -> BehaviorNodeRequirementIsDrone {
        BehaviorNodeRequirementIsDrone::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasKineticResistance {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasKineticResistance {
    pub fn new() -> Self {
        Self { r#value: 1f32 }
    }
    pub fn with_value(mut self, r#value: impl Into<f32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<f32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasKineticResistance {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasKineticResistance"
    }
}
impl Default for BehaviorNodeRequirementHasKineticResistance {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasKineticResistance> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasKineticResistance) -> Self {
        Self::HasKineticResistance(item)
    }
}
impl BehaviorNodeRequirementHasKineticResistance {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_kinetic_resistance() -> BehaviorNodeRequirementHasKineticResistance {
        BehaviorNodeRequirementHasKineticResistance::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasHighManeuverability {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasHighManeuverability {
    pub fn new() -> Self {
        Self { r#value: 1f32 }
    }
    pub fn with_value(mut self, r#value: impl Into<f32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<f32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasHighManeuverability {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasHighManeuverability"
    }
}
impl Default for BehaviorNodeRequirementHasHighManeuverability {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasHighManeuverability> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasHighManeuverability) -> Self {
        Self::HasHighManeuverability(item)
    }
}
impl BehaviorNodeRequirementHasHighManeuverability {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_high_maneuverability() -> BehaviorNodeRequirementHasHighManeuverability {
        BehaviorNodeRequirementHasHighManeuverability::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorNodeRequirementHasHighRammingDamage {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasHighRammingDamage {
    pub fn new() -> Self {
        Self {
            r#value: Default::default(),
        }
    }
    pub fn with_value(mut self, r#value: impl Into<f32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<f32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasHighRammingDamage {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorNodeRequirementHasHighRammingDamage"
    }
}
impl Default for BehaviorNodeRequirementHasHighRammingDamage {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorNodeRequirementHasHighRammingDamage> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasHighRammingDamage) -> Self {
        Self::HasHighRammingDamage(item)
    }
}
impl BehaviorNodeRequirementHasHighRammingDamage {
    pub fn wrap(self) -> BehaviorNodeRequirement {
        self.into()
    }
}
impl BehaviorNodeRequirement {
    pub fn has_high_ramming_damage() -> BehaviorNodeRequirementHasHighRammingDamage {
        BehaviorNodeRequirementHasHighRammingDamage::new()
    }
}
impl serde::Serialize for BehaviorNodeRequirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "BehaviorNodeRequirement")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: BehaviorRequirementType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Empty(x) => AdjTagged {
                t: BehaviorRequirementType::Empty,
                c: x,
            }
            .serialize(serializer),
            Self::Any(x) => AdjTagged {
                t: BehaviorRequirementType::Any,
                c: x,
            }
            .serialize(serializer),
            Self::All(x) => AdjTagged {
                t: BehaviorRequirementType::All,
                c: x,
            }
            .serialize(serializer),
            Self::None(x) => AdjTagged {
                t: BehaviorRequirementType::None,
                c: x,
            }
            .serialize(serializer),
            Self::AiLevel(x) => AdjTagged {
                t: BehaviorRequirementType::AiLevel,
                c: x,
            }
            .serialize(serializer),
            Self::MinAiLevel(x) => AdjTagged {
                t: BehaviorRequirementType::MinAiLevel,
                c: x,
            }
            .serialize(serializer),
            Self::SizeClass(x) => AdjTagged {
                t: BehaviorRequirementType::SizeClass,
                c: x,
            }
            .serialize(serializer),
            Self::HasDevice(x) => AdjTagged {
                t: BehaviorRequirementType::HasDevice,
                c: x,
            }
            .serialize(serializer),
            Self::HasDrones(x) => AdjTagged {
                t: BehaviorRequirementType::HasDrones,
                c: x,
            }
            .serialize(serializer),
            Self::HasAnyWeapon(x) => AdjTagged {
                t: BehaviorRequirementType::HasAnyWeapon,
                c: x,
            }
            .serialize(serializer),
            Self::CanRepairAllies(x) => AdjTagged {
                t: BehaviorRequirementType::CanRepairAllies,
                c: x,
            }
            .serialize(serializer),
            Self::HasHighRecoilWeapon(x) => AdjTagged {
                t: BehaviorRequirementType::HasHighRecoilWeapon,
                c: x,
            }
            .serialize(serializer),
            Self::HasChargeableWeapon(x) => AdjTagged {
                t: BehaviorRequirementType::HasChargeableWeapon,
                c: x,
            }
            .serialize(serializer),
            Self::HasRemotelyControlledWeapon(x) => AdjTagged {
                t: BehaviorRequirementType::HasRemotelyControlledWeapon,
                c: x,
            }
            .serialize(serializer),
            Self::HasLongRangeWeapon(x) => AdjTagged {
                t: BehaviorRequirementType::HasLongRangeWeapon,
                c: x,
            }
            .serialize(serializer),
            Self::HasEngine(x) => AdjTagged {
                t: BehaviorRequirementType::HasEngine,
                c: x,
            }
            .serialize(serializer),
            Self::IsDrone(x) => AdjTagged {
                t: BehaviorRequirementType::IsDrone,
                c: x,
            }
            .serialize(serializer),
            Self::HasKineticResistance(x) => AdjTagged {
                t: BehaviorRequirementType::HasKineticResistance,
                c: x,
            }
            .serialize(serializer),
            Self::HasHighManeuverability(x) => AdjTagged {
                t: BehaviorRequirementType::HasHighManeuverability,
                c: x,
            }
            .serialize(serializer),
            Self::HasHighRammingDamage(x) => AdjTagged {
                t: BehaviorRequirementType::HasHighRammingDamage,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl DatabaseItem for BehaviorNodeRequirement {
    fn validate(&mut self) {
        match self {
            Self::Empty(x) => x.validate(),
            Self::Any(x) => x.validate(),
            Self::All(x) => x.validate(),
            Self::None(x) => x.validate(),
            Self::AiLevel(x) => x.validate(),
            Self::MinAiLevel(x) => x.validate(),
            Self::SizeClass(x) => x.validate(),
            Self::HasDevice(x) => x.validate(),
            Self::HasDrones(x) => x.validate(),
            Self::HasAnyWeapon(x) => x.validate(),
            Self::CanRepairAllies(x) => x.validate(),
            Self::HasHighRecoilWeapon(x) => x.validate(),
            Self::HasChargeableWeapon(x) => x.validate(),
            Self::HasRemotelyControlledWeapon(x) => x.validate(),
            Self::HasLongRangeWeapon(x) => x.validate(),
            Self::HasEngine(x) => x.validate(),
            Self::IsDrone(x) => x.validate(),
            Self::HasKineticResistance(x) => x.validate(),
            Self::HasHighManeuverability(x) => x.validate(),
            Self::HasHighRammingDamage(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "BehaviorNodeRequirement"
    }
}
impl BehaviorNodeRequirement {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Empty(_) => BehaviorNodeRequirementEmpty::type_name(),
            Self::Any(_) => BehaviorNodeRequirementAny::type_name(),
            Self::All(_) => BehaviorNodeRequirementAll::type_name(),
            Self::None(_) => BehaviorNodeRequirementNone::type_name(),
            Self::AiLevel(_) => BehaviorNodeRequirementAiLevel::type_name(),
            Self::MinAiLevel(_) => BehaviorNodeRequirementMinAiLevel::type_name(),
            Self::SizeClass(_) => BehaviorNodeRequirementSizeClass::type_name(),
            Self::HasDevice(_) => BehaviorNodeRequirementHasDevice::type_name(),
            Self::HasDrones(_) => BehaviorNodeRequirementHasDrones::type_name(),
            Self::HasAnyWeapon(_) => BehaviorNodeRequirementHasAnyWeapon::type_name(),
            Self::CanRepairAllies(_) => BehaviorNodeRequirementCanRepairAllies::type_name(),
            Self::HasHighRecoilWeapon(_) => BehaviorNodeRequirementHasHighRecoilWeapon::type_name(),
            Self::HasChargeableWeapon(_) => BehaviorNodeRequirementHasChargeableWeapon::type_name(),
            Self::HasRemotelyControlledWeapon(_) => {
                BehaviorNodeRequirementHasRemotelyControlledWeapon::type_name()
            }
            Self::HasLongRangeWeapon(_) => BehaviorNodeRequirementHasLongRangeWeapon::type_name(),
            Self::HasEngine(_) => BehaviorNodeRequirementHasEngine::type_name(),
            Self::IsDrone(_) => BehaviorNodeRequirementIsDrone::type_name(),
            Self::HasKineticResistance(_) => {
                BehaviorNodeRequirementHasKineticResistance::type_name()
            }
            Self::HasHighManeuverability(_) => {
                BehaviorNodeRequirementHasHighManeuverability::type_name()
            }
            Self::HasHighRammingDamage(_) => {
                BehaviorNodeRequirementHasHighRammingDamage::type_name()
            }
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Ai/BehaviorTreeNode.xml
#[derive(Debug, Clone)]
pub enum BehaviorTreeNode {
    Success(BehaviorTreeNodeSuccess),
    Failure(BehaviorTreeNodeFailure),
    SubTree(BehaviorTreeNodeSubTree),
    Selector(BehaviorTreeNodeSelector),
    Sequence(BehaviorTreeNodeSequence),
    Parallel(BehaviorTreeNodeParallel),
    RandomSelector(BehaviorTreeNodeRandomSelector),
    Invertor(BehaviorTreeNodeInvertor),
    Cooldown(BehaviorTreeNodeCooldown),
    Execute(BehaviorTreeNodeExecute),
    ParallelSequence(BehaviorTreeNodeParallelSequence),
    PreserveTarget(BehaviorTreeNodePreserveTarget),
    IfThenElse(BehaviorTreeNodeIfThenElse),
    HasEnoughEnergy(BehaviorTreeNodeHasEnoughEnergy),
    IsLowOnHp(BehaviorTreeNodeIsLowOnHp),
    IsControledByPlayer(BehaviorTreeNodeIsControledByPlayer),
    HasIncomingThreat(BehaviorTreeNodeHasIncomingThreat),
    HasAdditionalTargets(BehaviorTreeNodeHasAdditionalTargets),
    IsFasterThanTarget(BehaviorTreeNodeIsFasterThanTarget),
    HasMainTarget(BehaviorTreeNodeHasMainTarget),
    MainTargetIsAlly(BehaviorTreeNodeMainTargetIsAlly),
    MainTargetIsEnemy(BehaviorTreeNodeMainTargetIsEnemy),
    MainTargetLowHp(BehaviorTreeNodeMainTargetLowHp),
    MainTargetWithinAttackRange(BehaviorTreeNodeMainTargetWithinAttackRange),
    HasMothership(BehaviorTreeNodeHasMothership),
    TargetDistance(BehaviorTreeNodeTargetDistance),
    HasLongerAttackRange(BehaviorTreeNodeHasLongerAttackRange),
    FindEnemy(BehaviorTreeNodeFindEnemy),
    MoveToAttackRange(BehaviorTreeNodeMoveToAttackRange),
    AttackMainTarget(BehaviorTreeNodeAttackMainTarget),
    SelectWeapon(BehaviorTreeNodeSelectWeapon),
    SpawnDrones(BehaviorTreeNodeSpawnDrones),
    Ram(BehaviorTreeNodeRam),
    DetonateShip(BehaviorTreeNodeDetonateShip),
    Vanish(BehaviorTreeNodeVanish),
    MaintainAttackRange(BehaviorTreeNodeMaintainAttackRange),
    Wait(BehaviorTreeNodeWait),
    LookAtTarget(BehaviorTreeNodeLookAtTarget),
    LookForAdditionalTargets(BehaviorTreeNodeLookForAdditionalTargets),
    LookForThreats(BehaviorTreeNodeLookForThreats),
    MatchVelocityWithTarget(BehaviorTreeNodeMatchVelocityWithTarget),
    ActivateDevice(BehaviorTreeNodeActivateDevice),
    RechargeEnergy(BehaviorTreeNodeRechargeEnergy),
    SustainAim(BehaviorTreeNodeSustainAim),
    ChargeWeapons(BehaviorTreeNodeChargeWeapons),
    Chase(BehaviorTreeNodeChase),
    AvoidThreats(BehaviorTreeNodeAvoidThreats),
    SlowDown(BehaviorTreeNodeSlowDown),
    UseRecoil(BehaviorTreeNodeUseRecoil),
    DefendWithFronalShield(BehaviorTreeNodeDefendWithFronalShield),
    TrackControllableAmmo(BehaviorTreeNodeTrackControllableAmmo),
    KeepDistance(BehaviorTreeNodeKeepDistance),
    ForgetMainTarget(BehaviorTreeNodeForgetMainTarget),
    EscapeTargetAttackRadius(BehaviorTreeNodeEscapeTargetAttackRadius),
    AttackAdditionalTargets(BehaviorTreeNodeAttackAdditionalTargets),
    TargetAllyStarbase(BehaviorTreeNodeTargetAllyStarbase),
    TargetEnemyStarbase(BehaviorTreeNodeTargetEnemyStarbase),
    BypassObstacles(BehaviorTreeNodeBypassObstacles),
    AttackTurretTargets(BehaviorTreeNodeAttackTurretTargets),
    EnginePropulsionForce(BehaviorTreeNodeEnginePropulsionForce),
    MotherShipRetreated(BehaviorTreeNodeMotherShipRetreated),
    MotherShipDestroyed(BehaviorTreeNodeMotherShipDestroyed),
    FlyAroundMothership(BehaviorTreeNodeFlyAroundMothership),
    GoBerserk(BehaviorTreeNodeGoBerserk),
    TargetMothership(BehaviorTreeNodeTargetMothership),
    MothershipLowHp(BehaviorTreeNodeMothershipLowHp),
    MothershipDistanceExceeded(BehaviorTreeNodeMothershipDistanceExceeded),
    MakeTargetMothership(BehaviorTreeNodeMakeTargetMothership),
    ShowMessage(BehaviorTreeNodeShowMessage),
    DebugLog(BehaviorTreeNodeDebugLog),
    SetValue(BehaviorTreeNodeSetValue),
    GetValue(BehaviorTreeNodeGetValue),
    SendMessage(BehaviorTreeNodeSendMessage),
    MessageReceived(BehaviorTreeNodeMessageReceived),
    TargetMessageSender(BehaviorTreeNodeTargetMessageSender),
    SaveTarget(BehaviorTreeNodeSaveTarget),
    LoadTarget(BehaviorTreeNodeLoadTarget),
    HasSavedTarget(BehaviorTreeNodeHasSavedTarget),
    ForgetSavedTarget(BehaviorTreeNodeForgetSavedTarget),
}
impl Default for BehaviorTreeNode {
    fn default() -> Self {
        Self::Success(Default::default())
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSuccess {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeSuccess {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSuccess {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSuccess"
    }
}
impl Default for BehaviorTreeNodeSuccess {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSuccess> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSuccess) -> Self {
        Self::Success(item)
    }
}
impl BehaviorTreeNodeSuccess {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn success() -> BehaviorTreeNodeSuccess {
        BehaviorTreeNodeSuccess::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeFailure {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeFailure {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeFailure {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeFailure"
    }
}
impl Default for BehaviorTreeNodeFailure {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeFailure> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeFailure) -> Self {
        Self::Failure(item)
    }
}
impl BehaviorTreeNodeFailure {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn failure() -> BehaviorTreeNodeFailure {
        BehaviorTreeNodeFailure::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSubTree {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#item_id: Option<BehaviorTreeId>,
}
impl BehaviorTreeNodeSubTree {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#item_id: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<Option<BehaviorTreeId>>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<Option<BehaviorTreeId>>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSubTree {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSubTree"
    }
}
impl Default for BehaviorTreeNodeSubTree {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSubTree> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSubTree) -> Self {
        Self::SubTree(item)
    }
}
impl BehaviorTreeNodeSubTree {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn sub_tree() -> BehaviorTreeNodeSubTree {
        BehaviorTreeNodeSubTree::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSelector {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeSelector {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_nodes(mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn set_nodes(&mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> &mut Self {
        self.r#nodes = r#nodes.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSelector {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSelector"
    }
}
impl Default for BehaviorTreeNodeSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSelector> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSelector) -> Self {
        Self::Selector(item)
    }
}
impl BehaviorTreeNodeSelector {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn selector() -> BehaviorTreeNodeSelector {
        BehaviorTreeNodeSelector::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSequence {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeSequence {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_nodes(mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn set_nodes(&mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> &mut Self {
        self.r#nodes = r#nodes.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSequence {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSequence"
    }
}
impl Default for BehaviorTreeNodeSequence {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSequence> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSequence) -> Self {
        Self::Sequence(item)
    }
}
impl BehaviorTreeNodeSequence {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn sequence() -> BehaviorTreeNodeSequence {
        BehaviorTreeNodeSequence::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeParallel {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeParallel {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_nodes(mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn set_nodes(&mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> &mut Self {
        self.r#nodes = r#nodes.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeParallel {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeParallel"
    }
}
impl Default for BehaviorTreeNodeParallel {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeParallel> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeParallel) -> Self {
        Self::Parallel(item)
    }
}
impl BehaviorTreeNodeParallel {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn parallel() -> BehaviorTreeNodeParallel {
        BehaviorTreeNodeParallel::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeRandomSelector {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeRandomSelector {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_nodes(mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn set_nodes(&mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> &mut Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeRandomSelector {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeRandomSelector"
    }
}
impl Default for BehaviorTreeNodeRandomSelector {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeRandomSelector> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeRandomSelector) -> Self {
        Self::RandomSelector(item)
    }
}
impl BehaviorTreeNodeRandomSelector {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn random_selector() -> BehaviorTreeNodeRandomSelector {
        BehaviorTreeNodeRandomSelector::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeInvertor {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: Box<BehaviorTreeNode>,
}
impl BehaviorTreeNodeInvertor {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_node(mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> Self {
        self.r#node = r#node.into();
        self
    }
    pub fn set_node(&mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> &mut Self {
        self.r#node = r#node.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeInvertor {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeInvertor"
    }
}
impl Default for BehaviorTreeNodeInvertor {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeInvertor> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeInvertor) -> Self {
        Self::Invertor(item)
    }
}
impl BehaviorTreeNodeInvertor {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn invertor() -> BehaviorTreeNodeInvertor {
        BehaviorTreeNodeInvertor::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeCooldown {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: Box<BehaviorTreeNode>,
    pub r#execution_mode: NodeExecutionMode,
    pub r#result: bool,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeCooldown {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
            r#execution_mode: Default::default(),
            r#result: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_node(mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> Self {
        self.r#node = r#node.into();
        self
    }
    pub fn set_node(&mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> &mut Self {
        self.r#node = r#node.into();
        self
    }
    pub fn with_execution_mode(mut self, r#execution_mode: impl Into<NodeExecutionMode>) -> Self {
        self.r#execution_mode = r#execution_mode.into();
        self
    }
    pub fn set_execution_mode(
        &mut self,
        r#execution_mode: impl Into<NodeExecutionMode>,
    ) -> &mut Self {
        self.r#execution_mode = r#execution_mode.into();
        self
    }
    pub fn with_result(mut self, r#result: impl Into<bool>) -> Self {
        self.r#result = r#result.into();
        self
    }
    pub fn set_result(&mut self, r#result: impl Into<bool>) -> &mut Self {
        self.r#result = r#result.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeCooldown {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeCooldown"
    }
}
impl Default for BehaviorTreeNodeCooldown {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeCooldown> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeCooldown) -> Self {
        Self::Cooldown(item)
    }
}
impl BehaviorTreeNodeCooldown {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn cooldown() -> BehaviorTreeNodeCooldown {
        BehaviorTreeNodeCooldown::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeExecute {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: Box<BehaviorTreeNode>,
    pub r#execution_mode: NodeExecutionMode,
    pub r#result: bool,
}
impl BehaviorTreeNodeExecute {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
            r#execution_mode: Default::default(),
            r#result: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_node(mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> Self {
        self.r#node = r#node.into();
        self
    }
    pub fn set_node(&mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> &mut Self {
        self.r#node = r#node.into();
        self
    }
    pub fn with_execution_mode(mut self, r#execution_mode: impl Into<NodeExecutionMode>) -> Self {
        self.r#execution_mode = r#execution_mode.into();
        self
    }
    pub fn set_execution_mode(
        &mut self,
        r#execution_mode: impl Into<NodeExecutionMode>,
    ) -> &mut Self {
        self.r#execution_mode = r#execution_mode.into();
        self
    }
    pub fn with_result(mut self, r#result: impl Into<bool>) -> Self {
        self.r#result = r#result.into();
        self
    }
    pub fn set_result(&mut self, r#result: impl Into<bool>) -> &mut Self {
        self.r#result = r#result.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeExecute {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeExecute"
    }
}
impl Default for BehaviorTreeNodeExecute {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeExecute> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeExecute) -> Self {
        Self::Execute(item)
    }
}
impl BehaviorTreeNodeExecute {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn execute() -> BehaviorTreeNodeExecute {
        BehaviorTreeNodeExecute::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeParallelSequence {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeParallelSequence {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_nodes(mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn set_nodes(&mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> &mut Self {
        self.r#nodes = r#nodes.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeParallelSequence {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeParallelSequence"
    }
}
impl Default for BehaviorTreeNodeParallelSequence {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeParallelSequence> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeParallelSequence) -> Self {
        Self::ParallelSequence(item)
    }
}
impl BehaviorTreeNodeParallelSequence {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn parallel_sequence() -> BehaviorTreeNodeParallelSequence {
        BehaviorTreeNodeParallelSequence::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodePreserveTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: Box<BehaviorTreeNode>,
}
impl BehaviorTreeNodePreserveTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_node(mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> Self {
        self.r#node = r#node.into();
        self
    }
    pub fn set_node(&mut self, r#node: impl Into<Box<BehaviorTreeNode>>) -> &mut Self {
        self.r#node = r#node.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodePreserveTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodePreserveTarget"
    }
}
impl Default for BehaviorTreeNodePreserveTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodePreserveTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodePreserveTarget) -> Self {
        Self::PreserveTarget(item)
    }
}
impl BehaviorTreeNodePreserveTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn preserve_target() -> BehaviorTreeNodePreserveTarget {
        BehaviorTreeNodePreserveTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeIfThenElse {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeIfThenElse {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_nodes(mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn set_nodes(&mut self, r#nodes: impl Into<Vec<BehaviorTreeNode>>) -> &mut Self {
        self.r#nodes = r#nodes.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeIfThenElse {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeIfThenElse"
    }
}
impl Default for BehaviorTreeNodeIfThenElse {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeIfThenElse> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIfThenElse) -> Self {
        Self::IfThenElse(item)
    }
}
impl BehaviorTreeNodeIfThenElse {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn if_then_else() -> BehaviorTreeNodeIfThenElse {
        BehaviorTreeNodeIfThenElse::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeHasEnoughEnergy {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeHasEnoughEnergy {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 0.1f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasEnoughEnergy {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeHasEnoughEnergy"
    }
}
impl Default for BehaviorTreeNodeHasEnoughEnergy {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeHasEnoughEnergy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasEnoughEnergy) -> Self {
        Self::HasEnoughEnergy(item)
    }
}
impl BehaviorTreeNodeHasEnoughEnergy {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn has_enough_energy() -> BehaviorTreeNodeHasEnoughEnergy {
        BehaviorTreeNodeHasEnoughEnergy::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeIsLowOnHp {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeIsLowOnHp {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeIsLowOnHp {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeIsLowOnHp"
    }
}
impl Default for BehaviorTreeNodeIsLowOnHp {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeIsLowOnHp> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIsLowOnHp) -> Self {
        Self::IsLowOnHp(item)
    }
}
impl BehaviorTreeNodeIsLowOnHp {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn is_low_on_hp() -> BehaviorTreeNodeIsLowOnHp {
        BehaviorTreeNodeIsLowOnHp::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeIsControledByPlayer {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeIsControledByPlayer {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeIsControledByPlayer {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeIsControledByPlayer"
    }
}
impl Default for BehaviorTreeNodeIsControledByPlayer {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeIsControledByPlayer> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIsControledByPlayer) -> Self {
        Self::IsControledByPlayer(item)
    }
}
impl BehaviorTreeNodeIsControledByPlayer {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn is_controled_by_player() -> BehaviorTreeNodeIsControledByPlayer {
        BehaviorTreeNodeIsControledByPlayer::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeHasIncomingThreat {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeHasIncomingThreat {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: 5f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasIncomingThreat {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeHasIncomingThreat"
    }
}
impl Default for BehaviorTreeNodeHasIncomingThreat {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeHasIncomingThreat> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasIncomingThreat) -> Self {
        Self::HasIncomingThreat(item)
    }
}
impl BehaviorTreeNodeHasIncomingThreat {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn has_incoming_threat() -> BehaviorTreeNodeHasIncomingThreat {
        BehaviorTreeNodeHasIncomingThreat::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeHasAdditionalTargets {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeHasAdditionalTargets {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasAdditionalTargets {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeHasAdditionalTargets"
    }
}
impl Default for BehaviorTreeNodeHasAdditionalTargets {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeHasAdditionalTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasAdditionalTargets) -> Self {
        Self::HasAdditionalTargets(item)
    }
}
impl BehaviorTreeNodeHasAdditionalTargets {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn has_additional_targets() -> BehaviorTreeNodeHasAdditionalTargets {
        BehaviorTreeNodeHasAdditionalTargets::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeIsFasterThanTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeIsFasterThanTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeIsFasterThanTarget {
    fn validate(&mut self) {
        if self.r#min_value < (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
        if self.r#min_value > (10f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 10f32,
                "Field got truncated"
            );
            self.r#min_value = 10f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeIsFasterThanTarget"
    }
}
impl Default for BehaviorTreeNodeIsFasterThanTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeIsFasterThanTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIsFasterThanTarget) -> Self {
        Self::IsFasterThanTarget(item)
    }
}
impl BehaviorTreeNodeIsFasterThanTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn is_faster_than_target() -> BehaviorTreeNodeIsFasterThanTarget {
        BehaviorTreeNodeIsFasterThanTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeHasMainTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeHasMainTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasMainTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeHasMainTarget"
    }
}
impl Default for BehaviorTreeNodeHasMainTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeHasMainTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasMainTarget) -> Self {
        Self::HasMainTarget(item)
    }
}
impl BehaviorTreeNodeHasMainTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn has_main_target() -> BehaviorTreeNodeHasMainTarget {
        BehaviorTreeNodeHasMainTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMainTargetIsAlly {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMainTargetIsAlly {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetIsAlly {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMainTargetIsAlly"
    }
}
impl Default for BehaviorTreeNodeMainTargetIsAlly {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMainTargetIsAlly> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetIsAlly) -> Self {
        Self::MainTargetIsAlly(item)
    }
}
impl BehaviorTreeNodeMainTargetIsAlly {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn main_target_is_ally() -> BehaviorTreeNodeMainTargetIsAlly {
        BehaviorTreeNodeMainTargetIsAlly::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMainTargetIsEnemy {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMainTargetIsEnemy {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetIsEnemy {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMainTargetIsEnemy"
    }
}
impl Default for BehaviorTreeNodeMainTargetIsEnemy {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMainTargetIsEnemy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetIsEnemy) -> Self {
        Self::MainTargetIsEnemy(item)
    }
}
impl BehaviorTreeNodeMainTargetIsEnemy {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn main_target_is_enemy() -> BehaviorTreeNodeMainTargetIsEnemy {
        BehaviorTreeNodeMainTargetIsEnemy::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMainTargetLowHp {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeMainTargetLowHp {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetLowHp {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMainTargetLowHp"
    }
}
impl Default for BehaviorTreeNodeMainTargetLowHp {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMainTargetLowHp> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetLowHp) -> Self {
        Self::MainTargetLowHp(item)
    }
}
impl BehaviorTreeNodeMainTargetLowHp {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn main_target_low_hp() -> BehaviorTreeNodeMainTargetLowHp {
        BehaviorTreeNodeMainTargetLowHp::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMainTargetWithinAttackRange {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    ///Linear interpolation between shortest and longest weapon ranges
    pub r#min_value: f32,
}
impl BehaviorTreeNodeMainTargetWithinAttackRange {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 1f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetWithinAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMainTargetWithinAttackRange"
    }
}
impl Default for BehaviorTreeNodeMainTargetWithinAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMainTargetWithinAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetWithinAttackRange) -> Self {
        Self::MainTargetWithinAttackRange(item)
    }
}
impl BehaviorTreeNodeMainTargetWithinAttackRange {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn main_target_within_attack_range() -> BehaviorTreeNodeMainTargetWithinAttackRange {
        BehaviorTreeNodeMainTargetWithinAttackRange::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeHasMothership {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeHasMothership {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasMothership {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeHasMothership"
    }
}
impl Default for BehaviorTreeNodeHasMothership {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeHasMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasMothership) -> Self {
        Self::HasMothership(item)
    }
}
impl BehaviorTreeNodeHasMothership {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn has_mothership() -> BehaviorTreeNodeHasMothership {
        BehaviorTreeNodeHasMothership::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeTargetDistance {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    ///Max distance. If value is 0, prefefined value will be used (e.g. DroneBay range)
    pub r#max_value: f32,
}
impl BehaviorTreeNodeTargetDistance {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetDistance {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeTargetDistance"
    }
}
impl Default for BehaviorTreeNodeTargetDistance {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeTargetDistance> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetDistance) -> Self {
        Self::TargetDistance(item)
    }
}
impl BehaviorTreeNodeTargetDistance {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn target_distance() -> BehaviorTreeNodeTargetDistance {
        BehaviorTreeNodeTargetDistance::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeHasLongerAttackRange {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeHasLongerAttackRange {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasLongerAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
        if self.r#min_value > (10f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 10f32,
                "Field got truncated"
            );
            self.r#min_value = 10f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeHasLongerAttackRange"
    }
}
impl Default for BehaviorTreeNodeHasLongerAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeHasLongerAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasLongerAttackRange) -> Self {
        Self::HasLongerAttackRange(item)
    }
}
impl BehaviorTreeNodeHasLongerAttackRange {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn has_longer_attack_range() -> BehaviorTreeNodeHasLongerAttackRange {
        BehaviorTreeNodeHasLongerAttackRange::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeFindEnemy {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
    pub r#in_range: bool,
    pub r#no_drones: bool,
}
impl BehaviorTreeNodeFindEnemy {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
            r#max_value: 5f32,
            r#in_range: Default::default(),
            r#no_drones: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn with_in_range(mut self, r#in_range: impl Into<bool>) -> Self {
        self.r#in_range = r#in_range.into();
        self
    }
    pub fn set_in_range(&mut self, r#in_range: impl Into<bool>) -> &mut Self {
        self.r#in_range = r#in_range.into();
        self
    }
    pub fn with_no_drones(mut self, r#no_drones: impl Into<bool>) -> Self {
        self.r#no_drones = r#no_drones.into();
        self
    }
    pub fn set_no_drones(&mut self, r#no_drones: impl Into<bool>) -> &mut Self {
        self.r#no_drones = r#no_drones.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeFindEnemy {
    fn validate(&mut self) {
        if self.r#min_value < (0.5f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0.5f32,
                "Field got truncated"
            );
            self.r#min_value = 0.5f32 as f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeFindEnemy"
    }
}
impl Default for BehaviorTreeNodeFindEnemy {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeFindEnemy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeFindEnemy) -> Self {
        Self::FindEnemy(item)
    }
}
impl BehaviorTreeNodeFindEnemy {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn find_enemy() -> BehaviorTreeNodeFindEnemy {
        BehaviorTreeNodeFindEnemy::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMoveToAttackRange {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    ///Linear interpolation between shortest and longest weapon ranges
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMoveToAttackRange {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 1f32,
            r#max_value: 1f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMoveToAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#max_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMoveToAttackRange"
    }
}
impl Default for BehaviorTreeNodeMoveToAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMoveToAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMoveToAttackRange) -> Self {
        Self::MoveToAttackRange(item)
    }
}
impl BehaviorTreeNodeMoveToAttackRange {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn move_to_attack_range() -> BehaviorTreeNodeMoveToAttackRange {
        BehaviorTreeNodeMoveToAttackRange::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeAttackMainTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeAttackMainTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_in_range(mut self, r#in_range: impl Into<bool>) -> Self {
        self.r#in_range = r#in_range.into();
        self
    }
    pub fn set_in_range(&mut self, r#in_range: impl Into<bool>) -> &mut Self {
        self.r#in_range = r#in_range.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeAttackMainTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeAttackMainTarget"
    }
}
impl Default for BehaviorTreeNodeAttackMainTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeAttackMainTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAttackMainTarget) -> Self {
        Self::AttackMainTarget(item)
    }
}
impl BehaviorTreeNodeAttackMainTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn attack_main_target() -> BehaviorTreeNodeAttackMainTarget {
        BehaviorTreeNodeAttackMainTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSelectWeapon {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#weapon_type: AiWeaponCategory,
}
impl BehaviorTreeNodeSelectWeapon {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#weapon_type: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_weapon_type(mut self, r#weapon_type: impl Into<AiWeaponCategory>) -> Self {
        self.r#weapon_type = r#weapon_type.into();
        self
    }
    pub fn set_weapon_type(&mut self, r#weapon_type: impl Into<AiWeaponCategory>) -> &mut Self {
        self.r#weapon_type = r#weapon_type.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSelectWeapon {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSelectWeapon"
    }
}
impl Default for BehaviorTreeNodeSelectWeapon {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSelectWeapon> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSelectWeapon) -> Self {
        Self::SelectWeapon(item)
    }
}
impl BehaviorTreeNodeSelectWeapon {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn select_weapon() -> BehaviorTreeNodeSelectWeapon {
        BehaviorTreeNodeSelectWeapon::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSpawnDrones {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeSpawnDrones {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSpawnDrones {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSpawnDrones"
    }
}
impl Default for BehaviorTreeNodeSpawnDrones {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSpawnDrones> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSpawnDrones) -> Self {
        Self::SpawnDrones(item)
    }
}
impl BehaviorTreeNodeSpawnDrones {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn spawn_drones() -> BehaviorTreeNodeSpawnDrones {
        BehaviorTreeNodeSpawnDrones::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeRam {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#use_systems: bool,
}
impl BehaviorTreeNodeRam {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#use_systems: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_use_systems(mut self, r#use_systems: impl Into<bool>) -> Self {
        self.r#use_systems = r#use_systems.into();
        self
    }
    pub fn set_use_systems(&mut self, r#use_systems: impl Into<bool>) -> &mut Self {
        self.r#use_systems = r#use_systems.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeRam {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeRam"
    }
}
impl Default for BehaviorTreeNodeRam {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeRam> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeRam) -> Self {
        Self::Ram(item)
    }
}
impl BehaviorTreeNodeRam {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn ram() -> BehaviorTreeNodeRam {
        BehaviorTreeNodeRam::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeDetonateShip {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeDetonateShip {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_in_range(mut self, r#in_range: impl Into<bool>) -> Self {
        self.r#in_range = r#in_range.into();
        self
    }
    pub fn set_in_range(&mut self, r#in_range: impl Into<bool>) -> &mut Self {
        self.r#in_range = r#in_range.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeDetonateShip {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeDetonateShip"
    }
}
impl Default for BehaviorTreeNodeDetonateShip {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeDetonateShip> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeDetonateShip) -> Self {
        Self::DetonateShip(item)
    }
}
impl BehaviorTreeNodeDetonateShip {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn detonate_ship() -> BehaviorTreeNodeDetonateShip {
        BehaviorTreeNodeDetonateShip::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeVanish {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeVanish {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeVanish {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeVanish"
    }
}
impl Default for BehaviorTreeNodeVanish {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeVanish> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeVanish) -> Self {
        Self::Vanish(item)
    }
}
impl BehaviorTreeNodeVanish {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn vanish() -> BehaviorTreeNodeVanish {
        BehaviorTreeNodeVanish::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMaintainAttackRange {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    ///Linear interpolation between shortest and longest weapon ranges
    pub r#min_value: f32,
    ///A valid distance between ships will be [range*(1-tolerance) .. range]
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMaintainAttackRange {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 1f32,
            r#max_value: 0.2f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMaintainAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#max_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMaintainAttackRange"
    }
}
impl Default for BehaviorTreeNodeMaintainAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMaintainAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMaintainAttackRange) -> Self {
        Self::MaintainAttackRange(item)
    }
}
impl BehaviorTreeNodeMaintainAttackRange {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn maintain_attack_range() -> BehaviorTreeNodeMaintainAttackRange {
        BehaviorTreeNodeMaintainAttackRange::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeWait {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeWait {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn with_in_range(mut self, r#in_range: impl Into<bool>) -> Self {
        self.r#in_range = r#in_range.into();
        self
    }
    pub fn set_in_range(&mut self, r#in_range: impl Into<bool>) -> &mut Self {
        self.r#in_range = r#in_range.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeWait {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeWait"
    }
}
impl Default for BehaviorTreeNodeWait {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeWait> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeWait) -> Self {
        Self::Wait(item)
    }
}
impl BehaviorTreeNodeWait {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn wait() -> BehaviorTreeNodeWait {
        BehaviorTreeNodeWait::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeLookAtTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeLookAtTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeLookAtTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeLookAtTarget"
    }
}
impl Default for BehaviorTreeNodeLookAtTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeLookAtTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLookAtTarget) -> Self {
        Self::LookAtTarget(item)
    }
}
impl BehaviorTreeNodeLookAtTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn look_at_target() -> BehaviorTreeNodeLookAtTarget {
        BehaviorTreeNodeLookAtTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeLookForAdditionalTargets {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeLookForAdditionalTargets {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeLookForAdditionalTargets {
    fn validate(&mut self) {
        if self.r#cooldown < (0.1f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0.1f32,
                "Field got truncated"
            );
            self.r#cooldown = 0.1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeLookForAdditionalTargets"
    }
}
impl Default for BehaviorTreeNodeLookForAdditionalTargets {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeLookForAdditionalTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLookForAdditionalTargets) -> Self {
        Self::LookForAdditionalTargets(item)
    }
}
impl BehaviorTreeNodeLookForAdditionalTargets {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn look_for_additional_targets() -> BehaviorTreeNodeLookForAdditionalTargets {
        BehaviorTreeNodeLookForAdditionalTargets::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeLookForThreats {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeLookForThreats {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeLookForThreats {
    fn validate(&mut self) {
        if self.r#cooldown < (0.1f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0.1f32,
                "Field got truncated"
            );
            self.r#cooldown = 0.1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeLookForThreats"
    }
}
impl Default for BehaviorTreeNodeLookForThreats {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeLookForThreats> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLookForThreats) -> Self {
        Self::LookForThreats(item)
    }
}
impl BehaviorTreeNodeLookForThreats {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn look_for_threats() -> BehaviorTreeNodeLookForThreats {
        BehaviorTreeNodeLookForThreats::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMatchVelocityWithTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    ///Acceptable speed deviation
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMatchVelocityWithTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: 0.2f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMatchVelocityWithTarget {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#max_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMatchVelocityWithTarget"
    }
}
impl Default for BehaviorTreeNodeMatchVelocityWithTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMatchVelocityWithTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMatchVelocityWithTarget) -> Self {
        Self::MatchVelocityWithTarget(item)
    }
}
impl BehaviorTreeNodeMatchVelocityWithTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn match_velocity_with_target() -> BehaviorTreeNodeMatchVelocityWithTarget {
        BehaviorTreeNodeMatchVelocityWithTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeActivateDevice {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#device_class: DeviceClass,
}
impl BehaviorTreeNodeActivateDevice {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#device_class: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_device_class(mut self, r#device_class: impl Into<DeviceClass>) -> Self {
        self.r#device_class = r#device_class.into();
        self
    }
    pub fn set_device_class(&mut self, r#device_class: impl Into<DeviceClass>) -> &mut Self {
        self.r#device_class = r#device_class.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeActivateDevice {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeActivateDevice"
    }
}
impl Default for BehaviorTreeNodeActivateDevice {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeActivateDevice> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeActivateDevice) -> Self {
        Self::ActivateDevice(item)
    }
}
impl BehaviorTreeNodeActivateDevice {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn activate_device() -> BehaviorTreeNodeActivateDevice {
        BehaviorTreeNodeActivateDevice::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeRechargeEnergy {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeRechargeEnergy {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 0.1f32,
            r#max_value: 0.9f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeRechargeEnergy {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#max_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeRechargeEnergy"
    }
}
impl Default for BehaviorTreeNodeRechargeEnergy {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeRechargeEnergy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeRechargeEnergy) -> Self {
        Self::RechargeEnergy(item)
    }
}
impl BehaviorTreeNodeRechargeEnergy {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn recharge_energy() -> BehaviorTreeNodeRechargeEnergy {
        BehaviorTreeNodeRechargeEnergy::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSustainAim {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeSustainAim {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSustainAim {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSustainAim"
    }
}
impl Default for BehaviorTreeNodeSustainAim {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSustainAim> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSustainAim) -> Self {
        Self::SustainAim(item)
    }
}
impl BehaviorTreeNodeSustainAim {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn sustain_aim() -> BehaviorTreeNodeSustainAim {
        BehaviorTreeNodeSustainAim::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeChargeWeapons {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeChargeWeapons {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeChargeWeapons {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeChargeWeapons"
    }
}
impl Default for BehaviorTreeNodeChargeWeapons {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeChargeWeapons> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeChargeWeapons) -> Self {
        Self::ChargeWeapons(item)
    }
}
impl BehaviorTreeNodeChargeWeapons {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn charge_weapons() -> BehaviorTreeNodeChargeWeapons {
        BehaviorTreeNodeChargeWeapons::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeChase {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeChase {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeChase {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeChase"
    }
}
impl Default for BehaviorTreeNodeChase {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeChase> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeChase) -> Self {
        Self::Chase(item)
    }
}
impl BehaviorTreeNodeChase {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn chase() -> BehaviorTreeNodeChase {
        BehaviorTreeNodeChase::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeAvoidThreats {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeAvoidThreats {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeAvoidThreats {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeAvoidThreats"
    }
}
impl Default for BehaviorTreeNodeAvoidThreats {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeAvoidThreats> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAvoidThreats) -> Self {
        Self::AvoidThreats(item)
    }
}
impl BehaviorTreeNodeAvoidThreats {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn avoid_threats() -> BehaviorTreeNodeAvoidThreats {
        BehaviorTreeNodeAvoidThreats::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSlowDown {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    ///Acceptable speed deviation
    pub r#max_value: f32,
}
impl BehaviorTreeNodeSlowDown {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: 0.2f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSlowDown {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#max_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSlowDown"
    }
}
impl Default for BehaviorTreeNodeSlowDown {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSlowDown> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSlowDown) -> Self {
        Self::SlowDown(item)
    }
}
impl BehaviorTreeNodeSlowDown {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn slow_down() -> BehaviorTreeNodeSlowDown {
        BehaviorTreeNodeSlowDown::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeUseRecoil {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeUseRecoil {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeUseRecoil {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeUseRecoil"
    }
}
impl Default for BehaviorTreeNodeUseRecoil {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeUseRecoil> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeUseRecoil) -> Self {
        Self::UseRecoil(item)
    }
}
impl BehaviorTreeNodeUseRecoil {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn use_recoil() -> BehaviorTreeNodeUseRecoil {
        BehaviorTreeNodeUseRecoil::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeDefendWithFronalShield {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeDefendWithFronalShield {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeDefendWithFronalShield {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeDefendWithFronalShield"
    }
}
impl Default for BehaviorTreeNodeDefendWithFronalShield {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeDefendWithFronalShield> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeDefendWithFronalShield) -> Self {
        Self::DefendWithFronalShield(item)
    }
}
impl BehaviorTreeNodeDefendWithFronalShield {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn defend_with_fronal_shield() -> BehaviorTreeNodeDefendWithFronalShield {
        BehaviorTreeNodeDefendWithFronalShield::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeTrackControllableAmmo {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTrackControllableAmmo {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeTrackControllableAmmo {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeTrackControllableAmmo"
    }
}
impl Default for BehaviorTreeNodeTrackControllableAmmo {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeTrackControllableAmmo> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTrackControllableAmmo) -> Self {
        Self::TrackControllableAmmo(item)
    }
}
impl BehaviorTreeNodeTrackControllableAmmo {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn track_controllable_ammo() -> BehaviorTreeNodeTrackControllableAmmo {
        BehaviorTreeNodeTrackControllableAmmo::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeKeepDistance {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeKeepDistance {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 2.5f32,
            r#max_value: 3.5f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeKeepDistance {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (100f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#min_value = 100f32 as f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
        if self.r#max_value > (100f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#max_value = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeKeepDistance"
    }
}
impl Default for BehaviorTreeNodeKeepDistance {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeKeepDistance> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeKeepDistance) -> Self {
        Self::KeepDistance(item)
    }
}
impl BehaviorTreeNodeKeepDistance {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn keep_distance() -> BehaviorTreeNodeKeepDistance {
        BehaviorTreeNodeKeepDistance::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeForgetMainTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeForgetMainTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeForgetMainTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeForgetMainTarget"
    }
}
impl Default for BehaviorTreeNodeForgetMainTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeForgetMainTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeForgetMainTarget) -> Self {
        Self::ForgetMainTarget(item)
    }
}
impl BehaviorTreeNodeForgetMainTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn forget_main_target() -> BehaviorTreeNodeForgetMainTarget {
        BehaviorTreeNodeForgetMainTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeEscapeTargetAttackRadius {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeEscapeTargetAttackRadius {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeEscapeTargetAttackRadius {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeEscapeTargetAttackRadius"
    }
}
impl Default for BehaviorTreeNodeEscapeTargetAttackRadius {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeEscapeTargetAttackRadius> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeEscapeTargetAttackRadius) -> Self {
        Self::EscapeTargetAttackRadius(item)
    }
}
impl BehaviorTreeNodeEscapeTargetAttackRadius {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn escape_target_attack_radius() -> BehaviorTreeNodeEscapeTargetAttackRadius {
        BehaviorTreeNodeEscapeTargetAttackRadius::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeAttackAdditionalTargets {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeAttackAdditionalTargets {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_in_range(mut self, r#in_range: impl Into<bool>) -> Self {
        self.r#in_range = r#in_range.into();
        self
    }
    pub fn set_in_range(&mut self, r#in_range: impl Into<bool>) -> &mut Self {
        self.r#in_range = r#in_range.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeAttackAdditionalTargets {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeAttackAdditionalTargets"
    }
}
impl Default for BehaviorTreeNodeAttackAdditionalTargets {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeAttackAdditionalTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAttackAdditionalTargets) -> Self {
        Self::AttackAdditionalTargets(item)
    }
}
impl BehaviorTreeNodeAttackAdditionalTargets {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn attack_additional_targets() -> BehaviorTreeNodeAttackAdditionalTargets {
        BehaviorTreeNodeAttackAdditionalTargets::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeTargetAllyStarbase {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetAllyStarbase {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetAllyStarbase {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeTargetAllyStarbase"
    }
}
impl Default for BehaviorTreeNodeTargetAllyStarbase {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeTargetAllyStarbase> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetAllyStarbase) -> Self {
        Self::TargetAllyStarbase(item)
    }
}
impl BehaviorTreeNodeTargetAllyStarbase {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn target_ally_starbase() -> BehaviorTreeNodeTargetAllyStarbase {
        BehaviorTreeNodeTargetAllyStarbase::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeTargetEnemyStarbase {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetEnemyStarbase {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetEnemyStarbase {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeTargetEnemyStarbase"
    }
}
impl Default for BehaviorTreeNodeTargetEnemyStarbase {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeTargetEnemyStarbase> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetEnemyStarbase) -> Self {
        Self::TargetEnemyStarbase(item)
    }
}
impl BehaviorTreeNodeTargetEnemyStarbase {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn target_enemy_starbase() -> BehaviorTreeNodeTargetEnemyStarbase {
        BehaviorTreeNodeTargetEnemyStarbase::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeBypassObstacles {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeBypassObstacles {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeBypassObstacles {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeBypassObstacles"
    }
}
impl Default for BehaviorTreeNodeBypassObstacles {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeBypassObstacles> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeBypassObstacles) -> Self {
        Self::BypassObstacles(item)
    }
}
impl BehaviorTreeNodeBypassObstacles {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn bypass_obstacles() -> BehaviorTreeNodeBypassObstacles {
        BehaviorTreeNodeBypassObstacles::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeAttackTurretTargets {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeAttackTurretTargets {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeAttackTurretTargets {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeAttackTurretTargets"
    }
}
impl Default for BehaviorTreeNodeAttackTurretTargets {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeAttackTurretTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAttackTurretTargets) -> Self {
        Self::AttackTurretTargets(item)
    }
}
impl BehaviorTreeNodeAttackTurretTargets {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn attack_turret_targets() -> BehaviorTreeNodeAttackTurretTargets {
        BehaviorTreeNodeAttackTurretTargets::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeEnginePropulsionForce {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeEnginePropulsionForce {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeEnginePropulsionForce {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeEnginePropulsionForce"
    }
}
impl Default for BehaviorTreeNodeEnginePropulsionForce {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeEnginePropulsionForce> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeEnginePropulsionForce) -> Self {
        Self::EnginePropulsionForce(item)
    }
}
impl BehaviorTreeNodeEnginePropulsionForce {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn engine_propulsion_force() -> BehaviorTreeNodeEnginePropulsionForce {
        BehaviorTreeNodeEnginePropulsionForce::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMotherShipRetreated {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMotherShipRetreated {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMotherShipRetreated {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMotherShipRetreated"
    }
}
impl Default for BehaviorTreeNodeMotherShipRetreated {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMotherShipRetreated> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMotherShipRetreated) -> Self {
        Self::MotherShipRetreated(item)
    }
}
impl BehaviorTreeNodeMotherShipRetreated {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn mother_ship_retreated() -> BehaviorTreeNodeMotherShipRetreated {
        BehaviorTreeNodeMotherShipRetreated::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMotherShipDestroyed {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMotherShipDestroyed {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMotherShipDestroyed {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMotherShipDestroyed"
    }
}
impl Default for BehaviorTreeNodeMotherShipDestroyed {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMotherShipDestroyed> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMotherShipDestroyed) -> Self {
        Self::MotherShipDestroyed(item)
    }
}
impl BehaviorTreeNodeMotherShipDestroyed {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn mother_ship_destroyed() -> BehaviorTreeNodeMotherShipDestroyed {
        BehaviorTreeNodeMotherShipDestroyed::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeFlyAroundMothership {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeFlyAroundMothership {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 2.5f32,
            r#max_value: 3.5f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeFlyAroundMothership {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (100f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#min_value = 100f32 as f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
        if self.r#max_value > (100f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#max_value = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeFlyAroundMothership"
    }
}
impl Default for BehaviorTreeNodeFlyAroundMothership {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeFlyAroundMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeFlyAroundMothership) -> Self {
        Self::FlyAroundMothership(item)
    }
}
impl BehaviorTreeNodeFlyAroundMothership {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn fly_around_mothership() -> BehaviorTreeNodeFlyAroundMothership {
        BehaviorTreeNodeFlyAroundMothership::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeGoBerserk {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeGoBerserk {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeGoBerserk {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeGoBerserk"
    }
}
impl Default for BehaviorTreeNodeGoBerserk {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeGoBerserk> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeGoBerserk) -> Self {
        Self::GoBerserk(item)
    }
}
impl BehaviorTreeNodeGoBerserk {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn go_berserk() -> BehaviorTreeNodeGoBerserk {
        BehaviorTreeNodeGoBerserk::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeTargetMothership {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetMothership {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetMothership {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeTargetMothership"
    }
}
impl Default for BehaviorTreeNodeTargetMothership {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeTargetMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetMothership) -> Self {
        Self::TargetMothership(item)
    }
}
impl BehaviorTreeNodeTargetMothership {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn target_mothership() -> BehaviorTreeNodeTargetMothership {
        BehaviorTreeNodeTargetMothership::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMothershipLowHp {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeMothershipLowHp {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<f32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<f32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMothershipLowHp {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMothershipLowHp"
    }
}
impl Default for BehaviorTreeNodeMothershipLowHp {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMothershipLowHp> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMothershipLowHp) -> Self {
        Self::MothershipLowHp(item)
    }
}
impl BehaviorTreeNodeMothershipLowHp {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn mothership_low_hp() -> BehaviorTreeNodeMothershipLowHp {
        BehaviorTreeNodeMothershipLowHp::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMothershipDistanceExceeded {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    ///Max distance. If value is 0, prefefined value will be used (e.g. DroneBay range)
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMothershipDistanceExceeded {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<f32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<f32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMothershipDistanceExceeded {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMothershipDistanceExceeded"
    }
}
impl Default for BehaviorTreeNodeMothershipDistanceExceeded {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMothershipDistanceExceeded> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMothershipDistanceExceeded) -> Self {
        Self::MothershipDistanceExceeded(item)
    }
}
impl BehaviorTreeNodeMothershipDistanceExceeded {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn mothership_distance_exceeded() -> BehaviorTreeNodeMothershipDistanceExceeded {
        BehaviorTreeNodeMothershipDistanceExceeded::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMakeTargetMothership {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMakeTargetMothership {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMakeTargetMothership {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMakeTargetMothership"
    }
}
impl Default for BehaviorTreeNodeMakeTargetMothership {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMakeTargetMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMakeTargetMothership) -> Self {
        Self::MakeTargetMothership(item)
    }
}
impl BehaviorTreeNodeMakeTargetMothership {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn make_target_mothership() -> BehaviorTreeNodeMakeTargetMothership {
        BehaviorTreeNodeMakeTargetMothership::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeShowMessage {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
    pub r#color: String,
}
impl BehaviorTreeNodeShowMessage {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
            r#color: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeShowMessage {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeShowMessage"
    }
}
impl Default for BehaviorTreeNodeShowMessage {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeShowMessage> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeShowMessage) -> Self {
        Self::ShowMessage(item)
    }
}
impl BehaviorTreeNodeShowMessage {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn show_message() -> BehaviorTreeNodeShowMessage {
        BehaviorTreeNodeShowMessage::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeDebugLog {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeDebugLog {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeDebugLog {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeDebugLog"
    }
}
impl Default for BehaviorTreeNodeDebugLog {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeDebugLog> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeDebugLog) -> Self {
        Self::DebugLog(item)
    }
}
impl BehaviorTreeNodeDebugLog {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn debug_log() -> BehaviorTreeNodeDebugLog {
        BehaviorTreeNodeDebugLog::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSetValue {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#result: bool,
    pub r#text: String,
}
impl BehaviorTreeNodeSetValue {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#result: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_result(mut self, r#result: impl Into<bool>) -> Self {
        self.r#result = r#result.into();
        self
    }
    pub fn set_result(&mut self, r#result: impl Into<bool>) -> &mut Self {
        self.r#result = r#result.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSetValue {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSetValue"
    }
}
impl Default for BehaviorTreeNodeSetValue {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSetValue> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSetValue) -> Self {
        Self::SetValue(item)
    }
}
impl BehaviorTreeNodeSetValue {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn set_value() -> BehaviorTreeNodeSetValue {
        BehaviorTreeNodeSetValue::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeGetValue {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeGetValue {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeGetValue {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeGetValue"
    }
}
impl Default for BehaviorTreeNodeGetValue {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeGetValue> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeGetValue) -> Self {
        Self::GetValue(item)
    }
}
impl BehaviorTreeNodeGetValue {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn get_value() -> BehaviorTreeNodeGetValue {
        BehaviorTreeNodeGetValue::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSendMessage {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeSendMessage {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSendMessage {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSendMessage"
    }
}
impl Default for BehaviorTreeNodeSendMessage {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSendMessage> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSendMessage) -> Self {
        Self::SendMessage(item)
    }
}
impl BehaviorTreeNodeSendMessage {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn send_message() -> BehaviorTreeNodeSendMessage {
        BehaviorTreeNodeSendMessage::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeMessageReceived {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeMessageReceived {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeMessageReceived {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeMessageReceived"
    }
}
impl Default for BehaviorTreeNodeMessageReceived {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeMessageReceived> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMessageReceived) -> Self {
        Self::MessageReceived(item)
    }
}
impl BehaviorTreeNodeMessageReceived {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn message_received() -> BehaviorTreeNodeMessageReceived {
        BehaviorTreeNodeMessageReceived::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeTargetMessageSender {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetMessageSender {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetMessageSender {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeTargetMessageSender"
    }
}
impl Default for BehaviorTreeNodeTargetMessageSender {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeTargetMessageSender> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetMessageSender) -> Self {
        Self::TargetMessageSender(item)
    }
}
impl BehaviorTreeNodeTargetMessageSender {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn target_message_sender() -> BehaviorTreeNodeTargetMessageSender {
        BehaviorTreeNodeTargetMessageSender::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeSaveTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeSaveTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeSaveTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeSaveTarget"
    }
}
impl Default for BehaviorTreeNodeSaveTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeSaveTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSaveTarget) -> Self {
        Self::SaveTarget(item)
    }
}
impl BehaviorTreeNodeSaveTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn save_target() -> BehaviorTreeNodeSaveTarget {
        BehaviorTreeNodeSaveTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeLoadTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeLoadTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeLoadTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeLoadTarget"
    }
}
impl Default for BehaviorTreeNodeLoadTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeLoadTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLoadTarget) -> Self {
        Self::LoadTarget(item)
    }
}
impl BehaviorTreeNodeLoadTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn load_target() -> BehaviorTreeNodeLoadTarget {
        BehaviorTreeNodeLoadTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeHasSavedTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeHasSavedTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasSavedTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeHasSavedTarget"
    }
}
impl Default for BehaviorTreeNodeHasSavedTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeHasSavedTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasSavedTarget) -> Self {
        Self::HasSavedTarget(item)
    }
}
impl BehaviorTreeNodeHasSavedTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn has_saved_target() -> BehaviorTreeNodeHasSavedTarget {
        BehaviorTreeNodeHasSavedTarget::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTreeNodeForgetSavedTarget {
    ///The node will not execute and will return FAILURE if the requirement is not met
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeForgetSavedTarget {
    pub fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<BehaviorNodeRequirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(
        &mut self,
        r#requirement: impl Into<BehaviorNodeRequirement>,
    ) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_text(mut self, r#text: impl Into<String>) -> Self {
        self.r#text = r#text.into();
        self
    }
    pub fn set_text(&mut self, r#text: impl Into<String>) -> &mut Self {
        self.r#text = r#text.into();
        self
    }
}
impl DatabaseItem for BehaviorTreeNodeForgetSavedTarget {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTreeNodeForgetSavedTarget"
    }
}
impl Default for BehaviorTreeNodeForgetSavedTarget {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BehaviorTreeNodeForgetSavedTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeForgetSavedTarget) -> Self {
        Self::ForgetSavedTarget(item)
    }
}
impl BehaviorTreeNodeForgetSavedTarget {
    pub fn wrap(self) -> BehaviorTreeNode {
        self.into()
    }
}
impl BehaviorTreeNode {
    pub fn forget_saved_target() -> BehaviorTreeNodeForgetSavedTarget {
        BehaviorTreeNodeForgetSavedTarget::new()
    }
}
impl serde::Serialize for BehaviorTreeNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "BehaviorTreeNode")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: BehaviorNodeType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Success(x) => AdjTagged {
                t: BehaviorNodeType::Success,
                c: x,
            }
            .serialize(serializer),
            Self::Failure(x) => AdjTagged {
                t: BehaviorNodeType::Failure,
                c: x,
            }
            .serialize(serializer),
            Self::SubTree(x) => AdjTagged {
                t: BehaviorNodeType::SubTree,
                c: x,
            }
            .serialize(serializer),
            Self::Selector(x) => AdjTagged {
                t: BehaviorNodeType::Selector,
                c: x,
            }
            .serialize(serializer),
            Self::Sequence(x) => AdjTagged {
                t: BehaviorNodeType::Sequence,
                c: x,
            }
            .serialize(serializer),
            Self::Parallel(x) => AdjTagged {
                t: BehaviorNodeType::Parallel,
                c: x,
            }
            .serialize(serializer),
            Self::RandomSelector(x) => AdjTagged {
                t: BehaviorNodeType::RandomSelector,
                c: x,
            }
            .serialize(serializer),
            Self::Invertor(x) => AdjTagged {
                t: BehaviorNodeType::Invertor,
                c: x,
            }
            .serialize(serializer),
            Self::Cooldown(x) => AdjTagged {
                t: BehaviorNodeType::Cooldown,
                c: x,
            }
            .serialize(serializer),
            Self::Execute(x) => AdjTagged {
                t: BehaviorNodeType::Execute,
                c: x,
            }
            .serialize(serializer),
            Self::ParallelSequence(x) => AdjTagged {
                t: BehaviorNodeType::ParallelSequence,
                c: x,
            }
            .serialize(serializer),
            Self::PreserveTarget(x) => AdjTagged {
                t: BehaviorNodeType::PreserveTarget,
                c: x,
            }
            .serialize(serializer),
            Self::IfThenElse(x) => AdjTagged {
                t: BehaviorNodeType::IfThenElse,
                c: x,
            }
            .serialize(serializer),
            Self::HasEnoughEnergy(x) => AdjTagged {
                t: BehaviorNodeType::HasEnoughEnergy,
                c: x,
            }
            .serialize(serializer),
            Self::IsLowOnHp(x) => AdjTagged {
                t: BehaviorNodeType::IsLowOnHp,
                c: x,
            }
            .serialize(serializer),
            Self::IsControledByPlayer(x) => AdjTagged {
                t: BehaviorNodeType::IsControledByPlayer,
                c: x,
            }
            .serialize(serializer),
            Self::HasIncomingThreat(x) => AdjTagged {
                t: BehaviorNodeType::HasIncomingThreat,
                c: x,
            }
            .serialize(serializer),
            Self::HasAdditionalTargets(x) => AdjTagged {
                t: BehaviorNodeType::HasAdditionalTargets,
                c: x,
            }
            .serialize(serializer),
            Self::IsFasterThanTarget(x) => AdjTagged {
                t: BehaviorNodeType::IsFasterThanTarget,
                c: x,
            }
            .serialize(serializer),
            Self::HasMainTarget(x) => AdjTagged {
                t: BehaviorNodeType::HasMainTarget,
                c: x,
            }
            .serialize(serializer),
            Self::MainTargetIsAlly(x) => AdjTagged {
                t: BehaviorNodeType::MainTargetIsAlly,
                c: x,
            }
            .serialize(serializer),
            Self::MainTargetIsEnemy(x) => AdjTagged {
                t: BehaviorNodeType::MainTargetIsEnemy,
                c: x,
            }
            .serialize(serializer),
            Self::MainTargetLowHp(x) => AdjTagged {
                t: BehaviorNodeType::MainTargetLowHp,
                c: x,
            }
            .serialize(serializer),
            Self::MainTargetWithinAttackRange(x) => AdjTagged {
                t: BehaviorNodeType::MainTargetWithinAttackRange,
                c: x,
            }
            .serialize(serializer),
            Self::HasMothership(x) => AdjTagged {
                t: BehaviorNodeType::HasMothership,
                c: x,
            }
            .serialize(serializer),
            Self::TargetDistance(x) => AdjTagged {
                t: BehaviorNodeType::TargetDistance,
                c: x,
            }
            .serialize(serializer),
            Self::HasLongerAttackRange(x) => AdjTagged {
                t: BehaviorNodeType::HasLongerAttackRange,
                c: x,
            }
            .serialize(serializer),
            Self::FindEnemy(x) => AdjTagged {
                t: BehaviorNodeType::FindEnemy,
                c: x,
            }
            .serialize(serializer),
            Self::MoveToAttackRange(x) => AdjTagged {
                t: BehaviorNodeType::MoveToAttackRange,
                c: x,
            }
            .serialize(serializer),
            Self::AttackMainTarget(x) => AdjTagged {
                t: BehaviorNodeType::AttackMainTarget,
                c: x,
            }
            .serialize(serializer),
            Self::SelectWeapon(x) => AdjTagged {
                t: BehaviorNodeType::SelectWeapon,
                c: x,
            }
            .serialize(serializer),
            Self::SpawnDrones(x) => AdjTagged {
                t: BehaviorNodeType::SpawnDrones,
                c: x,
            }
            .serialize(serializer),
            Self::Ram(x) => AdjTagged {
                t: BehaviorNodeType::Ram,
                c: x,
            }
            .serialize(serializer),
            Self::DetonateShip(x) => AdjTagged {
                t: BehaviorNodeType::DetonateShip,
                c: x,
            }
            .serialize(serializer),
            Self::Vanish(x) => AdjTagged {
                t: BehaviorNodeType::Vanish,
                c: x,
            }
            .serialize(serializer),
            Self::MaintainAttackRange(x) => AdjTagged {
                t: BehaviorNodeType::MaintainAttackRange,
                c: x,
            }
            .serialize(serializer),
            Self::Wait(x) => AdjTagged {
                t: BehaviorNodeType::Wait,
                c: x,
            }
            .serialize(serializer),
            Self::LookAtTarget(x) => AdjTagged {
                t: BehaviorNodeType::LookAtTarget,
                c: x,
            }
            .serialize(serializer),
            Self::LookForAdditionalTargets(x) => AdjTagged {
                t: BehaviorNodeType::LookForAdditionalTargets,
                c: x,
            }
            .serialize(serializer),
            Self::LookForThreats(x) => AdjTagged {
                t: BehaviorNodeType::LookForThreats,
                c: x,
            }
            .serialize(serializer),
            Self::MatchVelocityWithTarget(x) => AdjTagged {
                t: BehaviorNodeType::MatchVelocityWithTarget,
                c: x,
            }
            .serialize(serializer),
            Self::ActivateDevice(x) => AdjTagged {
                t: BehaviorNodeType::ActivateDevice,
                c: x,
            }
            .serialize(serializer),
            Self::RechargeEnergy(x) => AdjTagged {
                t: BehaviorNodeType::RechargeEnergy,
                c: x,
            }
            .serialize(serializer),
            Self::SustainAim(x) => AdjTagged {
                t: BehaviorNodeType::SustainAim,
                c: x,
            }
            .serialize(serializer),
            Self::ChargeWeapons(x) => AdjTagged {
                t: BehaviorNodeType::ChargeWeapons,
                c: x,
            }
            .serialize(serializer),
            Self::Chase(x) => AdjTagged {
                t: BehaviorNodeType::Chase,
                c: x,
            }
            .serialize(serializer),
            Self::AvoidThreats(x) => AdjTagged {
                t: BehaviorNodeType::AvoidThreats,
                c: x,
            }
            .serialize(serializer),
            Self::SlowDown(x) => AdjTagged {
                t: BehaviorNodeType::SlowDown,
                c: x,
            }
            .serialize(serializer),
            Self::UseRecoil(x) => AdjTagged {
                t: BehaviorNodeType::UseRecoil,
                c: x,
            }
            .serialize(serializer),
            Self::DefendWithFronalShield(x) => AdjTagged {
                t: BehaviorNodeType::DefendWithFronalShield,
                c: x,
            }
            .serialize(serializer),
            Self::TrackControllableAmmo(x) => AdjTagged {
                t: BehaviorNodeType::TrackControllableAmmo,
                c: x,
            }
            .serialize(serializer),
            Self::KeepDistance(x) => AdjTagged {
                t: BehaviorNodeType::KeepDistance,
                c: x,
            }
            .serialize(serializer),
            Self::ForgetMainTarget(x) => AdjTagged {
                t: BehaviorNodeType::ForgetMainTarget,
                c: x,
            }
            .serialize(serializer),
            Self::EscapeTargetAttackRadius(x) => AdjTagged {
                t: BehaviorNodeType::EscapeTargetAttackRadius,
                c: x,
            }
            .serialize(serializer),
            Self::AttackAdditionalTargets(x) => AdjTagged {
                t: BehaviorNodeType::AttackAdditionalTargets,
                c: x,
            }
            .serialize(serializer),
            Self::TargetAllyStarbase(x) => AdjTagged {
                t: BehaviorNodeType::TargetAllyStarbase,
                c: x,
            }
            .serialize(serializer),
            Self::TargetEnemyStarbase(x) => AdjTagged {
                t: BehaviorNodeType::TargetEnemyStarbase,
                c: x,
            }
            .serialize(serializer),
            Self::BypassObstacles(x) => AdjTagged {
                t: BehaviorNodeType::BypassObstacles,
                c: x,
            }
            .serialize(serializer),
            Self::AttackTurretTargets(x) => AdjTagged {
                t: BehaviorNodeType::AttackTurretTargets,
                c: x,
            }
            .serialize(serializer),
            Self::EnginePropulsionForce(x) => AdjTagged {
                t: BehaviorNodeType::EnginePropulsionForce,
                c: x,
            }
            .serialize(serializer),
            Self::MotherShipRetreated(x) => AdjTagged {
                t: BehaviorNodeType::MotherShipRetreated,
                c: x,
            }
            .serialize(serializer),
            Self::MotherShipDestroyed(x) => AdjTagged {
                t: BehaviorNodeType::MotherShipDestroyed,
                c: x,
            }
            .serialize(serializer),
            Self::FlyAroundMothership(x) => AdjTagged {
                t: BehaviorNodeType::FlyAroundMothership,
                c: x,
            }
            .serialize(serializer),
            Self::GoBerserk(x) => AdjTagged {
                t: BehaviorNodeType::GoBerserk,
                c: x,
            }
            .serialize(serializer),
            Self::TargetMothership(x) => AdjTagged {
                t: BehaviorNodeType::TargetMothership,
                c: x,
            }
            .serialize(serializer),
            Self::MothershipLowHp(x) => AdjTagged {
                t: BehaviorNodeType::MothershipLowHp,
                c: x,
            }
            .serialize(serializer),
            Self::MothershipDistanceExceeded(x) => AdjTagged {
                t: BehaviorNodeType::MothershipDistanceExceeded,
                c: x,
            }
            .serialize(serializer),
            Self::MakeTargetMothership(x) => AdjTagged {
                t: BehaviorNodeType::MakeTargetMothership,
                c: x,
            }
            .serialize(serializer),
            Self::ShowMessage(x) => AdjTagged {
                t: BehaviorNodeType::ShowMessage,
                c: x,
            }
            .serialize(serializer),
            Self::DebugLog(x) => AdjTagged {
                t: BehaviorNodeType::DebugLog,
                c: x,
            }
            .serialize(serializer),
            Self::SetValue(x) => AdjTagged {
                t: BehaviorNodeType::SetValue,
                c: x,
            }
            .serialize(serializer),
            Self::GetValue(x) => AdjTagged {
                t: BehaviorNodeType::GetValue,
                c: x,
            }
            .serialize(serializer),
            Self::SendMessage(x) => AdjTagged {
                t: BehaviorNodeType::SendMessage,
                c: x,
            }
            .serialize(serializer),
            Self::MessageReceived(x) => AdjTagged {
                t: BehaviorNodeType::MessageReceived,
                c: x,
            }
            .serialize(serializer),
            Self::TargetMessageSender(x) => AdjTagged {
                t: BehaviorNodeType::TargetMessageSender,
                c: x,
            }
            .serialize(serializer),
            Self::SaveTarget(x) => AdjTagged {
                t: BehaviorNodeType::SaveTarget,
                c: x,
            }
            .serialize(serializer),
            Self::LoadTarget(x) => AdjTagged {
                t: BehaviorNodeType::LoadTarget,
                c: x,
            }
            .serialize(serializer),
            Self::HasSavedTarget(x) => AdjTagged {
                t: BehaviorNodeType::HasSavedTarget,
                c: x,
            }
            .serialize(serializer),
            Self::ForgetSavedTarget(x) => AdjTagged {
                t: BehaviorNodeType::ForgetSavedTarget,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl BehaviorTreeNode {
    pub fn r#requirement(&self) -> &BehaviorNodeRequirement {
        match self {
            Self::Success(x) => &x.r#requirement,
            Self::Failure(x) => &x.r#requirement,
            Self::SubTree(x) => &x.r#requirement,
            Self::Selector(x) => &x.r#requirement,
            Self::Sequence(x) => &x.r#requirement,
            Self::Parallel(x) => &x.r#requirement,
            Self::RandomSelector(x) => &x.r#requirement,
            Self::Invertor(x) => &x.r#requirement,
            Self::Cooldown(x) => &x.r#requirement,
            Self::Execute(x) => &x.r#requirement,
            Self::ParallelSequence(x) => &x.r#requirement,
            Self::PreserveTarget(x) => &x.r#requirement,
            Self::IfThenElse(x) => &x.r#requirement,
            Self::HasEnoughEnergy(x) => &x.r#requirement,
            Self::IsLowOnHp(x) => &x.r#requirement,
            Self::IsControledByPlayer(x) => &x.r#requirement,
            Self::HasIncomingThreat(x) => &x.r#requirement,
            Self::HasAdditionalTargets(x) => &x.r#requirement,
            Self::IsFasterThanTarget(x) => &x.r#requirement,
            Self::HasMainTarget(x) => &x.r#requirement,
            Self::MainTargetIsAlly(x) => &x.r#requirement,
            Self::MainTargetIsEnemy(x) => &x.r#requirement,
            Self::MainTargetLowHp(x) => &x.r#requirement,
            Self::MainTargetWithinAttackRange(x) => &x.r#requirement,
            Self::HasMothership(x) => &x.r#requirement,
            Self::TargetDistance(x) => &x.r#requirement,
            Self::HasLongerAttackRange(x) => &x.r#requirement,
            Self::FindEnemy(x) => &x.r#requirement,
            Self::MoveToAttackRange(x) => &x.r#requirement,
            Self::AttackMainTarget(x) => &x.r#requirement,
            Self::SelectWeapon(x) => &x.r#requirement,
            Self::SpawnDrones(x) => &x.r#requirement,
            Self::Ram(x) => &x.r#requirement,
            Self::DetonateShip(x) => &x.r#requirement,
            Self::Vanish(x) => &x.r#requirement,
            Self::MaintainAttackRange(x) => &x.r#requirement,
            Self::Wait(x) => &x.r#requirement,
            Self::LookAtTarget(x) => &x.r#requirement,
            Self::LookForAdditionalTargets(x) => &x.r#requirement,
            Self::LookForThreats(x) => &x.r#requirement,
            Self::MatchVelocityWithTarget(x) => &x.r#requirement,
            Self::ActivateDevice(x) => &x.r#requirement,
            Self::RechargeEnergy(x) => &x.r#requirement,
            Self::SustainAim(x) => &x.r#requirement,
            Self::ChargeWeapons(x) => &x.r#requirement,
            Self::Chase(x) => &x.r#requirement,
            Self::AvoidThreats(x) => &x.r#requirement,
            Self::SlowDown(x) => &x.r#requirement,
            Self::UseRecoil(x) => &x.r#requirement,
            Self::DefendWithFronalShield(x) => &x.r#requirement,
            Self::TrackControllableAmmo(x) => &x.r#requirement,
            Self::KeepDistance(x) => &x.r#requirement,
            Self::ForgetMainTarget(x) => &x.r#requirement,
            Self::EscapeTargetAttackRadius(x) => &x.r#requirement,
            Self::AttackAdditionalTargets(x) => &x.r#requirement,
            Self::TargetAllyStarbase(x) => &x.r#requirement,
            Self::TargetEnemyStarbase(x) => &x.r#requirement,
            Self::BypassObstacles(x) => &x.r#requirement,
            Self::AttackTurretTargets(x) => &x.r#requirement,
            Self::EnginePropulsionForce(x) => &x.r#requirement,
            Self::MotherShipRetreated(x) => &x.r#requirement,
            Self::MotherShipDestroyed(x) => &x.r#requirement,
            Self::FlyAroundMothership(x) => &x.r#requirement,
            Self::GoBerserk(x) => &x.r#requirement,
            Self::TargetMothership(x) => &x.r#requirement,
            Self::MothershipLowHp(x) => &x.r#requirement,
            Self::MothershipDistanceExceeded(x) => &x.r#requirement,
            Self::MakeTargetMothership(x) => &x.r#requirement,
            Self::ShowMessage(x) => &x.r#requirement,
            Self::DebugLog(x) => &x.r#requirement,
            Self::SetValue(x) => &x.r#requirement,
            Self::GetValue(x) => &x.r#requirement,
            Self::SendMessage(x) => &x.r#requirement,
            Self::MessageReceived(x) => &x.r#requirement,
            Self::TargetMessageSender(x) => &x.r#requirement,
            Self::SaveTarget(x) => &x.r#requirement,
            Self::LoadTarget(x) => &x.r#requirement,
            Self::HasSavedTarget(x) => &x.r#requirement,
            Self::ForgetSavedTarget(x) => &x.r#requirement,
        }
    }
    pub fn requirement_mut(&mut self) -> &mut BehaviorNodeRequirement {
        match self {
            Self::Success(x) => &mut x.r#requirement,
            Self::Failure(x) => &mut x.r#requirement,
            Self::SubTree(x) => &mut x.r#requirement,
            Self::Selector(x) => &mut x.r#requirement,
            Self::Sequence(x) => &mut x.r#requirement,
            Self::Parallel(x) => &mut x.r#requirement,
            Self::RandomSelector(x) => &mut x.r#requirement,
            Self::Invertor(x) => &mut x.r#requirement,
            Self::Cooldown(x) => &mut x.r#requirement,
            Self::Execute(x) => &mut x.r#requirement,
            Self::ParallelSequence(x) => &mut x.r#requirement,
            Self::PreserveTarget(x) => &mut x.r#requirement,
            Self::IfThenElse(x) => &mut x.r#requirement,
            Self::HasEnoughEnergy(x) => &mut x.r#requirement,
            Self::IsLowOnHp(x) => &mut x.r#requirement,
            Self::IsControledByPlayer(x) => &mut x.r#requirement,
            Self::HasIncomingThreat(x) => &mut x.r#requirement,
            Self::HasAdditionalTargets(x) => &mut x.r#requirement,
            Self::IsFasterThanTarget(x) => &mut x.r#requirement,
            Self::HasMainTarget(x) => &mut x.r#requirement,
            Self::MainTargetIsAlly(x) => &mut x.r#requirement,
            Self::MainTargetIsEnemy(x) => &mut x.r#requirement,
            Self::MainTargetLowHp(x) => &mut x.r#requirement,
            Self::MainTargetWithinAttackRange(x) => &mut x.r#requirement,
            Self::HasMothership(x) => &mut x.r#requirement,
            Self::TargetDistance(x) => &mut x.r#requirement,
            Self::HasLongerAttackRange(x) => &mut x.r#requirement,
            Self::FindEnemy(x) => &mut x.r#requirement,
            Self::MoveToAttackRange(x) => &mut x.r#requirement,
            Self::AttackMainTarget(x) => &mut x.r#requirement,
            Self::SelectWeapon(x) => &mut x.r#requirement,
            Self::SpawnDrones(x) => &mut x.r#requirement,
            Self::Ram(x) => &mut x.r#requirement,
            Self::DetonateShip(x) => &mut x.r#requirement,
            Self::Vanish(x) => &mut x.r#requirement,
            Self::MaintainAttackRange(x) => &mut x.r#requirement,
            Self::Wait(x) => &mut x.r#requirement,
            Self::LookAtTarget(x) => &mut x.r#requirement,
            Self::LookForAdditionalTargets(x) => &mut x.r#requirement,
            Self::LookForThreats(x) => &mut x.r#requirement,
            Self::MatchVelocityWithTarget(x) => &mut x.r#requirement,
            Self::ActivateDevice(x) => &mut x.r#requirement,
            Self::RechargeEnergy(x) => &mut x.r#requirement,
            Self::SustainAim(x) => &mut x.r#requirement,
            Self::ChargeWeapons(x) => &mut x.r#requirement,
            Self::Chase(x) => &mut x.r#requirement,
            Self::AvoidThreats(x) => &mut x.r#requirement,
            Self::SlowDown(x) => &mut x.r#requirement,
            Self::UseRecoil(x) => &mut x.r#requirement,
            Self::DefendWithFronalShield(x) => &mut x.r#requirement,
            Self::TrackControllableAmmo(x) => &mut x.r#requirement,
            Self::KeepDistance(x) => &mut x.r#requirement,
            Self::ForgetMainTarget(x) => &mut x.r#requirement,
            Self::EscapeTargetAttackRadius(x) => &mut x.r#requirement,
            Self::AttackAdditionalTargets(x) => &mut x.r#requirement,
            Self::TargetAllyStarbase(x) => &mut x.r#requirement,
            Self::TargetEnemyStarbase(x) => &mut x.r#requirement,
            Self::BypassObstacles(x) => &mut x.r#requirement,
            Self::AttackTurretTargets(x) => &mut x.r#requirement,
            Self::EnginePropulsionForce(x) => &mut x.r#requirement,
            Self::MotherShipRetreated(x) => &mut x.r#requirement,
            Self::MotherShipDestroyed(x) => &mut x.r#requirement,
            Self::FlyAroundMothership(x) => &mut x.r#requirement,
            Self::GoBerserk(x) => &mut x.r#requirement,
            Self::TargetMothership(x) => &mut x.r#requirement,
            Self::MothershipLowHp(x) => &mut x.r#requirement,
            Self::MothershipDistanceExceeded(x) => &mut x.r#requirement,
            Self::MakeTargetMothership(x) => &mut x.r#requirement,
            Self::ShowMessage(x) => &mut x.r#requirement,
            Self::DebugLog(x) => &mut x.r#requirement,
            Self::SetValue(x) => &mut x.r#requirement,
            Self::GetValue(x) => &mut x.r#requirement,
            Self::SendMessage(x) => &mut x.r#requirement,
            Self::MessageReceived(x) => &mut x.r#requirement,
            Self::TargetMessageSender(x) => &mut x.r#requirement,
            Self::SaveTarget(x) => &mut x.r#requirement,
            Self::LoadTarget(x) => &mut x.r#requirement,
            Self::HasSavedTarget(x) => &mut x.r#requirement,
            Self::ForgetSavedTarget(x) => &mut x.r#requirement,
        }
    }
}
impl DatabaseItem for BehaviorTreeNode {
    fn validate(&mut self) {
        match self {
            Self::Success(x) => x.validate(),
            Self::Failure(x) => x.validate(),
            Self::SubTree(x) => x.validate(),
            Self::Selector(x) => x.validate(),
            Self::Sequence(x) => x.validate(),
            Self::Parallel(x) => x.validate(),
            Self::RandomSelector(x) => x.validate(),
            Self::Invertor(x) => x.validate(),
            Self::Cooldown(x) => x.validate(),
            Self::Execute(x) => x.validate(),
            Self::ParallelSequence(x) => x.validate(),
            Self::PreserveTarget(x) => x.validate(),
            Self::IfThenElse(x) => x.validate(),
            Self::HasEnoughEnergy(x) => x.validate(),
            Self::IsLowOnHp(x) => x.validate(),
            Self::IsControledByPlayer(x) => x.validate(),
            Self::HasIncomingThreat(x) => x.validate(),
            Self::HasAdditionalTargets(x) => x.validate(),
            Self::IsFasterThanTarget(x) => x.validate(),
            Self::HasMainTarget(x) => x.validate(),
            Self::MainTargetIsAlly(x) => x.validate(),
            Self::MainTargetIsEnemy(x) => x.validate(),
            Self::MainTargetLowHp(x) => x.validate(),
            Self::MainTargetWithinAttackRange(x) => x.validate(),
            Self::HasMothership(x) => x.validate(),
            Self::TargetDistance(x) => x.validate(),
            Self::HasLongerAttackRange(x) => x.validate(),
            Self::FindEnemy(x) => x.validate(),
            Self::MoveToAttackRange(x) => x.validate(),
            Self::AttackMainTarget(x) => x.validate(),
            Self::SelectWeapon(x) => x.validate(),
            Self::SpawnDrones(x) => x.validate(),
            Self::Ram(x) => x.validate(),
            Self::DetonateShip(x) => x.validate(),
            Self::Vanish(x) => x.validate(),
            Self::MaintainAttackRange(x) => x.validate(),
            Self::Wait(x) => x.validate(),
            Self::LookAtTarget(x) => x.validate(),
            Self::LookForAdditionalTargets(x) => x.validate(),
            Self::LookForThreats(x) => x.validate(),
            Self::MatchVelocityWithTarget(x) => x.validate(),
            Self::ActivateDevice(x) => x.validate(),
            Self::RechargeEnergy(x) => x.validate(),
            Self::SustainAim(x) => x.validate(),
            Self::ChargeWeapons(x) => x.validate(),
            Self::Chase(x) => x.validate(),
            Self::AvoidThreats(x) => x.validate(),
            Self::SlowDown(x) => x.validate(),
            Self::UseRecoil(x) => x.validate(),
            Self::DefendWithFronalShield(x) => x.validate(),
            Self::TrackControllableAmmo(x) => x.validate(),
            Self::KeepDistance(x) => x.validate(),
            Self::ForgetMainTarget(x) => x.validate(),
            Self::EscapeTargetAttackRadius(x) => x.validate(),
            Self::AttackAdditionalTargets(x) => x.validate(),
            Self::TargetAllyStarbase(x) => x.validate(),
            Self::TargetEnemyStarbase(x) => x.validate(),
            Self::BypassObstacles(x) => x.validate(),
            Self::AttackTurretTargets(x) => x.validate(),
            Self::EnginePropulsionForce(x) => x.validate(),
            Self::MotherShipRetreated(x) => x.validate(),
            Self::MotherShipDestroyed(x) => x.validate(),
            Self::FlyAroundMothership(x) => x.validate(),
            Self::GoBerserk(x) => x.validate(),
            Self::TargetMothership(x) => x.validate(),
            Self::MothershipLowHp(x) => x.validate(),
            Self::MothershipDistanceExceeded(x) => x.validate(),
            Self::MakeTargetMothership(x) => x.validate(),
            Self::ShowMessage(x) => x.validate(),
            Self::DebugLog(x) => x.validate(),
            Self::SetValue(x) => x.validate(),
            Self::GetValue(x) => x.validate(),
            Self::SendMessage(x) => x.validate(),
            Self::MessageReceived(x) => x.validate(),
            Self::TargetMessageSender(x) => x.validate(),
            Self::SaveTarget(x) => x.validate(),
            Self::LoadTarget(x) => x.validate(),
            Self::HasSavedTarget(x) => x.validate(),
            Self::ForgetSavedTarget(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "BehaviorTreeNode"
    }
}
impl BehaviorTreeNode {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Success(_) => BehaviorTreeNodeSuccess::type_name(),
            Self::Failure(_) => BehaviorTreeNodeFailure::type_name(),
            Self::SubTree(_) => BehaviorTreeNodeSubTree::type_name(),
            Self::Selector(_) => BehaviorTreeNodeSelector::type_name(),
            Self::Sequence(_) => BehaviorTreeNodeSequence::type_name(),
            Self::Parallel(_) => BehaviorTreeNodeParallel::type_name(),
            Self::RandomSelector(_) => BehaviorTreeNodeRandomSelector::type_name(),
            Self::Invertor(_) => BehaviorTreeNodeInvertor::type_name(),
            Self::Cooldown(_) => BehaviorTreeNodeCooldown::type_name(),
            Self::Execute(_) => BehaviorTreeNodeExecute::type_name(),
            Self::ParallelSequence(_) => BehaviorTreeNodeParallelSequence::type_name(),
            Self::PreserveTarget(_) => BehaviorTreeNodePreserveTarget::type_name(),
            Self::IfThenElse(_) => BehaviorTreeNodeIfThenElse::type_name(),
            Self::HasEnoughEnergy(_) => BehaviorTreeNodeHasEnoughEnergy::type_name(),
            Self::IsLowOnHp(_) => BehaviorTreeNodeIsLowOnHp::type_name(),
            Self::IsControledByPlayer(_) => BehaviorTreeNodeIsControledByPlayer::type_name(),
            Self::HasIncomingThreat(_) => BehaviorTreeNodeHasIncomingThreat::type_name(),
            Self::HasAdditionalTargets(_) => BehaviorTreeNodeHasAdditionalTargets::type_name(),
            Self::IsFasterThanTarget(_) => BehaviorTreeNodeIsFasterThanTarget::type_name(),
            Self::HasMainTarget(_) => BehaviorTreeNodeHasMainTarget::type_name(),
            Self::MainTargetIsAlly(_) => BehaviorTreeNodeMainTargetIsAlly::type_name(),
            Self::MainTargetIsEnemy(_) => BehaviorTreeNodeMainTargetIsEnemy::type_name(),
            Self::MainTargetLowHp(_) => BehaviorTreeNodeMainTargetLowHp::type_name(),
            Self::MainTargetWithinAttackRange(_) => {
                BehaviorTreeNodeMainTargetWithinAttackRange::type_name()
            }
            Self::HasMothership(_) => BehaviorTreeNodeHasMothership::type_name(),
            Self::TargetDistance(_) => BehaviorTreeNodeTargetDistance::type_name(),
            Self::HasLongerAttackRange(_) => BehaviorTreeNodeHasLongerAttackRange::type_name(),
            Self::FindEnemy(_) => BehaviorTreeNodeFindEnemy::type_name(),
            Self::MoveToAttackRange(_) => BehaviorTreeNodeMoveToAttackRange::type_name(),
            Self::AttackMainTarget(_) => BehaviorTreeNodeAttackMainTarget::type_name(),
            Self::SelectWeapon(_) => BehaviorTreeNodeSelectWeapon::type_name(),
            Self::SpawnDrones(_) => BehaviorTreeNodeSpawnDrones::type_name(),
            Self::Ram(_) => BehaviorTreeNodeRam::type_name(),
            Self::DetonateShip(_) => BehaviorTreeNodeDetonateShip::type_name(),
            Self::Vanish(_) => BehaviorTreeNodeVanish::type_name(),
            Self::MaintainAttackRange(_) => BehaviorTreeNodeMaintainAttackRange::type_name(),
            Self::Wait(_) => BehaviorTreeNodeWait::type_name(),
            Self::LookAtTarget(_) => BehaviorTreeNodeLookAtTarget::type_name(),
            Self::LookForAdditionalTargets(_) => {
                BehaviorTreeNodeLookForAdditionalTargets::type_name()
            }
            Self::LookForThreats(_) => BehaviorTreeNodeLookForThreats::type_name(),
            Self::MatchVelocityWithTarget(_) => {
                BehaviorTreeNodeMatchVelocityWithTarget::type_name()
            }
            Self::ActivateDevice(_) => BehaviorTreeNodeActivateDevice::type_name(),
            Self::RechargeEnergy(_) => BehaviorTreeNodeRechargeEnergy::type_name(),
            Self::SustainAim(_) => BehaviorTreeNodeSustainAim::type_name(),
            Self::ChargeWeapons(_) => BehaviorTreeNodeChargeWeapons::type_name(),
            Self::Chase(_) => BehaviorTreeNodeChase::type_name(),
            Self::AvoidThreats(_) => BehaviorTreeNodeAvoidThreats::type_name(),
            Self::SlowDown(_) => BehaviorTreeNodeSlowDown::type_name(),
            Self::UseRecoil(_) => BehaviorTreeNodeUseRecoil::type_name(),
            Self::DefendWithFronalShield(_) => BehaviorTreeNodeDefendWithFronalShield::type_name(),
            Self::TrackControllableAmmo(_) => BehaviorTreeNodeTrackControllableAmmo::type_name(),
            Self::KeepDistance(_) => BehaviorTreeNodeKeepDistance::type_name(),
            Self::ForgetMainTarget(_) => BehaviorTreeNodeForgetMainTarget::type_name(),
            Self::EscapeTargetAttackRadius(_) => {
                BehaviorTreeNodeEscapeTargetAttackRadius::type_name()
            }
            Self::AttackAdditionalTargets(_) => {
                BehaviorTreeNodeAttackAdditionalTargets::type_name()
            }
            Self::TargetAllyStarbase(_) => BehaviorTreeNodeTargetAllyStarbase::type_name(),
            Self::TargetEnemyStarbase(_) => BehaviorTreeNodeTargetEnemyStarbase::type_name(),
            Self::BypassObstacles(_) => BehaviorTreeNodeBypassObstacles::type_name(),
            Self::AttackTurretTargets(_) => BehaviorTreeNodeAttackTurretTargets::type_name(),
            Self::EnginePropulsionForce(_) => BehaviorTreeNodeEnginePropulsionForce::type_name(),
            Self::MotherShipRetreated(_) => BehaviorTreeNodeMotherShipRetreated::type_name(),
            Self::MotherShipDestroyed(_) => BehaviorTreeNodeMotherShipDestroyed::type_name(),
            Self::FlyAroundMothership(_) => BehaviorTreeNodeFlyAroundMothership::type_name(),
            Self::GoBerserk(_) => BehaviorTreeNodeGoBerserk::type_name(),
            Self::TargetMothership(_) => BehaviorTreeNodeTargetMothership::type_name(),
            Self::MothershipLowHp(_) => BehaviorTreeNodeMothershipLowHp::type_name(),
            Self::MothershipDistanceExceeded(_) => {
                BehaviorTreeNodeMothershipDistanceExceeded::type_name()
            }
            Self::MakeTargetMothership(_) => BehaviorTreeNodeMakeTargetMothership::type_name(),
            Self::ShowMessage(_) => BehaviorTreeNodeShowMessage::type_name(),
            Self::DebugLog(_) => BehaviorTreeNodeDebugLog::type_name(),
            Self::SetValue(_) => BehaviorTreeNodeSetValue::type_name(),
            Self::GetValue(_) => BehaviorTreeNodeGetValue::type_name(),
            Self::SendMessage(_) => BehaviorTreeNodeSendMessage::type_name(),
            Self::MessageReceived(_) => BehaviorTreeNodeMessageReceived::type_name(),
            Self::TargetMessageSender(_) => BehaviorTreeNodeTargetMessageSender::type_name(),
            Self::SaveTarget(_) => BehaviorTreeNodeSaveTarget::type_name(),
            Self::LoadTarget(_) => BehaviorTreeNodeLoadTarget::type_name(),
            Self::HasSavedTarget(_) => BehaviorTreeNodeHasSavedTarget::type_name(),
            Self::ForgetSavedTarget(_) => BehaviorTreeNodeForgetSavedTarget::type_name(),
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Barrel.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Barrel {
    pub r#position: glam::f32::Vec2,
    pub r#rotation: f32,
    pub r#offset: f32,
    pub r#platform_type: i32,
    pub r#auto_aiming_arc: f32,
    pub r#rotation_speed: f32,
    pub r#weapon_class: String,
    pub r#image: String,
    pub r#size: f32,
}
impl Barrel {
    pub fn new() -> Self {
        Self {
            r#position: Default::default(),
            r#rotation: Default::default(),
            r#offset: Default::default(),
            r#platform_type: Default::default(),
            r#auto_aiming_arc: Default::default(),
            r#rotation_speed: Default::default(),
            r#weapon_class: Default::default(),
            r#image: Default::default(),
            r#size: Default::default(),
        }
    }
    pub fn with_position(mut self, r#position: impl Into<glam::f32::Vec2>) -> Self {
        self.r#position = r#position.into();
        self
    }
    pub fn set_position(&mut self, r#position: impl Into<glam::f32::Vec2>) -> &mut Self {
        self.r#position = r#position.into();
        self
    }
    pub fn with_rotation(mut self, r#rotation: impl Into<f32>) -> Self {
        self.r#rotation = r#rotation.into();
        self
    }
    pub fn set_rotation(&mut self, r#rotation: impl Into<f32>) -> &mut Self {
        self.r#rotation = r#rotation.into();
        self
    }
    pub fn with_offset(mut self, r#offset: impl Into<f32>) -> Self {
        self.r#offset = r#offset.into();
        self
    }
    pub fn set_offset(&mut self, r#offset: impl Into<f32>) -> &mut Self {
        self.r#offset = r#offset.into();
        self
    }
    pub fn with_platform_type(mut self, r#platform_type: impl Into<i32>) -> Self {
        self.r#platform_type = r#platform_type.into();
        self
    }
    pub fn set_platform_type(&mut self, r#platform_type: impl Into<i32>) -> &mut Self {
        self.r#platform_type = r#platform_type.into();
        self
    }
    pub fn with_auto_aiming_arc(mut self, r#auto_aiming_arc: impl Into<f32>) -> Self {
        self.r#auto_aiming_arc = r#auto_aiming_arc.into();
        self
    }
    pub fn set_auto_aiming_arc(&mut self, r#auto_aiming_arc: impl Into<f32>) -> &mut Self {
        self.r#auto_aiming_arc = r#auto_aiming_arc.into();
        self
    }
    pub fn with_rotation_speed(mut self, r#rotation_speed: impl Into<f32>) -> Self {
        self.r#rotation_speed = r#rotation_speed.into();
        self
    }
    pub fn set_rotation_speed(&mut self, r#rotation_speed: impl Into<f32>) -> &mut Self {
        self.r#rotation_speed = r#rotation_speed.into();
        self
    }
    pub fn with_weapon_class(mut self, r#weapon_class: impl Into<String>) -> Self {
        self.r#weapon_class = r#weapon_class.into();
        self
    }
    pub fn set_weapon_class(&mut self, r#weapon_class: impl Into<String>) -> &mut Self {
        self.r#weapon_class = r#weapon_class.into();
        self
    }
    pub fn with_image(mut self, r#image: impl Into<String>) -> Self {
        self.r#image = r#image.into();
        self
    }
    pub fn set_image(&mut self, r#image: impl Into<String>) -> &mut Self {
        self.r#image = r#image.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
}
impl DatabaseItem for Barrel {
    fn validate(&mut self) {
        if self.r#rotation < (-360f32 as f32) {
            tracing::warn!(
                field = "r#rotation",
                value = self.r#rotation,
                min = -360f32,
                "Field got truncated"
            );
            self.r#rotation = -360f32 as f32;
        }
        if self.r#rotation > (360f32 as f32) {
            tracing::warn!(
                field = "r#rotation",
                value = self.r#rotation,
                max = 360f32,
                "Field got truncated"
            );
            self.r#rotation = 360f32 as f32;
        }
        if self.r#offset < (0f32 as f32) {
            tracing::warn!(
                field = "r#offset",
                value = self.r#offset,
                min = 0f32,
                "Field got truncated"
            );
            self.r#offset = 0f32 as f32;
        }
        if self.r#offset > (1f32 as f32) {
            tracing::warn!(
                field = "r#offset",
                value = self.r#offset,
                max = 1f32,
                "Field got truncated"
            );
            self.r#offset = 1f32 as f32;
        }
        let dw: i32 = Default::default();
        if self.r#platform_type != dw {
            tracing::error!(
                ield = "r#platform_type",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        if self.r#auto_aiming_arc < (0f32 as f32) {
            tracing::warn!(
                field = "r#auto_aiming_arc",
                value = self.r#auto_aiming_arc,
                min = 0f32,
                "Field got truncated"
            );
            self.r#auto_aiming_arc = 0f32 as f32;
        }
        if self.r#auto_aiming_arc > (360f32 as f32) {
            tracing::warn!(
                field = "r#auto_aiming_arc",
                value = self.r#auto_aiming_arc,
                max = 360f32,
                "Field got truncated"
            );
            self.r#auto_aiming_arc = 360f32 as f32;
        }
        if self.r#rotation_speed < (0f32 as f32) {
            tracing::warn!(
                field = "r#rotation_speed",
                value = self.r#rotation_speed,
                min = 0f32,
                "Field got truncated"
            );
            self.r#rotation_speed = 0f32 as f32;
        }
        if self.r#rotation_speed > (1000f32 as f32) {
            tracing::warn!(
                field = "r#rotation_speed",
                value = self.r#rotation_speed,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#rotation_speed = 1000f32 as f32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#size = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "Barrel"
    }
}
impl Default for Barrel {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ComponentRestrictions.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComponentRestrictions {
    pub r#ship_sizes: std::collections::HashSet<SizeClass>,
    pub r#not_for_organic_ships: bool,
    pub r#not_for_mechanic_ships: bool,
    pub r#unique_component_tag: String,
    pub r#max_component_amount: i32,
}
impl ComponentRestrictions {
    pub fn new() -> Self {
        Self {
            r#ship_sizes: Default::default(),
            r#not_for_organic_ships: Default::default(),
            r#not_for_mechanic_ships: Default::default(),
            r#unique_component_tag: Default::default(),
            r#max_component_amount: Default::default(),
        }
    }
    pub fn with_ship_sizes(
        mut self,
        r#ship_sizes: impl Into<std::collections::HashSet<SizeClass>>,
    ) -> Self {
        self.r#ship_sizes = r#ship_sizes.into();
        self
    }
    pub fn set_ship_sizes(
        &mut self,
        r#ship_sizes: impl Into<std::collections::HashSet<SizeClass>>,
    ) -> &mut Self {
        self.r#ship_sizes = r#ship_sizes.into();
        self
    }
    pub fn with_not_for_organic_ships(mut self, r#not_for_organic_ships: impl Into<bool>) -> Self {
        self.r#not_for_organic_ships = r#not_for_organic_ships.into();
        self
    }
    pub fn set_not_for_organic_ships(
        &mut self,
        r#not_for_organic_ships: impl Into<bool>,
    ) -> &mut Self {
        self.r#not_for_organic_ships = r#not_for_organic_ships.into();
        self
    }
    pub fn with_not_for_mechanic_ships(
        mut self,
        r#not_for_mechanic_ships: impl Into<bool>,
    ) -> Self {
        self.r#not_for_mechanic_ships = r#not_for_mechanic_ships.into();
        self
    }
    pub fn set_not_for_mechanic_ships(
        &mut self,
        r#not_for_mechanic_ships: impl Into<bool>,
    ) -> &mut Self {
        self.r#not_for_mechanic_ships = r#not_for_mechanic_ships.into();
        self
    }
    pub fn with_unique_component_tag(mut self, r#unique_component_tag: impl Into<String>) -> Self {
        self.r#unique_component_tag = r#unique_component_tag.into();
        self
    }
    pub fn set_unique_component_tag(
        &mut self,
        r#unique_component_tag: impl Into<String>,
    ) -> &mut Self {
        self.r#unique_component_tag = r#unique_component_tag.into();
        self
    }
    pub fn with_max_component_amount(mut self, r#max_component_amount: impl Into<i32>) -> Self {
        self.r#max_component_amount = r#max_component_amount.into();
        self
    }
    pub fn set_max_component_amount(
        &mut self,
        r#max_component_amount: impl Into<i32>,
    ) -> &mut Self {
        self.r#max_component_amount = r#max_component_amount.into();
        self
    }
}
impl DatabaseItem for ComponentRestrictions {
    fn validate(&mut self) {
        if self.r#max_component_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_component_amount",
                value = self.r#max_component_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_component_amount = 0f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "ComponentRestrictions"
    }
}
impl Default for ComponentRestrictions {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Engine.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Engine {
    pub r#position: glam::f32::Vec2,
    pub r#size: f32,
}
impl Engine {
    pub fn new() -> Self {
        Self {
            r#position: Default::default(),
            r#size: Default::default(),
        }
    }
    pub fn with_position(mut self, r#position: impl Into<glam::f32::Vec2>) -> Self {
        self.r#position = r#position.into();
        self
    }
    pub fn set_position(&mut self, r#position: impl Into<glam::f32::Vec2>) -> &mut Self {
        self.r#position = r#position.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
}
impl DatabaseItem for Engine {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (1f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 1f32,
                "Field got truncated"
            );
            self.r#size = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "Engine"
    }
}
impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/InstalledComponent.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct InstalledComponent {
    pub r#component_id: ComponentId,
    pub r#modification: Option<ComponentModId>,
    pub r#quality: ModificationQuality,
    pub r#x: i32,
    pub r#y: i32,
    pub r#barrel_id: i32,
    pub r#behaviour: i32,
    pub r#key_binding: i32,
}
impl InstalledComponent {
    pub fn new(r#component_id: ComponentId) -> Self {
        Self {
            r#component_id,
            r#modification: Default::default(),
            r#quality: Default::default(),
            r#x: Default::default(),
            r#y: Default::default(),
            r#barrel_id: Default::default(),
            r#behaviour: Default::default(),
            r#key_binding: Default::default(),
        }
    }
    pub fn with_component_id(mut self, r#component_id: impl Into<ComponentId>) -> Self {
        self.r#component_id = r#component_id.into();
        self
    }
    pub fn set_component_id(&mut self, r#component_id: impl Into<ComponentId>) -> &mut Self {
        self.r#component_id = r#component_id.into();
        self
    }
    pub fn with_modification(mut self, r#modification: impl Into<Option<ComponentModId>>) -> Self {
        self.r#modification = r#modification.into();
        self
    }
    pub fn set_modification(
        &mut self,
        r#modification: impl Into<Option<ComponentModId>>,
    ) -> &mut Self {
        self.r#modification = r#modification.into();
        self
    }
    pub fn with_quality(mut self, r#quality: impl Into<ModificationQuality>) -> Self {
        self.r#quality = r#quality.into();
        self
    }
    pub fn set_quality(&mut self, r#quality: impl Into<ModificationQuality>) -> &mut Self {
        self.r#quality = r#quality.into();
        self
    }
    pub fn with_x(mut self, r#x: impl Into<i32>) -> Self {
        self.r#x = r#x.into();
        self
    }
    pub fn set_x(&mut self, r#x: impl Into<i32>) -> &mut Self {
        self.r#x = r#x.into();
        self
    }
    pub fn with_y(mut self, r#y: impl Into<i32>) -> Self {
        self.r#y = r#y.into();
        self
    }
    pub fn set_y(&mut self, r#y: impl Into<i32>) -> &mut Self {
        self.r#y = r#y.into();
        self
    }
    pub fn with_barrel_id(mut self, r#barrel_id: impl Into<i32>) -> Self {
        self.r#barrel_id = r#barrel_id.into();
        self
    }
    pub fn set_barrel_id(&mut self, r#barrel_id: impl Into<i32>) -> &mut Self {
        self.r#barrel_id = r#barrel_id.into();
        self
    }
    pub fn with_behaviour(mut self, r#behaviour: impl Into<i32>) -> Self {
        self.r#behaviour = r#behaviour.into();
        self
    }
    pub fn set_behaviour(&mut self, r#behaviour: impl Into<i32>) -> &mut Self {
        self.r#behaviour = r#behaviour.into();
        self
    }
    pub fn with_key_binding(mut self, r#key_binding: impl Into<i32>) -> Self {
        self.r#key_binding = r#key_binding.into();
        self
    }
    pub fn set_key_binding(&mut self, r#key_binding: impl Into<i32>) -> &mut Self {
        self.r#key_binding = r#key_binding.into();
        self
    }
}
impl DatabaseItem for InstalledComponent {
    fn validate(&mut self) {
        if self.r#x < (-32768f32 as i32) {
            tracing::warn!(
                field = "r#x",
                value = self.r#x,
                min = -32768f32,
                "Field got truncated"
            );
            self.r#x = -32768f32 as i32;
        }
        if self.r#x > (32767f32 as i32) {
            tracing::warn!(
                field = "r#x",
                value = self.r#x,
                max = 32767f32,
                "Field got truncated"
            );
            self.r#x = 32767f32 as i32;
        }
        if self.r#y < (-32768f32 as i32) {
            tracing::warn!(
                field = "r#y",
                value = self.r#y,
                min = -32768f32,
                "Field got truncated"
            );
            self.r#y = -32768f32 as i32;
        }
        if self.r#y > (32767f32 as i32) {
            tracing::warn!(
                field = "r#y",
                value = self.r#y,
                max = 32767f32,
                "Field got truncated"
            );
            self.r#y = 32767f32 as i32;
        }
        if self.r#barrel_id < (0f32 as i32) {
            tracing::warn!(
                field = "r#barrel_id",
                value = self.r#barrel_id,
                min = 0f32,
                "Field got truncated"
            );
            self.r#barrel_id = 0f32 as i32;
        }
        if self.r#barrel_id > (255f32 as i32) {
            tracing::warn!(
                field = "r#barrel_id",
                value = self.r#barrel_id,
                max = 255f32,
                "Field got truncated"
            );
            self.r#barrel_id = 255f32 as i32;
        }
        if self.r#behaviour < (0f32 as i32) {
            tracing::warn!(
                field = "r#behaviour",
                value = self.r#behaviour,
                min = 0f32,
                "Field got truncated"
            );
            self.r#behaviour = 0f32 as i32;
        }
        if self.r#behaviour > (10f32 as i32) {
            tracing::warn!(
                field = "r#behaviour",
                value = self.r#behaviour,
                max = 10f32,
                "Field got truncated"
            );
            self.r#behaviour = 10f32 as i32;
        }
        if self.r#key_binding < (-10f32 as i32) {
            tracing::warn!(
                field = "r#key_binding",
                value = self.r#key_binding,
                min = -10f32,
                "Field got truncated"
            );
            self.r#key_binding = -10f32 as i32;
        }
        if self.r#key_binding > (10f32 as i32) {
            tracing::warn!(
                field = "r#key_binding",
                value = self.r#key_binding,
                max = 10f32,
                "Field got truncated"
            );
            self.r#key_binding = 10f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "InstalledComponent"
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/FactionFilter.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionFilter {
    pub r#type: FactionFilterType,
    pub r#list: Vec<FactionId>,
}
impl FactionFilter {
    pub fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#list: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: impl Into<FactionFilterType>) -> Self {
        self.r#type = r#type.into();
        self
    }
    pub fn set_type(&mut self, r#type: impl Into<FactionFilterType>) -> &mut Self {
        self.r#type = r#type.into();
        self
    }
    pub fn with_list(mut self, r#list: impl Into<Vec<FactionId>>) -> Self {
        self.r#list = r#list.into();
        self
    }
    pub fn set_list(&mut self, r#list: impl Into<Vec<FactionId>>) -> &mut Self {
        self.r#list = r#list.into();
        self
    }
}
impl DatabaseItem for FactionFilter {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "FactionFilter"
    }
}
impl Default for FactionFilter {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/LootContent.xml
#[derive(Debug, Clone)]
pub enum LootContent {
    None(LootContentNone),
    SomeMoney(LootContentSomeMoney),
    Fuel(LootContentFuel),
    Money(LootContentMoney),
    Stars(LootContentStars),
    StarMap(LootContentStarMap),
    RandomComponents(LootContentRandomComponents),
    RandomItems(LootContentRandomItems),
    AllItems(LootContentAllItems),
    ItemsWithChance(LootContentItemsWithChance),
    QuestItem(LootContentQuestItem),
    Ship(LootContentShip),
    EmptyShip(LootContentEmptyShip),
    Component(LootContentComponent),
    Blueprint(LootContentBlueprint),
    ResearchPoints(LootContentResearchPoints),
    Satellite(LootContentSatellite),
}
impl Default for LootContent {
    fn default() -> Self {
        Self::None(Default::default())
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentNone {}
impl LootContentNone {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for LootContentNone {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootContentNone"
    }
}
impl Default for LootContentNone {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentNone> for LootContent {
    fn from(item: LootContentNone) -> Self {
        Self::None(item)
    }
}
impl LootContentNone {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn none() -> LootContentNone {
        LootContentNone::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentSomeMoney {
    pub r#value_ratio: f32,
}
impl LootContentSomeMoney {
    pub fn new() -> Self {
        Self {
            r#value_ratio: Default::default(),
        }
    }
    pub fn with_value_ratio(mut self, r#value_ratio: impl Into<f32>) -> Self {
        self.r#value_ratio = r#value_ratio.into();
        self
    }
    pub fn set_value_ratio(&mut self, r#value_ratio: impl Into<f32>) -> &mut Self {
        self.r#value_ratio = r#value_ratio.into();
        self
    }
}
impl DatabaseItem for LootContentSomeMoney {
    fn validate(&mut self) {
        if self.r#value_ratio < (0.001f32 as f32) {
            tracing::warn!(
                field = "r#value_ratio",
                value = self.r#value_ratio,
                min = 0.001f32,
                "Field got truncated"
            );
            self.r#value_ratio = 0.001f32 as f32;
        }
        if self.r#value_ratio > (1000f32 as f32) {
            tracing::warn!(
                field = "r#value_ratio",
                value = self.r#value_ratio,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#value_ratio = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentSomeMoney"
    }
}
impl Default for LootContentSomeMoney {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentSomeMoney> for LootContent {
    fn from(item: LootContentSomeMoney) -> Self {
        Self::SomeMoney(item)
    }
}
impl LootContentSomeMoney {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn some_money() -> LootContentSomeMoney {
        LootContentSomeMoney::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentFuel {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentFuel {
    pub fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
}
impl DatabaseItem for LootContentFuel {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentFuel"
    }
}
impl Default for LootContentFuel {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentFuel> for LootContent {
    fn from(item: LootContentFuel) -> Self {
        Self::Fuel(item)
    }
}
impl LootContentFuel {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn fuel() -> LootContentFuel {
        LootContentFuel::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentMoney {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentMoney {
    pub fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
}
impl DatabaseItem for LootContentMoney {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentMoney"
    }
}
impl Default for LootContentMoney {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentMoney> for LootContent {
    fn from(item: LootContentMoney) -> Self {
        Self::Money(item)
    }
}
impl LootContentMoney {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn money() -> LootContentMoney {
        LootContentMoney::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentStars {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentStars {
    pub fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
}
impl DatabaseItem for LootContentStars {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentStars"
    }
}
impl Default for LootContentStars {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentStars> for LootContent {
    fn from(item: LootContentStars) -> Self {
        Self::Stars(item)
    }
}
impl LootContentStars {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn stars() -> LootContentStars {
        LootContentStars::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentStarMap {}
impl LootContentStarMap {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for LootContentStarMap {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootContentStarMap"
    }
}
impl Default for LootContentStarMap {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentStarMap> for LootContent {
    fn from(item: LootContentStarMap) -> Self {
        Self::StarMap(item)
    }
}
impl LootContentStarMap {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn star_map() -> LootContentStarMap {
        LootContentStarMap::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentRandomComponents {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
    pub r#value_ratio: f32,
    pub r#factions: FactionFilter,
}
impl LootContentRandomComponents {
    pub fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
            r#value_ratio: Default::default(),
            r#factions: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn with_value_ratio(mut self, r#value_ratio: impl Into<f32>) -> Self {
        self.r#value_ratio = r#value_ratio.into();
        self
    }
    pub fn set_value_ratio(&mut self, r#value_ratio: impl Into<f32>) -> &mut Self {
        self.r#value_ratio = r#value_ratio.into();
        self
    }
    pub fn with_factions(mut self, r#factions: impl Into<FactionFilter>) -> Self {
        self.r#factions = r#factions.into();
        self
    }
    pub fn set_factions(&mut self, r#factions: impl Into<FactionFilter>) -> &mut Self {
        self.r#factions = r#factions.into();
        self
    }
}
impl DatabaseItem for LootContentRandomComponents {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
        if self.r#value_ratio < (0.001f32 as f32) {
            tracing::warn!(
                field = "r#value_ratio",
                value = self.r#value_ratio,
                min = 0.001f32,
                "Field got truncated"
            );
            self.r#value_ratio = 0.001f32 as f32;
        }
        if self.r#value_ratio > (1000f32 as f32) {
            tracing::warn!(
                field = "r#value_ratio",
                value = self.r#value_ratio,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#value_ratio = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentRandomComponents"
    }
}
impl Default for LootContentRandomComponents {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentRandomComponents> for LootContent {
    fn from(item: LootContentRandomComponents) -> Self {
        Self::RandomComponents(item)
    }
}
impl LootContentRandomComponents {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn random_components() -> LootContentRandomComponents {
        LootContentRandomComponents::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentRandomItems {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
    pub r#items: Vec<LootItem>,
}
impl LootContentRandomItems {
    pub fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
            r#items: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn with_items(mut self, r#items: impl Into<Vec<LootItem>>) -> Self {
        self.r#items = r#items.into();
        self
    }
    pub fn set_items(&mut self, r#items: impl Into<Vec<LootItem>>) -> &mut Self {
        self.r#items = r#items.into();
        self
    }
}
impl DatabaseItem for LootContentRandomItems {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentRandomItems"
    }
}
impl Default for LootContentRandomItems {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentRandomItems> for LootContent {
    fn from(item: LootContentRandomItems) -> Self {
        Self::RandomItems(item)
    }
}
impl LootContentRandomItems {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn random_items() -> LootContentRandomItems {
        LootContentRandomItems::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentAllItems {
    pub r#items: Vec<LootItem>,
}
impl LootContentAllItems {
    pub fn new() -> Self {
        Self {
            r#items: Default::default(),
        }
    }
    pub fn with_items(mut self, r#items: impl Into<Vec<LootItem>>) -> Self {
        self.r#items = r#items.into();
        self
    }
    pub fn set_items(&mut self, r#items: impl Into<Vec<LootItem>>) -> &mut Self {
        self.r#items = r#items.into();
        self
    }
}
impl DatabaseItem for LootContentAllItems {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootContentAllItems"
    }
}
impl Default for LootContentAllItems {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentAllItems> for LootContent {
    fn from(item: LootContentAllItems) -> Self {
        Self::AllItems(item)
    }
}
impl LootContentAllItems {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn all_items() -> LootContentAllItems {
        LootContentAllItems::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentItemsWithChance {
    pub r#items: Vec<LootItem>,
}
impl LootContentItemsWithChance {
    pub fn new() -> Self {
        Self {
            r#items: Default::default(),
        }
    }
    pub fn with_items(mut self, r#items: impl Into<Vec<LootItem>>) -> Self {
        self.r#items = r#items.into();
        self
    }
    pub fn set_items(&mut self, r#items: impl Into<Vec<LootItem>>) -> &mut Self {
        self.r#items = r#items.into();
        self
    }
}
impl DatabaseItem for LootContentItemsWithChance {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootContentItemsWithChance"
    }
}
impl Default for LootContentItemsWithChance {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentItemsWithChance> for LootContent {
    fn from(item: LootContentItemsWithChance) -> Self {
        Self::ItemsWithChance(item)
    }
}
impl LootContentItemsWithChance {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn items_with_chance() -> LootContentItemsWithChance {
        LootContentItemsWithChance::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentQuestItem {
    pub r#item_id: QuestItemId,
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentQuestItem {
    pub fn new(r#item_id: QuestItemId) -> Self {
        Self {
            r#item_id,
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<QuestItemId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<QuestItemId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
}
impl DatabaseItem for LootContentQuestItem {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentQuestItem"
    }
}
impl From<LootContentQuestItem> for LootContent {
    fn from(item: LootContentQuestItem) -> Self {
        Self::QuestItem(item)
    }
}
impl LootContentQuestItem {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn quest_item(r#item_id: QuestItemId) -> LootContentQuestItem {
        LootContentQuestItem::new(r#item_id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentShip {
    pub r#item_id: ShipBuildId,
}
impl LootContentShip {
    pub fn new(r#item_id: ShipBuildId) -> Self {
        Self { r#item_id }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<ShipBuildId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<ShipBuildId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
}
impl DatabaseItem for LootContentShip {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootContentShip"
    }
}
impl From<LootContentShip> for LootContent {
    fn from(item: LootContentShip) -> Self {
        Self::Ship(item)
    }
}
impl LootContentShip {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn ship(r#item_id: ShipBuildId) -> LootContentShip {
        LootContentShip::new(r#item_id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentEmptyShip {
    pub r#item_id: ShipId,
}
impl LootContentEmptyShip {
    pub fn new(r#item_id: ShipId) -> Self {
        Self { r#item_id }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<ShipId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<ShipId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
}
impl DatabaseItem for LootContentEmptyShip {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootContentEmptyShip"
    }
}
impl From<LootContentEmptyShip> for LootContent {
    fn from(item: LootContentEmptyShip) -> Self {
        Self::EmptyShip(item)
    }
}
impl LootContentEmptyShip {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn empty_ship(r#item_id: ShipId) -> LootContentEmptyShip {
        LootContentEmptyShip::new(r#item_id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentComponent {
    pub r#item_id: ComponentId,
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentComponent {
    pub fn new(r#item_id: ComponentId) -> Self {
        Self {
            r#item_id,
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<ComponentId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<ComponentId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
}
impl DatabaseItem for LootContentComponent {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentComponent"
    }
}
impl From<LootContentComponent> for LootContent {
    fn from(item: LootContentComponent) -> Self {
        Self::Component(item)
    }
}
impl LootContentComponent {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn component(r#item_id: ComponentId) -> LootContentComponent {
        LootContentComponent::new(r#item_id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentBlueprint {
    pub r#item_id: TechnologyId,
}
impl LootContentBlueprint {
    pub fn new(r#item_id: TechnologyId) -> Self {
        Self { r#item_id }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<TechnologyId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<TechnologyId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
}
impl DatabaseItem for LootContentBlueprint {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootContentBlueprint"
    }
}
impl From<LootContentBlueprint> for LootContent {
    fn from(item: LootContentBlueprint) -> Self {
        Self::Blueprint(item)
    }
}
impl LootContentBlueprint {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn blueprint(r#item_id: TechnologyId) -> LootContentBlueprint {
        LootContentBlueprint::new(r#item_id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentResearchPoints {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
    pub r#factions: FactionFilter,
}
impl LootContentResearchPoints {
    pub fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
            r#factions: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn with_factions(mut self, r#factions: impl Into<FactionFilter>) -> Self {
        self.r#factions = r#factions.into();
        self
    }
    pub fn set_factions(&mut self, r#factions: impl Into<FactionFilter>) -> &mut Self {
        self.r#factions = r#factions.into();
        self
    }
}
impl DatabaseItem for LootContentResearchPoints {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentResearchPoints"
    }
}
impl Default for LootContentResearchPoints {
    fn default() -> Self {
        Self::new()
    }
}
impl From<LootContentResearchPoints> for LootContent {
    fn from(item: LootContentResearchPoints) -> Self {
        Self::ResearchPoints(item)
    }
}
impl LootContentResearchPoints {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn research_points() -> LootContentResearchPoints {
        LootContentResearchPoints::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootContentSatellite {
    pub r#item_id: SatelliteId,
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentSatellite {
    pub fn new(r#item_id: SatelliteId) -> Self {
        Self {
            r#item_id,
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<SatelliteId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<SatelliteId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn with_min_amount(mut self, r#min_amount: impl Into<i32>) -> Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn set_min_amount(&mut self, r#min_amount: impl Into<i32>) -> &mut Self {
        self.r#min_amount = r#min_amount.into();
        self
    }
    pub fn with_max_amount(mut self, r#max_amount: impl Into<i32>) -> Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
    pub fn set_max_amount(&mut self, r#max_amount: impl Into<i32>) -> &mut Self {
        self.r#max_amount = r#max_amount.into();
        self
    }
}
impl DatabaseItem for LootContentSatellite {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_amount = 0f32 as i32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#min_amount",
                value = self.r#min_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#min_amount = 1000000000f32 as i32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_amount = 0f32 as i32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#max_amount",
                value = self.r#max_amount,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#max_amount = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "LootContentSatellite"
    }
}
impl From<LootContentSatellite> for LootContent {
    fn from(item: LootContentSatellite) -> Self {
        Self::Satellite(item)
    }
}
impl LootContentSatellite {
    pub fn wrap(self) -> LootContent {
        self.into()
    }
}
impl LootContent {
    pub fn satellite(r#item_id: SatelliteId) -> LootContentSatellite {
        LootContentSatellite::new(r#item_id)
    }
}
impl serde::Serialize for LootContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "LootContent")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: LootItemType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::None(x) => AdjTagged {
                t: LootItemType::None,
                c: x,
            }
            .serialize(serializer),
            Self::SomeMoney(x) => AdjTagged {
                t: LootItemType::SomeMoney,
                c: x,
            }
            .serialize(serializer),
            Self::Fuel(x) => AdjTagged {
                t: LootItemType::Fuel,
                c: x,
            }
            .serialize(serializer),
            Self::Money(x) => AdjTagged {
                t: LootItemType::Money,
                c: x,
            }
            .serialize(serializer),
            Self::Stars(x) => AdjTagged {
                t: LootItemType::Stars,
                c: x,
            }
            .serialize(serializer),
            Self::StarMap(x) => AdjTagged {
                t: LootItemType::StarMap,
                c: x,
            }
            .serialize(serializer),
            Self::RandomComponents(x) => AdjTagged {
                t: LootItemType::RandomComponents,
                c: x,
            }
            .serialize(serializer),
            Self::RandomItems(x) => AdjTagged {
                t: LootItemType::RandomItems,
                c: x,
            }
            .serialize(serializer),
            Self::AllItems(x) => AdjTagged {
                t: LootItemType::AllItems,
                c: x,
            }
            .serialize(serializer),
            Self::ItemsWithChance(x) => AdjTagged {
                t: LootItemType::ItemsWithChance,
                c: x,
            }
            .serialize(serializer),
            Self::QuestItem(x) => AdjTagged {
                t: LootItemType::QuestItem,
                c: x,
            }
            .serialize(serializer),
            Self::Ship(x) => AdjTagged {
                t: LootItemType::Ship,
                c: x,
            }
            .serialize(serializer),
            Self::EmptyShip(x) => AdjTagged {
                t: LootItemType::EmptyShip,
                c: x,
            }
            .serialize(serializer),
            Self::Component(x) => AdjTagged {
                t: LootItemType::Component,
                c: x,
            }
            .serialize(serializer),
            Self::Blueprint(x) => AdjTagged {
                t: LootItemType::Blueprint,
                c: x,
            }
            .serialize(serializer),
            Self::ResearchPoints(x) => AdjTagged {
                t: LootItemType::ResearchPoints,
                c: x,
            }
            .serialize(serializer),
            Self::Satellite(x) => AdjTagged {
                t: LootItemType::Satellite,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl DatabaseItem for LootContent {
    fn validate(&mut self) {
        match self {
            Self::None(x) => x.validate(),
            Self::SomeMoney(x) => x.validate(),
            Self::Fuel(x) => x.validate(),
            Self::Money(x) => x.validate(),
            Self::Stars(x) => x.validate(),
            Self::StarMap(x) => x.validate(),
            Self::RandomComponents(x) => x.validate(),
            Self::RandomItems(x) => x.validate(),
            Self::AllItems(x) => x.validate(),
            Self::ItemsWithChance(x) => x.validate(),
            Self::QuestItem(x) => x.validate(),
            Self::Ship(x) => x.validate(),
            Self::EmptyShip(x) => x.validate(),
            Self::Component(x) => x.validate(),
            Self::Blueprint(x) => x.validate(),
            Self::ResearchPoints(x) => x.validate(),
            Self::Satellite(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "LootContent"
    }
}
impl LootContent {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::None(_) => LootContentNone::type_name(),
            Self::SomeMoney(_) => LootContentSomeMoney::type_name(),
            Self::Fuel(_) => LootContentFuel::type_name(),
            Self::Money(_) => LootContentMoney::type_name(),
            Self::Stars(_) => LootContentStars::type_name(),
            Self::StarMap(_) => LootContentStarMap::type_name(),
            Self::RandomComponents(_) => LootContentRandomComponents::type_name(),
            Self::RandomItems(_) => LootContentRandomItems::type_name(),
            Self::AllItems(_) => LootContentAllItems::type_name(),
            Self::ItemsWithChance(_) => LootContentItemsWithChance::type_name(),
            Self::QuestItem(_) => LootContentQuestItem::type_name(),
            Self::Ship(_) => LootContentShip::type_name(),
            Self::EmptyShip(_) => LootContentEmptyShip::type_name(),
            Self::Component(_) => LootContentComponent::type_name(),
            Self::Blueprint(_) => LootContentBlueprint::type_name(),
            Self::ResearchPoints(_) => LootContentResearchPoints::type_name(),
            Self::Satellite(_) => LootContentSatellite::type_name(),
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/LootItem.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootItem {
    pub r#weight: f32,
    pub r#loot: LootContent,
}
impl LootItem {
    pub fn new() -> Self {
        Self {
            r#weight: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_weight(mut self, r#weight: impl Into<f32>) -> Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn set_weight(&mut self, r#weight: impl Into<f32>) -> &mut Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<LootContent>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<LootContent>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for LootItem {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "LootItem"
    }
}
impl Default for LootItem {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Node.xml
#[derive(Debug, Clone)]
pub enum Node {
    Undefined(NodeUndefined),
    ComingSoon(NodeComingSoon),
    ShowDialog(NodeShowDialog),
    OpenShipyard(NodeOpenShipyard),
    OpenWorkshop(NodeOpenWorkshop),
    Switch(NodeSwitch),
    Random(NodeRandom),
    Condition(NodeCondition),
    AttackFleet(NodeAttackFleet),
    AttackOccupants(NodeAttackOccupants),
    AttackStarbase(NodeAttackStarbase),
    DestroyOccupants(NodeDestroyOccupants),
    SuppressOccupants(NodeSuppressOccupants),
    Retreat(NodeRetreat),
    ReceiveItem(NodeReceiveItem),
    RemoveItem(NodeRemoveItem),
    Trade(NodeTrade),
    CompleteQuest(NodeCompleteQuest),
    FailQuest(NodeFailQuest),
    CancelQuest(NodeCancelQuest),
    StartQuest(NodeStartQuest),
    SetCharacterRelations(NodeSetCharacterRelations),
    SetFactionRelations(NodeSetFactionRelations),
    SetFactionStarbasePower(NodeSetFactionStarbasePower),
    ChangeCharacterRelations(NodeChangeCharacterRelations),
    ChangeFactionRelations(NodeChangeFactionRelations),
    ChangeFactionStarbasePower(NodeChangeFactionStarbasePower),
    CaptureStarBase(NodeCaptureStarBase),
    LiberateStarBase(NodeLiberateStarBase),
    ChangeFaction(NodeChangeFaction),
}
impl Default for Node {
    fn default() -> Self {
        Self::Undefined(Default::default())
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeUndefined {
    pub r#id: i32,
}
impl NodeUndefined {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
}
impl DatabaseItem for NodeUndefined {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeUndefined"
    }
}
impl Default for NodeUndefined {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeUndefined> for Node {
    fn from(item: NodeUndefined) -> Self {
        Self::Undefined(item)
    }
}
impl NodeUndefined {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn undefined() -> NodeUndefined {
        NodeUndefined::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeComingSoon {
    pub r#id: i32,
}
impl NodeComingSoon {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
}
impl DatabaseItem for NodeComingSoon {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeComingSoon"
    }
}
impl Default for NodeComingSoon {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeComingSoon> for Node {
    fn from(item: NodeComingSoon) -> Self {
        Self::ComingSoon(item)
    }
}
impl NodeComingSoon {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn coming_soon() -> NodeComingSoon {
        NodeComingSoon::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeShowDialog {
    pub r#id: i32,
    pub r#required_view: RequiredViewMode,
    pub r#message: String,
    pub r#enemy: Option<FleetId>,
    pub r#loot: Option<LootId>,
    pub r#character: Option<CharacterId>,
    pub r#actions: Vec<NodeAction>,
}
impl NodeShowDialog {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#required_view: Default::default(),
            r#message: Default::default(),
            r#enemy: Default::default(),
            r#loot: Default::default(),
            r#character: Default::default(),
            r#actions: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_required_view(mut self, r#required_view: impl Into<RequiredViewMode>) -> Self {
        self.r#required_view = r#required_view.into();
        self
    }
    pub fn set_required_view(&mut self, r#required_view: impl Into<RequiredViewMode>) -> &mut Self {
        self.r#required_view = r#required_view.into();
        self
    }
    pub fn with_message(mut self, r#message: impl Into<String>) -> Self {
        self.r#message = r#message.into();
        self
    }
    pub fn set_message(&mut self, r#message: impl Into<String>) -> &mut Self {
        self.r#message = r#message.into();
        self
    }
    pub fn with_enemy(mut self, r#enemy: impl Into<Option<FleetId>>) -> Self {
        self.r#enemy = r#enemy.into();
        self
    }
    pub fn set_enemy(&mut self, r#enemy: impl Into<Option<FleetId>>) -> &mut Self {
        self.r#enemy = r#enemy.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<Option<LootId>>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<Option<LootId>>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn with_character(mut self, r#character: impl Into<Option<CharacterId>>) -> Self {
        self.r#character = r#character.into();
        self
    }
    pub fn set_character(&mut self, r#character: impl Into<Option<CharacterId>>) -> &mut Self {
        self.r#character = r#character.into();
        self
    }
    pub fn with_actions(mut self, r#actions: impl Into<Vec<NodeAction>>) -> Self {
        self.r#actions = r#actions.into();
        self
    }
    pub fn set_actions(&mut self, r#actions: impl Into<Vec<NodeAction>>) -> &mut Self {
        self.r#actions = r#actions.into();
        self
    }
}
impl DatabaseItem for NodeShowDialog {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeShowDialog"
    }
}
impl Default for NodeShowDialog {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeShowDialog> for Node {
    fn from(item: NodeShowDialog) -> Self {
        Self::ShowDialog(item)
    }
}
impl NodeShowDialog {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn show_dialog() -> NodeShowDialog {
        NodeShowDialog::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeOpenShipyard {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#faction: Option<FactionId>,
    pub r#value: i32,
}
impl NodeOpenShipyard {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#faction: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeOpenShipyard {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as i32;
        }
        if self.r#value > (10000f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#value = 10000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeOpenShipyard"
    }
}
impl Default for NodeOpenShipyard {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeOpenShipyard> for Node {
    fn from(item: NodeOpenShipyard) -> Self {
        Self::OpenShipyard(item)
    }
}
impl NodeOpenShipyard {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn open_shipyard() -> NodeOpenShipyard {
        NodeOpenShipyard::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeOpenWorkshop {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#faction: Option<FactionId>,
    pub r#value: i32,
}
impl NodeOpenWorkshop {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#faction: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeOpenWorkshop {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as i32;
        }
        if self.r#value > (10000f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#value = 10000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeOpenWorkshop"
    }
}
impl Default for NodeOpenWorkshop {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeOpenWorkshop> for Node {
    fn from(item: NodeOpenWorkshop) -> Self {
        Self::OpenWorkshop(item)
    }
}
impl NodeOpenWorkshop {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn open_workshop() -> NodeOpenWorkshop {
        NodeOpenWorkshop::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSwitch {
    pub r#id: i32,
    pub r#message: String,
    pub r#default_transition: i32,
    pub r#transitions: Vec<NodeTransition>,
}
impl NodeSwitch {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#message: Default::default(),
            r#default_transition: Default::default(),
            r#transitions: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_message(mut self, r#message: impl Into<String>) -> Self {
        self.r#message = r#message.into();
        self
    }
    pub fn set_message(&mut self, r#message: impl Into<String>) -> &mut Self {
        self.r#message = r#message.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_transitions(mut self, r#transitions: impl Into<Vec<NodeTransition>>) -> Self {
        self.r#transitions = r#transitions.into();
        self
    }
    pub fn set_transitions(&mut self, r#transitions: impl Into<Vec<NodeTransition>>) -> &mut Self {
        self.r#transitions = r#transitions.into();
        self
    }
}
impl DatabaseItem for NodeSwitch {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (0f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 0f32,
                "Field got truncated"
            );
            self.r#default_transition = 0f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeSwitch"
    }
}
impl Default for NodeSwitch {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeSwitch> for Node {
    fn from(item: NodeSwitch) -> Self {
        Self::Switch(item)
    }
}
impl NodeSwitch {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn switch() -> NodeSwitch {
        NodeSwitch::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeRandom {
    pub r#id: i32,
    pub r#message: String,
    pub r#default_transition: i32,
    pub r#transitions: Vec<NodeTransition>,
}
impl NodeRandom {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#message: Default::default(),
            r#default_transition: Default::default(),
            r#transitions: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_message(mut self, r#message: impl Into<String>) -> Self {
        self.r#message = r#message.into();
        self
    }
    pub fn set_message(&mut self, r#message: impl Into<String>) -> &mut Self {
        self.r#message = r#message.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_transitions(mut self, r#transitions: impl Into<Vec<NodeTransition>>) -> Self {
        self.r#transitions = r#transitions.into();
        self
    }
    pub fn set_transitions(&mut self, r#transitions: impl Into<Vec<NodeTransition>>) -> &mut Self {
        self.r#transitions = r#transitions.into();
        self
    }
}
impl DatabaseItem for NodeRandom {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (0f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 0f32,
                "Field got truncated"
            );
            self.r#default_transition = 0f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeRandom"
    }
}
impl Default for NodeRandom {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeRandom> for Node {
    fn from(item: NodeRandom) -> Self {
        Self::Random(item)
    }
}
impl NodeRandom {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn random() -> NodeRandom {
        NodeRandom::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeCondition {
    pub r#id: i32,
    pub r#message: String,
    pub r#transitions: Vec<NodeTransition>,
}
impl NodeCondition {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#message: Default::default(),
            r#transitions: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_message(mut self, r#message: impl Into<String>) -> Self {
        self.r#message = r#message.into();
        self
    }
    pub fn set_message(&mut self, r#message: impl Into<String>) -> &mut Self {
        self.r#message = r#message.into();
        self
    }
    pub fn with_transitions(mut self, r#transitions: impl Into<Vec<NodeTransition>>) -> Self {
        self.r#transitions = r#transitions.into();
        self
    }
    pub fn set_transitions(&mut self, r#transitions: impl Into<Vec<NodeTransition>>) -> &mut Self {
        self.r#transitions = r#transitions.into();
        self
    }
}
impl DatabaseItem for NodeCondition {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeCondition"
    }
}
impl Default for NodeCondition {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeCondition> for Node {
    fn from(item: NodeCondition) -> Self {
        Self::Condition(item)
    }
}
impl NodeCondition {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn condition() -> NodeCondition {
        NodeCondition::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeAttackFleet {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#failure_transition: i32,
    pub r#enemy: Option<FleetId>,
    pub r#loot: Option<LootId>,
}
impl NodeAttackFleet {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#failure_transition: Default::default(),
            r#enemy: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_failure_transition(mut self, r#failure_transition: impl Into<i32>) -> Self {
        self.r#failure_transition = r#failure_transition.into();
        self
    }
    pub fn set_failure_transition(&mut self, r#failure_transition: impl Into<i32>) -> &mut Self {
        self.r#failure_transition = r#failure_transition.into();
        self
    }
    pub fn with_enemy(mut self, r#enemy: impl Into<Option<FleetId>>) -> Self {
        self.r#enemy = r#enemy.into();
        self
    }
    pub fn set_enemy(&mut self, r#enemy: impl Into<Option<FleetId>>) -> &mut Self {
        self.r#enemy = r#enemy.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<Option<LootId>>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<Option<LootId>>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for NodeAttackFleet {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#failure_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#failure_transition",
                value = self.r#failure_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#failure_transition = 1f32 as i32;
        }
        if self.r#failure_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#failure_transition",
                value = self.r#failure_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#failure_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeAttackFleet"
    }
}
impl Default for NodeAttackFleet {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeAttackFleet> for Node {
    fn from(item: NodeAttackFleet) -> Self {
        Self::AttackFleet(item)
    }
}
impl NodeAttackFleet {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn attack_fleet() -> NodeAttackFleet {
        NodeAttackFleet::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeAttackOccupants {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#failure_transition: i32,
}
impl NodeAttackOccupants {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#failure_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_failure_transition(mut self, r#failure_transition: impl Into<i32>) -> Self {
        self.r#failure_transition = r#failure_transition.into();
        self
    }
    pub fn set_failure_transition(&mut self, r#failure_transition: impl Into<i32>) -> &mut Self {
        self.r#failure_transition = r#failure_transition.into();
        self
    }
}
impl DatabaseItem for NodeAttackOccupants {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#failure_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#failure_transition",
                value = self.r#failure_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#failure_transition = 1f32 as i32;
        }
        if self.r#failure_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#failure_transition",
                value = self.r#failure_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#failure_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeAttackOccupants"
    }
}
impl Default for NodeAttackOccupants {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeAttackOccupants> for Node {
    fn from(item: NodeAttackOccupants) -> Self {
        Self::AttackOccupants(item)
    }
}
impl NodeAttackOccupants {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn attack_occupants() -> NodeAttackOccupants {
        NodeAttackOccupants::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeAttackStarbase {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#failure_transition: i32,
}
impl NodeAttackStarbase {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#failure_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_failure_transition(mut self, r#failure_transition: impl Into<i32>) -> Self {
        self.r#failure_transition = r#failure_transition.into();
        self
    }
    pub fn set_failure_transition(&mut self, r#failure_transition: impl Into<i32>) -> &mut Self {
        self.r#failure_transition = r#failure_transition.into();
        self
    }
}
impl DatabaseItem for NodeAttackStarbase {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#failure_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#failure_transition",
                value = self.r#failure_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#failure_transition = 1f32 as i32;
        }
        if self.r#failure_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#failure_transition",
                value = self.r#failure_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#failure_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeAttackStarbase"
    }
}
impl Default for NodeAttackStarbase {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeAttackStarbase> for Node {
    fn from(item: NodeAttackStarbase) -> Self {
        Self::AttackStarbase(item)
    }
}
impl NodeAttackStarbase {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn attack_starbase() -> NodeAttackStarbase {
        NodeAttackStarbase::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDestroyOccupants {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeDestroyOccupants {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
}
impl DatabaseItem for NodeDestroyOccupants {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeDestroyOccupants"
    }
}
impl Default for NodeDestroyOccupants {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeDestroyOccupants> for Node {
    fn from(item: NodeDestroyOccupants) -> Self {
        Self::DestroyOccupants(item)
    }
}
impl NodeDestroyOccupants {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn destroy_occupants() -> NodeDestroyOccupants {
        NodeDestroyOccupants::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSuppressOccupants {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeSuppressOccupants {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
}
impl DatabaseItem for NodeSuppressOccupants {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeSuppressOccupants"
    }
}
impl Default for NodeSuppressOccupants {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeSuppressOccupants> for Node {
    fn from(item: NodeSuppressOccupants) -> Self {
        Self::SuppressOccupants(item)
    }
}
impl NodeSuppressOccupants {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn suppress_occupants() -> NodeSuppressOccupants {
        NodeSuppressOccupants::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeRetreat {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeRetreat {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
}
impl DatabaseItem for NodeRetreat {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeRetreat"
    }
}
impl Default for NodeRetreat {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeRetreat> for Node {
    fn from(item: NodeRetreat) -> Self {
        Self::Retreat(item)
    }
}
impl NodeRetreat {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn retreat() -> NodeRetreat {
        NodeRetreat::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeReceiveItem {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#loot: Option<LootId>,
}
impl NodeReceiveItem {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<Option<LootId>>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<Option<LootId>>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for NodeReceiveItem {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeReceiveItem"
    }
}
impl Default for NodeReceiveItem {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeReceiveItem> for Node {
    fn from(item: NodeReceiveItem) -> Self {
        Self::ReceiveItem(item)
    }
}
impl NodeReceiveItem {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn receive_item() -> NodeReceiveItem {
        NodeReceiveItem::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeRemoveItem {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#loot: Option<LootId>,
}
impl NodeRemoveItem {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<Option<LootId>>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<Option<LootId>>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for NodeRemoveItem {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeRemoveItem"
    }
}
impl Default for NodeRemoveItem {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeRemoveItem> for Node {
    fn from(item: NodeRemoveItem) -> Self {
        Self::RemoveItem(item)
    }
}
impl NodeRemoveItem {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn remove_item() -> NodeRemoveItem {
        NodeRemoveItem::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeTrade {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#loot: Option<LootId>,
}
impl NodeTrade {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<Option<LootId>>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<Option<LootId>>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for NodeTrade {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeTrade"
    }
}
impl Default for NodeTrade {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeTrade> for Node {
    fn from(item: NodeTrade) -> Self {
        Self::Trade(item)
    }
}
impl NodeTrade {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn trade() -> NodeTrade {
        NodeTrade::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeCompleteQuest {
    pub r#id: i32,
}
impl NodeCompleteQuest {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
}
impl DatabaseItem for NodeCompleteQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeCompleteQuest"
    }
}
impl Default for NodeCompleteQuest {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeCompleteQuest> for Node {
    fn from(item: NodeCompleteQuest) -> Self {
        Self::CompleteQuest(item)
    }
}
impl NodeCompleteQuest {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn complete_quest() -> NodeCompleteQuest {
        NodeCompleteQuest::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeFailQuest {
    pub r#id: i32,
}
impl NodeFailQuest {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
}
impl DatabaseItem for NodeFailQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeFailQuest"
    }
}
impl Default for NodeFailQuest {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeFailQuest> for Node {
    fn from(item: NodeFailQuest) -> Self {
        Self::FailQuest(item)
    }
}
impl NodeFailQuest {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn fail_quest() -> NodeFailQuest {
        NodeFailQuest::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeCancelQuest {
    pub r#id: i32,
}
impl NodeCancelQuest {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
}
impl DatabaseItem for NodeCancelQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeCancelQuest"
    }
}
impl Default for NodeCancelQuest {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeCancelQuest> for Node {
    fn from(item: NodeCancelQuest) -> Self {
        Self::CancelQuest(item)
    }
}
impl NodeCancelQuest {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn cancel_quest() -> NodeCancelQuest {
        NodeCancelQuest::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStartQuest {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#quest: Option<QuestId>,
}
impl NodeStartQuest {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#quest: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_quest(mut self, r#quest: impl Into<Option<QuestId>>) -> Self {
        self.r#quest = r#quest.into();
        self
    }
    pub fn set_quest(&mut self, r#quest: impl Into<Option<QuestId>>) -> &mut Self {
        self.r#quest = r#quest.into();
        self
    }
}
impl DatabaseItem for NodeStartQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeStartQuest"
    }
}
impl Default for NodeStartQuest {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeStartQuest> for Node {
    fn from(item: NodeStartQuest) -> Self {
        Self::StartQuest(item)
    }
}
impl NodeStartQuest {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn start_quest() -> NodeStartQuest {
        NodeStartQuest::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSetCharacterRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#character: Option<CharacterId>,
    pub r#value: i32,
}
impl NodeSetCharacterRelations {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#character: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_character(mut self, r#character: impl Into<Option<CharacterId>>) -> Self {
        self.r#character = r#character.into();
        self
    }
    pub fn set_character(&mut self, r#character: impl Into<Option<CharacterId>>) -> &mut Self {
        self.r#character = r#character.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeSetCharacterRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#value = -100f32 as i32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#value = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeSetCharacterRelations"
    }
}
impl Default for NodeSetCharacterRelations {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeSetCharacterRelations> for Node {
    fn from(item: NodeSetCharacterRelations) -> Self {
        Self::SetCharacterRelations(item)
    }
}
impl NodeSetCharacterRelations {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn set_character_relations() -> NodeSetCharacterRelations {
        NodeSetCharacterRelations::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSetFactionRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#value: i32,
}
impl NodeSetFactionRelations {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeSetFactionRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#value = -100f32 as i32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#value = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeSetFactionRelations"
    }
}
impl Default for NodeSetFactionRelations {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeSetFactionRelations> for Node {
    fn from(item: NodeSetFactionRelations) -> Self {
        Self::SetFactionRelations(item)
    }
}
impl NodeSetFactionRelations {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn set_faction_relations() -> NodeSetFactionRelations {
        NodeSetFactionRelations::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSetFactionStarbasePower {
    pub r#id: i32,
    pub r#default_transition: i32,
    ///Percentage value
    pub r#value: i32,
}
impl NodeSetFactionStarbasePower {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeSetFactionStarbasePower {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as i32;
        }
        if self.r#value > (100000f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 100000f32,
                "Field got truncated"
            );
            self.r#value = 100000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeSetFactionStarbasePower"
    }
}
impl Default for NodeSetFactionStarbasePower {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeSetFactionStarbasePower> for Node {
    fn from(item: NodeSetFactionStarbasePower) -> Self {
        Self::SetFactionStarbasePower(item)
    }
}
impl NodeSetFactionStarbasePower {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn set_faction_starbase_power() -> NodeSetFactionStarbasePower {
        NodeSetFactionStarbasePower::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeChangeCharacterRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#character: Option<CharacterId>,
    pub r#value: i32,
}
impl NodeChangeCharacterRelations {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#character: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_character(mut self, r#character: impl Into<Option<CharacterId>>) -> Self {
        self.r#character = r#character.into();
        self
    }
    pub fn set_character(&mut self, r#character: impl Into<Option<CharacterId>>) -> &mut Self {
        self.r#character = r#character.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeChangeCharacterRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#value = -100f32 as i32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#value = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeChangeCharacterRelations"
    }
}
impl Default for NodeChangeCharacterRelations {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeChangeCharacterRelations> for Node {
    fn from(item: NodeChangeCharacterRelations) -> Self {
        Self::ChangeCharacterRelations(item)
    }
}
impl NodeChangeCharacterRelations {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn change_character_relations() -> NodeChangeCharacterRelations {
        NodeChangeCharacterRelations::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeChangeFactionRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#value: i32,
}
impl NodeChangeFactionRelations {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeChangeFactionRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#value = -100f32 as i32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#value = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeChangeFactionRelations"
    }
}
impl Default for NodeChangeFactionRelations {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeChangeFactionRelations> for Node {
    fn from(item: NodeChangeFactionRelations) -> Self {
        Self::ChangeFactionRelations(item)
    }
}
impl NodeChangeFactionRelations {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn change_faction_relations() -> NodeChangeFactionRelations {
        NodeChangeFactionRelations::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeChangeFactionStarbasePower {
    pub r#id: i32,
    pub r#default_transition: i32,
    ///Percentage value
    pub r#value: i32,
}
impl NodeChangeFactionStarbasePower {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for NodeChangeFactionStarbasePower {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
        if self.r#value < (-100000f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = -100000f32,
                "Field got truncated"
            );
            self.r#value = -100000f32 as i32;
        }
        if self.r#value > (100000f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                max = 100000f32,
                "Field got truncated"
            );
            self.r#value = 100000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeChangeFactionStarbasePower"
    }
}
impl Default for NodeChangeFactionStarbasePower {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeChangeFactionStarbasePower> for Node {
    fn from(item: NodeChangeFactionStarbasePower) -> Self {
        Self::ChangeFactionStarbasePower(item)
    }
}
impl NodeChangeFactionStarbasePower {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn change_faction_starbase_power() -> NodeChangeFactionStarbasePower {
        NodeChangeFactionStarbasePower::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeCaptureStarBase {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeCaptureStarBase {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
}
impl DatabaseItem for NodeCaptureStarBase {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeCaptureStarBase"
    }
}
impl Default for NodeCaptureStarBase {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeCaptureStarBase> for Node {
    fn from(item: NodeCaptureStarBase) -> Self {
        Self::CaptureStarBase(item)
    }
}
impl NodeCaptureStarBase {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn capture_star_base() -> NodeCaptureStarBase {
        NodeCaptureStarBase::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeLiberateStarBase {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeLiberateStarBase {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
}
impl DatabaseItem for NodeLiberateStarBase {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeLiberateStarBase"
    }
}
impl Default for NodeLiberateStarBase {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeLiberateStarBase> for Node {
    fn from(item: NodeLiberateStarBase) -> Self {
        Self::LiberateStarBase(item)
    }
}
impl NodeLiberateStarBase {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn liberate_star_base() -> NodeLiberateStarBase {
        NodeLiberateStarBase::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeChangeFaction {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#faction: Option<FactionId>,
}
impl NodeChangeFaction {
    pub fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#faction: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<i32>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<i32>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_default_transition(mut self, r#default_transition: impl Into<i32>) -> Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn set_default_transition(&mut self, r#default_transition: impl Into<i32>) -> &mut Self {
        self.r#default_transition = r#default_transition.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
}
impl DatabaseItem for NodeChangeFaction {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                min = 1f32,
                "Field got truncated"
            );
            self.r#id = 1f32 as i32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                field = "r#id",
                value = self.r#id,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#id = 999999f32 as i32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_transition = 1f32 as i32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                field = "r#default_transition",
                value = self.r#default_transition,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#default_transition = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeChangeFaction"
    }
}
impl Default for NodeChangeFaction {
    fn default() -> Self {
        Self::new()
    }
}
impl From<NodeChangeFaction> for Node {
    fn from(item: NodeChangeFaction) -> Self {
        Self::ChangeFaction(item)
    }
}
impl NodeChangeFaction {
    pub fn wrap(self) -> Node {
        self.into()
    }
}
impl Node {
    pub fn change_faction() -> NodeChangeFaction {
        NodeChangeFaction::new()
    }
}
impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "Node")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: NodeType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Undefined(x) => AdjTagged {
                t: NodeType::Undefined,
                c: x,
            }
            .serialize(serializer),
            Self::ComingSoon(x) => AdjTagged {
                t: NodeType::ComingSoon,
                c: x,
            }
            .serialize(serializer),
            Self::ShowDialog(x) => AdjTagged {
                t: NodeType::ShowDialog,
                c: x,
            }
            .serialize(serializer),
            Self::OpenShipyard(x) => AdjTagged {
                t: NodeType::OpenShipyard,
                c: x,
            }
            .serialize(serializer),
            Self::OpenWorkshop(x) => AdjTagged {
                t: NodeType::OpenWorkshop,
                c: x,
            }
            .serialize(serializer),
            Self::Switch(x) => AdjTagged {
                t: NodeType::Switch,
                c: x,
            }
            .serialize(serializer),
            Self::Random(x) => AdjTagged {
                t: NodeType::Random,
                c: x,
            }
            .serialize(serializer),
            Self::Condition(x) => AdjTagged {
                t: NodeType::Condition,
                c: x,
            }
            .serialize(serializer),
            Self::AttackFleet(x) => AdjTagged {
                t: NodeType::AttackFleet,
                c: x,
            }
            .serialize(serializer),
            Self::AttackOccupants(x) => AdjTagged {
                t: NodeType::AttackOccupants,
                c: x,
            }
            .serialize(serializer),
            Self::AttackStarbase(x) => AdjTagged {
                t: NodeType::AttackStarbase,
                c: x,
            }
            .serialize(serializer),
            Self::DestroyOccupants(x) => AdjTagged {
                t: NodeType::DestroyOccupants,
                c: x,
            }
            .serialize(serializer),
            Self::SuppressOccupants(x) => AdjTagged {
                t: NodeType::SuppressOccupants,
                c: x,
            }
            .serialize(serializer),
            Self::Retreat(x) => AdjTagged {
                t: NodeType::Retreat,
                c: x,
            }
            .serialize(serializer),
            Self::ReceiveItem(x) => AdjTagged {
                t: NodeType::ReceiveItem,
                c: x,
            }
            .serialize(serializer),
            Self::RemoveItem(x) => AdjTagged {
                t: NodeType::RemoveItem,
                c: x,
            }
            .serialize(serializer),
            Self::Trade(x) => AdjTagged {
                t: NodeType::Trade,
                c: x,
            }
            .serialize(serializer),
            Self::CompleteQuest(x) => AdjTagged {
                t: NodeType::CompleteQuest,
                c: x,
            }
            .serialize(serializer),
            Self::FailQuest(x) => AdjTagged {
                t: NodeType::FailQuest,
                c: x,
            }
            .serialize(serializer),
            Self::CancelQuest(x) => AdjTagged {
                t: NodeType::CancelQuest,
                c: x,
            }
            .serialize(serializer),
            Self::StartQuest(x) => AdjTagged {
                t: NodeType::StartQuest,
                c: x,
            }
            .serialize(serializer),
            Self::SetCharacterRelations(x) => AdjTagged {
                t: NodeType::SetCharacterRelations,
                c: x,
            }
            .serialize(serializer),
            Self::SetFactionRelations(x) => AdjTagged {
                t: NodeType::SetFactionRelations,
                c: x,
            }
            .serialize(serializer),
            Self::SetFactionStarbasePower(x) => AdjTagged {
                t: NodeType::SetFactionStarbasePower,
                c: x,
            }
            .serialize(serializer),
            Self::ChangeCharacterRelations(x) => AdjTagged {
                t: NodeType::ChangeCharacterRelations,
                c: x,
            }
            .serialize(serializer),
            Self::ChangeFactionRelations(x) => AdjTagged {
                t: NodeType::ChangeFactionRelations,
                c: x,
            }
            .serialize(serializer),
            Self::ChangeFactionStarbasePower(x) => AdjTagged {
                t: NodeType::ChangeFactionStarbasePower,
                c: x,
            }
            .serialize(serializer),
            Self::CaptureStarBase(x) => AdjTagged {
                t: NodeType::CaptureStarBase,
                c: x,
            }
            .serialize(serializer),
            Self::LiberateStarBase(x) => AdjTagged {
                t: NodeType::LiberateStarBase,
                c: x,
            }
            .serialize(serializer),
            Self::ChangeFaction(x) => AdjTagged {
                t: NodeType::ChangeFaction,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl Node {
    pub fn r#id(&self) -> &i32 {
        match self {
            Self::Undefined(x) => &x.r#id,
            Self::ComingSoon(x) => &x.r#id,
            Self::ShowDialog(x) => &x.r#id,
            Self::OpenShipyard(x) => &x.r#id,
            Self::OpenWorkshop(x) => &x.r#id,
            Self::Switch(x) => &x.r#id,
            Self::Random(x) => &x.r#id,
            Self::Condition(x) => &x.r#id,
            Self::AttackFleet(x) => &x.r#id,
            Self::AttackOccupants(x) => &x.r#id,
            Self::AttackStarbase(x) => &x.r#id,
            Self::DestroyOccupants(x) => &x.r#id,
            Self::SuppressOccupants(x) => &x.r#id,
            Self::Retreat(x) => &x.r#id,
            Self::ReceiveItem(x) => &x.r#id,
            Self::RemoveItem(x) => &x.r#id,
            Self::Trade(x) => &x.r#id,
            Self::CompleteQuest(x) => &x.r#id,
            Self::FailQuest(x) => &x.r#id,
            Self::CancelQuest(x) => &x.r#id,
            Self::StartQuest(x) => &x.r#id,
            Self::SetCharacterRelations(x) => &x.r#id,
            Self::SetFactionRelations(x) => &x.r#id,
            Self::SetFactionStarbasePower(x) => &x.r#id,
            Self::ChangeCharacterRelations(x) => &x.r#id,
            Self::ChangeFactionRelations(x) => &x.r#id,
            Self::ChangeFactionStarbasePower(x) => &x.r#id,
            Self::CaptureStarBase(x) => &x.r#id,
            Self::LiberateStarBase(x) => &x.r#id,
            Self::ChangeFaction(x) => &x.r#id,
        }
    }
    pub fn id_mut(&mut self) -> &mut i32 {
        match self {
            Self::Undefined(x) => &mut x.r#id,
            Self::ComingSoon(x) => &mut x.r#id,
            Self::ShowDialog(x) => &mut x.r#id,
            Self::OpenShipyard(x) => &mut x.r#id,
            Self::OpenWorkshop(x) => &mut x.r#id,
            Self::Switch(x) => &mut x.r#id,
            Self::Random(x) => &mut x.r#id,
            Self::Condition(x) => &mut x.r#id,
            Self::AttackFleet(x) => &mut x.r#id,
            Self::AttackOccupants(x) => &mut x.r#id,
            Self::AttackStarbase(x) => &mut x.r#id,
            Self::DestroyOccupants(x) => &mut x.r#id,
            Self::SuppressOccupants(x) => &mut x.r#id,
            Self::Retreat(x) => &mut x.r#id,
            Self::ReceiveItem(x) => &mut x.r#id,
            Self::RemoveItem(x) => &mut x.r#id,
            Self::Trade(x) => &mut x.r#id,
            Self::CompleteQuest(x) => &mut x.r#id,
            Self::FailQuest(x) => &mut x.r#id,
            Self::CancelQuest(x) => &mut x.r#id,
            Self::StartQuest(x) => &mut x.r#id,
            Self::SetCharacterRelations(x) => &mut x.r#id,
            Self::SetFactionRelations(x) => &mut x.r#id,
            Self::SetFactionStarbasePower(x) => &mut x.r#id,
            Self::ChangeCharacterRelations(x) => &mut x.r#id,
            Self::ChangeFactionRelations(x) => &mut x.r#id,
            Self::ChangeFactionStarbasePower(x) => &mut x.r#id,
            Self::CaptureStarBase(x) => &mut x.r#id,
            Self::LiberateStarBase(x) => &mut x.r#id,
            Self::ChangeFaction(x) => &mut x.r#id,
        }
    }
}
impl DatabaseItem for Node {
    fn validate(&mut self) {
        match self {
            Self::Undefined(x) => x.validate(),
            Self::ComingSoon(x) => x.validate(),
            Self::ShowDialog(x) => x.validate(),
            Self::OpenShipyard(x) => x.validate(),
            Self::OpenWorkshop(x) => x.validate(),
            Self::Switch(x) => x.validate(),
            Self::Random(x) => x.validate(),
            Self::Condition(x) => x.validate(),
            Self::AttackFleet(x) => x.validate(),
            Self::AttackOccupants(x) => x.validate(),
            Self::AttackStarbase(x) => x.validate(),
            Self::DestroyOccupants(x) => x.validate(),
            Self::SuppressOccupants(x) => x.validate(),
            Self::Retreat(x) => x.validate(),
            Self::ReceiveItem(x) => x.validate(),
            Self::RemoveItem(x) => x.validate(),
            Self::Trade(x) => x.validate(),
            Self::CompleteQuest(x) => x.validate(),
            Self::FailQuest(x) => x.validate(),
            Self::CancelQuest(x) => x.validate(),
            Self::StartQuest(x) => x.validate(),
            Self::SetCharacterRelations(x) => x.validate(),
            Self::SetFactionRelations(x) => x.validate(),
            Self::SetFactionStarbasePower(x) => x.validate(),
            Self::ChangeCharacterRelations(x) => x.validate(),
            Self::ChangeFactionRelations(x) => x.validate(),
            Self::ChangeFactionStarbasePower(x) => x.validate(),
            Self::CaptureStarBase(x) => x.validate(),
            Self::LiberateStarBase(x) => x.validate(),
            Self::ChangeFaction(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "Node"
    }
}
impl Node {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Undefined(_) => NodeUndefined::type_name(),
            Self::ComingSoon(_) => NodeComingSoon::type_name(),
            Self::ShowDialog(_) => NodeShowDialog::type_name(),
            Self::OpenShipyard(_) => NodeOpenShipyard::type_name(),
            Self::OpenWorkshop(_) => NodeOpenWorkshop::type_name(),
            Self::Switch(_) => NodeSwitch::type_name(),
            Self::Random(_) => NodeRandom::type_name(),
            Self::Condition(_) => NodeCondition::type_name(),
            Self::AttackFleet(_) => NodeAttackFleet::type_name(),
            Self::AttackOccupants(_) => NodeAttackOccupants::type_name(),
            Self::AttackStarbase(_) => NodeAttackStarbase::type_name(),
            Self::DestroyOccupants(_) => NodeDestroyOccupants::type_name(),
            Self::SuppressOccupants(_) => NodeSuppressOccupants::type_name(),
            Self::Retreat(_) => NodeRetreat::type_name(),
            Self::ReceiveItem(_) => NodeReceiveItem::type_name(),
            Self::RemoveItem(_) => NodeRemoveItem::type_name(),
            Self::Trade(_) => NodeTrade::type_name(),
            Self::CompleteQuest(_) => NodeCompleteQuest::type_name(),
            Self::FailQuest(_) => NodeFailQuest::type_name(),
            Self::CancelQuest(_) => NodeCancelQuest::type_name(),
            Self::StartQuest(_) => NodeStartQuest::type_name(),
            Self::SetCharacterRelations(_) => NodeSetCharacterRelations::type_name(),
            Self::SetFactionRelations(_) => NodeSetFactionRelations::type_name(),
            Self::SetFactionStarbasePower(_) => NodeSetFactionStarbasePower::type_name(),
            Self::ChangeCharacterRelations(_) => NodeChangeCharacterRelations::type_name(),
            Self::ChangeFactionRelations(_) => NodeChangeFactionRelations::type_name(),
            Self::ChangeFactionStarbasePower(_) => NodeChangeFactionStarbasePower::type_name(),
            Self::CaptureStarBase(_) => NodeCaptureStarBase::type_name(),
            Self::LiberateStarBase(_) => NodeLiberateStarBase::type_name(),
            Self::ChangeFaction(_) => NodeChangeFaction::type_name(),
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/NodeAction.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeAction {
    pub r#target_node: i32,
    pub r#requirement: Requirement,
    pub r#button_text: String,
}
impl NodeAction {
    pub fn new() -> Self {
        Self {
            r#target_node: Default::default(),
            r#requirement: Default::default(),
            r#button_text: Default::default(),
        }
    }
    pub fn with_target_node(mut self, r#target_node: impl Into<i32>) -> Self {
        self.r#target_node = r#target_node.into();
        self
    }
    pub fn set_target_node(&mut self, r#target_node: impl Into<i32>) -> &mut Self {
        self.r#target_node = r#target_node.into();
        self
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<Requirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(&mut self, r#requirement: impl Into<Requirement>) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_button_text(mut self, r#button_text: impl Into<String>) -> Self {
        self.r#button_text = r#button_text.into();
        self
    }
    pub fn set_button_text(&mut self, r#button_text: impl Into<String>) -> &mut Self {
        self.r#button_text = r#button_text.into();
        self
    }
}
impl DatabaseItem for NodeAction {
    fn validate(&mut self) {
        if self.r#target_node < (1f32 as i32) {
            tracing::warn!(
                field = "r#target_node",
                value = self.r#target_node,
                min = 1f32,
                "Field got truncated"
            );
            self.r#target_node = 1f32 as i32;
        }
        if self.r#target_node > (1000f32 as i32) {
            tracing::warn!(
                field = "r#target_node",
                value = self.r#target_node,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#target_node = 1000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "NodeAction"
    }
}
impl Default for NodeAction {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/NodeTransition.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeTransition {
    pub r#target_node: i32,
    pub r#requirement: Requirement,
    pub r#weight: f32,
}
impl NodeTransition {
    pub fn new() -> Self {
        Self {
            r#target_node: Default::default(),
            r#requirement: Default::default(),
            r#weight: Default::default(),
        }
    }
    pub fn with_target_node(mut self, r#target_node: impl Into<i32>) -> Self {
        self.r#target_node = r#target_node.into();
        self
    }
    pub fn set_target_node(&mut self, r#target_node: impl Into<i32>) -> &mut Self {
        self.r#target_node = r#target_node.into();
        self
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<Requirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(&mut self, r#requirement: impl Into<Requirement>) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_weight(mut self, r#weight: impl Into<f32>) -> Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn set_weight(&mut self, r#weight: impl Into<f32>) -> &mut Self {
        self.r#weight = r#weight.into();
        self
    }
}
impl DatabaseItem for NodeTransition {
    fn validate(&mut self) {
        if self.r#target_node < (1f32 as i32) {
            tracing::warn!(
                field = "r#target_node",
                value = self.r#target_node,
                min = 1f32,
                "Field got truncated"
            );
            self.r#target_node = 1f32 as i32;
        }
        if self.r#target_node > (1000f32 as i32) {
            tracing::warn!(
                field = "r#target_node",
                value = self.r#target_node,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#target_node = 1000f32 as i32;
        }
        if self.r#weight < (0f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                min = 0f32,
                "Field got truncated"
            );
            self.r#weight = 0f32 as f32;
        }
        if self.r#weight > (1000f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#weight = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "NodeTransition"
    }
}
impl Default for NodeTransition {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/QuestOrigin.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuestOrigin {
    pub r#type: QuestOriginType,
    pub r#factions: FactionFilter,
    pub r#min_distance: i32,
    pub r#max_distance: i32,
    pub r#min_relations: i32,
    pub r#max_relations: i32,
}
impl QuestOrigin {
    pub fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#factions: Default::default(),
            r#min_distance: Default::default(),
            r#max_distance: Default::default(),
            r#min_relations: Default::default(),
            r#max_relations: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: impl Into<QuestOriginType>) -> Self {
        self.r#type = r#type.into();
        self
    }
    pub fn set_type(&mut self, r#type: impl Into<QuestOriginType>) -> &mut Self {
        self.r#type = r#type.into();
        self
    }
    pub fn with_factions(mut self, r#factions: impl Into<FactionFilter>) -> Self {
        self.r#factions = r#factions.into();
        self
    }
    pub fn set_factions(&mut self, r#factions: impl Into<FactionFilter>) -> &mut Self {
        self.r#factions = r#factions.into();
        self
    }
    pub fn with_min_distance(mut self, r#min_distance: impl Into<i32>) -> Self {
        self.r#min_distance = r#min_distance.into();
        self
    }
    pub fn set_min_distance(&mut self, r#min_distance: impl Into<i32>) -> &mut Self {
        self.r#min_distance = r#min_distance.into();
        self
    }
    pub fn with_max_distance(mut self, r#max_distance: impl Into<i32>) -> Self {
        self.r#max_distance = r#max_distance.into();
        self
    }
    pub fn set_max_distance(&mut self, r#max_distance: impl Into<i32>) -> &mut Self {
        self.r#max_distance = r#max_distance.into();
        self
    }
    pub fn with_min_relations(mut self, r#min_relations: impl Into<i32>) -> Self {
        self.r#min_relations = r#min_relations.into();
        self
    }
    pub fn set_min_relations(&mut self, r#min_relations: impl Into<i32>) -> &mut Self {
        self.r#min_relations = r#min_relations.into();
        self
    }
    pub fn with_max_relations(mut self, r#max_relations: impl Into<i32>) -> Self {
        self.r#max_relations = r#max_relations.into();
        self
    }
    pub fn set_max_relations(&mut self, r#max_relations: impl Into<i32>) -> &mut Self {
        self.r#max_relations = r#max_relations.into();
        self
    }
}
impl DatabaseItem for QuestOrigin {
    fn validate(&mut self) {
        if self.r#min_distance < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_distance",
                value = self.r#min_distance,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_distance = 0f32 as i32;
        }
        if self.r#min_distance > (9999f32 as i32) {
            tracing::warn!(
                field = "r#min_distance",
                value = self.r#min_distance,
                max = 9999f32,
                "Field got truncated"
            );
            self.r#min_distance = 9999f32 as i32;
        }
        if self.r#max_distance < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_distance",
                value = self.r#max_distance,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_distance = 0f32 as i32;
        }
        if self.r#max_distance > (9999f32 as i32) {
            tracing::warn!(
                field = "r#max_distance",
                value = self.r#max_distance,
                max = 9999f32,
                "Field got truncated"
            );
            self.r#max_distance = 9999f32 as i32;
        }
        if self.r#min_relations < (-100f32 as i32) {
            tracing::warn!(
                field = "r#min_relations",
                value = self.r#min_relations,
                min = -100f32,
                "Field got truncated"
            );
            self.r#min_relations = -100f32 as i32;
        }
        if self.r#min_relations > (100f32 as i32) {
            tracing::warn!(
                field = "r#min_relations",
                value = self.r#min_relations,
                max = 100f32,
                "Field got truncated"
            );
            self.r#min_relations = 100f32 as i32;
        }
        if self.r#max_relations < (-100f32 as i32) {
            tracing::warn!(
                field = "r#max_relations",
                value = self.r#max_relations,
                min = -100f32,
                "Field got truncated"
            );
            self.r#max_relations = -100f32 as i32;
        }
        if self.r#max_relations > (100f32 as i32) {
            tracing::warn!(
                field = "r#max_relations",
                value = self.r#max_relations,
                max = 100f32,
                "Field got truncated"
            );
            self.r#max_relations = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "QuestOrigin"
    }
}
impl Default for QuestOrigin {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Requirement.xml
#[derive(Debug, Clone)]
pub enum Requirement {
    Empty(RequirementEmpty),
    Any(RequirementAny),
    All(RequirementAll),
    None(RequirementNone),
    PlayerPosition(RequirementPlayerPosition),
    RandomStarSystem(RequirementRandomStarSystem),
    AggressiveOccupants(RequirementAggressiveOccupants),
    QuestCompleted(RequirementQuestCompleted),
    QuestActive(RequirementQuestActive),
    CharacterRelations(RequirementCharacterRelations),
    FactionRelations(RequirementFactionRelations),
    StarbaseCaptured(RequirementStarbaseCaptured),
    FactionStarbasePower(RequirementFactionStarbasePower),
    IsHostileFaction(RequirementIsHostileFaction),
    Faction(RequirementFaction),
    HaveQuestItem(RequirementHaveQuestItem),
    HaveItem(RequirementHaveItem),
    HaveItemById(RequirementHaveItemById),
    ComeToOrigin(RequirementComeToOrigin),
    TimeSinceQuestStart(RequirementTimeSinceQuestStart),
    TimeSinceLastCompletion(RequirementTimeSinceLastCompletion),
}
impl Default for Requirement {
    fn default() -> Self {
        Self::Empty(Default::default())
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementEmpty {}
impl RequirementEmpty {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementEmpty {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementEmpty"
    }
}
impl Default for RequirementEmpty {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementEmpty> for Requirement {
    fn from(item: RequirementEmpty) -> Self {
        Self::Empty(item)
    }
}
impl RequirementEmpty {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn empty() -> RequirementEmpty {
        RequirementEmpty::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementAny {
    pub r#requirements: Vec<Requirement>,
}
impl RequirementAny {
    pub fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(mut self, r#requirements: impl Into<Vec<Requirement>>) -> Self {
        self.r#requirements = r#requirements.into();
        self
    }
    pub fn set_requirements(&mut self, r#requirements: impl Into<Vec<Requirement>>) -> &mut Self {
        self.r#requirements = r#requirements.into();
        self
    }
}
impl DatabaseItem for RequirementAny {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementAny"
    }
}
impl Default for RequirementAny {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementAny> for Requirement {
    fn from(item: RequirementAny) -> Self {
        Self::Any(item)
    }
}
impl RequirementAny {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn any() -> RequirementAny {
        RequirementAny::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementAll {
    pub r#requirements: Vec<Requirement>,
}
impl RequirementAll {
    pub fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(mut self, r#requirements: impl Into<Vec<Requirement>>) -> Self {
        self.r#requirements = r#requirements.into();
        self
    }
    pub fn set_requirements(&mut self, r#requirements: impl Into<Vec<Requirement>>) -> &mut Self {
        self.r#requirements = r#requirements.into();
        self
    }
}
impl DatabaseItem for RequirementAll {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementAll"
    }
}
impl Default for RequirementAll {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementAll> for Requirement {
    fn from(item: RequirementAll) -> Self {
        Self::All(item)
    }
}
impl RequirementAll {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn all() -> RequirementAll {
        RequirementAll::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementNone {
    pub r#requirements: Vec<Requirement>,
}
impl RequirementNone {
    pub fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(mut self, r#requirements: impl Into<Vec<Requirement>>) -> Self {
        self.r#requirements = r#requirements.into();
        self
    }
    pub fn set_requirements(&mut self, r#requirements: impl Into<Vec<Requirement>>) -> &mut Self {
        self.r#requirements = r#requirements.into();
        self
    }
}
impl DatabaseItem for RequirementNone {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementNone"
    }
}
impl Default for RequirementNone {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementNone> for Requirement {
    fn from(item: RequirementNone) -> Self {
        Self::None(item)
    }
}
impl RequirementNone {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn none() -> RequirementNone {
        RequirementNone::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementPlayerPosition {
    pub r#min_value: i32,
    pub r#max_value: i32,
    pub r#bool_value: bool,
}
impl RequirementPlayerPosition {
    pub fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
            r#bool_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<i32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<i32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn with_bool_value(mut self, r#bool_value: impl Into<bool>) -> Self {
        self.r#bool_value = r#bool_value.into();
        self
    }
    pub fn set_bool_value(&mut self, r#bool_value: impl Into<bool>) -> &mut Self {
        self.r#bool_value = r#bool_value.into();
        self
    }
}
impl DatabaseItem for RequirementPlayerPosition {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as i32;
        }
        if self.r#min_value > (10000f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#min_value = 10000f32 as i32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as i32;
        }
        if self.r#max_value > (10000f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#max_value = 10000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementPlayerPosition"
    }
}
impl Default for RequirementPlayerPosition {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementPlayerPosition> for Requirement {
    fn from(item: RequirementPlayerPosition) -> Self {
        Self::PlayerPosition(item)
    }
}
impl RequirementPlayerPosition {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn player_position() -> RequirementPlayerPosition {
        RequirementPlayerPosition::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementRandomStarSystem {
    pub r#min_value: i32,
    pub r#max_value: i32,
    pub r#bool_value: bool,
}
impl RequirementRandomStarSystem {
    pub fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
            r#bool_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<i32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<i32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn with_bool_value(mut self, r#bool_value: impl Into<bool>) -> Self {
        self.r#bool_value = r#bool_value.into();
        self
    }
    pub fn set_bool_value(&mut self, r#bool_value: impl Into<bool>) -> &mut Self {
        self.r#bool_value = r#bool_value.into();
        self
    }
}
impl DatabaseItem for RequirementRandomStarSystem {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as i32;
        }
        if self.r#min_value > (10000f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#min_value = 10000f32 as i32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as i32;
        }
        if self.r#max_value > (10000f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#max_value = 10000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementRandomStarSystem"
    }
}
impl Default for RequirementRandomStarSystem {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementRandomStarSystem> for Requirement {
    fn from(item: RequirementRandomStarSystem) -> Self {
        Self::RandomStarSystem(item)
    }
}
impl RequirementRandomStarSystem {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn random_star_system() -> RequirementRandomStarSystem {
        RequirementRandomStarSystem::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementAggressiveOccupants {}
impl RequirementAggressiveOccupants {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementAggressiveOccupants {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementAggressiveOccupants"
    }
}
impl Default for RequirementAggressiveOccupants {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementAggressiveOccupants> for Requirement {
    fn from(item: RequirementAggressiveOccupants) -> Self {
        Self::AggressiveOccupants(item)
    }
}
impl RequirementAggressiveOccupants {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn aggressive_occupants() -> RequirementAggressiveOccupants {
        RequirementAggressiveOccupants::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementQuestCompleted {
    pub r#item_id: Option<QuestId>,
}
impl RequirementQuestCompleted {
    pub fn new() -> Self {
        Self {
            r#item_id: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<Option<QuestId>>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<Option<QuestId>>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
}
impl DatabaseItem for RequirementQuestCompleted {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementQuestCompleted"
    }
}
impl Default for RequirementQuestCompleted {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementQuestCompleted> for Requirement {
    fn from(item: RequirementQuestCompleted) -> Self {
        Self::QuestCompleted(item)
    }
}
impl RequirementQuestCompleted {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn quest_completed() -> RequirementQuestCompleted {
        RequirementQuestCompleted::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementQuestActive {
    pub r#item_id: Option<QuestId>,
}
impl RequirementQuestActive {
    pub fn new() -> Self {
        Self {
            r#item_id: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<Option<QuestId>>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<Option<QuestId>>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
}
impl DatabaseItem for RequirementQuestActive {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementQuestActive"
    }
}
impl Default for RequirementQuestActive {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementQuestActive> for Requirement {
    fn from(item: RequirementQuestActive) -> Self {
        Self::QuestActive(item)
    }
}
impl RequirementQuestActive {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn quest_active() -> RequirementQuestActive {
        RequirementQuestActive::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementCharacterRelations {
    pub r#min_value: i32,
    pub r#max_value: i32,
    pub r#character: Option<CharacterId>,
}
impl RequirementCharacterRelations {
    pub fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
            r#character: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<i32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<i32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn with_character(mut self, r#character: impl Into<Option<CharacterId>>) -> Self {
        self.r#character = r#character.into();
        self
    }
    pub fn set_character(&mut self, r#character: impl Into<Option<CharacterId>>) -> &mut Self {
        self.r#character = r#character.into();
        self
    }
}
impl DatabaseItem for RequirementCharacterRelations {
    fn validate(&mut self) {
        if self.r#min_value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#min_value = -100f32 as i32;
        }
        if self.r#min_value > (100f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#min_value = 100f32 as i32;
        }
        if self.r#max_value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#max_value = -100f32 as i32;
        }
        if self.r#max_value > (100f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#max_value = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementCharacterRelations"
    }
}
impl Default for RequirementCharacterRelations {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementCharacterRelations> for Requirement {
    fn from(item: RequirementCharacterRelations) -> Self {
        Self::CharacterRelations(item)
    }
}
impl RequirementCharacterRelations {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn character_relations() -> RequirementCharacterRelations {
        RequirementCharacterRelations::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementFactionRelations {
    pub r#min_value: i32,
    pub r#max_value: i32,
}
impl RequirementFactionRelations {
    pub fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<i32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<i32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for RequirementFactionRelations {
    fn validate(&mut self) {
        if self.r#min_value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#min_value = -100f32 as i32;
        }
        if self.r#min_value > (100f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#min_value = 100f32 as i32;
        }
        if self.r#max_value < (-100f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = -100f32,
                "Field got truncated"
            );
            self.r#max_value = -100f32 as i32;
        }
        if self.r#max_value > (100f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 100f32,
                "Field got truncated"
            );
            self.r#max_value = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementFactionRelations"
    }
}
impl Default for RequirementFactionRelations {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementFactionRelations> for Requirement {
    fn from(item: RequirementFactionRelations) -> Self {
        Self::FactionRelations(item)
    }
}
impl RequirementFactionRelations {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn faction_relations() -> RequirementFactionRelations {
        RequirementFactionRelations::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementStarbaseCaptured {}
impl RequirementStarbaseCaptured {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementStarbaseCaptured {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementStarbaseCaptured"
    }
}
impl Default for RequirementStarbaseCaptured {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementStarbaseCaptured> for Requirement {
    fn from(item: RequirementStarbaseCaptured) -> Self {
        Self::StarbaseCaptured(item)
    }
}
impl RequirementStarbaseCaptured {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn starbase_captured() -> RequirementStarbaseCaptured {
        RequirementStarbaseCaptured::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementFactionStarbasePower {
    ///Percentage value
    pub r#min_value: i32,
    ///Percentage value
    pub r#max_value: i32,
}
impl RequirementFactionStarbasePower {
    pub fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<i32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<i32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for RequirementFactionStarbasePower {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as i32;
        }
        if self.r#min_value > (100000f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 100000f32,
                "Field got truncated"
            );
            self.r#min_value = 100000f32 as i32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as i32;
        }
        if self.r#max_value > (100000f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 100000f32,
                "Field got truncated"
            );
            self.r#max_value = 100000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementFactionStarbasePower"
    }
}
impl Default for RequirementFactionStarbasePower {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementFactionStarbasePower> for Requirement {
    fn from(item: RequirementFactionStarbasePower) -> Self {
        Self::FactionStarbasePower(item)
    }
}
impl RequirementFactionStarbasePower {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn faction_starbase_power() -> RequirementFactionStarbasePower {
        RequirementFactionStarbasePower::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementIsHostileFaction {}
impl RequirementIsHostileFaction {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementIsHostileFaction {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementIsHostileFaction"
    }
}
impl Default for RequirementIsHostileFaction {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementIsHostileFaction> for Requirement {
    fn from(item: RequirementIsHostileFaction) -> Self {
        Self::IsHostileFaction(item)
    }
}
impl RequirementIsHostileFaction {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn is_hostile_faction() -> RequirementIsHostileFaction {
        RequirementIsHostileFaction::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementFaction {
    pub r#faction: Option<FactionId>,
}
impl RequirementFaction {
    pub fn new() -> Self {
        Self {
            r#faction: Default::default(),
        }
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
}
impl DatabaseItem for RequirementFaction {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementFaction"
    }
}
impl Default for RequirementFaction {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementFaction> for Requirement {
    fn from(item: RequirementFaction) -> Self {
        Self::Faction(item)
    }
}
impl RequirementFaction {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn faction() -> RequirementFaction {
        RequirementFaction::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementHaveQuestItem {
    pub r#item_id: Option<QuestItemId>,
    pub r#min_value: i32,
}
impl RequirementHaveQuestItem {
    pub fn new() -> Self {
        Self {
            r#item_id: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<Option<QuestItemId>>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<Option<QuestItemId>>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
}
impl DatabaseItem for RequirementHaveQuestItem {
    fn validate(&mut self) {
        if self.r#min_value < (1f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 1f32,
                "Field got truncated"
            );
            self.r#min_value = 1f32 as i32;
        }
        if self.r#min_value > (1000000f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#min_value = 1000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementHaveQuestItem"
    }
}
impl Default for RequirementHaveQuestItem {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementHaveQuestItem> for Requirement {
    fn from(item: RequirementHaveQuestItem) -> Self {
        Self::HaveQuestItem(item)
    }
}
impl RequirementHaveQuestItem {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn have_quest_item() -> RequirementHaveQuestItem {
        RequirementHaveQuestItem::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementHaveItem {
    pub r#loot: LootContent,
}
impl RequirementHaveItem {
    pub fn new() -> Self {
        Self {
            r#loot: Default::default(),
        }
    }
    pub fn with_loot(mut self, r#loot: impl Into<LootContent>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<LootContent>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for RequirementHaveItem {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementHaveItem"
    }
}
impl Default for RequirementHaveItem {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementHaveItem> for Requirement {
    fn from(item: RequirementHaveItem) -> Self {
        Self::HaveItem(item)
    }
}
impl RequirementHaveItem {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn have_item() -> RequirementHaveItem {
        RequirementHaveItem::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementHaveItemById {
    pub r#item_id: Option<LootId>,
}
impl RequirementHaveItemById {
    pub fn new() -> Self {
        Self {
            r#item_id: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<Option<LootId>>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<Option<LootId>>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
}
impl DatabaseItem for RequirementHaveItemById {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementHaveItemById"
    }
}
impl Default for RequirementHaveItemById {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementHaveItemById> for Requirement {
    fn from(item: RequirementHaveItemById) -> Self {
        Self::HaveItemById(item)
    }
}
impl RequirementHaveItemById {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn have_item_by_id() -> RequirementHaveItemById {
        RequirementHaveItemById::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementComeToOrigin {
    pub r#bool_value: bool,
}
impl RequirementComeToOrigin {
    pub fn new() -> Self {
        Self {
            r#bool_value: Default::default(),
        }
    }
    pub fn with_bool_value(mut self, r#bool_value: impl Into<bool>) -> Self {
        self.r#bool_value = r#bool_value.into();
        self
    }
    pub fn set_bool_value(&mut self, r#bool_value: impl Into<bool>) -> &mut Self {
        self.r#bool_value = r#bool_value.into();
        self
    }
}
impl DatabaseItem for RequirementComeToOrigin {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "RequirementComeToOrigin"
    }
}
impl Default for RequirementComeToOrigin {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementComeToOrigin> for Requirement {
    fn from(item: RequirementComeToOrigin) -> Self {
        Self::ComeToOrigin(item)
    }
}
impl RequirementComeToOrigin {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn come_to_origin() -> RequirementComeToOrigin {
        RequirementComeToOrigin::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementTimeSinceQuestStart {
    pub r#min_value: i32,
    pub r#max_value: i32,
}
impl RequirementTimeSinceQuestStart {
    pub fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<i32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<i32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for RequirementTimeSinceQuestStart {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as i32;
        }
        if self.r#min_value > (999999f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#min_value = 999999f32 as i32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as i32;
        }
        if self.r#max_value > (999999f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#max_value = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementTimeSinceQuestStart"
    }
}
impl Default for RequirementTimeSinceQuestStart {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementTimeSinceQuestStart> for Requirement {
    fn from(item: RequirementTimeSinceQuestStart) -> Self {
        Self::TimeSinceQuestStart(item)
    }
}
impl RequirementTimeSinceQuestStart {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn time_since_quest_start() -> RequirementTimeSinceQuestStart {
        RequirementTimeSinceQuestStart::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequirementTimeSinceLastCompletion {
    pub r#min_value: i32,
    pub r#max_value: i32,
}
impl RequirementTimeSinceLastCompletion {
    pub fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: impl Into<i32>) -> Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn set_min_value(&mut self, r#min_value: impl Into<i32>) -> &mut Self {
        self.r#min_value = r#min_value.into();
        self
    }
    pub fn with_max_value(mut self, r#max_value: impl Into<i32>) -> Self {
        self.r#max_value = r#max_value.into();
        self
    }
    pub fn set_max_value(&mut self, r#max_value: impl Into<i32>) -> &mut Self {
        self.r#max_value = r#max_value.into();
        self
    }
}
impl DatabaseItem for RequirementTimeSinceLastCompletion {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#min_value = 0f32 as i32;
        }
        if self.r#min_value > (999999f32 as i32) {
            tracing::warn!(
                field = "r#min_value",
                value = self.r#min_value,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#min_value = 999999f32 as i32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_value = 0f32 as i32;
        }
        if self.r#max_value > (999999f32 as i32) {
            tracing::warn!(
                field = "r#max_value",
                value = self.r#max_value,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#max_value = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "RequirementTimeSinceLastCompletion"
    }
}
impl Default for RequirementTimeSinceLastCompletion {
    fn default() -> Self {
        Self::new()
    }
}
impl From<RequirementTimeSinceLastCompletion> for Requirement {
    fn from(item: RequirementTimeSinceLastCompletion) -> Self {
        Self::TimeSinceLastCompletion(item)
    }
}
impl RequirementTimeSinceLastCompletion {
    pub fn wrap(self) -> Requirement {
        self.into()
    }
}
impl Requirement {
    pub fn time_since_last_completion() -> RequirementTimeSinceLastCompletion {
        RequirementTimeSinceLastCompletion::new()
    }
}
impl serde::Serialize for Requirement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "Requirement")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: RequirementType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Empty(x) => AdjTagged {
                t: RequirementType::Empty,
                c: x,
            }
            .serialize(serializer),
            Self::Any(x) => AdjTagged {
                t: RequirementType::Any,
                c: x,
            }
            .serialize(serializer),
            Self::All(x) => AdjTagged {
                t: RequirementType::All,
                c: x,
            }
            .serialize(serializer),
            Self::None(x) => AdjTagged {
                t: RequirementType::None,
                c: x,
            }
            .serialize(serializer),
            Self::PlayerPosition(x) => AdjTagged {
                t: RequirementType::PlayerPosition,
                c: x,
            }
            .serialize(serializer),
            Self::RandomStarSystem(x) => AdjTagged {
                t: RequirementType::RandomStarSystem,
                c: x,
            }
            .serialize(serializer),
            Self::AggressiveOccupants(x) => AdjTagged {
                t: RequirementType::AggressiveOccupants,
                c: x,
            }
            .serialize(serializer),
            Self::QuestCompleted(x) => AdjTagged {
                t: RequirementType::QuestCompleted,
                c: x,
            }
            .serialize(serializer),
            Self::QuestActive(x) => AdjTagged {
                t: RequirementType::QuestActive,
                c: x,
            }
            .serialize(serializer),
            Self::CharacterRelations(x) => AdjTagged {
                t: RequirementType::CharacterRelations,
                c: x,
            }
            .serialize(serializer),
            Self::FactionRelations(x) => AdjTagged {
                t: RequirementType::FactionRelations,
                c: x,
            }
            .serialize(serializer),
            Self::StarbaseCaptured(x) => AdjTagged {
                t: RequirementType::StarbaseCaptured,
                c: x,
            }
            .serialize(serializer),
            Self::FactionStarbasePower(x) => AdjTagged {
                t: RequirementType::FactionStarbasePower,
                c: x,
            }
            .serialize(serializer),
            Self::IsHostileFaction(x) => AdjTagged {
                t: RequirementType::IsHostileFaction,
                c: x,
            }
            .serialize(serializer),
            Self::Faction(x) => AdjTagged {
                t: RequirementType::Faction,
                c: x,
            }
            .serialize(serializer),
            Self::HaveQuestItem(x) => AdjTagged {
                t: RequirementType::HaveQuestItem,
                c: x,
            }
            .serialize(serializer),
            Self::HaveItem(x) => AdjTagged {
                t: RequirementType::HaveItem,
                c: x,
            }
            .serialize(serializer),
            Self::HaveItemById(x) => AdjTagged {
                t: RequirementType::HaveItemById,
                c: x,
            }
            .serialize(serializer),
            Self::ComeToOrigin(x) => AdjTagged {
                t: RequirementType::ComeToOrigin,
                c: x,
            }
            .serialize(serializer),
            Self::TimeSinceQuestStart(x) => AdjTagged {
                t: RequirementType::TimeSinceQuestStart,
                c: x,
            }
            .serialize(serializer),
            Self::TimeSinceLastCompletion(x) => AdjTagged {
                t: RequirementType::TimeSinceLastCompletion,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl DatabaseItem for Requirement {
    fn validate(&mut self) {
        match self {
            Self::Empty(x) => x.validate(),
            Self::Any(x) => x.validate(),
            Self::All(x) => x.validate(),
            Self::None(x) => x.validate(),
            Self::PlayerPosition(x) => x.validate(),
            Self::RandomStarSystem(x) => x.validate(),
            Self::AggressiveOccupants(x) => x.validate(),
            Self::QuestCompleted(x) => x.validate(),
            Self::QuestActive(x) => x.validate(),
            Self::CharacterRelations(x) => x.validate(),
            Self::FactionRelations(x) => x.validate(),
            Self::StarbaseCaptured(x) => x.validate(),
            Self::FactionStarbasePower(x) => x.validate(),
            Self::IsHostileFaction(x) => x.validate(),
            Self::Faction(x) => x.validate(),
            Self::HaveQuestItem(x) => x.validate(),
            Self::HaveItem(x) => x.validate(),
            Self::HaveItemById(x) => x.validate(),
            Self::ComeToOrigin(x) => x.validate(),
            Self::TimeSinceQuestStart(x) => x.validate(),
            Self::TimeSinceLastCompletion(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "Requirement"
    }
}
impl Requirement {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Empty(_) => RequirementEmpty::type_name(),
            Self::Any(_) => RequirementAny::type_name(),
            Self::All(_) => RequirementAll::type_name(),
            Self::None(_) => RequirementNone::type_name(),
            Self::PlayerPosition(_) => RequirementPlayerPosition::type_name(),
            Self::RandomStarSystem(_) => RequirementRandomStarSystem::type_name(),
            Self::AggressiveOccupants(_) => RequirementAggressiveOccupants::type_name(),
            Self::QuestCompleted(_) => RequirementQuestCompleted::type_name(),
            Self::QuestActive(_) => RequirementQuestActive::type_name(),
            Self::CharacterRelations(_) => RequirementCharacterRelations::type_name(),
            Self::FactionRelations(_) => RequirementFactionRelations::type_name(),
            Self::StarbaseCaptured(_) => RequirementStarbaseCaptured::type_name(),
            Self::FactionStarbasePower(_) => RequirementFactionStarbasePower::type_name(),
            Self::IsHostileFaction(_) => RequirementIsHostileFaction::type_name(),
            Self::Faction(_) => RequirementFaction::type_name(),
            Self::HaveQuestItem(_) => RequirementHaveQuestItem::type_name(),
            Self::HaveItem(_) => RequirementHaveItem::type_name(),
            Self::HaveItemById(_) => RequirementHaveItemById::type_name(),
            Self::ComeToOrigin(_) => RequirementComeToOrigin::type_name(),
            Self::TimeSinceQuestStart(_) => RequirementTimeSinceQuestStart::type_name(),
            Self::TimeSinceLastCompletion(_) => RequirementTimeSinceLastCompletion::type_name(),
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/DebugCode.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DebugCode {
    pub r#code: i32,
    pub r#loot: LootContent,
}
impl DebugCode {
    pub fn new() -> Self {
        Self {
            r#code: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_code(mut self, r#code: impl Into<i32>) -> Self {
        self.r#code = r#code.into();
        self
    }
    pub fn set_code(&mut self, r#code: impl Into<i32>) -> &mut Self {
        self.r#code = r#code.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<LootContent>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<LootContent>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for DebugCode {
    fn validate(&mut self) {
        if self.r#code < (0f32 as i32) {
            tracing::warn!(
                field = "r#code",
                value = self.r#code,
                min = 0f32,
                "Field got truncated"
            );
            self.r#code = 0f32 as i32;
        }
        if self.r#code > (999999f32 as i32) {
            tracing::warn!(
                field = "r#code",
                value = self.r#code,
                max = 999999f32,
                "Field got truncated"
            );
            self.r#code = 999999f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "DebugCode"
    }
}
impl Default for DebugCode {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ShipToValue.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipToValue {
    pub r#ship: Option<ShipId>,
    pub r#value: i32,
}
impl ShipToValue {
    pub fn new() -> Self {
        Self {
            r#ship: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_ship(mut self, r#ship: impl Into<Option<ShipId>>) -> Self {
        self.r#ship = r#ship.into();
        self
    }
    pub fn set_ship(&mut self, r#ship: impl Into<Option<ShipId>>) -> &mut Self {
        self.r#ship = r#ship.into();
        self
    }
    pub fn with_value(mut self, r#value: impl Into<i32>) -> Self {
        self.r#value = r#value.into();
        self
    }
    pub fn set_value(&mut self, r#value: impl Into<i32>) -> &mut Self {
        self.r#value = r#value.into();
        self
    }
}
impl DatabaseItem for ShipToValue {
    fn validate(&mut self) {
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                field = "r#value",
                value = self.r#value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#value = 0f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "ShipToValue"
    }
}
impl Default for ShipToValue {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/SoundTrack.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoundTrack {
    pub r#audio: String,
}
impl SoundTrack {
    pub fn new() -> Self {
        Self {
            r#audio: Default::default(),
        }
    }
    pub fn with_audio(mut self, r#audio: impl Into<String>) -> Self {
        self.r#audio = r#audio.into();
        self
    }
    pub fn set_audio(&mut self, r#audio: impl Into<String>) -> &mut Self {
        self.r#audio = r#audio.into();
        self
    }
}
impl DatabaseItem for SoundTrack {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "SoundTrack"
    }
}
impl Default for SoundTrack {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ShipFeatures.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipFeatures {
    pub r#energy_resistance: f32,
    pub r#kinetic_resistance: f32,
    pub r#heat_resistance: f32,
    pub r#ship_weight_bonus: f32,
    pub r#equipment_weight_bonus: f32,
    pub r#velocity_bonus: f32,
    pub r#turn_rate_bonus: f32,
    pub r#armor_bonus: f32,
    pub r#shield_bonus: f32,
    pub r#energy_bonus: f32,
    pub r#regeneration: bool,
    pub r#builtin_devices: Vec<DeviceId>,
}
impl ShipFeatures {
    pub fn new() -> Self {
        Self {
            r#energy_resistance: Default::default(),
            r#kinetic_resistance: Default::default(),
            r#heat_resistance: Default::default(),
            r#ship_weight_bonus: Default::default(),
            r#equipment_weight_bonus: Default::default(),
            r#velocity_bonus: Default::default(),
            r#turn_rate_bonus: Default::default(),
            r#armor_bonus: Default::default(),
            r#shield_bonus: Default::default(),
            r#energy_bonus: Default::default(),
            r#regeneration: Default::default(),
            r#builtin_devices: Default::default(),
        }
    }
    pub fn with_energy_resistance(mut self, r#energy_resistance: impl Into<f32>) -> Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn set_energy_resistance(&mut self, r#energy_resistance: impl Into<f32>) -> &mut Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: impl Into<f32>) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn set_kinetic_resistance(&mut self, r#kinetic_resistance: impl Into<f32>) -> &mut Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn with_heat_resistance(mut self, r#heat_resistance: impl Into<f32>) -> Self {
        self.r#heat_resistance = r#heat_resistance.into();
        self
    }
    pub fn set_heat_resistance(&mut self, r#heat_resistance: impl Into<f32>) -> &mut Self {
        self.r#heat_resistance = r#heat_resistance.into();
        self
    }
    pub fn with_ship_weight_bonus(mut self, r#ship_weight_bonus: impl Into<f32>) -> Self {
        self.r#ship_weight_bonus = r#ship_weight_bonus.into();
        self
    }
    pub fn set_ship_weight_bonus(&mut self, r#ship_weight_bonus: impl Into<f32>) -> &mut Self {
        self.r#ship_weight_bonus = r#ship_weight_bonus.into();
        self
    }
    pub fn with_equipment_weight_bonus(mut self, r#equipment_weight_bonus: impl Into<f32>) -> Self {
        self.r#equipment_weight_bonus = r#equipment_weight_bonus.into();
        self
    }
    pub fn set_equipment_weight_bonus(
        &mut self,
        r#equipment_weight_bonus: impl Into<f32>,
    ) -> &mut Self {
        self.r#equipment_weight_bonus = r#equipment_weight_bonus.into();
        self
    }
    pub fn with_velocity_bonus(mut self, r#velocity_bonus: impl Into<f32>) -> Self {
        self.r#velocity_bonus = r#velocity_bonus.into();
        self
    }
    pub fn set_velocity_bonus(&mut self, r#velocity_bonus: impl Into<f32>) -> &mut Self {
        self.r#velocity_bonus = r#velocity_bonus.into();
        self
    }
    pub fn with_turn_rate_bonus(mut self, r#turn_rate_bonus: impl Into<f32>) -> Self {
        self.r#turn_rate_bonus = r#turn_rate_bonus.into();
        self
    }
    pub fn set_turn_rate_bonus(&mut self, r#turn_rate_bonus: impl Into<f32>) -> &mut Self {
        self.r#turn_rate_bonus = r#turn_rate_bonus.into();
        self
    }
    pub fn with_armor_bonus(mut self, r#armor_bonus: impl Into<f32>) -> Self {
        self.r#armor_bonus = r#armor_bonus.into();
        self
    }
    pub fn set_armor_bonus(&mut self, r#armor_bonus: impl Into<f32>) -> &mut Self {
        self.r#armor_bonus = r#armor_bonus.into();
        self
    }
    pub fn with_shield_bonus(mut self, r#shield_bonus: impl Into<f32>) -> Self {
        self.r#shield_bonus = r#shield_bonus.into();
        self
    }
    pub fn set_shield_bonus(&mut self, r#shield_bonus: impl Into<f32>) -> &mut Self {
        self.r#shield_bonus = r#shield_bonus.into();
        self
    }
    pub fn with_energy_bonus(mut self, r#energy_bonus: impl Into<f32>) -> Self {
        self.r#energy_bonus = r#energy_bonus.into();
        self
    }
    pub fn set_energy_bonus(&mut self, r#energy_bonus: impl Into<f32>) -> &mut Self {
        self.r#energy_bonus = r#energy_bonus.into();
        self
    }
    pub fn with_regeneration(mut self, r#regeneration: impl Into<bool>) -> Self {
        self.r#regeneration = r#regeneration.into();
        self
    }
    pub fn set_regeneration(&mut self, r#regeneration: impl Into<bool>) -> &mut Self {
        self.r#regeneration = r#regeneration.into();
        self
    }
    pub fn with_builtin_devices(mut self, r#builtin_devices: impl Into<Vec<DeviceId>>) -> Self {
        self.r#builtin_devices = r#builtin_devices.into();
        self
    }
    pub fn set_builtin_devices(
        &mut self,
        r#builtin_devices: impl Into<Vec<DeviceId>>,
    ) -> &mut Self {
        self.r#builtin_devices = r#builtin_devices.into();
        self
    }
}
impl DatabaseItem for ShipFeatures {
    fn validate(&mut self) {
        if self.r#energy_resistance < (-100f32 as f32) {
            tracing::warn!(
                field = "r#energy_resistance",
                value = self.r#energy_resistance,
                min = -100f32,
                "Field got truncated"
            );
            self.r#energy_resistance = -100f32 as f32;
        }
        if self.r#energy_resistance > (100f32 as f32) {
            tracing::warn!(
                field = "r#energy_resistance",
                value = self.r#energy_resistance,
                max = 100f32,
                "Field got truncated"
            );
            self.r#energy_resistance = 100f32 as f32;
        }
        if self.r#kinetic_resistance < (-100f32 as f32) {
            tracing::warn!(
                field = "r#kinetic_resistance",
                value = self.r#kinetic_resistance,
                min = -100f32,
                "Field got truncated"
            );
            self.r#kinetic_resistance = -100f32 as f32;
        }
        if self.r#kinetic_resistance > (100f32 as f32) {
            tracing::warn!(
                field = "r#kinetic_resistance",
                value = self.r#kinetic_resistance,
                max = 100f32,
                "Field got truncated"
            );
            self.r#kinetic_resistance = 100f32 as f32;
        }
        if self.r#heat_resistance < (-100f32 as f32) {
            tracing::warn!(
                field = "r#heat_resistance",
                value = self.r#heat_resistance,
                min = -100f32,
                "Field got truncated"
            );
            self.r#heat_resistance = -100f32 as f32;
        }
        if self.r#heat_resistance > (100f32 as f32) {
            tracing::warn!(
                field = "r#heat_resistance",
                value = self.r#heat_resistance,
                max = 100f32,
                "Field got truncated"
            );
            self.r#heat_resistance = 100f32 as f32;
        }
        if self.r#ship_weight_bonus < (-1f32 as f32) {
            tracing::warn!(
                field = "r#ship_weight_bonus",
                value = self.r#ship_weight_bonus,
                min = -1f32,
                "Field got truncated"
            );
            self.r#ship_weight_bonus = -1f32 as f32;
        }
        if self.r#ship_weight_bonus > (10f32 as f32) {
            tracing::warn!(
                field = "r#ship_weight_bonus",
                value = self.r#ship_weight_bonus,
                max = 10f32,
                "Field got truncated"
            );
            self.r#ship_weight_bonus = 10f32 as f32;
        }
        if self.r#equipment_weight_bonus < (-1f32 as f32) {
            tracing::warn!(
                field = "r#equipment_weight_bonus",
                value = self.r#equipment_weight_bonus,
                min = -1f32,
                "Field got truncated"
            );
            self.r#equipment_weight_bonus = -1f32 as f32;
        }
        if self.r#equipment_weight_bonus > (10f32 as f32) {
            tracing::warn!(
                field = "r#equipment_weight_bonus",
                value = self.r#equipment_weight_bonus,
                max = 10f32,
                "Field got truncated"
            );
            self.r#equipment_weight_bonus = 10f32 as f32;
        }
        if self.r#velocity_bonus < (-1f32 as f32) {
            tracing::warn!(
                field = "r#velocity_bonus",
                value = self.r#velocity_bonus,
                min = -1f32,
                "Field got truncated"
            );
            self.r#velocity_bonus = -1f32 as f32;
        }
        if self.r#velocity_bonus > (10f32 as f32) {
            tracing::warn!(
                field = "r#velocity_bonus",
                value = self.r#velocity_bonus,
                max = 10f32,
                "Field got truncated"
            );
            self.r#velocity_bonus = 10f32 as f32;
        }
        if self.r#turn_rate_bonus < (-1f32 as f32) {
            tracing::warn!(
                field = "r#turn_rate_bonus",
                value = self.r#turn_rate_bonus,
                min = -1f32,
                "Field got truncated"
            );
            self.r#turn_rate_bonus = -1f32 as f32;
        }
        if self.r#turn_rate_bonus > (10f32 as f32) {
            tracing::warn!(
                field = "r#turn_rate_bonus",
                value = self.r#turn_rate_bonus,
                max = 10f32,
                "Field got truncated"
            );
            self.r#turn_rate_bonus = 10f32 as f32;
        }
        if self.r#armor_bonus < (-1f32 as f32) {
            tracing::warn!(
                field = "r#armor_bonus",
                value = self.r#armor_bonus,
                min = -1f32,
                "Field got truncated"
            );
            self.r#armor_bonus = -1f32 as f32;
        }
        if self.r#armor_bonus > (10f32 as f32) {
            tracing::warn!(
                field = "r#armor_bonus",
                value = self.r#armor_bonus,
                max = 10f32,
                "Field got truncated"
            );
            self.r#armor_bonus = 10f32 as f32;
        }
        if self.r#shield_bonus < (-1f32 as f32) {
            tracing::warn!(
                field = "r#shield_bonus",
                value = self.r#shield_bonus,
                min = -1f32,
                "Field got truncated"
            );
            self.r#shield_bonus = -1f32 as f32;
        }
        if self.r#shield_bonus > (10f32 as f32) {
            tracing::warn!(
                field = "r#shield_bonus",
                value = self.r#shield_bonus,
                max = 10f32,
                "Field got truncated"
            );
            self.r#shield_bonus = 10f32 as f32;
        }
        if self.r#energy_bonus < (-1f32 as f32) {
            tracing::warn!(
                field = "r#energy_bonus",
                value = self.r#energy_bonus,
                min = -1f32,
                "Field got truncated"
            );
            self.r#energy_bonus = -1f32 as f32;
        }
        if self.r#energy_bonus > (10f32 as f32) {
            tracing::warn!(
                field = "r#energy_bonus",
                value = self.r#energy_bonus,
                max = 10f32,
                "Field got truncated"
            );
            self.r#energy_bonus = 10f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "ShipFeatures"
    }
}
impl Default for ShipFeatures {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/StatModification.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct StatModification {
    pub r#type: StatModificationType,
    pub r#gray_3: f32,
    pub r#gray_2: f32,
    pub r#gray_1: f32,
    pub r#green: f32,
    pub r#purple: f32,
    pub r#gold: f32,
}
impl StatModification {
    pub fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#gray_3: Default::default(),
            r#gray_2: Default::default(),
            r#gray_1: Default::default(),
            r#green: Default::default(),
            r#purple: Default::default(),
            r#gold: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: impl Into<StatModificationType>) -> Self {
        self.r#type = r#type.into();
        self
    }
    pub fn set_type(&mut self, r#type: impl Into<StatModificationType>) -> &mut Self {
        self.r#type = r#type.into();
        self
    }
    pub fn with_gray_3(mut self, r#gray_3: impl Into<f32>) -> Self {
        self.r#gray_3 = r#gray_3.into();
        self
    }
    pub fn set_gray_3(&mut self, r#gray_3: impl Into<f32>) -> &mut Self {
        self.r#gray_3 = r#gray_3.into();
        self
    }
    pub fn with_gray_2(mut self, r#gray_2: impl Into<f32>) -> Self {
        self.r#gray_2 = r#gray_2.into();
        self
    }
    pub fn set_gray_2(&mut self, r#gray_2: impl Into<f32>) -> &mut Self {
        self.r#gray_2 = r#gray_2.into();
        self
    }
    pub fn with_gray_1(mut self, r#gray_1: impl Into<f32>) -> Self {
        self.r#gray_1 = r#gray_1.into();
        self
    }
    pub fn set_gray_1(&mut self, r#gray_1: impl Into<f32>) -> &mut Self {
        self.r#gray_1 = r#gray_1.into();
        self
    }
    pub fn with_green(mut self, r#green: impl Into<f32>) -> Self {
        self.r#green = r#green.into();
        self
    }
    pub fn set_green(&mut self, r#green: impl Into<f32>) -> &mut Self {
        self.r#green = r#green.into();
        self
    }
    pub fn with_purple(mut self, r#purple: impl Into<f32>) -> Self {
        self.r#purple = r#purple.into();
        self
    }
    pub fn set_purple(&mut self, r#purple: impl Into<f32>) -> &mut Self {
        self.r#purple = r#purple.into();
        self
    }
    pub fn with_gold(mut self, r#gold: impl Into<f32>) -> Self {
        self.r#gold = r#gold.into();
        self
    }
    pub fn set_gold(&mut self, r#gold: impl Into<f32>) -> &mut Self {
        self.r#gold = r#gold.into();
        self
    }
}
impl DatabaseItem for StatModification {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "StatModification"
    }
}
impl Default for StatModification {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletBody.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletBody {
    pub r#size: f32,
    pub r#length: f32,
    pub r#velocity: f32,
    ///How hard is the ammunition affected by the parent velocity during spawn.
    pub r#parent_velocity_effect: f32,
    ///Specifies whenever ammunition is attached to the parent ship or ammo. Moving ammo will move in parent's coordinate space
    pub r#attached_to_parent: bool,
    pub r#range: f32,
    pub r#lifetime: f32,
    pub r#weight: f32,
    pub r#hit_points: i32,
    pub r#color: String,
    pub r#bullet_prefab: Option<BulletPrefabId>,
    pub r#energy_cost: f32,
    pub r#can_be_disarmed: bool,
    pub r#friendly_fire: bool,
    ///Hints for AI and auto-aim0 on usage of this weapon
    pub r#ai_bullet_behavior: AiBulletBehavior,
    pub r#type: BulletTypeObsolete,
}
impl BulletBody {
    pub fn new() -> Self {
        Self {
            r#size: Default::default(),
            r#length: Default::default(),
            r#velocity: Default::default(),
            r#parent_velocity_effect: 1f32,
            r#attached_to_parent: Default::default(),
            r#range: Default::default(),
            r#lifetime: Default::default(),
            r#weight: Default::default(),
            r#hit_points: Default::default(),
            r#color: Default::default(),
            r#bullet_prefab: Default::default(),
            r#energy_cost: Default::default(),
            r#can_be_disarmed: Default::default(),
            r#friendly_fire: Default::default(),
            r#ai_bullet_behavior: Default::default(),
            r#type: Default::default(),
        }
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_length(mut self, r#length: impl Into<f32>) -> Self {
        self.r#length = r#length.into();
        self
    }
    pub fn set_length(&mut self, r#length: impl Into<f32>) -> &mut Self {
        self.r#length = r#length.into();
        self
    }
    pub fn with_velocity(mut self, r#velocity: impl Into<f32>) -> Self {
        self.r#velocity = r#velocity.into();
        self
    }
    pub fn set_velocity(&mut self, r#velocity: impl Into<f32>) -> &mut Self {
        self.r#velocity = r#velocity.into();
        self
    }
    pub fn with_parent_velocity_effect(mut self, r#parent_velocity_effect: impl Into<f32>) -> Self {
        self.r#parent_velocity_effect = r#parent_velocity_effect.into();
        self
    }
    pub fn set_parent_velocity_effect(
        &mut self,
        r#parent_velocity_effect: impl Into<f32>,
    ) -> &mut Self {
        self.r#parent_velocity_effect = r#parent_velocity_effect.into();
        self
    }
    pub fn with_attached_to_parent(mut self, r#attached_to_parent: impl Into<bool>) -> Self {
        self.r#attached_to_parent = r#attached_to_parent.into();
        self
    }
    pub fn set_attached_to_parent(&mut self, r#attached_to_parent: impl Into<bool>) -> &mut Self {
        self.r#attached_to_parent = r#attached_to_parent.into();
        self
    }
    pub fn with_range(mut self, r#range: impl Into<f32>) -> Self {
        self.r#range = r#range.into();
        self
    }
    pub fn set_range(&mut self, r#range: impl Into<f32>) -> &mut Self {
        self.r#range = r#range.into();
        self
    }
    pub fn with_lifetime(mut self, r#lifetime: impl Into<f32>) -> Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn set_lifetime(&mut self, r#lifetime: impl Into<f32>) -> &mut Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn with_weight(mut self, r#weight: impl Into<f32>) -> Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn set_weight(&mut self, r#weight: impl Into<f32>) -> &mut Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn with_hit_points(mut self, r#hit_points: impl Into<i32>) -> Self {
        self.r#hit_points = r#hit_points.into();
        self
    }
    pub fn set_hit_points(&mut self, r#hit_points: impl Into<i32>) -> &mut Self {
        self.r#hit_points = r#hit_points.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_bullet_prefab(
        mut self,
        r#bullet_prefab: impl Into<Option<BulletPrefabId>>,
    ) -> Self {
        self.r#bullet_prefab = r#bullet_prefab.into();
        self
    }
    pub fn set_bullet_prefab(
        &mut self,
        r#bullet_prefab: impl Into<Option<BulletPrefabId>>,
    ) -> &mut Self {
        self.r#bullet_prefab = r#bullet_prefab.into();
        self
    }
    pub fn with_energy_cost(mut self, r#energy_cost: impl Into<f32>) -> Self {
        self.r#energy_cost = r#energy_cost.into();
        self
    }
    pub fn set_energy_cost(&mut self, r#energy_cost: impl Into<f32>) -> &mut Self {
        self.r#energy_cost = r#energy_cost.into();
        self
    }
    pub fn with_can_be_disarmed(mut self, r#can_be_disarmed: impl Into<bool>) -> Self {
        self.r#can_be_disarmed = r#can_be_disarmed.into();
        self
    }
    pub fn set_can_be_disarmed(&mut self, r#can_be_disarmed: impl Into<bool>) -> &mut Self {
        self.r#can_be_disarmed = r#can_be_disarmed.into();
        self
    }
    pub fn with_friendly_fire(mut self, r#friendly_fire: impl Into<bool>) -> Self {
        self.r#friendly_fire = r#friendly_fire.into();
        self
    }
    pub fn set_friendly_fire(&mut self, r#friendly_fire: impl Into<bool>) -> &mut Self {
        self.r#friendly_fire = r#friendly_fire.into();
        self
    }
    pub fn with_ai_bullet_behavior(
        mut self,
        r#ai_bullet_behavior: impl Into<AiBulletBehavior>,
    ) -> Self {
        self.r#ai_bullet_behavior = r#ai_bullet_behavior.into();
        self
    }
    pub fn set_ai_bullet_behavior(
        &mut self,
        r#ai_bullet_behavior: impl Into<AiBulletBehavior>,
    ) -> &mut Self {
        self.r#ai_bullet_behavior = r#ai_bullet_behavior.into();
        self
    }
    pub fn with_type(mut self, r#type: impl Into<BulletTypeObsolete>) -> Self {
        self.r#type = r#type.into();
        self
    }
    pub fn set_type(&mut self, r#type: impl Into<BulletTypeObsolete>) -> &mut Self {
        self.r#type = r#type.into();
        self
    }
}
impl DatabaseItem for BulletBody {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (1000f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#size = 1000f32 as f32;
        }
        if self.r#length < (0f32 as f32) {
            tracing::warn!(
                field = "r#length",
                value = self.r#length,
                min = 0f32,
                "Field got truncated"
            );
            self.r#length = 0f32 as f32;
        }
        if self.r#length > (1000f32 as f32) {
            tracing::warn!(
                field = "r#length",
                value = self.r#length,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#length = 1000f32 as f32;
        }
        if self.r#velocity < (0f32 as f32) {
            tracing::warn!(
                field = "r#velocity",
                value = self.r#velocity,
                min = 0f32,
                "Field got truncated"
            );
            self.r#velocity = 0f32 as f32;
        }
        if self.r#velocity > (1000f32 as f32) {
            tracing::warn!(
                field = "r#velocity",
                value = self.r#velocity,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#velocity = 1000f32 as f32;
        }
        if self.r#parent_velocity_effect < (-1000f32 as f32) {
            tracing::warn!(
                field = "r#parent_velocity_effect",
                value = self.r#parent_velocity_effect,
                min = -1000f32,
                "Field got truncated"
            );
            self.r#parent_velocity_effect = -1000f32 as f32;
        }
        if self.r#parent_velocity_effect > (1000f32 as f32) {
            tracing::warn!(
                field = "r#parent_velocity_effect",
                value = self.r#parent_velocity_effect,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#parent_velocity_effect = 1000f32 as f32;
        }
        if self.r#range < (0f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                min = 0f32,
                "Field got truncated"
            );
            self.r#range = 0f32 as f32;
        }
        if self.r#range > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#range = 1000000000f32 as f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                min = 0f32,
                "Field got truncated"
            );
            self.r#lifetime = 0f32 as f32;
        }
        if self.r#lifetime > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#lifetime = 1000000000f32 as f32;
        }
        if self.r#weight < (0f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                min = 0f32,
                "Field got truncated"
            );
            self.r#weight = 0f32 as f32;
        }
        if self.r#weight > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#weight = 1000000000f32 as f32;
        }
        if self.r#hit_points < (0f32 as i32) {
            tracing::warn!(
                field = "r#hit_points",
                value = self.r#hit_points,
                min = 0f32,
                "Field got truncated"
            );
            self.r#hit_points = 0f32 as i32;
        }
        if self.r#hit_points > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#hit_points",
                value = self.r#hit_points,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#hit_points = 1000000000f32 as i32;
        }
        if self.r#energy_cost < (0f32 as f32) {
            tracing::warn!(
                field = "r#energy_cost",
                value = self.r#energy_cost,
                min = 0f32,
                "Field got truncated"
            );
            self.r#energy_cost = 0f32 as f32;
        }
        if self.r#energy_cost > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_cost",
                value = self.r#energy_cost,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#energy_cost = 1000000000f32 as f32;
        }
        let dw: BulletTypeObsolete = Default::default();
        if self.r#type != dw {
            tracing::error!(
                ield = "r#type",
                "Obsolete field usage detected, generated code may not work",
            );
        }
    }
    fn type_name() -> &'static str {
        "BulletBody"
    }
}
impl Default for BulletBody {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletController.xml
#[derive(Debug, Clone)]
pub enum BulletController {
    Projectile(BulletControllerProjectile),
    Homing(BulletControllerHoming),
    Beam(BulletControllerBeam),
    Parametric(BulletControllerParametric),
}
impl Default for BulletController {
    fn default() -> Self {
        Self::Projectile(Default::default())
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletControllerProjectile {}
impl BulletControllerProjectile {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BulletControllerProjectile {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletControllerProjectile"
    }
}
impl Default for BulletControllerProjectile {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletControllerProjectile> for BulletController {
    fn from(item: BulletControllerProjectile) -> Self {
        Self::Projectile(item)
    }
}
impl BulletControllerProjectile {
    pub fn wrap(self) -> BulletController {
        self.into()
    }
}
impl BulletController {
    pub fn projectile() -> BulletControllerProjectile {
        BulletControllerProjectile::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletControllerHoming {
    pub r#starting_velocity_modifier: f32,
    pub r#ignore_rotation: bool,
    pub r#smart_aim: bool,
}
impl BulletControllerHoming {
    pub fn new() -> Self {
        Self {
            r#starting_velocity_modifier: 1f32,
            r#ignore_rotation: Default::default(),
            r#smart_aim: Default::default(),
        }
    }
    pub fn with_starting_velocity_modifier(
        mut self,
        r#starting_velocity_modifier: impl Into<f32>,
    ) -> Self {
        self.r#starting_velocity_modifier = r#starting_velocity_modifier.into();
        self
    }
    pub fn set_starting_velocity_modifier(
        &mut self,
        r#starting_velocity_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#starting_velocity_modifier = r#starting_velocity_modifier.into();
        self
    }
    pub fn with_ignore_rotation(mut self, r#ignore_rotation: impl Into<bool>) -> Self {
        self.r#ignore_rotation = r#ignore_rotation.into();
        self
    }
    pub fn set_ignore_rotation(&mut self, r#ignore_rotation: impl Into<bool>) -> &mut Self {
        self.r#ignore_rotation = r#ignore_rotation.into();
        self
    }
    pub fn with_smart_aim(mut self, r#smart_aim: impl Into<bool>) -> Self {
        self.r#smart_aim = r#smart_aim.into();
        self
    }
    pub fn set_smart_aim(&mut self, r#smart_aim: impl Into<bool>) -> &mut Self {
        self.r#smart_aim = r#smart_aim.into();
        self
    }
}
impl DatabaseItem for BulletControllerHoming {
    fn validate(&mut self) {
        if self.r#starting_velocity_modifier < (0f32 as f32) {
            tracing::warn!(
                field = "r#starting_velocity_modifier",
                value = self.r#starting_velocity_modifier,
                min = 0f32,
                "Field got truncated"
            );
            self.r#starting_velocity_modifier = 0f32 as f32;
        }
        if self.r#starting_velocity_modifier > (1000f32 as f32) {
            tracing::warn!(
                field = "r#starting_velocity_modifier",
                value = self.r#starting_velocity_modifier,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#starting_velocity_modifier = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BulletControllerHoming"
    }
}
impl Default for BulletControllerHoming {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletControllerHoming> for BulletController {
    fn from(item: BulletControllerHoming) -> Self {
        Self::Homing(item)
    }
}
impl BulletControllerHoming {
    pub fn wrap(self) -> BulletController {
        self.into()
    }
}
impl BulletController {
    pub fn homing() -> BulletControllerHoming {
        BulletControllerHoming::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletControllerBeam {}
impl BulletControllerBeam {
    pub fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BulletControllerBeam {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletControllerBeam"
    }
}
impl Default for BulletControllerBeam {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletControllerBeam> for BulletController {
    fn from(item: BulletControllerBeam) -> Self {
        Self::Beam(item)
    }
}
impl BulletControllerBeam {
    pub fn wrap(self) -> BulletController {
        self.into()
    }
}
impl BulletController {
    pub fn beam() -> BulletControllerBeam {
        BulletControllerBeam::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletControllerParametric {
    pub r#x: String,
    pub r#y: String,
    pub r#rotation: String,
    pub r#size: String,
    pub r#length: String,
}
impl BulletControllerParametric {
    pub fn new() -> Self {
        Self {
            r#x: "0".to_string(),
            r#y: "0".to_string(),
            r#rotation: "0".to_string(),
            r#size: "1".to_string(),
            r#length: "1".to_string(),
        }
    }
    pub fn with_x(mut self, r#x: impl Into<String>) -> Self {
        self.r#x = r#x.into();
        self
    }
    pub fn set_x(&mut self, r#x: impl Into<String>) -> &mut Self {
        self.r#x = r#x.into();
        self
    }
    pub fn with_y(mut self, r#y: impl Into<String>) -> Self {
        self.r#y = r#y.into();
        self
    }
    pub fn set_y(&mut self, r#y: impl Into<String>) -> &mut Self {
        self.r#y = r#y.into();
        self
    }
    pub fn with_rotation(mut self, r#rotation: impl Into<String>) -> Self {
        self.r#rotation = r#rotation.into();
        self
    }
    pub fn set_rotation(&mut self, r#rotation: impl Into<String>) -> &mut Self {
        self.r#rotation = r#rotation.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<String>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<String>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_length(mut self, r#length: impl Into<String>) -> Self {
        self.r#length = r#length.into();
        self
    }
    pub fn set_length(&mut self, r#length: impl Into<String>) -> &mut Self {
        self.r#length = r#length.into();
        self
    }
}
impl DatabaseItem for BulletControllerParametric {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BulletControllerParametric"
    }
}
impl Default for BulletControllerParametric {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletControllerParametric> for BulletController {
    fn from(item: BulletControllerParametric) -> Self {
        Self::Parametric(item)
    }
}
impl BulletControllerParametric {
    pub fn wrap(self) -> BulletController {
        self.into()
    }
}
impl BulletController {
    pub fn parametric() -> BulletControllerParametric {
        BulletControllerParametric::new()
    }
}
impl serde::Serialize for BulletController {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "BulletController")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: BulletControllerType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Projectile(x) => AdjTagged {
                t: BulletControllerType::Projectile,
                c: x,
            }
            .serialize(serializer),
            Self::Homing(x) => AdjTagged {
                t: BulletControllerType::Homing,
                c: x,
            }
            .serialize(serializer),
            Self::Beam(x) => AdjTagged {
                t: BulletControllerType::Beam,
                c: x,
            }
            .serialize(serializer),
            Self::Parametric(x) => AdjTagged {
                t: BulletControllerType::Parametric,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl DatabaseItem for BulletController {
    fn validate(&mut self) {
        match self {
            Self::Projectile(x) => x.validate(),
            Self::Homing(x) => x.validate(),
            Self::Beam(x) => x.validate(),
            Self::Parametric(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "BulletController"
    }
}
impl BulletController {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Projectile(_) => BulletControllerProjectile::type_name(),
            Self::Homing(_) => BulletControllerHoming::type_name(),
            Self::Beam(_) => BulletControllerBeam::type_name(),
            Self::Parametric(_) => BulletControllerParametric::type_name(),
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletTrigger.xml
#[derive(Debug, Clone)]
pub enum BulletTrigger {
    None(BulletTriggerNone),
    PlaySfx(BulletTriggerPlaySfx),
    SpawnBullet(BulletTriggerSpawnBullet),
    Detonate(BulletTriggerDetonate),
    SpawnStaticSfx(BulletTriggerSpawnStaticSfx),
    GravityField(BulletTriggerGravityField),
}
impl Default for BulletTrigger {
    fn default() -> Self {
        Self::None(Default::default())
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletTriggerNone {
    pub r#condition: BulletTriggerCondition,
    pub r#cooldown: f32,
}
impl BulletTriggerNone {
    pub fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: impl Into<BulletTriggerCondition>) -> Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn set_condition(&mut self, r#condition: impl Into<BulletTriggerCondition>) -> &mut Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BulletTriggerNone {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#cooldown = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BulletTriggerNone"
    }
}
impl Default for BulletTriggerNone {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletTriggerNone> for BulletTrigger {
    fn from(item: BulletTriggerNone) -> Self {
        Self::None(item)
    }
}
impl BulletTriggerNone {
    pub fn wrap(self) -> BulletTrigger {
        self.into()
    }
}
impl BulletTrigger {
    pub fn none() -> BulletTriggerNone {
        BulletTriggerNone::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletTriggerPlaySfx {
    pub r#condition: BulletTriggerCondition,
    pub r#visual_effect: Option<VisualEffectId>,
    pub r#audio_clip: String,
    pub r#color: String,
    pub r#color_mode: ColorMode,
    pub r#size: f32,
    pub r#lifetime: f32,
    pub r#cooldown: f32,
}
impl BulletTriggerPlaySfx {
    pub fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#visual_effect: Default::default(),
            r#audio_clip: Default::default(),
            r#color: Default::default(),
            r#color_mode: Default::default(),
            r#size: Default::default(),
            r#lifetime: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: impl Into<BulletTriggerCondition>) -> Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn set_condition(&mut self, r#condition: impl Into<BulletTriggerCondition>) -> &mut Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn with_visual_effect(
        mut self,
        r#visual_effect: impl Into<Option<VisualEffectId>>,
    ) -> Self {
        self.r#visual_effect = r#visual_effect.into();
        self
    }
    pub fn set_visual_effect(
        &mut self,
        r#visual_effect: impl Into<Option<VisualEffectId>>,
    ) -> &mut Self {
        self.r#visual_effect = r#visual_effect.into();
        self
    }
    pub fn with_audio_clip(mut self, r#audio_clip: impl Into<String>) -> Self {
        self.r#audio_clip = r#audio_clip.into();
        self
    }
    pub fn set_audio_clip(&mut self, r#audio_clip: impl Into<String>) -> &mut Self {
        self.r#audio_clip = r#audio_clip.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_color_mode(mut self, r#color_mode: impl Into<ColorMode>) -> Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn set_color_mode(&mut self, r#color_mode: impl Into<ColorMode>) -> &mut Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_lifetime(mut self, r#lifetime: impl Into<f32>) -> Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn set_lifetime(&mut self, r#lifetime: impl Into<f32>) -> &mut Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BulletTriggerPlaySfx {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#size = 100f32 as f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                min = 0f32,
                "Field got truncated"
            );
            self.r#lifetime = 0f32 as f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#lifetime = 1000f32 as f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#cooldown = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BulletTriggerPlaySfx"
    }
}
impl Default for BulletTriggerPlaySfx {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletTriggerPlaySfx> for BulletTrigger {
    fn from(item: BulletTriggerPlaySfx) -> Self {
        Self::PlaySfx(item)
    }
}
impl BulletTriggerPlaySfx {
    pub fn wrap(self) -> BulletTrigger {
        self.into()
    }
}
impl BulletTrigger {
    pub fn play_sfx() -> BulletTriggerPlaySfx {
        BulletTriggerPlaySfx::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletTriggerSpawnBullet {
    pub r#condition: BulletTriggerCondition,
    pub r#audio_clip: String,
    pub r#ammunition: Option<AmmunitionId>,
    pub r#color: String,
    pub r#color_mode: ColorMode,
    pub r#quantity: i32,
    pub r#size: f32,
    pub r#cooldown: f32,
    pub r#random_factor: f32,
    pub r#power_multiplier: f32,
    pub r#max_nesting_level: i32,
    pub r#rotation: String,
    pub r#offset_x: String,
    pub r#offset_y: String,
}
impl BulletTriggerSpawnBullet {
    pub fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#audio_clip: Default::default(),
            r#ammunition: Default::default(),
            r#color: Default::default(),
            r#color_mode: Default::default(),
            r#quantity: Default::default(),
            r#size: Default::default(),
            r#cooldown: Default::default(),
            r#random_factor: Default::default(),
            r#power_multiplier: Default::default(),
            r#max_nesting_level: Default::default(),
            r#rotation: "IF(Quantity == 1, 0, RANDOM(0, 360))".to_string(),
            r#offset_x: "IF(Quantity == 1, 0, Size / 2)".to_string(),
            r#offset_y: "0".to_string(),
        }
    }
    pub fn with_condition(mut self, r#condition: impl Into<BulletTriggerCondition>) -> Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn set_condition(&mut self, r#condition: impl Into<BulletTriggerCondition>) -> &mut Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn with_audio_clip(mut self, r#audio_clip: impl Into<String>) -> Self {
        self.r#audio_clip = r#audio_clip.into();
        self
    }
    pub fn set_audio_clip(&mut self, r#audio_clip: impl Into<String>) -> &mut Self {
        self.r#audio_clip = r#audio_clip.into();
        self
    }
    pub fn with_ammunition(mut self, r#ammunition: impl Into<Option<AmmunitionId>>) -> Self {
        self.r#ammunition = r#ammunition.into();
        self
    }
    pub fn set_ammunition(&mut self, r#ammunition: impl Into<Option<AmmunitionId>>) -> &mut Self {
        self.r#ammunition = r#ammunition.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_color_mode(mut self, r#color_mode: impl Into<ColorMode>) -> Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn set_color_mode(&mut self, r#color_mode: impl Into<ColorMode>) -> &mut Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn with_quantity(mut self, r#quantity: impl Into<i32>) -> Self {
        self.r#quantity = r#quantity.into();
        self
    }
    pub fn set_quantity(&mut self, r#quantity: impl Into<i32>) -> &mut Self {
        self.r#quantity = r#quantity.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn with_random_factor(mut self, r#random_factor: impl Into<f32>) -> Self {
        self.r#random_factor = r#random_factor.into();
        self
    }
    pub fn set_random_factor(&mut self, r#random_factor: impl Into<f32>) -> &mut Self {
        self.r#random_factor = r#random_factor.into();
        self
    }
    pub fn with_power_multiplier(mut self, r#power_multiplier: impl Into<f32>) -> Self {
        self.r#power_multiplier = r#power_multiplier.into();
        self
    }
    pub fn set_power_multiplier(&mut self, r#power_multiplier: impl Into<f32>) -> &mut Self {
        self.r#power_multiplier = r#power_multiplier.into();
        self
    }
    pub fn with_max_nesting_level(mut self, r#max_nesting_level: impl Into<i32>) -> Self {
        self.r#max_nesting_level = r#max_nesting_level.into();
        self
    }
    pub fn set_max_nesting_level(&mut self, r#max_nesting_level: impl Into<i32>) -> &mut Self {
        self.r#max_nesting_level = r#max_nesting_level.into();
        self
    }
    pub fn with_rotation(mut self, r#rotation: impl Into<String>) -> Self {
        self.r#rotation = r#rotation.into();
        self
    }
    pub fn set_rotation(&mut self, r#rotation: impl Into<String>) -> &mut Self {
        self.r#rotation = r#rotation.into();
        self
    }
    pub fn with_offset_x(mut self, r#offset_x: impl Into<String>) -> Self {
        self.r#offset_x = r#offset_x.into();
        self
    }
    pub fn set_offset_x(&mut self, r#offset_x: impl Into<String>) -> &mut Self {
        self.r#offset_x = r#offset_x.into();
        self
    }
    pub fn with_offset_y(mut self, r#offset_y: impl Into<String>) -> Self {
        self.r#offset_y = r#offset_y.into();
        self
    }
    pub fn set_offset_y(&mut self, r#offset_y: impl Into<String>) -> &mut Self {
        self.r#offset_y = r#offset_y.into();
        self
    }
}
impl DatabaseItem for BulletTriggerSpawnBullet {
    fn validate(&mut self) {
        if self.r#quantity < (0f32 as i32) {
            tracing::warn!(
                field = "r#quantity",
                value = self.r#quantity,
                min = 0f32,
                "Field got truncated"
            );
            self.r#quantity = 0f32 as i32;
        }
        if self.r#quantity > (1000f32 as i32) {
            tracing::warn!(
                field = "r#quantity",
                value = self.r#quantity,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#quantity = 1000f32 as i32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#size = 100f32 as f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#cooldown = 1000f32 as f32;
        }
        if self.r#random_factor < (0f32 as f32) {
            tracing::warn!(
                field = "r#random_factor",
                value = self.r#random_factor,
                min = 0f32,
                "Field got truncated"
            );
            self.r#random_factor = 0f32 as f32;
        }
        if self.r#random_factor > (1f32 as f32) {
            tracing::warn!(
                field = "r#random_factor",
                value = self.r#random_factor,
                max = 1f32,
                "Field got truncated"
            );
            self.r#random_factor = 1f32 as f32;
        }
        if self.r#power_multiplier < (0f32 as f32) {
            tracing::warn!(
                field = "r#power_multiplier",
                value = self.r#power_multiplier,
                min = 0f32,
                "Field got truncated"
            );
            self.r#power_multiplier = 0f32 as f32;
        }
        if self.r#max_nesting_level < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_nesting_level",
                value = self.r#max_nesting_level,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_nesting_level = 0f32 as i32;
        }
        if self.r#max_nesting_level > (100f32 as i32) {
            tracing::warn!(
                field = "r#max_nesting_level",
                value = self.r#max_nesting_level,
                max = 100f32,
                "Field got truncated"
            );
            self.r#max_nesting_level = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "BulletTriggerSpawnBullet"
    }
}
impl Default for BulletTriggerSpawnBullet {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletTriggerSpawnBullet> for BulletTrigger {
    fn from(item: BulletTriggerSpawnBullet) -> Self {
        Self::SpawnBullet(item)
    }
}
impl BulletTriggerSpawnBullet {
    pub fn wrap(self) -> BulletTrigger {
        self.into()
    }
}
impl BulletTrigger {
    pub fn spawn_bullet() -> BulletTriggerSpawnBullet {
        BulletTriggerSpawnBullet::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletTriggerDetonate {
    pub r#condition: BulletTriggerCondition,
    pub r#cooldown: f32,
}
impl BulletTriggerDetonate {
    pub fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: impl Into<BulletTriggerCondition>) -> Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn set_condition(&mut self, r#condition: impl Into<BulletTriggerCondition>) -> &mut Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BulletTriggerDetonate {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#cooldown = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BulletTriggerDetonate"
    }
}
impl Default for BulletTriggerDetonate {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletTriggerDetonate> for BulletTrigger {
    fn from(item: BulletTriggerDetonate) -> Self {
        Self::Detonate(item)
    }
}
impl BulletTriggerDetonate {
    pub fn wrap(self) -> BulletTrigger {
        self.into()
    }
}
impl BulletTrigger {
    pub fn detonate() -> BulletTriggerDetonate {
        BulletTriggerDetonate::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletTriggerSpawnStaticSfx {
    pub r#condition: BulletTriggerCondition,
    pub r#visual_effect: Option<VisualEffectId>,
    pub r#audio_clip: String,
    pub r#color: String,
    pub r#color_mode: ColorMode,
    pub r#size: f32,
    pub r#lifetime: f32,
    pub r#cooldown: f32,
}
impl BulletTriggerSpawnStaticSfx {
    pub fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#visual_effect: Default::default(),
            r#audio_clip: Default::default(),
            r#color: Default::default(),
            r#color_mode: Default::default(),
            r#size: Default::default(),
            r#lifetime: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: impl Into<BulletTriggerCondition>) -> Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn set_condition(&mut self, r#condition: impl Into<BulletTriggerCondition>) -> &mut Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn with_visual_effect(
        mut self,
        r#visual_effect: impl Into<Option<VisualEffectId>>,
    ) -> Self {
        self.r#visual_effect = r#visual_effect.into();
        self
    }
    pub fn set_visual_effect(
        &mut self,
        r#visual_effect: impl Into<Option<VisualEffectId>>,
    ) -> &mut Self {
        self.r#visual_effect = r#visual_effect.into();
        self
    }
    pub fn with_audio_clip(mut self, r#audio_clip: impl Into<String>) -> Self {
        self.r#audio_clip = r#audio_clip.into();
        self
    }
    pub fn set_audio_clip(&mut self, r#audio_clip: impl Into<String>) -> &mut Self {
        self.r#audio_clip = r#audio_clip.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_color_mode(mut self, r#color_mode: impl Into<ColorMode>) -> Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn set_color_mode(&mut self, r#color_mode: impl Into<ColorMode>) -> &mut Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_lifetime(mut self, r#lifetime: impl Into<f32>) -> Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn set_lifetime(&mut self, r#lifetime: impl Into<f32>) -> &mut Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
}
impl DatabaseItem for BulletTriggerSpawnStaticSfx {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#size = 100f32 as f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                min = 0f32,
                "Field got truncated"
            );
            self.r#lifetime = 0f32 as f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#lifetime = 1000f32 as f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#cooldown = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BulletTriggerSpawnStaticSfx"
    }
}
impl Default for BulletTriggerSpawnStaticSfx {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletTriggerSpawnStaticSfx> for BulletTrigger {
    fn from(item: BulletTriggerSpawnStaticSfx) -> Self {
        Self::SpawnStaticSfx(item)
    }
}
impl BulletTriggerSpawnStaticSfx {
    pub fn wrap(self) -> BulletTrigger {
        self.into()
    }
}
impl BulletTrigger {
    pub fn spawn_static_sfx() -> BulletTriggerSpawnStaticSfx {
        BulletTriggerSpawnStaticSfx::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletTriggerGravityField {
    pub r#condition: BulletTriggerCondition,
    pub r#size: f32,
    pub r#cooldown: f32,
    pub r#power_multiplier: f32,
}
impl BulletTriggerGravityField {
    pub fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#size: Default::default(),
            r#cooldown: Default::default(),
            r#power_multiplier: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: impl Into<BulletTriggerCondition>) -> Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn set_condition(&mut self, r#condition: impl Into<BulletTriggerCondition>) -> &mut Self {
        self.r#condition = r#condition.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn with_power_multiplier(mut self, r#power_multiplier: impl Into<f32>) -> Self {
        self.r#power_multiplier = r#power_multiplier.into();
        self
    }
    pub fn set_power_multiplier(&mut self, r#power_multiplier: impl Into<f32>) -> &mut Self {
        self.r#power_multiplier = r#power_multiplier.into();
        self
    }
}
impl DatabaseItem for BulletTriggerGravityField {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#size = 100f32 as f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#cooldown = 1000f32 as f32;
        }
        if self.r#power_multiplier < (0f32 as f32) {
            tracing::warn!(
                field = "r#power_multiplier",
                value = self.r#power_multiplier,
                min = 0f32,
                "Field got truncated"
            );
            self.r#power_multiplier = 0f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BulletTriggerGravityField"
    }
}
impl Default for BulletTriggerGravityField {
    fn default() -> Self {
        Self::new()
    }
}
impl From<BulletTriggerGravityField> for BulletTrigger {
    fn from(item: BulletTriggerGravityField) -> Self {
        Self::GravityField(item)
    }
}
impl BulletTriggerGravityField {
    pub fn wrap(self) -> BulletTrigger {
        self.into()
    }
}
impl BulletTrigger {
    pub fn gravity_field() -> BulletTriggerGravityField {
        BulletTriggerGravityField::new()
    }
}
impl serde::Serialize for BulletTrigger {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "BulletTrigger")]
        struct AdjTagged<T> {
            #[serde(rename = "EffectType")]
            t: BulletEffectType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::None(x) => AdjTagged {
                t: BulletEffectType::None,
                c: x,
            }
            .serialize(serializer),
            Self::PlaySfx(x) => AdjTagged {
                t: BulletEffectType::PlaySfx,
                c: x,
            }
            .serialize(serializer),
            Self::SpawnBullet(x) => AdjTagged {
                t: BulletEffectType::SpawnBullet,
                c: x,
            }
            .serialize(serializer),
            Self::Detonate(x) => AdjTagged {
                t: BulletEffectType::Detonate,
                c: x,
            }
            .serialize(serializer),
            Self::SpawnStaticSfx(x) => AdjTagged {
                t: BulletEffectType::SpawnStaticSfx,
                c: x,
            }
            .serialize(serializer),
            Self::GravityField(x) => AdjTagged {
                t: BulletEffectType::GravityField,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl BulletTrigger {
    pub fn r#condition(&self) -> &BulletTriggerCondition {
        match self {
            Self::None(x) => &x.r#condition,
            Self::PlaySfx(x) => &x.r#condition,
            Self::SpawnBullet(x) => &x.r#condition,
            Self::Detonate(x) => &x.r#condition,
            Self::SpawnStaticSfx(x) => &x.r#condition,
            Self::GravityField(x) => &x.r#condition,
        }
    }
    pub fn condition_mut(&mut self) -> &mut BulletTriggerCondition {
        match self {
            Self::None(x) => &mut x.r#condition,
            Self::PlaySfx(x) => &mut x.r#condition,
            Self::SpawnBullet(x) => &mut x.r#condition,
            Self::Detonate(x) => &mut x.r#condition,
            Self::SpawnStaticSfx(x) => &mut x.r#condition,
            Self::GravityField(x) => &mut x.r#condition,
        }
    }
}
impl BulletTrigger {
    pub fn r#cooldown(&self) -> &f32 {
        match self {
            Self::None(x) => &x.r#cooldown,
            Self::PlaySfx(x) => &x.r#cooldown,
            Self::SpawnBullet(x) => &x.r#cooldown,
            Self::Detonate(x) => &x.r#cooldown,
            Self::SpawnStaticSfx(x) => &x.r#cooldown,
            Self::GravityField(x) => &x.r#cooldown,
        }
    }
    pub fn cooldown_mut(&mut self) -> &mut f32 {
        match self {
            Self::None(x) => &mut x.r#cooldown,
            Self::PlaySfx(x) => &mut x.r#cooldown,
            Self::SpawnBullet(x) => &mut x.r#cooldown,
            Self::Detonate(x) => &mut x.r#cooldown,
            Self::SpawnStaticSfx(x) => &mut x.r#cooldown,
            Self::GravityField(x) => &mut x.r#cooldown,
        }
    }
}
impl DatabaseItem for BulletTrigger {
    fn validate(&mut self) {
        match self {
            Self::None(x) => x.validate(),
            Self::PlaySfx(x) => x.validate(),
            Self::SpawnBullet(x) => x.validate(),
            Self::Detonate(x) => x.validate(),
            Self::SpawnStaticSfx(x) => x.validate(),
            Self::GravityField(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "BulletTrigger"
    }
}
impl BulletTrigger {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::None(_) => BulletTriggerNone::type_name(),
            Self::PlaySfx(_) => BulletTriggerPlaySfx::type_name(),
            Self::SpawnBullet(_) => BulletTriggerSpawnBullet::type_name(),
            Self::Detonate(_) => BulletTriggerDetonate::type_name(),
            Self::SpawnStaticSfx(_) => BulletTriggerSpawnStaticSfx::type_name(),
            Self::GravityField(_) => BulletTriggerGravityField::type_name(),
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/ImpactEffect.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImpactEffect {
    pub r#type: ImpactEffectType,
    pub r#damage_type: DamageType,
    pub r#power: f32,
    pub r#factor: f32,
}
impl ImpactEffect {
    pub fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#damage_type: Default::default(),
            r#power: Default::default(),
            r#factor: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: impl Into<ImpactEffectType>) -> Self {
        self.r#type = r#type.into();
        self
    }
    pub fn set_type(&mut self, r#type: impl Into<ImpactEffectType>) -> &mut Self {
        self.r#type = r#type.into();
        self
    }
    pub fn with_damage_type(mut self, r#damage_type: impl Into<DamageType>) -> Self {
        self.r#damage_type = r#damage_type.into();
        self
    }
    pub fn set_damage_type(&mut self, r#damage_type: impl Into<DamageType>) -> &mut Self {
        self.r#damage_type = r#damage_type.into();
        self
    }
    pub fn with_power(mut self, r#power: impl Into<f32>) -> Self {
        self.r#power = r#power.into();
        self
    }
    pub fn set_power(&mut self, r#power: impl Into<f32>) -> &mut Self {
        self.r#power = r#power.into();
        self
    }
    pub fn with_factor(mut self, r#factor: impl Into<f32>) -> Self {
        self.r#factor = r#factor.into();
        self
    }
    pub fn set_factor(&mut self, r#factor: impl Into<f32>) -> &mut Self {
        self.r#factor = r#factor.into();
        self
    }
}
impl DatabaseItem for ImpactEffect {
    fn validate(&mut self) {
        if self.r#power < (0f32 as f32) {
            tracing::warn!(
                field = "r#power",
                value = self.r#power,
                min = 0f32,
                "Field got truncated"
            );
            self.r#power = 0f32 as f32;
        }
        if self.r#power > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#power",
                value = self.r#power,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#power = 1000000000f32 as f32;
        }
        if self.r#factor < (0f32 as f32) {
            tracing::warn!(
                field = "r#factor",
                value = self.r#factor,
                min = 0f32,
                "Field got truncated"
            );
            self.r#factor = 0f32 as f32;
        }
        if self.r#factor > (1f32 as f32) {
            tracing::warn!(
                field = "r#factor",
                value = self.r#factor,
                max = 1f32,
                "Field got truncated"
            );
            self.r#factor = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "ImpactEffect"
    }
}
impl Default for ImpactEffect {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/VisualEffectElement.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VisualEffectElement {
    pub r#type: VisualEffectType,
    pub r#image: String,
    pub r#color_mode: ColorMode,
    pub r#color: String,
    pub r#quantity: i32,
    pub r#size: f32,
    pub r#growth_rate: f32,
    pub r#turn_rate: f32,
    pub r#start_time: f32,
    pub r#lifetime: f32,
    pub r#particle_size: f32,
    pub r#loop: bool,
}
impl VisualEffectElement {
    pub fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#image: Default::default(),
            r#color_mode: Default::default(),
            r#color: Default::default(),
            r#quantity: 1i32,
            r#size: 1f32,
            r#growth_rate: Default::default(),
            r#turn_rate: Default::default(),
            r#start_time: Default::default(),
            r#lifetime: 1f32,
            r#particle_size: 1f32,
            r#loop: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: impl Into<VisualEffectType>) -> Self {
        self.r#type = r#type.into();
        self
    }
    pub fn set_type(&mut self, r#type: impl Into<VisualEffectType>) -> &mut Self {
        self.r#type = r#type.into();
        self
    }
    pub fn with_image(mut self, r#image: impl Into<String>) -> Self {
        self.r#image = r#image.into();
        self
    }
    pub fn set_image(&mut self, r#image: impl Into<String>) -> &mut Self {
        self.r#image = r#image.into();
        self
    }
    pub fn with_color_mode(mut self, r#color_mode: impl Into<ColorMode>) -> Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn set_color_mode(&mut self, r#color_mode: impl Into<ColorMode>) -> &mut Self {
        self.r#color_mode = r#color_mode.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_quantity(mut self, r#quantity: impl Into<i32>) -> Self {
        self.r#quantity = r#quantity.into();
        self
    }
    pub fn set_quantity(&mut self, r#quantity: impl Into<i32>) -> &mut Self {
        self.r#quantity = r#quantity.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_growth_rate(mut self, r#growth_rate: impl Into<f32>) -> Self {
        self.r#growth_rate = r#growth_rate.into();
        self
    }
    pub fn set_growth_rate(&mut self, r#growth_rate: impl Into<f32>) -> &mut Self {
        self.r#growth_rate = r#growth_rate.into();
        self
    }
    pub fn with_turn_rate(mut self, r#turn_rate: impl Into<f32>) -> Self {
        self.r#turn_rate = r#turn_rate.into();
        self
    }
    pub fn set_turn_rate(&mut self, r#turn_rate: impl Into<f32>) -> &mut Self {
        self.r#turn_rate = r#turn_rate.into();
        self
    }
    pub fn with_start_time(mut self, r#start_time: impl Into<f32>) -> Self {
        self.r#start_time = r#start_time.into();
        self
    }
    pub fn set_start_time(&mut self, r#start_time: impl Into<f32>) -> &mut Self {
        self.r#start_time = r#start_time.into();
        self
    }
    pub fn with_lifetime(mut self, r#lifetime: impl Into<f32>) -> Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn set_lifetime(&mut self, r#lifetime: impl Into<f32>) -> &mut Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn with_particle_size(mut self, r#particle_size: impl Into<f32>) -> Self {
        self.r#particle_size = r#particle_size.into();
        self
    }
    pub fn set_particle_size(&mut self, r#particle_size: impl Into<f32>) -> &mut Self {
        self.r#particle_size = r#particle_size.into();
        self
    }
    pub fn with_loop(mut self, r#loop: impl Into<bool>) -> Self {
        self.r#loop = r#loop.into();
        self
    }
    pub fn set_loop(&mut self, r#loop: impl Into<bool>) -> &mut Self {
        self.r#loop = r#loop.into();
        self
    }
}
impl DatabaseItem for VisualEffectElement {
    fn validate(&mut self) {
        if self.r#quantity < (1f32 as i32) {
            tracing::warn!(
                field = "r#quantity",
                value = self.r#quantity,
                min = 1f32,
                "Field got truncated"
            );
            self.r#quantity = 1f32 as i32;
        }
        if self.r#quantity > (100f32 as i32) {
            tracing::warn!(
                field = "r#quantity",
                value = self.r#quantity,
                max = 100f32,
                "Field got truncated"
            );
            self.r#quantity = 100f32 as i32;
        }
        if self.r#size < (0.001f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0.001f32,
                "Field got truncated"
            );
            self.r#size = 0.001f32 as f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#size = 100f32 as f32;
        }
        if self.r#growth_rate < (-1f32 as f32) {
            tracing::warn!(
                field = "r#growth_rate",
                value = self.r#growth_rate,
                min = -1f32,
                "Field got truncated"
            );
            self.r#growth_rate = -1f32 as f32;
        }
        if self.r#growth_rate > (100f32 as f32) {
            tracing::warn!(
                field = "r#growth_rate",
                value = self.r#growth_rate,
                max = 100f32,
                "Field got truncated"
            );
            self.r#growth_rate = 100f32 as f32;
        }
        if self.r#turn_rate < (-1000f32 as f32) {
            tracing::warn!(
                field = "r#turn_rate",
                value = self.r#turn_rate,
                min = -1000f32,
                "Field got truncated"
            );
            self.r#turn_rate = -1000f32 as f32;
        }
        if self.r#turn_rate > (1000f32 as f32) {
            tracing::warn!(
                field = "r#turn_rate",
                value = self.r#turn_rate,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#turn_rate = 1000f32 as f32;
        }
        if self.r#start_time < (0f32 as f32) {
            tracing::warn!(
                field = "r#start_time",
                value = self.r#start_time,
                min = 0f32,
                "Field got truncated"
            );
            self.r#start_time = 0f32 as f32;
        }
        if self.r#start_time > (1000f32 as f32) {
            tracing::warn!(
                field = "r#start_time",
                value = self.r#start_time,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#start_time = 1000f32 as f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                min = 0f32,
                "Field got truncated"
            );
            self.r#lifetime = 0f32 as f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#lifetime = 1000f32 as f32;
        }
        if self.r#particle_size < (0.001f32 as f32) {
            tracing::warn!(
                field = "r#particle_size",
                value = self.r#particle_size,
                min = 0.001f32,
                "Field got truncated"
            );
            self.r#particle_size = 0.001f32 as f32;
        }
        if self.r#particle_size > (100f32 as f32) {
            tracing::warn!(
                field = "r#particle_size",
                value = self.r#particle_size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#particle_size = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "VisualEffectElement"
    }
}
impl Default for VisualEffectElement {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/CombatSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CombatSettings {
    pub r#enemy_ai: Option<BehaviorTreeId>,
    pub r#autopilot_ai: Option<BehaviorTreeId>,
    pub r#clone_ai: Option<BehaviorTreeId>,
    pub r#defensive_drone_ai: Option<BehaviorTreeId>,
    pub r#offensive_drone_ai: Option<BehaviorTreeId>,
    pub r#starbase_ai: Option<BehaviorTreeId>,
    pub r#default_combat_rules: Option<CombatRulesId>,
}
impl CombatSettings {
    pub fn new() -> Self {
        Self {
            r#enemy_ai: Default::default(),
            r#autopilot_ai: Default::default(),
            r#clone_ai: Default::default(),
            r#defensive_drone_ai: Default::default(),
            r#offensive_drone_ai: Default::default(),
            r#starbase_ai: Default::default(),
            r#default_combat_rules: Default::default(),
        }
    }
    pub fn with_enemy_ai(mut self, r#enemy_ai: impl Into<Option<BehaviorTreeId>>) -> Self {
        self.r#enemy_ai = r#enemy_ai.into();
        self
    }
    pub fn set_enemy_ai(&mut self, r#enemy_ai: impl Into<Option<BehaviorTreeId>>) -> &mut Self {
        self.r#enemy_ai = r#enemy_ai.into();
        self
    }
    pub fn with_autopilot_ai(mut self, r#autopilot_ai: impl Into<Option<BehaviorTreeId>>) -> Self {
        self.r#autopilot_ai = r#autopilot_ai.into();
        self
    }
    pub fn set_autopilot_ai(
        &mut self,
        r#autopilot_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> &mut Self {
        self.r#autopilot_ai = r#autopilot_ai.into();
        self
    }
    pub fn with_clone_ai(mut self, r#clone_ai: impl Into<Option<BehaviorTreeId>>) -> Self {
        self.r#clone_ai = r#clone_ai.into();
        self
    }
    pub fn set_clone_ai(&mut self, r#clone_ai: impl Into<Option<BehaviorTreeId>>) -> &mut Self {
        self.r#clone_ai = r#clone_ai.into();
        self
    }
    pub fn with_defensive_drone_ai(
        mut self,
        r#defensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> Self {
        self.r#defensive_drone_ai = r#defensive_drone_ai.into();
        self
    }
    pub fn set_defensive_drone_ai(
        &mut self,
        r#defensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> &mut Self {
        self.r#defensive_drone_ai = r#defensive_drone_ai.into();
        self
    }
    pub fn with_offensive_drone_ai(
        mut self,
        r#offensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> Self {
        self.r#offensive_drone_ai = r#offensive_drone_ai.into();
        self
    }
    pub fn set_offensive_drone_ai(
        &mut self,
        r#offensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> &mut Self {
        self.r#offensive_drone_ai = r#offensive_drone_ai.into();
        self
    }
    pub fn with_starbase_ai(mut self, r#starbase_ai: impl Into<Option<BehaviorTreeId>>) -> Self {
        self.r#starbase_ai = r#starbase_ai.into();
        self
    }
    pub fn set_starbase_ai(
        &mut self,
        r#starbase_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> &mut Self {
        self.r#starbase_ai = r#starbase_ai.into();
        self
    }
    pub fn with_default_combat_rules(
        mut self,
        r#default_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#default_combat_rules = r#default_combat_rules.into();
        self
    }
    pub fn set_default_combat_rules(
        &mut self,
        r#default_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#default_combat_rules = r#default_combat_rules.into();
        self
    }
}
impl DatabaseItem for CombatSettings {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "CombatSettings"
    }
}
impl Default for CombatSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/DatabaseSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DatabaseSettings {
    pub r#database_version: i32,
    pub r#database_version_minor: i32,
    pub r#mod_name: String,
    pub r#mod_id: String,
    pub r#mod_version: i32,
    pub r#unload_original_database: bool,
}
impl DatabaseSettings {
    pub fn new() -> Self {
        Self {
            r#database_version: Default::default(),
            r#database_version_minor: Default::default(),
            r#mod_name: Default::default(),
            r#mod_id: Default::default(),
            r#mod_version: Default::default(),
            r#unload_original_database: Default::default(),
        }
    }
    pub fn with_database_version(mut self, r#database_version: impl Into<i32>) -> Self {
        self.r#database_version = r#database_version.into();
        self
    }
    pub fn set_database_version(&mut self, r#database_version: impl Into<i32>) -> &mut Self {
        self.r#database_version = r#database_version.into();
        self
    }
    pub fn with_database_version_minor(mut self, r#database_version_minor: impl Into<i32>) -> Self {
        self.r#database_version_minor = r#database_version_minor.into();
        self
    }
    pub fn set_database_version_minor(
        &mut self,
        r#database_version_minor: impl Into<i32>,
    ) -> &mut Self {
        self.r#database_version_minor = r#database_version_minor.into();
        self
    }
    pub fn with_mod_name(mut self, r#mod_name: impl Into<String>) -> Self {
        self.r#mod_name = r#mod_name.into();
        self
    }
    pub fn set_mod_name(&mut self, r#mod_name: impl Into<String>) -> &mut Self {
        self.r#mod_name = r#mod_name.into();
        self
    }
    pub fn with_mod_id(mut self, r#mod_id: impl Into<String>) -> Self {
        self.r#mod_id = r#mod_id.into();
        self
    }
    pub fn set_mod_id(&mut self, r#mod_id: impl Into<String>) -> &mut Self {
        self.r#mod_id = r#mod_id.into();
        self
    }
    pub fn with_mod_version(mut self, r#mod_version: impl Into<i32>) -> Self {
        self.r#mod_version = r#mod_version.into();
        self
    }
    pub fn set_mod_version(&mut self, r#mod_version: impl Into<i32>) -> &mut Self {
        self.r#mod_version = r#mod_version.into();
        self
    }
    pub fn with_unload_original_database(
        mut self,
        r#unload_original_database: impl Into<bool>,
    ) -> Self {
        self.r#unload_original_database = r#unload_original_database.into();
        self
    }
    pub fn set_unload_original_database(
        &mut self,
        r#unload_original_database: impl Into<bool>,
    ) -> &mut Self {
        self.r#unload_original_database = r#unload_original_database.into();
        self
    }
}
impl DatabaseItem for DatabaseSettings {
    fn validate(&mut self) {
        if self.r#database_version < (1f32 as i32) {
            tracing::warn!(
                field = "r#database_version",
                value = self.r#database_version,
                min = 1f32,
                "Field got truncated"
            );
            self.r#database_version = 1f32 as i32;
        }
        if self.r#database_version_minor < (0f32 as i32) {
            tracing::warn!(
                field = "r#database_version_minor",
                value = self.r#database_version_minor,
                min = 0f32,
                "Field got truncated"
            );
            self.r#database_version_minor = 0f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "DatabaseSettings"
    }
}
impl Default for DatabaseSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/DebugSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DebugSettings {
    pub r#codes: Vec<DebugCode>,
    pub r#enable_debug_console: bool,
}
impl DebugSettings {
    pub fn new() -> Self {
        Self {
            r#codes: Default::default(),
            r#enable_debug_console: Default::default(),
        }
    }
    pub fn with_codes(mut self, r#codes: impl Into<Vec<DebugCode>>) -> Self {
        self.r#codes = r#codes.into();
        self
    }
    pub fn set_codes(&mut self, r#codes: impl Into<Vec<DebugCode>>) -> &mut Self {
        self.r#codes = r#codes.into();
        self
    }
    pub fn with_enable_debug_console(mut self, r#enable_debug_console: impl Into<bool>) -> Self {
        self.r#enable_debug_console = r#enable_debug_console.into();
        self
    }
    pub fn set_enable_debug_console(
        &mut self,
        r#enable_debug_console: impl Into<bool>,
    ) -> &mut Self {
        self.r#enable_debug_console = r#enable_debug_console.into();
        self
    }
}
impl DatabaseItem for DebugSettings {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "DebugSettings"
    }
}
impl Default for DebugSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ExplorationSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExplorationSettings {
    pub r#outpost_ship: Option<ShipId>,
    pub r#turret_ship: Option<ShipId>,
    pub r#infected_planet_faction: Option<FactionId>,
    pub r#hive_ship_build: Option<ShipBuildId>,
    pub r#gas_cloud_dps: String,
}
impl ExplorationSettings {
    pub fn new() -> Self {
        Self {
            r#outpost_ship: Default::default(),
            r#turret_ship: Default::default(),
            r#infected_planet_faction: Default::default(),
            r#hive_ship_build: Default::default(),
            r#gas_cloud_dps: "MIN(level*2,500)".to_string(),
        }
    }
    pub fn with_outpost_ship(mut self, r#outpost_ship: impl Into<Option<ShipId>>) -> Self {
        self.r#outpost_ship = r#outpost_ship.into();
        self
    }
    pub fn set_outpost_ship(&mut self, r#outpost_ship: impl Into<Option<ShipId>>) -> &mut Self {
        self.r#outpost_ship = r#outpost_ship.into();
        self
    }
    pub fn with_turret_ship(mut self, r#turret_ship: impl Into<Option<ShipId>>) -> Self {
        self.r#turret_ship = r#turret_ship.into();
        self
    }
    pub fn set_turret_ship(&mut self, r#turret_ship: impl Into<Option<ShipId>>) -> &mut Self {
        self.r#turret_ship = r#turret_ship.into();
        self
    }
    pub fn with_infected_planet_faction(
        mut self,
        r#infected_planet_faction: impl Into<Option<FactionId>>,
    ) -> Self {
        self.r#infected_planet_faction = r#infected_planet_faction.into();
        self
    }
    pub fn set_infected_planet_faction(
        &mut self,
        r#infected_planet_faction: impl Into<Option<FactionId>>,
    ) -> &mut Self {
        self.r#infected_planet_faction = r#infected_planet_faction.into();
        self
    }
    pub fn with_hive_ship_build(
        mut self,
        r#hive_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#hive_ship_build = r#hive_ship_build.into();
        self
    }
    pub fn set_hive_ship_build(
        &mut self,
        r#hive_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#hive_ship_build = r#hive_ship_build.into();
        self
    }
    pub fn with_gas_cloud_dps(mut self, r#gas_cloud_dps: impl Into<String>) -> Self {
        self.r#gas_cloud_dps = r#gas_cloud_dps.into();
        self
    }
    pub fn set_gas_cloud_dps(&mut self, r#gas_cloud_dps: impl Into<String>) -> &mut Self {
        self.r#gas_cloud_dps = r#gas_cloud_dps.into();
        self
    }
}
impl DatabaseItem for ExplorationSettings {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ExplorationSettings"
    }
}
impl Default for ExplorationSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/FactionsSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FactionsSettings {
    pub r#starbase_initial_defense: String,
    pub r#starbase_min_defense: i32,
    pub r#defense_loss_per_enemy_defeated: i32,
}
impl FactionsSettings {
    pub fn new() -> Self {
        Self {
            r#starbase_initial_defense: "MIN(1000, 300 + 5*distance)".to_string(),
            r#starbase_min_defense: 50i32,
            r#defense_loss_per_enemy_defeated: 10i32,
        }
    }
    pub fn with_starbase_initial_defense(
        mut self,
        r#starbase_initial_defense: impl Into<String>,
    ) -> Self {
        self.r#starbase_initial_defense = r#starbase_initial_defense.into();
        self
    }
    pub fn set_starbase_initial_defense(
        &mut self,
        r#starbase_initial_defense: impl Into<String>,
    ) -> &mut Self {
        self.r#starbase_initial_defense = r#starbase_initial_defense.into();
        self
    }
    pub fn with_starbase_min_defense(mut self, r#starbase_min_defense: impl Into<i32>) -> Self {
        self.r#starbase_min_defense = r#starbase_min_defense.into();
        self
    }
    pub fn set_starbase_min_defense(
        &mut self,
        r#starbase_min_defense: impl Into<i32>,
    ) -> &mut Self {
        self.r#starbase_min_defense = r#starbase_min_defense.into();
        self
    }
    pub fn with_defense_loss_per_enemy_defeated(
        mut self,
        r#defense_loss_per_enemy_defeated: impl Into<i32>,
    ) -> Self {
        self.r#defense_loss_per_enemy_defeated = r#defense_loss_per_enemy_defeated.into();
        self
    }
    pub fn set_defense_loss_per_enemy_defeated(
        &mut self,
        r#defense_loss_per_enemy_defeated: impl Into<i32>,
    ) -> &mut Self {
        self.r#defense_loss_per_enemy_defeated = r#defense_loss_per_enemy_defeated.into();
        self
    }
}
impl DatabaseItem for FactionsSettings {
    fn validate(&mut self) {
        if self.r#starbase_min_defense < (1f32 as i32) {
            tracing::warn!(
                field = "r#starbase_min_defense",
                value = self.r#starbase_min_defense,
                min = 1f32,
                "Field got truncated"
            );
            self.r#starbase_min_defense = 1f32 as i32;
        }
        if self.r#defense_loss_per_enemy_defeated < (0f32 as i32) {
            tracing::warn!(
                field = "r#defense_loss_per_enemy_defeated",
                value = self.r#defense_loss_per_enemy_defeated,
                min = 0f32,
                "Field got truncated"
            );
            self.r#defense_loss_per_enemy_defeated = 0f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "FactionsSettings"
    }
}
impl Default for FactionsSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/FrontierSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FrontierSettings {
    pub r#base_command_points: i32,
    pub r#max_extra_command_points: i32,
    pub r#supporter_pack_ship: Option<ShipId>,
    pub r#falcon_pack_ship: Option<ShipId>,
    pub r#big_boss_easy_build: Option<ShipBuildId>,
    pub r#big_boss_normal_build: Option<ShipBuildId>,
    pub r#big_boss_hard_build: Option<ShipBuildId>,
    pub r#demo_scene_starbase_build: Option<ShipBuildId>,
    pub r#tutorial_starbase_build: Option<ShipBuildId>,
    pub r#default_starbase_build: Option<ShipBuildId>,
    pub r#exploration_starbase: Option<ShipId>,
    pub r#merchant_ship_build: Option<ShipBuildId>,
    pub r#smuggler_ship_build: Option<ShipBuildId>,
    pub r#engineer_ship_build: Option<ShipBuildId>,
    pub r#mercenary_ship_build: Option<ShipBuildId>,
    pub r#shipyard_ship_build: Option<ShipBuildId>,
    pub r#santa_ship_build: Option<ShipBuildId>,
    pub r#salvage_drone_build: Option<ShipBuildId>,
    pub r#custom_ship_levels: Vec<ShipToValue>,
    pub r#custom_ship_prices: Vec<ShipToValue>,
    pub r#exploration_ships: Vec<ShipId>,
}
impl FrontierSettings {
    pub fn new() -> Self {
        Self {
            r#base_command_points: Default::default(),
            r#max_extra_command_points: Default::default(),
            r#supporter_pack_ship: Default::default(),
            r#falcon_pack_ship: Default::default(),
            r#big_boss_easy_build: Default::default(),
            r#big_boss_normal_build: Default::default(),
            r#big_boss_hard_build: Default::default(),
            r#demo_scene_starbase_build: Default::default(),
            r#tutorial_starbase_build: Default::default(),
            r#default_starbase_build: Default::default(),
            r#exploration_starbase: Default::default(),
            r#merchant_ship_build: Default::default(),
            r#smuggler_ship_build: Default::default(),
            r#engineer_ship_build: Default::default(),
            r#mercenary_ship_build: Default::default(),
            r#shipyard_ship_build: Default::default(),
            r#santa_ship_build: Default::default(),
            r#salvage_drone_build: Default::default(),
            r#custom_ship_levels: Default::default(),
            r#custom_ship_prices: Default::default(),
            r#exploration_ships: Default::default(),
        }
    }
    pub fn with_base_command_points(mut self, r#base_command_points: impl Into<i32>) -> Self {
        self.r#base_command_points = r#base_command_points.into();
        self
    }
    pub fn set_base_command_points(&mut self, r#base_command_points: impl Into<i32>) -> &mut Self {
        self.r#base_command_points = r#base_command_points.into();
        self
    }
    pub fn with_max_extra_command_points(
        mut self,
        r#max_extra_command_points: impl Into<i32>,
    ) -> Self {
        self.r#max_extra_command_points = r#max_extra_command_points.into();
        self
    }
    pub fn set_max_extra_command_points(
        &mut self,
        r#max_extra_command_points: impl Into<i32>,
    ) -> &mut Self {
        self.r#max_extra_command_points = r#max_extra_command_points.into();
        self
    }
    pub fn with_supporter_pack_ship(
        mut self,
        r#supporter_pack_ship: impl Into<Option<ShipId>>,
    ) -> Self {
        self.r#supporter_pack_ship = r#supporter_pack_ship.into();
        self
    }
    pub fn set_supporter_pack_ship(
        &mut self,
        r#supporter_pack_ship: impl Into<Option<ShipId>>,
    ) -> &mut Self {
        self.r#supporter_pack_ship = r#supporter_pack_ship.into();
        self
    }
    pub fn with_falcon_pack_ship(mut self, r#falcon_pack_ship: impl Into<Option<ShipId>>) -> Self {
        self.r#falcon_pack_ship = r#falcon_pack_ship.into();
        self
    }
    pub fn set_falcon_pack_ship(
        &mut self,
        r#falcon_pack_ship: impl Into<Option<ShipId>>,
    ) -> &mut Self {
        self.r#falcon_pack_ship = r#falcon_pack_ship.into();
        self
    }
    pub fn with_big_boss_easy_build(
        mut self,
        r#big_boss_easy_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#big_boss_easy_build = r#big_boss_easy_build.into();
        self
    }
    pub fn set_big_boss_easy_build(
        &mut self,
        r#big_boss_easy_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#big_boss_easy_build = r#big_boss_easy_build.into();
        self
    }
    pub fn with_big_boss_normal_build(
        mut self,
        r#big_boss_normal_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#big_boss_normal_build = r#big_boss_normal_build.into();
        self
    }
    pub fn set_big_boss_normal_build(
        &mut self,
        r#big_boss_normal_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#big_boss_normal_build = r#big_boss_normal_build.into();
        self
    }
    pub fn with_big_boss_hard_build(
        mut self,
        r#big_boss_hard_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#big_boss_hard_build = r#big_boss_hard_build.into();
        self
    }
    pub fn set_big_boss_hard_build(
        &mut self,
        r#big_boss_hard_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#big_boss_hard_build = r#big_boss_hard_build.into();
        self
    }
    pub fn with_demo_scene_starbase_build(
        mut self,
        r#demo_scene_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#demo_scene_starbase_build = r#demo_scene_starbase_build.into();
        self
    }
    pub fn set_demo_scene_starbase_build(
        &mut self,
        r#demo_scene_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#demo_scene_starbase_build = r#demo_scene_starbase_build.into();
        self
    }
    pub fn with_tutorial_starbase_build(
        mut self,
        r#tutorial_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#tutorial_starbase_build = r#tutorial_starbase_build.into();
        self
    }
    pub fn set_tutorial_starbase_build(
        &mut self,
        r#tutorial_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#tutorial_starbase_build = r#tutorial_starbase_build.into();
        self
    }
    pub fn with_default_starbase_build(
        mut self,
        r#default_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#default_starbase_build = r#default_starbase_build.into();
        self
    }
    pub fn set_default_starbase_build(
        &mut self,
        r#default_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#default_starbase_build = r#default_starbase_build.into();
        self
    }
    pub fn with_exploration_starbase(
        mut self,
        r#exploration_starbase: impl Into<Option<ShipId>>,
    ) -> Self {
        self.r#exploration_starbase = r#exploration_starbase.into();
        self
    }
    pub fn set_exploration_starbase(
        &mut self,
        r#exploration_starbase: impl Into<Option<ShipId>>,
    ) -> &mut Self {
        self.r#exploration_starbase = r#exploration_starbase.into();
        self
    }
    pub fn with_merchant_ship_build(
        mut self,
        r#merchant_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#merchant_ship_build = r#merchant_ship_build.into();
        self
    }
    pub fn set_merchant_ship_build(
        &mut self,
        r#merchant_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#merchant_ship_build = r#merchant_ship_build.into();
        self
    }
    pub fn with_smuggler_ship_build(
        mut self,
        r#smuggler_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#smuggler_ship_build = r#smuggler_ship_build.into();
        self
    }
    pub fn set_smuggler_ship_build(
        &mut self,
        r#smuggler_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#smuggler_ship_build = r#smuggler_ship_build.into();
        self
    }
    pub fn with_engineer_ship_build(
        mut self,
        r#engineer_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#engineer_ship_build = r#engineer_ship_build.into();
        self
    }
    pub fn set_engineer_ship_build(
        &mut self,
        r#engineer_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#engineer_ship_build = r#engineer_ship_build.into();
        self
    }
    pub fn with_mercenary_ship_build(
        mut self,
        r#mercenary_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#mercenary_ship_build = r#mercenary_ship_build.into();
        self
    }
    pub fn set_mercenary_ship_build(
        &mut self,
        r#mercenary_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#mercenary_ship_build = r#mercenary_ship_build.into();
        self
    }
    pub fn with_shipyard_ship_build(
        mut self,
        r#shipyard_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#shipyard_ship_build = r#shipyard_ship_build.into();
        self
    }
    pub fn set_shipyard_ship_build(
        &mut self,
        r#shipyard_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#shipyard_ship_build = r#shipyard_ship_build.into();
        self
    }
    pub fn with_santa_ship_build(
        mut self,
        r#santa_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#santa_ship_build = r#santa_ship_build.into();
        self
    }
    pub fn set_santa_ship_build(
        &mut self,
        r#santa_ship_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#santa_ship_build = r#santa_ship_build.into();
        self
    }
    pub fn with_salvage_drone_build(
        mut self,
        r#salvage_drone_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#salvage_drone_build = r#salvage_drone_build.into();
        self
    }
    pub fn set_salvage_drone_build(
        &mut self,
        r#salvage_drone_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#salvage_drone_build = r#salvage_drone_build.into();
        self
    }
    pub fn with_custom_ship_levels(
        mut self,
        r#custom_ship_levels: impl Into<Vec<ShipToValue>>,
    ) -> Self {
        self.r#custom_ship_levels = r#custom_ship_levels.into();
        self
    }
    pub fn set_custom_ship_levels(
        &mut self,
        r#custom_ship_levels: impl Into<Vec<ShipToValue>>,
    ) -> &mut Self {
        self.r#custom_ship_levels = r#custom_ship_levels.into();
        self
    }
    pub fn with_custom_ship_prices(
        mut self,
        r#custom_ship_prices: impl Into<Vec<ShipToValue>>,
    ) -> Self {
        self.r#custom_ship_prices = r#custom_ship_prices.into();
        self
    }
    pub fn set_custom_ship_prices(
        &mut self,
        r#custom_ship_prices: impl Into<Vec<ShipToValue>>,
    ) -> &mut Self {
        self.r#custom_ship_prices = r#custom_ship_prices.into();
        self
    }
    pub fn with_exploration_ships(mut self, r#exploration_ships: impl Into<Vec<ShipId>>) -> Self {
        self.r#exploration_ships = r#exploration_ships.into();
        self
    }
    pub fn set_exploration_ships(
        &mut self,
        r#exploration_ships: impl Into<Vec<ShipId>>,
    ) -> &mut Self {
        self.r#exploration_ships = r#exploration_ships.into();
        self
    }
}
impl DatabaseItem for FrontierSettings {
    fn validate(&mut self) {
        if self.r#base_command_points < (0f32 as i32) {
            tracing::warn!(
                field = "r#base_command_points",
                value = self.r#base_command_points,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_command_points = 0f32 as i32;
        }
        if self.r#max_extra_command_points < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_extra_command_points",
                value = self.r#max_extra_command_points,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_extra_command_points = 0f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "FrontierSettings"
    }
}
impl Default for FrontierSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/GalaxySettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GalaxySettings {
    pub r#abandoned_starbase_faction: Option<FactionId>,
    pub r#starting_ship_builds: Vec<ShipBuildId>,
    pub r#starting_inventory: Option<LootId>,
    pub r#supporter_pack_ship: Option<ShipBuildId>,
    pub r#default_starbase_build: Option<ShipBuildId>,
    pub r#max_enemy_ships_level: i32,
    pub r#enemy_level: String,
    pub r#ship_min_spawn_distance: String,
    pub r#capture_starbase_quest: Option<QuestId>,
    pub r#starting_invenory: Option<LootId>,
    pub r#survival_combat_rules: Option<CombatRulesId>,
    pub r#starbase_combat_rules: Option<CombatRulesId>,
    pub r#flagship_combat_rules: Option<CombatRulesId>,
    pub r#arena_combat_rules: Option<CombatRulesId>,
    pub r#challenge_combat_rules: Option<CombatRulesId>,
    pub r#quick_combat_rules: Option<CombatRulesId>,
}
impl GalaxySettings {
    pub fn new() -> Self {
        Self {
            r#abandoned_starbase_faction: Default::default(),
            r#starting_ship_builds: Default::default(),
            r#starting_inventory: Default::default(),
            r#supporter_pack_ship: Default::default(),
            r#default_starbase_build: Default::default(),
            r#max_enemy_ships_level: 300i32,
            r#enemy_level: "MIN(3*distance/5 - 5, MaxEnemyShipsLevel)".to_string(),
            r#ship_min_spawn_distance: "IF(size == Destroyer, 5, size == Cruiser, 15, size == Battleship, 50, size == Titan, 100, 0)"
                .to_string(),
            r#capture_starbase_quest: Default::default(),
            r#starting_invenory: Default::default(),
            r#survival_combat_rules: Default::default(),
            r#starbase_combat_rules: Default::default(),
            r#flagship_combat_rules: Default::default(),
            r#arena_combat_rules: Default::default(),
            r#challenge_combat_rules: Default::default(),
            r#quick_combat_rules: Default::default(),
        }
    }
    pub fn with_abandoned_starbase_faction(
        mut self,
        r#abandoned_starbase_faction: impl Into<Option<FactionId>>,
    ) -> Self {
        self.r#abandoned_starbase_faction = r#abandoned_starbase_faction.into();
        self
    }
    pub fn set_abandoned_starbase_faction(
        &mut self,
        r#abandoned_starbase_faction: impl Into<Option<FactionId>>,
    ) -> &mut Self {
        self.r#abandoned_starbase_faction = r#abandoned_starbase_faction.into();
        self
    }
    pub fn with_starting_ship_builds(
        mut self,
        r#starting_ship_builds: impl Into<Vec<ShipBuildId>>,
    ) -> Self {
        self.r#starting_ship_builds = r#starting_ship_builds.into();
        self
    }
    pub fn set_starting_ship_builds(
        &mut self,
        r#starting_ship_builds: impl Into<Vec<ShipBuildId>>,
    ) -> &mut Self {
        self.r#starting_ship_builds = r#starting_ship_builds.into();
        self
    }
    pub fn with_starting_inventory(
        mut self,
        r#starting_inventory: impl Into<Option<LootId>>,
    ) -> Self {
        self.r#starting_inventory = r#starting_inventory.into();
        self
    }
    pub fn set_starting_inventory(
        &mut self,
        r#starting_inventory: impl Into<Option<LootId>>,
    ) -> &mut Self {
        self.r#starting_inventory = r#starting_inventory.into();
        self
    }
    pub fn with_supporter_pack_ship(
        mut self,
        r#supporter_pack_ship: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#supporter_pack_ship = r#supporter_pack_ship.into();
        self
    }
    pub fn set_supporter_pack_ship(
        &mut self,
        r#supporter_pack_ship: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#supporter_pack_ship = r#supporter_pack_ship.into();
        self
    }
    pub fn with_default_starbase_build(
        mut self,
        r#default_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> Self {
        self.r#default_starbase_build = r#default_starbase_build.into();
        self
    }
    pub fn set_default_starbase_build(
        &mut self,
        r#default_starbase_build: impl Into<Option<ShipBuildId>>,
    ) -> &mut Self {
        self.r#default_starbase_build = r#default_starbase_build.into();
        self
    }
    pub fn with_max_enemy_ships_level(mut self, r#max_enemy_ships_level: impl Into<i32>) -> Self {
        self.r#max_enemy_ships_level = r#max_enemy_ships_level.into();
        self
    }
    pub fn set_max_enemy_ships_level(
        &mut self,
        r#max_enemy_ships_level: impl Into<i32>,
    ) -> &mut Self {
        self.r#max_enemy_ships_level = r#max_enemy_ships_level.into();
        self
    }
    pub fn with_enemy_level(mut self, r#enemy_level: impl Into<String>) -> Self {
        self.r#enemy_level = r#enemy_level.into();
        self
    }
    pub fn set_enemy_level(&mut self, r#enemy_level: impl Into<String>) -> &mut Self {
        self.r#enemy_level = r#enemy_level.into();
        self
    }
    pub fn with_ship_min_spawn_distance(
        mut self,
        r#ship_min_spawn_distance: impl Into<String>,
    ) -> Self {
        self.r#ship_min_spawn_distance = r#ship_min_spawn_distance.into();
        self
    }
    pub fn set_ship_min_spawn_distance(
        &mut self,
        r#ship_min_spawn_distance: impl Into<String>,
    ) -> &mut Self {
        self.r#ship_min_spawn_distance = r#ship_min_spawn_distance.into();
        self
    }
    pub fn with_capture_starbase_quest(
        mut self,
        r#capture_starbase_quest: impl Into<Option<QuestId>>,
    ) -> Self {
        self.r#capture_starbase_quest = r#capture_starbase_quest.into();
        self
    }
    pub fn set_capture_starbase_quest(
        &mut self,
        r#capture_starbase_quest: impl Into<Option<QuestId>>,
    ) -> &mut Self {
        self.r#capture_starbase_quest = r#capture_starbase_quest.into();
        self
    }
    pub fn with_starting_invenory(
        mut self,
        r#starting_invenory: impl Into<Option<LootId>>,
    ) -> Self {
        self.r#starting_invenory = r#starting_invenory.into();
        self
    }
    pub fn set_starting_invenory(
        &mut self,
        r#starting_invenory: impl Into<Option<LootId>>,
    ) -> &mut Self {
        self.r#starting_invenory = r#starting_invenory.into();
        self
    }
    pub fn with_survival_combat_rules(
        mut self,
        r#survival_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#survival_combat_rules = r#survival_combat_rules.into();
        self
    }
    pub fn set_survival_combat_rules(
        &mut self,
        r#survival_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#survival_combat_rules = r#survival_combat_rules.into();
        self
    }
    pub fn with_starbase_combat_rules(
        mut self,
        r#starbase_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#starbase_combat_rules = r#starbase_combat_rules.into();
        self
    }
    pub fn set_starbase_combat_rules(
        &mut self,
        r#starbase_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#starbase_combat_rules = r#starbase_combat_rules.into();
        self
    }
    pub fn with_flagship_combat_rules(
        mut self,
        r#flagship_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#flagship_combat_rules = r#flagship_combat_rules.into();
        self
    }
    pub fn set_flagship_combat_rules(
        &mut self,
        r#flagship_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#flagship_combat_rules = r#flagship_combat_rules.into();
        self
    }
    pub fn with_arena_combat_rules(
        mut self,
        r#arena_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#arena_combat_rules = r#arena_combat_rules.into();
        self
    }
    pub fn set_arena_combat_rules(
        &mut self,
        r#arena_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#arena_combat_rules = r#arena_combat_rules.into();
        self
    }
    pub fn with_challenge_combat_rules(
        mut self,
        r#challenge_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#challenge_combat_rules = r#challenge_combat_rules.into();
        self
    }
    pub fn set_challenge_combat_rules(
        &mut self,
        r#challenge_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#challenge_combat_rules = r#challenge_combat_rules.into();
        self
    }
    pub fn with_quick_combat_rules(
        mut self,
        r#quick_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#quick_combat_rules = r#quick_combat_rules.into();
        self
    }
    pub fn set_quick_combat_rules(
        &mut self,
        r#quick_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#quick_combat_rules = r#quick_combat_rules.into();
        self
    }
}
impl DatabaseItem for GalaxySettings {
    fn validate(&mut self) {
        if self.r#max_enemy_ships_level < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_enemy_ships_level",
                value = self.r#max_enemy_ships_level,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_enemy_ships_level = 0f32 as i32;
        }
        if self.r#max_enemy_ships_level > (500f32 as i32) {
            tracing::warn!(
                field = "r#max_enemy_ships_level",
                value = self.r#max_enemy_ships_level,
                max = 500f32,
                "Field got truncated"
            );
            self.r#max_enemy_ships_level = 500f32 as i32;
        }
        let dw: Option<LootId> = Default::default();
        if self.r#starting_invenory != dw {
            tracing::error!(
                ield = "r#starting_invenory",
                "Obsolete field usage detected, generated code may not work",
            );
        }
    }
    fn type_name() -> &'static str {
        "GalaxySettings"
    }
}
impl Default for GalaxySettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/MusicPlaylist.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct MusicPlaylist {
    pub r#main_menu_music: Vec<SoundTrack>,
    pub r#galaxy_map_music: Vec<SoundTrack>,
    pub r#combat_music: Vec<SoundTrack>,
    pub r#exploration_music: Vec<SoundTrack>,
}
impl MusicPlaylist {
    pub fn new() -> Self {
        Self {
            r#main_menu_music: Default::default(),
            r#galaxy_map_music: Default::default(),
            r#combat_music: Default::default(),
            r#exploration_music: Default::default(),
        }
    }
    pub fn with_main_menu_music(mut self, r#main_menu_music: impl Into<Vec<SoundTrack>>) -> Self {
        self.r#main_menu_music = r#main_menu_music.into();
        self
    }
    pub fn set_main_menu_music(
        &mut self,
        r#main_menu_music: impl Into<Vec<SoundTrack>>,
    ) -> &mut Self {
        self.r#main_menu_music = r#main_menu_music.into();
        self
    }
    pub fn with_galaxy_map_music(mut self, r#galaxy_map_music: impl Into<Vec<SoundTrack>>) -> Self {
        self.r#galaxy_map_music = r#galaxy_map_music.into();
        self
    }
    pub fn set_galaxy_map_music(
        &mut self,
        r#galaxy_map_music: impl Into<Vec<SoundTrack>>,
    ) -> &mut Self {
        self.r#galaxy_map_music = r#galaxy_map_music.into();
        self
    }
    pub fn with_combat_music(mut self, r#combat_music: impl Into<Vec<SoundTrack>>) -> Self {
        self.r#combat_music = r#combat_music.into();
        self
    }
    pub fn set_combat_music(&mut self, r#combat_music: impl Into<Vec<SoundTrack>>) -> &mut Self {
        self.r#combat_music = r#combat_music.into();
        self
    }
    pub fn with_exploration_music(
        mut self,
        r#exploration_music: impl Into<Vec<SoundTrack>>,
    ) -> Self {
        self.r#exploration_music = r#exploration_music.into();
        self
    }
    pub fn set_exploration_music(
        &mut self,
        r#exploration_music: impl Into<Vec<SoundTrack>>,
    ) -> &mut Self {
        self.r#exploration_music = r#exploration_music.into();
        self
    }
}
impl DatabaseItem for MusicPlaylist {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "MusicPlaylist"
    }
}
impl Default for MusicPlaylist {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ShipModSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipModSettings {
    pub r#remove_weapon_slot_mod: bool,
    pub r#heat_defense_value: f32,
    pub r#kinetic_defense_value: f32,
    pub r#energy_defense_value: f32,
    pub r#regeneration_value: f32,
    pub r#regeneration_armor: f32,
    pub r#weight_reduction: f32,
    pub r#attack_reduction: f32,
}
impl ShipModSettings {
    pub fn new() -> Self {
        Self {
            r#remove_weapon_slot_mod: Default::default(),
            r#heat_defense_value: 0.5f32,
            r#kinetic_defense_value: 0.5f32,
            r#energy_defense_value: 0.5f32,
            r#regeneration_value: 0.01f32,
            r#regeneration_armor: 0.85f32,
            r#weight_reduction: 0.8f32,
            r#attack_reduction: 0.2f32,
        }
    }
    pub fn with_remove_weapon_slot_mod(
        mut self,
        r#remove_weapon_slot_mod: impl Into<bool>,
    ) -> Self {
        self.r#remove_weapon_slot_mod = r#remove_weapon_slot_mod.into();
        self
    }
    pub fn set_remove_weapon_slot_mod(
        &mut self,
        r#remove_weapon_slot_mod: impl Into<bool>,
    ) -> &mut Self {
        self.r#remove_weapon_slot_mod = r#remove_weapon_slot_mod.into();
        self
    }
    pub fn with_heat_defense_value(mut self, r#heat_defense_value: impl Into<f32>) -> Self {
        self.r#heat_defense_value = r#heat_defense_value.into();
        self
    }
    pub fn set_heat_defense_value(&mut self, r#heat_defense_value: impl Into<f32>) -> &mut Self {
        self.r#heat_defense_value = r#heat_defense_value.into();
        self
    }
    pub fn with_kinetic_defense_value(mut self, r#kinetic_defense_value: impl Into<f32>) -> Self {
        self.r#kinetic_defense_value = r#kinetic_defense_value.into();
        self
    }
    pub fn set_kinetic_defense_value(
        &mut self,
        r#kinetic_defense_value: impl Into<f32>,
    ) -> &mut Self {
        self.r#kinetic_defense_value = r#kinetic_defense_value.into();
        self
    }
    pub fn with_energy_defense_value(mut self, r#energy_defense_value: impl Into<f32>) -> Self {
        self.r#energy_defense_value = r#energy_defense_value.into();
        self
    }
    pub fn set_energy_defense_value(
        &mut self,
        r#energy_defense_value: impl Into<f32>,
    ) -> &mut Self {
        self.r#energy_defense_value = r#energy_defense_value.into();
        self
    }
    pub fn with_regeneration_value(mut self, r#regeneration_value: impl Into<f32>) -> Self {
        self.r#regeneration_value = r#regeneration_value.into();
        self
    }
    pub fn set_regeneration_value(&mut self, r#regeneration_value: impl Into<f32>) -> &mut Self {
        self.r#regeneration_value = r#regeneration_value.into();
        self
    }
    pub fn with_regeneration_armor(mut self, r#regeneration_armor: impl Into<f32>) -> Self {
        self.r#regeneration_armor = r#regeneration_armor.into();
        self
    }
    pub fn set_regeneration_armor(&mut self, r#regeneration_armor: impl Into<f32>) -> &mut Self {
        self.r#regeneration_armor = r#regeneration_armor.into();
        self
    }
    pub fn with_weight_reduction(mut self, r#weight_reduction: impl Into<f32>) -> Self {
        self.r#weight_reduction = r#weight_reduction.into();
        self
    }
    pub fn set_weight_reduction(&mut self, r#weight_reduction: impl Into<f32>) -> &mut Self {
        self.r#weight_reduction = r#weight_reduction.into();
        self
    }
    pub fn with_attack_reduction(mut self, r#attack_reduction: impl Into<f32>) -> Self {
        self.r#attack_reduction = r#attack_reduction.into();
        self
    }
    pub fn set_attack_reduction(&mut self, r#attack_reduction: impl Into<f32>) -> &mut Self {
        self.r#attack_reduction = r#attack_reduction.into();
        self
    }
}
impl DatabaseItem for ShipModSettings {
    fn validate(&mut self) {
        if self.r#heat_defense_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#heat_defense_value",
                value = self.r#heat_defense_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#heat_defense_value = 0f32 as f32;
        }
        if self.r#heat_defense_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#heat_defense_value",
                value = self.r#heat_defense_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#heat_defense_value = 1f32 as f32;
        }
        if self.r#kinetic_defense_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#kinetic_defense_value",
                value = self.r#kinetic_defense_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#kinetic_defense_value = 0f32 as f32;
        }
        if self.r#kinetic_defense_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#kinetic_defense_value",
                value = self.r#kinetic_defense_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#kinetic_defense_value = 1f32 as f32;
        }
        if self.r#energy_defense_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#energy_defense_value",
                value = self.r#energy_defense_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#energy_defense_value = 0f32 as f32;
        }
        if self.r#energy_defense_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#energy_defense_value",
                value = self.r#energy_defense_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#energy_defense_value = 1f32 as f32;
        }
        if self.r#regeneration_value < (0f32 as f32) {
            tracing::warn!(
                field = "r#regeneration_value",
                value = self.r#regeneration_value,
                min = 0f32,
                "Field got truncated"
            );
            self.r#regeneration_value = 0f32 as f32;
        }
        if self.r#regeneration_value > (1f32 as f32) {
            tracing::warn!(
                field = "r#regeneration_value",
                value = self.r#regeneration_value,
                max = 1f32,
                "Field got truncated"
            );
            self.r#regeneration_value = 1f32 as f32;
        }
        if self.r#regeneration_armor < (0f32 as f32) {
            tracing::warn!(
                field = "r#regeneration_armor",
                value = self.r#regeneration_armor,
                min = 0f32,
                "Field got truncated"
            );
            self.r#regeneration_armor = 0f32 as f32;
        }
        if self.r#regeneration_armor > (1f32 as f32) {
            tracing::warn!(
                field = "r#regeneration_armor",
                value = self.r#regeneration_armor,
                max = 1f32,
                "Field got truncated"
            );
            self.r#regeneration_armor = 1f32 as f32;
        }
        if self.r#weight_reduction < (0f32 as f32) {
            tracing::warn!(
                field = "r#weight_reduction",
                value = self.r#weight_reduction,
                min = 0f32,
                "Field got truncated"
            );
            self.r#weight_reduction = 0f32 as f32;
        }
        if self.r#weight_reduction > (1f32 as f32) {
            tracing::warn!(
                field = "r#weight_reduction",
                value = self.r#weight_reduction,
                max = 1f32,
                "Field got truncated"
            );
            self.r#weight_reduction = 1f32 as f32;
        }
        if self.r#attack_reduction < (0f32 as f32) {
            tracing::warn!(
                field = "r#attack_reduction",
                value = self.r#attack_reduction,
                min = 0f32,
                "Field got truncated"
            );
            self.r#attack_reduction = 0f32 as f32;
        }
        if self.r#attack_reduction > (1f32 as f32) {
            tracing::warn!(
                field = "r#attack_reduction",
                value = self.r#attack_reduction,
                max = 1f32,
                "Field got truncated"
            );
            self.r#attack_reduction = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "ShipModSettings"
    }
}
impl Default for ShipModSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ShipSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipSettings {
    pub r#default_weight_per_cell: f32,
    pub r#minimum_weight_per_cell: f32,
    pub r#base_armor_points: f32,
    pub r#armor_points_per_cell: f32,
    pub r#armor_repair_cooldown: f32,
    pub r#base_energy_points: f32,
    pub r#base_energy_recharge_rate: f32,
    pub r#energy_recharge_cooldown: f32,
    pub r#base_shield_recharge_rate: f32,
    pub r#shield_recharge_cooldown: f32,
    pub r#base_drone_reconstruction_speed: f32,
    pub r#max_velocity: f32,
    pub r#max_turn_rate: f32,
}
impl ShipSettings {
    pub fn new() -> Self {
        Self {
            r#default_weight_per_cell: Default::default(),
            r#minimum_weight_per_cell: Default::default(),
            r#base_armor_points: Default::default(),
            r#armor_points_per_cell: Default::default(),
            r#armor_repair_cooldown: Default::default(),
            r#base_energy_points: Default::default(),
            r#base_energy_recharge_rate: Default::default(),
            r#energy_recharge_cooldown: Default::default(),
            r#base_shield_recharge_rate: Default::default(),
            r#shield_recharge_cooldown: Default::default(),
            r#base_drone_reconstruction_speed: Default::default(),
            r#max_velocity: Default::default(),
            r#max_turn_rate: Default::default(),
        }
    }
    pub fn with_default_weight_per_cell(
        mut self,
        r#default_weight_per_cell: impl Into<f32>,
    ) -> Self {
        self.r#default_weight_per_cell = r#default_weight_per_cell.into();
        self
    }
    pub fn set_default_weight_per_cell(
        &mut self,
        r#default_weight_per_cell: impl Into<f32>,
    ) -> &mut Self {
        self.r#default_weight_per_cell = r#default_weight_per_cell.into();
        self
    }
    pub fn with_minimum_weight_per_cell(
        mut self,
        r#minimum_weight_per_cell: impl Into<f32>,
    ) -> Self {
        self.r#minimum_weight_per_cell = r#minimum_weight_per_cell.into();
        self
    }
    pub fn set_minimum_weight_per_cell(
        &mut self,
        r#minimum_weight_per_cell: impl Into<f32>,
    ) -> &mut Self {
        self.r#minimum_weight_per_cell = r#minimum_weight_per_cell.into();
        self
    }
    pub fn with_base_armor_points(mut self, r#base_armor_points: impl Into<f32>) -> Self {
        self.r#base_armor_points = r#base_armor_points.into();
        self
    }
    pub fn set_base_armor_points(&mut self, r#base_armor_points: impl Into<f32>) -> &mut Self {
        self.r#base_armor_points = r#base_armor_points.into();
        self
    }
    pub fn with_armor_points_per_cell(mut self, r#armor_points_per_cell: impl Into<f32>) -> Self {
        self.r#armor_points_per_cell = r#armor_points_per_cell.into();
        self
    }
    pub fn set_armor_points_per_cell(
        &mut self,
        r#armor_points_per_cell: impl Into<f32>,
    ) -> &mut Self {
        self.r#armor_points_per_cell = r#armor_points_per_cell.into();
        self
    }
    pub fn with_armor_repair_cooldown(mut self, r#armor_repair_cooldown: impl Into<f32>) -> Self {
        self.r#armor_repair_cooldown = r#armor_repair_cooldown.into();
        self
    }
    pub fn set_armor_repair_cooldown(
        &mut self,
        r#armor_repair_cooldown: impl Into<f32>,
    ) -> &mut Self {
        self.r#armor_repair_cooldown = r#armor_repair_cooldown.into();
        self
    }
    pub fn with_base_energy_points(mut self, r#base_energy_points: impl Into<f32>) -> Self {
        self.r#base_energy_points = r#base_energy_points.into();
        self
    }
    pub fn set_base_energy_points(&mut self, r#base_energy_points: impl Into<f32>) -> &mut Self {
        self.r#base_energy_points = r#base_energy_points.into();
        self
    }
    pub fn with_base_energy_recharge_rate(
        mut self,
        r#base_energy_recharge_rate: impl Into<f32>,
    ) -> Self {
        self.r#base_energy_recharge_rate = r#base_energy_recharge_rate.into();
        self
    }
    pub fn set_base_energy_recharge_rate(
        &mut self,
        r#base_energy_recharge_rate: impl Into<f32>,
    ) -> &mut Self {
        self.r#base_energy_recharge_rate = r#base_energy_recharge_rate.into();
        self
    }
    pub fn with_energy_recharge_cooldown(
        mut self,
        r#energy_recharge_cooldown: impl Into<f32>,
    ) -> Self {
        self.r#energy_recharge_cooldown = r#energy_recharge_cooldown.into();
        self
    }
    pub fn set_energy_recharge_cooldown(
        &mut self,
        r#energy_recharge_cooldown: impl Into<f32>,
    ) -> &mut Self {
        self.r#energy_recharge_cooldown = r#energy_recharge_cooldown.into();
        self
    }
    pub fn with_base_shield_recharge_rate(
        mut self,
        r#base_shield_recharge_rate: impl Into<f32>,
    ) -> Self {
        self.r#base_shield_recharge_rate = r#base_shield_recharge_rate.into();
        self
    }
    pub fn set_base_shield_recharge_rate(
        &mut self,
        r#base_shield_recharge_rate: impl Into<f32>,
    ) -> &mut Self {
        self.r#base_shield_recharge_rate = r#base_shield_recharge_rate.into();
        self
    }
    pub fn with_shield_recharge_cooldown(
        mut self,
        r#shield_recharge_cooldown: impl Into<f32>,
    ) -> Self {
        self.r#shield_recharge_cooldown = r#shield_recharge_cooldown.into();
        self
    }
    pub fn set_shield_recharge_cooldown(
        &mut self,
        r#shield_recharge_cooldown: impl Into<f32>,
    ) -> &mut Self {
        self.r#shield_recharge_cooldown = r#shield_recharge_cooldown.into();
        self
    }
    pub fn with_base_drone_reconstruction_speed(
        mut self,
        r#base_drone_reconstruction_speed: impl Into<f32>,
    ) -> Self {
        self.r#base_drone_reconstruction_speed = r#base_drone_reconstruction_speed.into();
        self
    }
    pub fn set_base_drone_reconstruction_speed(
        &mut self,
        r#base_drone_reconstruction_speed: impl Into<f32>,
    ) -> &mut Self {
        self.r#base_drone_reconstruction_speed = r#base_drone_reconstruction_speed.into();
        self
    }
    pub fn with_max_velocity(mut self, r#max_velocity: impl Into<f32>) -> Self {
        self.r#max_velocity = r#max_velocity.into();
        self
    }
    pub fn set_max_velocity(&mut self, r#max_velocity: impl Into<f32>) -> &mut Self {
        self.r#max_velocity = r#max_velocity.into();
        self
    }
    pub fn with_max_turn_rate(mut self, r#max_turn_rate: impl Into<f32>) -> Self {
        self.r#max_turn_rate = r#max_turn_rate.into();
        self
    }
    pub fn set_max_turn_rate(&mut self, r#max_turn_rate: impl Into<f32>) -> &mut Self {
        self.r#max_turn_rate = r#max_turn_rate.into();
        self
    }
}
impl DatabaseItem for ShipSettings {
    fn validate(&mut self) {
        if self.r#default_weight_per_cell < (1f32 as f32) {
            tracing::warn!(
                field = "r#default_weight_per_cell",
                value = self.r#default_weight_per_cell,
                min = 1f32,
                "Field got truncated"
            );
            self.r#default_weight_per_cell = 1f32 as f32;
        }
        if self.r#default_weight_per_cell > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#default_weight_per_cell",
                value = self.r#default_weight_per_cell,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#default_weight_per_cell = 1000000f32 as f32;
        }
        if self.r#minimum_weight_per_cell < (1f32 as f32) {
            tracing::warn!(
                field = "r#minimum_weight_per_cell",
                value = self.r#minimum_weight_per_cell,
                min = 1f32,
                "Field got truncated"
            );
            self.r#minimum_weight_per_cell = 1f32 as f32;
        }
        if self.r#minimum_weight_per_cell > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#minimum_weight_per_cell",
                value = self.r#minimum_weight_per_cell,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#minimum_weight_per_cell = 1000000f32 as f32;
        }
        if self.r#base_armor_points < (0f32 as f32) {
            tracing::warn!(
                field = "r#base_armor_points",
                value = self.r#base_armor_points,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_armor_points = 0f32 as f32;
        }
        if self.r#base_armor_points > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#base_armor_points",
                value = self.r#base_armor_points,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#base_armor_points = 1000000f32 as f32;
        }
        if self.r#armor_points_per_cell < (0f32 as f32) {
            tracing::warn!(
                field = "r#armor_points_per_cell",
                value = self.r#armor_points_per_cell,
                min = 0f32,
                "Field got truncated"
            );
            self.r#armor_points_per_cell = 0f32 as f32;
        }
        if self.r#armor_points_per_cell > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#armor_points_per_cell",
                value = self.r#armor_points_per_cell,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#armor_points_per_cell = 1000000f32 as f32;
        }
        if self.r#armor_repair_cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#armor_repair_cooldown",
                value = self.r#armor_repair_cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#armor_repair_cooldown = 0f32 as f32;
        }
        if self.r#armor_repair_cooldown > (60f32 as f32) {
            tracing::warn!(
                field = "r#armor_repair_cooldown",
                value = self.r#armor_repair_cooldown,
                max = 60f32,
                "Field got truncated"
            );
            self.r#armor_repair_cooldown = 60f32 as f32;
        }
        if self.r#base_energy_points < (0f32 as f32) {
            tracing::warn!(
                field = "r#base_energy_points",
                value = self.r#base_energy_points,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_energy_points = 0f32 as f32;
        }
        if self.r#base_energy_points > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#base_energy_points",
                value = self.r#base_energy_points,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#base_energy_points = 1000000f32 as f32;
        }
        if self.r#base_energy_recharge_rate < (0f32 as f32) {
            tracing::warn!(
                field = "r#base_energy_recharge_rate",
                value = self.r#base_energy_recharge_rate,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_energy_recharge_rate = 0f32 as f32;
        }
        if self.r#base_energy_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#base_energy_recharge_rate",
                value = self.r#base_energy_recharge_rate,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#base_energy_recharge_rate = 1000000f32 as f32;
        }
        if self.r#energy_recharge_cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#energy_recharge_cooldown",
                value = self.r#energy_recharge_cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#energy_recharge_cooldown = 0f32 as f32;
        }
        if self.r#energy_recharge_cooldown > (60f32 as f32) {
            tracing::warn!(
                field = "r#energy_recharge_cooldown",
                value = self.r#energy_recharge_cooldown,
                max = 60f32,
                "Field got truncated"
            );
            self.r#energy_recharge_cooldown = 60f32 as f32;
        }
        if self.r#base_shield_recharge_rate < (0f32 as f32) {
            tracing::warn!(
                field = "r#base_shield_recharge_rate",
                value = self.r#base_shield_recharge_rate,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_shield_recharge_rate = 0f32 as f32;
        }
        if self.r#base_shield_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#base_shield_recharge_rate",
                value = self.r#base_shield_recharge_rate,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#base_shield_recharge_rate = 1000000f32 as f32;
        }
        if self.r#shield_recharge_cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#shield_recharge_cooldown",
                value = self.r#shield_recharge_cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#shield_recharge_cooldown = 0f32 as f32;
        }
        if self.r#shield_recharge_cooldown > (60f32 as f32) {
            tracing::warn!(
                field = "r#shield_recharge_cooldown",
                value = self.r#shield_recharge_cooldown,
                max = 60f32,
                "Field got truncated"
            );
            self.r#shield_recharge_cooldown = 60f32 as f32;
        }
        if self.r#base_drone_reconstruction_speed < (0f32 as f32) {
            tracing::warn!(
                field = "r#base_drone_reconstruction_speed",
                value = self.r#base_drone_reconstruction_speed,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_drone_reconstruction_speed = 0f32 as f32;
        }
        if self.r#base_drone_reconstruction_speed > (100f32 as f32) {
            tracing::warn!(
                field = "r#base_drone_reconstruction_speed",
                value = self.r#base_drone_reconstruction_speed,
                max = 100f32,
                "Field got truncated"
            );
            self.r#base_drone_reconstruction_speed = 100f32 as f32;
        }
        if self.r#max_velocity < (5f32 as f32) {
            tracing::warn!(
                field = "r#max_velocity",
                value = self.r#max_velocity,
                min = 5f32,
                "Field got truncated"
            );
            self.r#max_velocity = 5f32 as f32;
        }
        if self.r#max_velocity > (30f32 as f32) {
            tracing::warn!(
                field = "r#max_velocity",
                value = self.r#max_velocity,
                max = 30f32,
                "Field got truncated"
            );
            self.r#max_velocity = 30f32 as f32;
        }
        if self.r#max_turn_rate < (5f32 as f32) {
            tracing::warn!(
                field = "r#max_turn_rate",
                value = self.r#max_turn_rate,
                min = 5f32,
                "Field got truncated"
            );
            self.r#max_turn_rate = 5f32 as f32;
        }
        if self.r#max_turn_rate > (30f32 as f32) {
            tracing::warn!(
                field = "r#max_turn_rate",
                value = self.r#max_turn_rate,
                max = 30f32,
                "Field got truncated"
            );
            self.r#max_turn_rate = 30f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "ShipSettings"
    }
}
impl Default for ShipSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/SkillSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SkillSettings {
    pub r#beat_all_enemies_faction_list: Vec<FactionId>,
    pub r#disable_exceed_the_limits: bool,
    pub r#fuel_tank_capacity: String,
    pub r#attack_bonus: String,
    pub r#defense_bonus: String,
    pub r#shield_strength_bonus: String,
    pub r#shield_recharge_bonus: String,
    pub r#experience_bonus: String,
    pub r#flight_speed: String,
    pub r#flight_range: String,
    pub r#exploration_loot_bonus: String,
    pub r#heat_resistance: String,
    pub r#kinetic_resistance: String,
    pub r#energy_resistance: String,
    pub r#merchant_price_factor: String,
    pub r#crafting_price_factor: String,
    pub r#crafting_level_reduction: String,
    pub r#max_player_ships_level: i32,
    pub r#increased_level_limit: i32,
    pub r#base_fuel_capacity: i32,
    pub r#base_flight_range: f32,
    pub r#base_flight_speed: f32,
}
impl SkillSettings {
    pub fn new() -> Self {
        Self {
            r#beat_all_enemies_faction_list: Default::default(),
            r#disable_exceed_the_limits: Default::default(),
            r#fuel_tank_capacity: "BaseFuelCapacity + 50*level".to_string(),
            r#attack_bonus: "0.1*level".to_string(),
            r#defense_bonus: "0.1*level".to_string(),
            r#shield_strength_bonus: "0.1*level".to_string(),
            r#shield_recharge_bonus: "0.1*level".to_string(),
            r#experience_bonus: "0.1*level".to_string(),
            r#flight_speed: "BaseFlightSpeed + 0.4*level".to_string(),
            r#flight_range: "BaseFlightRange + 0.09*level".to_string(),
            r#exploration_loot_bonus: "0.1*level".to_string(),
            r#heat_resistance: "0.1*level".to_string(),
            r#kinetic_resistance: "0.1*level".to_string(),
            r#energy_resistance: "0.1*level".to_string(),
            r#merchant_price_factor: "1 - 0.05*level".to_string(),
            r#crafting_price_factor: "1 - 0.05*level".to_string(),
            r#crafting_level_reduction: "5*level".to_string(),
            r#max_player_ships_level: 100i32,
            r#increased_level_limit: 200i32,
            r#base_fuel_capacity: 100i32,
            r#base_flight_range: 1.5f32,
            r#base_flight_speed: 1f32,
        }
    }
    pub fn with_beat_all_enemies_faction_list(
        mut self,
        r#beat_all_enemies_faction_list: impl Into<Vec<FactionId>>,
    ) -> Self {
        self.r#beat_all_enemies_faction_list = r#beat_all_enemies_faction_list.into();
        self
    }
    pub fn set_beat_all_enemies_faction_list(
        &mut self,
        r#beat_all_enemies_faction_list: impl Into<Vec<FactionId>>,
    ) -> &mut Self {
        self.r#beat_all_enemies_faction_list = r#beat_all_enemies_faction_list.into();
        self
    }
    pub fn with_disable_exceed_the_limits(
        mut self,
        r#disable_exceed_the_limits: impl Into<bool>,
    ) -> Self {
        self.r#disable_exceed_the_limits = r#disable_exceed_the_limits.into();
        self
    }
    pub fn set_disable_exceed_the_limits(
        &mut self,
        r#disable_exceed_the_limits: impl Into<bool>,
    ) -> &mut Self {
        self.r#disable_exceed_the_limits = r#disable_exceed_the_limits.into();
        self
    }
    pub fn with_fuel_tank_capacity(mut self, r#fuel_tank_capacity: impl Into<String>) -> Self {
        self.r#fuel_tank_capacity = r#fuel_tank_capacity.into();
        self
    }
    pub fn set_fuel_tank_capacity(&mut self, r#fuel_tank_capacity: impl Into<String>) -> &mut Self {
        self.r#fuel_tank_capacity = r#fuel_tank_capacity.into();
        self
    }
    pub fn with_attack_bonus(mut self, r#attack_bonus: impl Into<String>) -> Self {
        self.r#attack_bonus = r#attack_bonus.into();
        self
    }
    pub fn set_attack_bonus(&mut self, r#attack_bonus: impl Into<String>) -> &mut Self {
        self.r#attack_bonus = r#attack_bonus.into();
        self
    }
    pub fn with_defense_bonus(mut self, r#defense_bonus: impl Into<String>) -> Self {
        self.r#defense_bonus = r#defense_bonus.into();
        self
    }
    pub fn set_defense_bonus(&mut self, r#defense_bonus: impl Into<String>) -> &mut Self {
        self.r#defense_bonus = r#defense_bonus.into();
        self
    }
    pub fn with_shield_strength_bonus(
        mut self,
        r#shield_strength_bonus: impl Into<String>,
    ) -> Self {
        self.r#shield_strength_bonus = r#shield_strength_bonus.into();
        self
    }
    pub fn set_shield_strength_bonus(
        &mut self,
        r#shield_strength_bonus: impl Into<String>,
    ) -> &mut Self {
        self.r#shield_strength_bonus = r#shield_strength_bonus.into();
        self
    }
    pub fn with_shield_recharge_bonus(
        mut self,
        r#shield_recharge_bonus: impl Into<String>,
    ) -> Self {
        self.r#shield_recharge_bonus = r#shield_recharge_bonus.into();
        self
    }
    pub fn set_shield_recharge_bonus(
        &mut self,
        r#shield_recharge_bonus: impl Into<String>,
    ) -> &mut Self {
        self.r#shield_recharge_bonus = r#shield_recharge_bonus.into();
        self
    }
    pub fn with_experience_bonus(mut self, r#experience_bonus: impl Into<String>) -> Self {
        self.r#experience_bonus = r#experience_bonus.into();
        self
    }
    pub fn set_experience_bonus(&mut self, r#experience_bonus: impl Into<String>) -> &mut Self {
        self.r#experience_bonus = r#experience_bonus.into();
        self
    }
    pub fn with_flight_speed(mut self, r#flight_speed: impl Into<String>) -> Self {
        self.r#flight_speed = r#flight_speed.into();
        self
    }
    pub fn set_flight_speed(&mut self, r#flight_speed: impl Into<String>) -> &mut Self {
        self.r#flight_speed = r#flight_speed.into();
        self
    }
    pub fn with_flight_range(mut self, r#flight_range: impl Into<String>) -> Self {
        self.r#flight_range = r#flight_range.into();
        self
    }
    pub fn set_flight_range(&mut self, r#flight_range: impl Into<String>) -> &mut Self {
        self.r#flight_range = r#flight_range.into();
        self
    }
    pub fn with_exploration_loot_bonus(
        mut self,
        r#exploration_loot_bonus: impl Into<String>,
    ) -> Self {
        self.r#exploration_loot_bonus = r#exploration_loot_bonus.into();
        self
    }
    pub fn set_exploration_loot_bonus(
        &mut self,
        r#exploration_loot_bonus: impl Into<String>,
    ) -> &mut Self {
        self.r#exploration_loot_bonus = r#exploration_loot_bonus.into();
        self
    }
    pub fn with_heat_resistance(mut self, r#heat_resistance: impl Into<String>) -> Self {
        self.r#heat_resistance = r#heat_resistance.into();
        self
    }
    pub fn set_heat_resistance(&mut self, r#heat_resistance: impl Into<String>) -> &mut Self {
        self.r#heat_resistance = r#heat_resistance.into();
        self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: impl Into<String>) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn set_kinetic_resistance(&mut self, r#kinetic_resistance: impl Into<String>) -> &mut Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn with_energy_resistance(mut self, r#energy_resistance: impl Into<String>) -> Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn set_energy_resistance(&mut self, r#energy_resistance: impl Into<String>) -> &mut Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn with_merchant_price_factor(
        mut self,
        r#merchant_price_factor: impl Into<String>,
    ) -> Self {
        self.r#merchant_price_factor = r#merchant_price_factor.into();
        self
    }
    pub fn set_merchant_price_factor(
        &mut self,
        r#merchant_price_factor: impl Into<String>,
    ) -> &mut Self {
        self.r#merchant_price_factor = r#merchant_price_factor.into();
        self
    }
    pub fn with_crafting_price_factor(
        mut self,
        r#crafting_price_factor: impl Into<String>,
    ) -> Self {
        self.r#crafting_price_factor = r#crafting_price_factor.into();
        self
    }
    pub fn set_crafting_price_factor(
        &mut self,
        r#crafting_price_factor: impl Into<String>,
    ) -> &mut Self {
        self.r#crafting_price_factor = r#crafting_price_factor.into();
        self
    }
    pub fn with_crafting_level_reduction(
        mut self,
        r#crafting_level_reduction: impl Into<String>,
    ) -> Self {
        self.r#crafting_level_reduction = r#crafting_level_reduction.into();
        self
    }
    pub fn set_crafting_level_reduction(
        &mut self,
        r#crafting_level_reduction: impl Into<String>,
    ) -> &mut Self {
        self.r#crafting_level_reduction = r#crafting_level_reduction.into();
        self
    }
    pub fn with_max_player_ships_level(mut self, r#max_player_ships_level: impl Into<i32>) -> Self {
        self.r#max_player_ships_level = r#max_player_ships_level.into();
        self
    }
    pub fn set_max_player_ships_level(
        &mut self,
        r#max_player_ships_level: impl Into<i32>,
    ) -> &mut Self {
        self.r#max_player_ships_level = r#max_player_ships_level.into();
        self
    }
    pub fn with_increased_level_limit(mut self, r#increased_level_limit: impl Into<i32>) -> Self {
        self.r#increased_level_limit = r#increased_level_limit.into();
        self
    }
    pub fn set_increased_level_limit(
        &mut self,
        r#increased_level_limit: impl Into<i32>,
    ) -> &mut Self {
        self.r#increased_level_limit = r#increased_level_limit.into();
        self
    }
    pub fn with_base_fuel_capacity(mut self, r#base_fuel_capacity: impl Into<i32>) -> Self {
        self.r#base_fuel_capacity = r#base_fuel_capacity.into();
        self
    }
    pub fn set_base_fuel_capacity(&mut self, r#base_fuel_capacity: impl Into<i32>) -> &mut Self {
        self.r#base_fuel_capacity = r#base_fuel_capacity.into();
        self
    }
    pub fn with_base_flight_range(mut self, r#base_flight_range: impl Into<f32>) -> Self {
        self.r#base_flight_range = r#base_flight_range.into();
        self
    }
    pub fn set_base_flight_range(&mut self, r#base_flight_range: impl Into<f32>) -> &mut Self {
        self.r#base_flight_range = r#base_flight_range.into();
        self
    }
    pub fn with_base_flight_speed(mut self, r#base_flight_speed: impl Into<f32>) -> Self {
        self.r#base_flight_speed = r#base_flight_speed.into();
        self
    }
    pub fn set_base_flight_speed(&mut self, r#base_flight_speed: impl Into<f32>) -> &mut Self {
        self.r#base_flight_speed = r#base_flight_speed.into();
        self
    }
}
impl DatabaseItem for SkillSettings {
    fn validate(&mut self) {
        if self.r#max_player_ships_level < (0f32 as i32) {
            tracing::warn!(
                field = "r#max_player_ships_level",
                value = self.r#max_player_ships_level,
                min = 0f32,
                "Field got truncated"
            );
            self.r#max_player_ships_level = 0f32 as i32;
        }
        if self.r#max_player_ships_level > (500f32 as i32) {
            tracing::warn!(
                field = "r#max_player_ships_level",
                value = self.r#max_player_ships_level,
                max = 500f32,
                "Field got truncated"
            );
            self.r#max_player_ships_level = 500f32 as i32;
        }
        if self.r#increased_level_limit < (0f32 as i32) {
            tracing::warn!(
                field = "r#increased_level_limit",
                value = self.r#increased_level_limit,
                min = 0f32,
                "Field got truncated"
            );
            self.r#increased_level_limit = 0f32 as i32;
        }
        if self.r#increased_level_limit > (1000f32 as i32) {
            tracing::warn!(
                field = "r#increased_level_limit",
                value = self.r#increased_level_limit,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#increased_level_limit = 1000f32 as i32;
        }
        if self.r#base_fuel_capacity < (10f32 as i32) {
            tracing::warn!(
                field = "r#base_fuel_capacity",
                value = self.r#base_fuel_capacity,
                min = 10f32,
                "Field got truncated"
            );
            self.r#base_fuel_capacity = 10f32 as i32;
        }
        if self.r#base_flight_range < (1.5f32 as f32) {
            tracing::warn!(
                field = "r#base_flight_range",
                value = self.r#base_flight_range,
                min = 1.5f32,
                "Field got truncated"
            );
            self.r#base_flight_range = 1.5f32 as f32;
        }
        if self.r#base_flight_speed < (1f32 as f32) {
            tracing::warn!(
                field = "r#base_flight_speed",
                value = self.r#base_flight_speed,
                min = 1f32,
                "Field got truncated"
            );
            self.r#base_flight_speed = 1f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "SkillSettings"
    }
}
impl Default for SkillSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/SpecialEventSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialEventSettings {
    pub r#enable_xmas_event: bool,
    pub r#xmas_days_before: i32,
    pub r#xmas_days_after: i32,
    pub r#xmas_quest: Option<QuestId>,
    pub r#xmas_combat_rules: Option<CombatRulesId>,
    pub r#convert_credits_to_snowflakes: String,
    pub r#enable_easter_event: bool,
    pub r#easter_days_before: i32,
    pub r#easter_days_after: i32,
    pub r#easter_quest: Option<QuestId>,
    pub r#enable_halloween_event: bool,
    pub r#halloween_days_before: i32,
    pub r#halloween_days_after: i32,
    pub r#halloween_quest: Option<QuestId>,
}
impl SpecialEventSettings {
    pub fn new() -> Self {
        Self {
            r#enable_xmas_event: true,
            r#xmas_days_before: 24i32,
            r#xmas_days_after: 15i32,
            r#xmas_quest: Default::default(),
            r#xmas_combat_rules: Default::default(),
            r#convert_credits_to_snowflakes: "1 + credits/500".to_string(),
            r#enable_easter_event: Default::default(),
            r#easter_days_before: Default::default(),
            r#easter_days_after: Default::default(),
            r#easter_quest: Default::default(),
            r#enable_halloween_event: Default::default(),
            r#halloween_days_before: Default::default(),
            r#halloween_days_after: Default::default(),
            r#halloween_quest: Default::default(),
        }
    }
    pub fn with_enable_xmas_event(mut self, r#enable_xmas_event: impl Into<bool>) -> Self {
        self.r#enable_xmas_event = r#enable_xmas_event.into();
        self
    }
    pub fn set_enable_xmas_event(&mut self, r#enable_xmas_event: impl Into<bool>) -> &mut Self {
        self.r#enable_xmas_event = r#enable_xmas_event.into();
        self
    }
    pub fn with_xmas_days_before(mut self, r#xmas_days_before: impl Into<i32>) -> Self {
        self.r#xmas_days_before = r#xmas_days_before.into();
        self
    }
    pub fn set_xmas_days_before(&mut self, r#xmas_days_before: impl Into<i32>) -> &mut Self {
        self.r#xmas_days_before = r#xmas_days_before.into();
        self
    }
    pub fn with_xmas_days_after(mut self, r#xmas_days_after: impl Into<i32>) -> Self {
        self.r#xmas_days_after = r#xmas_days_after.into();
        self
    }
    pub fn set_xmas_days_after(&mut self, r#xmas_days_after: impl Into<i32>) -> &mut Self {
        self.r#xmas_days_after = r#xmas_days_after.into();
        self
    }
    pub fn with_xmas_quest(mut self, r#xmas_quest: impl Into<Option<QuestId>>) -> Self {
        self.r#xmas_quest = r#xmas_quest.into();
        self
    }
    pub fn set_xmas_quest(&mut self, r#xmas_quest: impl Into<Option<QuestId>>) -> &mut Self {
        self.r#xmas_quest = r#xmas_quest.into();
        self
    }
    pub fn with_xmas_combat_rules(
        mut self,
        r#xmas_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> Self {
        self.r#xmas_combat_rules = r#xmas_combat_rules.into();
        self
    }
    pub fn set_xmas_combat_rules(
        &mut self,
        r#xmas_combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#xmas_combat_rules = r#xmas_combat_rules.into();
        self
    }
    pub fn with_convert_credits_to_snowflakes(
        mut self,
        r#convert_credits_to_snowflakes: impl Into<String>,
    ) -> Self {
        self.r#convert_credits_to_snowflakes = r#convert_credits_to_snowflakes.into();
        self
    }
    pub fn set_convert_credits_to_snowflakes(
        &mut self,
        r#convert_credits_to_snowflakes: impl Into<String>,
    ) -> &mut Self {
        self.r#convert_credits_to_snowflakes = r#convert_credits_to_snowflakes.into();
        self
    }
    pub fn with_enable_easter_event(mut self, r#enable_easter_event: impl Into<bool>) -> Self {
        self.r#enable_easter_event = r#enable_easter_event.into();
        self
    }
    pub fn set_enable_easter_event(&mut self, r#enable_easter_event: impl Into<bool>) -> &mut Self {
        self.r#enable_easter_event = r#enable_easter_event.into();
        self
    }
    pub fn with_easter_days_before(mut self, r#easter_days_before: impl Into<i32>) -> Self {
        self.r#easter_days_before = r#easter_days_before.into();
        self
    }
    pub fn set_easter_days_before(&mut self, r#easter_days_before: impl Into<i32>) -> &mut Self {
        self.r#easter_days_before = r#easter_days_before.into();
        self
    }
    pub fn with_easter_days_after(mut self, r#easter_days_after: impl Into<i32>) -> Self {
        self.r#easter_days_after = r#easter_days_after.into();
        self
    }
    pub fn set_easter_days_after(&mut self, r#easter_days_after: impl Into<i32>) -> &mut Self {
        self.r#easter_days_after = r#easter_days_after.into();
        self
    }
    pub fn with_easter_quest(mut self, r#easter_quest: impl Into<Option<QuestId>>) -> Self {
        self.r#easter_quest = r#easter_quest.into();
        self
    }
    pub fn set_easter_quest(&mut self, r#easter_quest: impl Into<Option<QuestId>>) -> &mut Self {
        self.r#easter_quest = r#easter_quest.into();
        self
    }
    pub fn with_enable_halloween_event(
        mut self,
        r#enable_halloween_event: impl Into<bool>,
    ) -> Self {
        self.r#enable_halloween_event = r#enable_halloween_event.into();
        self
    }
    pub fn set_enable_halloween_event(
        &mut self,
        r#enable_halloween_event: impl Into<bool>,
    ) -> &mut Self {
        self.r#enable_halloween_event = r#enable_halloween_event.into();
        self
    }
    pub fn with_halloween_days_before(mut self, r#halloween_days_before: impl Into<i32>) -> Self {
        self.r#halloween_days_before = r#halloween_days_before.into();
        self
    }
    pub fn set_halloween_days_before(
        &mut self,
        r#halloween_days_before: impl Into<i32>,
    ) -> &mut Self {
        self.r#halloween_days_before = r#halloween_days_before.into();
        self
    }
    pub fn with_halloween_days_after(mut self, r#halloween_days_after: impl Into<i32>) -> Self {
        self.r#halloween_days_after = r#halloween_days_after.into();
        self
    }
    pub fn set_halloween_days_after(
        &mut self,
        r#halloween_days_after: impl Into<i32>,
    ) -> &mut Self {
        self.r#halloween_days_after = r#halloween_days_after.into();
        self
    }
    pub fn with_halloween_quest(mut self, r#halloween_quest: impl Into<Option<QuestId>>) -> Self {
        self.r#halloween_quest = r#halloween_quest.into();
        self
    }
    pub fn set_halloween_quest(
        &mut self,
        r#halloween_quest: impl Into<Option<QuestId>>,
    ) -> &mut Self {
        self.r#halloween_quest = r#halloween_quest.into();
        self
    }
}
impl DatabaseItem for SpecialEventSettings {
    fn validate(&mut self) {
        if self.r#xmas_days_before < (0f32 as i32) {
            tracing::warn!(
                field = "r#xmas_days_before",
                value = self.r#xmas_days_before,
                min = 0f32,
                "Field got truncated"
            );
            self.r#xmas_days_before = 0f32 as i32;
        }
        if self.r#xmas_days_before > (30f32 as i32) {
            tracing::warn!(
                field = "r#xmas_days_before",
                value = self.r#xmas_days_before,
                max = 30f32,
                "Field got truncated"
            );
            self.r#xmas_days_before = 30f32 as i32;
        }
        if self.r#xmas_days_after < (0f32 as i32) {
            tracing::warn!(
                field = "r#xmas_days_after",
                value = self.r#xmas_days_after,
                min = 0f32,
                "Field got truncated"
            );
            self.r#xmas_days_after = 0f32 as i32;
        }
        if self.r#xmas_days_after > (30f32 as i32) {
            tracing::warn!(
                field = "r#xmas_days_after",
                value = self.r#xmas_days_after,
                max = 30f32,
                "Field got truncated"
            );
            self.r#xmas_days_after = 30f32 as i32;
        }
        if self.r#easter_days_before < (0f32 as i32) {
            tracing::warn!(
                field = "r#easter_days_before",
                value = self.r#easter_days_before,
                min = 0f32,
                "Field got truncated"
            );
            self.r#easter_days_before = 0f32 as i32;
        }
        if self.r#easter_days_before > (30f32 as i32) {
            tracing::warn!(
                field = "r#easter_days_before",
                value = self.r#easter_days_before,
                max = 30f32,
                "Field got truncated"
            );
            self.r#easter_days_before = 30f32 as i32;
        }
        if self.r#easter_days_after < (0f32 as i32) {
            tracing::warn!(
                field = "r#easter_days_after",
                value = self.r#easter_days_after,
                min = 0f32,
                "Field got truncated"
            );
            self.r#easter_days_after = 0f32 as i32;
        }
        if self.r#easter_days_after > (30f32 as i32) {
            tracing::warn!(
                field = "r#easter_days_after",
                value = self.r#easter_days_after,
                max = 30f32,
                "Field got truncated"
            );
            self.r#easter_days_after = 30f32 as i32;
        }
        if self.r#halloween_days_before < (0f32 as i32) {
            tracing::warn!(
                field = "r#halloween_days_before",
                value = self.r#halloween_days_before,
                min = 0f32,
                "Field got truncated"
            );
            self.r#halloween_days_before = 0f32 as i32;
        }
        if self.r#halloween_days_before > (30f32 as i32) {
            tracing::warn!(
                field = "r#halloween_days_before",
                value = self.r#halloween_days_before,
                max = 30f32,
                "Field got truncated"
            );
            self.r#halloween_days_before = 30f32 as i32;
        }
        if self.r#halloween_days_after < (0f32 as i32) {
            tracing::warn!(
                field = "r#halloween_days_after",
                value = self.r#halloween_days_after,
                min = 0f32,
                "Field got truncated"
            );
            self.r#halloween_days_after = 0f32 as i32;
        }
        if self.r#halloween_days_after > (30f32 as i32) {
            tracing::warn!(
                field = "r#halloween_days_after",
                value = self.r#halloween_days_after,
                max = 30f32,
                "Field got truncated"
            );
            self.r#halloween_days_after = 30f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "SpecialEventSettings"
    }
}
impl Default for SpecialEventSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/UiSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct UiSettings {
    pub r#window_color: String,
    pub r#scroll_bar_color: String,
    pub r#icon_color: String,
    pub r#selection_color: String,
    pub r#button_color: String,
    pub r#button_focus_color: String,
    pub r#button_text_color: String,
    pub r#button_icon_color: String,
    pub r#warning_button_color: String,
    pub r#warning_button_focus_color: String,
    pub r#warning_button_text_color: String,
    pub r#warning_button_icon_color: String,
    pub r#premium_button_color: String,
    pub r#premium_button_focus_color: String,
    pub r#premium_button_text_color: String,
    pub r#premium_button_icon_color: String,
    pub r#text_color: String,
    pub r#error_text_color: String,
    pub r#header_text_color: String,
    pub r#pale_text_color: String,
    pub r#bright_text_color: String,
    pub r#background_dark: String,
    pub r#low_quality_item_color: String,
    pub r#common_quality_item_color: String,
    pub r#medium_quality_item_color: String,
    pub r#high_quality_item_color: String,
    pub r#perfect_quality_item_color: String,
    pub r#available_tech_color: String,
    pub r#unavailable_tech_color: String,
    pub r#obtained_tech_color: String,
    pub r#hidden_tech_color: String,
    pub r#credits_color: String,
    pub r#stars_color: String,
    pub r#money_color: String,
    pub r#fuel_color: String,
    pub r#tokens_color: String,
}
impl UiSettings {
    pub fn new() -> Self {
        Self {
            r#window_color: "#50C0FF".to_string(),
            r#scroll_bar_color: "#C050C0FF".to_string(),
            r#icon_color: "#80FFFF".to_string(),
            r#selection_color: "#80FFFF".to_string(),
            r#button_color: "#50C0FF".to_string(),
            r#button_focus_color: "#4050C0FF".to_string(),
            r#button_text_color: "#80FFFF".to_string(),
            r#button_icon_color: "#E080FFFF".to_string(),
            r#warning_button_color: "#FF8050".to_string(),
            r#warning_button_focus_color: "#20FF8050".to_string(),
            r#warning_button_text_color: "#FFFFC0".to_string(),
            r#warning_button_icon_color: "#FFFFC0".to_string(),
            r#premium_button_color: "#FFFFC0".to_string(),
            r#premium_button_focus_color: "#40FFFFC0".to_string(),
            r#premium_button_text_color: "#FFFFE0".to_string(),
            r#premium_button_icon_color: "#FFFFC0".to_string(),
            r#text_color: "#80FFFF".to_string(),
            r#error_text_color: "#FFC000".to_string(),
            r#header_text_color: "#FFFFC0".to_string(),
            r#pale_text_color: "#A0FFFFFF".to_string(),
            r#bright_text_color: "#FFFFFF".to_string(),
            r#background_dark: "#000000".to_string(),
            r#low_quality_item_color: "#C0C0C0".to_string(),
            r#common_quality_item_color: "#80FFFF".to_string(),
            r#medium_quality_item_color: "#80FF80".to_string(),
            r#high_quality_item_color: "#F09FFF".to_string(),
            r#perfect_quality_item_color: "#FFDF51".to_string(),
            r#available_tech_color: "#FFFFC0".to_string(),
            r#unavailable_tech_color: "#808080".to_string(),
            r#obtained_tech_color: "#50C0FF".to_string(),
            r#hidden_tech_color: "#8080FF".to_string(),
            r#credits_color: "#00FF00".to_string(),
            r#stars_color: "#FFF0A0".to_string(),
            r#money_color: "#FFF0A0".to_string(),
            r#fuel_color: "#00FFFF".to_string(),
            r#tokens_color: "#8080FF".to_string(),
        }
    }
    pub fn with_window_color(mut self, r#window_color: impl Into<String>) -> Self {
        self.r#window_color = r#window_color.into();
        self
    }
    pub fn set_window_color(&mut self, r#window_color: impl Into<String>) -> &mut Self {
        self.r#window_color = r#window_color.into();
        self
    }
    pub fn with_scroll_bar_color(mut self, r#scroll_bar_color: impl Into<String>) -> Self {
        self.r#scroll_bar_color = r#scroll_bar_color.into();
        self
    }
    pub fn set_scroll_bar_color(&mut self, r#scroll_bar_color: impl Into<String>) -> &mut Self {
        self.r#scroll_bar_color = r#scroll_bar_color.into();
        self
    }
    pub fn with_icon_color(mut self, r#icon_color: impl Into<String>) -> Self {
        self.r#icon_color = r#icon_color.into();
        self
    }
    pub fn set_icon_color(&mut self, r#icon_color: impl Into<String>) -> &mut Self {
        self.r#icon_color = r#icon_color.into();
        self
    }
    pub fn with_selection_color(mut self, r#selection_color: impl Into<String>) -> Self {
        self.r#selection_color = r#selection_color.into();
        self
    }
    pub fn set_selection_color(&mut self, r#selection_color: impl Into<String>) -> &mut Self {
        self.r#selection_color = r#selection_color.into();
        self
    }
    pub fn with_button_color(mut self, r#button_color: impl Into<String>) -> Self {
        self.r#button_color = r#button_color.into();
        self
    }
    pub fn set_button_color(&mut self, r#button_color: impl Into<String>) -> &mut Self {
        self.r#button_color = r#button_color.into();
        self
    }
    pub fn with_button_focus_color(mut self, r#button_focus_color: impl Into<String>) -> Self {
        self.r#button_focus_color = r#button_focus_color.into();
        self
    }
    pub fn set_button_focus_color(&mut self, r#button_focus_color: impl Into<String>) -> &mut Self {
        self.r#button_focus_color = r#button_focus_color.into();
        self
    }
    pub fn with_button_text_color(mut self, r#button_text_color: impl Into<String>) -> Self {
        self.r#button_text_color = r#button_text_color.into();
        self
    }
    pub fn set_button_text_color(&mut self, r#button_text_color: impl Into<String>) -> &mut Self {
        self.r#button_text_color = r#button_text_color.into();
        self
    }
    pub fn with_button_icon_color(mut self, r#button_icon_color: impl Into<String>) -> Self {
        self.r#button_icon_color = r#button_icon_color.into();
        self
    }
    pub fn set_button_icon_color(&mut self, r#button_icon_color: impl Into<String>) -> &mut Self {
        self.r#button_icon_color = r#button_icon_color.into();
        self
    }
    pub fn with_warning_button_color(mut self, r#warning_button_color: impl Into<String>) -> Self {
        self.r#warning_button_color = r#warning_button_color.into();
        self
    }
    pub fn set_warning_button_color(
        &mut self,
        r#warning_button_color: impl Into<String>,
    ) -> &mut Self {
        self.r#warning_button_color = r#warning_button_color.into();
        self
    }
    pub fn with_warning_button_focus_color(
        mut self,
        r#warning_button_focus_color: impl Into<String>,
    ) -> Self {
        self.r#warning_button_focus_color = r#warning_button_focus_color.into();
        self
    }
    pub fn set_warning_button_focus_color(
        &mut self,
        r#warning_button_focus_color: impl Into<String>,
    ) -> &mut Self {
        self.r#warning_button_focus_color = r#warning_button_focus_color.into();
        self
    }
    pub fn with_warning_button_text_color(
        mut self,
        r#warning_button_text_color: impl Into<String>,
    ) -> Self {
        self.r#warning_button_text_color = r#warning_button_text_color.into();
        self
    }
    pub fn set_warning_button_text_color(
        &mut self,
        r#warning_button_text_color: impl Into<String>,
    ) -> &mut Self {
        self.r#warning_button_text_color = r#warning_button_text_color.into();
        self
    }
    pub fn with_warning_button_icon_color(
        mut self,
        r#warning_button_icon_color: impl Into<String>,
    ) -> Self {
        self.r#warning_button_icon_color = r#warning_button_icon_color.into();
        self
    }
    pub fn set_warning_button_icon_color(
        &mut self,
        r#warning_button_icon_color: impl Into<String>,
    ) -> &mut Self {
        self.r#warning_button_icon_color = r#warning_button_icon_color.into();
        self
    }
    pub fn with_premium_button_color(mut self, r#premium_button_color: impl Into<String>) -> Self {
        self.r#premium_button_color = r#premium_button_color.into();
        self
    }
    pub fn set_premium_button_color(
        &mut self,
        r#premium_button_color: impl Into<String>,
    ) -> &mut Self {
        self.r#premium_button_color = r#premium_button_color.into();
        self
    }
    pub fn with_premium_button_focus_color(
        mut self,
        r#premium_button_focus_color: impl Into<String>,
    ) -> Self {
        self.r#premium_button_focus_color = r#premium_button_focus_color.into();
        self
    }
    pub fn set_premium_button_focus_color(
        &mut self,
        r#premium_button_focus_color: impl Into<String>,
    ) -> &mut Self {
        self.r#premium_button_focus_color = r#premium_button_focus_color.into();
        self
    }
    pub fn with_premium_button_text_color(
        mut self,
        r#premium_button_text_color: impl Into<String>,
    ) -> Self {
        self.r#premium_button_text_color = r#premium_button_text_color.into();
        self
    }
    pub fn set_premium_button_text_color(
        &mut self,
        r#premium_button_text_color: impl Into<String>,
    ) -> &mut Self {
        self.r#premium_button_text_color = r#premium_button_text_color.into();
        self
    }
    pub fn with_premium_button_icon_color(
        mut self,
        r#premium_button_icon_color: impl Into<String>,
    ) -> Self {
        self.r#premium_button_icon_color = r#premium_button_icon_color.into();
        self
    }
    pub fn set_premium_button_icon_color(
        &mut self,
        r#premium_button_icon_color: impl Into<String>,
    ) -> &mut Self {
        self.r#premium_button_icon_color = r#premium_button_icon_color.into();
        self
    }
    pub fn with_text_color(mut self, r#text_color: impl Into<String>) -> Self {
        self.r#text_color = r#text_color.into();
        self
    }
    pub fn set_text_color(&mut self, r#text_color: impl Into<String>) -> &mut Self {
        self.r#text_color = r#text_color.into();
        self
    }
    pub fn with_error_text_color(mut self, r#error_text_color: impl Into<String>) -> Self {
        self.r#error_text_color = r#error_text_color.into();
        self
    }
    pub fn set_error_text_color(&mut self, r#error_text_color: impl Into<String>) -> &mut Self {
        self.r#error_text_color = r#error_text_color.into();
        self
    }
    pub fn with_header_text_color(mut self, r#header_text_color: impl Into<String>) -> Self {
        self.r#header_text_color = r#header_text_color.into();
        self
    }
    pub fn set_header_text_color(&mut self, r#header_text_color: impl Into<String>) -> &mut Self {
        self.r#header_text_color = r#header_text_color.into();
        self
    }
    pub fn with_pale_text_color(mut self, r#pale_text_color: impl Into<String>) -> Self {
        self.r#pale_text_color = r#pale_text_color.into();
        self
    }
    pub fn set_pale_text_color(&mut self, r#pale_text_color: impl Into<String>) -> &mut Self {
        self.r#pale_text_color = r#pale_text_color.into();
        self
    }
    pub fn with_bright_text_color(mut self, r#bright_text_color: impl Into<String>) -> Self {
        self.r#bright_text_color = r#bright_text_color.into();
        self
    }
    pub fn set_bright_text_color(&mut self, r#bright_text_color: impl Into<String>) -> &mut Self {
        self.r#bright_text_color = r#bright_text_color.into();
        self
    }
    pub fn with_background_dark(mut self, r#background_dark: impl Into<String>) -> Self {
        self.r#background_dark = r#background_dark.into();
        self
    }
    pub fn set_background_dark(&mut self, r#background_dark: impl Into<String>) -> &mut Self {
        self.r#background_dark = r#background_dark.into();
        self
    }
    pub fn with_low_quality_item_color(
        mut self,
        r#low_quality_item_color: impl Into<String>,
    ) -> Self {
        self.r#low_quality_item_color = r#low_quality_item_color.into();
        self
    }
    pub fn set_low_quality_item_color(
        &mut self,
        r#low_quality_item_color: impl Into<String>,
    ) -> &mut Self {
        self.r#low_quality_item_color = r#low_quality_item_color.into();
        self
    }
    pub fn with_common_quality_item_color(
        mut self,
        r#common_quality_item_color: impl Into<String>,
    ) -> Self {
        self.r#common_quality_item_color = r#common_quality_item_color.into();
        self
    }
    pub fn set_common_quality_item_color(
        &mut self,
        r#common_quality_item_color: impl Into<String>,
    ) -> &mut Self {
        self.r#common_quality_item_color = r#common_quality_item_color.into();
        self
    }
    pub fn with_medium_quality_item_color(
        mut self,
        r#medium_quality_item_color: impl Into<String>,
    ) -> Self {
        self.r#medium_quality_item_color = r#medium_quality_item_color.into();
        self
    }
    pub fn set_medium_quality_item_color(
        &mut self,
        r#medium_quality_item_color: impl Into<String>,
    ) -> &mut Self {
        self.r#medium_quality_item_color = r#medium_quality_item_color.into();
        self
    }
    pub fn with_high_quality_item_color(
        mut self,
        r#high_quality_item_color: impl Into<String>,
    ) -> Self {
        self.r#high_quality_item_color = r#high_quality_item_color.into();
        self
    }
    pub fn set_high_quality_item_color(
        &mut self,
        r#high_quality_item_color: impl Into<String>,
    ) -> &mut Self {
        self.r#high_quality_item_color = r#high_quality_item_color.into();
        self
    }
    pub fn with_perfect_quality_item_color(
        mut self,
        r#perfect_quality_item_color: impl Into<String>,
    ) -> Self {
        self.r#perfect_quality_item_color = r#perfect_quality_item_color.into();
        self
    }
    pub fn set_perfect_quality_item_color(
        &mut self,
        r#perfect_quality_item_color: impl Into<String>,
    ) -> &mut Self {
        self.r#perfect_quality_item_color = r#perfect_quality_item_color.into();
        self
    }
    pub fn with_available_tech_color(mut self, r#available_tech_color: impl Into<String>) -> Self {
        self.r#available_tech_color = r#available_tech_color.into();
        self
    }
    pub fn set_available_tech_color(
        &mut self,
        r#available_tech_color: impl Into<String>,
    ) -> &mut Self {
        self.r#available_tech_color = r#available_tech_color.into();
        self
    }
    pub fn with_unavailable_tech_color(
        mut self,
        r#unavailable_tech_color: impl Into<String>,
    ) -> Self {
        self.r#unavailable_tech_color = r#unavailable_tech_color.into();
        self
    }
    pub fn set_unavailable_tech_color(
        &mut self,
        r#unavailable_tech_color: impl Into<String>,
    ) -> &mut Self {
        self.r#unavailable_tech_color = r#unavailable_tech_color.into();
        self
    }
    pub fn with_obtained_tech_color(mut self, r#obtained_tech_color: impl Into<String>) -> Self {
        self.r#obtained_tech_color = r#obtained_tech_color.into();
        self
    }
    pub fn set_obtained_tech_color(
        &mut self,
        r#obtained_tech_color: impl Into<String>,
    ) -> &mut Self {
        self.r#obtained_tech_color = r#obtained_tech_color.into();
        self
    }
    pub fn with_hidden_tech_color(mut self, r#hidden_tech_color: impl Into<String>) -> Self {
        self.r#hidden_tech_color = r#hidden_tech_color.into();
        self
    }
    pub fn set_hidden_tech_color(&mut self, r#hidden_tech_color: impl Into<String>) -> &mut Self {
        self.r#hidden_tech_color = r#hidden_tech_color.into();
        self
    }
    pub fn with_credits_color(mut self, r#credits_color: impl Into<String>) -> Self {
        self.r#credits_color = r#credits_color.into();
        self
    }
    pub fn set_credits_color(&mut self, r#credits_color: impl Into<String>) -> &mut Self {
        self.r#credits_color = r#credits_color.into();
        self
    }
    pub fn with_stars_color(mut self, r#stars_color: impl Into<String>) -> Self {
        self.r#stars_color = r#stars_color.into();
        self
    }
    pub fn set_stars_color(&mut self, r#stars_color: impl Into<String>) -> &mut Self {
        self.r#stars_color = r#stars_color.into();
        self
    }
    pub fn with_money_color(mut self, r#money_color: impl Into<String>) -> Self {
        self.r#money_color = r#money_color.into();
        self
    }
    pub fn set_money_color(&mut self, r#money_color: impl Into<String>) -> &mut Self {
        self.r#money_color = r#money_color.into();
        self
    }
    pub fn with_fuel_color(mut self, r#fuel_color: impl Into<String>) -> Self {
        self.r#fuel_color = r#fuel_color.into();
        self
    }
    pub fn set_fuel_color(&mut self, r#fuel_color: impl Into<String>) -> &mut Self {
        self.r#fuel_color = r#fuel_color.into();
        self
    }
    pub fn with_tokens_color(mut self, r#tokens_color: impl Into<String>) -> Self {
        self.r#tokens_color = r#tokens_color.into();
        self
    }
    pub fn set_tokens_color(&mut self, r#tokens_color: impl Into<String>) -> &mut Self {
        self.r#tokens_color = r#tokens_color.into();
        self
    }
}
impl DatabaseItem for UiSettings {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "UiSettings"
    }
}
impl Default for UiSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Ai/BehaviorTree.xml
pub type BehaviorTreeId = DatabaseItemId<BehaviorTree>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BehaviorTree {
    pub r#id: BehaviorTreeId,
    pub r#root_node: BehaviorTreeNode,
}
impl BehaviorTree {
    pub fn new(r#id: BehaviorTreeId) -> Self {
        Self {
            r#id,
            r#root_node: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<BehaviorTreeId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<BehaviorTreeId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_root_node(mut self, r#root_node: impl Into<BehaviorTreeNode>) -> Self {
        self.r#root_node = r#root_node.into();
        self
    }
    pub fn set_root_node(&mut self, r#root_node: impl Into<BehaviorTreeNode>) -> &mut Self {
        self.r#root_node = r#root_node.into();
        self
    }
}
impl DatabaseItem for BehaviorTree {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "BehaviorTree"
    }
}
impl DatabaseItemWithId for BehaviorTree {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/AmmunitionObsolete.xml
pub type AmmunitionObsoleteId = DatabaseItemId<AmmunitionObsolete>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AmmunitionObsolete {
    pub r#id: AmmunitionObsoleteId,
    pub r#ammunition_class: AmmunitionClassObsolete,
    pub r#damage_type: DamageType,
    pub r#impulse: f32,
    pub r#recoil: f32,
    pub r#size: f32,
    pub r#initial_position: glam::f32::Vec2,
    pub r#area_of_effect: f32,
    pub r#damage: f32,
    pub r#range: f32,
    pub r#velocity: f32,
    pub r#life_time: f32,
    pub r#hit_points: i32,
    pub r#ignores_ship_velocity: bool,
    pub r#energy_cost: f32,
    pub r#coupled_ammunition_id: Option<AmmunitionObsoleteId>,
    pub r#color: String,
    pub r#fire_sound: String,
    pub r#hit_sound: String,
    pub r#hit_effect_prefab: String,
    pub r#bullet_prefab: String,
}
impl AmmunitionObsolete {
    pub fn new(r#id: AmmunitionObsoleteId) -> Self {
        Self {
            r#id,
            r#ammunition_class: Default::default(),
            r#damage_type: Default::default(),
            r#impulse: Default::default(),
            r#recoil: Default::default(),
            r#size: Default::default(),
            r#initial_position: Default::default(),
            r#area_of_effect: Default::default(),
            r#damage: Default::default(),
            r#range: Default::default(),
            r#velocity: Default::default(),
            r#life_time: Default::default(),
            r#hit_points: Default::default(),
            r#ignores_ship_velocity: Default::default(),
            r#energy_cost: Default::default(),
            r#coupled_ammunition_id: Default::default(),
            r#color: Default::default(),
            r#fire_sound: Default::default(),
            r#hit_sound: Default::default(),
            r#hit_effect_prefab: Default::default(),
            r#bullet_prefab: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<AmmunitionObsoleteId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<AmmunitionObsoleteId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_ammunition_class(
        mut self,
        r#ammunition_class: impl Into<AmmunitionClassObsolete>,
    ) -> Self {
        self.r#ammunition_class = r#ammunition_class.into();
        self
    }
    pub fn set_ammunition_class(
        &mut self,
        r#ammunition_class: impl Into<AmmunitionClassObsolete>,
    ) -> &mut Self {
        self.r#ammunition_class = r#ammunition_class.into();
        self
    }
    pub fn with_damage_type(mut self, r#damage_type: impl Into<DamageType>) -> Self {
        self.r#damage_type = r#damage_type.into();
        self
    }
    pub fn set_damage_type(&mut self, r#damage_type: impl Into<DamageType>) -> &mut Self {
        self.r#damage_type = r#damage_type.into();
        self
    }
    pub fn with_impulse(mut self, r#impulse: impl Into<f32>) -> Self {
        self.r#impulse = r#impulse.into();
        self
    }
    pub fn set_impulse(&mut self, r#impulse: impl Into<f32>) -> &mut Self {
        self.r#impulse = r#impulse.into();
        self
    }
    pub fn with_recoil(mut self, r#recoil: impl Into<f32>) -> Self {
        self.r#recoil = r#recoil.into();
        self
    }
    pub fn set_recoil(&mut self, r#recoil: impl Into<f32>) -> &mut Self {
        self.r#recoil = r#recoil.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_initial_position(mut self, r#initial_position: impl Into<glam::f32::Vec2>) -> Self {
        self.r#initial_position = r#initial_position.into();
        self
    }
    pub fn set_initial_position(
        &mut self,
        r#initial_position: impl Into<glam::f32::Vec2>,
    ) -> &mut Self {
        self.r#initial_position = r#initial_position.into();
        self
    }
    pub fn with_area_of_effect(mut self, r#area_of_effect: impl Into<f32>) -> Self {
        self.r#area_of_effect = r#area_of_effect.into();
        self
    }
    pub fn set_area_of_effect(&mut self, r#area_of_effect: impl Into<f32>) -> &mut Self {
        self.r#area_of_effect = r#area_of_effect.into();
        self
    }
    pub fn with_damage(mut self, r#damage: impl Into<f32>) -> Self {
        self.r#damage = r#damage.into();
        self
    }
    pub fn set_damage(&mut self, r#damage: impl Into<f32>) -> &mut Self {
        self.r#damage = r#damage.into();
        self
    }
    pub fn with_range(mut self, r#range: impl Into<f32>) -> Self {
        self.r#range = r#range.into();
        self
    }
    pub fn set_range(&mut self, r#range: impl Into<f32>) -> &mut Self {
        self.r#range = r#range.into();
        self
    }
    pub fn with_velocity(mut self, r#velocity: impl Into<f32>) -> Self {
        self.r#velocity = r#velocity.into();
        self
    }
    pub fn set_velocity(&mut self, r#velocity: impl Into<f32>) -> &mut Self {
        self.r#velocity = r#velocity.into();
        self
    }
    pub fn with_life_time(mut self, r#life_time: impl Into<f32>) -> Self {
        self.r#life_time = r#life_time.into();
        self
    }
    pub fn set_life_time(&mut self, r#life_time: impl Into<f32>) -> &mut Self {
        self.r#life_time = r#life_time.into();
        self
    }
    pub fn with_hit_points(mut self, r#hit_points: impl Into<i32>) -> Self {
        self.r#hit_points = r#hit_points.into();
        self
    }
    pub fn set_hit_points(&mut self, r#hit_points: impl Into<i32>) -> &mut Self {
        self.r#hit_points = r#hit_points.into();
        self
    }
    pub fn with_ignores_ship_velocity(mut self, r#ignores_ship_velocity: impl Into<bool>) -> Self {
        self.r#ignores_ship_velocity = r#ignores_ship_velocity.into();
        self
    }
    pub fn set_ignores_ship_velocity(
        &mut self,
        r#ignores_ship_velocity: impl Into<bool>,
    ) -> &mut Self {
        self.r#ignores_ship_velocity = r#ignores_ship_velocity.into();
        self
    }
    pub fn with_energy_cost(mut self, r#energy_cost: impl Into<f32>) -> Self {
        self.r#energy_cost = r#energy_cost.into();
        self
    }
    pub fn set_energy_cost(&mut self, r#energy_cost: impl Into<f32>) -> &mut Self {
        self.r#energy_cost = r#energy_cost.into();
        self
    }
    pub fn with_coupled_ammunition_id(
        mut self,
        r#coupled_ammunition_id: impl Into<Option<AmmunitionObsoleteId>>,
    ) -> Self {
        self.r#coupled_ammunition_id = r#coupled_ammunition_id.into();
        self
    }
    pub fn set_coupled_ammunition_id(
        &mut self,
        r#coupled_ammunition_id: impl Into<Option<AmmunitionObsoleteId>>,
    ) -> &mut Self {
        self.r#coupled_ammunition_id = r#coupled_ammunition_id.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_fire_sound(mut self, r#fire_sound: impl Into<String>) -> Self {
        self.r#fire_sound = r#fire_sound.into();
        self
    }
    pub fn set_fire_sound(&mut self, r#fire_sound: impl Into<String>) -> &mut Self {
        self.r#fire_sound = r#fire_sound.into();
        self
    }
    pub fn with_hit_sound(mut self, r#hit_sound: impl Into<String>) -> Self {
        self.r#hit_sound = r#hit_sound.into();
        self
    }
    pub fn set_hit_sound(&mut self, r#hit_sound: impl Into<String>) -> &mut Self {
        self.r#hit_sound = r#hit_sound.into();
        self
    }
    pub fn with_hit_effect_prefab(mut self, r#hit_effect_prefab: impl Into<String>) -> Self {
        self.r#hit_effect_prefab = r#hit_effect_prefab.into();
        self
    }
    pub fn set_hit_effect_prefab(&mut self, r#hit_effect_prefab: impl Into<String>) -> &mut Self {
        self.r#hit_effect_prefab = r#hit_effect_prefab.into();
        self
    }
    pub fn with_bullet_prefab(mut self, r#bullet_prefab: impl Into<String>) -> Self {
        self.r#bullet_prefab = r#bullet_prefab.into();
        self
    }
    pub fn set_bullet_prefab(&mut self, r#bullet_prefab: impl Into<String>) -> &mut Self {
        self.r#bullet_prefab = r#bullet_prefab.into();
        self
    }
}
impl DatabaseItem for AmmunitionObsolete {
    fn validate(&mut self) {
        if self.r#impulse < (0f32 as f32) {
            tracing::warn!(
                field = "r#impulse",
                value = self.r#impulse,
                min = 0f32,
                "Field got truncated"
            );
            self.r#impulse = 0f32 as f32;
        }
        if self.r#impulse > (10f32 as f32) {
            tracing::warn!(
                field = "r#impulse",
                value = self.r#impulse,
                max = 10f32,
                "Field got truncated"
            );
            self.r#impulse = 10f32 as f32;
        }
        if self.r#recoil < (0f32 as f32) {
            tracing::warn!(
                field = "r#recoil",
                value = self.r#recoil,
                min = 0f32,
                "Field got truncated"
            );
            self.r#recoil = 0f32 as f32;
        }
        if self.r#recoil > (10f32 as f32) {
            tracing::warn!(
                field = "r#recoil",
                value = self.r#recoil,
                max = 10f32,
                "Field got truncated"
            );
            self.r#recoil = 10f32 as f32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (1000f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#size = 1000f32 as f32;
        }
        if self.r#area_of_effect < (0f32 as f32) {
            tracing::warn!(
                field = "r#area_of_effect",
                value = self.r#area_of_effect,
                min = 0f32,
                "Field got truncated"
            );
            self.r#area_of_effect = 0f32 as f32;
        }
        if self.r#area_of_effect > (1000f32 as f32) {
            tracing::warn!(
                field = "r#area_of_effect",
                value = self.r#area_of_effect,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#area_of_effect = 1000f32 as f32;
        }
        if self.r#damage < (0f32 as f32) {
            tracing::warn!(
                field = "r#damage",
                value = self.r#damage,
                min = 0f32,
                "Field got truncated"
            );
            self.r#damage = 0f32 as f32;
        }
        if self.r#damage > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#damage",
                value = self.r#damage,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#damage = 1000000000f32 as f32;
        }
        if self.r#range < (0f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                min = 0f32,
                "Field got truncated"
            );
            self.r#range = 0f32 as f32;
        }
        if self.r#range > (1000f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#range = 1000f32 as f32;
        }
        if self.r#velocity < (0f32 as f32) {
            tracing::warn!(
                field = "r#velocity",
                value = self.r#velocity,
                min = 0f32,
                "Field got truncated"
            );
            self.r#velocity = 0f32 as f32;
        }
        if self.r#velocity > (1000f32 as f32) {
            tracing::warn!(
                field = "r#velocity",
                value = self.r#velocity,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#velocity = 1000f32 as f32;
        }
        if self.r#life_time < (0f32 as f32) {
            tracing::warn!(
                field = "r#life_time",
                value = self.r#life_time,
                min = 0f32,
                "Field got truncated"
            );
            self.r#life_time = 0f32 as f32;
        }
        if self.r#life_time > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#life_time",
                value = self.r#life_time,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#life_time = 1000000000f32 as f32;
        }
        if self.r#hit_points < (0f32 as i32) {
            tracing::warn!(
                field = "r#hit_points",
                value = self.r#hit_points,
                min = 0f32,
                "Field got truncated"
            );
            self.r#hit_points = 0f32 as i32;
        }
        if self.r#hit_points > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#hit_points",
                value = self.r#hit_points,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#hit_points = 1000000000f32 as i32;
        }
        if self.r#energy_cost < (0f32 as f32) {
            tracing::warn!(
                field = "r#energy_cost",
                value = self.r#energy_cost,
                min = 0f32,
                "Field got truncated"
            );
            self.r#energy_cost = 0f32 as f32;
        }
        if self.r#energy_cost > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_cost",
                value = self.r#energy_cost,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#energy_cost = 1000000000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "AmmunitionObsolete"
    }
}
impl DatabaseItemWithId for AmmunitionObsolete {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Component.xml
pub type ComponentId = DatabaseItemId<Component>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Component {
    pub r#id: ComponentId,
    pub r#name: String,
    pub r#description: String,
    pub r#display_category: ComponentCategory,
    pub r#availability: Availability,
    pub r#component_stats_id: ComponentStatsId,
    pub r#faction: Option<FactionId>,
    pub r#level: i32,
    pub r#icon: String,
    pub r#color: String,
    pub r#layout: String,
    pub r#cell_type: String,
    pub r#device_id: Option<DeviceId>,
    pub r#weapon_id: Option<WeaponId>,
    pub r#ammunition_id: Option<AmmunitionId>,
    pub r#weapon_slot_type: String,
    pub r#drone_bay_id: Option<DroneBayId>,
    pub r#drone_id: Option<ShipBuildId>,
    pub r#restrictions: ComponentRestrictions,
    pub r#possible_modifications: Vec<ComponentModId>,
}
impl Component {
    pub fn new(r#id: ComponentId, r#component_stats_id: ComponentStatsId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#description: Default::default(),
            r#display_category: Default::default(),
            r#availability: Default::default(),
            r#component_stats_id,
            r#faction: Default::default(),
            r#level: Default::default(),
            r#icon: Default::default(),
            r#color: Default::default(),
            r#layout: Default::default(),
            r#cell_type: Default::default(),
            r#device_id: Default::default(),
            r#weapon_id: Default::default(),
            r#ammunition_id: Default::default(),
            r#weapon_slot_type: Default::default(),
            r#drone_bay_id: Default::default(),
            r#drone_id: Default::default(),
            r#restrictions: Default::default(),
            r#possible_modifications: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<ComponentId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<ComponentId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_description(mut self, r#description: impl Into<String>) -> Self {
        self.r#description = r#description.into();
        self
    }
    pub fn set_description(&mut self, r#description: impl Into<String>) -> &mut Self {
        self.r#description = r#description.into();
        self
    }
    pub fn with_display_category(
        mut self,
        r#display_category: impl Into<ComponentCategory>,
    ) -> Self {
        self.r#display_category = r#display_category.into();
        self
    }
    pub fn set_display_category(
        &mut self,
        r#display_category: impl Into<ComponentCategory>,
    ) -> &mut Self {
        self.r#display_category = r#display_category.into();
        self
    }
    pub fn with_availability(mut self, r#availability: impl Into<Availability>) -> Self {
        self.r#availability = r#availability.into();
        self
    }
    pub fn set_availability(&mut self, r#availability: impl Into<Availability>) -> &mut Self {
        self.r#availability = r#availability.into();
        self
    }
    pub fn with_component_stats_id(
        mut self,
        r#component_stats_id: impl Into<ComponentStatsId>,
    ) -> Self {
        self.r#component_stats_id = r#component_stats_id.into();
        self
    }
    pub fn set_component_stats_id(
        &mut self,
        r#component_stats_id: impl Into<ComponentStatsId>,
    ) -> &mut Self {
        self.r#component_stats_id = r#component_stats_id.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn with_level(mut self, r#level: impl Into<i32>) -> Self {
        self.r#level = r#level.into();
        self
    }
    pub fn set_level(&mut self, r#level: impl Into<i32>) -> &mut Self {
        self.r#level = r#level.into();
        self
    }
    pub fn with_icon(mut self, r#icon: impl Into<String>) -> Self {
        self.r#icon = r#icon.into();
        self
    }
    pub fn set_icon(&mut self, r#icon: impl Into<String>) -> &mut Self {
        self.r#icon = r#icon.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_layout(mut self, r#layout: impl Into<String>) -> Self {
        self.r#layout = r#layout.into();
        self
    }
    pub fn set_layout(&mut self, r#layout: impl Into<String>) -> &mut Self {
        self.r#layout = r#layout.into();
        self
    }
    pub fn with_cell_type(mut self, r#cell_type: impl Into<String>) -> Self {
        self.r#cell_type = r#cell_type.into();
        self
    }
    pub fn set_cell_type(&mut self, r#cell_type: impl Into<String>) -> &mut Self {
        self.r#cell_type = r#cell_type.into();
        self
    }
    pub fn with_device_id(mut self, r#device_id: impl Into<Option<DeviceId>>) -> Self {
        self.r#device_id = r#device_id.into();
        self
    }
    pub fn set_device_id(&mut self, r#device_id: impl Into<Option<DeviceId>>) -> &mut Self {
        self.r#device_id = r#device_id.into();
        self
    }
    pub fn with_weapon_id(mut self, r#weapon_id: impl Into<Option<WeaponId>>) -> Self {
        self.r#weapon_id = r#weapon_id.into();
        self
    }
    pub fn set_weapon_id(&mut self, r#weapon_id: impl Into<Option<WeaponId>>) -> &mut Self {
        self.r#weapon_id = r#weapon_id.into();
        self
    }
    pub fn with_ammunition_id(mut self, r#ammunition_id: impl Into<Option<AmmunitionId>>) -> Self {
        self.r#ammunition_id = r#ammunition_id.into();
        self
    }
    pub fn set_ammunition_id(
        &mut self,
        r#ammunition_id: impl Into<Option<AmmunitionId>>,
    ) -> &mut Self {
        self.r#ammunition_id = r#ammunition_id.into();
        self
    }
    pub fn with_weapon_slot_type(mut self, r#weapon_slot_type: impl Into<String>) -> Self {
        self.r#weapon_slot_type = r#weapon_slot_type.into();
        self
    }
    pub fn set_weapon_slot_type(&mut self, r#weapon_slot_type: impl Into<String>) -> &mut Self {
        self.r#weapon_slot_type = r#weapon_slot_type.into();
        self
    }
    pub fn with_drone_bay_id(mut self, r#drone_bay_id: impl Into<Option<DroneBayId>>) -> Self {
        self.r#drone_bay_id = r#drone_bay_id.into();
        self
    }
    pub fn set_drone_bay_id(&mut self, r#drone_bay_id: impl Into<Option<DroneBayId>>) -> &mut Self {
        self.r#drone_bay_id = r#drone_bay_id.into();
        self
    }
    pub fn with_drone_id(mut self, r#drone_id: impl Into<Option<ShipBuildId>>) -> Self {
        self.r#drone_id = r#drone_id.into();
        self
    }
    pub fn set_drone_id(&mut self, r#drone_id: impl Into<Option<ShipBuildId>>) -> &mut Self {
        self.r#drone_id = r#drone_id.into();
        self
    }
    pub fn with_restrictions(mut self, r#restrictions: impl Into<ComponentRestrictions>) -> Self {
        self.r#restrictions = r#restrictions.into();
        self
    }
    pub fn set_restrictions(
        &mut self,
        r#restrictions: impl Into<ComponentRestrictions>,
    ) -> &mut Self {
        self.r#restrictions = r#restrictions.into();
        self
    }
    pub fn with_possible_modifications(
        mut self,
        r#possible_modifications: impl Into<Vec<ComponentModId>>,
    ) -> Self {
        self.r#possible_modifications = r#possible_modifications.into();
        self
    }
    pub fn set_possible_modifications(
        &mut self,
        r#possible_modifications: impl Into<Vec<ComponentModId>>,
    ) -> &mut Self {
        self.r#possible_modifications = r#possible_modifications.into();
        self
    }
}
impl DatabaseItem for Component {
    fn validate(&mut self) {
        if self.r#level < (0f32 as i32) {
            tracing::warn!(
                field = "r#level",
                value = self.r#level,
                min = 0f32,
                "Field got truncated"
            );
            self.r#level = 0f32 as i32;
        }
        let dw: String = Default::default();
        if self.r#cell_type != dw {
            tracing::error!(
                ield = "r#cell_type",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: String = Default::default();
        if self.r#weapon_slot_type != dw {
            tracing::error!(
                ield = "r#weapon_slot_type",
                "Obsolete field usage detected, generated code may not work",
            );
        }
    }
    fn type_name() -> &'static str {
        "Component"
    }
}
impl DatabaseItemWithId for Component {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ComponentMod.xml
pub type ComponentModId = DatabaseItemId<ComponentMod>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComponentMod {
    pub r#id: ComponentModId,
    pub r#description: String,
    pub r#modifications: Vec<StatModification>,
}
impl ComponentMod {
    pub fn new(r#id: ComponentModId) -> Self {
        Self {
            r#id,
            r#description: Default::default(),
            r#modifications: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<ComponentModId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<ComponentModId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_description(mut self, r#description: impl Into<String>) -> Self {
        self.r#description = r#description.into();
        self
    }
    pub fn set_description(&mut self, r#description: impl Into<String>) -> &mut Self {
        self.r#description = r#description.into();
        self
    }
    pub fn with_modifications(mut self, r#modifications: impl Into<Vec<StatModification>>) -> Self {
        self.r#modifications = r#modifications.into();
        self
    }
    pub fn set_modifications(
        &mut self,
        r#modifications: impl Into<Vec<StatModification>>,
    ) -> &mut Self {
        self.r#modifications = r#modifications.into();
        self
    }
}
impl DatabaseItem for ComponentMod {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "ComponentMod"
    }
}
impl DatabaseItemWithId for ComponentMod {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ComponentStats.xml
pub type ComponentStatsId = DatabaseItemId<ComponentStats>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComponentStats {
    pub r#id: ComponentStatsId,
    pub r#type: ComponentStatsType,
    pub r#armor_points: f32,
    pub r#armor_repair_rate: f32,
    pub r#armor_repair_cooldown_modifier: f32,
    pub r#energy_points: f32,
    pub r#energy_recharge_rate: f32,
    pub r#energy_recharge_cooldown_modifier: f32,
    pub r#shield_points: f32,
    pub r#shield_recharge_rate: f32,
    pub r#shield_recharge_cooldown_modifier: f32,
    pub r#weight: f32,
    pub r#ramming_damage: f32,
    pub r#energy_absorption: f32,
    pub r#kinetic_resistance: f32,
    pub r#energy_resistance: f32,
    pub r#thermal_resistance: f32,
    pub r#engine_power: f32,
    pub r#turn_rate: f32,
    pub r#autopilot: bool,
    pub r#drone_range_modifier: f32,
    pub r#drone_damage_modifier: f32,
    pub r#drone_defense_modifier: f32,
    pub r#drone_speed_modifier: f32,
    pub r#drones_built_per_second: f32,
    pub r#drone_build_time_modifier: f32,
    pub r#weapon_fire_rate_modifier: f32,
    pub r#weapon_damage_modifier: f32,
    pub r#weapon_range_modifier: f32,
    pub r#weapon_energy_cost_modifier: f32,
    pub r#alter_weapon_platform: i32,
    pub r#auto_aiming_arc: f32,
    pub r#turret_turn_speed: f32,
}
impl ComponentStats {
    pub fn new(r#id: ComponentStatsId) -> Self {
        Self {
            r#id,
            r#type: Default::default(),
            r#armor_points: Default::default(),
            r#armor_repair_rate: Default::default(),
            r#armor_repair_cooldown_modifier: Default::default(),
            r#energy_points: Default::default(),
            r#energy_recharge_rate: Default::default(),
            r#energy_recharge_cooldown_modifier: Default::default(),
            r#shield_points: Default::default(),
            r#shield_recharge_rate: Default::default(),
            r#shield_recharge_cooldown_modifier: Default::default(),
            r#weight: Default::default(),
            r#ramming_damage: Default::default(),
            r#energy_absorption: Default::default(),
            r#kinetic_resistance: Default::default(),
            r#energy_resistance: Default::default(),
            r#thermal_resistance: Default::default(),
            r#engine_power: Default::default(),
            r#turn_rate: Default::default(),
            r#autopilot: Default::default(),
            r#drone_range_modifier: Default::default(),
            r#drone_damage_modifier: Default::default(),
            r#drone_defense_modifier: Default::default(),
            r#drone_speed_modifier: Default::default(),
            r#drones_built_per_second: Default::default(),
            r#drone_build_time_modifier: Default::default(),
            r#weapon_fire_rate_modifier: Default::default(),
            r#weapon_damage_modifier: Default::default(),
            r#weapon_range_modifier: Default::default(),
            r#weapon_energy_cost_modifier: Default::default(),
            r#alter_weapon_platform: Default::default(),
            r#auto_aiming_arc: Default::default(),
            r#turret_turn_speed: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<ComponentStatsId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<ComponentStatsId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_type(mut self, r#type: impl Into<ComponentStatsType>) -> Self {
        self.r#type = r#type.into();
        self
    }
    pub fn set_type(&mut self, r#type: impl Into<ComponentStatsType>) -> &mut Self {
        self.r#type = r#type.into();
        self
    }
    pub fn with_armor_points(mut self, r#armor_points: impl Into<f32>) -> Self {
        self.r#armor_points = r#armor_points.into();
        self
    }
    pub fn set_armor_points(&mut self, r#armor_points: impl Into<f32>) -> &mut Self {
        self.r#armor_points = r#armor_points.into();
        self
    }
    pub fn with_armor_repair_rate(mut self, r#armor_repair_rate: impl Into<f32>) -> Self {
        self.r#armor_repair_rate = r#armor_repair_rate.into();
        self
    }
    pub fn set_armor_repair_rate(&mut self, r#armor_repair_rate: impl Into<f32>) -> &mut Self {
        self.r#armor_repair_rate = r#armor_repair_rate.into();
        self
    }
    pub fn with_armor_repair_cooldown_modifier(
        mut self,
        r#armor_repair_cooldown_modifier: impl Into<f32>,
    ) -> Self {
        self.r#armor_repair_cooldown_modifier = r#armor_repair_cooldown_modifier.into();
        self
    }
    pub fn set_armor_repair_cooldown_modifier(
        &mut self,
        r#armor_repair_cooldown_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#armor_repair_cooldown_modifier = r#armor_repair_cooldown_modifier.into();
        self
    }
    pub fn with_energy_points(mut self, r#energy_points: impl Into<f32>) -> Self {
        self.r#energy_points = r#energy_points.into();
        self
    }
    pub fn set_energy_points(&mut self, r#energy_points: impl Into<f32>) -> &mut Self {
        self.r#energy_points = r#energy_points.into();
        self
    }
    pub fn with_energy_recharge_rate(mut self, r#energy_recharge_rate: impl Into<f32>) -> Self {
        self.r#energy_recharge_rate = r#energy_recharge_rate.into();
        self
    }
    pub fn set_energy_recharge_rate(
        &mut self,
        r#energy_recharge_rate: impl Into<f32>,
    ) -> &mut Self {
        self.r#energy_recharge_rate = r#energy_recharge_rate.into();
        self
    }
    pub fn with_energy_recharge_cooldown_modifier(
        mut self,
        r#energy_recharge_cooldown_modifier: impl Into<f32>,
    ) -> Self {
        self.r#energy_recharge_cooldown_modifier = r#energy_recharge_cooldown_modifier.into();
        self
    }
    pub fn set_energy_recharge_cooldown_modifier(
        &mut self,
        r#energy_recharge_cooldown_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#energy_recharge_cooldown_modifier = r#energy_recharge_cooldown_modifier.into();
        self
    }
    pub fn with_shield_points(mut self, r#shield_points: impl Into<f32>) -> Self {
        self.r#shield_points = r#shield_points.into();
        self
    }
    pub fn set_shield_points(&mut self, r#shield_points: impl Into<f32>) -> &mut Self {
        self.r#shield_points = r#shield_points.into();
        self
    }
    pub fn with_shield_recharge_rate(mut self, r#shield_recharge_rate: impl Into<f32>) -> Self {
        self.r#shield_recharge_rate = r#shield_recharge_rate.into();
        self
    }
    pub fn set_shield_recharge_rate(
        &mut self,
        r#shield_recharge_rate: impl Into<f32>,
    ) -> &mut Self {
        self.r#shield_recharge_rate = r#shield_recharge_rate.into();
        self
    }
    pub fn with_shield_recharge_cooldown_modifier(
        mut self,
        r#shield_recharge_cooldown_modifier: impl Into<f32>,
    ) -> Self {
        self.r#shield_recharge_cooldown_modifier = r#shield_recharge_cooldown_modifier.into();
        self
    }
    pub fn set_shield_recharge_cooldown_modifier(
        &mut self,
        r#shield_recharge_cooldown_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#shield_recharge_cooldown_modifier = r#shield_recharge_cooldown_modifier.into();
        self
    }
    pub fn with_weight(mut self, r#weight: impl Into<f32>) -> Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn set_weight(&mut self, r#weight: impl Into<f32>) -> &mut Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn with_ramming_damage(mut self, r#ramming_damage: impl Into<f32>) -> Self {
        self.r#ramming_damage = r#ramming_damage.into();
        self
    }
    pub fn set_ramming_damage(&mut self, r#ramming_damage: impl Into<f32>) -> &mut Self {
        self.r#ramming_damage = r#ramming_damage.into();
        self
    }
    pub fn with_energy_absorption(mut self, r#energy_absorption: impl Into<f32>) -> Self {
        self.r#energy_absorption = r#energy_absorption.into();
        self
    }
    pub fn set_energy_absorption(&mut self, r#energy_absorption: impl Into<f32>) -> &mut Self {
        self.r#energy_absorption = r#energy_absorption.into();
        self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: impl Into<f32>) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn set_kinetic_resistance(&mut self, r#kinetic_resistance: impl Into<f32>) -> &mut Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn with_energy_resistance(mut self, r#energy_resistance: impl Into<f32>) -> Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn set_energy_resistance(&mut self, r#energy_resistance: impl Into<f32>) -> &mut Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn with_thermal_resistance(mut self, r#thermal_resistance: impl Into<f32>) -> Self {
        self.r#thermal_resistance = r#thermal_resistance.into();
        self
    }
    pub fn set_thermal_resistance(&mut self, r#thermal_resistance: impl Into<f32>) -> &mut Self {
        self.r#thermal_resistance = r#thermal_resistance.into();
        self
    }
    pub fn with_engine_power(mut self, r#engine_power: impl Into<f32>) -> Self {
        self.r#engine_power = r#engine_power.into();
        self
    }
    pub fn set_engine_power(&mut self, r#engine_power: impl Into<f32>) -> &mut Self {
        self.r#engine_power = r#engine_power.into();
        self
    }
    pub fn with_turn_rate(mut self, r#turn_rate: impl Into<f32>) -> Self {
        self.r#turn_rate = r#turn_rate.into();
        self
    }
    pub fn set_turn_rate(&mut self, r#turn_rate: impl Into<f32>) -> &mut Self {
        self.r#turn_rate = r#turn_rate.into();
        self
    }
    pub fn with_autopilot(mut self, r#autopilot: impl Into<bool>) -> Self {
        self.r#autopilot = r#autopilot.into();
        self
    }
    pub fn set_autopilot(&mut self, r#autopilot: impl Into<bool>) -> &mut Self {
        self.r#autopilot = r#autopilot.into();
        self
    }
    pub fn with_drone_range_modifier(mut self, r#drone_range_modifier: impl Into<f32>) -> Self {
        self.r#drone_range_modifier = r#drone_range_modifier.into();
        self
    }
    pub fn set_drone_range_modifier(
        &mut self,
        r#drone_range_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#drone_range_modifier = r#drone_range_modifier.into();
        self
    }
    pub fn with_drone_damage_modifier(mut self, r#drone_damage_modifier: impl Into<f32>) -> Self {
        self.r#drone_damage_modifier = r#drone_damage_modifier.into();
        self
    }
    pub fn set_drone_damage_modifier(
        &mut self,
        r#drone_damage_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#drone_damage_modifier = r#drone_damage_modifier.into();
        self
    }
    pub fn with_drone_defense_modifier(mut self, r#drone_defense_modifier: impl Into<f32>) -> Self {
        self.r#drone_defense_modifier = r#drone_defense_modifier.into();
        self
    }
    pub fn set_drone_defense_modifier(
        &mut self,
        r#drone_defense_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#drone_defense_modifier = r#drone_defense_modifier.into();
        self
    }
    pub fn with_drone_speed_modifier(mut self, r#drone_speed_modifier: impl Into<f32>) -> Self {
        self.r#drone_speed_modifier = r#drone_speed_modifier.into();
        self
    }
    pub fn set_drone_speed_modifier(
        &mut self,
        r#drone_speed_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#drone_speed_modifier = r#drone_speed_modifier.into();
        self
    }
    pub fn with_drones_built_per_second(
        mut self,
        r#drones_built_per_second: impl Into<f32>,
    ) -> Self {
        self.r#drones_built_per_second = r#drones_built_per_second.into();
        self
    }
    pub fn set_drones_built_per_second(
        &mut self,
        r#drones_built_per_second: impl Into<f32>,
    ) -> &mut Self {
        self.r#drones_built_per_second = r#drones_built_per_second.into();
        self
    }
    pub fn with_drone_build_time_modifier(
        mut self,
        r#drone_build_time_modifier: impl Into<f32>,
    ) -> Self {
        self.r#drone_build_time_modifier = r#drone_build_time_modifier.into();
        self
    }
    pub fn set_drone_build_time_modifier(
        &mut self,
        r#drone_build_time_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#drone_build_time_modifier = r#drone_build_time_modifier.into();
        self
    }
    pub fn with_weapon_fire_rate_modifier(
        mut self,
        r#weapon_fire_rate_modifier: impl Into<f32>,
    ) -> Self {
        self.r#weapon_fire_rate_modifier = r#weapon_fire_rate_modifier.into();
        self
    }
    pub fn set_weapon_fire_rate_modifier(
        &mut self,
        r#weapon_fire_rate_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#weapon_fire_rate_modifier = r#weapon_fire_rate_modifier.into();
        self
    }
    pub fn with_weapon_damage_modifier(mut self, r#weapon_damage_modifier: impl Into<f32>) -> Self {
        self.r#weapon_damage_modifier = r#weapon_damage_modifier.into();
        self
    }
    pub fn set_weapon_damage_modifier(
        &mut self,
        r#weapon_damage_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#weapon_damage_modifier = r#weapon_damage_modifier.into();
        self
    }
    pub fn with_weapon_range_modifier(mut self, r#weapon_range_modifier: impl Into<f32>) -> Self {
        self.r#weapon_range_modifier = r#weapon_range_modifier.into();
        self
    }
    pub fn set_weapon_range_modifier(
        &mut self,
        r#weapon_range_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#weapon_range_modifier = r#weapon_range_modifier.into();
        self
    }
    pub fn with_weapon_energy_cost_modifier(
        mut self,
        r#weapon_energy_cost_modifier: impl Into<f32>,
    ) -> Self {
        self.r#weapon_energy_cost_modifier = r#weapon_energy_cost_modifier.into();
        self
    }
    pub fn set_weapon_energy_cost_modifier(
        &mut self,
        r#weapon_energy_cost_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#weapon_energy_cost_modifier = r#weapon_energy_cost_modifier.into();
        self
    }
    pub fn with_alter_weapon_platform(mut self, r#alter_weapon_platform: impl Into<i32>) -> Self {
        self.r#alter_weapon_platform = r#alter_weapon_platform.into();
        self
    }
    pub fn set_alter_weapon_platform(
        &mut self,
        r#alter_weapon_platform: impl Into<i32>,
    ) -> &mut Self {
        self.r#alter_weapon_platform = r#alter_weapon_platform.into();
        self
    }
    pub fn with_auto_aiming_arc(mut self, r#auto_aiming_arc: impl Into<f32>) -> Self {
        self.r#auto_aiming_arc = r#auto_aiming_arc.into();
        self
    }
    pub fn set_auto_aiming_arc(&mut self, r#auto_aiming_arc: impl Into<f32>) -> &mut Self {
        self.r#auto_aiming_arc = r#auto_aiming_arc.into();
        self
    }
    pub fn with_turret_turn_speed(mut self, r#turret_turn_speed: impl Into<f32>) -> Self {
        self.r#turret_turn_speed = r#turret_turn_speed.into();
        self
    }
    pub fn set_turret_turn_speed(&mut self, r#turret_turn_speed: impl Into<f32>) -> &mut Self {
        self.r#turret_turn_speed = r#turret_turn_speed.into();
        self
    }
}
impl DatabaseItem for ComponentStats {
    fn validate(&mut self) {
        if self.r#armor_points < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#armor_points",
                value = self.r#armor_points,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#armor_points = -1000000f32 as f32;
        }
        if self.r#armor_points > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#armor_points",
                value = self.r#armor_points,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#armor_points = 1000000f32 as f32;
        }
        if self.r#armor_repair_rate < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#armor_repair_rate",
                value = self.r#armor_repair_rate,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#armor_repair_rate = -1000000f32 as f32;
        }
        if self.r#armor_repair_rate > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#armor_repair_rate",
                value = self.r#armor_repair_rate,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#armor_repair_rate = 1000000f32 as f32;
        }
        if self.r#armor_repair_cooldown_modifier < (-1f32 as f32) {
            tracing::warn!(
                field = "r#armor_repair_cooldown_modifier",
                value = self.r#armor_repair_cooldown_modifier,
                min = -1f32,
                "Field got truncated"
            );
            self.r#armor_repair_cooldown_modifier = -1f32 as f32;
        }
        if self.r#armor_repair_cooldown_modifier > (1f32 as f32) {
            tracing::warn!(
                field = "r#armor_repair_cooldown_modifier",
                value = self.r#armor_repair_cooldown_modifier,
                max = 1f32,
                "Field got truncated"
            );
            self.r#armor_repair_cooldown_modifier = 1f32 as f32;
        }
        if self.r#energy_points < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_points",
                value = self.r#energy_points,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#energy_points = -1000000f32 as f32;
        }
        if self.r#energy_points > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_points",
                value = self.r#energy_points,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#energy_points = 1000000f32 as f32;
        }
        if self.r#energy_recharge_rate < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_recharge_rate",
                value = self.r#energy_recharge_rate,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#energy_recharge_rate = -1000000f32 as f32;
        }
        if self.r#energy_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_recharge_rate",
                value = self.r#energy_recharge_rate,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#energy_recharge_rate = 1000000f32 as f32;
        }
        if self.r#energy_recharge_cooldown_modifier < (-5f32 as f32) {
            tracing::warn!(
                field = "r#energy_recharge_cooldown_modifier",
                value = self.r#energy_recharge_cooldown_modifier,
                min = -5f32,
                "Field got truncated"
            );
            self.r#energy_recharge_cooldown_modifier = -5f32 as f32;
        }
        if self.r#energy_recharge_cooldown_modifier > (5f32 as f32) {
            tracing::warn!(
                field = "r#energy_recharge_cooldown_modifier",
                value = self.r#energy_recharge_cooldown_modifier,
                max = 5f32,
                "Field got truncated"
            );
            self.r#energy_recharge_cooldown_modifier = 5f32 as f32;
        }
        if self.r#shield_points < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#shield_points",
                value = self.r#shield_points,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#shield_points = -1000000f32 as f32;
        }
        if self.r#shield_points > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#shield_points",
                value = self.r#shield_points,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#shield_points = 1000000f32 as f32;
        }
        if self.r#shield_recharge_rate < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#shield_recharge_rate",
                value = self.r#shield_recharge_rate,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#shield_recharge_rate = -1000000f32 as f32;
        }
        if self.r#shield_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#shield_recharge_rate",
                value = self.r#shield_recharge_rate,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#shield_recharge_rate = 1000000f32 as f32;
        }
        if self.r#shield_recharge_cooldown_modifier < (-5f32 as f32) {
            tracing::warn!(
                field = "r#shield_recharge_cooldown_modifier",
                value = self.r#shield_recharge_cooldown_modifier,
                min = -5f32,
                "Field got truncated"
            );
            self.r#shield_recharge_cooldown_modifier = -5f32 as f32;
        }
        if self.r#shield_recharge_cooldown_modifier > (5f32 as f32) {
            tracing::warn!(
                field = "r#shield_recharge_cooldown_modifier",
                value = self.r#shield_recharge_cooldown_modifier,
                max = 5f32,
                "Field got truncated"
            );
            self.r#shield_recharge_cooldown_modifier = 5f32 as f32;
        }
        if self.r#weight < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#weight = -1000000f32 as f32;
        }
        if self.r#weight > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#weight = 1000000f32 as f32;
        }
        if self.r#ramming_damage < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#ramming_damage",
                value = self.r#ramming_damage,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#ramming_damage = -1000000f32 as f32;
        }
        if self.r#ramming_damage > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#ramming_damage",
                value = self.r#ramming_damage,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#ramming_damage = 1000000f32 as f32;
        }
        if self.r#energy_absorption < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_absorption",
                value = self.r#energy_absorption,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#energy_absorption = -1000000f32 as f32;
        }
        if self.r#energy_absorption > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_absorption",
                value = self.r#energy_absorption,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#energy_absorption = 1000000f32 as f32;
        }
        if self.r#kinetic_resistance < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#kinetic_resistance",
                value = self.r#kinetic_resistance,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#kinetic_resistance = -1000000f32 as f32;
        }
        if self.r#kinetic_resistance > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#kinetic_resistance",
                value = self.r#kinetic_resistance,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#kinetic_resistance = 1000000f32 as f32;
        }
        if self.r#energy_resistance < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_resistance",
                value = self.r#energy_resistance,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#energy_resistance = -1000000f32 as f32;
        }
        if self.r#energy_resistance > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_resistance",
                value = self.r#energy_resistance,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#energy_resistance = 1000000f32 as f32;
        }
        if self.r#thermal_resistance < (-1000000f32 as f32) {
            tracing::warn!(
                field = "r#thermal_resistance",
                value = self.r#thermal_resistance,
                min = -1000000f32,
                "Field got truncated"
            );
            self.r#thermal_resistance = -1000000f32 as f32;
        }
        if self.r#thermal_resistance > (1000000f32 as f32) {
            tracing::warn!(
                field = "r#thermal_resistance",
                value = self.r#thermal_resistance,
                max = 1000000f32,
                "Field got truncated"
            );
            self.r#thermal_resistance = 1000000f32 as f32;
        }
        if self.r#engine_power < (0f32 as f32) {
            tracing::warn!(
                field = "r#engine_power",
                value = self.r#engine_power,
                min = 0f32,
                "Field got truncated"
            );
            self.r#engine_power = 0f32 as f32;
        }
        if self.r#engine_power > (2000f32 as f32) {
            tracing::warn!(
                field = "r#engine_power",
                value = self.r#engine_power,
                max = 2000f32,
                "Field got truncated"
            );
            self.r#engine_power = 2000f32 as f32;
        }
        if self.r#turn_rate < (0f32 as f32) {
            tracing::warn!(
                field = "r#turn_rate",
                value = self.r#turn_rate,
                min = 0f32,
                "Field got truncated"
            );
            self.r#turn_rate = 0f32 as f32;
        }
        if self.r#turn_rate > (2000f32 as f32) {
            tracing::warn!(
                field = "r#turn_rate",
                value = self.r#turn_rate,
                max = 2000f32,
                "Field got truncated"
            );
            self.r#turn_rate = 2000f32 as f32;
        }
        if self.r#drone_range_modifier < (-50f32 as f32) {
            tracing::warn!(
                field = "r#drone_range_modifier",
                value = self.r#drone_range_modifier,
                min = -50f32,
                "Field got truncated"
            );
            self.r#drone_range_modifier = -50f32 as f32;
        }
        if self.r#drone_range_modifier > (50f32 as f32) {
            tracing::warn!(
                field = "r#drone_range_modifier",
                value = self.r#drone_range_modifier,
                max = 50f32,
                "Field got truncated"
            );
            self.r#drone_range_modifier = 50f32 as f32;
        }
        if self.r#drone_damage_modifier < (-50f32 as f32) {
            tracing::warn!(
                field = "r#drone_damage_modifier",
                value = self.r#drone_damage_modifier,
                min = -50f32,
                "Field got truncated"
            );
            self.r#drone_damage_modifier = -50f32 as f32;
        }
        if self.r#drone_damage_modifier > (50f32 as f32) {
            tracing::warn!(
                field = "r#drone_damage_modifier",
                value = self.r#drone_damage_modifier,
                max = 50f32,
                "Field got truncated"
            );
            self.r#drone_damage_modifier = 50f32 as f32;
        }
        if self.r#drone_defense_modifier < (-50f32 as f32) {
            tracing::warn!(
                field = "r#drone_defense_modifier",
                value = self.r#drone_defense_modifier,
                min = -50f32,
                "Field got truncated"
            );
            self.r#drone_defense_modifier = -50f32 as f32;
        }
        if self.r#drone_defense_modifier > (50f32 as f32) {
            tracing::warn!(
                field = "r#drone_defense_modifier",
                value = self.r#drone_defense_modifier,
                max = 50f32,
                "Field got truncated"
            );
            self.r#drone_defense_modifier = 50f32 as f32;
        }
        if self.r#drone_speed_modifier < (-50f32 as f32) {
            tracing::warn!(
                field = "r#drone_speed_modifier",
                value = self.r#drone_speed_modifier,
                min = -50f32,
                "Field got truncated"
            );
            self.r#drone_speed_modifier = -50f32 as f32;
        }
        if self.r#drone_speed_modifier > (50f32 as f32) {
            tracing::warn!(
                field = "r#drone_speed_modifier",
                value = self.r#drone_speed_modifier,
                max = 50f32,
                "Field got truncated"
            );
            self.r#drone_speed_modifier = 50f32 as f32;
        }
        if self.r#drones_built_per_second < (0f32 as f32) {
            tracing::warn!(
                field = "r#drones_built_per_second",
                value = self.r#drones_built_per_second,
                min = 0f32,
                "Field got truncated"
            );
            self.r#drones_built_per_second = 0f32 as f32;
        }
        if self.r#drones_built_per_second > (100f32 as f32) {
            tracing::warn!(
                field = "r#drones_built_per_second",
                value = self.r#drones_built_per_second,
                max = 100f32,
                "Field got truncated"
            );
            self.r#drones_built_per_second = 100f32 as f32;
        }
        if self.r#drone_build_time_modifier < (0f32 as f32) {
            tracing::warn!(
                field = "r#drone_build_time_modifier",
                value = self.r#drone_build_time_modifier,
                min = 0f32,
                "Field got truncated"
            );
            self.r#drone_build_time_modifier = 0f32 as f32;
        }
        if self.r#drone_build_time_modifier > (100f32 as f32) {
            tracing::warn!(
                field = "r#drone_build_time_modifier",
                value = self.r#drone_build_time_modifier,
                max = 100f32,
                "Field got truncated"
            );
            self.r#drone_build_time_modifier = 100f32 as f32;
        }
        if self.r#weapon_fire_rate_modifier < (-100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_fire_rate_modifier",
                value = self.r#weapon_fire_rate_modifier,
                min = -100f32,
                "Field got truncated"
            );
            self.r#weapon_fire_rate_modifier = -100f32 as f32;
        }
        if self.r#weapon_fire_rate_modifier > (100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_fire_rate_modifier",
                value = self.r#weapon_fire_rate_modifier,
                max = 100f32,
                "Field got truncated"
            );
            self.r#weapon_fire_rate_modifier = 100f32 as f32;
        }
        if self.r#weapon_damage_modifier < (-100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_damage_modifier",
                value = self.r#weapon_damage_modifier,
                min = -100f32,
                "Field got truncated"
            );
            self.r#weapon_damage_modifier = -100f32 as f32;
        }
        if self.r#weapon_damage_modifier > (100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_damage_modifier",
                value = self.r#weapon_damage_modifier,
                max = 100f32,
                "Field got truncated"
            );
            self.r#weapon_damage_modifier = 100f32 as f32;
        }
        if self.r#weapon_range_modifier < (-100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_range_modifier",
                value = self.r#weapon_range_modifier,
                min = -100f32,
                "Field got truncated"
            );
            self.r#weapon_range_modifier = -100f32 as f32;
        }
        if self.r#weapon_range_modifier > (100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_range_modifier",
                value = self.r#weapon_range_modifier,
                max = 100f32,
                "Field got truncated"
            );
            self.r#weapon_range_modifier = 100f32 as f32;
        }
        if self.r#weapon_energy_cost_modifier < (-100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_energy_cost_modifier",
                value = self.r#weapon_energy_cost_modifier,
                min = -100f32,
                "Field got truncated"
            );
            self.r#weapon_energy_cost_modifier = -100f32 as f32;
        }
        if self.r#weapon_energy_cost_modifier > (100f32 as f32) {
            tracing::warn!(
                field = "r#weapon_energy_cost_modifier",
                value = self.r#weapon_energy_cost_modifier,
                max = 100f32,
                "Field got truncated"
            );
            self.r#weapon_energy_cost_modifier = 100f32 as f32;
        }
        let dw: i32 = Default::default();
        if self.r#alter_weapon_platform != dw {
            tracing::error!(
                ield = "r#alter_weapon_platform",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        if self.r#auto_aiming_arc < (0f32 as f32) {
            tracing::warn!(
                field = "r#auto_aiming_arc",
                value = self.r#auto_aiming_arc,
                min = 0f32,
                "Field got truncated"
            );
            self.r#auto_aiming_arc = 0f32 as f32;
        }
        if self.r#auto_aiming_arc > (360f32 as f32) {
            tracing::warn!(
                field = "r#auto_aiming_arc",
                value = self.r#auto_aiming_arc,
                max = 360f32,
                "Field got truncated"
            );
            self.r#auto_aiming_arc = 360f32 as f32;
        }
        if self.r#turret_turn_speed < (-1000f32 as f32) {
            tracing::warn!(
                field = "r#turret_turn_speed",
                value = self.r#turret_turn_speed,
                min = -1000f32,
                "Field got truncated"
            );
            self.r#turret_turn_speed = -1000f32 as f32;
        }
        if self.r#turret_turn_speed > (1000f32 as f32) {
            tracing::warn!(
                field = "r#turret_turn_speed",
                value = self.r#turret_turn_speed,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#turret_turn_speed = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "ComponentStats"
    }
}
impl DatabaseItemWithId for ComponentStats {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Device.xml
pub type DeviceId = DatabaseItemId<Device>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Device {
    pub r#id: DeviceId,
    pub r#device_class: DeviceClass,
    pub r#energy_consumption: f32,
    pub r#passive_energy_consumption: f32,
    pub r#power: f32,
    pub r#range: f32,
    pub r#size: f32,
    pub r#cooldown: f32,
    pub r#lifetime: f32,
    pub r#offset: glam::f32::Vec2,
    pub r#activation_type: ActivationType,
    pub r#color: String,
    pub r#sound: String,
    pub r#effect_prefab: String,
    pub r#object_prefab: String,
    pub r#prefab: Option<GameObjectPrefabId>,
    pub r#control_button_icon: String,
}
impl Device {
    pub fn new(r#id: DeviceId) -> Self {
        Self {
            r#id,
            r#device_class: Default::default(),
            r#energy_consumption: Default::default(),
            r#passive_energy_consumption: Default::default(),
            r#power: Default::default(),
            r#range: Default::default(),
            r#size: Default::default(),
            r#cooldown: Default::default(),
            r#lifetime: Default::default(),
            r#offset: Default::default(),
            r#activation_type: Default::default(),
            r#color: Default::default(),
            r#sound: Default::default(),
            r#effect_prefab: Default::default(),
            r#object_prefab: Default::default(),
            r#prefab: Default::default(),
            r#control_button_icon: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<DeviceId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<DeviceId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_device_class(mut self, r#device_class: impl Into<DeviceClass>) -> Self {
        self.r#device_class = r#device_class.into();
        self
    }
    pub fn set_device_class(&mut self, r#device_class: impl Into<DeviceClass>) -> &mut Self {
        self.r#device_class = r#device_class.into();
        self
    }
    pub fn with_energy_consumption(mut self, r#energy_consumption: impl Into<f32>) -> Self {
        self.r#energy_consumption = r#energy_consumption.into();
        self
    }
    pub fn set_energy_consumption(&mut self, r#energy_consumption: impl Into<f32>) -> &mut Self {
        self.r#energy_consumption = r#energy_consumption.into();
        self
    }
    pub fn with_passive_energy_consumption(
        mut self,
        r#passive_energy_consumption: impl Into<f32>,
    ) -> Self {
        self.r#passive_energy_consumption = r#passive_energy_consumption.into();
        self
    }
    pub fn set_passive_energy_consumption(
        &mut self,
        r#passive_energy_consumption: impl Into<f32>,
    ) -> &mut Self {
        self.r#passive_energy_consumption = r#passive_energy_consumption.into();
        self
    }
    pub fn with_power(mut self, r#power: impl Into<f32>) -> Self {
        self.r#power = r#power.into();
        self
    }
    pub fn set_power(&mut self, r#power: impl Into<f32>) -> &mut Self {
        self.r#power = r#power.into();
        self
    }
    pub fn with_range(mut self, r#range: impl Into<f32>) -> Self {
        self.r#range = r#range.into();
        self
    }
    pub fn set_range(&mut self, r#range: impl Into<f32>) -> &mut Self {
        self.r#range = r#range.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_cooldown(mut self, r#cooldown: impl Into<f32>) -> Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn set_cooldown(&mut self, r#cooldown: impl Into<f32>) -> &mut Self {
        self.r#cooldown = r#cooldown.into();
        self
    }
    pub fn with_lifetime(mut self, r#lifetime: impl Into<f32>) -> Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn set_lifetime(&mut self, r#lifetime: impl Into<f32>) -> &mut Self {
        self.r#lifetime = r#lifetime.into();
        self
    }
    pub fn with_offset(mut self, r#offset: impl Into<glam::f32::Vec2>) -> Self {
        self.r#offset = r#offset.into();
        self
    }
    pub fn set_offset(&mut self, r#offset: impl Into<glam::f32::Vec2>) -> &mut Self {
        self.r#offset = r#offset.into();
        self
    }
    pub fn with_activation_type(mut self, r#activation_type: impl Into<ActivationType>) -> Self {
        self.r#activation_type = r#activation_type.into();
        self
    }
    pub fn set_activation_type(
        &mut self,
        r#activation_type: impl Into<ActivationType>,
    ) -> &mut Self {
        self.r#activation_type = r#activation_type.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_sound(mut self, r#sound: impl Into<String>) -> Self {
        self.r#sound = r#sound.into();
        self
    }
    pub fn set_sound(&mut self, r#sound: impl Into<String>) -> &mut Self {
        self.r#sound = r#sound.into();
        self
    }
    pub fn with_effect_prefab(mut self, r#effect_prefab: impl Into<String>) -> Self {
        self.r#effect_prefab = r#effect_prefab.into();
        self
    }
    pub fn set_effect_prefab(&mut self, r#effect_prefab: impl Into<String>) -> &mut Self {
        self.r#effect_prefab = r#effect_prefab.into();
        self
    }
    pub fn with_object_prefab(mut self, r#object_prefab: impl Into<String>) -> Self {
        self.r#object_prefab = r#object_prefab.into();
        self
    }
    pub fn set_object_prefab(&mut self, r#object_prefab: impl Into<String>) -> &mut Self {
        self.r#object_prefab = r#object_prefab.into();
        self
    }
    pub fn with_prefab(mut self, r#prefab: impl Into<Option<GameObjectPrefabId>>) -> Self {
        self.r#prefab = r#prefab.into();
        self
    }
    pub fn set_prefab(&mut self, r#prefab: impl Into<Option<GameObjectPrefabId>>) -> &mut Self {
        self.r#prefab = r#prefab.into();
        self
    }
    pub fn with_control_button_icon(mut self, r#control_button_icon: impl Into<String>) -> Self {
        self.r#control_button_icon = r#control_button_icon.into();
        self
    }
    pub fn set_control_button_icon(
        &mut self,
        r#control_button_icon: impl Into<String>,
    ) -> &mut Self {
        self.r#control_button_icon = r#control_button_icon.into();
        self
    }
}
impl DatabaseItem for Device {
    fn validate(&mut self) {
        if self.r#energy_consumption < (0f32 as f32) {
            tracing::warn!(
                field = "r#energy_consumption",
                value = self.r#energy_consumption,
                min = 0f32,
                "Field got truncated"
            );
            self.r#energy_consumption = 0f32 as f32;
        }
        if self.r#energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_consumption",
                value = self.r#energy_consumption,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#energy_consumption = 1000000000f32 as f32;
        }
        if self.r#passive_energy_consumption < (0f32 as f32) {
            tracing::warn!(
                field = "r#passive_energy_consumption",
                value = self.r#passive_energy_consumption,
                min = 0f32,
                "Field got truncated"
            );
            self.r#passive_energy_consumption = 0f32 as f32;
        }
        if self.r#passive_energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#passive_energy_consumption",
                value = self.r#passive_energy_consumption,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#passive_energy_consumption = 1000000000f32 as f32;
        }
        if self.r#power < (0f32 as f32) {
            tracing::warn!(
                field = "r#power",
                value = self.r#power,
                min = 0f32,
                "Field got truncated"
            );
            self.r#power = 0f32 as f32;
        }
        if self.r#power > (1000f32 as f32) {
            tracing::warn!(
                field = "r#power",
                value = self.r#power,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#power = 1000f32 as f32;
        }
        if self.r#range < (0f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                min = 0f32,
                "Field got truncated"
            );
            self.r#range = 0f32 as f32;
        }
        if self.r#range > (1000f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#range = 1000f32 as f32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#size = 0f32 as f32;
        }
        if self.r#size > (1000f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#size = 1000f32 as f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                min = 0f32,
                "Field got truncated"
            );
            self.r#cooldown = 0f32 as f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                field = "r#cooldown",
                value = self.r#cooldown,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#cooldown = 1000f32 as f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                min = 0f32,
                "Field got truncated"
            );
            self.r#lifetime = 0f32 as f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                field = "r#lifetime",
                value = self.r#lifetime,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#lifetime = 1000f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "Device"
    }
}
impl DatabaseItemWithId for Device {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/DroneBay.xml
pub type DroneBayId = DatabaseItemId<DroneBay>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct DroneBay {
    pub r#id: DroneBayId,
    pub r#energy_consumption: f32,
    pub r#passive_energy_consumption: f32,
    pub r#range: f32,
    pub r#damage_multiplier: f32,
    pub r#defense_multiplier: f32,
    pub r#speed_multiplier: f32,
    pub r#build_extra_cycles: i32,
    pub r#improved_ai: bool,
    pub r#capacity: i32,
    pub r#activation_type: ActivationType,
    pub r#launch_sound: String,
    pub r#launch_effect_prefab: String,
    pub r#control_button_icon: String,
    pub r#defensive_drone_ai: Option<BehaviorTreeId>,
    pub r#offensive_drone_ai: Option<BehaviorTreeId>,
}
impl DroneBay {
    pub fn new(r#id: DroneBayId) -> Self {
        Self {
            r#id,
            r#energy_consumption: Default::default(),
            r#passive_energy_consumption: Default::default(),
            r#range: Default::default(),
            r#damage_multiplier: Default::default(),
            r#defense_multiplier: Default::default(),
            r#speed_multiplier: Default::default(),
            r#build_extra_cycles: Default::default(),
            r#improved_ai: Default::default(),
            r#capacity: Default::default(),
            r#activation_type: Default::default(),
            r#launch_sound: Default::default(),
            r#launch_effect_prefab: Default::default(),
            r#control_button_icon: Default::default(),
            r#defensive_drone_ai: Default::default(),
            r#offensive_drone_ai: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<DroneBayId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<DroneBayId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_energy_consumption(mut self, r#energy_consumption: impl Into<f32>) -> Self {
        self.r#energy_consumption = r#energy_consumption.into();
        self
    }
    pub fn set_energy_consumption(&mut self, r#energy_consumption: impl Into<f32>) -> &mut Self {
        self.r#energy_consumption = r#energy_consumption.into();
        self
    }
    pub fn with_passive_energy_consumption(
        mut self,
        r#passive_energy_consumption: impl Into<f32>,
    ) -> Self {
        self.r#passive_energy_consumption = r#passive_energy_consumption.into();
        self
    }
    pub fn set_passive_energy_consumption(
        &mut self,
        r#passive_energy_consumption: impl Into<f32>,
    ) -> &mut Self {
        self.r#passive_energy_consumption = r#passive_energy_consumption.into();
        self
    }
    pub fn with_range(mut self, r#range: impl Into<f32>) -> Self {
        self.r#range = r#range.into();
        self
    }
    pub fn set_range(&mut self, r#range: impl Into<f32>) -> &mut Self {
        self.r#range = r#range.into();
        self
    }
    pub fn with_damage_multiplier(mut self, r#damage_multiplier: impl Into<f32>) -> Self {
        self.r#damage_multiplier = r#damage_multiplier.into();
        self
    }
    pub fn set_damage_multiplier(&mut self, r#damage_multiplier: impl Into<f32>) -> &mut Self {
        self.r#damage_multiplier = r#damage_multiplier.into();
        self
    }
    pub fn with_defense_multiplier(mut self, r#defense_multiplier: impl Into<f32>) -> Self {
        self.r#defense_multiplier = r#defense_multiplier.into();
        self
    }
    pub fn set_defense_multiplier(&mut self, r#defense_multiplier: impl Into<f32>) -> &mut Self {
        self.r#defense_multiplier = r#defense_multiplier.into();
        self
    }
    pub fn with_speed_multiplier(mut self, r#speed_multiplier: impl Into<f32>) -> Self {
        self.r#speed_multiplier = r#speed_multiplier.into();
        self
    }
    pub fn set_speed_multiplier(&mut self, r#speed_multiplier: impl Into<f32>) -> &mut Self {
        self.r#speed_multiplier = r#speed_multiplier.into();
        self
    }
    pub fn with_build_extra_cycles(mut self, r#build_extra_cycles: impl Into<i32>) -> Self {
        self.r#build_extra_cycles = r#build_extra_cycles.into();
        self
    }
    pub fn set_build_extra_cycles(&mut self, r#build_extra_cycles: impl Into<i32>) -> &mut Self {
        self.r#build_extra_cycles = r#build_extra_cycles.into();
        self
    }
    pub fn with_improved_ai(mut self, r#improved_ai: impl Into<bool>) -> Self {
        self.r#improved_ai = r#improved_ai.into();
        self
    }
    pub fn set_improved_ai(&mut self, r#improved_ai: impl Into<bool>) -> &mut Self {
        self.r#improved_ai = r#improved_ai.into();
        self
    }
    pub fn with_capacity(mut self, r#capacity: impl Into<i32>) -> Self {
        self.r#capacity = r#capacity.into();
        self
    }
    pub fn set_capacity(&mut self, r#capacity: impl Into<i32>) -> &mut Self {
        self.r#capacity = r#capacity.into();
        self
    }
    pub fn with_activation_type(mut self, r#activation_type: impl Into<ActivationType>) -> Self {
        self.r#activation_type = r#activation_type.into();
        self
    }
    pub fn set_activation_type(
        &mut self,
        r#activation_type: impl Into<ActivationType>,
    ) -> &mut Self {
        self.r#activation_type = r#activation_type.into();
        self
    }
    pub fn with_launch_sound(mut self, r#launch_sound: impl Into<String>) -> Self {
        self.r#launch_sound = r#launch_sound.into();
        self
    }
    pub fn set_launch_sound(&mut self, r#launch_sound: impl Into<String>) -> &mut Self {
        self.r#launch_sound = r#launch_sound.into();
        self
    }
    pub fn with_launch_effect_prefab(mut self, r#launch_effect_prefab: impl Into<String>) -> Self {
        self.r#launch_effect_prefab = r#launch_effect_prefab.into();
        self
    }
    pub fn set_launch_effect_prefab(
        &mut self,
        r#launch_effect_prefab: impl Into<String>,
    ) -> &mut Self {
        self.r#launch_effect_prefab = r#launch_effect_prefab.into();
        self
    }
    pub fn with_control_button_icon(mut self, r#control_button_icon: impl Into<String>) -> Self {
        self.r#control_button_icon = r#control_button_icon.into();
        self
    }
    pub fn set_control_button_icon(
        &mut self,
        r#control_button_icon: impl Into<String>,
    ) -> &mut Self {
        self.r#control_button_icon = r#control_button_icon.into();
        self
    }
    pub fn with_defensive_drone_ai(
        mut self,
        r#defensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> Self {
        self.r#defensive_drone_ai = r#defensive_drone_ai.into();
        self
    }
    pub fn set_defensive_drone_ai(
        &mut self,
        r#defensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> &mut Self {
        self.r#defensive_drone_ai = r#defensive_drone_ai.into();
        self
    }
    pub fn with_offensive_drone_ai(
        mut self,
        r#offensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> Self {
        self.r#offensive_drone_ai = r#offensive_drone_ai.into();
        self
    }
    pub fn set_offensive_drone_ai(
        &mut self,
        r#offensive_drone_ai: impl Into<Option<BehaviorTreeId>>,
    ) -> &mut Self {
        self.r#offensive_drone_ai = r#offensive_drone_ai.into();
        self
    }
}
impl DatabaseItem for DroneBay {
    fn validate(&mut self) {
        if self.r#energy_consumption < (0f32 as f32) {
            tracing::warn!(
                field = "r#energy_consumption",
                value = self.r#energy_consumption,
                min = 0f32,
                "Field got truncated"
            );
            self.r#energy_consumption = 0f32 as f32;
        }
        if self.r#energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#energy_consumption",
                value = self.r#energy_consumption,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#energy_consumption = 1000000000f32 as f32;
        }
        if self.r#passive_energy_consumption < (0f32 as f32) {
            tracing::warn!(
                field = "r#passive_energy_consumption",
                value = self.r#passive_energy_consumption,
                min = 0f32,
                "Field got truncated"
            );
            self.r#passive_energy_consumption = 0f32 as f32;
        }
        if self.r#passive_energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                field = "r#passive_energy_consumption",
                value = self.r#passive_energy_consumption,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#passive_energy_consumption = 1000000000f32 as f32;
        }
        if self.r#range < (1f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                min = 1f32,
                "Field got truncated"
            );
            self.r#range = 1f32 as f32;
        }
        if self.r#range > (1000f32 as f32) {
            tracing::warn!(
                field = "r#range",
                value = self.r#range,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#range = 1000f32 as f32;
        }
        if self.r#damage_multiplier < (0.01f32 as f32) {
            tracing::warn!(
                field = "r#damage_multiplier",
                value = self.r#damage_multiplier,
                min = 0.01f32,
                "Field got truncated"
            );
            self.r#damage_multiplier = 0.01f32 as f32;
        }
        if self.r#damage_multiplier > (1000f32 as f32) {
            tracing::warn!(
                field = "r#damage_multiplier",
                value = self.r#damage_multiplier,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#damage_multiplier = 1000f32 as f32;
        }
        if self.r#defense_multiplier < (0.01f32 as f32) {
            tracing::warn!(
                field = "r#defense_multiplier",
                value = self.r#defense_multiplier,
                min = 0.01f32,
                "Field got truncated"
            );
            self.r#defense_multiplier = 0.01f32 as f32;
        }
        if self.r#defense_multiplier > (1000f32 as f32) {
            tracing::warn!(
                field = "r#defense_multiplier",
                value = self.r#defense_multiplier,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#defense_multiplier = 1000f32 as f32;
        }
        if self.r#speed_multiplier < (0.01f32 as f32) {
            tracing::warn!(
                field = "r#speed_multiplier",
                value = self.r#speed_multiplier,
                min = 0.01f32,
                "Field got truncated"
            );
            self.r#speed_multiplier = 0.01f32 as f32;
        }
        if self.r#speed_multiplier > (1000f32 as f32) {
            tracing::warn!(
                field = "r#speed_multiplier",
                value = self.r#speed_multiplier,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#speed_multiplier = 1000f32 as f32;
        }
        if self.r#build_extra_cycles < (0f32 as i32) {
            tracing::warn!(
                field = "r#build_extra_cycles",
                value = self.r#build_extra_cycles,
                min = 0f32,
                "Field got truncated"
            );
            self.r#build_extra_cycles = 0f32 as i32;
        }
        if self.r#build_extra_cycles > (100f32 as i32) {
            tracing::warn!(
                field = "r#build_extra_cycles",
                value = self.r#build_extra_cycles,
                max = 100f32,
                "Field got truncated"
            );
            self.r#build_extra_cycles = 100f32 as i32;
        }
        let dw: bool = Default::default();
        if self.r#improved_ai != dw {
            tracing::error!(
                ield = "r#improved_ai",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        if self.r#capacity < (1f32 as i32) {
            tracing::warn!(
                field = "r#capacity",
                value = self.r#capacity,
                min = 1f32,
                "Field got truncated"
            );
            self.r#capacity = 1f32 as i32;
        }
        if self.r#capacity > (1000f32 as i32) {
            tracing::warn!(
                field = "r#capacity",
                value = self.r#capacity,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#capacity = 1000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "DroneBay"
    }
}
impl DatabaseItemWithId for DroneBay {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Faction.xml
pub type FactionId = DatabaseItemId<Faction>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Faction {
    pub r#id: FactionId,
    pub r#name: String,
    pub r#color: String,
    pub r#no_territories: bool,
    pub r#home_star_distance: i32,
    pub r#home_star_distance_max: i32,
    pub r#no_wandering_ships: bool,
    pub r#wandering_ships_distance: i32,
    pub r#wandering_ships_distance_max: i32,
    pub r#hide_from_merchants: bool,
    pub r#hide_research_tree: bool,
    pub r#no_missions: bool,
    pub r#hidden: bool,
    pub r#hostile: bool,
}
impl Faction {
    pub fn new(r#id: FactionId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#color: Default::default(),
            r#no_territories: Default::default(),
            r#home_star_distance: Default::default(),
            r#home_star_distance_max: Default::default(),
            r#no_wandering_ships: Default::default(),
            r#wandering_ships_distance: Default::default(),
            r#wandering_ships_distance_max: Default::default(),
            r#hide_from_merchants: Default::default(),
            r#hide_research_tree: Default::default(),
            r#no_missions: Default::default(),
            r#hidden: Default::default(),
            r#hostile: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<FactionId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<FactionId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_no_territories(mut self, r#no_territories: impl Into<bool>) -> Self {
        self.r#no_territories = r#no_territories.into();
        self
    }
    pub fn set_no_territories(&mut self, r#no_territories: impl Into<bool>) -> &mut Self {
        self.r#no_territories = r#no_territories.into();
        self
    }
    pub fn with_home_star_distance(mut self, r#home_star_distance: impl Into<i32>) -> Self {
        self.r#home_star_distance = r#home_star_distance.into();
        self
    }
    pub fn set_home_star_distance(&mut self, r#home_star_distance: impl Into<i32>) -> &mut Self {
        self.r#home_star_distance = r#home_star_distance.into();
        self
    }
    pub fn with_home_star_distance_max(mut self, r#home_star_distance_max: impl Into<i32>) -> Self {
        self.r#home_star_distance_max = r#home_star_distance_max.into();
        self
    }
    pub fn set_home_star_distance_max(
        &mut self,
        r#home_star_distance_max: impl Into<i32>,
    ) -> &mut Self {
        self.r#home_star_distance_max = r#home_star_distance_max.into();
        self
    }
    pub fn with_no_wandering_ships(mut self, r#no_wandering_ships: impl Into<bool>) -> Self {
        self.r#no_wandering_ships = r#no_wandering_ships.into();
        self
    }
    pub fn set_no_wandering_ships(&mut self, r#no_wandering_ships: impl Into<bool>) -> &mut Self {
        self.r#no_wandering_ships = r#no_wandering_ships.into();
        self
    }
    pub fn with_wandering_ships_distance(
        mut self,
        r#wandering_ships_distance: impl Into<i32>,
    ) -> Self {
        self.r#wandering_ships_distance = r#wandering_ships_distance.into();
        self
    }
    pub fn set_wandering_ships_distance(
        &mut self,
        r#wandering_ships_distance: impl Into<i32>,
    ) -> &mut Self {
        self.r#wandering_ships_distance = r#wandering_ships_distance.into();
        self
    }
    pub fn with_wandering_ships_distance_max(
        mut self,
        r#wandering_ships_distance_max: impl Into<i32>,
    ) -> Self {
        self.r#wandering_ships_distance_max = r#wandering_ships_distance_max.into();
        self
    }
    pub fn set_wandering_ships_distance_max(
        &mut self,
        r#wandering_ships_distance_max: impl Into<i32>,
    ) -> &mut Self {
        self.r#wandering_ships_distance_max = r#wandering_ships_distance_max.into();
        self
    }
    pub fn with_hide_from_merchants(mut self, r#hide_from_merchants: impl Into<bool>) -> Self {
        self.r#hide_from_merchants = r#hide_from_merchants.into();
        self
    }
    pub fn set_hide_from_merchants(&mut self, r#hide_from_merchants: impl Into<bool>) -> &mut Self {
        self.r#hide_from_merchants = r#hide_from_merchants.into();
        self
    }
    pub fn with_hide_research_tree(mut self, r#hide_research_tree: impl Into<bool>) -> Self {
        self.r#hide_research_tree = r#hide_research_tree.into();
        self
    }
    pub fn set_hide_research_tree(&mut self, r#hide_research_tree: impl Into<bool>) -> &mut Self {
        self.r#hide_research_tree = r#hide_research_tree.into();
        self
    }
    pub fn with_no_missions(mut self, r#no_missions: impl Into<bool>) -> Self {
        self.r#no_missions = r#no_missions.into();
        self
    }
    pub fn set_no_missions(&mut self, r#no_missions: impl Into<bool>) -> &mut Self {
        self.r#no_missions = r#no_missions.into();
        self
    }
    pub fn with_hidden(mut self, r#hidden: impl Into<bool>) -> Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn set_hidden(&mut self, r#hidden: impl Into<bool>) -> &mut Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn with_hostile(mut self, r#hostile: impl Into<bool>) -> Self {
        self.r#hostile = r#hostile.into();
        self
    }
    pub fn set_hostile(&mut self, r#hostile: impl Into<bool>) -> &mut Self {
        self.r#hostile = r#hostile.into();
        self
    }
}
impl DatabaseItem for Faction {
    fn validate(&mut self) {
        if self.r#home_star_distance < (0f32 as i32) {
            tracing::warn!(
                field = "r#home_star_distance",
                value = self.r#home_star_distance,
                min = 0f32,
                "Field got truncated"
            );
            self.r#home_star_distance = 0f32 as i32;
        }
        if self.r#home_star_distance > (5000f32 as i32) {
            tracing::warn!(
                field = "r#home_star_distance",
                value = self.r#home_star_distance,
                max = 5000f32,
                "Field got truncated"
            );
            self.r#home_star_distance = 5000f32 as i32;
        }
        if self.r#home_star_distance_max < (0f32 as i32) {
            tracing::warn!(
                field = "r#home_star_distance_max",
                value = self.r#home_star_distance_max,
                min = 0f32,
                "Field got truncated"
            );
            self.r#home_star_distance_max = 0f32 as i32;
        }
        if self.r#home_star_distance_max > (5000f32 as i32) {
            tracing::warn!(
                field = "r#home_star_distance_max",
                value = self.r#home_star_distance_max,
                max = 5000f32,
                "Field got truncated"
            );
            self.r#home_star_distance_max = 5000f32 as i32;
        }
        if self.r#wandering_ships_distance < (0f32 as i32) {
            tracing::warn!(
                field = "r#wandering_ships_distance",
                value = self.r#wandering_ships_distance,
                min = 0f32,
                "Field got truncated"
            );
            self.r#wandering_ships_distance = 0f32 as i32;
        }
        if self.r#wandering_ships_distance > (5000f32 as i32) {
            tracing::warn!(
                field = "r#wandering_ships_distance",
                value = self.r#wandering_ships_distance,
                max = 5000f32,
                "Field got truncated"
            );
            self.r#wandering_ships_distance = 5000f32 as i32;
        }
        if self.r#wandering_ships_distance_max < (0f32 as i32) {
            tracing::warn!(
                field = "r#wandering_ships_distance_max",
                value = self.r#wandering_ships_distance_max,
                min = 0f32,
                "Field got truncated"
            );
            self.r#wandering_ships_distance_max = 0f32 as i32;
        }
        if self.r#wandering_ships_distance_max > (5000f32 as i32) {
            tracing::warn!(
                field = "r#wandering_ships_distance_max",
                value = self.r#wandering_ships_distance_max,
                max = 5000f32,
                "Field got truncated"
            );
            self.r#wandering_ships_distance_max = 5000f32 as i32;
        }
        let dw: bool = Default::default();
        if self.r#hidden != dw {
            tracing::error!(
                ield = "r#hidden",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: bool = Default::default();
        if self.r#hostile != dw {
            tracing::error!(
                ield = "r#hostile",
                "Obsolete field usage detected, generated code may not work",
            );
        }
    }
    fn type_name() -> &'static str {
        "Faction"
    }
}
impl DatabaseItemWithId for Faction {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/GameObjectPrefab.xml
pub type GameObjectPrefabId = DatabaseItemId<GameObjectPrefab>;
#[derive(Debug, Clone)]
pub enum GameObjectPrefab {
    Undefined(GameObjectPrefabUndefined),
    WormTailSegment(GameObjectPrefabWormTailSegment),
    CircularSpriteObject(GameObjectPrefabCircularSpriteObject),
    CircularOutlineObject(GameObjectPrefabCircularOutlineObject),
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameObjectPrefabUndefined {
    pub r#id: GameObjectPrefabId,
}
impl GameObjectPrefabUndefined {
    pub fn new(r#id: GameObjectPrefabId) -> Self {
        Self { r#id }
    }
    pub fn with_id(mut self, r#id: impl Into<GameObjectPrefabId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<GameObjectPrefabId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
}
impl DatabaseItem for GameObjectPrefabUndefined {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "GameObjectPrefabUndefined"
    }
}
impl From<GameObjectPrefabUndefined> for GameObjectPrefab {
    fn from(item: GameObjectPrefabUndefined) -> Self {
        Self::Undefined(item)
    }
}
impl GameObjectPrefabUndefined {
    pub fn wrap(self) -> GameObjectPrefab {
        self.into()
    }
}
impl GameObjectPrefab {
    pub fn undefined(r#id: GameObjectPrefabId) -> GameObjectPrefabUndefined {
        GameObjectPrefabUndefined::new(r#id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameObjectPrefabWormTailSegment {
    pub r#id: GameObjectPrefabId,
    pub r#image_1: String,
    pub r#image_2: String,
    pub r#image_scale: f32,
    pub r#image_offset: f32,
    pub r#length: f32,
    pub r#offset_1: f32,
    pub r#offset_2: f32,
    pub r#angle_1: f32,
    pub r#angle_2: f32,
}
impl GameObjectPrefabWormTailSegment {
    pub fn new(r#id: GameObjectPrefabId) -> Self {
        Self {
            r#id,
            r#image_1: Default::default(),
            r#image_2: Default::default(),
            r#image_scale: Default::default(),
            r#image_offset: Default::default(),
            r#length: Default::default(),
            r#offset_1: Default::default(),
            r#offset_2: Default::default(),
            r#angle_1: Default::default(),
            r#angle_2: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<GameObjectPrefabId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<GameObjectPrefabId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_image_1(mut self, r#image_1: impl Into<String>) -> Self {
        self.r#image_1 = r#image_1.into();
        self
    }
    pub fn set_image_1(&mut self, r#image_1: impl Into<String>) -> &mut Self {
        self.r#image_1 = r#image_1.into();
        self
    }
    pub fn with_image_2(mut self, r#image_2: impl Into<String>) -> Self {
        self.r#image_2 = r#image_2.into();
        self
    }
    pub fn set_image_2(&mut self, r#image_2: impl Into<String>) -> &mut Self {
        self.r#image_2 = r#image_2.into();
        self
    }
    pub fn with_image_scale(mut self, r#image_scale: impl Into<f32>) -> Self {
        self.r#image_scale = r#image_scale.into();
        self
    }
    pub fn set_image_scale(&mut self, r#image_scale: impl Into<f32>) -> &mut Self {
        self.r#image_scale = r#image_scale.into();
        self
    }
    pub fn with_image_offset(mut self, r#image_offset: impl Into<f32>) -> Self {
        self.r#image_offset = r#image_offset.into();
        self
    }
    pub fn set_image_offset(&mut self, r#image_offset: impl Into<f32>) -> &mut Self {
        self.r#image_offset = r#image_offset.into();
        self
    }
    pub fn with_length(mut self, r#length: impl Into<f32>) -> Self {
        self.r#length = r#length.into();
        self
    }
    pub fn set_length(&mut self, r#length: impl Into<f32>) -> &mut Self {
        self.r#length = r#length.into();
        self
    }
    pub fn with_offset_1(mut self, r#offset_1: impl Into<f32>) -> Self {
        self.r#offset_1 = r#offset_1.into();
        self
    }
    pub fn set_offset_1(&mut self, r#offset_1: impl Into<f32>) -> &mut Self {
        self.r#offset_1 = r#offset_1.into();
        self
    }
    pub fn with_offset_2(mut self, r#offset_2: impl Into<f32>) -> Self {
        self.r#offset_2 = r#offset_2.into();
        self
    }
    pub fn set_offset_2(&mut self, r#offset_2: impl Into<f32>) -> &mut Self {
        self.r#offset_2 = r#offset_2.into();
        self
    }
    pub fn with_angle_1(mut self, r#angle_1: impl Into<f32>) -> Self {
        self.r#angle_1 = r#angle_1.into();
        self
    }
    pub fn set_angle_1(&mut self, r#angle_1: impl Into<f32>) -> &mut Self {
        self.r#angle_1 = r#angle_1.into();
        self
    }
    pub fn with_angle_2(mut self, r#angle_2: impl Into<f32>) -> Self {
        self.r#angle_2 = r#angle_2.into();
        self
    }
    pub fn set_angle_2(&mut self, r#angle_2: impl Into<f32>) -> &mut Self {
        self.r#angle_2 = r#angle_2.into();
        self
    }
}
impl DatabaseItem for GameObjectPrefabWormTailSegment {
    fn validate(&mut self) {
        if self.r#image_scale < (0f32 as f32) {
            tracing::warn!(
                field = "r#image_scale",
                value = self.r#image_scale,
                min = 0f32,
                "Field got truncated"
            );
            self.r#image_scale = 0f32 as f32;
        }
        if self.r#image_scale > (10f32 as f32) {
            tracing::warn!(
                field = "r#image_scale",
                value = self.r#image_scale,
                max = 10f32,
                "Field got truncated"
            );
            self.r#image_scale = 10f32 as f32;
        }
        if self.r#image_offset < (-1f32 as f32) {
            tracing::warn!(
                field = "r#image_offset",
                value = self.r#image_offset,
                min = -1f32,
                "Field got truncated"
            );
            self.r#image_offset = -1f32 as f32;
        }
        if self.r#image_offset > (1f32 as f32) {
            tracing::warn!(
                field = "r#image_offset",
                value = self.r#image_offset,
                max = 1f32,
                "Field got truncated"
            );
            self.r#image_offset = 1f32 as f32;
        }
        if self.r#length < (0f32 as f32) {
            tracing::warn!(
                field = "r#length",
                value = self.r#length,
                min = 0f32,
                "Field got truncated"
            );
            self.r#length = 0f32 as f32;
        }
        if self.r#length > (1f32 as f32) {
            tracing::warn!(
                field = "r#length",
                value = self.r#length,
                max = 1f32,
                "Field got truncated"
            );
            self.r#length = 1f32 as f32;
        }
        if self.r#offset_1 < (0f32 as f32) {
            tracing::warn!(
                field = "r#offset_1",
                value = self.r#offset_1,
                min = 0f32,
                "Field got truncated"
            );
            self.r#offset_1 = 0f32 as f32;
        }
        if self.r#offset_1 > (1f32 as f32) {
            tracing::warn!(
                field = "r#offset_1",
                value = self.r#offset_1,
                max = 1f32,
                "Field got truncated"
            );
            self.r#offset_1 = 1f32 as f32;
        }
        if self.r#offset_2 < (-1f32 as f32) {
            tracing::warn!(
                field = "r#offset_2",
                value = self.r#offset_2,
                min = -1f32,
                "Field got truncated"
            );
            self.r#offset_2 = -1f32 as f32;
        }
        if self.r#offset_2 > (1f32 as f32) {
            tracing::warn!(
                field = "r#offset_2",
                value = self.r#offset_2,
                max = 1f32,
                "Field got truncated"
            );
            self.r#offset_2 = 1f32 as f32;
        }
        if self.r#angle_1 < (0f32 as f32) {
            tracing::warn!(
                field = "r#angle_1",
                value = self.r#angle_1,
                min = 0f32,
                "Field got truncated"
            );
            self.r#angle_1 = 0f32 as f32;
        }
        if self.r#angle_1 > (180f32 as f32) {
            tracing::warn!(
                field = "r#angle_1",
                value = self.r#angle_1,
                max = 180f32,
                "Field got truncated"
            );
            self.r#angle_1 = 180f32 as f32;
        }
        if self.r#angle_2 < (0f32 as f32) {
            tracing::warn!(
                field = "r#angle_2",
                value = self.r#angle_2,
                min = 0f32,
                "Field got truncated"
            );
            self.r#angle_2 = 0f32 as f32;
        }
        if self.r#angle_2 > (180f32 as f32) {
            tracing::warn!(
                field = "r#angle_2",
                value = self.r#angle_2,
                max = 180f32,
                "Field got truncated"
            );
            self.r#angle_2 = 180f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "GameObjectPrefabWormTailSegment"
    }
}
impl From<GameObjectPrefabWormTailSegment> for GameObjectPrefab {
    fn from(item: GameObjectPrefabWormTailSegment) -> Self {
        Self::WormTailSegment(item)
    }
}
impl GameObjectPrefabWormTailSegment {
    pub fn wrap(self) -> GameObjectPrefab {
        self.into()
    }
}
impl GameObjectPrefab {
    pub fn worm_tail_segment(r#id: GameObjectPrefabId) -> GameObjectPrefabWormTailSegment {
        GameObjectPrefabWormTailSegment::new(r#id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameObjectPrefabCircularSpriteObject {
    pub r#id: GameObjectPrefabId,
    pub r#image_1: String,
    pub r#image_scale: f32,
}
impl GameObjectPrefabCircularSpriteObject {
    pub fn new(r#id: GameObjectPrefabId) -> Self {
        Self {
            r#id,
            r#image_1: Default::default(),
            r#image_scale: 1f32,
        }
    }
    pub fn with_id(mut self, r#id: impl Into<GameObjectPrefabId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<GameObjectPrefabId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_image_1(mut self, r#image_1: impl Into<String>) -> Self {
        self.r#image_1 = r#image_1.into();
        self
    }
    pub fn set_image_1(&mut self, r#image_1: impl Into<String>) -> &mut Self {
        self.r#image_1 = r#image_1.into();
        self
    }
    pub fn with_image_scale(mut self, r#image_scale: impl Into<f32>) -> Self {
        self.r#image_scale = r#image_scale.into();
        self
    }
    pub fn set_image_scale(&mut self, r#image_scale: impl Into<f32>) -> &mut Self {
        self.r#image_scale = r#image_scale.into();
        self
    }
}
impl DatabaseItem for GameObjectPrefabCircularSpriteObject {
    fn validate(&mut self) {
        if self.r#image_scale < (0f32 as f32) {
            tracing::warn!(
                field = "r#image_scale",
                value = self.r#image_scale,
                min = 0f32,
                "Field got truncated"
            );
            self.r#image_scale = 0f32 as f32;
        }
        if self.r#image_scale > (10f32 as f32) {
            tracing::warn!(
                field = "r#image_scale",
                value = self.r#image_scale,
                max = 10f32,
                "Field got truncated"
            );
            self.r#image_scale = 10f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "GameObjectPrefabCircularSpriteObject"
    }
}
impl From<GameObjectPrefabCircularSpriteObject> for GameObjectPrefab {
    fn from(item: GameObjectPrefabCircularSpriteObject) -> Self {
        Self::CircularSpriteObject(item)
    }
}
impl GameObjectPrefabCircularSpriteObject {
    pub fn wrap(self) -> GameObjectPrefab {
        self.into()
    }
}
impl GameObjectPrefab {
    pub fn circular_sprite_object(
        r#id: GameObjectPrefabId,
    ) -> GameObjectPrefabCircularSpriteObject {
        GameObjectPrefabCircularSpriteObject::new(r#id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameObjectPrefabCircularOutlineObject {
    pub r#id: GameObjectPrefabId,
    pub r#image_1: String,
    pub r#image_scale: f32,
    pub r#thickness: f32,
    pub r#aspect_ratio: f32,
}
impl GameObjectPrefabCircularOutlineObject {
    pub fn new(r#id: GameObjectPrefabId) -> Self {
        Self {
            r#id,
            r#image_1: Default::default(),
            r#image_scale: 1f32,
            r#thickness: 0.1f32,
            r#aspect_ratio: 1f32,
        }
    }
    pub fn with_id(mut self, r#id: impl Into<GameObjectPrefabId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<GameObjectPrefabId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_image_1(mut self, r#image_1: impl Into<String>) -> Self {
        self.r#image_1 = r#image_1.into();
        self
    }
    pub fn set_image_1(&mut self, r#image_1: impl Into<String>) -> &mut Self {
        self.r#image_1 = r#image_1.into();
        self
    }
    pub fn with_image_scale(mut self, r#image_scale: impl Into<f32>) -> Self {
        self.r#image_scale = r#image_scale.into();
        self
    }
    pub fn set_image_scale(&mut self, r#image_scale: impl Into<f32>) -> &mut Self {
        self.r#image_scale = r#image_scale.into();
        self
    }
    pub fn with_thickness(mut self, r#thickness: impl Into<f32>) -> Self {
        self.r#thickness = r#thickness.into();
        self
    }
    pub fn set_thickness(&mut self, r#thickness: impl Into<f32>) -> &mut Self {
        self.r#thickness = r#thickness.into();
        self
    }
    pub fn with_aspect_ratio(mut self, r#aspect_ratio: impl Into<f32>) -> Self {
        self.r#aspect_ratio = r#aspect_ratio.into();
        self
    }
    pub fn set_aspect_ratio(&mut self, r#aspect_ratio: impl Into<f32>) -> &mut Self {
        self.r#aspect_ratio = r#aspect_ratio.into();
        self
    }
}
impl DatabaseItem for GameObjectPrefabCircularOutlineObject {
    fn validate(&mut self) {
        if self.r#image_scale < (0f32 as f32) {
            tracing::warn!(
                field = "r#image_scale",
                value = self.r#image_scale,
                min = 0f32,
                "Field got truncated"
            );
            self.r#image_scale = 0f32 as f32;
        }
        if self.r#image_scale > (10f32 as f32) {
            tracing::warn!(
                field = "r#image_scale",
                value = self.r#image_scale,
                max = 10f32,
                "Field got truncated"
            );
            self.r#image_scale = 10f32 as f32;
        }
        if self.r#thickness < (0f32 as f32) {
            tracing::warn!(
                field = "r#thickness",
                value = self.r#thickness,
                min = 0f32,
                "Field got truncated"
            );
            self.r#thickness = 0f32 as f32;
        }
        if self.r#thickness > (1f32 as f32) {
            tracing::warn!(
                field = "r#thickness",
                value = self.r#thickness,
                max = 1f32,
                "Field got truncated"
            );
            self.r#thickness = 1f32 as f32;
        }
        if self.r#aspect_ratio < (0f32 as f32) {
            tracing::warn!(
                field = "r#aspect_ratio",
                value = self.r#aspect_ratio,
                min = 0f32,
                "Field got truncated"
            );
            self.r#aspect_ratio = 0f32 as f32;
        }
        if self.r#aspect_ratio > (100f32 as f32) {
            tracing::warn!(
                field = "r#aspect_ratio",
                value = self.r#aspect_ratio,
                max = 100f32,
                "Field got truncated"
            );
            self.r#aspect_ratio = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "GameObjectPrefabCircularOutlineObject"
    }
}
impl From<GameObjectPrefabCircularOutlineObject> for GameObjectPrefab {
    fn from(item: GameObjectPrefabCircularOutlineObject) -> Self {
        Self::CircularOutlineObject(item)
    }
}
impl GameObjectPrefabCircularOutlineObject {
    pub fn wrap(self) -> GameObjectPrefab {
        self.into()
    }
}
impl GameObjectPrefab {
    pub fn circular_outline_object(
        r#id: GameObjectPrefabId,
    ) -> GameObjectPrefabCircularOutlineObject {
        GameObjectPrefabCircularOutlineObject::new(r#id)
    }
}
impl serde::Serialize for GameObjectPrefab {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "GameObjectPrefab")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: ObjectPrefabType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Undefined(x) => AdjTagged {
                t: ObjectPrefabType::Undefined,
                c: x,
            }
            .serialize(serializer),
            Self::WormTailSegment(x) => AdjTagged {
                t: ObjectPrefabType::WormTailSegment,
                c: x,
            }
            .serialize(serializer),
            Self::CircularSpriteObject(x) => AdjTagged {
                t: ObjectPrefabType::CircularSpriteObject,
                c: x,
            }
            .serialize(serializer),
            Self::CircularOutlineObject(x) => AdjTagged {
                t: ObjectPrefabType::CircularOutlineObject,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl GameObjectPrefab {
    pub fn r#id(&self) -> &GameObjectPrefabId {
        match self {
            Self::Undefined(x) => &x.r#id,
            Self::WormTailSegment(x) => &x.r#id,
            Self::CircularSpriteObject(x) => &x.r#id,
            Self::CircularOutlineObject(x) => &x.r#id,
        }
    }
    pub fn id_mut(&mut self) -> &mut GameObjectPrefabId {
        match self {
            Self::Undefined(x) => &mut x.r#id,
            Self::WormTailSegment(x) => &mut x.r#id,
            Self::CircularSpriteObject(x) => &mut x.r#id,
            Self::CircularOutlineObject(x) => &mut x.r#id,
        }
    }
}
impl DatabaseItem for GameObjectPrefab {
    fn validate(&mut self) {
        match self {
            Self::Undefined(x) => x.validate(),
            Self::WormTailSegment(x) => x.validate(),
            Self::CircularSpriteObject(x) => x.validate(),
            Self::CircularOutlineObject(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "GameObjectPrefab"
    }
}
impl GameObjectPrefab {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Undefined(_) => GameObjectPrefabUndefined::type_name(),
            Self::WormTailSegment(_) => GameObjectPrefabWormTailSegment::type_name(),
            Self::CircularSpriteObject(_) => GameObjectPrefabCircularSpriteObject::type_name(),
            Self::CircularOutlineObject(_) => GameObjectPrefabCircularOutlineObject::type_name(),
        }
    }
}
impl DatabaseItemWithId for GameObjectPrefab {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        *x.id()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Character.xml
pub type CharacterId = DatabaseItemId<Character>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Character {
    pub r#id: CharacterId,
    pub r#name: String,
    pub r#avatar_icon: String,
    pub r#faction: Option<FactionId>,
    pub r#inventory: Option<LootId>,
    pub r#fleet: Option<FleetId>,
    pub r#relations: i32,
    pub r#is_unique: bool,
}
impl Character {
    pub fn new(r#id: CharacterId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#avatar_icon: Default::default(),
            r#faction: Default::default(),
            r#inventory: Default::default(),
            r#fleet: Default::default(),
            r#relations: Default::default(),
            r#is_unique: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<CharacterId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<CharacterId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_avatar_icon(mut self, r#avatar_icon: impl Into<String>) -> Self {
        self.r#avatar_icon = r#avatar_icon.into();
        self
    }
    pub fn set_avatar_icon(&mut self, r#avatar_icon: impl Into<String>) -> &mut Self {
        self.r#avatar_icon = r#avatar_icon.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn with_inventory(mut self, r#inventory: impl Into<Option<LootId>>) -> Self {
        self.r#inventory = r#inventory.into();
        self
    }
    pub fn set_inventory(&mut self, r#inventory: impl Into<Option<LootId>>) -> &mut Self {
        self.r#inventory = r#inventory.into();
        self
    }
    pub fn with_fleet(mut self, r#fleet: impl Into<Option<FleetId>>) -> Self {
        self.r#fleet = r#fleet.into();
        self
    }
    pub fn set_fleet(&mut self, r#fleet: impl Into<Option<FleetId>>) -> &mut Self {
        self.r#fleet = r#fleet.into();
        self
    }
    pub fn with_relations(mut self, r#relations: impl Into<i32>) -> Self {
        self.r#relations = r#relations.into();
        self
    }
    pub fn set_relations(&mut self, r#relations: impl Into<i32>) -> &mut Self {
        self.r#relations = r#relations.into();
        self
    }
    pub fn with_is_unique(mut self, r#is_unique: impl Into<bool>) -> Self {
        self.r#is_unique = r#is_unique.into();
        self
    }
    pub fn set_is_unique(&mut self, r#is_unique: impl Into<bool>) -> &mut Self {
        self.r#is_unique = r#is_unique.into();
        self
    }
}
impl DatabaseItem for Character {
    fn validate(&mut self) {
        if self.r#relations < (-100f32 as i32) {
            tracing::warn!(
                field = "r#relations",
                value = self.r#relations,
                min = -100f32,
                "Field got truncated"
            );
            self.r#relations = -100f32 as i32;
        }
        if self.r#relations > (100f32 as i32) {
            tracing::warn!(
                field = "r#relations",
                value = self.r#relations,
                max = 100f32,
                "Field got truncated"
            );
            self.r#relations = 100f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "Character"
    }
}
impl DatabaseItemWithId for Character {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/CombatRules.xml
pub type CombatRulesId = DatabaseItemId<CombatRules>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CombatRules {
    pub r#id: CombatRulesId,
    pub r#initial_enemy_ships: String,
    pub r#max_enemy_ships: String,
    pub r#battle_map_size: i32,
    pub r#time_limit: String,
    pub r#time_out_mode: TimeOutMode,
    pub r#loot_condition: RewardCondition,
    pub r#exp_condition: RewardCondition,
    pub r#ship_selection: PlayerShipSelectionMode,
    pub r#disable_skill_bonuses: bool,
    pub r#disable_random_loot: bool,
    pub r#disable_asteroids: bool,
    pub r#disable_planet: bool,
    pub r#next_enemy_button: bool,
    ///For debug purposes
    pub r#kill_them_all_button: bool,
    pub r#custom_soundtrack: Vec<SoundTrack>,
}
impl CombatRules {
    pub fn new(r#id: CombatRulesId) -> Self {
        Self {
            r#id,
            r#initial_enemy_ships: "1".to_string(),
            r#max_enemy_ships: "12".to_string(),
            r#battle_map_size: 200i32,
            r#time_limit: "MAX(40, 100 - level)".to_string(),
            r#time_out_mode: Default::default(),
            r#loot_condition: Default::default(),
            r#exp_condition: Default::default(),
            r#ship_selection: Default::default(),
            r#disable_skill_bonuses: Default::default(),
            r#disable_random_loot: Default::default(),
            r#disable_asteroids: Default::default(),
            r#disable_planet: Default::default(),
            r#next_enemy_button: true,
            r#kill_them_all_button: Default::default(),
            r#custom_soundtrack: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<CombatRulesId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<CombatRulesId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_initial_enemy_ships(mut self, r#initial_enemy_ships: impl Into<String>) -> Self {
        self.r#initial_enemy_ships = r#initial_enemy_ships.into();
        self
    }
    pub fn set_initial_enemy_ships(
        &mut self,
        r#initial_enemy_ships: impl Into<String>,
    ) -> &mut Self {
        self.r#initial_enemy_ships = r#initial_enemy_ships.into();
        self
    }
    pub fn with_max_enemy_ships(mut self, r#max_enemy_ships: impl Into<String>) -> Self {
        self.r#max_enemy_ships = r#max_enemy_ships.into();
        self
    }
    pub fn set_max_enemy_ships(&mut self, r#max_enemy_ships: impl Into<String>) -> &mut Self {
        self.r#max_enemy_ships = r#max_enemy_ships.into();
        self
    }
    pub fn with_battle_map_size(mut self, r#battle_map_size: impl Into<i32>) -> Self {
        self.r#battle_map_size = r#battle_map_size.into();
        self
    }
    pub fn set_battle_map_size(&mut self, r#battle_map_size: impl Into<i32>) -> &mut Self {
        self.r#battle_map_size = r#battle_map_size.into();
        self
    }
    pub fn with_time_limit(mut self, r#time_limit: impl Into<String>) -> Self {
        self.r#time_limit = r#time_limit.into();
        self
    }
    pub fn set_time_limit(&mut self, r#time_limit: impl Into<String>) -> &mut Self {
        self.r#time_limit = r#time_limit.into();
        self
    }
    pub fn with_time_out_mode(mut self, r#time_out_mode: impl Into<TimeOutMode>) -> Self {
        self.r#time_out_mode = r#time_out_mode.into();
        self
    }
    pub fn set_time_out_mode(&mut self, r#time_out_mode: impl Into<TimeOutMode>) -> &mut Self {
        self.r#time_out_mode = r#time_out_mode.into();
        self
    }
    pub fn with_loot_condition(mut self, r#loot_condition: impl Into<RewardCondition>) -> Self {
        self.r#loot_condition = r#loot_condition.into();
        self
    }
    pub fn set_loot_condition(
        &mut self,
        r#loot_condition: impl Into<RewardCondition>,
    ) -> &mut Self {
        self.r#loot_condition = r#loot_condition.into();
        self
    }
    pub fn with_exp_condition(mut self, r#exp_condition: impl Into<RewardCondition>) -> Self {
        self.r#exp_condition = r#exp_condition.into();
        self
    }
    pub fn set_exp_condition(&mut self, r#exp_condition: impl Into<RewardCondition>) -> &mut Self {
        self.r#exp_condition = r#exp_condition.into();
        self
    }
    pub fn with_ship_selection(
        mut self,
        r#ship_selection: impl Into<PlayerShipSelectionMode>,
    ) -> Self {
        self.r#ship_selection = r#ship_selection.into();
        self
    }
    pub fn set_ship_selection(
        &mut self,
        r#ship_selection: impl Into<PlayerShipSelectionMode>,
    ) -> &mut Self {
        self.r#ship_selection = r#ship_selection.into();
        self
    }
    pub fn with_disable_skill_bonuses(mut self, r#disable_skill_bonuses: impl Into<bool>) -> Self {
        self.r#disable_skill_bonuses = r#disable_skill_bonuses.into();
        self
    }
    pub fn set_disable_skill_bonuses(
        &mut self,
        r#disable_skill_bonuses: impl Into<bool>,
    ) -> &mut Self {
        self.r#disable_skill_bonuses = r#disable_skill_bonuses.into();
        self
    }
    pub fn with_disable_random_loot(mut self, r#disable_random_loot: impl Into<bool>) -> Self {
        self.r#disable_random_loot = r#disable_random_loot.into();
        self
    }
    pub fn set_disable_random_loot(&mut self, r#disable_random_loot: impl Into<bool>) -> &mut Self {
        self.r#disable_random_loot = r#disable_random_loot.into();
        self
    }
    pub fn with_disable_asteroids(mut self, r#disable_asteroids: impl Into<bool>) -> Self {
        self.r#disable_asteroids = r#disable_asteroids.into();
        self
    }
    pub fn set_disable_asteroids(&mut self, r#disable_asteroids: impl Into<bool>) -> &mut Self {
        self.r#disable_asteroids = r#disable_asteroids.into();
        self
    }
    pub fn with_disable_planet(mut self, r#disable_planet: impl Into<bool>) -> Self {
        self.r#disable_planet = r#disable_planet.into();
        self
    }
    pub fn set_disable_planet(&mut self, r#disable_planet: impl Into<bool>) -> &mut Self {
        self.r#disable_planet = r#disable_planet.into();
        self
    }
    pub fn with_next_enemy_button(mut self, r#next_enemy_button: impl Into<bool>) -> Self {
        self.r#next_enemy_button = r#next_enemy_button.into();
        self
    }
    pub fn set_next_enemy_button(&mut self, r#next_enemy_button: impl Into<bool>) -> &mut Self {
        self.r#next_enemy_button = r#next_enemy_button.into();
        self
    }
    pub fn with_kill_them_all_button(mut self, r#kill_them_all_button: impl Into<bool>) -> Self {
        self.r#kill_them_all_button = r#kill_them_all_button.into();
        self
    }
    pub fn set_kill_them_all_button(
        &mut self,
        r#kill_them_all_button: impl Into<bool>,
    ) -> &mut Self {
        self.r#kill_them_all_button = r#kill_them_all_button.into();
        self
    }
    pub fn with_custom_soundtrack(
        mut self,
        r#custom_soundtrack: impl Into<Vec<SoundTrack>>,
    ) -> Self {
        self.r#custom_soundtrack = r#custom_soundtrack.into();
        self
    }
    pub fn set_custom_soundtrack(
        &mut self,
        r#custom_soundtrack: impl Into<Vec<SoundTrack>>,
    ) -> &mut Self {
        self.r#custom_soundtrack = r#custom_soundtrack.into();
        self
    }
}
impl DatabaseItem for CombatRules {
    fn validate(&mut self) {
        if self.r#battle_map_size < (50f32 as i32) {
            tracing::warn!(
                field = "r#battle_map_size",
                value = self.r#battle_map_size,
                min = 50f32,
                "Field got truncated"
            );
            self.r#battle_map_size = 50f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "CombatRules"
    }
}
impl DatabaseItemWithId for CombatRules {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Fleet.xml
pub type FleetId = DatabaseItemId<Fleet>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fleet {
    pub r#id: FleetId,
    pub r#factions: FactionFilter,
    pub r#level_bonus: i32,
    pub r#no_random_ships: bool,
    pub r#combat_time_limit: i32,
    pub r#loot_condition: RewardCondition,
    pub r#exp_condition: RewardCondition,
    pub r#specific_ships: Vec<ShipBuildId>,
    pub r#no_ship_changing: bool,
    pub r#player_has_one_ship: bool,
    pub r#combat_rules: Option<CombatRulesId>,
}
impl Fleet {
    pub fn new(r#id: FleetId) -> Self {
        Self {
            r#id,
            r#factions: Default::default(),
            r#level_bonus: Default::default(),
            r#no_random_ships: Default::default(),
            r#combat_time_limit: Default::default(),
            r#loot_condition: Default::default(),
            r#exp_condition: Default::default(),
            r#specific_ships: Default::default(),
            r#no_ship_changing: Default::default(),
            r#player_has_one_ship: Default::default(),
            r#combat_rules: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<FleetId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<FleetId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_factions(mut self, r#factions: impl Into<FactionFilter>) -> Self {
        self.r#factions = r#factions.into();
        self
    }
    pub fn set_factions(&mut self, r#factions: impl Into<FactionFilter>) -> &mut Self {
        self.r#factions = r#factions.into();
        self
    }
    pub fn with_level_bonus(mut self, r#level_bonus: impl Into<i32>) -> Self {
        self.r#level_bonus = r#level_bonus.into();
        self
    }
    pub fn set_level_bonus(&mut self, r#level_bonus: impl Into<i32>) -> &mut Self {
        self.r#level_bonus = r#level_bonus.into();
        self
    }
    pub fn with_no_random_ships(mut self, r#no_random_ships: impl Into<bool>) -> Self {
        self.r#no_random_ships = r#no_random_ships.into();
        self
    }
    pub fn set_no_random_ships(&mut self, r#no_random_ships: impl Into<bool>) -> &mut Self {
        self.r#no_random_ships = r#no_random_ships.into();
        self
    }
    pub fn with_combat_time_limit(mut self, r#combat_time_limit: impl Into<i32>) -> Self {
        self.r#combat_time_limit = r#combat_time_limit.into();
        self
    }
    pub fn set_combat_time_limit(&mut self, r#combat_time_limit: impl Into<i32>) -> &mut Self {
        self.r#combat_time_limit = r#combat_time_limit.into();
        self
    }
    pub fn with_loot_condition(mut self, r#loot_condition: impl Into<RewardCondition>) -> Self {
        self.r#loot_condition = r#loot_condition.into();
        self
    }
    pub fn set_loot_condition(
        &mut self,
        r#loot_condition: impl Into<RewardCondition>,
    ) -> &mut Self {
        self.r#loot_condition = r#loot_condition.into();
        self
    }
    pub fn with_exp_condition(mut self, r#exp_condition: impl Into<RewardCondition>) -> Self {
        self.r#exp_condition = r#exp_condition.into();
        self
    }
    pub fn set_exp_condition(&mut self, r#exp_condition: impl Into<RewardCondition>) -> &mut Self {
        self.r#exp_condition = r#exp_condition.into();
        self
    }
    pub fn with_specific_ships(mut self, r#specific_ships: impl Into<Vec<ShipBuildId>>) -> Self {
        self.r#specific_ships = r#specific_ships.into();
        self
    }
    pub fn set_specific_ships(
        &mut self,
        r#specific_ships: impl Into<Vec<ShipBuildId>>,
    ) -> &mut Self {
        self.r#specific_ships = r#specific_ships.into();
        self
    }
    pub fn with_no_ship_changing(mut self, r#no_ship_changing: impl Into<bool>) -> Self {
        self.r#no_ship_changing = r#no_ship_changing.into();
        self
    }
    pub fn set_no_ship_changing(&mut self, r#no_ship_changing: impl Into<bool>) -> &mut Self {
        self.r#no_ship_changing = r#no_ship_changing.into();
        self
    }
    pub fn with_player_has_one_ship(mut self, r#player_has_one_ship: impl Into<bool>) -> Self {
        self.r#player_has_one_ship = r#player_has_one_ship.into();
        self
    }
    pub fn set_player_has_one_ship(&mut self, r#player_has_one_ship: impl Into<bool>) -> &mut Self {
        self.r#player_has_one_ship = r#player_has_one_ship.into();
        self
    }
    pub fn with_combat_rules(mut self, r#combat_rules: impl Into<Option<CombatRulesId>>) -> Self {
        self.r#combat_rules = r#combat_rules.into();
        self
    }
    pub fn set_combat_rules(
        &mut self,
        r#combat_rules: impl Into<Option<CombatRulesId>>,
    ) -> &mut Self {
        self.r#combat_rules = r#combat_rules.into();
        self
    }
}
impl DatabaseItem for Fleet {
    fn validate(&mut self) {
        if self.r#level_bonus < (-10000f32 as i32) {
            tracing::warn!(
                field = "r#level_bonus",
                value = self.r#level_bonus,
                min = -10000f32,
                "Field got truncated"
            );
            self.r#level_bonus = -10000f32 as i32;
        }
        if self.r#level_bonus > (10000f32 as i32) {
            tracing::warn!(
                field = "r#level_bonus",
                value = self.r#level_bonus,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#level_bonus = 10000f32 as i32;
        }
        if self.r#combat_time_limit < (0f32 as i32) {
            tracing::warn!(
                field = "r#combat_time_limit",
                value = self.r#combat_time_limit,
                min = 0f32,
                "Field got truncated"
            );
            self.r#combat_time_limit = 0f32 as i32;
        }
        if self.r#combat_time_limit > (999f32 as i32) {
            tracing::warn!(
                field = "r#combat_time_limit",
                value = self.r#combat_time_limit,
                max = 999f32,
                "Field got truncated"
            );
            self.r#combat_time_limit = 999f32 as i32;
        }
        let dw: i32 = Default::default();
        if self.r#combat_time_limit != dw {
            tracing::error!(
                ield = "r#combat_time_limit",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: RewardCondition = Default::default();
        if self.r#loot_condition != dw {
            tracing::error!(
                ield = "r#loot_condition",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: RewardCondition = Default::default();
        if self.r#exp_condition != dw {
            tracing::error!(
                ield = "r#exp_condition",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: bool = Default::default();
        if self.r#no_ship_changing != dw {
            tracing::error!(
                ield = "r#no_ship_changing",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: bool = Default::default();
        if self.r#player_has_one_ship != dw {
            tracing::error!(
                ield = "r#player_has_one_ship",
                "Obsolete field usage detected, generated code may not work",
            );
        }
    }
    fn type_name() -> &'static str {
        "Fleet"
    }
}
impl DatabaseItemWithId for Fleet {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Loot.xml
pub type LootId = DatabaseItemId<Loot>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Loot {
    pub r#id: LootId,
    pub r#loot: LootContent,
}
impl Loot {
    pub fn new(r#id: LootId) -> Self {
        Self {
            r#id,
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<LootId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<LootId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_loot(mut self, r#loot: impl Into<LootContent>) -> Self {
        self.r#loot = r#loot.into();
        self
    }
    pub fn set_loot(&mut self, r#loot: impl Into<LootContent>) -> &mut Self {
        self.r#loot = r#loot.into();
        self
    }
}
impl DatabaseItem for Loot {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "Loot"
    }
}
impl DatabaseItemWithId for Loot {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Quest.xml
pub type QuestId = DatabaseItemId<Quest>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Quest {
    pub r#id: QuestId,
    pub r#name: String,
    pub r#quest_type: QuestType,
    pub r#start_condition: StartCondition,
    pub r#weight: f32,
    pub r#origin: QuestOrigin,
    pub r#requirement: Requirement,
    pub r#level: i32,
    pub r#use_random_seed: bool,
    pub r#nodes: Vec<Node>,
}
impl Quest {
    pub fn new(r#id: QuestId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#quest_type: Default::default(),
            r#start_condition: Default::default(),
            r#weight: Default::default(),
            r#origin: Default::default(),
            r#requirement: Default::default(),
            r#level: Default::default(),
            r#use_random_seed: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<QuestId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<QuestId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_quest_type(mut self, r#quest_type: impl Into<QuestType>) -> Self {
        self.r#quest_type = r#quest_type.into();
        self
    }
    pub fn set_quest_type(&mut self, r#quest_type: impl Into<QuestType>) -> &mut Self {
        self.r#quest_type = r#quest_type.into();
        self
    }
    pub fn with_start_condition(mut self, r#start_condition: impl Into<StartCondition>) -> Self {
        self.r#start_condition = r#start_condition.into();
        self
    }
    pub fn set_start_condition(
        &mut self,
        r#start_condition: impl Into<StartCondition>,
    ) -> &mut Self {
        self.r#start_condition = r#start_condition.into();
        self
    }
    pub fn with_weight(mut self, r#weight: impl Into<f32>) -> Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn set_weight(&mut self, r#weight: impl Into<f32>) -> &mut Self {
        self.r#weight = r#weight.into();
        self
    }
    pub fn with_origin(mut self, r#origin: impl Into<QuestOrigin>) -> Self {
        self.r#origin = r#origin.into();
        self
    }
    pub fn set_origin(&mut self, r#origin: impl Into<QuestOrigin>) -> &mut Self {
        self.r#origin = r#origin.into();
        self
    }
    pub fn with_requirement(mut self, r#requirement: impl Into<Requirement>) -> Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn set_requirement(&mut self, r#requirement: impl Into<Requirement>) -> &mut Self {
        self.r#requirement = r#requirement.into();
        self
    }
    pub fn with_level(mut self, r#level: impl Into<i32>) -> Self {
        self.r#level = r#level.into();
        self
    }
    pub fn set_level(&mut self, r#level: impl Into<i32>) -> &mut Self {
        self.r#level = r#level.into();
        self
    }
    pub fn with_use_random_seed(mut self, r#use_random_seed: impl Into<bool>) -> Self {
        self.r#use_random_seed = r#use_random_seed.into();
        self
    }
    pub fn set_use_random_seed(&mut self, r#use_random_seed: impl Into<bool>) -> &mut Self {
        self.r#use_random_seed = r#use_random_seed.into();
        self
    }
    pub fn with_nodes(mut self, r#nodes: impl Into<Vec<Node>>) -> Self {
        self.r#nodes = r#nodes.into();
        self
    }
    pub fn set_nodes(&mut self, r#nodes: impl Into<Vec<Node>>) -> &mut Self {
        self.r#nodes = r#nodes.into();
        self
    }
}
impl DatabaseItem for Quest {
    fn validate(&mut self) {
        if self.r#weight < (0f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                min = 0f32,
                "Field got truncated"
            );
            self.r#weight = 0f32 as f32;
        }
        if self.r#weight > (1000f32 as f32) {
            tracing::warn!(
                field = "r#weight",
                value = self.r#weight,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#weight = 1000f32 as f32;
        }
        if self.r#level < (0f32 as i32) {
            tracing::warn!(
                field = "r#level",
                value = self.r#level,
                min = 0f32,
                "Field got truncated"
            );
            self.r#level = 0f32 as i32;
        }
        if self.r#level > (1000f32 as i32) {
            tracing::warn!(
                field = "r#level",
                value = self.r#level,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#level = 1000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "Quest"
    }
}
impl DatabaseItemWithId for Quest {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/QuestItem.xml
pub type QuestItemId = DatabaseItemId<QuestItem>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuestItem {
    pub r#id: QuestItemId,
    pub r#name: String,
    pub r#description: String,
    pub r#icon: String,
    pub r#color: String,
    pub r#price: i32,
}
impl QuestItem {
    pub fn new(r#id: QuestItemId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#description: Default::default(),
            r#icon: Default::default(),
            r#color: Default::default(),
            r#price: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<QuestItemId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<QuestItemId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_description(mut self, r#description: impl Into<String>) -> Self {
        self.r#description = r#description.into();
        self
    }
    pub fn set_description(&mut self, r#description: impl Into<String>) -> &mut Self {
        self.r#description = r#description.into();
        self
    }
    pub fn with_icon(mut self, r#icon: impl Into<String>) -> Self {
        self.r#icon = r#icon.into();
        self
    }
    pub fn set_icon(&mut self, r#icon: impl Into<String>) -> &mut Self {
        self.r#icon = r#icon.into();
        self
    }
    pub fn with_color(mut self, r#color: impl Into<String>) -> Self {
        self.r#color = r#color.into();
        self
    }
    pub fn set_color(&mut self, r#color: impl Into<String>) -> &mut Self {
        self.r#color = r#color.into();
        self
    }
    pub fn with_price(mut self, r#price: impl Into<i32>) -> Self {
        self.r#price = r#price.into();
        self
    }
    pub fn set_price(&mut self, r#price: impl Into<i32>) -> &mut Self {
        self.r#price = r#price.into();
        self
    }
}
impl DatabaseItem for QuestItem {
    fn validate(&mut self) {
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                min = 0f32,
                "Field got truncated"
            );
            self.r#price = 0f32 as i32;
        }
        if self.r#price > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#price = 1000000000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "QuestItem"
    }
}
impl DatabaseItemWithId for QuestItem {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Satellite.xml
pub type SatelliteId = DatabaseItemId<Satellite>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Satellite {
    pub r#id: SatelliteId,
    pub r#name: String,
    pub r#model_image: String,
    pub r#model_scale: f32,
    pub r#size_class: SizeClass,
    pub r#layout: String,
    pub r#barrels: Vec<Barrel>,
}
impl Satellite {
    pub fn new(r#id: SatelliteId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#model_image: Default::default(),
            r#model_scale: Default::default(),
            r#size_class: Default::default(),
            r#layout: Default::default(),
            r#barrels: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<SatelliteId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<SatelliteId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_model_image(mut self, r#model_image: impl Into<String>) -> Self {
        self.r#model_image = r#model_image.into();
        self
    }
    pub fn set_model_image(&mut self, r#model_image: impl Into<String>) -> &mut Self {
        self.r#model_image = r#model_image.into();
        self
    }
    pub fn with_model_scale(mut self, r#model_scale: impl Into<f32>) -> Self {
        self.r#model_scale = r#model_scale.into();
        self
    }
    pub fn set_model_scale(&mut self, r#model_scale: impl Into<f32>) -> &mut Self {
        self.r#model_scale = r#model_scale.into();
        self
    }
    pub fn with_size_class(mut self, r#size_class: impl Into<SizeClass>) -> Self {
        self.r#size_class = r#size_class.into();
        self
    }
    pub fn set_size_class(&mut self, r#size_class: impl Into<SizeClass>) -> &mut Self {
        self.r#size_class = r#size_class.into();
        self
    }
    pub fn with_layout(mut self, r#layout: impl Into<String>) -> Self {
        self.r#layout = r#layout.into();
        self
    }
    pub fn set_layout(&mut self, r#layout: impl Into<String>) -> &mut Self {
        self.r#layout = r#layout.into();
        self
    }
    pub fn with_barrels(mut self, r#barrels: impl Into<Vec<Barrel>>) -> Self {
        self.r#barrels = r#barrels.into();
        self
    }
    pub fn set_barrels(&mut self, r#barrels: impl Into<Vec<Barrel>>) -> &mut Self {
        self.r#barrels = r#barrels.into();
        self
    }
}
impl DatabaseItem for Satellite {
    fn validate(&mut self) {
        if self.r#model_scale < (0.1f32 as f32) {
            tracing::warn!(
                field = "r#model_scale",
                value = self.r#model_scale,
                min = 0.1f32,
                "Field got truncated"
            );
            self.r#model_scale = 0.1f32 as f32;
        }
        if self.r#model_scale > (100f32 as f32) {
            tracing::warn!(
                field = "r#model_scale",
                value = self.r#model_scale,
                max = 100f32,
                "Field got truncated"
            );
            self.r#model_scale = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "Satellite"
    }
}
impl DatabaseItemWithId for Satellite {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/SatelliteBuild.xml
pub type SatelliteBuildId = DatabaseItemId<SatelliteBuild>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SatelliteBuild {
    pub r#id: SatelliteBuildId,
    pub r#satellite_id: SatelliteId,
    pub r#not_available_in_game: bool,
    pub r#difficulty_class: DifficultyClass,
    pub r#components: Vec<InstalledComponent>,
}
impl SatelliteBuild {
    pub fn new(r#id: SatelliteBuildId, r#satellite_id: SatelliteId) -> Self {
        Self {
            r#id,
            r#satellite_id,
            r#not_available_in_game: Default::default(),
            r#difficulty_class: Default::default(),
            r#components: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<SatelliteBuildId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<SatelliteBuildId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_satellite_id(mut self, r#satellite_id: impl Into<SatelliteId>) -> Self {
        self.r#satellite_id = r#satellite_id.into();
        self
    }
    pub fn set_satellite_id(&mut self, r#satellite_id: impl Into<SatelliteId>) -> &mut Self {
        self.r#satellite_id = r#satellite_id.into();
        self
    }
    pub fn with_not_available_in_game(mut self, r#not_available_in_game: impl Into<bool>) -> Self {
        self.r#not_available_in_game = r#not_available_in_game.into();
        self
    }
    pub fn set_not_available_in_game(
        &mut self,
        r#not_available_in_game: impl Into<bool>,
    ) -> &mut Self {
        self.r#not_available_in_game = r#not_available_in_game.into();
        self
    }
    pub fn with_difficulty_class(mut self, r#difficulty_class: impl Into<DifficultyClass>) -> Self {
        self.r#difficulty_class = r#difficulty_class.into();
        self
    }
    pub fn set_difficulty_class(
        &mut self,
        r#difficulty_class: impl Into<DifficultyClass>,
    ) -> &mut Self {
        self.r#difficulty_class = r#difficulty_class.into();
        self
    }
    pub fn with_components(mut self, r#components: impl Into<Vec<InstalledComponent>>) -> Self {
        self.r#components = r#components.into();
        self
    }
    pub fn set_components(
        &mut self,
        r#components: impl Into<Vec<InstalledComponent>>,
    ) -> &mut Self {
        self.r#components = r#components.into();
        self
    }
}
impl DatabaseItem for SatelliteBuild {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "SatelliteBuild"
    }
}
impl DatabaseItemWithId for SatelliteBuild {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Ship.xml
pub type ShipId = DatabaseItemId<Ship>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ship {
    pub r#id: ShipId,
    pub r#ship_type: ShipType,
    pub r#ship_rarity: ShipRarity,
    pub r#size_class: SizeClass,
    pub r#name: String,
    pub r#description: String,
    pub r#faction: Option<FactionId>,
    pub r#icon_image: String,
    pub r#icon_scale: f32,
    pub r#model_image: String,
    pub r#model_scale: f32,
    pub r#engine_color: String,
    pub r#engines: Vec<Engine>,
    pub r#layout: String,
    pub r#barrels: Vec<Barrel>,
    pub r#features: ShipFeatures,
    pub r#collider_tolerance: f32,
    pub r#engine_position: glam::f32::Vec2,
    pub r#engine_size: f32,
    pub r#ship_category: i32,
    pub r#energy_resistance: f32,
    pub r#kinetic_resistance: f32,
    pub r#heat_resistance: f32,
    pub r#regeneration: bool,
    pub r#builtin_devices: Vec<DeviceId>,
    pub r#base_weight_modifier: f32,
}
impl Ship {
    pub fn new(r#id: ShipId) -> Self {
        Self {
            r#id,
            r#ship_type: Default::default(),
            r#ship_rarity: Default::default(),
            r#size_class: Default::default(),
            r#name: Default::default(),
            r#description: Default::default(),
            r#faction: Default::default(),
            r#icon_image: Default::default(),
            r#icon_scale: Default::default(),
            r#model_image: Default::default(),
            r#model_scale: Default::default(),
            r#engine_color: Default::default(),
            r#engines: Default::default(),
            r#layout: Default::default(),
            r#barrels: Default::default(),
            r#features: Default::default(),
            r#collider_tolerance: 0.02f32,
            r#engine_position: Default::default(),
            r#engine_size: Default::default(),
            r#ship_category: Default::default(),
            r#energy_resistance: Default::default(),
            r#kinetic_resistance: Default::default(),
            r#heat_resistance: Default::default(),
            r#regeneration: Default::default(),
            r#builtin_devices: Default::default(),
            r#base_weight_modifier: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<ShipId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<ShipId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_ship_type(mut self, r#ship_type: impl Into<ShipType>) -> Self {
        self.r#ship_type = r#ship_type.into();
        self
    }
    pub fn set_ship_type(&mut self, r#ship_type: impl Into<ShipType>) -> &mut Self {
        self.r#ship_type = r#ship_type.into();
        self
    }
    pub fn with_ship_rarity(mut self, r#ship_rarity: impl Into<ShipRarity>) -> Self {
        self.r#ship_rarity = r#ship_rarity.into();
        self
    }
    pub fn set_ship_rarity(&mut self, r#ship_rarity: impl Into<ShipRarity>) -> &mut Self {
        self.r#ship_rarity = r#ship_rarity.into();
        self
    }
    pub fn with_size_class(mut self, r#size_class: impl Into<SizeClass>) -> Self {
        self.r#size_class = r#size_class.into();
        self
    }
    pub fn set_size_class(&mut self, r#size_class: impl Into<SizeClass>) -> &mut Self {
        self.r#size_class = r#size_class.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_description(mut self, r#description: impl Into<String>) -> Self {
        self.r#description = r#description.into();
        self
    }
    pub fn set_description(&mut self, r#description: impl Into<String>) -> &mut Self {
        self.r#description = r#description.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn with_icon_image(mut self, r#icon_image: impl Into<String>) -> Self {
        self.r#icon_image = r#icon_image.into();
        self
    }
    pub fn set_icon_image(&mut self, r#icon_image: impl Into<String>) -> &mut Self {
        self.r#icon_image = r#icon_image.into();
        self
    }
    pub fn with_icon_scale(mut self, r#icon_scale: impl Into<f32>) -> Self {
        self.r#icon_scale = r#icon_scale.into();
        self
    }
    pub fn set_icon_scale(&mut self, r#icon_scale: impl Into<f32>) -> &mut Self {
        self.r#icon_scale = r#icon_scale.into();
        self
    }
    pub fn with_model_image(mut self, r#model_image: impl Into<String>) -> Self {
        self.r#model_image = r#model_image.into();
        self
    }
    pub fn set_model_image(&mut self, r#model_image: impl Into<String>) -> &mut Self {
        self.r#model_image = r#model_image.into();
        self
    }
    pub fn with_model_scale(mut self, r#model_scale: impl Into<f32>) -> Self {
        self.r#model_scale = r#model_scale.into();
        self
    }
    pub fn set_model_scale(&mut self, r#model_scale: impl Into<f32>) -> &mut Self {
        self.r#model_scale = r#model_scale.into();
        self
    }
    pub fn with_engine_color(mut self, r#engine_color: impl Into<String>) -> Self {
        self.r#engine_color = r#engine_color.into();
        self
    }
    pub fn set_engine_color(&mut self, r#engine_color: impl Into<String>) -> &mut Self {
        self.r#engine_color = r#engine_color.into();
        self
    }
    pub fn with_engines(mut self, r#engines: impl Into<Vec<Engine>>) -> Self {
        self.r#engines = r#engines.into();
        self
    }
    pub fn set_engines(&mut self, r#engines: impl Into<Vec<Engine>>) -> &mut Self {
        self.r#engines = r#engines.into();
        self
    }
    pub fn with_layout(mut self, r#layout: impl Into<String>) -> Self {
        self.r#layout = r#layout.into();
        self
    }
    pub fn set_layout(&mut self, r#layout: impl Into<String>) -> &mut Self {
        self.r#layout = r#layout.into();
        self
    }
    pub fn with_barrels(mut self, r#barrels: impl Into<Vec<Barrel>>) -> Self {
        self.r#barrels = r#barrels.into();
        self
    }
    pub fn set_barrels(&mut self, r#barrels: impl Into<Vec<Barrel>>) -> &mut Self {
        self.r#barrels = r#barrels.into();
        self
    }
    pub fn with_features(mut self, r#features: impl Into<ShipFeatures>) -> Self {
        self.r#features = r#features.into();
        self
    }
    pub fn set_features(&mut self, r#features: impl Into<ShipFeatures>) -> &mut Self {
        self.r#features = r#features.into();
        self
    }
    pub fn with_collider_tolerance(mut self, r#collider_tolerance: impl Into<f32>) -> Self {
        self.r#collider_tolerance = r#collider_tolerance.into();
        self
    }
    pub fn set_collider_tolerance(&mut self, r#collider_tolerance: impl Into<f32>) -> &mut Self {
        self.r#collider_tolerance = r#collider_tolerance.into();
        self
    }
    pub fn with_engine_position(mut self, r#engine_position: impl Into<glam::f32::Vec2>) -> Self {
        self.r#engine_position = r#engine_position.into();
        self
    }
    pub fn set_engine_position(
        &mut self,
        r#engine_position: impl Into<glam::f32::Vec2>,
    ) -> &mut Self {
        self.r#engine_position = r#engine_position.into();
        self
    }
    pub fn with_engine_size(mut self, r#engine_size: impl Into<f32>) -> Self {
        self.r#engine_size = r#engine_size.into();
        self
    }
    pub fn set_engine_size(&mut self, r#engine_size: impl Into<f32>) -> &mut Self {
        self.r#engine_size = r#engine_size.into();
        self
    }
    pub fn with_ship_category(mut self, r#ship_category: impl Into<i32>) -> Self {
        self.r#ship_category = r#ship_category.into();
        self
    }
    pub fn set_ship_category(&mut self, r#ship_category: impl Into<i32>) -> &mut Self {
        self.r#ship_category = r#ship_category.into();
        self
    }
    pub fn with_energy_resistance(mut self, r#energy_resistance: impl Into<f32>) -> Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn set_energy_resistance(&mut self, r#energy_resistance: impl Into<f32>) -> &mut Self {
        self.r#energy_resistance = r#energy_resistance.into();
        self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: impl Into<f32>) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn set_kinetic_resistance(&mut self, r#kinetic_resistance: impl Into<f32>) -> &mut Self {
        self.r#kinetic_resistance = r#kinetic_resistance.into();
        self
    }
    pub fn with_heat_resistance(mut self, r#heat_resistance: impl Into<f32>) -> Self {
        self.r#heat_resistance = r#heat_resistance.into();
        self
    }
    pub fn set_heat_resistance(&mut self, r#heat_resistance: impl Into<f32>) -> &mut Self {
        self.r#heat_resistance = r#heat_resistance.into();
        self
    }
    pub fn with_regeneration(mut self, r#regeneration: impl Into<bool>) -> Self {
        self.r#regeneration = r#regeneration.into();
        self
    }
    pub fn set_regeneration(&mut self, r#regeneration: impl Into<bool>) -> &mut Self {
        self.r#regeneration = r#regeneration.into();
        self
    }
    pub fn with_builtin_devices(mut self, r#builtin_devices: impl Into<Vec<DeviceId>>) -> Self {
        self.r#builtin_devices = r#builtin_devices.into();
        self
    }
    pub fn set_builtin_devices(
        &mut self,
        r#builtin_devices: impl Into<Vec<DeviceId>>,
    ) -> &mut Self {
        self.r#builtin_devices = r#builtin_devices.into();
        self
    }
    pub fn with_base_weight_modifier(mut self, r#base_weight_modifier: impl Into<f32>) -> Self {
        self.r#base_weight_modifier = r#base_weight_modifier.into();
        self
    }
    pub fn set_base_weight_modifier(
        &mut self,
        r#base_weight_modifier: impl Into<f32>,
    ) -> &mut Self {
        self.r#base_weight_modifier = r#base_weight_modifier.into();
        self
    }
}
impl DatabaseItem for Ship {
    fn validate(&mut self) {
        if self.r#icon_scale < (0.1f32 as f32) {
            tracing::warn!(
                field = "r#icon_scale",
                value = self.r#icon_scale,
                min = 0.1f32,
                "Field got truncated"
            );
            self.r#icon_scale = 0.1f32 as f32;
        }
        if self.r#icon_scale > (100f32 as f32) {
            tracing::warn!(
                field = "r#icon_scale",
                value = self.r#icon_scale,
                max = 100f32,
                "Field got truncated"
            );
            self.r#icon_scale = 100f32 as f32;
        }
        if self.r#model_scale < (0.1f32 as f32) {
            tracing::warn!(
                field = "r#model_scale",
                value = self.r#model_scale,
                min = 0.1f32,
                "Field got truncated"
            );
            self.r#model_scale = 0.1f32 as f32;
        }
        if self.r#model_scale > (100f32 as f32) {
            tracing::warn!(
                field = "r#model_scale",
                value = self.r#model_scale,
                max = 100f32,
                "Field got truncated"
            );
            self.r#model_scale = 100f32 as f32;
        }
        if self.r#collider_tolerance < (0f32 as f32) {
            tracing::warn!(
                field = "r#collider_tolerance",
                value = self.r#collider_tolerance,
                min = 0f32,
                "Field got truncated"
            );
            self.r#collider_tolerance = 0f32 as f32;
        }
        if self.r#collider_tolerance > (1f32 as f32) {
            tracing::warn!(
                field = "r#collider_tolerance",
                value = self.r#collider_tolerance,
                max = 1f32,
                "Field got truncated"
            );
            self.r#collider_tolerance = 1f32 as f32;
        }
        let dw: glam::f32::Vec2 = Default::default();
        if self.r#engine_position != dw {
            tracing::error!(
                ield = "r#engine_position",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: f32 = Default::default();
        if self.r#engine_size != dw {
            tracing::error!(
                ield = "r#engine_size",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: i32 = Default::default();
        if self.r#ship_category != dw {
            tracing::error!(
                ield = "r#ship_category",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: f32 = Default::default();
        if self.r#energy_resistance != dw {
            tracing::error!(
                ield = "r#energy_resistance",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: f32 = Default::default();
        if self.r#kinetic_resistance != dw {
            tracing::error!(
                ield = "r#kinetic_resistance",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: f32 = Default::default();
        if self.r#heat_resistance != dw {
            tracing::error!(
                ield = "r#heat_resistance",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: bool = Default::default();
        if self.r#regeneration != dw {
            tracing::error!(
                ield = "r#regeneration",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: Vec<DeviceId> = Default::default();
        if self.r#builtin_devices != dw {
            tracing::error!(
                ield = "r#builtin_devices",
                "Obsolete field usage detected, generated code may not work",
            );
        }
        let dw: f32 = Default::default();
        if self.r#base_weight_modifier != dw {
            tracing::error!(
                ield = "r#base_weight_modifier",
                "Obsolete field usage detected, generated code may not work",
            );
        }
    }
    fn type_name() -> &'static str {
        "Ship"
    }
}
impl DatabaseItemWithId for Ship {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ShipBuild.xml
pub type ShipBuildId = DatabaseItemId<ShipBuild>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ShipBuild {
    pub r#id: ShipBuildId,
    pub r#ship_id: ShipId,
    pub r#available_for_player: bool,
    pub r#available_for_enemy: bool,
    pub r#difficulty_class: DifficultyClass,
    pub r#build_faction: Option<FactionId>,
    pub r#custom_ai: Option<BehaviorTreeId>,
    pub r#components: Vec<InstalledComponent>,
    pub r#not_available_in_game: bool,
}
impl ShipBuild {
    pub fn new(r#id: ShipBuildId, r#ship_id: ShipId) -> Self {
        Self {
            r#id,
            r#ship_id,
            r#available_for_player: true,
            r#available_for_enemy: true,
            r#difficulty_class: Default::default(),
            r#build_faction: Default::default(),
            r#custom_ai: Default::default(),
            r#components: Default::default(),
            r#not_available_in_game: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<ShipBuildId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<ShipBuildId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_ship_id(mut self, r#ship_id: impl Into<ShipId>) -> Self {
        self.r#ship_id = r#ship_id.into();
        self
    }
    pub fn set_ship_id(&mut self, r#ship_id: impl Into<ShipId>) -> &mut Self {
        self.r#ship_id = r#ship_id.into();
        self
    }
    pub fn with_available_for_player(mut self, r#available_for_player: impl Into<bool>) -> Self {
        self.r#available_for_player = r#available_for_player.into();
        self
    }
    pub fn set_available_for_player(
        &mut self,
        r#available_for_player: impl Into<bool>,
    ) -> &mut Self {
        self.r#available_for_player = r#available_for_player.into();
        self
    }
    pub fn with_available_for_enemy(mut self, r#available_for_enemy: impl Into<bool>) -> Self {
        self.r#available_for_enemy = r#available_for_enemy.into();
        self
    }
    pub fn set_available_for_enemy(&mut self, r#available_for_enemy: impl Into<bool>) -> &mut Self {
        self.r#available_for_enemy = r#available_for_enemy.into();
        self
    }
    pub fn with_difficulty_class(mut self, r#difficulty_class: impl Into<DifficultyClass>) -> Self {
        self.r#difficulty_class = r#difficulty_class.into();
        self
    }
    pub fn set_difficulty_class(
        &mut self,
        r#difficulty_class: impl Into<DifficultyClass>,
    ) -> &mut Self {
        self.r#difficulty_class = r#difficulty_class.into();
        self
    }
    pub fn with_build_faction(mut self, r#build_faction: impl Into<Option<FactionId>>) -> Self {
        self.r#build_faction = r#build_faction.into();
        self
    }
    pub fn set_build_faction(
        &mut self,
        r#build_faction: impl Into<Option<FactionId>>,
    ) -> &mut Self {
        self.r#build_faction = r#build_faction.into();
        self
    }
    pub fn with_custom_ai(mut self, r#custom_ai: impl Into<Option<BehaviorTreeId>>) -> Self {
        self.r#custom_ai = r#custom_ai.into();
        self
    }
    pub fn set_custom_ai(&mut self, r#custom_ai: impl Into<Option<BehaviorTreeId>>) -> &mut Self {
        self.r#custom_ai = r#custom_ai.into();
        self
    }
    pub fn with_components(mut self, r#components: impl Into<Vec<InstalledComponent>>) -> Self {
        self.r#components = r#components.into();
        self
    }
    pub fn set_components(
        &mut self,
        r#components: impl Into<Vec<InstalledComponent>>,
    ) -> &mut Self {
        self.r#components = r#components.into();
        self
    }
    pub fn with_not_available_in_game(mut self, r#not_available_in_game: impl Into<bool>) -> Self {
        self.r#not_available_in_game = r#not_available_in_game.into();
        self
    }
    pub fn set_not_available_in_game(
        &mut self,
        r#not_available_in_game: impl Into<bool>,
    ) -> &mut Self {
        self.r#not_available_in_game = r#not_available_in_game.into();
        self
    }
}
impl DatabaseItem for ShipBuild {
    fn validate(&mut self) {
        let dw: bool = Default::default();
        if self.r#not_available_in_game != dw {
            tracing::error!(
                ield = "r#not_available_in_game",
                "Obsolete field usage detected, generated code may not work",
            );
        }
    }
    fn type_name() -> &'static str {
        "ShipBuild"
    }
}
impl DatabaseItemWithId for ShipBuild {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Skill.xml
pub type SkillId = DatabaseItemId<Skill>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Skill {
    pub r#id: SkillId,
    pub r#name: String,
    pub r#icon: String,
    pub r#description: String,
    pub r#base_requirement: f32,
    pub r#requirement_per_level: f32,
    pub r#base_price: f32,
    pub r#price_per_level: f32,
    pub r#max_level: i32,
}
impl Skill {
    pub fn new(r#id: SkillId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#icon: Default::default(),
            r#description: Default::default(),
            r#base_requirement: Default::default(),
            r#requirement_per_level: Default::default(),
            r#base_price: Default::default(),
            r#price_per_level: Default::default(),
            r#max_level: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<SkillId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<SkillId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_name(mut self, r#name: impl Into<String>) -> Self {
        self.r#name = r#name.into();
        self
    }
    pub fn set_name(&mut self, r#name: impl Into<String>) -> &mut Self {
        self.r#name = r#name.into();
        self
    }
    pub fn with_icon(mut self, r#icon: impl Into<String>) -> Self {
        self.r#icon = r#icon.into();
        self
    }
    pub fn set_icon(&mut self, r#icon: impl Into<String>) -> &mut Self {
        self.r#icon = r#icon.into();
        self
    }
    pub fn with_description(mut self, r#description: impl Into<String>) -> Self {
        self.r#description = r#description.into();
        self
    }
    pub fn set_description(&mut self, r#description: impl Into<String>) -> &mut Self {
        self.r#description = r#description.into();
        self
    }
    pub fn with_base_requirement(mut self, r#base_requirement: impl Into<f32>) -> Self {
        self.r#base_requirement = r#base_requirement.into();
        self
    }
    pub fn set_base_requirement(&mut self, r#base_requirement: impl Into<f32>) -> &mut Self {
        self.r#base_requirement = r#base_requirement.into();
        self
    }
    pub fn with_requirement_per_level(mut self, r#requirement_per_level: impl Into<f32>) -> Self {
        self.r#requirement_per_level = r#requirement_per_level.into();
        self
    }
    pub fn set_requirement_per_level(
        &mut self,
        r#requirement_per_level: impl Into<f32>,
    ) -> &mut Self {
        self.r#requirement_per_level = r#requirement_per_level.into();
        self
    }
    pub fn with_base_price(mut self, r#base_price: impl Into<f32>) -> Self {
        self.r#base_price = r#base_price.into();
        self
    }
    pub fn set_base_price(&mut self, r#base_price: impl Into<f32>) -> &mut Self {
        self.r#base_price = r#base_price.into();
        self
    }
    pub fn with_price_per_level(mut self, r#price_per_level: impl Into<f32>) -> Self {
        self.r#price_per_level = r#price_per_level.into();
        self
    }
    pub fn set_price_per_level(&mut self, r#price_per_level: impl Into<f32>) -> &mut Self {
        self.r#price_per_level = r#price_per_level.into();
        self
    }
    pub fn with_max_level(mut self, r#max_level: impl Into<i32>) -> Self {
        self.r#max_level = r#max_level.into();
        self
    }
    pub fn set_max_level(&mut self, r#max_level: impl Into<i32>) -> &mut Self {
        self.r#max_level = r#max_level.into();
        self
    }
}
impl DatabaseItem for Skill {
    fn validate(&mut self) {
        if self.r#base_requirement < (0f32 as f32) {
            tracing::warn!(
                field = "r#base_requirement",
                value = self.r#base_requirement,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_requirement = 0f32 as f32;
        }
        if self.r#base_requirement > (100f32 as f32) {
            tracing::warn!(
                field = "r#base_requirement",
                value = self.r#base_requirement,
                max = 100f32,
                "Field got truncated"
            );
            self.r#base_requirement = 100f32 as f32;
        }
        if self.r#requirement_per_level < (0f32 as f32) {
            tracing::warn!(
                field = "r#requirement_per_level",
                value = self.r#requirement_per_level,
                min = 0f32,
                "Field got truncated"
            );
            self.r#requirement_per_level = 0f32 as f32;
        }
        if self.r#requirement_per_level > (100f32 as f32) {
            tracing::warn!(
                field = "r#requirement_per_level",
                value = self.r#requirement_per_level,
                max = 100f32,
                "Field got truncated"
            );
            self.r#requirement_per_level = 100f32 as f32;
        }
        if self.r#base_price < (0f32 as f32) {
            tracing::warn!(
                field = "r#base_price",
                value = self.r#base_price,
                min = 0f32,
                "Field got truncated"
            );
            self.r#base_price = 0f32 as f32;
        }
        if self.r#base_price > (100f32 as f32) {
            tracing::warn!(
                field = "r#base_price",
                value = self.r#base_price,
                max = 100f32,
                "Field got truncated"
            );
            self.r#base_price = 100f32 as f32;
        }
        if self.r#price_per_level < (0f32 as f32) {
            tracing::warn!(
                field = "r#price_per_level",
                value = self.r#price_per_level,
                min = 0f32,
                "Field got truncated"
            );
            self.r#price_per_level = 0f32 as f32;
        }
        if self.r#price_per_level > (100f32 as f32) {
            tracing::warn!(
                field = "r#price_per_level",
                value = self.r#price_per_level,
                max = 100f32,
                "Field got truncated"
            );
            self.r#price_per_level = 100f32 as f32;
        }
        if self.r#max_level < (1f32 as i32) {
            tracing::warn!(
                field = "r#max_level",
                value = self.r#max_level,
                min = 1f32,
                "Field got truncated"
            );
            self.r#max_level = 1f32 as i32;
        }
        if self.r#max_level > (1000f32 as i32) {
            tracing::warn!(
                field = "r#max_level",
                value = self.r#max_level,
                max = 1000f32,
                "Field got truncated"
            );
            self.r#max_level = 1000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "Skill"
    }
}
impl DatabaseItemWithId for Skill {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Technology.xml
pub type TechnologyId = DatabaseItemId<Technology>;
#[derive(Debug, Clone)]
pub enum Technology {
    Component(TechnologyComponent),
    Ship(TechnologyShip),
    Satellite(TechnologySatellite),
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyComponent {
    pub r#id: TechnologyId,
    pub r#item_id: ComponentId,
    pub r#faction: Option<FactionId>,
    pub r#price: i32,
    pub r#hidden: bool,
    pub r#special: bool,
    pub r#dependencies: Vec<TechnologyId>,
}
impl TechnologyComponent {
    pub fn new(r#id: TechnologyId, r#item_id: ComponentId) -> Self {
        Self {
            r#id,
            r#item_id,
            r#faction: Default::default(),
            r#price: Default::default(),
            r#hidden: Default::default(),
            r#special: Default::default(),
            r#dependencies: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<TechnologyId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<TechnologyId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<ComponentId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<ComponentId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn with_price(mut self, r#price: impl Into<i32>) -> Self {
        self.r#price = r#price.into();
        self
    }
    pub fn set_price(&mut self, r#price: impl Into<i32>) -> &mut Self {
        self.r#price = r#price.into();
        self
    }
    pub fn with_hidden(mut self, r#hidden: impl Into<bool>) -> Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn set_hidden(&mut self, r#hidden: impl Into<bool>) -> &mut Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn with_special(mut self, r#special: impl Into<bool>) -> Self {
        self.r#special = r#special.into();
        self
    }
    pub fn set_special(&mut self, r#special: impl Into<bool>) -> &mut Self {
        self.r#special = r#special.into();
        self
    }
    pub fn with_dependencies(mut self, r#dependencies: impl Into<Vec<TechnologyId>>) -> Self {
        self.r#dependencies = r#dependencies.into();
        self
    }
    pub fn set_dependencies(&mut self, r#dependencies: impl Into<Vec<TechnologyId>>) -> &mut Self {
        self.r#dependencies = r#dependencies.into();
        self
    }
}
impl DatabaseItem for TechnologyComponent {
    fn validate(&mut self) {
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                min = 0f32,
                "Field got truncated"
            );
            self.r#price = 0f32 as i32;
        }
        if self.r#price > (10000f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#price = 10000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "TechnologyComponent"
    }
}
impl From<TechnologyComponent> for Technology {
    fn from(item: TechnologyComponent) -> Self {
        Self::Component(item)
    }
}
impl TechnologyComponent {
    pub fn wrap(self) -> Technology {
        self.into()
    }
}
impl Technology {
    pub fn component(r#id: TechnologyId, r#item_id: ComponentId) -> TechnologyComponent {
        TechnologyComponent::new(r#id, r#item_id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyShip {
    pub r#id: TechnologyId,
    pub r#item_id: ShipId,
    pub r#price: i32,
    pub r#hidden: bool,
    pub r#special: bool,
    pub r#dependencies: Vec<TechnologyId>,
}
impl TechnologyShip {
    pub fn new(r#id: TechnologyId, r#item_id: ShipId) -> Self {
        Self {
            r#id,
            r#item_id,
            r#price: Default::default(),
            r#hidden: Default::default(),
            r#special: Default::default(),
            r#dependencies: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<TechnologyId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<TechnologyId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<ShipId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<ShipId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn with_price(mut self, r#price: impl Into<i32>) -> Self {
        self.r#price = r#price.into();
        self
    }
    pub fn set_price(&mut self, r#price: impl Into<i32>) -> &mut Self {
        self.r#price = r#price.into();
        self
    }
    pub fn with_hidden(mut self, r#hidden: impl Into<bool>) -> Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn set_hidden(&mut self, r#hidden: impl Into<bool>) -> &mut Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn with_special(mut self, r#special: impl Into<bool>) -> Self {
        self.r#special = r#special.into();
        self
    }
    pub fn set_special(&mut self, r#special: impl Into<bool>) -> &mut Self {
        self.r#special = r#special.into();
        self
    }
    pub fn with_dependencies(mut self, r#dependencies: impl Into<Vec<TechnologyId>>) -> Self {
        self.r#dependencies = r#dependencies.into();
        self
    }
    pub fn set_dependencies(&mut self, r#dependencies: impl Into<Vec<TechnologyId>>) -> &mut Self {
        self.r#dependencies = r#dependencies.into();
        self
    }
}
impl DatabaseItem for TechnologyShip {
    fn validate(&mut self) {
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                min = 0f32,
                "Field got truncated"
            );
            self.r#price = 0f32 as i32;
        }
        if self.r#price > (10000f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#price = 10000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "TechnologyShip"
    }
}
impl From<TechnologyShip> for Technology {
    fn from(item: TechnologyShip) -> Self {
        Self::Ship(item)
    }
}
impl TechnologyShip {
    pub fn wrap(self) -> Technology {
        self.into()
    }
}
impl Technology {
    pub fn ship(r#id: TechnologyId, r#item_id: ShipId) -> TechnologyShip {
        TechnologyShip::new(r#id, r#item_id)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologySatellite {
    pub r#id: TechnologyId,
    pub r#item_id: SatelliteId,
    pub r#faction: Option<FactionId>,
    pub r#price: i32,
    pub r#hidden: bool,
    pub r#special: bool,
    pub r#dependencies: Vec<TechnologyId>,
}
impl TechnologySatellite {
    pub fn new(r#id: TechnologyId, r#item_id: SatelliteId) -> Self {
        Self {
            r#id,
            r#item_id,
            r#faction: Default::default(),
            r#price: Default::default(),
            r#hidden: Default::default(),
            r#special: Default::default(),
            r#dependencies: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<TechnologyId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<TechnologyId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_item_id(mut self, r#item_id: impl Into<SatelliteId>) -> Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn set_item_id(&mut self, r#item_id: impl Into<SatelliteId>) -> &mut Self {
        self.r#item_id = r#item_id.into();
        self
    }
    pub fn with_faction(mut self, r#faction: impl Into<Option<FactionId>>) -> Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn set_faction(&mut self, r#faction: impl Into<Option<FactionId>>) -> &mut Self {
        self.r#faction = r#faction.into();
        self
    }
    pub fn with_price(mut self, r#price: impl Into<i32>) -> Self {
        self.r#price = r#price.into();
        self
    }
    pub fn set_price(&mut self, r#price: impl Into<i32>) -> &mut Self {
        self.r#price = r#price.into();
        self
    }
    pub fn with_hidden(mut self, r#hidden: impl Into<bool>) -> Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn set_hidden(&mut self, r#hidden: impl Into<bool>) -> &mut Self {
        self.r#hidden = r#hidden.into();
        self
    }
    pub fn with_special(mut self, r#special: impl Into<bool>) -> Self {
        self.r#special = r#special.into();
        self
    }
    pub fn set_special(&mut self, r#special: impl Into<bool>) -> &mut Self {
        self.r#special = r#special.into();
        self
    }
    pub fn with_dependencies(mut self, r#dependencies: impl Into<Vec<TechnologyId>>) -> Self {
        self.r#dependencies = r#dependencies.into();
        self
    }
    pub fn set_dependencies(&mut self, r#dependencies: impl Into<Vec<TechnologyId>>) -> &mut Self {
        self.r#dependencies = r#dependencies.into();
        self
    }
}
impl DatabaseItem for TechnologySatellite {
    fn validate(&mut self) {
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                min = 0f32,
                "Field got truncated"
            );
            self.r#price = 0f32 as i32;
        }
        if self.r#price > (10000f32 as i32) {
            tracing::warn!(
                field = "r#price",
                value = self.r#price,
                max = 10000f32,
                "Field got truncated"
            );
            self.r#price = 10000f32 as i32;
        }
    }
    fn type_name() -> &'static str {
        "TechnologySatellite"
    }
}
impl From<TechnologySatellite> for Technology {
    fn from(item: TechnologySatellite) -> Self {
        Self::Satellite(item)
    }
}
impl TechnologySatellite {
    pub fn wrap(self) -> Technology {
        self.into()
    }
}
impl Technology {
    pub fn satellite(r#id: TechnologyId, r#item_id: SatelliteId) -> TechnologySatellite {
        TechnologySatellite::new(r#id, r#item_id)
    }
}
impl serde::Serialize for Technology {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "Technology")]
        struct AdjTagged<T> {
            #[serde(rename = "Type")]
            t: TechType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Component(x) => AdjTagged {
                t: TechType::Component,
                c: x,
            }
            .serialize(serializer),
            Self::Ship(x) => AdjTagged {
                t: TechType::Ship,
                c: x,
            }
            .serialize(serializer),
            Self::Satellite(x) => AdjTagged {
                t: TechType::Satellite,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl Technology {
    pub fn r#id(&self) -> &TechnologyId {
        match self {
            Self::Component(x) => &x.r#id,
            Self::Ship(x) => &x.r#id,
            Self::Satellite(x) => &x.r#id,
        }
    }
    pub fn id_mut(&mut self) -> &mut TechnologyId {
        match self {
            Self::Component(x) => &mut x.r#id,
            Self::Ship(x) => &mut x.r#id,
            Self::Satellite(x) => &mut x.r#id,
        }
    }
}
impl Technology {
    pub fn r#price(&self) -> &i32 {
        match self {
            Self::Component(x) => &x.r#price,
            Self::Ship(x) => &x.r#price,
            Self::Satellite(x) => &x.r#price,
        }
    }
    pub fn price_mut(&mut self) -> &mut i32 {
        match self {
            Self::Component(x) => &mut x.r#price,
            Self::Ship(x) => &mut x.r#price,
            Self::Satellite(x) => &mut x.r#price,
        }
    }
}
impl Technology {
    pub fn r#hidden(&self) -> &bool {
        match self {
            Self::Component(x) => &x.r#hidden,
            Self::Ship(x) => &x.r#hidden,
            Self::Satellite(x) => &x.r#hidden,
        }
    }
    pub fn hidden_mut(&mut self) -> &mut bool {
        match self {
            Self::Component(x) => &mut x.r#hidden,
            Self::Ship(x) => &mut x.r#hidden,
            Self::Satellite(x) => &mut x.r#hidden,
        }
    }
}
impl Technology {
    pub fn r#special(&self) -> &bool {
        match self {
            Self::Component(x) => &x.r#special,
            Self::Ship(x) => &x.r#special,
            Self::Satellite(x) => &x.r#special,
        }
    }
    pub fn special_mut(&mut self) -> &mut bool {
        match self {
            Self::Component(x) => &mut x.r#special,
            Self::Ship(x) => &mut x.r#special,
            Self::Satellite(x) => &mut x.r#special,
        }
    }
}
impl Technology {
    pub fn r#dependencies(&self) -> &Vec<TechnologyId> {
        match self {
            Self::Component(x) => &x.r#dependencies,
            Self::Ship(x) => &x.r#dependencies,
            Self::Satellite(x) => &x.r#dependencies,
        }
    }
    pub fn dependencies_mut(&mut self) -> &mut Vec<TechnologyId> {
        match self {
            Self::Component(x) => &mut x.r#dependencies,
            Self::Ship(x) => &mut x.r#dependencies,
            Self::Satellite(x) => &mut x.r#dependencies,
        }
    }
}
impl DatabaseItem for Technology {
    fn validate(&mut self) {
        match self {
            Self::Component(x) => x.validate(),
            Self::Ship(x) => x.validate(),
            Self::Satellite(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "Technology"
    }
}
impl Technology {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Component(_) => TechnologyComponent::type_name(),
            Self::Ship(_) => TechnologyShip::type_name(),
            Self::Satellite(_) => TechnologySatellite::type_name(),
        }
    }
}
impl DatabaseItemWithId for Technology {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        *x.id()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/Ammunition.xml
pub type AmmunitionId = DatabaseItemId<Ammunition>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ammunition {
    pub r#id: AmmunitionId,
    pub r#body: BulletBody,
    pub r#controller: BulletController,
    pub r#triggers: Vec<BulletTrigger>,
    pub r#impact_type: BulletImpactType,
    pub r#effects: Vec<ImpactEffect>,
}
impl Ammunition {
    pub fn new(r#id: AmmunitionId) -> Self {
        Self {
            r#id,
            r#body: Default::default(),
            r#controller: Default::default(),
            r#triggers: Default::default(),
            r#impact_type: Default::default(),
            r#effects: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<AmmunitionId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<AmmunitionId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_body(mut self, r#body: impl Into<BulletBody>) -> Self {
        self.r#body = r#body.into();
        self
    }
    pub fn set_body(&mut self, r#body: impl Into<BulletBody>) -> &mut Self {
        self.r#body = r#body.into();
        self
    }
    pub fn with_controller(mut self, r#controller: impl Into<BulletController>) -> Self {
        self.r#controller = r#controller.into();
        self
    }
    pub fn set_controller(&mut self, r#controller: impl Into<BulletController>) -> &mut Self {
        self.r#controller = r#controller.into();
        self
    }
    pub fn with_triggers(mut self, r#triggers: impl Into<Vec<BulletTrigger>>) -> Self {
        self.r#triggers = r#triggers.into();
        self
    }
    pub fn set_triggers(&mut self, r#triggers: impl Into<Vec<BulletTrigger>>) -> &mut Self {
        self.r#triggers = r#triggers.into();
        self
    }
    pub fn with_impact_type(mut self, r#impact_type: impl Into<BulletImpactType>) -> Self {
        self.r#impact_type = r#impact_type.into();
        self
    }
    pub fn set_impact_type(&mut self, r#impact_type: impl Into<BulletImpactType>) -> &mut Self {
        self.r#impact_type = r#impact_type.into();
        self
    }
    pub fn with_effects(mut self, r#effects: impl Into<Vec<ImpactEffect>>) -> Self {
        self.r#effects = r#effects.into();
        self
    }
    pub fn set_effects(&mut self, r#effects: impl Into<Vec<ImpactEffect>>) -> &mut Self {
        self.r#effects = r#effects.into();
        self
    }
}
impl DatabaseItem for Ammunition {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "Ammunition"
    }
}
impl DatabaseItemWithId for Ammunition {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletPerfab.xml
pub type BulletPrefabId = DatabaseItemId<BulletPrefab>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulletPrefab {
    pub r#id: BulletPrefabId,
    pub r#shape: BulletShape,
    pub r#image: String,
    pub r#size: f32,
    pub r#margins: f32,
    pub r#deformation: f32,
    pub r#main_color: String,
    pub r#main_color_mode: ColorMode,
    pub r#second_color: String,
    pub r#second_color_mode: ColorMode,
}
impl BulletPrefab {
    pub fn new(r#id: BulletPrefabId) -> Self {
        Self {
            r#id,
            r#shape: Default::default(),
            r#image: Default::default(),
            r#size: Default::default(),
            r#margins: Default::default(),
            r#deformation: Default::default(),
            r#main_color: Default::default(),
            r#main_color_mode: Default::default(),
            r#second_color: Default::default(),
            r#second_color_mode: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<BulletPrefabId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<BulletPrefabId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_shape(mut self, r#shape: impl Into<BulletShape>) -> Self {
        self.r#shape = r#shape.into();
        self
    }
    pub fn set_shape(&mut self, r#shape: impl Into<BulletShape>) -> &mut Self {
        self.r#shape = r#shape.into();
        self
    }
    pub fn with_image(mut self, r#image: impl Into<String>) -> Self {
        self.r#image = r#image.into();
        self
    }
    pub fn set_image(&mut self, r#image: impl Into<String>) -> &mut Self {
        self.r#image = r#image.into();
        self
    }
    pub fn with_size(mut self, r#size: impl Into<f32>) -> Self {
        self.r#size = r#size.into();
        self
    }
    pub fn set_size(&mut self, r#size: impl Into<f32>) -> &mut Self {
        self.r#size = r#size.into();
        self
    }
    pub fn with_margins(mut self, r#margins: impl Into<f32>) -> Self {
        self.r#margins = r#margins.into();
        self
    }
    pub fn set_margins(&mut self, r#margins: impl Into<f32>) -> &mut Self {
        self.r#margins = r#margins.into();
        self
    }
    pub fn with_deformation(mut self, r#deformation: impl Into<f32>) -> Self {
        self.r#deformation = r#deformation.into();
        self
    }
    pub fn set_deformation(&mut self, r#deformation: impl Into<f32>) -> &mut Self {
        self.r#deformation = r#deformation.into();
        self
    }
    pub fn with_main_color(mut self, r#main_color: impl Into<String>) -> Self {
        self.r#main_color = r#main_color.into();
        self
    }
    pub fn set_main_color(&mut self, r#main_color: impl Into<String>) -> &mut Self {
        self.r#main_color = r#main_color.into();
        self
    }
    pub fn with_main_color_mode(mut self, r#main_color_mode: impl Into<ColorMode>) -> Self {
        self.r#main_color_mode = r#main_color_mode.into();
        self
    }
    pub fn set_main_color_mode(&mut self, r#main_color_mode: impl Into<ColorMode>) -> &mut Self {
        self.r#main_color_mode = r#main_color_mode.into();
        self
    }
    pub fn with_second_color(mut self, r#second_color: impl Into<String>) -> Self {
        self.r#second_color = r#second_color.into();
        self
    }
    pub fn set_second_color(&mut self, r#second_color: impl Into<String>) -> &mut Self {
        self.r#second_color = r#second_color.into();
        self
    }
    pub fn with_second_color_mode(mut self, r#second_color_mode: impl Into<ColorMode>) -> Self {
        self.r#second_color_mode = r#second_color_mode.into();
        self
    }
    pub fn set_second_color_mode(
        &mut self,
        r#second_color_mode: impl Into<ColorMode>,
    ) -> &mut Self {
        self.r#second_color_mode = r#second_color_mode.into();
        self
    }
}
impl DatabaseItem for BulletPrefab {
    fn validate(&mut self) {
        if self.r#size < (0.01f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                min = 0.01f32,
                "Field got truncated"
            );
            self.r#size = 0.01f32 as f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                field = "r#size",
                value = self.r#size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#size = 100f32 as f32;
        }
        if self.r#margins < (0f32 as f32) {
            tracing::warn!(
                field = "r#margins",
                value = self.r#margins,
                min = 0f32,
                "Field got truncated"
            );
            self.r#margins = 0f32 as f32;
        }
        if self.r#margins > (1f32 as f32) {
            tracing::warn!(
                field = "r#margins",
                value = self.r#margins,
                max = 1f32,
                "Field got truncated"
            );
            self.r#margins = 1f32 as f32;
        }
        if self.r#deformation < (-100f32 as f32) {
            tracing::warn!(
                field = "r#deformation",
                value = self.r#deformation,
                min = -100f32,
                "Field got truncated"
            );
            self.r#deformation = -100f32 as f32;
        }
        if self.r#deformation > (100f32 as f32) {
            tracing::warn!(
                field = "r#deformation",
                value = self.r#deformation,
                max = 100f32,
                "Field got truncated"
            );
            self.r#deformation = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "BulletPrefab"
    }
}
impl DatabaseItemWithId for BulletPrefab {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/VisualEffect.xml
pub type VisualEffectId = DatabaseItemId<VisualEffect>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct VisualEffect {
    pub r#id: VisualEffectId,
    pub r#elements: Vec<VisualEffectElement>,
}
impl VisualEffect {
    pub fn new(r#id: VisualEffectId) -> Self {
        Self {
            r#id,
            r#elements: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<VisualEffectId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<VisualEffectId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_elements(mut self, r#elements: impl Into<Vec<VisualEffectElement>>) -> Self {
        self.r#elements = r#elements.into();
        self
    }
    pub fn set_elements(&mut self, r#elements: impl Into<Vec<VisualEffectElement>>) -> &mut Self {
        self.r#elements = r#elements.into();
        self
    }
}
impl DatabaseItem for VisualEffect {
    fn validate(&mut self) {}
    fn type_name() -> &'static str {
        "VisualEffect"
    }
}
impl DatabaseItemWithId for VisualEffect {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/Weapon.xml
pub type WeaponId = DatabaseItemId<Weapon>;
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Weapon {
    pub r#id: WeaponId,
    pub r#weapon_class: WeaponClass,
    pub r#fire_rate: f32,
    pub r#spread: f32,
    pub r#magazine: i32,
    pub r#activation_type: ActivationType,
    pub r#shot_sound: String,
    pub r#charge_sound: String,
    pub r#shot_effect_prefab: String,
    pub r#visual_effect: Option<VisualEffectId>,
    pub r#effect_size: f32,
    pub r#control_button_icon: String,
}
impl Weapon {
    pub fn new(r#id: WeaponId) -> Self {
        Self {
            r#id,
            r#weapon_class: Default::default(),
            r#fire_rate: Default::default(),
            r#spread: Default::default(),
            r#magazine: Default::default(),
            r#activation_type: Default::default(),
            r#shot_sound: Default::default(),
            r#charge_sound: Default::default(),
            r#shot_effect_prefab: Default::default(),
            r#visual_effect: Default::default(),
            r#effect_size: Default::default(),
            r#control_button_icon: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: impl Into<WeaponId>) -> Self {
        self.r#id = r#id.into();
        self
    }
    pub fn set_id(&mut self, r#id: impl Into<WeaponId>) -> &mut Self {
        self.r#id = r#id.into();
        self
    }
    pub fn with_weapon_class(mut self, r#weapon_class: impl Into<WeaponClass>) -> Self {
        self.r#weapon_class = r#weapon_class.into();
        self
    }
    pub fn set_weapon_class(&mut self, r#weapon_class: impl Into<WeaponClass>) -> &mut Self {
        self.r#weapon_class = r#weapon_class.into();
        self
    }
    pub fn with_fire_rate(mut self, r#fire_rate: impl Into<f32>) -> Self {
        self.r#fire_rate = r#fire_rate.into();
        self
    }
    pub fn set_fire_rate(&mut self, r#fire_rate: impl Into<f32>) -> &mut Self {
        self.r#fire_rate = r#fire_rate.into();
        self
    }
    pub fn with_spread(mut self, r#spread: impl Into<f32>) -> Self {
        self.r#spread = r#spread.into();
        self
    }
    pub fn set_spread(&mut self, r#spread: impl Into<f32>) -> &mut Self {
        self.r#spread = r#spread.into();
        self
    }
    pub fn with_magazine(mut self, r#magazine: impl Into<i32>) -> Self {
        self.r#magazine = r#magazine.into();
        self
    }
    pub fn set_magazine(&mut self, r#magazine: impl Into<i32>) -> &mut Self {
        self.r#magazine = r#magazine.into();
        self
    }
    pub fn with_activation_type(mut self, r#activation_type: impl Into<ActivationType>) -> Self {
        self.r#activation_type = r#activation_type.into();
        self
    }
    pub fn set_activation_type(
        &mut self,
        r#activation_type: impl Into<ActivationType>,
    ) -> &mut Self {
        self.r#activation_type = r#activation_type.into();
        self
    }
    pub fn with_shot_sound(mut self, r#shot_sound: impl Into<String>) -> Self {
        self.r#shot_sound = r#shot_sound.into();
        self
    }
    pub fn set_shot_sound(&mut self, r#shot_sound: impl Into<String>) -> &mut Self {
        self.r#shot_sound = r#shot_sound.into();
        self
    }
    pub fn with_charge_sound(mut self, r#charge_sound: impl Into<String>) -> Self {
        self.r#charge_sound = r#charge_sound.into();
        self
    }
    pub fn set_charge_sound(&mut self, r#charge_sound: impl Into<String>) -> &mut Self {
        self.r#charge_sound = r#charge_sound.into();
        self
    }
    pub fn with_shot_effect_prefab(mut self, r#shot_effect_prefab: impl Into<String>) -> Self {
        self.r#shot_effect_prefab = r#shot_effect_prefab.into();
        self
    }
    pub fn set_shot_effect_prefab(&mut self, r#shot_effect_prefab: impl Into<String>) -> &mut Self {
        self.r#shot_effect_prefab = r#shot_effect_prefab.into();
        self
    }
    pub fn with_visual_effect(
        mut self,
        r#visual_effect: impl Into<Option<VisualEffectId>>,
    ) -> Self {
        self.r#visual_effect = r#visual_effect.into();
        self
    }
    pub fn set_visual_effect(
        &mut self,
        r#visual_effect: impl Into<Option<VisualEffectId>>,
    ) -> &mut Self {
        self.r#visual_effect = r#visual_effect.into();
        self
    }
    pub fn with_effect_size(mut self, r#effect_size: impl Into<f32>) -> Self {
        self.r#effect_size = r#effect_size.into();
        self
    }
    pub fn set_effect_size(&mut self, r#effect_size: impl Into<f32>) -> &mut Self {
        self.r#effect_size = r#effect_size.into();
        self
    }
    pub fn with_control_button_icon(mut self, r#control_button_icon: impl Into<String>) -> Self {
        self.r#control_button_icon = r#control_button_icon.into();
        self
    }
    pub fn set_control_button_icon(
        &mut self,
        r#control_button_icon: impl Into<String>,
    ) -> &mut Self {
        self.r#control_button_icon = r#control_button_icon.into();
        self
    }
}
impl DatabaseItem for Weapon {
    fn validate(&mut self) {
        if self.r#fire_rate < (0f32 as f32) {
            tracing::warn!(
                field = "r#fire_rate",
                value = self.r#fire_rate,
                min = 0f32,
                "Field got truncated"
            );
            self.r#fire_rate = 0f32 as f32;
        }
        if self.r#fire_rate > (100f32 as f32) {
            tracing::warn!(
                field = "r#fire_rate",
                value = self.r#fire_rate,
                max = 100f32,
                "Field got truncated"
            );
            self.r#fire_rate = 100f32 as f32;
        }
        if self.r#spread < (0f32 as f32) {
            tracing::warn!(
                field = "r#spread",
                value = self.r#spread,
                min = 0f32,
                "Field got truncated"
            );
            self.r#spread = 0f32 as f32;
        }
        if self.r#spread > (360f32 as f32) {
            tracing::warn!(
                field = "r#spread",
                value = self.r#spread,
                max = 360f32,
                "Field got truncated"
            );
            self.r#spread = 360f32 as f32;
        }
        if self.r#magazine < (0f32 as i32) {
            tracing::warn!(
                field = "r#magazine",
                value = self.r#magazine,
                min = 0f32,
                "Field got truncated"
            );
            self.r#magazine = 0f32 as i32;
        }
        if self.r#magazine > (1000000000f32 as i32) {
            tracing::warn!(
                field = "r#magazine",
                value = self.r#magazine,
                max = 1000000000f32,
                "Field got truncated"
            );
            self.r#magazine = 1000000000f32 as i32;
        }
        if self.r#effect_size < (0f32 as f32) {
            tracing::warn!(
                field = "r#effect_size",
                value = self.r#effect_size,
                min = 0f32,
                "Field got truncated"
            );
            self.r#effect_size = 0f32 as f32;
        }
        if self.r#effect_size > (100f32 as f32) {
            tracing::warn!(
                field = "r#effect_size",
                value = self.r#effect_size,
                max = 100f32,
                "Field got truncated"
            );
            self.r#effect_size = 100f32 as f32;
        }
    }
    fn type_name() -> &'static str {
        "Weapon"
    }
}
impl DatabaseItemWithId for Weapon {
    fn id(&self) -> DatabaseItemId<Self> {
        let x = self;
        x.id
    }
}

// Core Database Item
#[derive(Debug, Clone)]
pub enum Item {
    Component(Component),
    Device(Device),
    Weapon(Weapon),
    AmmunitionObsolete(AmmunitionObsolete),
    DroneBay(DroneBay),
    Ship(Ship),
    Satellite(Satellite),
    ShipBuild(ShipBuild),
    SatelliteBuild(SatelliteBuild),
    Technology(Technology),
    ComponentStats(ComponentStats),
    ComponentMod(ComponentMod),
    Skill(Skill),
    Faction(Faction),
    Quest(Quest),
    Loot(Loot),
    Fleet(Fleet),
    Character(Character),
    QuestItem(QuestItem),
    Ammunition(Ammunition),
    VisualEffect(VisualEffect),
    BulletPrefab(BulletPrefab),
    BehaviorTree(BehaviorTree),
    GameObjectPrefab(GameObjectPrefab),
    CombatRules(CombatRules),
    ShipSettings(ShipSettings),
    GalaxySettings(GalaxySettings),
    DatabaseSettings(DatabaseSettings),
    ExplorationSettings(ExplorationSettings),
    FrontierSettings(FrontierSettings),
    ShipModSettings(ShipModSettings),
    SpecialEventSettings(SpecialEventSettings),
    SkillSettings(SkillSettings),
    DebugSettings(DebugSettings),
    CombatSettings(CombatSettings),
    UiSettings(UiSettings),
    FactionsSettings(FactionsSettings),
    MusicPlaylist(MusicPlaylist),
}
impl From<Component> for Item {
    fn from(item: Component) -> Self {
        Self::Component(item)
    }
}
impl Component {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn component(r#id: ComponentId, r#component_stats_id: ComponentStatsId) -> Component {
        Component::new(r#id, r#component_stats_id)
    }
}
impl From<Device> for Item {
    fn from(item: Device) -> Self {
        Self::Device(item)
    }
}
impl Device {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn device(r#id: DeviceId) -> Device {
        Device::new(r#id)
    }
}
impl From<Weapon> for Item {
    fn from(item: Weapon) -> Self {
        Self::Weapon(item)
    }
}
impl Weapon {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn weapon(r#id: WeaponId) -> Weapon {
        Weapon::new(r#id)
    }
}
impl From<AmmunitionObsolete> for Item {
    fn from(item: AmmunitionObsolete) -> Self {
        Self::AmmunitionObsolete(item)
    }
}
impl AmmunitionObsolete {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn ammunition_obsolete(r#id: AmmunitionObsoleteId) -> AmmunitionObsolete {
        AmmunitionObsolete::new(r#id)
    }
}
impl From<DroneBay> for Item {
    fn from(item: DroneBay) -> Self {
        Self::DroneBay(item)
    }
}
impl DroneBay {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn drone_bay(r#id: DroneBayId) -> DroneBay {
        DroneBay::new(r#id)
    }
}
impl From<Ship> for Item {
    fn from(item: Ship) -> Self {
        Self::Ship(item)
    }
}
impl Ship {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn ship(r#id: ShipId) -> Ship {
        Ship::new(r#id)
    }
}
impl From<Satellite> for Item {
    fn from(item: Satellite) -> Self {
        Self::Satellite(item)
    }
}
impl Satellite {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn satellite(r#id: SatelliteId) -> Satellite {
        Satellite::new(r#id)
    }
}
impl From<ShipBuild> for Item {
    fn from(item: ShipBuild) -> Self {
        Self::ShipBuild(item)
    }
}
impl ShipBuild {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn ship_build(r#id: ShipBuildId, r#ship_id: ShipId) -> ShipBuild {
        ShipBuild::new(r#id, r#ship_id)
    }
}
impl From<SatelliteBuild> for Item {
    fn from(item: SatelliteBuild) -> Self {
        Self::SatelliteBuild(item)
    }
}
impl SatelliteBuild {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn satellite_build(r#id: SatelliteBuildId, r#satellite_id: SatelliteId) -> SatelliteBuild {
        SatelliteBuild::new(r#id, r#satellite_id)
    }
}
impl From<Technology> for Item {
    fn from(item: Technology) -> Self {
        Self::Technology(item)
    }
}
impl Technology {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl From<ComponentStats> for Item {
    fn from(item: ComponentStats) -> Self {
        Self::ComponentStats(item)
    }
}
impl ComponentStats {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn component_stats(r#id: ComponentStatsId) -> ComponentStats {
        ComponentStats::new(r#id)
    }
}
impl From<ComponentMod> for Item {
    fn from(item: ComponentMod) -> Self {
        Self::ComponentMod(item)
    }
}
impl ComponentMod {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn component_mod(r#id: ComponentModId) -> ComponentMod {
        ComponentMod::new(r#id)
    }
}
impl From<Skill> for Item {
    fn from(item: Skill) -> Self {
        Self::Skill(item)
    }
}
impl Skill {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn skill(r#id: SkillId) -> Skill {
        Skill::new(r#id)
    }
}
impl From<Faction> for Item {
    fn from(item: Faction) -> Self {
        Self::Faction(item)
    }
}
impl Faction {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn faction(r#id: FactionId) -> Faction {
        Faction::new(r#id)
    }
}
impl From<Quest> for Item {
    fn from(item: Quest) -> Self {
        Self::Quest(item)
    }
}
impl Quest {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn quest(r#id: QuestId) -> Quest {
        Quest::new(r#id)
    }
}
impl From<Loot> for Item {
    fn from(item: Loot) -> Self {
        Self::Loot(item)
    }
}
impl Loot {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn loot(r#id: LootId) -> Loot {
        Loot::new(r#id)
    }
}
impl From<Fleet> for Item {
    fn from(item: Fleet) -> Self {
        Self::Fleet(item)
    }
}
impl Fleet {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn fleet(r#id: FleetId) -> Fleet {
        Fleet::new(r#id)
    }
}
impl From<Character> for Item {
    fn from(item: Character) -> Self {
        Self::Character(item)
    }
}
impl Character {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn character(r#id: CharacterId) -> Character {
        Character::new(r#id)
    }
}
impl From<QuestItem> for Item {
    fn from(item: QuestItem) -> Self {
        Self::QuestItem(item)
    }
}
impl QuestItem {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn quest_item(r#id: QuestItemId) -> QuestItem {
        QuestItem::new(r#id)
    }
}
impl From<Ammunition> for Item {
    fn from(item: Ammunition) -> Self {
        Self::Ammunition(item)
    }
}
impl Ammunition {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn ammunition(r#id: AmmunitionId) -> Ammunition {
        Ammunition::new(r#id)
    }
}
impl From<VisualEffect> for Item {
    fn from(item: VisualEffect) -> Self {
        Self::VisualEffect(item)
    }
}
impl VisualEffect {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn visual_effect(r#id: VisualEffectId) -> VisualEffect {
        VisualEffect::new(r#id)
    }
}
impl From<BulletPrefab> for Item {
    fn from(item: BulletPrefab) -> Self {
        Self::BulletPrefab(item)
    }
}
impl BulletPrefab {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn bullet_prefab(r#id: BulletPrefabId) -> BulletPrefab {
        BulletPrefab::new(r#id)
    }
}
impl From<BehaviorTree> for Item {
    fn from(item: BehaviorTree) -> Self {
        Self::BehaviorTree(item)
    }
}
impl BehaviorTree {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn behavior_tree(r#id: BehaviorTreeId) -> BehaviorTree {
        BehaviorTree::new(r#id)
    }
}
impl From<GameObjectPrefab> for Item {
    fn from(item: GameObjectPrefab) -> Self {
        Self::GameObjectPrefab(item)
    }
}
impl GameObjectPrefab {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl From<CombatRules> for Item {
    fn from(item: CombatRules) -> Self {
        Self::CombatRules(item)
    }
}
impl CombatRules {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn combat_rules(r#id: CombatRulesId) -> CombatRules {
        CombatRules::new(r#id)
    }
}
impl From<ShipSettings> for Item {
    fn from(item: ShipSettings) -> Self {
        Self::ShipSettings(item)
    }
}
impl ShipSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn ship_settings() -> ShipSettings {
        ShipSettings::new()
    }
}
impl From<GalaxySettings> for Item {
    fn from(item: GalaxySettings) -> Self {
        Self::GalaxySettings(item)
    }
}
impl GalaxySettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn galaxy_settings() -> GalaxySettings {
        GalaxySettings::new()
    }
}
impl From<DatabaseSettings> for Item {
    fn from(item: DatabaseSettings) -> Self {
        Self::DatabaseSettings(item)
    }
}
impl DatabaseSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn database_settings() -> DatabaseSettings {
        DatabaseSettings::new()
    }
}
impl From<ExplorationSettings> for Item {
    fn from(item: ExplorationSettings) -> Self {
        Self::ExplorationSettings(item)
    }
}
impl ExplorationSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn exploration_settings() -> ExplorationSettings {
        ExplorationSettings::new()
    }
}
impl From<FrontierSettings> for Item {
    fn from(item: FrontierSettings) -> Self {
        Self::FrontierSettings(item)
    }
}
impl FrontierSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn frontier_settings() -> FrontierSettings {
        FrontierSettings::new()
    }
}
impl From<ShipModSettings> for Item {
    fn from(item: ShipModSettings) -> Self {
        Self::ShipModSettings(item)
    }
}
impl ShipModSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn ship_mod_settings() -> ShipModSettings {
        ShipModSettings::new()
    }
}
impl From<SpecialEventSettings> for Item {
    fn from(item: SpecialEventSettings) -> Self {
        Self::SpecialEventSettings(item)
    }
}
impl SpecialEventSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn special_event_settings() -> SpecialEventSettings {
        SpecialEventSettings::new()
    }
}
impl From<SkillSettings> for Item {
    fn from(item: SkillSettings) -> Self {
        Self::SkillSettings(item)
    }
}
impl SkillSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn skill_settings() -> SkillSettings {
        SkillSettings::new()
    }
}
impl From<DebugSettings> for Item {
    fn from(item: DebugSettings) -> Self {
        Self::DebugSettings(item)
    }
}
impl DebugSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn debug_settings() -> DebugSettings {
        DebugSettings::new()
    }
}
impl From<CombatSettings> for Item {
    fn from(item: CombatSettings) -> Self {
        Self::CombatSettings(item)
    }
}
impl CombatSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn combat_settings() -> CombatSettings {
        CombatSettings::new()
    }
}
impl From<UiSettings> for Item {
    fn from(item: UiSettings) -> Self {
        Self::UiSettings(item)
    }
}
impl UiSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn ui_settings() -> UiSettings {
        UiSettings::new()
    }
}
impl From<FactionsSettings> for Item {
    fn from(item: FactionsSettings) -> Self {
        Self::FactionsSettings(item)
    }
}
impl FactionsSettings {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn factions_settings() -> FactionsSettings {
        FactionsSettings::new()
    }
}
impl From<MusicPlaylist> for Item {
    fn from(item: MusicPlaylist) -> Self {
        Self::MusicPlaylist(item)
    }
}
impl MusicPlaylist {
    pub fn wrap(self) -> Item {
        self.into()
    }
}
impl Item {
    pub fn music_playlist() -> MusicPlaylist {
        MusicPlaylist::new()
    }
}
impl serde::Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(serde::Serialize)]
        #[serde(rename = "Item")]
        struct AdjTagged<T> {
            #[serde(rename = "ItemType")]
            t: ItemType,
            #[serde(flatten)]
            c: T,
        }
        match self {
            Self::Component(x) => AdjTagged {
                t: ItemType::Component,
                c: x,
            }
            .serialize(serializer),
            Self::Device(x) => AdjTagged {
                t: ItemType::Device,
                c: x,
            }
            .serialize(serializer),
            Self::Weapon(x) => AdjTagged {
                t: ItemType::Weapon,
                c: x,
            }
            .serialize(serializer),
            Self::AmmunitionObsolete(x) => AdjTagged {
                t: ItemType::AmmunitionObsolete,
                c: x,
            }
            .serialize(serializer),
            Self::DroneBay(x) => AdjTagged {
                t: ItemType::DroneBay,
                c: x,
            }
            .serialize(serializer),
            Self::Ship(x) => AdjTagged {
                t: ItemType::Ship,
                c: x,
            }
            .serialize(serializer),
            Self::Satellite(x) => AdjTagged {
                t: ItemType::Satellite,
                c: x,
            }
            .serialize(serializer),
            Self::ShipBuild(x) => AdjTagged {
                t: ItemType::ShipBuild,
                c: x,
            }
            .serialize(serializer),
            Self::SatelliteBuild(x) => AdjTagged {
                t: ItemType::SatelliteBuild,
                c: x,
            }
            .serialize(serializer),
            Self::Technology(x) => AdjTagged {
                t: ItemType::Technology,
                c: x,
            }
            .serialize(serializer),
            Self::ComponentStats(x) => AdjTagged {
                t: ItemType::ComponentStats,
                c: x,
            }
            .serialize(serializer),
            Self::ComponentMod(x) => AdjTagged {
                t: ItemType::ComponentMod,
                c: x,
            }
            .serialize(serializer),
            Self::Skill(x) => AdjTagged {
                t: ItemType::Skill,
                c: x,
            }
            .serialize(serializer),
            Self::Faction(x) => AdjTagged {
                t: ItemType::Faction,
                c: x,
            }
            .serialize(serializer),
            Self::Quest(x) => AdjTagged {
                t: ItemType::Quest,
                c: x,
            }
            .serialize(serializer),
            Self::Loot(x) => AdjTagged {
                t: ItemType::Loot,
                c: x,
            }
            .serialize(serializer),
            Self::Fleet(x) => AdjTagged {
                t: ItemType::Fleet,
                c: x,
            }
            .serialize(serializer),
            Self::Character(x) => AdjTagged {
                t: ItemType::Character,
                c: x,
            }
            .serialize(serializer),
            Self::QuestItem(x) => AdjTagged {
                t: ItemType::QuestItem,
                c: x,
            }
            .serialize(serializer),
            Self::Ammunition(x) => AdjTagged {
                t: ItemType::Ammunition,
                c: x,
            }
            .serialize(serializer),
            Self::VisualEffect(x) => AdjTagged {
                t: ItemType::VisualEffect,
                c: x,
            }
            .serialize(serializer),
            Self::BulletPrefab(x) => AdjTagged {
                t: ItemType::BulletPrefab,
                c: x,
            }
            .serialize(serializer),
            Self::BehaviorTree(x) => AdjTagged {
                t: ItemType::BehaviorTree,
                c: x,
            }
            .serialize(serializer),
            Self::GameObjectPrefab(x) => AdjTagged {
                t: ItemType::GameObjectPrefab,
                c: x,
            }
            .serialize(serializer),
            Self::CombatRules(x) => AdjTagged {
                t: ItemType::CombatRules,
                c: x,
            }
            .serialize(serializer),
            Self::ShipSettings(x) => AdjTagged {
                t: ItemType::ShipSettings,
                c: x,
            }
            .serialize(serializer),
            Self::GalaxySettings(x) => AdjTagged {
                t: ItemType::GalaxySettings,
                c: x,
            }
            .serialize(serializer),
            Self::DatabaseSettings(x) => AdjTagged {
                t: ItemType::DatabaseSettings,
                c: x,
            }
            .serialize(serializer),
            Self::ExplorationSettings(x) => AdjTagged {
                t: ItemType::ExplorationSettings,
                c: x,
            }
            .serialize(serializer),
            Self::FrontierSettings(x) => AdjTagged {
                t: ItemType::FrontierSettings,
                c: x,
            }
            .serialize(serializer),
            Self::ShipModSettings(x) => AdjTagged {
                t: ItemType::ShipModSettings,
                c: x,
            }
            .serialize(serializer),
            Self::SpecialEventSettings(x) => AdjTagged {
                t: ItemType::SpecialEventSettings,
                c: x,
            }
            .serialize(serializer),
            Self::SkillSettings(x) => AdjTagged {
                t: ItemType::SkillSettings,
                c: x,
            }
            .serialize(serializer),
            Self::DebugSettings(x) => AdjTagged {
                t: ItemType::DebugSettings,
                c: x,
            }
            .serialize(serializer),
            Self::CombatSettings(x) => AdjTagged {
                t: ItemType::CombatSettings,
                c: x,
            }
            .serialize(serializer),
            Self::UiSettings(x) => AdjTagged {
                t: ItemType::UiSettings,
                c: x,
            }
            .serialize(serializer),
            Self::FactionsSettings(x) => AdjTagged {
                t: ItemType::FactionsSettings,
                c: x,
            }
            .serialize(serializer),
            Self::MusicPlaylist(x) => AdjTagged {
                t: ItemType::MusicPlaylist,
                c: x,
            }
            .serialize(serializer),
        }
    }
}
impl DatabaseItem for Item {
    fn validate(&mut self) {
        match self {
            Self::Component(x) => x.validate(),
            Self::Device(x) => x.validate(),
            Self::Weapon(x) => x.validate(),
            Self::AmmunitionObsolete(x) => x.validate(),
            Self::DroneBay(x) => x.validate(),
            Self::Ship(x) => x.validate(),
            Self::Satellite(x) => x.validate(),
            Self::ShipBuild(x) => x.validate(),
            Self::SatelliteBuild(x) => x.validate(),
            Self::Technology(x) => x.validate(),
            Self::ComponentStats(x) => x.validate(),
            Self::ComponentMod(x) => x.validate(),
            Self::Skill(x) => x.validate(),
            Self::Faction(x) => x.validate(),
            Self::Quest(x) => x.validate(),
            Self::Loot(x) => x.validate(),
            Self::Fleet(x) => x.validate(),
            Self::Character(x) => x.validate(),
            Self::QuestItem(x) => x.validate(),
            Self::Ammunition(x) => x.validate(),
            Self::VisualEffect(x) => x.validate(),
            Self::BulletPrefab(x) => x.validate(),
            Self::BehaviorTree(x) => x.validate(),
            Self::GameObjectPrefab(x) => x.validate(),
            Self::CombatRules(x) => x.validate(),
            Self::ShipSettings(x) => x.validate(),
            Self::GalaxySettings(x) => x.validate(),
            Self::DatabaseSettings(x) => x.validate(),
            Self::ExplorationSettings(x) => x.validate(),
            Self::FrontierSettings(x) => x.validate(),
            Self::ShipModSettings(x) => x.validate(),
            Self::SpecialEventSettings(x) => x.validate(),
            Self::SkillSettings(x) => x.validate(),
            Self::DebugSettings(x) => x.validate(),
            Self::CombatSettings(x) => x.validate(),
            Self::UiSettings(x) => x.validate(),
            Self::FactionsSettings(x) => x.validate(),
            Self::MusicPlaylist(x) => x.validate(),
        }
    }
    fn type_name() -> &'static str {
        "Item"
    }
}
impl Item {
    pub fn inner_type_name(&self) -> &'static str {
        match self {
            Self::Component(_) => Component::type_name(),
            Self::Device(_) => Device::type_name(),
            Self::Weapon(_) => Weapon::type_name(),
            Self::AmmunitionObsolete(_) => AmmunitionObsolete::type_name(),
            Self::DroneBay(_) => DroneBay::type_name(),
            Self::Ship(_) => Ship::type_name(),
            Self::Satellite(_) => Satellite::type_name(),
            Self::ShipBuild(_) => ShipBuild::type_name(),
            Self::SatelliteBuild(_) => SatelliteBuild::type_name(),
            Self::Technology(_) => Technology::type_name(),
            Self::ComponentStats(_) => ComponentStats::type_name(),
            Self::ComponentMod(_) => ComponentMod::type_name(),
            Self::Skill(_) => Skill::type_name(),
            Self::Faction(_) => Faction::type_name(),
            Self::Quest(_) => Quest::type_name(),
            Self::Loot(_) => Loot::type_name(),
            Self::Fleet(_) => Fleet::type_name(),
            Self::Character(_) => Character::type_name(),
            Self::QuestItem(_) => QuestItem::type_name(),
            Self::Ammunition(_) => Ammunition::type_name(),
            Self::VisualEffect(_) => VisualEffect::type_name(),
            Self::BulletPrefab(_) => BulletPrefab::type_name(),
            Self::BehaviorTree(_) => BehaviorTree::type_name(),
            Self::GameObjectPrefab(_) => GameObjectPrefab::type_name(),
            Self::CombatRules(_) => CombatRules::type_name(),
            Self::ShipSettings(_) => ShipSettings::type_name(),
            Self::GalaxySettings(_) => GalaxySettings::type_name(),
            Self::DatabaseSettings(_) => DatabaseSettings::type_name(),
            Self::ExplorationSettings(_) => ExplorationSettings::type_name(),
            Self::FrontierSettings(_) => FrontierSettings::type_name(),
            Self::ShipModSettings(_) => ShipModSettings::type_name(),
            Self::SpecialEventSettings(_) => SpecialEventSettings::type_name(),
            Self::SkillSettings(_) => SkillSettings::type_name(),
            Self::DebugSettings(_) => DebugSettings::type_name(),
            Self::CombatSettings(_) => CombatSettings::type_name(),
            Self::UiSettings(_) => UiSettings::type_name(),
            Self::FactionsSettings(_) => FactionsSettings::type_name(),
            Self::MusicPlaylist(_) => MusicPlaylist::type_name(),
        }
    }
}
impl Item {
    /// Fetches untyped ID of the inner item, or None if content is a setting
    pub fn id(&self) -> Option<i32> {
        match self {
            Self::Component(x) => Some((x.id).0),
            Self::Device(x) => Some((x.id).0),
            Self::Weapon(x) => Some((x.id).0),
            Self::AmmunitionObsolete(x) => Some((x.id).0),
            Self::DroneBay(x) => Some((x.id).0),
            Self::Ship(x) => Some((x.id).0),
            Self::Satellite(x) => Some((x.id).0),
            Self::ShipBuild(x) => Some((x.id).0),
            Self::SatelliteBuild(x) => Some((x.id).0),
            Self::Technology(x) => Some(x.id().0),
            Self::ComponentStats(x) => Some((x.id).0),
            Self::ComponentMod(x) => Some((x.id).0),
            Self::Skill(x) => Some((x.id).0),
            Self::Faction(x) => Some((x.id).0),
            Self::Quest(x) => Some((x.id).0),
            Self::Loot(x) => Some((x.id).0),
            Self::Fleet(x) => Some((x.id).0),
            Self::Character(x) => Some((x.id).0),
            Self::QuestItem(x) => Some((x.id).0),
            Self::Ammunition(x) => Some((x.id).0),
            Self::VisualEffect(x) => Some((x.id).0),
            Self::BulletPrefab(x) => Some((x.id).0),
            Self::BehaviorTree(x) => Some((x.id).0),
            Self::GameObjectPrefab(x) => Some(x.id().0),
            Self::CombatRules(x) => Some((x.id).0),
            Self::ShipSettings(_) => None,
            Self::GalaxySettings(_) => None,
            Self::DatabaseSettings(_) => None,
            Self::ExplorationSettings(_) => None,
            Self::FrontierSettings(_) => None,
            Self::ShipModSettings(_) => None,
            Self::SpecialEventSettings(_) => None,
            Self::SkillSettings(_) => None,
            Self::DebugSettings(_) => None,
            Self::CombatSettings(_) => None,
            Self::UiSettings(_) => None,
            Self::FactionsSettings(_) => None,
            Self::MusicPlaylist(_) => None,
        }
    }
}
#[macro_export]
macro_rules! apply_items {
    ($macro_name:ident) => {
        $macro_name! { component(r#id : (DatabaseItemId < Component >),
        r#component_stats_id : (DatabaseItemId < ComponentStats >),) -> Component,
        device(r#id : (DatabaseItemId < Device >),) -> Device, weapon(r#id :
        (DatabaseItemId < Weapon >),) -> Weapon, ammunition_obsolete(r#id :
        (DatabaseItemId < AmmunitionObsolete >),) -> AmmunitionObsolete, drone_bay(r#id :
        (DatabaseItemId < DroneBay >),) -> DroneBay, ship(r#id : (DatabaseItemId < Ship
        >),) -> Ship, satellite(r#id : (DatabaseItemId < Satellite >),) -> Satellite,
        ship_build(r#id : (DatabaseItemId < ShipBuild >), r#ship_id : (DatabaseItemId <
        Ship >),) -> ShipBuild, satellite_build(r#id : (DatabaseItemId < SatelliteBuild
        >), r#satellite_id : (DatabaseItemId < Satellite >),) -> SatelliteBuild,
        component_stats(r#id : (DatabaseItemId < ComponentStats >),) -> ComponentStats,
        component_mod(r#id : (DatabaseItemId < ComponentMod >),) -> ComponentMod,
        skill(r#id : (DatabaseItemId < Skill >),) -> Skill, faction(r#id :
        (DatabaseItemId < Faction >),) -> Faction, quest(r#id : (DatabaseItemId < Quest
        >),) -> Quest, loot(r#id : (DatabaseItemId < Loot >),) -> Loot, fleet(r#id :
        (DatabaseItemId < Fleet >),) -> Fleet, character(r#id : (DatabaseItemId <
        Character >),) -> Character, quest_item(r#id : (DatabaseItemId < QuestItem >),)
        -> QuestItem, ammunition(r#id : (DatabaseItemId < Ammunition >),) -> Ammunition,
        visual_effect(r#id : (DatabaseItemId < VisualEffect >),) -> VisualEffect,
        bullet_prefab(r#id : (DatabaseItemId < BulletPrefab >),) -> BulletPrefab,
        behavior_tree(r#id : (DatabaseItemId < BehaviorTree >),) -> BehaviorTree,
        combat_rules(r#id : (DatabaseItemId < CombatRules >),) -> CombatRules,
        ship_settings() -> ShipSettings, galaxy_settings() -> GalaxySettings,
        database_settings() -> DatabaseSettings, exploration_settings() ->
        ExplorationSettings, frontier_settings() -> FrontierSettings, ship_mod_settings()
        -> ShipModSettings, special_event_settings() -> SpecialEventSettings,
        skill_settings() -> SkillSettings, debug_settings() -> DebugSettings,
        combat_settings() -> CombatSettings, ui_settings() -> UiSettings,
        factions_settings() -> FactionsSettings, music_playlist() -> MusicPlaylist, }
    };
}
