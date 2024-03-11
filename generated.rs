
// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/versions.xml
pub trait DatabaseItem: serde::Serialize {
    fn validate(&mut self);
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Enums/ActivationType.xml
#[repr(i32)]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum ActivationType {
    #[default]
    None,
    Manual,
    Mixed,
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
    ///0
    Easy = 0isize,
    ///1
    Medium = 1isize,
    ///2
    Hard = 2isize,
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
    ///0
    All = 0isize,
    ///1
    Repair = 1isize,
    ///2
    Damage = 2isize,
    ///3
    CaptureDrone = 3isize,
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
    ///0
    Success = 0isize,
    ///9
    Failure = 9isize,
    ///1
    SubTree = 1isize,
    ///2
    Selector = 2isize,
    ///3
    Sequence = 3isize,
    ///4
    Parallel = 4isize,
    ///5
    RandomSelector = 5isize,
    ///6
    Invertor = 6isize,
    ///7
    Cooldown = 7isize,
    ///8
    Execute = 8isize,
    ///10
    ParallelSequence = 10isize,
    ///11
    PreserveTarget = 11isize,
    ///12
    IfThenElse = 12isize,
    ///50
    HasEnoughEnergy = 50isize,
    ///51
    IsLowOnHp = 51isize,
    ///52
    IsControledByPlayer = 52isize,
    ///53
    HasIncomingThreat = 53isize,
    ///54
    HasAdditionalTargets = 54isize,
    ///55
    IsFasterThanTarget = 55isize,
    ///56
    HasMainTarget = 56isize,
    ///57
    MainTargetIsAlly = 57isize,
    ///58
    MainTargetIsEnemy = 58isize,
    ///59
    MainTargetLowHp = 59isize,
    ///60
    MainTargetWithinAttackRange = 60isize,
    ///61
    HasMothership = 61isize,
    ///62
    TargetDistance = 62isize,
    ///63
    HasLongerAttackRange = 63isize,
    ///100
    FindEnemy = 100isize,
    ///101
    MoveToAttackRange = 101isize,
    ///102
    AttackMainTarget = 102isize,
    ///103
    SelectWeapon = 103isize,
    ///104
    SpawnDrones = 104isize,
    ///105
    Ram = 105isize,
    ///106
    DetonateShip = 106isize,
    ///107
    Vanish = 107isize,
    ///108
    MaintainAttackRange = 108isize,
    ///109
    Wait = 109isize,
    ///110
    LookAtTarget = 110isize,
    ///111
    LookForAdditionalTargets = 111isize,
    ///112
    LookForThreats = 112isize,
    ///113
    MatchVelocityWithTarget = 113isize,
    ///114
    ActivateDevice = 114isize,
    ///115
    RechargeEnergy = 115isize,
    ///116
    SustainAim = 116isize,
    ///117
    ChargeWeapons = 117isize,
    ///118
    Chase = 118isize,
    ///119
    AvoidThreats = 119isize,
    ///120
    SlowDown = 120isize,
    ///121
    UseRecoil = 121isize,
    ///122
    DefendWithFronalShield = 122isize,
    ///123
    TrackControllableAmmo = 123isize,
    ///124
    KeepDistance = 124isize,
    ///125
    ForgetMainTarget = 125isize,
    ///126
    EscapeTargetAttackRadius = 126isize,
    ///127
    AttackAdditionalTargets = 127isize,
    ///128
    TargetAllyStarbase = 128isize,
    ///129
    TargetEnemyStarbase = 129isize,
    ///130
    BypassObstacles = 130isize,
    ///131
    AttackTurretTargets = 131isize,
    ///150
    EnginePropulsionForce = 150isize,
    ///200
    MotherShipRetreated = 200isize,
    ///201
    MotherShipDestroyed = 201isize,
    ///202
    FlyAroundMothership = 202isize,
    ///203
    GoBerserk = 203isize,
    ///204
    TargetMothership = 204isize,
    ///205
    MothershipLowHp = 205isize,
    ///206
    MothershipDistanceExceeded = 206isize,
    ///207
    MakeTargetMothership = 207isize,
    ///300
    ShowMessage = 300isize,
    ///301
    DebugLog = 301isize,
    ///302
    SetValue = 302isize,
    ///303
    GetValue = 303isize,
    ///304
    SendMessage = 304isize,
    ///305
    MessageReceived = 305isize,
    ///306
    TargetMessageSender = 306isize,
    ///307
    SaveTarget = 307isize,
    ///308
    LoadTarget = 308isize,
    ///309
    HasSavedTarget = 309isize,
    ///310
    ForgetSavedTarget = 310isize,
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
    ///0
    Empty = 0isize,
    ///1
    Any = 1isize,
    ///2
    All = 2isize,
    ///3
    None = 3isize,
    ///5
    AiLevel = 5isize,
    ///6
    MinAiLevel = 6isize,
    ///7
    SizeClass = 7isize,
    ///10
    HasDevice = 10isize,
    ///12
    HasDrones = 12isize,
    ///11
    HasAnyWeapon = 11isize,
    ///13
    CanRepairAllies = 13isize,
    ///14
    HasHighRecoilWeapon = 14isize,
    ///15
    HasChargeableWeapon = 15isize,
    ///16
    HasRemotelyControlledWeapon = 16isize,
    ///17
    HasLongRangeWeapon = 17isize,
    ///18
    HasEngine = 18isize,
    ///50
    IsDrone = 50isize,
    ///100
    HasKineticResistance = 100isize,
    ///101
    HasHighManeuverability = 101isize,
    ///102
    HasHighRammingDamage = 102isize,
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
    ///0
    UntilSucceeds = 0isize,
    ///1
    UntilFails = 1isize,
    ///2
    UntilFinishes = 2isize,
    ///3
    Infinitely = 3isize,
    ///4
    OneTime = 4isize,
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
    ///0
    None = 0isize,
    ///1
    Common = 1isize,
    ///2
    Rare = 2isize,
    ///3
    Special = 3isize,
    ///4
    Hidden = 4isize,
    ///5
    LootOnly = 5isize,
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
impl serde::Serialize for CellType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl Display for CellType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    ///0
    PerComponent = 0isize,
    ///1
    PerOneCell = 1isize,
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
    ///0
    Default = 0isize,
    ///1
    Class1 = 1isize,
    ///2
    Class2 = 2isize,
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
    ///0
    Undefined = 0isize,
    ///1
    Component = 1isize,
    ///2
    Device = 2isize,
    ///3
    Weapon = 3isize,
    ///4
    AmmunitionObsolete = 4isize,
    ///5
    DroneBay = 5isize,
    ///6
    Ship = 6isize,
    ///7
    Satellite = 7isize,
    ///8
    ShipBuild = 8isize,
    ///9
    SatelliteBuild = 9isize,
    ///10
    Technology = 10isize,
    ///11
    ComponentStats = 11isize,
    ///12
    ComponentMod = 12isize,
    ///13
    Skill = 13isize,
    ///14
    Faction = 14isize,
    ///15
    Quest = 15isize,
    ///16
    Loot = 16isize,
    ///18
    Fleet = 18isize,
    ///19
    Character = 19isize,
    ///20
    QuestItem = 20isize,
    ///25
    Ammunition = 25isize,
    ///26
    VisualEffect = 26isize,
    ///27
    BulletPrefab = 27isize,
    ///28
    BehaviorTree = 28isize,
    ///29
    GameObjectPrefab = 29isize,
    ///30
    CombatRules = 30isize,
    ///100
    ShipSettings = 100isize,
    ///101
    GalaxySettings = 101isize,
    ///102
    DatabaseSettings = 102isize,
    ///103
    ExplorationSettings = 103isize,
    ///104
    FrontierSettings = 104isize,
    ///105
    ShipModSettings = 105isize,
    ///106
    SpecialEventSettings = 106isize,
    ///107
    SkillSettings = 107isize,
    ///108
    DebugSettings = 108isize,
    ///109
    CombatSettings = 109isize,
    ///110
    UiSettings = 110isize,
    ///111
    FactionsSettings = 111isize,
    ///112
    MusicPlaylist = 112isize,
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
    ///0
    N3 = 0isize,
    ///1
    N2 = 1isize,
    ///2
    N1 = 2isize,
    ///3
    P1 = 3isize,
    ///4
    P2 = 4isize,
    ///5
    P3 = 5isize,
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
    ///0
    Undefined = 0isize,
    ///1
    WormTailSegment = 1isize,
    ///2
    CircularSpriteObject = 2isize,
    ///3
    CircularOutlineObject = 3isize,
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
    ///0
    AllButList = 0isize,
    ///1
    ListOnly = 1isize,
    ///2
    StarOwnersAndList = 2isize,
    ///3
    AllAvailable = 3isize,
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
    ///0
    None = 0isize,
    ///1
    SomeMoney = 1isize,
    ///2
    Fuel = 2isize,
    ///3
    Money = 3isize,
    ///4
    Stars = 4isize,
    ///5
    StarMap = 5isize,
    ///10
    RandomComponents = 10isize,
    ///20
    RandomItems = 20isize,
    ///21
    AllItems = 21isize,
    ///22
    ItemsWithChance = 22isize,
    ///25
    QuestItem = 25isize,
    ///30
    Ship = 30isize,
    ///31
    EmptyShip = 31isize,
    ///35
    Component = 35isize,
    ///40
    Blueprint = 40isize,
    ///41
    ResearchPoints = 41isize,
    ///45
    Satellite = 45isize,
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
    ///0
    Undefined = 0isize,
    ///1
    ComingSoon = 1isize,
    ///10
    ShowDialog = 10isize,
    ///11
    OpenShipyard = 11isize,
    ///12
    OpenWorkshop = 12isize,
    ///15
    Switch = 15isize,
    ///16
    Random = 16isize,
    ///17
    Condition = 17isize,
    ///20
    AttackFleet = 20isize,
    ///21
    AttackOccupants = 21isize,
    ///22
    AttackStarbase = 22isize,
    ///25
    DestroyOccupants = 25isize,
    ///26
    SuppressOccupants = 26isize,
    ///30
    Retreat = 30isize,
    ///35
    ReceiveItem = 35isize,
    ///36
    RemoveItem = 36isize,
    ///37
    Trade = 37isize,
    ///40
    CompleteQuest = 40isize,
    ///41
    FailQuest = 41isize,
    ///42
    CancelQuest = 42isize,
    ///43
    StartQuest = 43isize,
    ///50
    SetCharacterRelations = 50isize,
    ///51
    SetFactionRelations = 51isize,
    ///52
    SetFactionStarbasePower = 52isize,
    ///55
    ChangeCharacterRelations = 55isize,
    ///56
    ChangeFactionRelations = 56isize,
    ///57
    ChangeFactionStarbasePower = 57isize,
    ///60
    CaptureStarBase = 60isize,
    ///61
    LiberateStarBase = 61isize,
    ///62
    ChangeFaction = 62isize,
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
    ///0
    Default = 0isize,
    ///1
    OnlyOneShip = 1isize,
    ///2
    ByOrder = 2isize,
    ///3
    NoRetreats = 3isize,
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
    ///0
    Common = 0isize,
    ///1
    Singleton = 1isize,
    ///2
    Storyline = 2isize,
    ///3
    Temporary = 3isize,
    ///4
    Urgent = 4isize,
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
    ///0
    Any = 0isize,
    ///1
    StarSystem = 1isize,
    ///2
    StarMap = 2isize,
    ///3
    GalaxyMap = 3isize,
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
    ///0
    Empty = 0isize,
    ///1
    Any = 1isize,
    ///2
    All = 2isize,
    ///3
    None = 3isize,
    ///6
    PlayerPosition = 6isize,
    ///7
    RandomStarSystem = 7isize,
    ///8
    AggressiveOccupants = 8isize,
    ///9
    QuestCompleted = 9isize,
    ///10
    QuestActive = 10isize,
    ///15
    CharacterRelations = 15isize,
    ///16
    FactionRelations = 16isize,
    ///17
    StarbaseCaptured = 17isize,
    ///18
    FactionStarbasePower = 18isize,
    ///19
    IsHostileFaction = 19isize,
    ///20
    Faction = 20isize,
    ///25
    HaveQuestItem = 25isize,
    ///26
    HaveItem = 26isize,
    ///27
    HaveItemById = 27isize,
    ///30
    ComeToOrigin = 30isize,
    ///40
    TimeSinceQuestStart = 40isize,
    ///41
    TimeSinceLastCompletion = 41isize,
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
    ///0
    Default = 0isize,
    ///1
    Always = 1isize,
    ///2
    Never = 2isize,
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
    ///0
    Manual = 0isize,
    ///1
    Beacon = 1isize,
    ///2
    LocalEncounter = 2isize,
    ///3
    FactionMission = 3isize,
    ///4
    GameStart = 4isize,
    ///5
    NewStarExplored = 5isize,
    ///6
    ArrivedAtStar = 6isize,
    ///7
    Daily = 7isize,
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
    ///0
    CallNextEnemy = 0isize,
    ///1
    DrainPlayerHp = 1isize,
    ///2
    CallNextEnemyOrDraw = 2isize,
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
    ///0
    Common = 0isize,
    ///1
    Drone = 1isize,
    ///2
    Starbase = 2isize,
    ///3
    Special = 3isize,
    ///4
    Flagship = 4isize,
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
    ///-1
    Undefined = -1isize,
    ///0
    Frigate = 0isize,
    ///1
    Destroyer = 1isize,
    ///2
    Cruiser = 2isize,
    ///3
    Battleship = 3isize,
    ///4
    Titan = 4isize,
    ///5
    Starbase = 5isize,
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
    ///0
    Undefined = 0isize,
    ///1
    ShipAttack = 1isize,
    ///2
    ShipDefense = 2isize,
    ///3
    StarbaseAttack = 3isize,
    ///4
    StarbaseDefense = 4isize,
    ///5
    QuickLearning = 5isize,
    ///6
    BetterPrices = 6isize,
    ///7
    BetterLoot = 7isize,
    ///8
    CommandPoints = 8isize,
    ///9
    SalvageDrones = 9isize,
    ///10
    Engineer = 10isize,
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
    ///0
    None = 0isize,
    ///1
    WeaponDamage = 1isize,
    ///2
    WeaponRange = 2isize,
    ///3
    WeaponFireRate = 3isize,
    ///4
    WeaponBulletSpeed = 4isize,
    ///5
    WeaponBulletMass = 5isize,
    ///6
    WeaponAoe = 6isize,
    ///10
    DroneAttack = 10isize,
    ///11
    DroneDefense = 11isize,
    ///12
    DroneSpeed = 12isize,
    ///13
    DroneRange = 13isize,
    ///20
    EnergyCapacity = 20isize,
    ///21
    EnergyRechargeRate = 21isize,
    ///22
    ShieldPoints = 22isize,
    ///23
    ShieldRechargeRate = 23isize,
    ///24
    ArmorPoints = 24isize,
    ///25
    ArmorRepairRate = 25isize,
    ///30
    Resistance = 30isize,
    ///40
    DeviceCooldown = 40isize,
    ///41
    DeviceRange = 41isize,
    ///42
    DevicePower = 42isize,
    ///50
    EnginePower = 50isize,
    ///51
    EngineTurnRate = 51isize,
    ///60
    Mass = 60isize,
    ///61
    EnergyCost = 61isize,
    ///62
    ExtraHitPoints = 62isize,
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
impl serde::Serialize for WeaponSlotType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
impl Display for WeaponSlotType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
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
impl From<BehaviorNodeRequirementEmpty> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementEmpty) -> Self {
        Self::Empty(item)
    }
}
impl From<BehaviorNodeRequirementAny> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementAny) -> Self {
        Self::Any(item)
    }
}
impl From<BehaviorNodeRequirementAll> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementAll) -> Self {
        Self::All(item)
    }
}
impl From<BehaviorNodeRequirementNone> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementNone) -> Self {
        Self::None(item)
    }
}
impl From<BehaviorNodeRequirementAiLevel> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementAiLevel) -> Self {
        Self::AiLevel(item)
    }
}
impl From<BehaviorNodeRequirementMinAiLevel> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementMinAiLevel) -> Self {
        Self::MinAiLevel(item)
    }
}
impl From<BehaviorNodeRequirementSizeClass> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementSizeClass) -> Self {
        Self::SizeClass(item)
    }
}
impl From<BehaviorNodeRequirementHasDevice> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasDevice) -> Self {
        Self::HasDevice(item)
    }
}
impl From<BehaviorNodeRequirementHasDrones> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasDrones) -> Self {
        Self::HasDrones(item)
    }
}
impl From<BehaviorNodeRequirementHasAnyWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasAnyWeapon) -> Self {
        Self::HasAnyWeapon(item)
    }
}
impl From<BehaviorNodeRequirementCanRepairAllies> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementCanRepairAllies) -> Self {
        Self::CanRepairAllies(item)
    }
}
impl From<BehaviorNodeRequirementHasHighRecoilWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasHighRecoilWeapon) -> Self {
        Self::HasHighRecoilWeapon(item)
    }
}
impl From<BehaviorNodeRequirementHasChargeableWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasChargeableWeapon) -> Self {
        Self::HasChargeableWeapon(item)
    }
}
impl From<BehaviorNodeRequirementHasRemotelyControlledWeapon>
for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasRemotelyControlledWeapon) -> Self {
        Self::HasRemotelyControlledWeapon(item)
    }
}
impl From<BehaviorNodeRequirementHasLongRangeWeapon> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasLongRangeWeapon) -> Self {
        Self::HasLongRangeWeapon(item)
    }
}
impl From<BehaviorNodeRequirementHasEngine> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasEngine) -> Self {
        Self::HasEngine(item)
    }
}
impl From<BehaviorNodeRequirementIsDrone> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementIsDrone) -> Self {
        Self::IsDrone(item)
    }
}
impl From<BehaviorNodeRequirementHasKineticResistance> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasKineticResistance) -> Self {
        Self::HasKineticResistance(item)
    }
}
impl From<BehaviorNodeRequirementHasHighManeuverability> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasHighManeuverability) -> Self {
        Self::HasHighManeuverability(item)
    }
}
impl From<BehaviorNodeRequirementHasHighRammingDamage> for BehaviorNodeRequirement {
    fn from(item: BehaviorNodeRequirementHasHighRammingDamage) -> Self {
        Self::HasHighRammingDamage(item)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementAll {
    pub r#requirements: Vec<BehaviorNodeRequirement>,
}
impl BehaviorNodeRequirementAll {
    fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(
        mut self,
        r#requirements: Vec<BehaviorNodeRequirement>,
    ) -> Self {
        self.r#requirements = r#requirements;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementAll {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementAll {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementMinAiLevel {
    pub r#difficulty_level: AiDifficultyLevel,
}
impl BehaviorNodeRequirementMinAiLevel {
    fn new() -> Self {
        Self {
            r#difficulty_level: Default::default(),
        }
    }
    pub fn with_difficulty_level(
        mut self,
        r#difficulty_level: AiDifficultyLevel,
    ) -> Self {
        self.r#difficulty_level = r#difficulty_level;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementMinAiLevel {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementMinAiLevel {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasHighManeuverability {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasHighManeuverability {
    fn new() -> Self {
        Self { r#value: 1f32 }
    }
    pub fn with_value(mut self, r#value: f32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasHighManeuverability {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
    }
}
impl Default for BehaviorNodeRequirementHasHighManeuverability {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasDrones {}
impl BehaviorNodeRequirementHasDrones {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasDrones {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementHasDrones {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementCanRepairAllies {}
impl BehaviorNodeRequirementCanRepairAllies {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementCanRepairAllies {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementCanRepairAllies {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasChargeableWeapon {}
impl BehaviorNodeRequirementHasChargeableWeapon {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasChargeableWeapon {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementHasChargeableWeapon {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasEngine {}
impl BehaviorNodeRequirementHasEngine {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasEngine {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementHasEngine {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasRemotelyControlledWeapon {}
impl BehaviorNodeRequirementHasRemotelyControlledWeapon {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasRemotelyControlledWeapon {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementHasRemotelyControlledWeapon {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementEmpty {}
impl BehaviorNodeRequirementEmpty {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementEmpty {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementEmpty {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasLongRangeWeapon {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasLongRangeWeapon {
    fn new() -> Self {
        Self {
            r#value: Default::default(),
        }
    }
    pub fn with_value(mut self, r#value: f32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasLongRangeWeapon {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
    }
}
impl Default for BehaviorNodeRequirementHasLongRangeWeapon {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasHighRammingDamage {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasHighRammingDamage {
    fn new() -> Self {
        Self {
            r#value: Default::default(),
        }
    }
    pub fn with_value(mut self, r#value: f32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasHighRammingDamage {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
    }
}
impl Default for BehaviorNodeRequirementHasHighRammingDamage {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementIsDrone {}
impl BehaviorNodeRequirementIsDrone {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementIsDrone {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementIsDrone {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasHighRecoilWeapon {}
impl BehaviorNodeRequirementHasHighRecoilWeapon {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasHighRecoilWeapon {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementHasHighRecoilWeapon {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasKineticResistance {
    pub r#value: f32,
}
impl BehaviorNodeRequirementHasKineticResistance {
    fn new() -> Self {
        Self { r#value: 1f32 }
    }
    pub fn with_value(mut self, r#value: f32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasKineticResistance {
    fn validate(&mut self) {
        if self.r#value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
    }
}
impl Default for BehaviorNodeRequirementHasKineticResistance {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementSizeClass {
    pub r#size_class: SizeClass,
}
impl BehaviorNodeRequirementSizeClass {
    fn new() -> Self {
        Self {
            r#size_class: Default::default(),
        }
    }
    pub fn with_size_class(mut self, r#size_class: SizeClass) -> Self {
        self.r#size_class = r#size_class;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementSizeClass {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementSizeClass {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementAny {
    pub r#requirements: Vec<BehaviorNodeRequirement>,
}
impl BehaviorNodeRequirementAny {
    fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(
        mut self,
        r#requirements: Vec<BehaviorNodeRequirement>,
    ) -> Self {
        self.r#requirements = r#requirements;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementAny {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementAny {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementAiLevel {
    pub r#difficulty_level: AiDifficultyLevel,
}
impl BehaviorNodeRequirementAiLevel {
    fn new() -> Self {
        Self {
            r#difficulty_level: Default::default(),
        }
    }
    pub fn with_difficulty_level(
        mut self,
        r#difficulty_level: AiDifficultyLevel,
    ) -> Self {
        self.r#difficulty_level = r#difficulty_level;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementAiLevel {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementAiLevel {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasDevice {
    pub r#device_class: DeviceClass,
}
impl BehaviorNodeRequirementHasDevice {
    fn new() -> Self {
        Self {
            r#device_class: Default::default(),
        }
    }
    pub fn with_device_class(mut self, r#device_class: DeviceClass) -> Self {
        self.r#device_class = r#device_class;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasDevice {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementHasDevice {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementNone {
    pub r#requirements: Vec<BehaviorNodeRequirement>,
}
impl BehaviorNodeRequirementNone {
    fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(
        mut self,
        r#requirements: Vec<BehaviorNodeRequirement>,
    ) -> Self {
        self.r#requirements = r#requirements;
        Self
    }
}
impl DatabaseItem for BehaviorNodeRequirementNone {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementNone {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorNodeRequirementHasAnyWeapon {}
impl BehaviorNodeRequirementHasAnyWeapon {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BehaviorNodeRequirementHasAnyWeapon {
    fn validate(&mut self) {}
}
impl Default for BehaviorNodeRequirementHasAnyWeapon {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Ai/BehaviorTreeNode.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
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
impl From<BehaviorTreeNodeSuccess> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSuccess) -> Self {
        Self::Success(item)
    }
}
impl From<BehaviorTreeNodeFailure> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeFailure) -> Self {
        Self::Failure(item)
    }
}
impl From<BehaviorTreeNodeSubTree> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSubTree) -> Self {
        Self::SubTree(item)
    }
}
impl From<BehaviorTreeNodeSelector> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSelector) -> Self {
        Self::Selector(item)
    }
}
impl From<BehaviorTreeNodeSequence> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSequence) -> Self {
        Self::Sequence(item)
    }
}
impl From<BehaviorTreeNodeParallel> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeParallel) -> Self {
        Self::Parallel(item)
    }
}
impl From<BehaviorTreeNodeRandomSelector> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeRandomSelector) -> Self {
        Self::RandomSelector(item)
    }
}
impl From<BehaviorTreeNodeInvertor> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeInvertor) -> Self {
        Self::Invertor(item)
    }
}
impl From<BehaviorTreeNodeCooldown> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeCooldown) -> Self {
        Self::Cooldown(item)
    }
}
impl From<BehaviorTreeNodeExecute> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeExecute) -> Self {
        Self::Execute(item)
    }
}
impl From<BehaviorTreeNodeParallelSequence> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeParallelSequence) -> Self {
        Self::ParallelSequence(item)
    }
}
impl From<BehaviorTreeNodePreserveTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodePreserveTarget) -> Self {
        Self::PreserveTarget(item)
    }
}
impl From<BehaviorTreeNodeIfThenElse> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIfThenElse) -> Self {
        Self::IfThenElse(item)
    }
}
impl From<BehaviorTreeNodeHasEnoughEnergy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasEnoughEnergy) -> Self {
        Self::HasEnoughEnergy(item)
    }
}
impl From<BehaviorTreeNodeIsLowOnHp> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIsLowOnHp) -> Self {
        Self::IsLowOnHp(item)
    }
}
impl From<BehaviorTreeNodeIsControledByPlayer> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIsControledByPlayer) -> Self {
        Self::IsControledByPlayer(item)
    }
}
impl From<BehaviorTreeNodeHasIncomingThreat> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasIncomingThreat) -> Self {
        Self::HasIncomingThreat(item)
    }
}
impl From<BehaviorTreeNodeHasAdditionalTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasAdditionalTargets) -> Self {
        Self::HasAdditionalTargets(item)
    }
}
impl From<BehaviorTreeNodeIsFasterThanTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeIsFasterThanTarget) -> Self {
        Self::IsFasterThanTarget(item)
    }
}
impl From<BehaviorTreeNodeHasMainTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasMainTarget) -> Self {
        Self::HasMainTarget(item)
    }
}
impl From<BehaviorTreeNodeMainTargetIsAlly> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetIsAlly) -> Self {
        Self::MainTargetIsAlly(item)
    }
}
impl From<BehaviorTreeNodeMainTargetIsEnemy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetIsEnemy) -> Self {
        Self::MainTargetIsEnemy(item)
    }
}
impl From<BehaviorTreeNodeMainTargetLowHp> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetLowHp) -> Self {
        Self::MainTargetLowHp(item)
    }
}
impl From<BehaviorTreeNodeMainTargetWithinAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMainTargetWithinAttackRange) -> Self {
        Self::MainTargetWithinAttackRange(item)
    }
}
impl From<BehaviorTreeNodeHasMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasMothership) -> Self {
        Self::HasMothership(item)
    }
}
impl From<BehaviorTreeNodeTargetDistance> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetDistance) -> Self {
        Self::TargetDistance(item)
    }
}
impl From<BehaviorTreeNodeHasLongerAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasLongerAttackRange) -> Self {
        Self::HasLongerAttackRange(item)
    }
}
impl From<BehaviorTreeNodeFindEnemy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeFindEnemy) -> Self {
        Self::FindEnemy(item)
    }
}
impl From<BehaviorTreeNodeMoveToAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMoveToAttackRange) -> Self {
        Self::MoveToAttackRange(item)
    }
}
impl From<BehaviorTreeNodeAttackMainTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAttackMainTarget) -> Self {
        Self::AttackMainTarget(item)
    }
}
impl From<BehaviorTreeNodeSelectWeapon> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSelectWeapon) -> Self {
        Self::SelectWeapon(item)
    }
}
impl From<BehaviorTreeNodeSpawnDrones> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSpawnDrones) -> Self {
        Self::SpawnDrones(item)
    }
}
impl From<BehaviorTreeNodeRam> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeRam) -> Self {
        Self::Ram(item)
    }
}
impl From<BehaviorTreeNodeDetonateShip> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeDetonateShip) -> Self {
        Self::DetonateShip(item)
    }
}
impl From<BehaviorTreeNodeVanish> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeVanish) -> Self {
        Self::Vanish(item)
    }
}
impl From<BehaviorTreeNodeMaintainAttackRange> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMaintainAttackRange) -> Self {
        Self::MaintainAttackRange(item)
    }
}
impl From<BehaviorTreeNodeWait> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeWait) -> Self {
        Self::Wait(item)
    }
}
impl From<BehaviorTreeNodeLookAtTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLookAtTarget) -> Self {
        Self::LookAtTarget(item)
    }
}
impl From<BehaviorTreeNodeLookForAdditionalTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLookForAdditionalTargets) -> Self {
        Self::LookForAdditionalTargets(item)
    }
}
impl From<BehaviorTreeNodeLookForThreats> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLookForThreats) -> Self {
        Self::LookForThreats(item)
    }
}
impl From<BehaviorTreeNodeMatchVelocityWithTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMatchVelocityWithTarget) -> Self {
        Self::MatchVelocityWithTarget(item)
    }
}
impl From<BehaviorTreeNodeActivateDevice> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeActivateDevice) -> Self {
        Self::ActivateDevice(item)
    }
}
impl From<BehaviorTreeNodeRechargeEnergy> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeRechargeEnergy) -> Self {
        Self::RechargeEnergy(item)
    }
}
impl From<BehaviorTreeNodeSustainAim> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSustainAim) -> Self {
        Self::SustainAim(item)
    }
}
impl From<BehaviorTreeNodeChargeWeapons> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeChargeWeapons) -> Self {
        Self::ChargeWeapons(item)
    }
}
impl From<BehaviorTreeNodeChase> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeChase) -> Self {
        Self::Chase(item)
    }
}
impl From<BehaviorTreeNodeAvoidThreats> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAvoidThreats) -> Self {
        Self::AvoidThreats(item)
    }
}
impl From<BehaviorTreeNodeSlowDown> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSlowDown) -> Self {
        Self::SlowDown(item)
    }
}
impl From<BehaviorTreeNodeUseRecoil> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeUseRecoil) -> Self {
        Self::UseRecoil(item)
    }
}
impl From<BehaviorTreeNodeDefendWithFronalShield> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeDefendWithFronalShield) -> Self {
        Self::DefendWithFronalShield(item)
    }
}
impl From<BehaviorTreeNodeTrackControllableAmmo> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTrackControllableAmmo) -> Self {
        Self::TrackControllableAmmo(item)
    }
}
impl From<BehaviorTreeNodeKeepDistance> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeKeepDistance) -> Self {
        Self::KeepDistance(item)
    }
}
impl From<BehaviorTreeNodeForgetMainTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeForgetMainTarget) -> Self {
        Self::ForgetMainTarget(item)
    }
}
impl From<BehaviorTreeNodeEscapeTargetAttackRadius> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeEscapeTargetAttackRadius) -> Self {
        Self::EscapeTargetAttackRadius(item)
    }
}
impl From<BehaviorTreeNodeAttackAdditionalTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAttackAdditionalTargets) -> Self {
        Self::AttackAdditionalTargets(item)
    }
}
impl From<BehaviorTreeNodeTargetAllyStarbase> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetAllyStarbase) -> Self {
        Self::TargetAllyStarbase(item)
    }
}
impl From<BehaviorTreeNodeTargetEnemyStarbase> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetEnemyStarbase) -> Self {
        Self::TargetEnemyStarbase(item)
    }
}
impl From<BehaviorTreeNodeBypassObstacles> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeBypassObstacles) -> Self {
        Self::BypassObstacles(item)
    }
}
impl From<BehaviorTreeNodeAttackTurretTargets> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeAttackTurretTargets) -> Self {
        Self::AttackTurretTargets(item)
    }
}
impl From<BehaviorTreeNodeEnginePropulsionForce> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeEnginePropulsionForce) -> Self {
        Self::EnginePropulsionForce(item)
    }
}
impl From<BehaviorTreeNodeMotherShipRetreated> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMotherShipRetreated) -> Self {
        Self::MotherShipRetreated(item)
    }
}
impl From<BehaviorTreeNodeMotherShipDestroyed> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMotherShipDestroyed) -> Self {
        Self::MotherShipDestroyed(item)
    }
}
impl From<BehaviorTreeNodeFlyAroundMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeFlyAroundMothership) -> Self {
        Self::FlyAroundMothership(item)
    }
}
impl From<BehaviorTreeNodeGoBerserk> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeGoBerserk) -> Self {
        Self::GoBerserk(item)
    }
}
impl From<BehaviorTreeNodeTargetMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetMothership) -> Self {
        Self::TargetMothership(item)
    }
}
impl From<BehaviorTreeNodeMothershipLowHp> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMothershipLowHp) -> Self {
        Self::MothershipLowHp(item)
    }
}
impl From<BehaviorTreeNodeMothershipDistanceExceeded> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMothershipDistanceExceeded) -> Self {
        Self::MothershipDistanceExceeded(item)
    }
}
impl From<BehaviorTreeNodeMakeTargetMothership> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMakeTargetMothership) -> Self {
        Self::MakeTargetMothership(item)
    }
}
impl From<BehaviorTreeNodeShowMessage> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeShowMessage) -> Self {
        Self::ShowMessage(item)
    }
}
impl From<BehaviorTreeNodeDebugLog> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeDebugLog) -> Self {
        Self::DebugLog(item)
    }
}
impl From<BehaviorTreeNodeSetValue> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSetValue) -> Self {
        Self::SetValue(item)
    }
}
impl From<BehaviorTreeNodeGetValue> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeGetValue) -> Self {
        Self::GetValue(item)
    }
}
impl From<BehaviorTreeNodeSendMessage> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSendMessage) -> Self {
        Self::SendMessage(item)
    }
}
impl From<BehaviorTreeNodeMessageReceived> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeMessageReceived) -> Self {
        Self::MessageReceived(item)
    }
}
impl From<BehaviorTreeNodeTargetMessageSender> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeTargetMessageSender) -> Self {
        Self::TargetMessageSender(item)
    }
}
impl From<BehaviorTreeNodeSaveTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeSaveTarget) -> Self {
        Self::SaveTarget(item)
    }
}
impl From<BehaviorTreeNodeLoadTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeLoadTarget) -> Self {
        Self::LoadTarget(item)
    }
}
impl From<BehaviorTreeNodeHasSavedTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeHasSavedTarget) -> Self {
        Self::HasSavedTarget(item)
    }
}
impl From<BehaviorTreeNodeForgetSavedTarget> for BehaviorTreeNode {
    fn from(item: BehaviorTreeNodeForgetSavedTarget) -> Self {
        Self::ForgetSavedTarget(item)
    }
}
impl BehaviorTreeNode {
    pub fn r#requirement(&self) {
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
    pub fn requirement_mut(&mut self) {
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
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSequence {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeSequence {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_nodes(mut self, r#nodes: Vec<BehaviorTreeNode>) -> Self {
        self.r#nodes = r#nodes;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSequence {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSequence {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMainTargetIsEnemy {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMainTargetIsEnemy {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetIsEnemy {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeMainTargetIsEnemy {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSendMessage {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeSendMessage {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSendMessage {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSendMessage {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeForgetMainTarget {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeForgetMainTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeForgetMainTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeForgetMainTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeRandomSelector {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeRandomSelector {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_nodes(mut self, r#nodes: Vec<BehaviorTreeNode>) -> Self {
        self.r#nodes = r#nodes;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeRandomSelector {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
    }
}
impl Default for BehaviorTreeNodeRandomSelector {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSpawnDrones {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeSpawnDrones {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSpawnDrones {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSpawnDrones {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeRam {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#use_systems: bool,
}
impl BehaviorTreeNodeRam {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#use_systems: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_use_systems(mut self, r#use_systems: bool) -> Self {
        self.r#use_systems = r#use_systems;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeRam {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeRam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSaveTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeSaveTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSaveTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSaveTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeTrackControllableAmmo {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTrackControllableAmmo {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeTrackControllableAmmo {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeTrackControllableAmmo {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeGetValue {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeGetValue {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeGetValue {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeGetValue {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeTargetAllyStarbase {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetAllyStarbase {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetAllyStarbase {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeTargetAllyStarbase {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeLookAtTarget {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeLookAtTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeLookAtTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeLookAtTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeIsControledByPlayer {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeIsControledByPlayer {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeIsControledByPlayer {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeIsControledByPlayer {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeHasIncomingThreat {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeHasIncomingThreat {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: 5f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasIncomingThreat {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
    }
}
impl Default for BehaviorTreeNodeHasIncomingThreat {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeIsLowOnHp {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeIsLowOnHp {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeIsLowOnHp {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeIsLowOnHp {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeHasEnoughEnergy {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeHasEnoughEnergy {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 0.1f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasEnoughEnergy {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeHasEnoughEnergy {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeAvoidThreats {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeAvoidThreats {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeAvoidThreats {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeAvoidThreats {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeIfThenElse {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeIfThenElse {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_nodes(mut self, r#nodes: Vec<BehaviorTreeNode>) -> Self {
        self.r#nodes = r#nodes;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeIfThenElse {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeIfThenElse {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMainTargetIsAlly {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMainTargetIsAlly {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetIsAlly {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeMainTargetIsAlly {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMainTargetWithinAttackRange {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeMainTargetWithinAttackRange {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 1f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetWithinAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeMainTargetWithinAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeEnginePropulsionForce {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeEnginePropulsionForce {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeEnginePropulsionForce {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeEnginePropulsionForce {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeHasAdditionalTargets {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeHasAdditionalTargets {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasAdditionalTargets {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeHasAdditionalTargets {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeTargetDistance {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeTargetDistance {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetDistance {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
    }
}
impl Default for BehaviorTreeNodeTargetDistance {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSelectWeapon {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#weapon_type: AiWeaponCategory,
}
impl BehaviorTreeNodeSelectWeapon {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#weapon_type: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_weapon_type(mut self, r#weapon_type: AiWeaponCategory) -> Self {
        self.r#weapon_type = r#weapon_type;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSelectWeapon {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSelectWeapon {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeTargetMothership {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetMothership {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetMothership {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeTargetMothership {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeTargetMessageSender {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetMessageSender {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetMessageSender {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeTargetMessageSender {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeAttackAdditionalTargets {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeAttackAdditionalTargets {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_in_range(mut self, r#in_range: bool) -> Self {
        self.r#in_range = r#in_range;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeAttackAdditionalTargets {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeAttackAdditionalTargets {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeFindEnemy {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
    pub r#in_range: bool,
    pub r#no_drones: bool,
}
impl BehaviorTreeNodeFindEnemy {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
            r#max_value: 5f32,
            r#in_range: Default::default(),
            r#no_drones: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
    pub fn with_in_range(mut self, r#in_range: bool) -> Self {
        self.r#in_range = r#in_range;
        Self
    }
    pub fn with_no_drones(mut self, r#no_drones: bool) -> Self {
        self.r#no_drones = r#no_drones;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeFindEnemy {
    fn validate(&mut self) {
        if self.r#min_value < (0.5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0.5f32
            );
            self.r#min_value = 0.5f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
    }
}
impl Default for BehaviorTreeNodeFindEnemy {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeLookForThreats {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeLookForThreats {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeLookForThreats {
    fn validate(&mut self) {
        if self.r#cooldown < (0.1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0.1f32
            );
            self.r#cooldown = 0.1f32;
        }
    }
}
impl Default for BehaviorTreeNodeLookForThreats {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMothershipLowHp {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeMothershipLowHp {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMothershipLowHp {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeMothershipLowHp {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSuccess {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeSuccess {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSuccess {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSuccess {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeAttackTurretTargets {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeAttackTurretTargets {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeAttackTurretTargets {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeAttackTurretTargets {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMothershipDistanceExceeded {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMothershipDistanceExceeded {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMothershipDistanceExceeded {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
    }
}
impl Default for BehaviorTreeNodeMothershipDistanceExceeded {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMoveToAttackRange {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMoveToAttackRange {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 1f32,
            r#max_value: 1f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMoveToAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 1f32
            );
            self.r#max_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeMoveToAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSustainAim {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeSustainAim {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSustainAim {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSustainAim {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeWait {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeWait {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
    pub fn with_in_range(mut self, r#in_range: bool) -> Self {
        self.r#in_range = r#in_range;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeWait {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
    }
}
impl Default for BehaviorTreeNodeWait {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeDebugLog {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeDebugLog {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeDebugLog {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeDebugLog {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeHasMainTarget {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeHasMainTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasMainTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeHasMainTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeFailure {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeFailure {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeFailure {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeFailure {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeChargeWeapons {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeChargeWeapons {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeChargeWeapons {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeChargeWeapons {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeParallelSequence {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeParallelSequence {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_nodes(mut self, r#nodes: Vec<BehaviorTreeNode>) -> Self {
        self.r#nodes = r#nodes;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeParallelSequence {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeParallelSequence {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeUseRecoil {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeUseRecoil {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeUseRecoil {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeUseRecoil {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeTargetEnemyStarbase {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeTargetEnemyStarbase {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeTargetEnemyStarbase {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeTargetEnemyStarbase {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeKeepDistance {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeKeepDistance {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 2.5f32,
            r#max_value: 3.5f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeKeepDistance {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 100f32
            );
            self.r#min_value = 100f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 100f32
            );
            self.r#max_value = 100f32;
        }
    }
}
impl Default for BehaviorTreeNodeKeepDistance {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeBypassObstacles {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeBypassObstacles {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeBypassObstacles {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeBypassObstacles {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeDefendWithFronalShield {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeDefendWithFronalShield {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeDefendWithFronalShield {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeDefendWithFronalShield {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSubTree {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#item_id: Option<BehaviorTreeId>,
}
impl BehaviorTreeNodeSubTree {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#item_id: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_item_id(mut self, r#item_id: Option<BehaviorTreeId>) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSubTree {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSubTree {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeDetonateShip {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeDetonateShip {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_in_range(mut self, r#in_range: bool) -> Self {
        self.r#in_range = r#in_range;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeDetonateShip {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeDetonateShip {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeLoadTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeLoadTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeLoadTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeLoadTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeHasLongerAttackRange {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeHasLongerAttackRange {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasLongerAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 1f32
            );
            self.r#min_value = 1f32;
        }
        if self.r#min_value > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 10f32
            );
            self.r#min_value = 10f32;
        }
    }
}
impl Default for BehaviorTreeNodeHasLongerAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeRechargeEnergy {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeRechargeEnergy {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 0.1f32,
            r#max_value: 0.9f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeRechargeEnergy {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 1f32
            );
            self.r#max_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeRechargeEnergy {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeInvertor {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: BehaviorTreeNode,
}
impl BehaviorTreeNodeInvertor {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_node(mut self, r#node: BehaviorTreeNode) -> Self {
        self.r#node = r#node;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeInvertor {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeInvertor {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeHasMothership {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeHasMothership {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasMothership {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeHasMothership {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeActivateDevice {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#device_class: DeviceClass,
}
impl BehaviorTreeNodeActivateDevice {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#device_class: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_device_class(mut self, r#device_class: DeviceClass) -> Self {
        self.r#device_class = r#device_class;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeActivateDevice {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeActivateDevice {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSetValue {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#result: bool,
    pub r#text: String,
}
impl BehaviorTreeNodeSetValue {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#result: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_result(mut self, r#result: bool) -> Self {
        self.r#result = r#result;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSetValue {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSetValue {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMaintainAttackRange {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMaintainAttackRange {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 1f32,
            r#max_value: 0.2f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMaintainAttackRange {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 1f32
            );
            self.r#max_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeMaintainAttackRange {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodePreserveTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: BehaviorTreeNode,
}
impl BehaviorTreeNodePreserveTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_node(mut self, r#node: BehaviorTreeNode) -> Self {
        self.r#node = r#node;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodePreserveTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodePreserveTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeVanish {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeVanish {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeVanish {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeVanish {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMatchVelocityWithTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeMatchVelocityWithTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: 0.2f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMatchVelocityWithTarget {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 1f32
            );
            self.r#max_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeMatchVelocityWithTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeEscapeTargetAttackRadius {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeEscapeTargetAttackRadius {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeEscapeTargetAttackRadius {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeEscapeTargetAttackRadius {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeShowMessage {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
    pub r#color: String,
}
impl BehaviorTreeNodeShowMessage {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
            r#color: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeShowMessage {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeShowMessage {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMainTargetLowHp {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeMainTargetLowHp {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMainTargetLowHp {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1f32
            );
            self.r#min_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeMainTargetLowHp {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMessageReceived {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeMessageReceived {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMessageReceived {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeMessageReceived {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMotherShipRetreated {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMotherShipRetreated {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMotherShipRetreated {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeMotherShipRetreated {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeChase {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeChase {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeChase {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeChase {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeLookForAdditionalTargets {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeLookForAdditionalTargets {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeLookForAdditionalTargets {
    fn validate(&mut self) {
        if self.r#cooldown < (0.1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0.1f32
            );
            self.r#cooldown = 0.1f32;
        }
    }
}
impl Default for BehaviorTreeNodeLookForAdditionalTargets {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeAttackMainTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#in_range: bool,
}
impl BehaviorTreeNodeAttackMainTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#in_range: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_in_range(mut self, r#in_range: bool) -> Self {
        self.r#in_range = r#in_range;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeAttackMainTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeAttackMainTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSelector {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeSelector {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_nodes(mut self, r#nodes: Vec<BehaviorTreeNode>) -> Self {
        self.r#nodes = r#nodes;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSelector {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeSelector {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeFlyAroundMothership {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeFlyAroundMothership {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: 2.5f32,
            r#max_value: 3.5f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeFlyAroundMothership {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 100f32
            );
            self.r#min_value = 100f32;
        }
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 100f32
            );
            self.r#max_value = 100f32;
        }
    }
}
impl Default for BehaviorTreeNodeFlyAroundMothership {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMotherShipDestroyed {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMotherShipDestroyed {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMotherShipDestroyed {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeMotherShipDestroyed {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeMakeTargetMothership {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeMakeTargetMothership {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeMakeTargetMothership {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeMakeTargetMothership {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeGoBerserk {
    pub r#requirement: BehaviorNodeRequirement,
}
impl BehaviorTreeNodeGoBerserk {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeGoBerserk {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeGoBerserk {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeHasSavedTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeHasSavedTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeHasSavedTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeHasSavedTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeForgetSavedTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#text: String,
}
impl BehaviorTreeNodeForgetSavedTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#text: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_text(mut self, r#text: String) -> Self {
        self.r#text = r#text;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeForgetSavedTarget {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeForgetSavedTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeExecute {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: BehaviorTreeNode,
    pub r#execution_mode: NodeExecutionMode,
    pub r#result: bool,
}
impl BehaviorTreeNodeExecute {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
            r#execution_mode: Default::default(),
            r#result: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_node(mut self, r#node: BehaviorTreeNode) -> Self {
        self.r#node = r#node;
        Self
    }
    pub fn with_execution_mode(mut self, r#execution_mode: NodeExecutionMode) -> Self {
        self.r#execution_mode = r#execution_mode;
        Self
    }
    pub fn with_result(mut self, r#result: bool) -> Self {
        self.r#result = r#result;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeExecute {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeExecute {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeSlowDown {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#max_value: f32,
}
impl BehaviorTreeNodeSlowDown {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#max_value: 0.2f32,
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: f32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeSlowDown {
    fn validate(&mut self) {
        if self.r#max_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 1f32
            );
            self.r#max_value = 1f32;
        }
    }
}
impl Default for BehaviorTreeNodeSlowDown {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeParallel {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#nodes: Vec<BehaviorTreeNode>,
}
impl BehaviorTreeNodeParallel {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#nodes: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_nodes(mut self, r#nodes: Vec<BehaviorTreeNode>) -> Self {
        self.r#nodes = r#nodes;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeParallel {
    fn validate(&mut self) {}
}
impl Default for BehaviorTreeNodeParallel {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeIsFasterThanTarget {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#min_value: f32,
}
impl BehaviorTreeNodeIsFasterThanTarget {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: f32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeIsFasterThanTarget {
    fn validate(&mut self) {
        if self.r#min_value < (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 1f32
            );
            self.r#min_value = 1f32;
        }
        if self.r#min_value > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 10f32
            );
            self.r#min_value = 10f32;
        }
    }
}
impl Default for BehaviorTreeNodeIsFasterThanTarget {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTreeNodeCooldown {
    pub r#requirement: BehaviorNodeRequirement,
    pub r#node: BehaviorTreeNode,
    pub r#execution_mode: NodeExecutionMode,
    pub r#result: bool,
    pub r#cooldown: f32,
}
impl BehaviorTreeNodeCooldown {
    fn new() -> Self {
        Self {
            r#requirement: Default::default(),
            r#node: Default::default(),
            r#execution_mode: Default::default(),
            r#result: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_requirement(mut self, r#requirement: BehaviorNodeRequirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_node(mut self, r#node: BehaviorTreeNode) -> Self {
        self.r#node = r#node;
        Self
    }
    pub fn with_execution_mode(mut self, r#execution_mode: NodeExecutionMode) -> Self {
        self.r#execution_mode = r#execution_mode;
        Self
    }
    pub fn with_result(mut self, r#result: bool) -> Self {
        self.r#result = r#result;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BehaviorTreeNodeCooldown {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
    }
}
impl Default for BehaviorTreeNodeCooldown {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Barrel.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_position(mut self, r#position: glam::f32::Vec2) -> Self {
        self.r#position = r#position;
        Self
    }
    pub fn with_rotation(mut self, r#rotation: f32) -> Self {
        self.r#rotation = r#rotation;
        Self
    }
    pub fn with_offset(mut self, r#offset: f32) -> Self {
        self.r#offset = r#offset;
        Self
    }
    pub fn with_platform_type(mut self, r#platform_type: i32) -> Self {
        self.r#platform_type = r#platform_type;
        Self
    }
    pub fn with_auto_aiming_arc(mut self, r#auto_aiming_arc: f32) -> Self {
        self.r#auto_aiming_arc = r#auto_aiming_arc;
        Self
    }
    pub fn with_rotation_speed(mut self, r#rotation_speed: f32) -> Self {
        self.r#rotation_speed = r#rotation_speed;
        Self
    }
    pub fn with_weapon_class(mut self, r#weapon_class: String) -> Self {
        self.r#weapon_class = r#weapon_class;
        Self
    }
    pub fn with_image(mut self, r#image: String) -> Self {
        self.r#image = r#image;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
}
impl DatabaseItem for Barrel {
    fn validate(&mut self) {
        if self.r#rotation < (-360f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#rotation", value = self.r#rotation, min
                = - 360f32
            );
            self.r#rotation = -360f32;
        }
        if self.r#rotation > (360f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#rotation", value = self.r#rotation, max
                = 360f32
            );
            self.r#rotation = 360f32;
        }
        if self.r#offset < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#offset", value = self.r#offset, min =
                0f32
            );
            self.r#offset = 0f32;
        }
        if self.r#offset > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#offset", value = self.r#offset, max =
                1f32
            );
            self.r#offset = 1f32;
        }
        if self.r#platform_type != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#platform_type"
            );
        }
        if self.r#auto_aiming_arc < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#auto_aiming_arc", value = self
                .r#auto_aiming_arc, min = 0f32
            );
            self.r#auto_aiming_arc = 0f32;
        }
        if self.r#auto_aiming_arc > (360f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#auto_aiming_arc", value = self
                .r#auto_aiming_arc, max = 360f32
            );
            self.r#auto_aiming_arc = 360f32;
        }
        if self.r#rotation_speed < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#rotation_speed", value = self
                .r#rotation_speed, min = 0f32
            );
            self.r#rotation_speed = 0f32;
        }
        if self.r#rotation_speed > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#rotation_speed", value = self
                .r#rotation_speed, max = 1000f32
            );
            self.r#rotation_speed = 1000f32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                100f32
            );
            self.r#size = 100f32;
        }
    }
}
impl Default for Barrel {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ComponentRestrictions.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct ComponentRestrictions {
    pub r#ship_sizes: std::collections::HashSet<SizeClass>,
    pub r#not_for_organic_ships: bool,
    pub r#not_for_mechanic_ships: bool,
    pub r#unique_component_tag: String,
    pub r#max_component_amount: i32,
}
impl ComponentRestrictions {
    fn new() -> Self {
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
        r#ship_sizes: std::collections::HashSet<SizeClass>,
    ) -> Self {
        self.r#ship_sizes = r#ship_sizes;
        Self
    }
    pub fn with_not_for_organic_ships(mut self, r#not_for_organic_ships: bool) -> Self {
        self.r#not_for_organic_ships = r#not_for_organic_ships;
        Self
    }
    pub fn with_not_for_mechanic_ships(
        mut self,
        r#not_for_mechanic_ships: bool,
    ) -> Self {
        self.r#not_for_mechanic_ships = r#not_for_mechanic_ships;
        Self
    }
    pub fn with_unique_component_tag(mut self, r#unique_component_tag: String) -> Self {
        self.r#unique_component_tag = r#unique_component_tag;
        Self
    }
    pub fn with_max_component_amount(mut self, r#max_component_amount: i32) -> Self {
        self.r#max_component_amount = r#max_component_amount;
        Self
    }
}
impl DatabaseItem for ComponentRestrictions {
    fn validate(&mut self) {
        if self.r#max_component_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_component_amount", value = self
                .r#max_component_amount, min = 0f32
            );
            self.r#max_component_amount = 0f32;
        }
    }
}
impl Default for ComponentRestrictions {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Engine.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct Engine {
    pub r#position: glam::f32::Vec2,
    pub r#size: f32,
}
impl Engine {
    fn new() -> Self {
        Self {
            r#position: Default::default(),
            r#size: Default::default(),
        }
    }
    pub fn with_position(mut self, r#position: glam::f32::Vec2) -> Self {
        self.r#position = r#position;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
}
impl DatabaseItem for Engine {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max = 1f32
            );
            self.r#size = 1f32;
        }
    }
}
impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/InstalledComponent.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#component_id: ComponentId) -> Self {
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
    pub fn with_component_id(mut self, r#component_id: ComponentId) -> Self {
        self.r#component_id = r#component_id;
        Self
    }
    pub fn with_modification(mut self, r#modification: Option<ComponentModId>) -> Self {
        self.r#modification = r#modification;
        Self
    }
    pub fn with_quality(mut self, r#quality: ModificationQuality) -> Self {
        self.r#quality = r#quality;
        Self
    }
    pub fn with_x(mut self, r#x: i32) -> Self {
        self.r#x = r#x;
        Self
    }
    pub fn with_y(mut self, r#y: i32) -> Self {
        self.r#y = r#y;
        Self
    }
    pub fn with_barrel_id(mut self, r#barrel_id: i32) -> Self {
        self.r#barrel_id = r#barrel_id;
        Self
    }
    pub fn with_behaviour(mut self, r#behaviour: i32) -> Self {
        self.r#behaviour = r#behaviour;
        Self
    }
    pub fn with_key_binding(mut self, r#key_binding: i32) -> Self {
        self.r#key_binding = r#key_binding;
        Self
    }
}
impl DatabaseItem for InstalledComponent {
    fn validate(&mut self) {
        if self.r#component_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#component_id"
            );
        }
        if self.r#x < (-32768f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#x", value = self.r#x, min = - 32768f32
            );
            self.r#x = -32768f32;
        }
        if self.r#x > (32767f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#x", value = self.r#x, max = 32767f32
            );
            self.r#x = 32767f32;
        }
        if self.r#y < (-32768f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#y", value = self.r#y, min = - 32768f32
            );
            self.r#y = -32768f32;
        }
        if self.r#y > (32767f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#y", value = self.r#y, max = 32767f32
            );
            self.r#y = 32767f32;
        }
        if self.r#barrel_id < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#barrel_id", value = self.r#barrel_id,
                min = 0f32
            );
            self.r#barrel_id = 0f32;
        }
        if self.r#barrel_id > (255f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#barrel_id", value = self.r#barrel_id,
                max = 255f32
            );
            self.r#barrel_id = 255f32;
        }
        if self.r#behaviour < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#behaviour", value = self.r#behaviour,
                min = 0f32
            );
            self.r#behaviour = 0f32;
        }
        if self.r#behaviour > (10f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#behaviour", value = self.r#behaviour,
                max = 10f32
            );
            self.r#behaviour = 10f32;
        }
        if self.r#key_binding < (-10f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#key_binding", value = self
                .r#key_binding, min = - 10f32
            );
            self.r#key_binding = -10f32;
        }
        if self.r#key_binding > (10f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#key_binding", value = self
                .r#key_binding, max = 10f32
            );
            self.r#key_binding = 10f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/FactionFilter.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct FactionFilter {
    pub r#type: FactionFilterType,
    pub r#list: Vec<FactionId>,
}
impl FactionFilter {
    fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#list: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: FactionFilterType) -> Self {
        self.r#type = r#type;
        Self
    }
    pub fn with_list(mut self, r#list: Vec<FactionId>) -> Self {
        self.r#list = r#list;
        Self
    }
}
impl DatabaseItem for FactionFilter {
    fn validate(&mut self) {}
}
impl Default for FactionFilter {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/LootContent.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
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
impl From<LootContentNone> for LootContent {
    fn from(item: LootContentNone) -> Self {
        Self::None(item)
    }
}
impl From<LootContentSomeMoney> for LootContent {
    fn from(item: LootContentSomeMoney) -> Self {
        Self::SomeMoney(item)
    }
}
impl From<LootContentFuel> for LootContent {
    fn from(item: LootContentFuel) -> Self {
        Self::Fuel(item)
    }
}
impl From<LootContentMoney> for LootContent {
    fn from(item: LootContentMoney) -> Self {
        Self::Money(item)
    }
}
impl From<LootContentStars> for LootContent {
    fn from(item: LootContentStars) -> Self {
        Self::Stars(item)
    }
}
impl From<LootContentStarMap> for LootContent {
    fn from(item: LootContentStarMap) -> Self {
        Self::StarMap(item)
    }
}
impl From<LootContentRandomComponents> for LootContent {
    fn from(item: LootContentRandomComponents) -> Self {
        Self::RandomComponents(item)
    }
}
impl From<LootContentRandomItems> for LootContent {
    fn from(item: LootContentRandomItems) -> Self {
        Self::RandomItems(item)
    }
}
impl From<LootContentAllItems> for LootContent {
    fn from(item: LootContentAllItems) -> Self {
        Self::AllItems(item)
    }
}
impl From<LootContentItemsWithChance> for LootContent {
    fn from(item: LootContentItemsWithChance) -> Self {
        Self::ItemsWithChance(item)
    }
}
impl From<LootContentQuestItem> for LootContent {
    fn from(item: LootContentQuestItem) -> Self {
        Self::QuestItem(item)
    }
}
impl From<LootContentShip> for LootContent {
    fn from(item: LootContentShip) -> Self {
        Self::Ship(item)
    }
}
impl From<LootContentEmptyShip> for LootContent {
    fn from(item: LootContentEmptyShip) -> Self {
        Self::EmptyShip(item)
    }
}
impl From<LootContentComponent> for LootContent {
    fn from(item: LootContentComponent) -> Self {
        Self::Component(item)
    }
}
impl From<LootContentBlueprint> for LootContent {
    fn from(item: LootContentBlueprint) -> Self {
        Self::Blueprint(item)
    }
}
impl From<LootContentResearchPoints> for LootContent {
    fn from(item: LootContentResearchPoints) -> Self {
        Self::ResearchPoints(item)
    }
}
impl From<LootContentSatellite> for LootContent {
    fn from(item: LootContentSatellite) -> Self {
        Self::Satellite(item)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentSatellite {
    pub r#item_id: SatelliteId,
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentSatellite {
    fn new(r#item_id: SatelliteId) -> Self {
        Self {
            r#item_id,
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: SatelliteId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
}
impl DatabaseItem for LootContentSatellite {
    fn validate(&mut self) {
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentFuel {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentFuel {
    fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
}
impl DatabaseItem for LootContentFuel {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
impl Default for LootContentFuel {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentEmptyShip {
    pub r#item_id: ShipId,
}
impl LootContentEmptyShip {
    fn new(r#item_id: ShipId) -> Self {
        Self { r#item_id }
    }
    pub fn with_item_id(mut self, r#item_id: ShipId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
}
impl DatabaseItem for LootContentEmptyShip {
    fn validate(&mut self) {
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentShip {
    pub r#item_id: ShipBuildId,
}
impl LootContentShip {
    fn new(r#item_id: ShipBuildId) -> Self {
        Self { r#item_id }
    }
    pub fn with_item_id(mut self, r#item_id: ShipBuildId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
}
impl DatabaseItem for LootContentShip {
    fn validate(&mut self) {
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentNone {}
impl LootContentNone {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for LootContentNone {
    fn validate(&mut self) {}
}
impl Default for LootContentNone {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentResearchPoints {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
    pub r#factions: FactionFilter,
}
impl LootContentResearchPoints {
    fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
            r#factions: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
    pub fn with_factions(mut self, r#factions: FactionFilter) -> Self {
        self.r#factions = r#factions;
        Self
    }
}
impl DatabaseItem for LootContentResearchPoints {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
impl Default for LootContentResearchPoints {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentStars {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentStars {
    fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
}
impl DatabaseItem for LootContentStars {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
impl Default for LootContentStars {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentBlueprint {
    pub r#item_id: TechnologyId,
}
impl LootContentBlueprint {
    fn new(r#item_id: TechnologyId) -> Self {
        Self { r#item_id }
    }
    pub fn with_item_id(mut self, r#item_id: TechnologyId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
}
impl DatabaseItem for LootContentBlueprint {
    fn validate(&mut self) {
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentRandomComponents {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
    pub r#value_ratio: f32,
    pub r#factions: FactionFilter,
}
impl LootContentRandomComponents {
    fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
            r#value_ratio: Default::default(),
            r#factions: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
    pub fn with_value_ratio(mut self, r#value_ratio: f32) -> Self {
        self.r#value_ratio = r#value_ratio;
        Self
    }
    pub fn with_factions(mut self, r#factions: FactionFilter) -> Self {
        self.r#factions = r#factions;
        Self
    }
}
impl DatabaseItem for LootContentRandomComponents {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
        if self.r#value_ratio < (0.001f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value_ratio", value = self
                .r#value_ratio, min = 0.001f32
            );
            self.r#value_ratio = 0.001f32;
        }
        if self.r#value_ratio > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value_ratio", value = self
                .r#value_ratio, max = 1000f32
            );
            self.r#value_ratio = 1000f32;
        }
    }
}
impl Default for LootContentRandomComponents {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentQuestItem {
    pub r#item_id: QuestItemId,
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentQuestItem {
    fn new(r#item_id: QuestItemId) -> Self {
        Self {
            r#item_id,
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: QuestItemId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
}
impl DatabaseItem for LootContentQuestItem {
    fn validate(&mut self) {
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentAllItems {
    pub r#items: Vec<LootItem>,
}
impl LootContentAllItems {
    fn new() -> Self {
        Self {
            r#items: Default::default(),
        }
    }
    pub fn with_items(mut self, r#items: Vec<LootItem>) -> Self {
        self.r#items = r#items;
        Self
    }
}
impl DatabaseItem for LootContentAllItems {
    fn validate(&mut self) {}
}
impl Default for LootContentAllItems {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentSomeMoney {
    pub r#value_ratio: f32,
}
impl LootContentSomeMoney {
    fn new() -> Self {
        Self {
            r#value_ratio: Default::default(),
        }
    }
    pub fn with_value_ratio(mut self, r#value_ratio: f32) -> Self {
        self.r#value_ratio = r#value_ratio;
        Self
    }
}
impl DatabaseItem for LootContentSomeMoney {
    fn validate(&mut self) {
        if self.r#value_ratio < (0.001f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value_ratio", value = self
                .r#value_ratio, min = 0.001f32
            );
            self.r#value_ratio = 0.001f32;
        }
        if self.r#value_ratio > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#value_ratio", value = self
                .r#value_ratio, max = 1000f32
            );
            self.r#value_ratio = 1000f32;
        }
    }
}
impl Default for LootContentSomeMoney {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentComponent {
    pub r#item_id: ComponentId,
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentComponent {
    fn new(r#item_id: ComponentId) -> Self {
        Self {
            r#item_id,
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: ComponentId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
}
impl DatabaseItem for LootContentComponent {
    fn validate(&mut self) {
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentStarMap {}
impl LootContentStarMap {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for LootContentStarMap {
    fn validate(&mut self) {}
}
impl Default for LootContentStarMap {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentRandomItems {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
    pub r#items: Vec<LootItem>,
}
impl LootContentRandomItems {
    fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
            r#items: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
    pub fn with_items(mut self, r#items: Vec<LootItem>) -> Self {
        self.r#items = r#items;
        Self
    }
}
impl DatabaseItem for LootContentRandomItems {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
impl Default for LootContentRandomItems {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentMoney {
    pub r#min_amount: i32,
    pub r#max_amount: i32,
}
impl LootContentMoney {
    fn new() -> Self {
        Self {
            r#min_amount: Default::default(),
            r#max_amount: Default::default(),
        }
    }
    pub fn with_min_amount(mut self, r#min_amount: i32) -> Self {
        self.r#min_amount = r#min_amount;
        Self
    }
    pub fn with_max_amount(mut self, r#max_amount: i32) -> Self {
        self.r#max_amount = r#max_amount;
        Self
    }
}
impl DatabaseItem for LootContentMoney {
    fn validate(&mut self) {
        if self.r#min_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                min = 0f32
            );
            self.r#min_amount = 0f32;
        }
        if self.r#min_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_amount", value = self.r#min_amount,
                max = 1000000000f32
            );
            self.r#min_amount = 1000000000f32;
        }
        if self.r#max_amount < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                min = 0f32
            );
            self.r#max_amount = 0f32;
        }
        if self.r#max_amount > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_amount", value = self.r#max_amount,
                max = 1000000000f32
            );
            self.r#max_amount = 1000000000f32;
        }
    }
}
impl Default for LootContentMoney {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootContentItemsWithChance {
    pub r#items: Vec<LootItem>,
}
impl LootContentItemsWithChance {
    fn new() -> Self {
        Self {
            r#items: Default::default(),
        }
    }
    pub fn with_items(mut self, r#items: Vec<LootItem>) -> Self {
        self.r#items = r#items;
        Self
    }
}
impl DatabaseItem for LootContentItemsWithChance {
    fn validate(&mut self) {}
}
impl Default for LootContentItemsWithChance {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/LootItem.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct LootItem {
    pub r#weight: f32,
    pub r#loot: LootContent,
}
impl LootItem {
    fn new() -> Self {
        Self {
            r#weight: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_weight(mut self, r#weight: f32) -> Self {
        self.r#weight = r#weight;
        Self
    }
    pub fn with_loot(mut self, r#loot: LootContent) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for LootItem {
    fn validate(&mut self) {}
}
impl Default for LootItem {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Node.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
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
impl From<NodeUndefined> for Node {
    fn from(item: NodeUndefined) -> Self {
        Self::Undefined(item)
    }
}
impl From<NodeComingSoon> for Node {
    fn from(item: NodeComingSoon) -> Self {
        Self::ComingSoon(item)
    }
}
impl From<NodeShowDialog> for Node {
    fn from(item: NodeShowDialog) -> Self {
        Self::ShowDialog(item)
    }
}
impl From<NodeOpenShipyard> for Node {
    fn from(item: NodeOpenShipyard) -> Self {
        Self::OpenShipyard(item)
    }
}
impl From<NodeOpenWorkshop> for Node {
    fn from(item: NodeOpenWorkshop) -> Self {
        Self::OpenWorkshop(item)
    }
}
impl From<NodeSwitch> for Node {
    fn from(item: NodeSwitch) -> Self {
        Self::Switch(item)
    }
}
impl From<NodeRandom> for Node {
    fn from(item: NodeRandom) -> Self {
        Self::Random(item)
    }
}
impl From<NodeCondition> for Node {
    fn from(item: NodeCondition) -> Self {
        Self::Condition(item)
    }
}
impl From<NodeAttackFleet> for Node {
    fn from(item: NodeAttackFleet) -> Self {
        Self::AttackFleet(item)
    }
}
impl From<NodeAttackOccupants> for Node {
    fn from(item: NodeAttackOccupants) -> Self {
        Self::AttackOccupants(item)
    }
}
impl From<NodeAttackStarbase> for Node {
    fn from(item: NodeAttackStarbase) -> Self {
        Self::AttackStarbase(item)
    }
}
impl From<NodeDestroyOccupants> for Node {
    fn from(item: NodeDestroyOccupants) -> Self {
        Self::DestroyOccupants(item)
    }
}
impl From<NodeSuppressOccupants> for Node {
    fn from(item: NodeSuppressOccupants) -> Self {
        Self::SuppressOccupants(item)
    }
}
impl From<NodeRetreat> for Node {
    fn from(item: NodeRetreat) -> Self {
        Self::Retreat(item)
    }
}
impl From<NodeReceiveItem> for Node {
    fn from(item: NodeReceiveItem) -> Self {
        Self::ReceiveItem(item)
    }
}
impl From<NodeRemoveItem> for Node {
    fn from(item: NodeRemoveItem) -> Self {
        Self::RemoveItem(item)
    }
}
impl From<NodeTrade> for Node {
    fn from(item: NodeTrade) -> Self {
        Self::Trade(item)
    }
}
impl From<NodeCompleteQuest> for Node {
    fn from(item: NodeCompleteQuest) -> Self {
        Self::CompleteQuest(item)
    }
}
impl From<NodeFailQuest> for Node {
    fn from(item: NodeFailQuest) -> Self {
        Self::FailQuest(item)
    }
}
impl From<NodeCancelQuest> for Node {
    fn from(item: NodeCancelQuest) -> Self {
        Self::CancelQuest(item)
    }
}
impl From<NodeStartQuest> for Node {
    fn from(item: NodeStartQuest) -> Self {
        Self::StartQuest(item)
    }
}
impl From<NodeSetCharacterRelations> for Node {
    fn from(item: NodeSetCharacterRelations) -> Self {
        Self::SetCharacterRelations(item)
    }
}
impl From<NodeSetFactionRelations> for Node {
    fn from(item: NodeSetFactionRelations) -> Self {
        Self::SetFactionRelations(item)
    }
}
impl From<NodeSetFactionStarbasePower> for Node {
    fn from(item: NodeSetFactionStarbasePower) -> Self {
        Self::SetFactionStarbasePower(item)
    }
}
impl From<NodeChangeCharacterRelations> for Node {
    fn from(item: NodeChangeCharacterRelations) -> Self {
        Self::ChangeCharacterRelations(item)
    }
}
impl From<NodeChangeFactionRelations> for Node {
    fn from(item: NodeChangeFactionRelations) -> Self {
        Self::ChangeFactionRelations(item)
    }
}
impl From<NodeChangeFactionStarbasePower> for Node {
    fn from(item: NodeChangeFactionStarbasePower) -> Self {
        Self::ChangeFactionStarbasePower(item)
    }
}
impl From<NodeCaptureStarBase> for Node {
    fn from(item: NodeCaptureStarBase) -> Self {
        Self::CaptureStarBase(item)
    }
}
impl From<NodeLiberateStarBase> for Node {
    fn from(item: NodeLiberateStarBase) -> Self {
        Self::LiberateStarBase(item)
    }
}
impl From<NodeChangeFaction> for Node {
    fn from(item: NodeChangeFaction) -> Self {
        Self::ChangeFaction(item)
    }
}
impl Node {
    pub fn r#id(&self) {
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
    pub fn id_mut(&mut self) {
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
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeCompleteQuest {
    pub r#id: i32,
}
impl NodeCompleteQuest {
    fn new() -> Self {
        Self { r#id: Default::default() }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
}
impl DatabaseItem for NodeCompleteQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
    }
}
impl Default for NodeCompleteQuest {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeChangeFactionRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#value: i32,
}
impl NodeChangeFactionRelations {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeChangeFactionRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min = -
                100f32
            );
            self.r#value = -100f32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                100f32
            );
            self.r#value = 100f32;
        }
    }
}
impl Default for NodeChangeFactionRelations {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeAttackFleet {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#failure_transition: i32,
    pub r#enemy: Option<FleetId>,
    pub r#loot: Option<LootId>,
}
impl NodeAttackFleet {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#failure_transition: Default::default(),
            r#enemy: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_failure_transition(mut self, r#failure_transition: i32) -> Self {
        self.r#failure_transition = r#failure_transition;
        Self
    }
    pub fn with_enemy(mut self, r#enemy: Option<FleetId>) -> Self {
        self.r#enemy = r#enemy;
        Self
    }
    pub fn with_loot(mut self, r#loot: Option<LootId>) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for NodeAttackFleet {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#failure_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#failure_transition", value = self
                .r#failure_transition, min = 1f32
            );
            self.r#failure_transition = 1f32;
        }
        if self.r#failure_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#failure_transition", value = self
                .r#failure_transition, max = 999999f32
            );
            self.r#failure_transition = 999999f32;
        }
    }
}
impl Default for NodeAttackFleet {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeRetreat {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeRetreat {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
}
impl DatabaseItem for NodeRetreat {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeRetreat {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeSetFactionStarbasePower {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#value: i32,
}
impl NodeSetFactionStarbasePower {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeSetFactionStarbasePower {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
        if self.r#value > (100000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                100000f32
            );
            self.r#value = 100000f32;
        }
    }
}
impl Default for NodeSetFactionStarbasePower {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeSetCharacterRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#character: Option<CharacterId>,
    pub r#value: i32,
}
impl NodeSetCharacterRelations {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#character: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_character(mut self, r#character: Option<CharacterId>) -> Self {
        self.r#character = r#character;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeSetCharacterRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min = -
                100f32
            );
            self.r#value = -100f32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                100f32
            );
            self.r#value = 100f32;
        }
    }
}
impl Default for NodeSetCharacterRelations {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeSwitch {
    pub r#id: i32,
    pub r#message: String,
    pub r#default_transition: i32,
    pub r#transitions: Vec<NodeTransition>,
}
impl NodeSwitch {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#message: Default::default(),
            r#default_transition: Default::default(),
            r#transitions: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_message(mut self, r#message: String) -> Self {
        self.r#message = r#message;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_transitions(mut self, r#transitions: Vec<NodeTransition>) -> Self {
        self.r#transitions = r#transitions;
        Self
    }
}
impl DatabaseItem for NodeSwitch {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 0f32
            );
            self.r#default_transition = 0f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeSwitch {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeRemoveItem {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#loot: Option<LootId>,
}
impl NodeRemoveItem {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_loot(mut self, r#loot: Option<LootId>) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for NodeRemoveItem {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeRemoveItem {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeOpenShipyard {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#faction: Option<FactionId>,
    pub r#value: i32,
}
impl NodeOpenShipyard {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#faction: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeOpenShipyard {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
        if self.r#value > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                10000f32
            );
            self.r#value = 10000f32;
        }
    }
}
impl Default for NodeOpenShipyard {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeChangeCharacterRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#character: Option<CharacterId>,
    pub r#value: i32,
}
impl NodeChangeCharacterRelations {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#character: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_character(mut self, r#character: Option<CharacterId>) -> Self {
        self.r#character = r#character;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeChangeCharacterRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min = -
                100f32
            );
            self.r#value = -100f32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                100f32
            );
            self.r#value = 100f32;
        }
    }
}
impl Default for NodeChangeCharacterRelations {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeReceiveItem {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#loot: Option<LootId>,
}
impl NodeReceiveItem {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_loot(mut self, r#loot: Option<LootId>) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for NodeReceiveItem {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeReceiveItem {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeStartQuest {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#quest: Option<QuestId>,
}
impl NodeStartQuest {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#quest: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_quest(mut self, r#quest: Option<QuestId>) -> Self {
        self.r#quest = r#quest;
        Self
    }
}
impl DatabaseItem for NodeStartQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeStartQuest {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeFailQuest {
    pub r#id: i32,
}
impl NodeFailQuest {
    fn new() -> Self {
        Self { r#id: Default::default() }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
}
impl DatabaseItem for NodeFailQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
    }
}
impl Default for NodeFailQuest {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeCaptureStarBase {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeCaptureStarBase {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
}
impl DatabaseItem for NodeCaptureStarBase {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeCaptureStarBase {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeSetFactionRelations {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#value: i32,
}
impl NodeSetFactionRelations {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeSetFactionRelations {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min = -
                100f32
            );
            self.r#value = -100f32;
        }
        if self.r#value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                100f32
            );
            self.r#value = 100f32;
        }
    }
}
impl Default for NodeSetFactionRelations {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeAttackOccupants {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#failure_transition: i32,
}
impl NodeAttackOccupants {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#failure_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_failure_transition(mut self, r#failure_transition: i32) -> Self {
        self.r#failure_transition = r#failure_transition;
        Self
    }
}
impl DatabaseItem for NodeAttackOccupants {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#failure_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#failure_transition", value = self
                .r#failure_transition, min = 1f32
            );
            self.r#failure_transition = 1f32;
        }
        if self.r#failure_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#failure_transition", value = self
                .r#failure_transition, max = 999999f32
            );
            self.r#failure_transition = 999999f32;
        }
    }
}
impl Default for NodeAttackOccupants {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_required_view(mut self, r#required_view: RequiredViewMode) -> Self {
        self.r#required_view = r#required_view;
        Self
    }
    pub fn with_message(mut self, r#message: String) -> Self {
        self.r#message = r#message;
        Self
    }
    pub fn with_enemy(mut self, r#enemy: Option<FleetId>) -> Self {
        self.r#enemy = r#enemy;
        Self
    }
    pub fn with_loot(mut self, r#loot: Option<LootId>) -> Self {
        self.r#loot = r#loot;
        Self
    }
    pub fn with_character(mut self, r#character: Option<CharacterId>) -> Self {
        self.r#character = r#character;
        Self
    }
    pub fn with_actions(mut self, r#actions: Vec<NodeAction>) -> Self {
        self.r#actions = r#actions;
        Self
    }
}
impl DatabaseItem for NodeShowDialog {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
    }
}
impl Default for NodeShowDialog {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeComingSoon {
    pub r#id: i32,
}
impl NodeComingSoon {
    fn new() -> Self {
        Self { r#id: Default::default() }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
}
impl DatabaseItem for NodeComingSoon {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
    }
}
impl Default for NodeComingSoon {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeOpenWorkshop {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#faction: Option<FactionId>,
    pub r#value: i32,
}
impl NodeOpenWorkshop {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#faction: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeOpenWorkshop {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
        if self.r#value > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                10000f32
            );
            self.r#value = 10000f32;
        }
    }
}
impl Default for NodeOpenWorkshop {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeAttackStarbase {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#failure_transition: i32,
}
impl NodeAttackStarbase {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#failure_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_failure_transition(mut self, r#failure_transition: i32) -> Self {
        self.r#failure_transition = r#failure_transition;
        Self
    }
}
impl DatabaseItem for NodeAttackStarbase {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#failure_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#failure_transition", value = self
                .r#failure_transition, min = 1f32
            );
            self.r#failure_transition = 1f32;
        }
        if self.r#failure_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#failure_transition", value = self
                .r#failure_transition, max = 999999f32
            );
            self.r#failure_transition = 999999f32;
        }
    }
}
impl Default for NodeAttackStarbase {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeRandom {
    pub r#id: i32,
    pub r#message: String,
    pub r#default_transition: i32,
    pub r#transitions: Vec<NodeTransition>,
}
impl NodeRandom {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#message: Default::default(),
            r#default_transition: Default::default(),
            r#transitions: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_message(mut self, r#message: String) -> Self {
        self.r#message = r#message;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_transitions(mut self, r#transitions: Vec<NodeTransition>) -> Self {
        self.r#transitions = r#transitions;
        Self
    }
}
impl DatabaseItem for NodeRandom {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 0f32
            );
            self.r#default_transition = 0f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeRandom {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeTrade {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#loot: Option<LootId>,
}
impl NodeTrade {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_loot(mut self, r#loot: Option<LootId>) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for NodeTrade {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeTrade {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeChangeFaction {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#faction: Option<FactionId>,
}
impl NodeChangeFaction {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#faction: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
}
impl DatabaseItem for NodeChangeFaction {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeChangeFaction {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeUndefined {
    pub r#id: i32,
}
impl NodeUndefined {
    fn new() -> Self {
        Self { r#id: Default::default() }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
}
impl DatabaseItem for NodeUndefined {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
    }
}
impl Default for NodeUndefined {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeCondition {
    pub r#id: i32,
    pub r#message: String,
    pub r#transitions: Vec<NodeTransition>,
}
impl NodeCondition {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#message: Default::default(),
            r#transitions: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_message(mut self, r#message: String) -> Self {
        self.r#message = r#message;
        Self
    }
    pub fn with_transitions(mut self, r#transitions: Vec<NodeTransition>) -> Self {
        self.r#transitions = r#transitions;
        Self
    }
}
impl DatabaseItem for NodeCondition {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
    }
}
impl Default for NodeCondition {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeSuppressOccupants {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeSuppressOccupants {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
}
impl DatabaseItem for NodeSuppressOccupants {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeSuppressOccupants {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeCancelQuest {
    pub r#id: i32,
}
impl NodeCancelQuest {
    fn new() -> Self {
        Self { r#id: Default::default() }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
}
impl DatabaseItem for NodeCancelQuest {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
    }
}
impl Default for NodeCancelQuest {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeDestroyOccupants {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeDestroyOccupants {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
}
impl DatabaseItem for NodeDestroyOccupants {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeDestroyOccupants {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeLiberateStarBase {
    pub r#id: i32,
    pub r#default_transition: i32,
}
impl NodeLiberateStarBase {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
}
impl DatabaseItem for NodeLiberateStarBase {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
    }
}
impl Default for NodeLiberateStarBase {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeChangeFactionStarbasePower {
    pub r#id: i32,
    pub r#default_transition: i32,
    pub r#value: i32,
}
impl NodeChangeFactionStarbasePower {
    fn new() -> Self {
        Self {
            r#id: Default::default(),
            r#default_transition: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: i32) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_default_transition(mut self, r#default_transition: i32) -> Self {
        self.r#default_transition = r#default_transition;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for NodeChangeFactionStarbasePower {
    fn validate(&mut self) {
        if self.r#id < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, min = 1f32
            );
            self.r#id = 1f32;
        }
        if self.r#id > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#id", value = self.r#id, max = 999999f32
            );
            self.r#id = 999999f32;
        }
        if self.r#default_transition < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, min = 1f32
            );
            self.r#default_transition = 1f32;
        }
        if self.r#default_transition > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_transition", value = self
                .r#default_transition, max = 999999f32
            );
            self.r#default_transition = 999999f32;
        }
        if self.r#value < (-100000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min = -
                100000f32
            );
            self.r#value = -100000f32;
        }
        if self.r#value > (100000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, max =
                100000f32
            );
            self.r#value = 100000f32;
        }
    }
}
impl Default for NodeChangeFactionStarbasePower {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/NodeAction.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeAction {
    pub r#target_node: i32,
    pub r#requirement: Requirement,
    pub r#button_text: String,
}
impl NodeAction {
    fn new() -> Self {
        Self {
            r#target_node: Default::default(),
            r#requirement: Default::default(),
            r#button_text: Default::default(),
        }
    }
    pub fn with_target_node(mut self, r#target_node: i32) -> Self {
        self.r#target_node = r#target_node;
        Self
    }
    pub fn with_requirement(mut self, r#requirement: Requirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_button_text(mut self, r#button_text: String) -> Self {
        self.r#button_text = r#button_text;
        Self
    }
}
impl DatabaseItem for NodeAction {
    fn validate(&mut self) {
        if self.r#target_node < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#target_node", value = self
                .r#target_node, min = 1f32
            );
            self.r#target_node = 1f32;
        }
        if self.r#target_node > (1000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#target_node", value = self
                .r#target_node, max = 1000f32
            );
            self.r#target_node = 1000f32;
        }
    }
}
impl Default for NodeAction {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/NodeTransition.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct NodeTransition {
    pub r#target_node: i32,
    pub r#requirement: Requirement,
    pub r#weight: f32,
}
impl NodeTransition {
    fn new() -> Self {
        Self {
            r#target_node: Default::default(),
            r#requirement: Default::default(),
            r#weight: Default::default(),
        }
    }
    pub fn with_target_node(mut self, r#target_node: i32) -> Self {
        self.r#target_node = r#target_node;
        Self
    }
    pub fn with_requirement(mut self, r#requirement: Requirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_weight(mut self, r#weight: f32) -> Self {
        self.r#weight = r#weight;
        Self
    }
}
impl DatabaseItem for NodeTransition {
    fn validate(&mut self) {
        if self.r#target_node < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#target_node", value = self
                .r#target_node, min = 1f32
            );
            self.r#target_node = 1f32;
        }
        if self.r#target_node > (1000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#target_node", value = self
                .r#target_node, max = 1000f32
            );
            self.r#target_node = 1000f32;
        }
        if self.r#weight < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, min =
                0f32
            );
            self.r#weight = 0f32;
        }
        if self.r#weight > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, max =
                1000f32
            );
            self.r#weight = 1000f32;
        }
    }
}
impl Default for NodeTransition {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/QuestOrigin.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuestOrigin {
    pub r#type: QuestOriginType,
    pub r#factions: FactionFilter,
    pub r#min_distance: i32,
    pub r#max_distance: i32,
    pub r#min_relations: i32,
    pub r#max_relations: i32,
}
impl QuestOrigin {
    fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#factions: Default::default(),
            r#min_distance: Default::default(),
            r#max_distance: Default::default(),
            r#min_relations: Default::default(),
            r#max_relations: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: QuestOriginType) -> Self {
        self.r#type = r#type;
        Self
    }
    pub fn with_factions(mut self, r#factions: FactionFilter) -> Self {
        self.r#factions = r#factions;
        Self
    }
    pub fn with_min_distance(mut self, r#min_distance: i32) -> Self {
        self.r#min_distance = r#min_distance;
        Self
    }
    pub fn with_max_distance(mut self, r#max_distance: i32) -> Self {
        self.r#max_distance = r#max_distance;
        Self
    }
    pub fn with_min_relations(mut self, r#min_relations: i32) -> Self {
        self.r#min_relations = r#min_relations;
        Self
    }
    pub fn with_max_relations(mut self, r#max_relations: i32) -> Self {
        self.r#max_relations = r#max_relations;
        Self
    }
}
impl DatabaseItem for QuestOrigin {
    fn validate(&mut self) {
        if self.r#min_distance < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_distance", value = self
                .r#min_distance, min = 0f32
            );
            self.r#min_distance = 0f32;
        }
        if self.r#min_distance > (9999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_distance", value = self
                .r#min_distance, max = 9999f32
            );
            self.r#min_distance = 9999f32;
        }
        if self.r#max_distance < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_distance", value = self
                .r#max_distance, min = 0f32
            );
            self.r#max_distance = 0f32;
        }
        if self.r#max_distance > (9999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_distance", value = self
                .r#max_distance, max = 9999f32
            );
            self.r#max_distance = 9999f32;
        }
        if self.r#min_relations < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_relations", value = self
                .r#min_relations, min = - 100f32
            );
            self.r#min_relations = -100f32;
        }
        if self.r#min_relations > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_relations", value = self
                .r#min_relations, max = 100f32
            );
            self.r#min_relations = 100f32;
        }
        if self.r#max_relations < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_relations", value = self
                .r#max_relations, min = - 100f32
            );
            self.r#max_relations = -100f32;
        }
        if self.r#max_relations > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_relations", value = self
                .r#max_relations, max = 100f32
            );
            self.r#max_relations = 100f32;
        }
    }
}
impl Default for QuestOrigin {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Requirement.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
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
impl From<RequirementEmpty> for Requirement {
    fn from(item: RequirementEmpty) -> Self {
        Self::Empty(item)
    }
}
impl From<RequirementAny> for Requirement {
    fn from(item: RequirementAny) -> Self {
        Self::Any(item)
    }
}
impl From<RequirementAll> for Requirement {
    fn from(item: RequirementAll) -> Self {
        Self::All(item)
    }
}
impl From<RequirementNone> for Requirement {
    fn from(item: RequirementNone) -> Self {
        Self::None(item)
    }
}
impl From<RequirementPlayerPosition> for Requirement {
    fn from(item: RequirementPlayerPosition) -> Self {
        Self::PlayerPosition(item)
    }
}
impl From<RequirementRandomStarSystem> for Requirement {
    fn from(item: RequirementRandomStarSystem) -> Self {
        Self::RandomStarSystem(item)
    }
}
impl From<RequirementAggressiveOccupants> for Requirement {
    fn from(item: RequirementAggressiveOccupants) -> Self {
        Self::AggressiveOccupants(item)
    }
}
impl From<RequirementQuestCompleted> for Requirement {
    fn from(item: RequirementQuestCompleted) -> Self {
        Self::QuestCompleted(item)
    }
}
impl From<RequirementQuestActive> for Requirement {
    fn from(item: RequirementQuestActive) -> Self {
        Self::QuestActive(item)
    }
}
impl From<RequirementCharacterRelations> for Requirement {
    fn from(item: RequirementCharacterRelations) -> Self {
        Self::CharacterRelations(item)
    }
}
impl From<RequirementFactionRelations> for Requirement {
    fn from(item: RequirementFactionRelations) -> Self {
        Self::FactionRelations(item)
    }
}
impl From<RequirementStarbaseCaptured> for Requirement {
    fn from(item: RequirementStarbaseCaptured) -> Self {
        Self::StarbaseCaptured(item)
    }
}
impl From<RequirementFactionStarbasePower> for Requirement {
    fn from(item: RequirementFactionStarbasePower) -> Self {
        Self::FactionStarbasePower(item)
    }
}
impl From<RequirementIsHostileFaction> for Requirement {
    fn from(item: RequirementIsHostileFaction) -> Self {
        Self::IsHostileFaction(item)
    }
}
impl From<RequirementFaction> for Requirement {
    fn from(item: RequirementFaction) -> Self {
        Self::Faction(item)
    }
}
impl From<RequirementHaveQuestItem> for Requirement {
    fn from(item: RequirementHaveQuestItem) -> Self {
        Self::HaveQuestItem(item)
    }
}
impl From<RequirementHaveItem> for Requirement {
    fn from(item: RequirementHaveItem) -> Self {
        Self::HaveItem(item)
    }
}
impl From<RequirementHaveItemById> for Requirement {
    fn from(item: RequirementHaveItemById) -> Self {
        Self::HaveItemById(item)
    }
}
impl From<RequirementComeToOrigin> for Requirement {
    fn from(item: RequirementComeToOrigin) -> Self {
        Self::ComeToOrigin(item)
    }
}
impl From<RequirementTimeSinceQuestStart> for Requirement {
    fn from(item: RequirementTimeSinceQuestStart) -> Self {
        Self::TimeSinceQuestStart(item)
    }
}
impl From<RequirementTimeSinceLastCompletion> for Requirement {
    fn from(item: RequirementTimeSinceLastCompletion) -> Self {
        Self::TimeSinceLastCompletion(item)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementPlayerPosition {
    pub r#min_value: i32,
    pub r#max_value: i32,
    pub r#bool_value: bool,
}
impl RequirementPlayerPosition {
    fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
            r#bool_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: i32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
    pub fn with_bool_value(mut self, r#bool_value: bool) -> Self {
        self.r#bool_value = r#bool_value;
        Self
    }
}
impl DatabaseItem for RequirementPlayerPosition {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 10000f32
            );
            self.r#min_value = 10000f32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 10000f32
            );
            self.r#max_value = 10000f32;
        }
    }
}
impl Default for RequirementPlayerPosition {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementAll {
    pub r#requirements: Vec<Requirement>,
}
impl RequirementAll {
    fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(mut self, r#requirements: Vec<Requirement>) -> Self {
        self.r#requirements = r#requirements;
        Self
    }
}
impl DatabaseItem for RequirementAll {
    fn validate(&mut self) {}
}
impl Default for RequirementAll {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementAny {
    pub r#requirements: Vec<Requirement>,
}
impl RequirementAny {
    fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(mut self, r#requirements: Vec<Requirement>) -> Self {
        self.r#requirements = r#requirements;
        Self
    }
}
impl DatabaseItem for RequirementAny {
    fn validate(&mut self) {}
}
impl Default for RequirementAny {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementTimeSinceQuestStart {
    pub r#min_value: i32,
    pub r#max_value: i32,
}
impl RequirementTimeSinceQuestStart {
    fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: i32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for RequirementTimeSinceQuestStart {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 999999f32
            );
            self.r#min_value = 999999f32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 999999f32
            );
            self.r#max_value = 999999f32;
        }
    }
}
impl Default for RequirementTimeSinceQuestStart {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementRandomStarSystem {
    pub r#min_value: i32,
    pub r#max_value: i32,
    pub r#bool_value: bool,
}
impl RequirementRandomStarSystem {
    fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
            r#bool_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: i32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
    pub fn with_bool_value(mut self, r#bool_value: bool) -> Self {
        self.r#bool_value = r#bool_value;
        Self
    }
}
impl DatabaseItem for RequirementRandomStarSystem {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 10000f32
            );
            self.r#min_value = 10000f32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 10000f32
            );
            self.r#max_value = 10000f32;
        }
    }
}
impl Default for RequirementRandomStarSystem {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementAggressiveOccupants {}
impl RequirementAggressiveOccupants {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementAggressiveOccupants {
    fn validate(&mut self) {}
}
impl Default for RequirementAggressiveOccupants {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementStarbaseCaptured {}
impl RequirementStarbaseCaptured {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementStarbaseCaptured {
    fn validate(&mut self) {}
}
impl Default for RequirementStarbaseCaptured {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementQuestActive {
    pub r#item_id: Option<QuestId>,
}
impl RequirementQuestActive {
    fn new() -> Self {
        Self {
            r#item_id: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: Option<QuestId>) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
}
impl DatabaseItem for RequirementQuestActive {
    fn validate(&mut self) {}
}
impl Default for RequirementQuestActive {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementFactionRelations {
    pub r#min_value: i32,
    pub r#max_value: i32,
}
impl RequirementFactionRelations {
    fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: i32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for RequirementFactionRelations {
    fn validate(&mut self) {
        if self.r#min_value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = - 100f32
            );
            self.r#min_value = -100f32;
        }
        if self.r#min_value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 100f32
            );
            self.r#min_value = 100f32;
        }
        if self.r#max_value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = - 100f32
            );
            self.r#max_value = -100f32;
        }
        if self.r#max_value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 100f32
            );
            self.r#max_value = 100f32;
        }
    }
}
impl Default for RequirementFactionRelations {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementFaction {
    pub r#faction: Option<FactionId>,
}
impl RequirementFaction {
    fn new() -> Self {
        Self {
            r#faction: Default::default(),
        }
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
}
impl DatabaseItem for RequirementFaction {
    fn validate(&mut self) {}
}
impl Default for RequirementFaction {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementHaveItem {
    pub r#loot: LootContent,
}
impl RequirementHaveItem {
    fn new() -> Self {
        Self { r#loot: Default::default() }
    }
    pub fn with_loot(mut self, r#loot: LootContent) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for RequirementHaveItem {
    fn validate(&mut self) {}
}
impl Default for RequirementHaveItem {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementHaveItemById {
    pub r#item_id: Option<LootId>,
}
impl RequirementHaveItemById {
    fn new() -> Self {
        Self {
            r#item_id: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: Option<LootId>) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
}
impl DatabaseItem for RequirementHaveItemById {
    fn validate(&mut self) {}
}
impl Default for RequirementHaveItemById {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementTimeSinceLastCompletion {
    pub r#min_value: i32,
    pub r#max_value: i32,
}
impl RequirementTimeSinceLastCompletion {
    fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: i32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for RequirementTimeSinceLastCompletion {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 999999f32
            );
            self.r#min_value = 999999f32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 999999f32
            );
            self.r#max_value = 999999f32;
        }
    }
}
impl Default for RequirementTimeSinceLastCompletion {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementComeToOrigin {
    pub r#bool_value: bool,
}
impl RequirementComeToOrigin {
    fn new() -> Self {
        Self {
            r#bool_value: Default::default(),
        }
    }
    pub fn with_bool_value(mut self, r#bool_value: bool) -> Self {
        self.r#bool_value = r#bool_value;
        Self
    }
}
impl DatabaseItem for RequirementComeToOrigin {
    fn validate(&mut self) {}
}
impl Default for RequirementComeToOrigin {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementCharacterRelations {
    pub r#min_value: i32,
    pub r#max_value: i32,
    pub r#character: Option<CharacterId>,
}
impl RequirementCharacterRelations {
    fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
            r#character: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: i32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
    pub fn with_character(mut self, r#character: Option<CharacterId>) -> Self {
        self.r#character = r#character;
        Self
    }
}
impl DatabaseItem for RequirementCharacterRelations {
    fn validate(&mut self) {
        if self.r#min_value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = - 100f32
            );
            self.r#min_value = -100f32;
        }
        if self.r#min_value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 100f32
            );
            self.r#min_value = 100f32;
        }
        if self.r#max_value < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = - 100f32
            );
            self.r#max_value = -100f32;
        }
        if self.r#max_value > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 100f32
            );
            self.r#max_value = 100f32;
        }
    }
}
impl Default for RequirementCharacterRelations {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementIsHostileFaction {}
impl RequirementIsHostileFaction {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementIsHostileFaction {
    fn validate(&mut self) {}
}
impl Default for RequirementIsHostileFaction {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementFactionStarbasePower {
    pub r#min_value: i32,
    pub r#max_value: i32,
}
impl RequirementFactionStarbasePower {
    fn new() -> Self {
        Self {
            r#min_value: Default::default(),
            r#max_value: Default::default(),
        }
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
    pub fn with_max_value(mut self, r#max_value: i32) -> Self {
        self.r#max_value = r#max_value;
        Self
    }
}
impl DatabaseItem for RequirementFactionStarbasePower {
    fn validate(&mut self) {
        if self.r#min_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 0f32
            );
            self.r#min_value = 0f32;
        }
        if self.r#min_value > (100000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 100000f32
            );
            self.r#min_value = 100000f32;
        }
        if self.r#max_value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                min = 0f32
            );
            self.r#max_value = 0f32;
        }
        if self.r#max_value > (100000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_value", value = self.r#max_value,
                max = 100000f32
            );
            self.r#max_value = 100000f32;
        }
    }
}
impl Default for RequirementFactionStarbasePower {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementQuestCompleted {
    pub r#item_id: Option<QuestId>,
}
impl RequirementQuestCompleted {
    fn new() -> Self {
        Self {
            r#item_id: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: Option<QuestId>) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
}
impl DatabaseItem for RequirementQuestCompleted {
    fn validate(&mut self) {}
}
impl Default for RequirementQuestCompleted {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementNone {
    pub r#requirements: Vec<Requirement>,
}
impl RequirementNone {
    fn new() -> Self {
        Self {
            r#requirements: Default::default(),
        }
    }
    pub fn with_requirements(mut self, r#requirements: Vec<Requirement>) -> Self {
        self.r#requirements = r#requirements;
        Self
    }
}
impl DatabaseItem for RequirementNone {
    fn validate(&mut self) {}
}
impl Default for RequirementNone {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementHaveQuestItem {
    pub r#item_id: Option<QuestItemId>,
    pub r#min_value: i32,
}
impl RequirementHaveQuestItem {
    fn new() -> Self {
        Self {
            r#item_id: Default::default(),
            r#min_value: Default::default(),
        }
    }
    pub fn with_item_id(mut self, r#item_id: Option<QuestItemId>) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
    pub fn with_min_value(mut self, r#min_value: i32) -> Self {
        self.r#min_value = r#min_value;
        Self
    }
}
impl DatabaseItem for RequirementHaveQuestItem {
    fn validate(&mut self) {
        if self.r#min_value < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                min = 1f32
            );
            self.r#min_value = 1f32;
        }
        if self.r#min_value > (1000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#min_value", value = self.r#min_value,
                max = 1000000f32
            );
            self.r#min_value = 1000000f32;
        }
    }
}
impl Default for RequirementHaveQuestItem {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct RequirementEmpty {}
impl RequirementEmpty {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for RequirementEmpty {
    fn validate(&mut self) {}
}
impl Default for RequirementEmpty {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/DebugCode.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct DebugCode {
    pub r#code: i32,
    pub r#loot: LootContent,
}
impl DebugCode {
    fn new() -> Self {
        Self {
            r#code: Default::default(),
            r#loot: Default::default(),
        }
    }
    pub fn with_code(mut self, r#code: i32) -> Self {
        self.r#code = r#code;
        Self
    }
    pub fn with_loot(mut self, r#loot: LootContent) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for DebugCode {
    fn validate(&mut self) {
        if self.r#code < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#code", value = self.r#code, min = 0f32
            );
            self.r#code = 0f32;
        }
        if self.r#code > (999999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#code", value = self.r#code, max =
                999999f32
            );
            self.r#code = 999999f32;
        }
    }
}
impl Default for DebugCode {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ShipToValue.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShipToValue {
    pub r#ship: Option<ShipId>,
    pub r#value: i32,
}
impl ShipToValue {
    fn new() -> Self {
        Self {
            r#ship: Default::default(),
            r#value: Default::default(),
        }
    }
    pub fn with_ship(mut self, r#ship: Option<ShipId>) -> Self {
        self.r#ship = r#ship;
        Self
    }
    pub fn with_value(mut self, r#value: i32) -> Self {
        self.r#value = r#value;
        Self
    }
}
impl DatabaseItem for ShipToValue {
    fn validate(&mut self) {
        if self.r#value < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#value", value = self.r#value, min =
                0f32
            );
            self.r#value = 0f32;
        }
    }
}
impl Default for ShipToValue {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/SoundTrack.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoundTrack {
    pub r#audio: String,
}
impl SoundTrack {
    fn new() -> Self {
        Self {
            r#audio: Default::default(),
        }
    }
    pub fn with_audio(mut self, r#audio: String) -> Self {
        self.r#audio = r#audio;
        Self
    }
}
impl DatabaseItem for SoundTrack {
    fn validate(&mut self) {}
}
impl Default for SoundTrack {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ShipFeatures.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_energy_resistance(mut self, r#energy_resistance: f32) -> Self {
        self.r#energy_resistance = r#energy_resistance;
        Self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: f32) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance;
        Self
    }
    pub fn with_heat_resistance(mut self, r#heat_resistance: f32) -> Self {
        self.r#heat_resistance = r#heat_resistance;
        Self
    }
    pub fn with_ship_weight_bonus(mut self, r#ship_weight_bonus: f32) -> Self {
        self.r#ship_weight_bonus = r#ship_weight_bonus;
        Self
    }
    pub fn with_equipment_weight_bonus(mut self, r#equipment_weight_bonus: f32) -> Self {
        self.r#equipment_weight_bonus = r#equipment_weight_bonus;
        Self
    }
    pub fn with_velocity_bonus(mut self, r#velocity_bonus: f32) -> Self {
        self.r#velocity_bonus = r#velocity_bonus;
        Self
    }
    pub fn with_turn_rate_bonus(mut self, r#turn_rate_bonus: f32) -> Self {
        self.r#turn_rate_bonus = r#turn_rate_bonus;
        Self
    }
    pub fn with_armor_bonus(mut self, r#armor_bonus: f32) -> Self {
        self.r#armor_bonus = r#armor_bonus;
        Self
    }
    pub fn with_shield_bonus(mut self, r#shield_bonus: f32) -> Self {
        self.r#shield_bonus = r#shield_bonus;
        Self
    }
    pub fn with_energy_bonus(mut self, r#energy_bonus: f32) -> Self {
        self.r#energy_bonus = r#energy_bonus;
        Self
    }
    pub fn with_regeneration(mut self, r#regeneration: bool) -> Self {
        self.r#regeneration = r#regeneration;
        Self
    }
    pub fn with_builtin_devices(mut self, r#builtin_devices: Vec<DeviceId>) -> Self {
        self.r#builtin_devices = r#builtin_devices;
        Self
    }
}
impl DatabaseItem for ShipFeatures {
    fn validate(&mut self) {
        if self.r#energy_resistance < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_resistance", value = self
                .r#energy_resistance, min = - 100f32
            );
            self.r#energy_resistance = -100f32;
        }
        if self.r#energy_resistance > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_resistance", value = self
                .r#energy_resistance, max = 100f32
            );
            self.r#energy_resistance = 100f32;
        }
        if self.r#kinetic_resistance < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_resistance", value = self
                .r#kinetic_resistance, min = - 100f32
            );
            self.r#kinetic_resistance = -100f32;
        }
        if self.r#kinetic_resistance > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_resistance", value = self
                .r#kinetic_resistance, max = 100f32
            );
            self.r#kinetic_resistance = 100f32;
        }
        if self.r#heat_resistance < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#heat_resistance", value = self
                .r#heat_resistance, min = - 100f32
            );
            self.r#heat_resistance = -100f32;
        }
        if self.r#heat_resistance > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#heat_resistance", value = self
                .r#heat_resistance, max = 100f32
            );
            self.r#heat_resistance = 100f32;
        }
        if self.r#ship_weight_bonus < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#ship_weight_bonus", value = self
                .r#ship_weight_bonus, min = - 1f32
            );
            self.r#ship_weight_bonus = -1f32;
        }
        if self.r#ship_weight_bonus > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#ship_weight_bonus", value = self
                .r#ship_weight_bonus, max = 10f32
            );
            self.r#ship_weight_bonus = 10f32;
        }
        if self.r#equipment_weight_bonus < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#equipment_weight_bonus", value = self
                .r#equipment_weight_bonus, min = - 1f32
            );
            self.r#equipment_weight_bonus = -1f32;
        }
        if self.r#equipment_weight_bonus > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#equipment_weight_bonus", value = self
                .r#equipment_weight_bonus, max = 10f32
            );
            self.r#equipment_weight_bonus = 10f32;
        }
        if self.r#velocity_bonus < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#velocity_bonus", value = self
                .r#velocity_bonus, min = - 1f32
            );
            self.r#velocity_bonus = -1f32;
        }
        if self.r#velocity_bonus > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#velocity_bonus", value = self
                .r#velocity_bonus, max = 10f32
            );
            self.r#velocity_bonus = 10f32;
        }
        if self.r#turn_rate_bonus < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turn_rate_bonus", value = self
                .r#turn_rate_bonus, min = - 1f32
            );
            self.r#turn_rate_bonus = -1f32;
        }
        if self.r#turn_rate_bonus > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turn_rate_bonus", value = self
                .r#turn_rate_bonus, max = 10f32
            );
            self.r#turn_rate_bonus = 10f32;
        }
        if self.r#armor_bonus < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_bonus", value = self
                .r#armor_bonus, min = - 1f32
            );
            self.r#armor_bonus = -1f32;
        }
        if self.r#armor_bonus > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_bonus", value = self
                .r#armor_bonus, max = 10f32
            );
            self.r#armor_bonus = 10f32;
        }
        if self.r#shield_bonus < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_bonus", value = self
                .r#shield_bonus, min = - 1f32
            );
            self.r#shield_bonus = -1f32;
        }
        if self.r#shield_bonus > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_bonus", value = self
                .r#shield_bonus, max = 10f32
            );
            self.r#shield_bonus = 10f32;
        }
        if self.r#energy_bonus < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_bonus", value = self
                .r#energy_bonus, min = - 1f32
            );
            self.r#energy_bonus = -1f32;
        }
        if self.r#energy_bonus > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_bonus", value = self
                .r#energy_bonus, max = 10f32
            );
            self.r#energy_bonus = 10f32;
        }
    }
}
impl Default for ShipFeatures {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/StatModification.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_type(mut self, r#type: StatModificationType) -> Self {
        self.r#type = r#type;
        Self
    }
    pub fn with_gray_3(mut self, r#gray_3: f32) -> Self {
        self.r#gray_3 = r#gray_3;
        Self
    }
    pub fn with_gray_2(mut self, r#gray_2: f32) -> Self {
        self.r#gray_2 = r#gray_2;
        Self
    }
    pub fn with_gray_1(mut self, r#gray_1: f32) -> Self {
        self.r#gray_1 = r#gray_1;
        Self
    }
    pub fn with_green(mut self, r#green: f32) -> Self {
        self.r#green = r#green;
        Self
    }
    pub fn with_purple(mut self, r#purple: f32) -> Self {
        self.r#purple = r#purple;
        Self
    }
    pub fn with_gold(mut self, r#gold: f32) -> Self {
        self.r#gold = r#gold;
        Self
    }
}
impl DatabaseItem for StatModification {
    fn validate(&mut self) {}
}
impl Default for StatModification {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletBody.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletBody {
    pub r#size: f32,
    pub r#length: f32,
    pub r#velocity: f32,
    pub r#parent_velocity_effect: f32,
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
    pub r#ai_bullet_behavior: AiBulletBehavior,
    pub r#type: BulletTypeObsolete,
}
impl BulletBody {
    fn new() -> Self {
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
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_length(mut self, r#length: f32) -> Self {
        self.r#length = r#length;
        Self
    }
    pub fn with_velocity(mut self, r#velocity: f32) -> Self {
        self.r#velocity = r#velocity;
        Self
    }
    pub fn with_parent_velocity_effect(mut self, r#parent_velocity_effect: f32) -> Self {
        self.r#parent_velocity_effect = r#parent_velocity_effect;
        Self
    }
    pub fn with_attached_to_parent(mut self, r#attached_to_parent: bool) -> Self {
        self.r#attached_to_parent = r#attached_to_parent;
        Self
    }
    pub fn with_range(mut self, r#range: f32) -> Self {
        self.r#range = r#range;
        Self
    }
    pub fn with_lifetime(mut self, r#lifetime: f32) -> Self {
        self.r#lifetime = r#lifetime;
        Self
    }
    pub fn with_weight(mut self, r#weight: f32) -> Self {
        self.r#weight = r#weight;
        Self
    }
    pub fn with_hit_points(mut self, r#hit_points: i32) -> Self {
        self.r#hit_points = r#hit_points;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_bullet_prefab(
        mut self,
        r#bullet_prefab: Option<BulletPrefabId>,
    ) -> Self {
        self.r#bullet_prefab = r#bullet_prefab;
        Self
    }
    pub fn with_energy_cost(mut self, r#energy_cost: f32) -> Self {
        self.r#energy_cost = r#energy_cost;
        Self
    }
    pub fn with_can_be_disarmed(mut self, r#can_be_disarmed: bool) -> Self {
        self.r#can_be_disarmed = r#can_be_disarmed;
        Self
    }
    pub fn with_friendly_fire(mut self, r#friendly_fire: bool) -> Self {
        self.r#friendly_fire = r#friendly_fire;
        Self
    }
    pub fn with_ai_bullet_behavior(
        mut self,
        r#ai_bullet_behavior: AiBulletBehavior,
    ) -> Self {
        self.r#ai_bullet_behavior = r#ai_bullet_behavior;
        Self
    }
    pub fn with_type(mut self, r#type: BulletTypeObsolete) -> Self {
        self.r#type = r#type;
        Self
    }
}
impl DatabaseItem for BulletBody {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                1000f32
            );
            self.r#size = 1000f32;
        }
        if self.r#length < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#length", value = self.r#length, min =
                0f32
            );
            self.r#length = 0f32;
        }
        if self.r#length > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#length", value = self.r#length, max =
                1000f32
            );
            self.r#length = 1000f32;
        }
        if self.r#velocity < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#velocity", value = self.r#velocity, min
                = 0f32
            );
            self.r#velocity = 0f32;
        }
        if self.r#velocity > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#velocity", value = self.r#velocity, max
                = 1000f32
            );
            self.r#velocity = 1000f32;
        }
        if self.r#parent_velocity_effect < (-1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#parent_velocity_effect", value = self
                .r#parent_velocity_effect, min = - 1000f32
            );
            self.r#parent_velocity_effect = -1000f32;
        }
        if self.r#parent_velocity_effect > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#parent_velocity_effect", value = self
                .r#parent_velocity_effect, max = 1000f32
            );
            self.r#parent_velocity_effect = 1000f32;
        }
        if self.r#range < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, min =
                0f32
            );
            self.r#range = 0f32;
        }
        if self.r#range > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, max =
                1000000000f32
            );
            self.r#range = 1000000000f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, min
                = 0f32
            );
            self.r#lifetime = 0f32;
        }
        if self.r#lifetime > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, max
                = 1000000000f32
            );
            self.r#lifetime = 1000000000f32;
        }
        if self.r#weight < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, min =
                0f32
            );
            self.r#weight = 0f32;
        }
        if self.r#weight > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, max =
                1000000000f32
            );
            self.r#weight = 1000000000f32;
        }
        if self.r#hit_points < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#hit_points", value = self.r#hit_points,
                min = 0f32
            );
            self.r#hit_points = 0f32;
        }
        if self.r#hit_points > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#hit_points", value = self.r#hit_points,
                max = 1000000000f32
            );
            self.r#hit_points = 1000000000f32;
        }
        if self.r#energy_cost < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_cost", value = self
                .r#energy_cost, min = 0f32
            );
            self.r#energy_cost = 0f32;
        }
        if self.r#energy_cost > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_cost", value = self
                .r#energy_cost, max = 1000000000f32
            );
            self.r#energy_cost = 1000000000f32;
        }
        if self.r#type != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#type"
            );
        }
    }
}
impl Default for BulletBody {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletController.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
pub enum BulletController {
    Projectile(BulletControllerProjectile),
    Homing(BulletControllerHoming),
    Beam(BulletControllerBeam),
    Parametric(BulletControllerParametric),
}
impl From<BulletControllerProjectile> for BulletController {
    fn from(item: BulletControllerProjectile) -> Self {
        Self::Projectile(item)
    }
}
impl From<BulletControllerHoming> for BulletController {
    fn from(item: BulletControllerHoming) -> Self {
        Self::Homing(item)
    }
}
impl From<BulletControllerBeam> for BulletController {
    fn from(item: BulletControllerBeam) -> Self {
        Self::Beam(item)
    }
}
impl From<BulletControllerParametric> for BulletController {
    fn from(item: BulletControllerParametric) -> Self {
        Self::Parametric(item)
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletControllerParametric {
    pub r#x: String,
    pub r#y: String,
    pub r#rotation: String,
    pub r#size: String,
    pub r#length: String,
}
impl BulletControllerParametric {
    fn new() -> Self {
        Self {
            r#x: default,
            r#y: default,
            r#rotation: default,
            r#size: default,
            r#length: default,
        }
    }
    pub fn with_x(mut self, r#x: String) -> Self {
        self.r#x = r#x;
        Self
    }
    pub fn with_y(mut self, r#y: String) -> Self {
        self.r#y = r#y;
        Self
    }
    pub fn with_rotation(mut self, r#rotation: String) -> Self {
        self.r#rotation = r#rotation;
        Self
    }
    pub fn with_size(mut self, r#size: String) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_length(mut self, r#length: String) -> Self {
        self.r#length = r#length;
        Self
    }
}
impl DatabaseItem for BulletControllerParametric {
    fn validate(&mut self) {}
}
impl Default for BulletControllerParametric {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletControllerProjectile {}
impl BulletControllerProjectile {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BulletControllerProjectile {
    fn validate(&mut self) {}
}
impl Default for BulletControllerProjectile {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletControllerBeam {}
impl BulletControllerBeam {
    fn new() -> Self {
        Self {}
    }
}
impl DatabaseItem for BulletControllerBeam {
    fn validate(&mut self) {}
}
impl Default for BulletControllerBeam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletControllerHoming {
    pub r#starting_velocity_modifier: f32,
    pub r#ignore_rotation: bool,
    pub r#smart_aim: bool,
}
impl BulletControllerHoming {
    fn new() -> Self {
        Self {
            r#starting_velocity_modifier: 1f32,
            r#ignore_rotation: Default::default(),
            r#smart_aim: Default::default(),
        }
    }
    pub fn with_starting_velocity_modifier(
        mut self,
        r#starting_velocity_modifier: f32,
    ) -> Self {
        self.r#starting_velocity_modifier = r#starting_velocity_modifier;
        Self
    }
    pub fn with_ignore_rotation(mut self, r#ignore_rotation: bool) -> Self {
        self.r#ignore_rotation = r#ignore_rotation;
        Self
    }
    pub fn with_smart_aim(mut self, r#smart_aim: bool) -> Self {
        self.r#smart_aim = r#smart_aim;
        Self
    }
}
impl DatabaseItem for BulletControllerHoming {
    fn validate(&mut self) {
        if self.r#starting_velocity_modifier < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#starting_velocity_modifier", value =
                self.r#starting_velocity_modifier, min = 0f32
            );
            self.r#starting_velocity_modifier = 0f32;
        }
        if self.r#starting_velocity_modifier > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#starting_velocity_modifier", value =
                self.r#starting_velocity_modifier, max = 1000f32
            );
            self.r#starting_velocity_modifier = 1000f32;
        }
    }
}
impl Default for BulletControllerHoming {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletTrigger.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "EffectType")]
pub enum BulletTrigger {
    None(BulletTriggerNone),
    PlaySfx(BulletTriggerPlaySfx),
    SpawnBullet(BulletTriggerSpawnBullet),
    Detonate(BulletTriggerDetonate),
    SpawnStaticSfx(BulletTriggerSpawnStaticSfx),
    GravityField(BulletTriggerGravityField),
}
impl From<BulletTriggerNone> for BulletTrigger {
    fn from(item: BulletTriggerNone) -> Self {
        Self::None(item)
    }
}
impl From<BulletTriggerPlaySfx> for BulletTrigger {
    fn from(item: BulletTriggerPlaySfx) -> Self {
        Self::PlaySfx(item)
    }
}
impl From<BulletTriggerSpawnBullet> for BulletTrigger {
    fn from(item: BulletTriggerSpawnBullet) -> Self {
        Self::SpawnBullet(item)
    }
}
impl From<BulletTriggerDetonate> for BulletTrigger {
    fn from(item: BulletTriggerDetonate) -> Self {
        Self::Detonate(item)
    }
}
impl From<BulletTriggerSpawnStaticSfx> for BulletTrigger {
    fn from(item: BulletTriggerSpawnStaticSfx) -> Self {
        Self::SpawnStaticSfx(item)
    }
}
impl From<BulletTriggerGravityField> for BulletTrigger {
    fn from(item: BulletTriggerGravityField) -> Self {
        Self::GravityField(item)
    }
}
impl BulletTrigger {
    pub fn r#condition(&self) {
        match self {
            Self::None(x) => &x.r#condition,
            Self::PlaySfx(x) => &x.r#condition,
            Self::SpawnBullet(x) => &x.r#condition,
            Self::Detonate(x) => &x.r#condition,
            Self::SpawnStaticSfx(x) => &x.r#condition,
            Self::GravityField(x) => &x.r#condition,
        }
    }
    pub fn condition_mut(&mut self) {
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
    pub fn r#cooldown(&self) {
        match self {
            Self::None(x) => &x.r#cooldown,
            Self::PlaySfx(x) => &x.r#cooldown,
            Self::SpawnBullet(x) => &x.r#cooldown,
            Self::Detonate(x) => &x.r#cooldown,
            Self::SpawnStaticSfx(x) => &x.r#cooldown,
            Self::GravityField(x) => &x.r#cooldown,
        }
    }
    pub fn cooldown_mut(&mut self) {
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
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_condition(mut self, r#condition: BulletTriggerCondition) -> Self {
        self.r#condition = r#condition;
        Self
    }
    pub fn with_visual_effect(
        mut self,
        r#visual_effect: Option<VisualEffectId>,
    ) -> Self {
        self.r#visual_effect = r#visual_effect;
        Self
    }
    pub fn with_audio_clip(mut self, r#audio_clip: String) -> Self {
        self.r#audio_clip = r#audio_clip;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_color_mode(mut self, r#color_mode: ColorMode) -> Self {
        self.r#color_mode = r#color_mode;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_lifetime(mut self, r#lifetime: f32) -> Self {
        self.r#lifetime = r#lifetime;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BulletTriggerPlaySfx {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                100f32
            );
            self.r#size = 100f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, min
                = 0f32
            );
            self.r#lifetime = 0f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, max
                = 1000f32
            );
            self.r#lifetime = 1000f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, max
                = 1000f32
            );
            self.r#cooldown = 1000f32;
        }
    }
}
impl Default for BulletTriggerPlaySfx {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletTriggerGravityField {
    pub r#condition: BulletTriggerCondition,
    pub r#size: f32,
    pub r#cooldown: f32,
    pub r#power_multiplier: f32,
}
impl BulletTriggerGravityField {
    fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#size: Default::default(),
            r#cooldown: Default::default(),
            r#power_multiplier: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: BulletTriggerCondition) -> Self {
        self.r#condition = r#condition;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
    pub fn with_power_multiplier(mut self, r#power_multiplier: f32) -> Self {
        self.r#power_multiplier = r#power_multiplier;
        Self
    }
}
impl DatabaseItem for BulletTriggerGravityField {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                100f32
            );
            self.r#size = 100f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, max
                = 1000f32
            );
            self.r#cooldown = 1000f32;
        }
        if self.r#power_multiplier < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#power_multiplier", value = self
                .r#power_multiplier, min = 0f32
            );
            self.r#power_multiplier = 0f32;
        }
    }
}
impl Default for BulletTriggerGravityField {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
            r#rotation: default,
            r#offset_x: default,
            r#offset_y: default,
        }
    }
    pub fn with_condition(mut self, r#condition: BulletTriggerCondition) -> Self {
        self.r#condition = r#condition;
        Self
    }
    pub fn with_audio_clip(mut self, r#audio_clip: String) -> Self {
        self.r#audio_clip = r#audio_clip;
        Self
    }
    pub fn with_ammunition(mut self, r#ammunition: Option<AmmunitionId>) -> Self {
        self.r#ammunition = r#ammunition;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_color_mode(mut self, r#color_mode: ColorMode) -> Self {
        self.r#color_mode = r#color_mode;
        Self
    }
    pub fn with_quantity(mut self, r#quantity: i32) -> Self {
        self.r#quantity = r#quantity;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
    pub fn with_random_factor(mut self, r#random_factor: f32) -> Self {
        self.r#random_factor = r#random_factor;
        Self
    }
    pub fn with_power_multiplier(mut self, r#power_multiplier: f32) -> Self {
        self.r#power_multiplier = r#power_multiplier;
        Self
    }
    pub fn with_max_nesting_level(mut self, r#max_nesting_level: i32) -> Self {
        self.r#max_nesting_level = r#max_nesting_level;
        Self
    }
    pub fn with_rotation(mut self, r#rotation: String) -> Self {
        self.r#rotation = r#rotation;
        Self
    }
    pub fn with_offset_x(mut self, r#offset_x: String) -> Self {
        self.r#offset_x = r#offset_x;
        Self
    }
    pub fn with_offset_y(mut self, r#offset_y: String) -> Self {
        self.r#offset_y = r#offset_y;
        Self
    }
}
impl DatabaseItem for BulletTriggerSpawnBullet {
    fn validate(&mut self) {
        if self.r#quantity < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#quantity", value = self.r#quantity, min
                = 0f32
            );
            self.r#quantity = 0f32;
        }
        if self.r#quantity > (1000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#quantity", value = self.r#quantity, max
                = 1000f32
            );
            self.r#quantity = 1000f32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                100f32
            );
            self.r#size = 100f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, max
                = 1000f32
            );
            self.r#cooldown = 1000f32;
        }
        if self.r#random_factor < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#random_factor", value = self
                .r#random_factor, min = 0f32
            );
            self.r#random_factor = 0f32;
        }
        if self.r#random_factor > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#random_factor", value = self
                .r#random_factor, max = 1f32
            );
            self.r#random_factor = 1f32;
        }
        if self.r#power_multiplier < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#power_multiplier", value = self
                .r#power_multiplier, min = 0f32
            );
            self.r#power_multiplier = 0f32;
        }
        if self.r#max_nesting_level < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_nesting_level", value = self
                .r#max_nesting_level, min = 0f32
            );
            self.r#max_nesting_level = 0f32;
        }
        if self.r#max_nesting_level > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_nesting_level", value = self
                .r#max_nesting_level, max = 100f32
            );
            self.r#max_nesting_level = 100f32;
        }
    }
}
impl Default for BulletTriggerSpawnBullet {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletTriggerNone {
    pub r#condition: BulletTriggerCondition,
    pub r#cooldown: f32,
}
impl BulletTriggerNone {
    fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: BulletTriggerCondition) -> Self {
        self.r#condition = r#condition;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BulletTriggerNone {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, max
                = 1000f32
            );
            self.r#cooldown = 1000f32;
        }
    }
}
impl Default for BulletTriggerNone {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct BulletTriggerDetonate {
    pub r#condition: BulletTriggerCondition,
    pub r#cooldown: f32,
}
impl BulletTriggerDetonate {
    fn new() -> Self {
        Self {
            r#condition: Default::default(),
            r#cooldown: Default::default(),
        }
    }
    pub fn with_condition(mut self, r#condition: BulletTriggerCondition) -> Self {
        self.r#condition = r#condition;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BulletTriggerDetonate {
    fn validate(&mut self) {
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, max
                = 1000f32
            );
            self.r#cooldown = 1000f32;
        }
    }
}
impl Default for BulletTriggerDetonate {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_condition(mut self, r#condition: BulletTriggerCondition) -> Self {
        self.r#condition = r#condition;
        Self
    }
    pub fn with_visual_effect(
        mut self,
        r#visual_effect: Option<VisualEffectId>,
    ) -> Self {
        self.r#visual_effect = r#visual_effect;
        Self
    }
    pub fn with_audio_clip(mut self, r#audio_clip: String) -> Self {
        self.r#audio_clip = r#audio_clip;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_color_mode(mut self, r#color_mode: ColorMode) -> Self {
        self.r#color_mode = r#color_mode;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_lifetime(mut self, r#lifetime: f32) -> Self {
        self.r#lifetime = r#lifetime;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
}
impl DatabaseItem for BulletTriggerSpawnStaticSfx {
    fn validate(&mut self) {
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                100f32
            );
            self.r#size = 100f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, min
                = 0f32
            );
            self.r#lifetime = 0f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, max
                = 1000f32
            );
            self.r#lifetime = 1000f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, max
                = 1000f32
            );
            self.r#cooldown = 1000f32;
        }
    }
}
impl Default for BulletTriggerSpawnStaticSfx {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/ImpactEffect.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct ImpactEffect {
    pub r#type: ImpactEffectType,
    pub r#damage_type: DamageType,
    pub r#power: f32,
    pub r#factor: f32,
}
impl ImpactEffect {
    fn new() -> Self {
        Self {
            r#type: Default::default(),
            r#damage_type: Default::default(),
            r#power: Default::default(),
            r#factor: Default::default(),
        }
    }
    pub fn with_type(mut self, r#type: ImpactEffectType) -> Self {
        self.r#type = r#type;
        Self
    }
    pub fn with_damage_type(mut self, r#damage_type: DamageType) -> Self {
        self.r#damage_type = r#damage_type;
        Self
    }
    pub fn with_power(mut self, r#power: f32) -> Self {
        self.r#power = r#power;
        Self
    }
    pub fn with_factor(mut self, r#factor: f32) -> Self {
        self.r#factor = r#factor;
        Self
    }
}
impl DatabaseItem for ImpactEffect {
    fn validate(&mut self) {
        if self.r#power < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#power", value = self.r#power, min =
                0f32
            );
            self.r#power = 0f32;
        }
        if self.r#power > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#power", value = self.r#power, max =
                1000000000f32
            );
            self.r#power = 1000000000f32;
        }
        if self.r#factor < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#factor", value = self.r#factor, min =
                0f32
            );
            self.r#factor = 0f32;
        }
        if self.r#factor > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#factor", value = self.r#factor, max =
                1f32
            );
            self.r#factor = 1f32;
        }
    }
}
impl Default for ImpactEffect {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/VisualEffectElement.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_type(mut self, r#type: VisualEffectType) -> Self {
        self.r#type = r#type;
        Self
    }
    pub fn with_image(mut self, r#image: String) -> Self {
        self.r#image = r#image;
        Self
    }
    pub fn with_color_mode(mut self, r#color_mode: ColorMode) -> Self {
        self.r#color_mode = r#color_mode;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_quantity(mut self, r#quantity: i32) -> Self {
        self.r#quantity = r#quantity;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_growth_rate(mut self, r#growth_rate: f32) -> Self {
        self.r#growth_rate = r#growth_rate;
        Self
    }
    pub fn with_turn_rate(mut self, r#turn_rate: f32) -> Self {
        self.r#turn_rate = r#turn_rate;
        Self
    }
    pub fn with_start_time(mut self, r#start_time: f32) -> Self {
        self.r#start_time = r#start_time;
        Self
    }
    pub fn with_lifetime(mut self, r#lifetime: f32) -> Self {
        self.r#lifetime = r#lifetime;
        Self
    }
    pub fn with_particle_size(mut self, r#particle_size: f32) -> Self {
        self.r#particle_size = r#particle_size;
        Self
    }
    pub fn with_loop(mut self, r#loop: bool) -> Self {
        self.r#loop = r#loop;
        Self
    }
}
impl DatabaseItem for VisualEffectElement {
    fn validate(&mut self) {
        if self.r#quantity < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#quantity", value = self.r#quantity, min
                = 1f32
            );
            self.r#quantity = 1f32;
        }
        if self.r#quantity > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#quantity", value = self.r#quantity, max
                = 100f32
            );
            self.r#quantity = 100f32;
        }
        if self.r#size < (0.001f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min =
                0.001f32
            );
            self.r#size = 0.001f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                100f32
            );
            self.r#size = 100f32;
        }
        if self.r#growth_rate < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#growth_rate", value = self
                .r#growth_rate, min = - 1f32
            );
            self.r#growth_rate = -1f32;
        }
        if self.r#growth_rate > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#growth_rate", value = self
                .r#growth_rate, max = 100f32
            );
            self.r#growth_rate = 100f32;
        }
        if self.r#turn_rate < (-1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turn_rate", value = self.r#turn_rate,
                min = - 1000f32
            );
            self.r#turn_rate = -1000f32;
        }
        if self.r#turn_rate > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turn_rate", value = self.r#turn_rate,
                max = 1000f32
            );
            self.r#turn_rate = 1000f32;
        }
        if self.r#start_time < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#start_time", value = self.r#start_time,
                min = 0f32
            );
            self.r#start_time = 0f32;
        }
        if self.r#start_time > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#start_time", value = self.r#start_time,
                max = 1000f32
            );
            self.r#start_time = 1000f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, min
                = 0f32
            );
            self.r#lifetime = 0f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, max
                = 1000f32
            );
            self.r#lifetime = 1000f32;
        }
        if self.r#particle_size < (0.001f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#particle_size", value = self
                .r#particle_size, min = 0.001f32
            );
            self.r#particle_size = 0.001f32;
        }
        if self.r#particle_size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#particle_size", value = self
                .r#particle_size, max = 100f32
            );
            self.r#particle_size = 100f32;
        }
    }
}
impl Default for VisualEffectElement {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/CombatSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_enemy_ai(mut self, r#enemy_ai: Option<BehaviorTreeId>) -> Self {
        self.r#enemy_ai = r#enemy_ai;
        Self
    }
    pub fn with_autopilot_ai(mut self, r#autopilot_ai: Option<BehaviorTreeId>) -> Self {
        self.r#autopilot_ai = r#autopilot_ai;
        Self
    }
    pub fn with_clone_ai(mut self, r#clone_ai: Option<BehaviorTreeId>) -> Self {
        self.r#clone_ai = r#clone_ai;
        Self
    }
    pub fn with_defensive_drone_ai(
        mut self,
        r#defensive_drone_ai: Option<BehaviorTreeId>,
    ) -> Self {
        self.r#defensive_drone_ai = r#defensive_drone_ai;
        Self
    }
    pub fn with_offensive_drone_ai(
        mut self,
        r#offensive_drone_ai: Option<BehaviorTreeId>,
    ) -> Self {
        self.r#offensive_drone_ai = r#offensive_drone_ai;
        Self
    }
    pub fn with_starbase_ai(mut self, r#starbase_ai: Option<BehaviorTreeId>) -> Self {
        self.r#starbase_ai = r#starbase_ai;
        Self
    }
    pub fn with_default_combat_rules(
        mut self,
        r#default_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#default_combat_rules = r#default_combat_rules;
        Self
    }
}
impl DatabaseItem for CombatSettings {
    fn validate(&mut self) {}
}
impl Default for CombatSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/DatabaseSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct DatabaseSettings {
    pub r#database_version: i32,
    pub r#database_version_minor: i32,
    pub r#mod_name: String,
    pub r#mod_id: String,
    pub r#mod_version: i32,
    pub r#unload_original_database: bool,
}
impl DatabaseSettings {
    fn new() -> Self {
        Self {
            r#database_version: Default::default(),
            r#database_version_minor: Default::default(),
            r#mod_name: Default::default(),
            r#mod_id: Default::default(),
            r#mod_version: Default::default(),
            r#unload_original_database: Default::default(),
        }
    }
    pub fn with_database_version(mut self, r#database_version: i32) -> Self {
        self.r#database_version = r#database_version;
        Self
    }
    pub fn with_database_version_minor(mut self, r#database_version_minor: i32) -> Self {
        self.r#database_version_minor = r#database_version_minor;
        Self
    }
    pub fn with_mod_name(mut self, r#mod_name: String) -> Self {
        self.r#mod_name = r#mod_name;
        Self
    }
    pub fn with_mod_id(mut self, r#mod_id: String) -> Self {
        self.r#mod_id = r#mod_id;
        Self
    }
    pub fn with_mod_version(mut self, r#mod_version: i32) -> Self {
        self.r#mod_version = r#mod_version;
        Self
    }
    pub fn with_unload_original_database(
        mut self,
        r#unload_original_database: bool,
    ) -> Self {
        self.r#unload_original_database = r#unload_original_database;
        Self
    }
}
impl DatabaseItem for DatabaseSettings {
    fn validate(&mut self) {
        if self.r#database_version < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#database_version", value = self
                .r#database_version, min = 1f32
            );
            self.r#database_version = 1f32;
        }
        if self.r#database_version_minor < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#database_version_minor", value = self
                .r#database_version_minor, min = 0f32
            );
            self.r#database_version_minor = 0f32;
        }
    }
}
impl Default for DatabaseSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/DebugSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct DebugSettings {
    pub r#codes: Vec<DebugCode>,
    pub r#enable_debug_console: bool,
}
impl DebugSettings {
    fn new() -> Self {
        Self {
            r#codes: Default::default(),
            r#enable_debug_console: Default::default(),
        }
    }
    pub fn with_codes(mut self, r#codes: Vec<DebugCode>) -> Self {
        self.r#codes = r#codes;
        Self
    }
    pub fn with_enable_debug_console(mut self, r#enable_debug_console: bool) -> Self {
        self.r#enable_debug_console = r#enable_debug_console;
        Self
    }
}
impl DatabaseItem for DebugSettings {
    fn validate(&mut self) {}
}
impl Default for DebugSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ExplorationSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct ExplorationSettings {
    pub r#outpost_ship: Option<ShipId>,
    pub r#turret_ship: Option<ShipId>,
    pub r#infected_planet_faction: Option<FactionId>,
    pub r#hive_ship_build: Option<ShipBuildId>,
    pub r#gas_cloud_dps: String,
}
impl ExplorationSettings {
    fn new() -> Self {
        Self {
            r#outpost_ship: Default::default(),
            r#turret_ship: Default::default(),
            r#infected_planet_faction: Default::default(),
            r#hive_ship_build: Default::default(),
            r#gas_cloud_dps: default,
        }
    }
    pub fn with_outpost_ship(mut self, r#outpost_ship: Option<ShipId>) -> Self {
        self.r#outpost_ship = r#outpost_ship;
        Self
    }
    pub fn with_turret_ship(mut self, r#turret_ship: Option<ShipId>) -> Self {
        self.r#turret_ship = r#turret_ship;
        Self
    }
    pub fn with_infected_planet_faction(
        mut self,
        r#infected_planet_faction: Option<FactionId>,
    ) -> Self {
        self.r#infected_planet_faction = r#infected_planet_faction;
        Self
    }
    pub fn with_hive_ship_build(
        mut self,
        r#hive_ship_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#hive_ship_build = r#hive_ship_build;
        Self
    }
    pub fn with_gas_cloud_dps(mut self, r#gas_cloud_dps: String) -> Self {
        self.r#gas_cloud_dps = r#gas_cloud_dps;
        Self
    }
}
impl DatabaseItem for ExplorationSettings {
    fn validate(&mut self) {
        if self.r#gas_cloud_dps < (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#gas_cloud_dps", value = self
                .r#gas_cloud_dps, min = 1f32
            );
            self.r#gas_cloud_dps = 1f32;
        }
    }
}
impl Default for ExplorationSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/FactionsSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct FactionsSettings {
    pub r#starbase_initial_defense: String,
    pub r#starbase_min_defense: i32,
    pub r#defense_loss_per_enemy_defeated: i32,
}
impl FactionsSettings {
    fn new() -> Self {
        Self {
            r#starbase_initial_defense: default,
            r#starbase_min_defense: 50i32,
            r#defense_loss_per_enemy_defeated: 10i32,
        }
    }
    pub fn with_starbase_initial_defense(
        mut self,
        r#starbase_initial_defense: String,
    ) -> Self {
        self.r#starbase_initial_defense = r#starbase_initial_defense;
        Self
    }
    pub fn with_starbase_min_defense(mut self, r#starbase_min_defense: i32) -> Self {
        self.r#starbase_min_defense = r#starbase_min_defense;
        Self
    }
    pub fn with_defense_loss_per_enemy_defeated(
        mut self,
        r#defense_loss_per_enemy_defeated: i32,
    ) -> Self {
        self.r#defense_loss_per_enemy_defeated = r#defense_loss_per_enemy_defeated;
        Self
    }
}
impl DatabaseItem for FactionsSettings {
    fn validate(&mut self) {
        if self.r#starbase_initial_defense < (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#starbase_initial_defense", value = self
                .r#starbase_initial_defense, min = 1f32
            );
            self.r#starbase_initial_defense = 1f32;
        }
        if self.r#starbase_initial_defense > (100000f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#starbase_initial_defense", value = self
                .r#starbase_initial_defense, max = 100000f32
            );
            self.r#starbase_initial_defense = 100000f32;
        }
        if self.r#starbase_min_defense < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#starbase_min_defense", value = self
                .r#starbase_min_defense, min = 1f32
            );
            self.r#starbase_min_defense = 1f32;
        }
        if self.r#defense_loss_per_enemy_defeated < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#defense_loss_per_enemy_defeated", value
                = self.r#defense_loss_per_enemy_defeated, min = 0f32
            );
            self.r#defense_loss_per_enemy_defeated = 0f32;
        }
    }
}
impl Default for FactionsSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/FrontierSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
    pub fn with_base_command_points(mut self, r#base_command_points: i32) -> Self {
        self.r#base_command_points = r#base_command_points;
        Self
    }
    pub fn with_max_extra_command_points(
        mut self,
        r#max_extra_command_points: i32,
    ) -> Self {
        self.r#max_extra_command_points = r#max_extra_command_points;
        Self
    }
    pub fn with_supporter_pack_ship(
        mut self,
        r#supporter_pack_ship: Option<ShipId>,
    ) -> Self {
        self.r#supporter_pack_ship = r#supporter_pack_ship;
        Self
    }
    pub fn with_falcon_pack_ship(mut self, r#falcon_pack_ship: Option<ShipId>) -> Self {
        self.r#falcon_pack_ship = r#falcon_pack_ship;
        Self
    }
    pub fn with_big_boss_easy_build(
        mut self,
        r#big_boss_easy_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#big_boss_easy_build = r#big_boss_easy_build;
        Self
    }
    pub fn with_big_boss_normal_build(
        mut self,
        r#big_boss_normal_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#big_boss_normal_build = r#big_boss_normal_build;
        Self
    }
    pub fn with_big_boss_hard_build(
        mut self,
        r#big_boss_hard_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#big_boss_hard_build = r#big_boss_hard_build;
        Self
    }
    pub fn with_demo_scene_starbase_build(
        mut self,
        r#demo_scene_starbase_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#demo_scene_starbase_build = r#demo_scene_starbase_build;
        Self
    }
    pub fn with_tutorial_starbase_build(
        mut self,
        r#tutorial_starbase_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#tutorial_starbase_build = r#tutorial_starbase_build;
        Self
    }
    pub fn with_default_starbase_build(
        mut self,
        r#default_starbase_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#default_starbase_build = r#default_starbase_build;
        Self
    }
    pub fn with_exploration_starbase(
        mut self,
        r#exploration_starbase: Option<ShipId>,
    ) -> Self {
        self.r#exploration_starbase = r#exploration_starbase;
        Self
    }
    pub fn with_merchant_ship_build(
        mut self,
        r#merchant_ship_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#merchant_ship_build = r#merchant_ship_build;
        Self
    }
    pub fn with_smuggler_ship_build(
        mut self,
        r#smuggler_ship_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#smuggler_ship_build = r#smuggler_ship_build;
        Self
    }
    pub fn with_engineer_ship_build(
        mut self,
        r#engineer_ship_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#engineer_ship_build = r#engineer_ship_build;
        Self
    }
    pub fn with_mercenary_ship_build(
        mut self,
        r#mercenary_ship_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#mercenary_ship_build = r#mercenary_ship_build;
        Self
    }
    pub fn with_shipyard_ship_build(
        mut self,
        r#shipyard_ship_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#shipyard_ship_build = r#shipyard_ship_build;
        Self
    }
    pub fn with_santa_ship_build(
        mut self,
        r#santa_ship_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#santa_ship_build = r#santa_ship_build;
        Self
    }
    pub fn with_salvage_drone_build(
        mut self,
        r#salvage_drone_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#salvage_drone_build = r#salvage_drone_build;
        Self
    }
    pub fn with_custom_ship_levels(
        mut self,
        r#custom_ship_levels: Vec<ShipToValue>,
    ) -> Self {
        self.r#custom_ship_levels = r#custom_ship_levels;
        Self
    }
    pub fn with_custom_ship_prices(
        mut self,
        r#custom_ship_prices: Vec<ShipToValue>,
    ) -> Self {
        self.r#custom_ship_prices = r#custom_ship_prices;
        Self
    }
    pub fn with_exploration_ships(mut self, r#exploration_ships: Vec<ShipId>) -> Self {
        self.r#exploration_ships = r#exploration_ships;
        Self
    }
}
impl DatabaseItem for FrontierSettings {
    fn validate(&mut self) {
        if self.r#base_command_points < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_command_points", value = self
                .r#base_command_points, min = 0f32
            );
            self.r#base_command_points = 0f32;
        }
        if self.r#max_extra_command_points < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_extra_command_points", value = self
                .r#max_extra_command_points, min = 0f32
            );
            self.r#max_extra_command_points = 0f32;
        }
    }
}
impl Default for FrontierSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/GalaxySettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
        Self {
            r#abandoned_starbase_faction: Default::default(),
            r#starting_ship_builds: Default::default(),
            r#starting_inventory: Default::default(),
            r#supporter_pack_ship: Default::default(),
            r#default_starbase_build: Default::default(),
            r#max_enemy_ships_level: 300i32,
            r#enemy_level: default,
            r#ship_min_spawn_distance: default,
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
        r#abandoned_starbase_faction: Option<FactionId>,
    ) -> Self {
        self.r#abandoned_starbase_faction = r#abandoned_starbase_faction;
        Self
    }
    pub fn with_starting_ship_builds(
        mut self,
        r#starting_ship_builds: Vec<ShipBuildId>,
    ) -> Self {
        self.r#starting_ship_builds = r#starting_ship_builds;
        Self
    }
    pub fn with_starting_inventory(
        mut self,
        r#starting_inventory: Option<LootId>,
    ) -> Self {
        self.r#starting_inventory = r#starting_inventory;
        Self
    }
    pub fn with_supporter_pack_ship(
        mut self,
        r#supporter_pack_ship: Option<ShipBuildId>,
    ) -> Self {
        self.r#supporter_pack_ship = r#supporter_pack_ship;
        Self
    }
    pub fn with_default_starbase_build(
        mut self,
        r#default_starbase_build: Option<ShipBuildId>,
    ) -> Self {
        self.r#default_starbase_build = r#default_starbase_build;
        Self
    }
    pub fn with_max_enemy_ships_level(mut self, r#max_enemy_ships_level: i32) -> Self {
        self.r#max_enemy_ships_level = r#max_enemy_ships_level;
        Self
    }
    pub fn with_enemy_level(mut self, r#enemy_level: String) -> Self {
        self.r#enemy_level = r#enemy_level;
        Self
    }
    pub fn with_ship_min_spawn_distance(
        mut self,
        r#ship_min_spawn_distance: String,
    ) -> Self {
        self.r#ship_min_spawn_distance = r#ship_min_spawn_distance;
        Self
    }
    pub fn with_capture_starbase_quest(
        mut self,
        r#capture_starbase_quest: Option<QuestId>,
    ) -> Self {
        self.r#capture_starbase_quest = r#capture_starbase_quest;
        Self
    }
    pub fn with_starting_invenory(
        mut self,
        r#starting_invenory: Option<LootId>,
    ) -> Self {
        self.r#starting_invenory = r#starting_invenory;
        Self
    }
    pub fn with_survival_combat_rules(
        mut self,
        r#survival_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#survival_combat_rules = r#survival_combat_rules;
        Self
    }
    pub fn with_starbase_combat_rules(
        mut self,
        r#starbase_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#starbase_combat_rules = r#starbase_combat_rules;
        Self
    }
    pub fn with_flagship_combat_rules(
        mut self,
        r#flagship_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#flagship_combat_rules = r#flagship_combat_rules;
        Self
    }
    pub fn with_arena_combat_rules(
        mut self,
        r#arena_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#arena_combat_rules = r#arena_combat_rules;
        Self
    }
    pub fn with_challenge_combat_rules(
        mut self,
        r#challenge_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#challenge_combat_rules = r#challenge_combat_rules;
        Self
    }
    pub fn with_quick_combat_rules(
        mut self,
        r#quick_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#quick_combat_rules = r#quick_combat_rules;
        Self
    }
}
impl DatabaseItem for GalaxySettings {
    fn validate(&mut self) {
        if self.r#max_enemy_ships_level < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_enemy_ships_level", value = self
                .r#max_enemy_ships_level, min = 0f32
            );
            self.r#max_enemy_ships_level = 0f32;
        }
        if self.r#max_enemy_ships_level > (500f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_enemy_ships_level", value = self
                .r#max_enemy_ships_level, max = 500f32
            );
            self.r#max_enemy_ships_level = 500f32;
        }
        if self.r#enemy_level < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#enemy_level", value = self
                .r#enemy_level, min = 0f32
            );
            self.r#enemy_level = 0f32;
        }
        if self.r#enemy_level > (500f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#enemy_level", value = self
                .r#enemy_level, max = 500f32
            );
            self.r#enemy_level = 500f32;
        }
        if self.r#ship_min_spawn_distance < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#ship_min_spawn_distance", value = self
                .r#ship_min_spawn_distance, min = 0f32
            );
            self.r#ship_min_spawn_distance = 0f32;
        }
        if self.r#ship_min_spawn_distance > (1000f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#ship_min_spawn_distance", value = self
                .r#ship_min_spawn_distance, max = 1000f32
            );
            self.r#ship_min_spawn_distance = 1000f32;
        }
        if self.r#starting_invenory != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#starting_invenory"
            );
        }
    }
}
impl Default for GalaxySettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/MusicPlaylist.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct MusicPlaylist {
    pub r#main_menu_music: Vec<SoundTrack>,
    pub r#galaxy_map_music: Vec<SoundTrack>,
    pub r#combat_music: Vec<SoundTrack>,
    pub r#exploration_music: Vec<SoundTrack>,
}
impl MusicPlaylist {
    fn new() -> Self {
        Self {
            r#main_menu_music: Default::default(),
            r#galaxy_map_music: Default::default(),
            r#combat_music: Default::default(),
            r#exploration_music: Default::default(),
        }
    }
    pub fn with_main_menu_music(mut self, r#main_menu_music: Vec<SoundTrack>) -> Self {
        self.r#main_menu_music = r#main_menu_music;
        Self
    }
    pub fn with_galaxy_map_music(mut self, r#galaxy_map_music: Vec<SoundTrack>) -> Self {
        self.r#galaxy_map_music = r#galaxy_map_music;
        Self
    }
    pub fn with_combat_music(mut self, r#combat_music: Vec<SoundTrack>) -> Self {
        self.r#combat_music = r#combat_music;
        Self
    }
    pub fn with_exploration_music(
        mut self,
        r#exploration_music: Vec<SoundTrack>,
    ) -> Self {
        self.r#exploration_music = r#exploration_music;
        Self
    }
}
impl DatabaseItem for MusicPlaylist {
    fn validate(&mut self) {}
}
impl Default for MusicPlaylist {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ShipModSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
        r#remove_weapon_slot_mod: bool,
    ) -> Self {
        self.r#remove_weapon_slot_mod = r#remove_weapon_slot_mod;
        Self
    }
    pub fn with_heat_defense_value(mut self, r#heat_defense_value: f32) -> Self {
        self.r#heat_defense_value = r#heat_defense_value;
        Self
    }
    pub fn with_kinetic_defense_value(mut self, r#kinetic_defense_value: f32) -> Self {
        self.r#kinetic_defense_value = r#kinetic_defense_value;
        Self
    }
    pub fn with_energy_defense_value(mut self, r#energy_defense_value: f32) -> Self {
        self.r#energy_defense_value = r#energy_defense_value;
        Self
    }
    pub fn with_regeneration_value(mut self, r#regeneration_value: f32) -> Self {
        self.r#regeneration_value = r#regeneration_value;
        Self
    }
    pub fn with_regeneration_armor(mut self, r#regeneration_armor: f32) -> Self {
        self.r#regeneration_armor = r#regeneration_armor;
        Self
    }
    pub fn with_weight_reduction(mut self, r#weight_reduction: f32) -> Self {
        self.r#weight_reduction = r#weight_reduction;
        Self
    }
    pub fn with_attack_reduction(mut self, r#attack_reduction: f32) -> Self {
        self.r#attack_reduction = r#attack_reduction;
        Self
    }
}
impl DatabaseItem for ShipModSettings {
    fn validate(&mut self) {
        if self.r#heat_defense_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#heat_defense_value", value = self
                .r#heat_defense_value, min = 0f32
            );
            self.r#heat_defense_value = 0f32;
        }
        if self.r#heat_defense_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#heat_defense_value", value = self
                .r#heat_defense_value, max = 1f32
            );
            self.r#heat_defense_value = 1f32;
        }
        if self.r#kinetic_defense_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_defense_value", value = self
                .r#kinetic_defense_value, min = 0f32
            );
            self.r#kinetic_defense_value = 0f32;
        }
        if self.r#kinetic_defense_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_defense_value", value = self
                .r#kinetic_defense_value, max = 1f32
            );
            self.r#kinetic_defense_value = 1f32;
        }
        if self.r#energy_defense_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_defense_value", value = self
                .r#energy_defense_value, min = 0f32
            );
            self.r#energy_defense_value = 0f32;
        }
        if self.r#energy_defense_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_defense_value", value = self
                .r#energy_defense_value, max = 1f32
            );
            self.r#energy_defense_value = 1f32;
        }
        if self.r#regeneration_value < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#regeneration_value", value = self
                .r#regeneration_value, min = 0f32
            );
            self.r#regeneration_value = 0f32;
        }
        if self.r#regeneration_value > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#regeneration_value", value = self
                .r#regeneration_value, max = 1f32
            );
            self.r#regeneration_value = 1f32;
        }
        if self.r#regeneration_armor < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#regeneration_armor", value = self
                .r#regeneration_armor, min = 0f32
            );
            self.r#regeneration_armor = 0f32;
        }
        if self.r#regeneration_armor > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#regeneration_armor", value = self
                .r#regeneration_armor, max = 1f32
            );
            self.r#regeneration_armor = 1f32;
        }
        if self.r#weight_reduction < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight_reduction", value = self
                .r#weight_reduction, min = 0f32
            );
            self.r#weight_reduction = 0f32;
        }
        if self.r#weight_reduction > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight_reduction", value = self
                .r#weight_reduction, max = 1f32
            );
            self.r#weight_reduction = 1f32;
        }
        if self.r#attack_reduction < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#attack_reduction", value = self
                .r#attack_reduction, min = 0f32
            );
            self.r#attack_reduction = 0f32;
        }
        if self.r#attack_reduction > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#attack_reduction", value = self
                .r#attack_reduction, max = 1f32
            );
            self.r#attack_reduction = 1f32;
        }
    }
}
impl Default for ShipModSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/ShipSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
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
        r#default_weight_per_cell: f32,
    ) -> Self {
        self.r#default_weight_per_cell = r#default_weight_per_cell;
        Self
    }
    pub fn with_minimum_weight_per_cell(
        mut self,
        r#minimum_weight_per_cell: f32,
    ) -> Self {
        self.r#minimum_weight_per_cell = r#minimum_weight_per_cell;
        Self
    }
    pub fn with_base_armor_points(mut self, r#base_armor_points: f32) -> Self {
        self.r#base_armor_points = r#base_armor_points;
        Self
    }
    pub fn with_armor_points_per_cell(mut self, r#armor_points_per_cell: f32) -> Self {
        self.r#armor_points_per_cell = r#armor_points_per_cell;
        Self
    }
    pub fn with_armor_repair_cooldown(mut self, r#armor_repair_cooldown: f32) -> Self {
        self.r#armor_repair_cooldown = r#armor_repair_cooldown;
        Self
    }
    pub fn with_base_energy_points(mut self, r#base_energy_points: f32) -> Self {
        self.r#base_energy_points = r#base_energy_points;
        Self
    }
    pub fn with_base_energy_recharge_rate(
        mut self,
        r#base_energy_recharge_rate: f32,
    ) -> Self {
        self.r#base_energy_recharge_rate = r#base_energy_recharge_rate;
        Self
    }
    pub fn with_energy_recharge_cooldown(
        mut self,
        r#energy_recharge_cooldown: f32,
    ) -> Self {
        self.r#energy_recharge_cooldown = r#energy_recharge_cooldown;
        Self
    }
    pub fn with_base_shield_recharge_rate(
        mut self,
        r#base_shield_recharge_rate: f32,
    ) -> Self {
        self.r#base_shield_recharge_rate = r#base_shield_recharge_rate;
        Self
    }
    pub fn with_shield_recharge_cooldown(
        mut self,
        r#shield_recharge_cooldown: f32,
    ) -> Self {
        self.r#shield_recharge_cooldown = r#shield_recharge_cooldown;
        Self
    }
    pub fn with_base_drone_reconstruction_speed(
        mut self,
        r#base_drone_reconstruction_speed: f32,
    ) -> Self {
        self.r#base_drone_reconstruction_speed = r#base_drone_reconstruction_speed;
        Self
    }
    pub fn with_max_velocity(mut self, r#max_velocity: f32) -> Self {
        self.r#max_velocity = r#max_velocity;
        Self
    }
    pub fn with_max_turn_rate(mut self, r#max_turn_rate: f32) -> Self {
        self.r#max_turn_rate = r#max_turn_rate;
        Self
    }
}
impl DatabaseItem for ShipSettings {
    fn validate(&mut self) {
        if self.r#default_weight_per_cell < (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_weight_per_cell", value = self
                .r#default_weight_per_cell, min = 1f32
            );
            self.r#default_weight_per_cell = 1f32;
        }
        if self.r#default_weight_per_cell > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#default_weight_per_cell", value = self
                .r#default_weight_per_cell, max = 1000000f32
            );
            self.r#default_weight_per_cell = 1000000f32;
        }
        if self.r#minimum_weight_per_cell < (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#minimum_weight_per_cell", value = self
                .r#minimum_weight_per_cell, min = 1f32
            );
            self.r#minimum_weight_per_cell = 1f32;
        }
        if self.r#minimum_weight_per_cell > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#minimum_weight_per_cell", value = self
                .r#minimum_weight_per_cell, max = 1000000f32
            );
            self.r#minimum_weight_per_cell = 1000000f32;
        }
        if self.r#base_armor_points < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_armor_points", value = self
                .r#base_armor_points, min = 0f32
            );
            self.r#base_armor_points = 0f32;
        }
        if self.r#base_armor_points > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_armor_points", value = self
                .r#base_armor_points, max = 1000000f32
            );
            self.r#base_armor_points = 1000000f32;
        }
        if self.r#armor_points_per_cell < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_points_per_cell", value = self
                .r#armor_points_per_cell, min = 0f32
            );
            self.r#armor_points_per_cell = 0f32;
        }
        if self.r#armor_points_per_cell > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_points_per_cell", value = self
                .r#armor_points_per_cell, max = 1000000f32
            );
            self.r#armor_points_per_cell = 1000000f32;
        }
        if self.r#armor_repair_cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_repair_cooldown", value = self
                .r#armor_repair_cooldown, min = 0f32
            );
            self.r#armor_repair_cooldown = 0f32;
        }
        if self.r#armor_repair_cooldown > (60f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_repair_cooldown", value = self
                .r#armor_repair_cooldown, max = 60f32
            );
            self.r#armor_repair_cooldown = 60f32;
        }
        if self.r#base_energy_points < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_energy_points", value = self
                .r#base_energy_points, min = 0f32
            );
            self.r#base_energy_points = 0f32;
        }
        if self.r#base_energy_points > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_energy_points", value = self
                .r#base_energy_points, max = 1000000f32
            );
            self.r#base_energy_points = 1000000f32;
        }
        if self.r#base_energy_recharge_rate < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_energy_recharge_rate", value =
                self.r#base_energy_recharge_rate, min = 0f32
            );
            self.r#base_energy_recharge_rate = 0f32;
        }
        if self.r#base_energy_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_energy_recharge_rate", value =
                self.r#base_energy_recharge_rate, max = 1000000f32
            );
            self.r#base_energy_recharge_rate = 1000000f32;
        }
        if self.r#energy_recharge_cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_recharge_cooldown", value = self
                .r#energy_recharge_cooldown, min = 0f32
            );
            self.r#energy_recharge_cooldown = 0f32;
        }
        if self.r#energy_recharge_cooldown > (60f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_recharge_cooldown", value = self
                .r#energy_recharge_cooldown, max = 60f32
            );
            self.r#energy_recharge_cooldown = 60f32;
        }
        if self.r#base_shield_recharge_rate < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_shield_recharge_rate", value =
                self.r#base_shield_recharge_rate, min = 0f32
            );
            self.r#base_shield_recharge_rate = 0f32;
        }
        if self.r#base_shield_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_shield_recharge_rate", value =
                self.r#base_shield_recharge_rate, max = 1000000f32
            );
            self.r#base_shield_recharge_rate = 1000000f32;
        }
        if self.r#shield_recharge_cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_recharge_cooldown", value = self
                .r#shield_recharge_cooldown, min = 0f32
            );
            self.r#shield_recharge_cooldown = 0f32;
        }
        if self.r#shield_recharge_cooldown > (60f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_recharge_cooldown", value = self
                .r#shield_recharge_cooldown, max = 60f32
            );
            self.r#shield_recharge_cooldown = 60f32;
        }
        if self.r#base_drone_reconstruction_speed < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_drone_reconstruction_speed", value
                = self.r#base_drone_reconstruction_speed, min = 0f32
            );
            self.r#base_drone_reconstruction_speed = 0f32;
        }
        if self.r#base_drone_reconstruction_speed > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_drone_reconstruction_speed", value
                = self.r#base_drone_reconstruction_speed, max = 100f32
            );
            self.r#base_drone_reconstruction_speed = 100f32;
        }
        if self.r#max_velocity < (5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_velocity", value = self
                .r#max_velocity, min = 5f32
            );
            self.r#max_velocity = 5f32;
        }
        if self.r#max_velocity > (30f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_velocity", value = self
                .r#max_velocity, max = 30f32
            );
            self.r#max_velocity = 30f32;
        }
        if self.r#max_turn_rate < (5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_turn_rate", value = self
                .r#max_turn_rate, min = 5f32
            );
            self.r#max_turn_rate = 5f32;
        }
        if self.r#max_turn_rate > (30f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_turn_rate", value = self
                .r#max_turn_rate, max = 30f32
            );
            self.r#max_turn_rate = 30f32;
        }
    }
}
impl Default for ShipSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/SkillSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
        Self {
            r#beat_all_enemies_faction_list: Default::default(),
            r#disable_exceed_the_limits: Default::default(),
            r#fuel_tank_capacity: default,
            r#attack_bonus: default,
            r#defense_bonus: default,
            r#shield_strength_bonus: default,
            r#shield_recharge_bonus: default,
            r#experience_bonus: default,
            r#flight_speed: default,
            r#flight_range: default,
            r#exploration_loot_bonus: default,
            r#heat_resistance: default,
            r#kinetic_resistance: default,
            r#energy_resistance: default,
            r#merchant_price_factor: default,
            r#crafting_price_factor: default,
            r#crafting_level_reduction: default,
            r#max_player_ships_level: 100i32,
            r#increased_level_limit: 200i32,
            r#base_fuel_capacity: 100i32,
            r#base_flight_range: 1.5f32,
            r#base_flight_speed: 1f32,
        }
    }
    pub fn with_beat_all_enemies_faction_list(
        mut self,
        r#beat_all_enemies_faction_list: Vec<FactionId>,
    ) -> Self {
        self.r#beat_all_enemies_faction_list = r#beat_all_enemies_faction_list;
        Self
    }
    pub fn with_disable_exceed_the_limits(
        mut self,
        r#disable_exceed_the_limits: bool,
    ) -> Self {
        self.r#disable_exceed_the_limits = r#disable_exceed_the_limits;
        Self
    }
    pub fn with_fuel_tank_capacity(mut self, r#fuel_tank_capacity: String) -> Self {
        self.r#fuel_tank_capacity = r#fuel_tank_capacity;
        Self
    }
    pub fn with_attack_bonus(mut self, r#attack_bonus: String) -> Self {
        self.r#attack_bonus = r#attack_bonus;
        Self
    }
    pub fn with_defense_bonus(mut self, r#defense_bonus: String) -> Self {
        self.r#defense_bonus = r#defense_bonus;
        Self
    }
    pub fn with_shield_strength_bonus(
        mut self,
        r#shield_strength_bonus: String,
    ) -> Self {
        self.r#shield_strength_bonus = r#shield_strength_bonus;
        Self
    }
    pub fn with_shield_recharge_bonus(
        mut self,
        r#shield_recharge_bonus: String,
    ) -> Self {
        self.r#shield_recharge_bonus = r#shield_recharge_bonus;
        Self
    }
    pub fn with_experience_bonus(mut self, r#experience_bonus: String) -> Self {
        self.r#experience_bonus = r#experience_bonus;
        Self
    }
    pub fn with_flight_speed(mut self, r#flight_speed: String) -> Self {
        self.r#flight_speed = r#flight_speed;
        Self
    }
    pub fn with_flight_range(mut self, r#flight_range: String) -> Self {
        self.r#flight_range = r#flight_range;
        Self
    }
    pub fn with_exploration_loot_bonus(
        mut self,
        r#exploration_loot_bonus: String,
    ) -> Self {
        self.r#exploration_loot_bonus = r#exploration_loot_bonus;
        Self
    }
    pub fn with_heat_resistance(mut self, r#heat_resistance: String) -> Self {
        self.r#heat_resistance = r#heat_resistance;
        Self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: String) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance;
        Self
    }
    pub fn with_energy_resistance(mut self, r#energy_resistance: String) -> Self {
        self.r#energy_resistance = r#energy_resistance;
        Self
    }
    pub fn with_merchant_price_factor(
        mut self,
        r#merchant_price_factor: String,
    ) -> Self {
        self.r#merchant_price_factor = r#merchant_price_factor;
        Self
    }
    pub fn with_crafting_price_factor(
        mut self,
        r#crafting_price_factor: String,
    ) -> Self {
        self.r#crafting_price_factor = r#crafting_price_factor;
        Self
    }
    pub fn with_crafting_level_reduction(
        mut self,
        r#crafting_level_reduction: String,
    ) -> Self {
        self.r#crafting_level_reduction = r#crafting_level_reduction;
        Self
    }
    pub fn with_max_player_ships_level(mut self, r#max_player_ships_level: i32) -> Self {
        self.r#max_player_ships_level = r#max_player_ships_level;
        Self
    }
    pub fn with_increased_level_limit(mut self, r#increased_level_limit: i32) -> Self {
        self.r#increased_level_limit = r#increased_level_limit;
        Self
    }
    pub fn with_base_fuel_capacity(mut self, r#base_fuel_capacity: i32) -> Self {
        self.r#base_fuel_capacity = r#base_fuel_capacity;
        Self
    }
    pub fn with_base_flight_range(mut self, r#base_flight_range: f32) -> Self {
        self.r#base_flight_range = r#base_flight_range;
        Self
    }
    pub fn with_base_flight_speed(mut self, r#base_flight_speed: f32) -> Self {
        self.r#base_flight_speed = r#base_flight_speed;
        Self
    }
}
impl DatabaseItem for SkillSettings {
    fn validate(&mut self) {
        if self.r#fuel_tank_capacity < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#fuel_tank_capacity", value = self
                .r#fuel_tank_capacity, min = 0f32
            );
            self.r#fuel_tank_capacity = 0f32;
        }
        if self.r#attack_bonus < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#attack_bonus", value = self
                .r#attack_bonus, min = 0f32
            );
            self.r#attack_bonus = 0f32;
        }
        if self.r#defense_bonus < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#defense_bonus", value = self
                .r#defense_bonus, min = 0f32
            );
            self.r#defense_bonus = 0f32;
        }
        if self.r#shield_strength_bonus < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_strength_bonus", value = self
                .r#shield_strength_bonus, min = 0f32
            );
            self.r#shield_strength_bonus = 0f32;
        }
        if self.r#shield_recharge_bonus < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_recharge_bonus", value = self
                .r#shield_recharge_bonus, min = 0f32
            );
            self.r#shield_recharge_bonus = 0f32;
        }
        if self.r#experience_bonus < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#experience_bonus", value = self
                .r#experience_bonus, min = 0f32
            );
            self.r#experience_bonus = 0f32;
        }
        if self.r#flight_speed < (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#flight_speed", value = self
                .r#flight_speed, min = 1f32
            );
            self.r#flight_speed = 1f32;
        }
        if self.r#flight_range < (1.5f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#flight_range", value = self
                .r#flight_range, min = 1.5f32
            );
            self.r#flight_range = 1.5f32;
        }
        if self.r#exploration_loot_bonus < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#exploration_loot_bonus", value = self
                .r#exploration_loot_bonus, min = 0f32
            );
            self.r#exploration_loot_bonus = 0f32;
        }
        if self.r#heat_resistance < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#heat_resistance", value = self
                .r#heat_resistance, min = 0f32
            );
            self.r#heat_resistance = 0f32;
        }
        if self.r#heat_resistance > (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#heat_resistance", value = self
                .r#heat_resistance, max = 1f32
            );
            self.r#heat_resistance = 1f32;
        }
        if self.r#kinetic_resistance < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_resistance", value = self
                .r#kinetic_resistance, min = 0f32
            );
            self.r#kinetic_resistance = 0f32;
        }
        if self.r#kinetic_resistance > (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_resistance", value = self
                .r#kinetic_resistance, max = 1f32
            );
            self.r#kinetic_resistance = 1f32;
        }
        if self.r#energy_resistance < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_resistance", value = self
                .r#energy_resistance, min = 0f32
            );
            self.r#energy_resistance = 0f32;
        }
        if self.r#energy_resistance > (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_resistance", value = self
                .r#energy_resistance, max = 1f32
            );
            self.r#energy_resistance = 1f32;
        }
        if self.r#merchant_price_factor < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#merchant_price_factor", value = self
                .r#merchant_price_factor, min = 0f32
            );
            self.r#merchant_price_factor = 0f32;
        }
        if self.r#crafting_price_factor < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#crafting_price_factor", value = self
                .r#crafting_price_factor, min = 0f32
            );
            self.r#crafting_price_factor = 0f32;
        }
        if self.r#crafting_level_reduction < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#crafting_level_reduction", value = self
                .r#crafting_level_reduction, min = 0f32
            );
            self.r#crafting_level_reduction = 0f32;
        }
        if self.r#max_player_ships_level < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_player_ships_level", value = self
                .r#max_player_ships_level, min = 0f32
            );
            self.r#max_player_ships_level = 0f32;
        }
        if self.r#max_player_ships_level > (500f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_player_ships_level", value = self
                .r#max_player_ships_level, max = 500f32
            );
            self.r#max_player_ships_level = 500f32;
        }
        if self.r#increased_level_limit < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#increased_level_limit", value = self
                .r#increased_level_limit, min = 0f32
            );
            self.r#increased_level_limit = 0f32;
        }
        if self.r#increased_level_limit > (1000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#increased_level_limit", value = self
                .r#increased_level_limit, max = 1000f32
            );
            self.r#increased_level_limit = 1000f32;
        }
        if self.r#base_fuel_capacity < (10f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_fuel_capacity", value = self
                .r#base_fuel_capacity, min = 10f32
            );
            self.r#base_fuel_capacity = 10f32;
        }
        if self.r#base_flight_range < (1.5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_flight_range", value = self
                .r#base_flight_range, min = 1.5f32
            );
            self.r#base_flight_range = 1.5f32;
        }
        if self.r#base_flight_speed < (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_flight_speed", value = self
                .r#base_flight_speed, min = 1f32
            );
            self.r#base_flight_speed = 1f32;
        }
    }
}
impl Default for SkillSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/SpecialEventSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
        Self {
            r#enable_xmas_event: true,
            r#xmas_days_before: 24i32,
            r#xmas_days_after: 15i32,
            r#xmas_quest: Default::default(),
            r#xmas_combat_rules: Default::default(),
            r#convert_credits_to_snowflakes: default,
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
    pub fn with_enable_xmas_event(mut self, r#enable_xmas_event: bool) -> Self {
        self.r#enable_xmas_event = r#enable_xmas_event;
        Self
    }
    pub fn with_xmas_days_before(mut self, r#xmas_days_before: i32) -> Self {
        self.r#xmas_days_before = r#xmas_days_before;
        Self
    }
    pub fn with_xmas_days_after(mut self, r#xmas_days_after: i32) -> Self {
        self.r#xmas_days_after = r#xmas_days_after;
        Self
    }
    pub fn with_xmas_quest(mut self, r#xmas_quest: Option<QuestId>) -> Self {
        self.r#xmas_quest = r#xmas_quest;
        Self
    }
    pub fn with_xmas_combat_rules(
        mut self,
        r#xmas_combat_rules: Option<CombatRulesId>,
    ) -> Self {
        self.r#xmas_combat_rules = r#xmas_combat_rules;
        Self
    }
    pub fn with_convert_credits_to_snowflakes(
        mut self,
        r#convert_credits_to_snowflakes: String,
    ) -> Self {
        self.r#convert_credits_to_snowflakes = r#convert_credits_to_snowflakes;
        Self
    }
    pub fn with_enable_easter_event(mut self, r#enable_easter_event: bool) -> Self {
        self.r#enable_easter_event = r#enable_easter_event;
        Self
    }
    pub fn with_easter_days_before(mut self, r#easter_days_before: i32) -> Self {
        self.r#easter_days_before = r#easter_days_before;
        Self
    }
    pub fn with_easter_days_after(mut self, r#easter_days_after: i32) -> Self {
        self.r#easter_days_after = r#easter_days_after;
        Self
    }
    pub fn with_easter_quest(mut self, r#easter_quest: Option<QuestId>) -> Self {
        self.r#easter_quest = r#easter_quest;
        Self
    }
    pub fn with_enable_halloween_event(
        mut self,
        r#enable_halloween_event: bool,
    ) -> Self {
        self.r#enable_halloween_event = r#enable_halloween_event;
        Self
    }
    pub fn with_halloween_days_before(mut self, r#halloween_days_before: i32) -> Self {
        self.r#halloween_days_before = r#halloween_days_before;
        Self
    }
    pub fn with_halloween_days_after(mut self, r#halloween_days_after: i32) -> Self {
        self.r#halloween_days_after = r#halloween_days_after;
        Self
    }
    pub fn with_halloween_quest(mut self, r#halloween_quest: Option<QuestId>) -> Self {
        self.r#halloween_quest = r#halloween_quest;
        Self
    }
}
impl DatabaseItem for SpecialEventSettings {
    fn validate(&mut self) {
        if self.r#xmas_days_before < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#xmas_days_before", value = self
                .r#xmas_days_before, min = 0f32
            );
            self.r#xmas_days_before = 0f32;
        }
        if self.r#xmas_days_before > (30f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#xmas_days_before", value = self
                .r#xmas_days_before, max = 30f32
            );
            self.r#xmas_days_before = 30f32;
        }
        if self.r#xmas_days_after < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#xmas_days_after", value = self
                .r#xmas_days_after, min = 0f32
            );
            self.r#xmas_days_after = 0f32;
        }
        if self.r#xmas_days_after > (30f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#xmas_days_after", value = self
                .r#xmas_days_after, max = 30f32
            );
            self.r#xmas_days_after = 30f32;
        }
        if self.r#convert_credits_to_snowflakes < (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#convert_credits_to_snowflakes", value =
                self.r#convert_credits_to_snowflakes, min = 1f32
            );
            self.r#convert_credits_to_snowflakes = 1f32;
        }
        if self.r#easter_days_before < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#easter_days_before", value = self
                .r#easter_days_before, min = 0f32
            );
            self.r#easter_days_before = 0f32;
        }
        if self.r#easter_days_before > (30f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#easter_days_before", value = self
                .r#easter_days_before, max = 30f32
            );
            self.r#easter_days_before = 30f32;
        }
        if self.r#easter_days_after < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#easter_days_after", value = self
                .r#easter_days_after, min = 0f32
            );
            self.r#easter_days_after = 0f32;
        }
        if self.r#easter_days_after > (30f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#easter_days_after", value = self
                .r#easter_days_after, max = 30f32
            );
            self.r#easter_days_after = 30f32;
        }
        if self.r#halloween_days_before < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#halloween_days_before", value = self
                .r#halloween_days_before, min = 0f32
            );
            self.r#halloween_days_before = 0f32;
        }
        if self.r#halloween_days_before > (30f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#halloween_days_before", value = self
                .r#halloween_days_before, max = 30f32
            );
            self.r#halloween_days_before = 30f32;
        }
        if self.r#halloween_days_after < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#halloween_days_after", value = self
                .r#halloween_days_after, min = 0f32
            );
            self.r#halloween_days_after = 0f32;
        }
        if self.r#halloween_days_after > (30f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#halloween_days_after", value = self
                .r#halloween_days_after, max = 30f32
            );
            self.r#halloween_days_after = 30f32;
        }
    }
}
impl Default for SpecialEventSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Settings/UiSettings.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new() -> Self {
        Self {
            r#window_color: default,
            r#scroll_bar_color: default,
            r#icon_color: default,
            r#selection_color: default,
            r#button_color: default,
            r#button_focus_color: default,
            r#button_text_color: default,
            r#button_icon_color: default,
            r#warning_button_color: default,
            r#warning_button_focus_color: default,
            r#warning_button_text_color: default,
            r#warning_button_icon_color: default,
            r#premium_button_color: default,
            r#premium_button_focus_color: default,
            r#premium_button_text_color: default,
            r#premium_button_icon_color: default,
            r#text_color: default,
            r#error_text_color: default,
            r#header_text_color: default,
            r#pale_text_color: default,
            r#bright_text_color: default,
            r#background_dark: default,
            r#low_quality_item_color: default,
            r#common_quality_item_color: default,
            r#medium_quality_item_color: default,
            r#high_quality_item_color: default,
            r#perfect_quality_item_color: default,
            r#available_tech_color: default,
            r#unavailable_tech_color: default,
            r#obtained_tech_color: default,
            r#hidden_tech_color: default,
            r#credits_color: default,
            r#stars_color: default,
            r#money_color: default,
            r#fuel_color: default,
            r#tokens_color: default,
        }
    }
    pub fn with_window_color(mut self, r#window_color: String) -> Self {
        self.r#window_color = r#window_color;
        Self
    }
    pub fn with_scroll_bar_color(mut self, r#scroll_bar_color: String) -> Self {
        self.r#scroll_bar_color = r#scroll_bar_color;
        Self
    }
    pub fn with_icon_color(mut self, r#icon_color: String) -> Self {
        self.r#icon_color = r#icon_color;
        Self
    }
    pub fn with_selection_color(mut self, r#selection_color: String) -> Self {
        self.r#selection_color = r#selection_color;
        Self
    }
    pub fn with_button_color(mut self, r#button_color: String) -> Self {
        self.r#button_color = r#button_color;
        Self
    }
    pub fn with_button_focus_color(mut self, r#button_focus_color: String) -> Self {
        self.r#button_focus_color = r#button_focus_color;
        Self
    }
    pub fn with_button_text_color(mut self, r#button_text_color: String) -> Self {
        self.r#button_text_color = r#button_text_color;
        Self
    }
    pub fn with_button_icon_color(mut self, r#button_icon_color: String) -> Self {
        self.r#button_icon_color = r#button_icon_color;
        Self
    }
    pub fn with_warning_button_color(mut self, r#warning_button_color: String) -> Self {
        self.r#warning_button_color = r#warning_button_color;
        Self
    }
    pub fn with_warning_button_focus_color(
        mut self,
        r#warning_button_focus_color: String,
    ) -> Self {
        self.r#warning_button_focus_color = r#warning_button_focus_color;
        Self
    }
    pub fn with_warning_button_text_color(
        mut self,
        r#warning_button_text_color: String,
    ) -> Self {
        self.r#warning_button_text_color = r#warning_button_text_color;
        Self
    }
    pub fn with_warning_button_icon_color(
        mut self,
        r#warning_button_icon_color: String,
    ) -> Self {
        self.r#warning_button_icon_color = r#warning_button_icon_color;
        Self
    }
    pub fn with_premium_button_color(mut self, r#premium_button_color: String) -> Self {
        self.r#premium_button_color = r#premium_button_color;
        Self
    }
    pub fn with_premium_button_focus_color(
        mut self,
        r#premium_button_focus_color: String,
    ) -> Self {
        self.r#premium_button_focus_color = r#premium_button_focus_color;
        Self
    }
    pub fn with_premium_button_text_color(
        mut self,
        r#premium_button_text_color: String,
    ) -> Self {
        self.r#premium_button_text_color = r#premium_button_text_color;
        Self
    }
    pub fn with_premium_button_icon_color(
        mut self,
        r#premium_button_icon_color: String,
    ) -> Self {
        self.r#premium_button_icon_color = r#premium_button_icon_color;
        Self
    }
    pub fn with_text_color(mut self, r#text_color: String) -> Self {
        self.r#text_color = r#text_color;
        Self
    }
    pub fn with_error_text_color(mut self, r#error_text_color: String) -> Self {
        self.r#error_text_color = r#error_text_color;
        Self
    }
    pub fn with_header_text_color(mut self, r#header_text_color: String) -> Self {
        self.r#header_text_color = r#header_text_color;
        Self
    }
    pub fn with_pale_text_color(mut self, r#pale_text_color: String) -> Self {
        self.r#pale_text_color = r#pale_text_color;
        Self
    }
    pub fn with_bright_text_color(mut self, r#bright_text_color: String) -> Self {
        self.r#bright_text_color = r#bright_text_color;
        Self
    }
    pub fn with_background_dark(mut self, r#background_dark: String) -> Self {
        self.r#background_dark = r#background_dark;
        Self
    }
    pub fn with_low_quality_item_color(
        mut self,
        r#low_quality_item_color: String,
    ) -> Self {
        self.r#low_quality_item_color = r#low_quality_item_color;
        Self
    }
    pub fn with_common_quality_item_color(
        mut self,
        r#common_quality_item_color: String,
    ) -> Self {
        self.r#common_quality_item_color = r#common_quality_item_color;
        Self
    }
    pub fn with_medium_quality_item_color(
        mut self,
        r#medium_quality_item_color: String,
    ) -> Self {
        self.r#medium_quality_item_color = r#medium_quality_item_color;
        Self
    }
    pub fn with_high_quality_item_color(
        mut self,
        r#high_quality_item_color: String,
    ) -> Self {
        self.r#high_quality_item_color = r#high_quality_item_color;
        Self
    }
    pub fn with_perfect_quality_item_color(
        mut self,
        r#perfect_quality_item_color: String,
    ) -> Self {
        self.r#perfect_quality_item_color = r#perfect_quality_item_color;
        Self
    }
    pub fn with_available_tech_color(mut self, r#available_tech_color: String) -> Self {
        self.r#available_tech_color = r#available_tech_color;
        Self
    }
    pub fn with_unavailable_tech_color(
        mut self,
        r#unavailable_tech_color: String,
    ) -> Self {
        self.r#unavailable_tech_color = r#unavailable_tech_color;
        Self
    }
    pub fn with_obtained_tech_color(mut self, r#obtained_tech_color: String) -> Self {
        self.r#obtained_tech_color = r#obtained_tech_color;
        Self
    }
    pub fn with_hidden_tech_color(mut self, r#hidden_tech_color: String) -> Self {
        self.r#hidden_tech_color = r#hidden_tech_color;
        Self
    }
    pub fn with_credits_color(mut self, r#credits_color: String) -> Self {
        self.r#credits_color = r#credits_color;
        Self
    }
    pub fn with_stars_color(mut self, r#stars_color: String) -> Self {
        self.r#stars_color = r#stars_color;
        Self
    }
    pub fn with_money_color(mut self, r#money_color: String) -> Self {
        self.r#money_color = r#money_color;
        Self
    }
    pub fn with_fuel_color(mut self, r#fuel_color: String) -> Self {
        self.r#fuel_color = r#fuel_color;
        Self
    }
    pub fn with_tokens_color(mut self, r#tokens_color: String) -> Self {
        self.r#tokens_color = r#tokens_color;
        Self
    }
}
impl DatabaseItem for UiSettings {
    fn validate(&mut self) {}
}
impl Default for UiSettings {
    fn default() -> Self {
        Self::new()
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Ai/BehaviorTree.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorTree {
    pub r#id: BehaviorTreeId,
    pub r#root_node: BehaviorTreeNode,
}
impl BehaviorTree {
    fn new(r#id: BehaviorTreeId) -> Self {
        Self {
            r#id,
            r#root_node: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: BehaviorTreeId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_root_node(mut self, r#root_node: BehaviorTreeNode) -> Self {
        self.r#root_node = r#root_node;
        Self
    }
}
impl DatabaseItem for BehaviorTree {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/AmmunitionObsolete.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: AmmunitionObsoleteId) -> Self {
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
    pub fn with_id(mut self, r#id: AmmunitionObsoleteId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_ammunition_class(
        mut self,
        r#ammunition_class: AmmunitionClassObsolete,
    ) -> Self {
        self.r#ammunition_class = r#ammunition_class;
        Self
    }
    pub fn with_damage_type(mut self, r#damage_type: DamageType) -> Self {
        self.r#damage_type = r#damage_type;
        Self
    }
    pub fn with_impulse(mut self, r#impulse: f32) -> Self {
        self.r#impulse = r#impulse;
        Self
    }
    pub fn with_recoil(mut self, r#recoil: f32) -> Self {
        self.r#recoil = r#recoil;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_initial_position(mut self, r#initial_position: glam::f32::Vec2) -> Self {
        self.r#initial_position = r#initial_position;
        Self
    }
    pub fn with_area_of_effect(mut self, r#area_of_effect: f32) -> Self {
        self.r#area_of_effect = r#area_of_effect;
        Self
    }
    pub fn with_damage(mut self, r#damage: f32) -> Self {
        self.r#damage = r#damage;
        Self
    }
    pub fn with_range(mut self, r#range: f32) -> Self {
        self.r#range = r#range;
        Self
    }
    pub fn with_velocity(mut self, r#velocity: f32) -> Self {
        self.r#velocity = r#velocity;
        Self
    }
    pub fn with_life_time(mut self, r#life_time: f32) -> Self {
        self.r#life_time = r#life_time;
        Self
    }
    pub fn with_hit_points(mut self, r#hit_points: i32) -> Self {
        self.r#hit_points = r#hit_points;
        Self
    }
    pub fn with_ignores_ship_velocity(mut self, r#ignores_ship_velocity: bool) -> Self {
        self.r#ignores_ship_velocity = r#ignores_ship_velocity;
        Self
    }
    pub fn with_energy_cost(mut self, r#energy_cost: f32) -> Self {
        self.r#energy_cost = r#energy_cost;
        Self
    }
    pub fn with_coupled_ammunition_id(
        mut self,
        r#coupled_ammunition_id: Option<AmmunitionObsoleteId>,
    ) -> Self {
        self.r#coupled_ammunition_id = r#coupled_ammunition_id;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_fire_sound(mut self, r#fire_sound: String) -> Self {
        self.r#fire_sound = r#fire_sound;
        Self
    }
    pub fn with_hit_sound(mut self, r#hit_sound: String) -> Self {
        self.r#hit_sound = r#hit_sound;
        Self
    }
    pub fn with_hit_effect_prefab(mut self, r#hit_effect_prefab: String) -> Self {
        self.r#hit_effect_prefab = r#hit_effect_prefab;
        Self
    }
    pub fn with_bullet_prefab(mut self, r#bullet_prefab: String) -> Self {
        self.r#bullet_prefab = r#bullet_prefab;
        Self
    }
}
impl DatabaseItem for AmmunitionObsolete {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#impulse < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#impulse", value = self.r#impulse, min =
                0f32
            );
            self.r#impulse = 0f32;
        }
        if self.r#impulse > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#impulse", value = self.r#impulse, max =
                10f32
            );
            self.r#impulse = 10f32;
        }
        if self.r#recoil < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#recoil", value = self.r#recoil, min =
                0f32
            );
            self.r#recoil = 0f32;
        }
        if self.r#recoil > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#recoil", value = self.r#recoil, max =
                10f32
            );
            self.r#recoil = 10f32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                1000f32
            );
            self.r#size = 1000f32;
        }
        if self.r#area_of_effect < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#area_of_effect", value = self
                .r#area_of_effect, min = 0f32
            );
            self.r#area_of_effect = 0f32;
        }
        if self.r#area_of_effect > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#area_of_effect", value = self
                .r#area_of_effect, max = 1000f32
            );
            self.r#area_of_effect = 1000f32;
        }
        if self.r#damage < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#damage", value = self.r#damage, min =
                0f32
            );
            self.r#damage = 0f32;
        }
        if self.r#damage > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#damage", value = self.r#damage, max =
                1000000000f32
            );
            self.r#damage = 1000000000f32;
        }
        if self.r#range < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, min =
                0f32
            );
            self.r#range = 0f32;
        }
        if self.r#range > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, max =
                1000f32
            );
            self.r#range = 1000f32;
        }
        if self.r#velocity < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#velocity", value = self.r#velocity, min
                = 0f32
            );
            self.r#velocity = 0f32;
        }
        if self.r#velocity > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#velocity", value = self.r#velocity, max
                = 1000f32
            );
            self.r#velocity = 1000f32;
        }
        if self.r#life_time < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#life_time", value = self.r#life_time,
                min = 0f32
            );
            self.r#life_time = 0f32;
        }
        if self.r#life_time > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#life_time", value = self.r#life_time,
                max = 1000000000f32
            );
            self.r#life_time = 1000000000f32;
        }
        if self.r#hit_points < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#hit_points", value = self.r#hit_points,
                min = 0f32
            );
            self.r#hit_points = 0f32;
        }
        if self.r#hit_points > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#hit_points", value = self.r#hit_points,
                max = 1000000000f32
            );
            self.r#hit_points = 1000000000f32;
        }
        if self.r#energy_cost < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_cost", value = self
                .r#energy_cost, min = 0f32
            );
            self.r#energy_cost = 0f32;
        }
        if self.r#energy_cost > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_cost", value = self
                .r#energy_cost, max = 1000000000f32
            );
            self.r#energy_cost = 1000000000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Component.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: ComponentId, r#component_stats_id: ComponentStatsId) -> Self {
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
    pub fn with_id(mut self, r#id: ComponentId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_description(mut self, r#description: String) -> Self {
        self.r#description = r#description;
        Self
    }
    pub fn with_display_category(
        mut self,
        r#display_category: ComponentCategory,
    ) -> Self {
        self.r#display_category = r#display_category;
        Self
    }
    pub fn with_availability(mut self, r#availability: Availability) -> Self {
        self.r#availability = r#availability;
        Self
    }
    pub fn with_component_stats_id(
        mut self,
        r#component_stats_id: ComponentStatsId,
    ) -> Self {
        self.r#component_stats_id = r#component_stats_id;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
    pub fn with_level(mut self, r#level: i32) -> Self {
        self.r#level = r#level;
        Self
    }
    pub fn with_icon(mut self, r#icon: String) -> Self {
        self.r#icon = r#icon;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_layout(mut self, r#layout: String) -> Self {
        self.r#layout = r#layout;
        Self
    }
    pub fn with_cell_type(mut self, r#cell_type: String) -> Self {
        self.r#cell_type = r#cell_type;
        Self
    }
    pub fn with_device_id(mut self, r#device_id: Option<DeviceId>) -> Self {
        self.r#device_id = r#device_id;
        Self
    }
    pub fn with_weapon_id(mut self, r#weapon_id: Option<WeaponId>) -> Self {
        self.r#weapon_id = r#weapon_id;
        Self
    }
    pub fn with_ammunition_id(mut self, r#ammunition_id: Option<AmmunitionId>) -> Self {
        self.r#ammunition_id = r#ammunition_id;
        Self
    }
    pub fn with_weapon_slot_type(mut self, r#weapon_slot_type: String) -> Self {
        self.r#weapon_slot_type = r#weapon_slot_type;
        Self
    }
    pub fn with_drone_bay_id(mut self, r#drone_bay_id: Option<DroneBayId>) -> Self {
        self.r#drone_bay_id = r#drone_bay_id;
        Self
    }
    pub fn with_drone_id(mut self, r#drone_id: Option<ShipBuildId>) -> Self {
        self.r#drone_id = r#drone_id;
        Self
    }
    pub fn with_restrictions(mut self, r#restrictions: ComponentRestrictions) -> Self {
        self.r#restrictions = r#restrictions;
        Self
    }
    pub fn with_possible_modifications(
        mut self,
        r#possible_modifications: Vec<ComponentModId>,
    ) -> Self {
        self.r#possible_modifications = r#possible_modifications;
        Self
    }
}
impl DatabaseItem for Component {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#component_stats_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field =
                "r#component_stats_id"
            );
        }
        if self.r#level < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#level", value = self.r#level, min =
                0f32
            );
            self.r#level = 0f32;
        }
        if self.r#cell_type != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#cell_type"
            );
        }
        if self.r#weapon_slot_type != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#weapon_slot_type"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ComponentMod.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct ComponentMod {
    pub r#id: ComponentModId,
    pub r#description: String,
    pub r#modifications: Vec<StatModification>,
}
impl ComponentMod {
    fn new(r#id: ComponentModId) -> Self {
        Self {
            r#id,
            r#description: Default::default(),
            r#modifications: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: ComponentModId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_description(mut self, r#description: String) -> Self {
        self.r#description = r#description;
        Self
    }
    pub fn with_modifications(mut self, r#modifications: Vec<StatModification>) -> Self {
        self.r#modifications = r#modifications;
        Self
    }
}
impl DatabaseItem for ComponentMod {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ComponentStats.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: ComponentStatsId) -> Self {
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
    pub fn with_id(mut self, r#id: ComponentStatsId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_type(mut self, r#type: ComponentStatsType) -> Self {
        self.r#type = r#type;
        Self
    }
    pub fn with_armor_points(mut self, r#armor_points: f32) -> Self {
        self.r#armor_points = r#armor_points;
        Self
    }
    pub fn with_armor_repair_rate(mut self, r#armor_repair_rate: f32) -> Self {
        self.r#armor_repair_rate = r#armor_repair_rate;
        Self
    }
    pub fn with_armor_repair_cooldown_modifier(
        mut self,
        r#armor_repair_cooldown_modifier: f32,
    ) -> Self {
        self.r#armor_repair_cooldown_modifier = r#armor_repair_cooldown_modifier;
        Self
    }
    pub fn with_energy_points(mut self, r#energy_points: f32) -> Self {
        self.r#energy_points = r#energy_points;
        Self
    }
    pub fn with_energy_recharge_rate(mut self, r#energy_recharge_rate: f32) -> Self {
        self.r#energy_recharge_rate = r#energy_recharge_rate;
        Self
    }
    pub fn with_energy_recharge_cooldown_modifier(
        mut self,
        r#energy_recharge_cooldown_modifier: f32,
    ) -> Self {
        self.r#energy_recharge_cooldown_modifier = r#energy_recharge_cooldown_modifier;
        Self
    }
    pub fn with_shield_points(mut self, r#shield_points: f32) -> Self {
        self.r#shield_points = r#shield_points;
        Self
    }
    pub fn with_shield_recharge_rate(mut self, r#shield_recharge_rate: f32) -> Self {
        self.r#shield_recharge_rate = r#shield_recharge_rate;
        Self
    }
    pub fn with_shield_recharge_cooldown_modifier(
        mut self,
        r#shield_recharge_cooldown_modifier: f32,
    ) -> Self {
        self.r#shield_recharge_cooldown_modifier = r#shield_recharge_cooldown_modifier;
        Self
    }
    pub fn with_weight(mut self, r#weight: f32) -> Self {
        self.r#weight = r#weight;
        Self
    }
    pub fn with_ramming_damage(mut self, r#ramming_damage: f32) -> Self {
        self.r#ramming_damage = r#ramming_damage;
        Self
    }
    pub fn with_energy_absorption(mut self, r#energy_absorption: f32) -> Self {
        self.r#energy_absorption = r#energy_absorption;
        Self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: f32) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance;
        Self
    }
    pub fn with_energy_resistance(mut self, r#energy_resistance: f32) -> Self {
        self.r#energy_resistance = r#energy_resistance;
        Self
    }
    pub fn with_thermal_resistance(mut self, r#thermal_resistance: f32) -> Self {
        self.r#thermal_resistance = r#thermal_resistance;
        Self
    }
    pub fn with_engine_power(mut self, r#engine_power: f32) -> Self {
        self.r#engine_power = r#engine_power;
        Self
    }
    pub fn with_turn_rate(mut self, r#turn_rate: f32) -> Self {
        self.r#turn_rate = r#turn_rate;
        Self
    }
    pub fn with_autopilot(mut self, r#autopilot: bool) -> Self {
        self.r#autopilot = r#autopilot;
        Self
    }
    pub fn with_drone_range_modifier(mut self, r#drone_range_modifier: f32) -> Self {
        self.r#drone_range_modifier = r#drone_range_modifier;
        Self
    }
    pub fn with_drone_damage_modifier(mut self, r#drone_damage_modifier: f32) -> Self {
        self.r#drone_damage_modifier = r#drone_damage_modifier;
        Self
    }
    pub fn with_drone_defense_modifier(mut self, r#drone_defense_modifier: f32) -> Self {
        self.r#drone_defense_modifier = r#drone_defense_modifier;
        Self
    }
    pub fn with_drone_speed_modifier(mut self, r#drone_speed_modifier: f32) -> Self {
        self.r#drone_speed_modifier = r#drone_speed_modifier;
        Self
    }
    pub fn with_drones_built_per_second(
        mut self,
        r#drones_built_per_second: f32,
    ) -> Self {
        self.r#drones_built_per_second = r#drones_built_per_second;
        Self
    }
    pub fn with_drone_build_time_modifier(
        mut self,
        r#drone_build_time_modifier: f32,
    ) -> Self {
        self.r#drone_build_time_modifier = r#drone_build_time_modifier;
        Self
    }
    pub fn with_weapon_fire_rate_modifier(
        mut self,
        r#weapon_fire_rate_modifier: f32,
    ) -> Self {
        self.r#weapon_fire_rate_modifier = r#weapon_fire_rate_modifier;
        Self
    }
    pub fn with_weapon_damage_modifier(mut self, r#weapon_damage_modifier: f32) -> Self {
        self.r#weapon_damage_modifier = r#weapon_damage_modifier;
        Self
    }
    pub fn with_weapon_range_modifier(mut self, r#weapon_range_modifier: f32) -> Self {
        self.r#weapon_range_modifier = r#weapon_range_modifier;
        Self
    }
    pub fn with_weapon_energy_cost_modifier(
        mut self,
        r#weapon_energy_cost_modifier: f32,
    ) -> Self {
        self.r#weapon_energy_cost_modifier = r#weapon_energy_cost_modifier;
        Self
    }
    pub fn with_alter_weapon_platform(mut self, r#alter_weapon_platform: i32) -> Self {
        self.r#alter_weapon_platform = r#alter_weapon_platform;
        Self
    }
    pub fn with_auto_aiming_arc(mut self, r#auto_aiming_arc: f32) -> Self {
        self.r#auto_aiming_arc = r#auto_aiming_arc;
        Self
    }
    pub fn with_turret_turn_speed(mut self, r#turret_turn_speed: f32) -> Self {
        self.r#turret_turn_speed = r#turret_turn_speed;
        Self
    }
}
impl DatabaseItem for ComponentStats {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#armor_points < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_points", value = self
                .r#armor_points, min = - 1000000f32
            );
            self.r#armor_points = -1000000f32;
        }
        if self.r#armor_points > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_points", value = self
                .r#armor_points, max = 1000000f32
            );
            self.r#armor_points = 1000000f32;
        }
        if self.r#armor_repair_rate < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_repair_rate", value = self
                .r#armor_repair_rate, min = - 1000000f32
            );
            self.r#armor_repair_rate = -1000000f32;
        }
        if self.r#armor_repair_rate > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_repair_rate", value = self
                .r#armor_repair_rate, max = 1000000f32
            );
            self.r#armor_repair_rate = 1000000f32;
        }
        if self.r#armor_repair_cooldown_modifier < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_repair_cooldown_modifier", value
                = self.r#armor_repair_cooldown_modifier, min = - 1f32
            );
            self.r#armor_repair_cooldown_modifier = -1f32;
        }
        if self.r#armor_repair_cooldown_modifier > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#armor_repair_cooldown_modifier", value
                = self.r#armor_repair_cooldown_modifier, max = 1f32
            );
            self.r#armor_repair_cooldown_modifier = 1f32;
        }
        if self.r#energy_points < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_points", value = self
                .r#energy_points, min = - 1000000f32
            );
            self.r#energy_points = -1000000f32;
        }
        if self.r#energy_points > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_points", value = self
                .r#energy_points, max = 1000000f32
            );
            self.r#energy_points = 1000000f32;
        }
        if self.r#energy_recharge_rate < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_recharge_rate", value = self
                .r#energy_recharge_rate, min = - 1000000f32
            );
            self.r#energy_recharge_rate = -1000000f32;
        }
        if self.r#energy_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_recharge_rate", value = self
                .r#energy_recharge_rate, max = 1000000f32
            );
            self.r#energy_recharge_rate = 1000000f32;
        }
        if self.r#energy_recharge_cooldown_modifier < (-5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_recharge_cooldown_modifier",
                value = self.r#energy_recharge_cooldown_modifier, min = - 5f32
            );
            self.r#energy_recharge_cooldown_modifier = -5f32;
        }
        if self.r#energy_recharge_cooldown_modifier > (5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_recharge_cooldown_modifier",
                value = self.r#energy_recharge_cooldown_modifier, max = 5f32
            );
            self.r#energy_recharge_cooldown_modifier = 5f32;
        }
        if self.r#shield_points < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_points", value = self
                .r#shield_points, min = - 1000000f32
            );
            self.r#shield_points = -1000000f32;
        }
        if self.r#shield_points > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_points", value = self
                .r#shield_points, max = 1000000f32
            );
            self.r#shield_points = 1000000f32;
        }
        if self.r#shield_recharge_rate < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_recharge_rate", value = self
                .r#shield_recharge_rate, min = - 1000000f32
            );
            self.r#shield_recharge_rate = -1000000f32;
        }
        if self.r#shield_recharge_rate > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_recharge_rate", value = self
                .r#shield_recharge_rate, max = 1000000f32
            );
            self.r#shield_recharge_rate = 1000000f32;
        }
        if self.r#shield_recharge_cooldown_modifier < (-5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_recharge_cooldown_modifier",
                value = self.r#shield_recharge_cooldown_modifier, min = - 5f32
            );
            self.r#shield_recharge_cooldown_modifier = -5f32;
        }
        if self.r#shield_recharge_cooldown_modifier > (5f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#shield_recharge_cooldown_modifier",
                value = self.r#shield_recharge_cooldown_modifier, max = 5f32
            );
            self.r#shield_recharge_cooldown_modifier = 5f32;
        }
        if self.r#weight < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, min = -
                1000000f32
            );
            self.r#weight = -1000000f32;
        }
        if self.r#weight > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, max =
                1000000f32
            );
            self.r#weight = 1000000f32;
        }
        if self.r#ramming_damage < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#ramming_damage", value = self
                .r#ramming_damage, min = - 1000000f32
            );
            self.r#ramming_damage = -1000000f32;
        }
        if self.r#ramming_damage > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#ramming_damage", value = self
                .r#ramming_damage, max = 1000000f32
            );
            self.r#ramming_damage = 1000000f32;
        }
        if self.r#energy_absorption < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_absorption", value = self
                .r#energy_absorption, min = - 1000000f32
            );
            self.r#energy_absorption = -1000000f32;
        }
        if self.r#energy_absorption > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_absorption", value = self
                .r#energy_absorption, max = 1000000f32
            );
            self.r#energy_absorption = 1000000f32;
        }
        if self.r#kinetic_resistance < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_resistance", value = self
                .r#kinetic_resistance, min = - 1000000f32
            );
            self.r#kinetic_resistance = -1000000f32;
        }
        if self.r#kinetic_resistance > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#kinetic_resistance", value = self
                .r#kinetic_resistance, max = 1000000f32
            );
            self.r#kinetic_resistance = 1000000f32;
        }
        if self.r#energy_resistance < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_resistance", value = self
                .r#energy_resistance, min = - 1000000f32
            );
            self.r#energy_resistance = -1000000f32;
        }
        if self.r#energy_resistance > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_resistance", value = self
                .r#energy_resistance, max = 1000000f32
            );
            self.r#energy_resistance = 1000000f32;
        }
        if self.r#thermal_resistance < (-1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#thermal_resistance", value = self
                .r#thermal_resistance, min = - 1000000f32
            );
            self.r#thermal_resistance = -1000000f32;
        }
        if self.r#thermal_resistance > (1000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#thermal_resistance", value = self
                .r#thermal_resistance, max = 1000000f32
            );
            self.r#thermal_resistance = 1000000f32;
        }
        if self.r#engine_power < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#engine_power", value = self
                .r#engine_power, min = 0f32
            );
            self.r#engine_power = 0f32;
        }
        if self.r#engine_power > (2000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#engine_power", value = self
                .r#engine_power, max = 2000f32
            );
            self.r#engine_power = 2000f32;
        }
        if self.r#turn_rate < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turn_rate", value = self.r#turn_rate,
                min = 0f32
            );
            self.r#turn_rate = 0f32;
        }
        if self.r#turn_rate > (2000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turn_rate", value = self.r#turn_rate,
                max = 2000f32
            );
            self.r#turn_rate = 2000f32;
        }
        if self.r#drone_range_modifier < (-50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_range_modifier", value = self
                .r#drone_range_modifier, min = - 50f32
            );
            self.r#drone_range_modifier = -50f32;
        }
        if self.r#drone_range_modifier > (50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_range_modifier", value = self
                .r#drone_range_modifier, max = 50f32
            );
            self.r#drone_range_modifier = 50f32;
        }
        if self.r#drone_damage_modifier < (-50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_damage_modifier", value = self
                .r#drone_damage_modifier, min = - 50f32
            );
            self.r#drone_damage_modifier = -50f32;
        }
        if self.r#drone_damage_modifier > (50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_damage_modifier", value = self
                .r#drone_damage_modifier, max = 50f32
            );
            self.r#drone_damage_modifier = 50f32;
        }
        if self.r#drone_defense_modifier < (-50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_defense_modifier", value = self
                .r#drone_defense_modifier, min = - 50f32
            );
            self.r#drone_defense_modifier = -50f32;
        }
        if self.r#drone_defense_modifier > (50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_defense_modifier", value = self
                .r#drone_defense_modifier, max = 50f32
            );
            self.r#drone_defense_modifier = 50f32;
        }
        if self.r#drone_speed_modifier < (-50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_speed_modifier", value = self
                .r#drone_speed_modifier, min = - 50f32
            );
            self.r#drone_speed_modifier = -50f32;
        }
        if self.r#drone_speed_modifier > (50f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_speed_modifier", value = self
                .r#drone_speed_modifier, max = 50f32
            );
            self.r#drone_speed_modifier = 50f32;
        }
        if self.r#drones_built_per_second < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drones_built_per_second", value = self
                .r#drones_built_per_second, min = 0f32
            );
            self.r#drones_built_per_second = 0f32;
        }
        if self.r#drones_built_per_second > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drones_built_per_second", value = self
                .r#drones_built_per_second, max = 100f32
            );
            self.r#drones_built_per_second = 100f32;
        }
        if self.r#drone_build_time_modifier < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_build_time_modifier", value =
                self.r#drone_build_time_modifier, min = 0f32
            );
            self.r#drone_build_time_modifier = 0f32;
        }
        if self.r#drone_build_time_modifier > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#drone_build_time_modifier", value =
                self.r#drone_build_time_modifier, max = 100f32
            );
            self.r#drone_build_time_modifier = 100f32;
        }
        if self.r#weapon_fire_rate_modifier < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_fire_rate_modifier", value =
                self.r#weapon_fire_rate_modifier, min = - 100f32
            );
            self.r#weapon_fire_rate_modifier = -100f32;
        }
        if self.r#weapon_fire_rate_modifier > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_fire_rate_modifier", value =
                self.r#weapon_fire_rate_modifier, max = 100f32
            );
            self.r#weapon_fire_rate_modifier = 100f32;
        }
        if self.r#weapon_damage_modifier < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_damage_modifier", value = self
                .r#weapon_damage_modifier, min = - 100f32
            );
            self.r#weapon_damage_modifier = -100f32;
        }
        if self.r#weapon_damage_modifier > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_damage_modifier", value = self
                .r#weapon_damage_modifier, max = 100f32
            );
            self.r#weapon_damage_modifier = 100f32;
        }
        if self.r#weapon_range_modifier < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_range_modifier", value = self
                .r#weapon_range_modifier, min = - 100f32
            );
            self.r#weapon_range_modifier = -100f32;
        }
        if self.r#weapon_range_modifier > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_range_modifier", value = self
                .r#weapon_range_modifier, max = 100f32
            );
            self.r#weapon_range_modifier = 100f32;
        }
        if self.r#weapon_energy_cost_modifier < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_energy_cost_modifier", value =
                self.r#weapon_energy_cost_modifier, min = - 100f32
            );
            self.r#weapon_energy_cost_modifier = -100f32;
        }
        if self.r#weapon_energy_cost_modifier > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weapon_energy_cost_modifier", value =
                self.r#weapon_energy_cost_modifier, max = 100f32
            );
            self.r#weapon_energy_cost_modifier = 100f32;
        }
        if self.r#alter_weapon_platform != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#alter_weapon_platform"
            );
        }
        if self.r#auto_aiming_arc < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#auto_aiming_arc", value = self
                .r#auto_aiming_arc, min = 0f32
            );
            self.r#auto_aiming_arc = 0f32;
        }
        if self.r#auto_aiming_arc > (360f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#auto_aiming_arc", value = self
                .r#auto_aiming_arc, max = 360f32
            );
            self.r#auto_aiming_arc = 360f32;
        }
        if self.r#turret_turn_speed < (-1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turret_turn_speed", value = self
                .r#turret_turn_speed, min = - 1000f32
            );
            self.r#turret_turn_speed = -1000f32;
        }
        if self.r#turret_turn_speed > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#turret_turn_speed", value = self
                .r#turret_turn_speed, max = 1000f32
            );
            self.r#turret_turn_speed = 1000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Device.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: DeviceId) -> Self {
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
    pub fn with_id(mut self, r#id: DeviceId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_device_class(mut self, r#device_class: DeviceClass) -> Self {
        self.r#device_class = r#device_class;
        Self
    }
    pub fn with_energy_consumption(mut self, r#energy_consumption: f32) -> Self {
        self.r#energy_consumption = r#energy_consumption;
        Self
    }
    pub fn with_passive_energy_consumption(
        mut self,
        r#passive_energy_consumption: f32,
    ) -> Self {
        self.r#passive_energy_consumption = r#passive_energy_consumption;
        Self
    }
    pub fn with_power(mut self, r#power: f32) -> Self {
        self.r#power = r#power;
        Self
    }
    pub fn with_range(mut self, r#range: f32) -> Self {
        self.r#range = r#range;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_cooldown(mut self, r#cooldown: f32) -> Self {
        self.r#cooldown = r#cooldown;
        Self
    }
    pub fn with_lifetime(mut self, r#lifetime: f32) -> Self {
        self.r#lifetime = r#lifetime;
        Self
    }
    pub fn with_offset(mut self, r#offset: glam::f32::Vec2) -> Self {
        self.r#offset = r#offset;
        Self
    }
    pub fn with_activation_type(mut self, r#activation_type: ActivationType) -> Self {
        self.r#activation_type = r#activation_type;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_sound(mut self, r#sound: String) -> Self {
        self.r#sound = r#sound;
        Self
    }
    pub fn with_effect_prefab(mut self, r#effect_prefab: String) -> Self {
        self.r#effect_prefab = r#effect_prefab;
        Self
    }
    pub fn with_object_prefab(mut self, r#object_prefab: String) -> Self {
        self.r#object_prefab = r#object_prefab;
        Self
    }
    pub fn with_prefab(mut self, r#prefab: Option<GameObjectPrefabId>) -> Self {
        self.r#prefab = r#prefab;
        Self
    }
    pub fn with_control_button_icon(mut self, r#control_button_icon: String) -> Self {
        self.r#control_button_icon = r#control_button_icon;
        Self
    }
}
impl DatabaseItem for Device {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#energy_consumption < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_consumption", value = self
                .r#energy_consumption, min = 0f32
            );
            self.r#energy_consumption = 0f32;
        }
        if self.r#energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_consumption", value = self
                .r#energy_consumption, max = 1000000000f32
            );
            self.r#energy_consumption = 1000000000f32;
        }
        if self.r#passive_energy_consumption < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#passive_energy_consumption", value =
                self.r#passive_energy_consumption, min = 0f32
            );
            self.r#passive_energy_consumption = 0f32;
        }
        if self.r#passive_energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#passive_energy_consumption", value =
                self.r#passive_energy_consumption, max = 1000000000f32
            );
            self.r#passive_energy_consumption = 1000000000f32;
        }
        if self.r#power < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#power", value = self.r#power, min =
                0f32
            );
            self.r#power = 0f32;
        }
        if self.r#power > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#power", value = self.r#power, max =
                1000f32
            );
            self.r#power = 1000f32;
        }
        if self.r#range < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, min =
                0f32
            );
            self.r#range = 0f32;
        }
        if self.r#range > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, max =
                1000f32
            );
            self.r#range = 1000f32;
        }
        if self.r#size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min = 0f32
            );
            self.r#size = 0f32;
        }
        if self.r#size > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                1000f32
            );
            self.r#size = 1000f32;
        }
        if self.r#cooldown < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, min
                = 0f32
            );
            self.r#cooldown = 0f32;
        }
        if self.r#cooldown > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#cooldown", value = self.r#cooldown, max
                = 1000f32
            );
            self.r#cooldown = 1000f32;
        }
        if self.r#lifetime < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, min
                = 0f32
            );
            self.r#lifetime = 0f32;
        }
        if self.r#lifetime > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#lifetime", value = self.r#lifetime, max
                = 1000f32
            );
            self.r#lifetime = 1000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/DroneBay.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: DroneBayId) -> Self {
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
    pub fn with_id(mut self, r#id: DroneBayId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_energy_consumption(mut self, r#energy_consumption: f32) -> Self {
        self.r#energy_consumption = r#energy_consumption;
        Self
    }
    pub fn with_passive_energy_consumption(
        mut self,
        r#passive_energy_consumption: f32,
    ) -> Self {
        self.r#passive_energy_consumption = r#passive_energy_consumption;
        Self
    }
    pub fn with_range(mut self, r#range: f32) -> Self {
        self.r#range = r#range;
        Self
    }
    pub fn with_damage_multiplier(mut self, r#damage_multiplier: f32) -> Self {
        self.r#damage_multiplier = r#damage_multiplier;
        Self
    }
    pub fn with_defense_multiplier(mut self, r#defense_multiplier: f32) -> Self {
        self.r#defense_multiplier = r#defense_multiplier;
        Self
    }
    pub fn with_speed_multiplier(mut self, r#speed_multiplier: f32) -> Self {
        self.r#speed_multiplier = r#speed_multiplier;
        Self
    }
    pub fn with_build_extra_cycles(mut self, r#build_extra_cycles: i32) -> Self {
        self.r#build_extra_cycles = r#build_extra_cycles;
        Self
    }
    pub fn with_improved_ai(mut self, r#improved_ai: bool) -> Self {
        self.r#improved_ai = r#improved_ai;
        Self
    }
    pub fn with_capacity(mut self, r#capacity: i32) -> Self {
        self.r#capacity = r#capacity;
        Self
    }
    pub fn with_activation_type(mut self, r#activation_type: ActivationType) -> Self {
        self.r#activation_type = r#activation_type;
        Self
    }
    pub fn with_launch_sound(mut self, r#launch_sound: String) -> Self {
        self.r#launch_sound = r#launch_sound;
        Self
    }
    pub fn with_launch_effect_prefab(mut self, r#launch_effect_prefab: String) -> Self {
        self.r#launch_effect_prefab = r#launch_effect_prefab;
        Self
    }
    pub fn with_control_button_icon(mut self, r#control_button_icon: String) -> Self {
        self.r#control_button_icon = r#control_button_icon;
        Self
    }
    pub fn with_defensive_drone_ai(
        mut self,
        r#defensive_drone_ai: Option<BehaviorTreeId>,
    ) -> Self {
        self.r#defensive_drone_ai = r#defensive_drone_ai;
        Self
    }
    pub fn with_offensive_drone_ai(
        mut self,
        r#offensive_drone_ai: Option<BehaviorTreeId>,
    ) -> Self {
        self.r#offensive_drone_ai = r#offensive_drone_ai;
        Self
    }
}
impl DatabaseItem for DroneBay {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#energy_consumption < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_consumption", value = self
                .r#energy_consumption, min = 0f32
            );
            self.r#energy_consumption = 0f32;
        }
        if self.r#energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#energy_consumption", value = self
                .r#energy_consumption, max = 1000000000f32
            );
            self.r#energy_consumption = 1000000000f32;
        }
        if self.r#passive_energy_consumption < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#passive_energy_consumption", value =
                self.r#passive_energy_consumption, min = 0f32
            );
            self.r#passive_energy_consumption = 0f32;
        }
        if self.r#passive_energy_consumption > (1000000000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#passive_energy_consumption", value =
                self.r#passive_energy_consumption, max = 1000000000f32
            );
            self.r#passive_energy_consumption = 1000000000f32;
        }
        if self.r#range < (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, min =
                1f32
            );
            self.r#range = 1f32;
        }
        if self.r#range > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#range", value = self.r#range, max =
                1000f32
            );
            self.r#range = 1000f32;
        }
        if self.r#damage_multiplier < (0.01f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#damage_multiplier", value = self
                .r#damage_multiplier, min = 0.01f32
            );
            self.r#damage_multiplier = 0.01f32;
        }
        if self.r#damage_multiplier > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#damage_multiplier", value = self
                .r#damage_multiplier, max = 1000f32
            );
            self.r#damage_multiplier = 1000f32;
        }
        if self.r#defense_multiplier < (0.01f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#defense_multiplier", value = self
                .r#defense_multiplier, min = 0.01f32
            );
            self.r#defense_multiplier = 0.01f32;
        }
        if self.r#defense_multiplier > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#defense_multiplier", value = self
                .r#defense_multiplier, max = 1000f32
            );
            self.r#defense_multiplier = 1000f32;
        }
        if self.r#speed_multiplier < (0.01f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#speed_multiplier", value = self
                .r#speed_multiplier, min = 0.01f32
            );
            self.r#speed_multiplier = 0.01f32;
        }
        if self.r#speed_multiplier > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#speed_multiplier", value = self
                .r#speed_multiplier, max = 1000f32
            );
            self.r#speed_multiplier = 1000f32;
        }
        if self.r#build_extra_cycles < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#build_extra_cycles", value = self
                .r#build_extra_cycles, min = 0f32
            );
            self.r#build_extra_cycles = 0f32;
        }
        if self.r#build_extra_cycles > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#build_extra_cycles", value = self
                .r#build_extra_cycles, max = 100f32
            );
            self.r#build_extra_cycles = 100f32;
        }
        if self.r#improved_ai != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#improved_ai"
            );
        }
        if self.r#capacity < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#capacity", value = self.r#capacity, min
                = 1f32
            );
            self.r#capacity = 1f32;
        }
        if self.r#capacity > (1000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#capacity", value = self.r#capacity, max
                = 1000f32
            );
            self.r#capacity = 1000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Faction.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: FactionId) -> Self {
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
    pub fn with_id(mut self, r#id: FactionId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_no_territories(mut self, r#no_territories: bool) -> Self {
        self.r#no_territories = r#no_territories;
        Self
    }
    pub fn with_home_star_distance(mut self, r#home_star_distance: i32) -> Self {
        self.r#home_star_distance = r#home_star_distance;
        Self
    }
    pub fn with_home_star_distance_max(mut self, r#home_star_distance_max: i32) -> Self {
        self.r#home_star_distance_max = r#home_star_distance_max;
        Self
    }
    pub fn with_no_wandering_ships(mut self, r#no_wandering_ships: bool) -> Self {
        self.r#no_wandering_ships = r#no_wandering_ships;
        Self
    }
    pub fn with_wandering_ships_distance(
        mut self,
        r#wandering_ships_distance: i32,
    ) -> Self {
        self.r#wandering_ships_distance = r#wandering_ships_distance;
        Self
    }
    pub fn with_wandering_ships_distance_max(
        mut self,
        r#wandering_ships_distance_max: i32,
    ) -> Self {
        self.r#wandering_ships_distance_max = r#wandering_ships_distance_max;
        Self
    }
    pub fn with_hide_from_merchants(mut self, r#hide_from_merchants: bool) -> Self {
        self.r#hide_from_merchants = r#hide_from_merchants;
        Self
    }
    pub fn with_hide_research_tree(mut self, r#hide_research_tree: bool) -> Self {
        self.r#hide_research_tree = r#hide_research_tree;
        Self
    }
    pub fn with_no_missions(mut self, r#no_missions: bool) -> Self {
        self.r#no_missions = r#no_missions;
        Self
    }
    pub fn with_hidden(mut self, r#hidden: bool) -> Self {
        self.r#hidden = r#hidden;
        Self
    }
    pub fn with_hostile(mut self, r#hostile: bool) -> Self {
        self.r#hostile = r#hostile;
        Self
    }
}
impl DatabaseItem for Faction {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#home_star_distance < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#home_star_distance", value = self
                .r#home_star_distance, min = 0f32
            );
            self.r#home_star_distance = 0f32;
        }
        if self.r#home_star_distance > (5000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#home_star_distance", value = self
                .r#home_star_distance, max = 5000f32
            );
            self.r#home_star_distance = 5000f32;
        }
        if self.r#home_star_distance_max < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#home_star_distance_max", value = self
                .r#home_star_distance_max, min = 0f32
            );
            self.r#home_star_distance_max = 0f32;
        }
        if self.r#home_star_distance_max > (5000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#home_star_distance_max", value = self
                .r#home_star_distance_max, max = 5000f32
            );
            self.r#home_star_distance_max = 5000f32;
        }
        if self.r#wandering_ships_distance < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#wandering_ships_distance", value = self
                .r#wandering_ships_distance, min = 0f32
            );
            self.r#wandering_ships_distance = 0f32;
        }
        if self.r#wandering_ships_distance > (5000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#wandering_ships_distance", value = self
                .r#wandering_ships_distance, max = 5000f32
            );
            self.r#wandering_ships_distance = 5000f32;
        }
        if self.r#wandering_ships_distance_max < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#wandering_ships_distance_max", value =
                self.r#wandering_ships_distance_max, min = 0f32
            );
            self.r#wandering_ships_distance_max = 0f32;
        }
        if self.r#wandering_ships_distance_max > (5000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#wandering_ships_distance_max", value =
                self.r#wandering_ships_distance_max, max = 5000f32
            );
            self.r#wandering_ships_distance_max = 5000f32;
        }
        if self.r#hidden != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#hidden"
            );
        }
        if self.r#hostile != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#hostile"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/GameObjectPrefab.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
pub enum GameObjectPrefab {
    Undefined(GameObjectPrefabUndefined),
    WormTailSegment(GameObjectPrefabWormTailSegment),
    CircularSpriteObject(GameObjectPrefabCircularSpriteObject),
    CircularOutlineObject(GameObjectPrefabCircularOutlineObject),
}
impl From<GameObjectPrefabUndefined> for GameObjectPrefab {
    fn from(item: GameObjectPrefabUndefined) -> Self {
        Self::Undefined(item)
    }
}
impl From<GameObjectPrefabWormTailSegment> for GameObjectPrefab {
    fn from(item: GameObjectPrefabWormTailSegment) -> Self {
        Self::WormTailSegment(item)
    }
}
impl From<GameObjectPrefabCircularSpriteObject> for GameObjectPrefab {
    fn from(item: GameObjectPrefabCircularSpriteObject) -> Self {
        Self::CircularSpriteObject(item)
    }
}
impl From<GameObjectPrefabCircularOutlineObject> for GameObjectPrefab {
    fn from(item: GameObjectPrefabCircularOutlineObject) -> Self {
        Self::CircularOutlineObject(item)
    }
}
impl GameObjectPrefab {
    pub fn r#id(&self) {
        match self {
            Self::Undefined(x) => &x.r#id,
            Self::WormTailSegment(x) => &x.r#id,
            Self::CircularSpriteObject(x) => &x.r#id,
            Self::CircularOutlineObject(x) => &x.r#id,
        }
    }
    pub fn id_mut(&mut self) {
        match self {
            Self::Undefined(x) => &mut x.r#id,
            Self::WormTailSegment(x) => &mut x.r#id,
            Self::CircularSpriteObject(x) => &mut x.r#id,
            Self::CircularOutlineObject(x) => &mut x.r#id,
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: GameObjectPrefabId) -> Self {
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
    pub fn with_id(mut self, r#id: GameObjectPrefabId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_image_1(mut self, r#image_1: String) -> Self {
        self.r#image_1 = r#image_1;
        Self
    }
    pub fn with_image_2(mut self, r#image_2: String) -> Self {
        self.r#image_2 = r#image_2;
        Self
    }
    pub fn with_image_scale(mut self, r#image_scale: f32) -> Self {
        self.r#image_scale = r#image_scale;
        Self
    }
    pub fn with_image_offset(mut self, r#image_offset: f32) -> Self {
        self.r#image_offset = r#image_offset;
        Self
    }
    pub fn with_length(mut self, r#length: f32) -> Self {
        self.r#length = r#length;
        Self
    }
    pub fn with_offset_1(mut self, r#offset_1: f32) -> Self {
        self.r#offset_1 = r#offset_1;
        Self
    }
    pub fn with_offset_2(mut self, r#offset_2: f32) -> Self {
        self.r#offset_2 = r#offset_2;
        Self
    }
    pub fn with_angle_1(mut self, r#angle_1: f32) -> Self {
        self.r#angle_1 = r#angle_1;
        Self
    }
    pub fn with_angle_2(mut self, r#angle_2: f32) -> Self {
        self.r#angle_2 = r#angle_2;
        Self
    }
}
impl DatabaseItem for GameObjectPrefabWormTailSegment {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#image_scale < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_scale", value = self
                .r#image_scale, min = 0f32
            );
            self.r#image_scale = 0f32;
        }
        if self.r#image_scale > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_scale", value = self
                .r#image_scale, max = 10f32
            );
            self.r#image_scale = 10f32;
        }
        if self.r#image_offset < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_offset", value = self
                .r#image_offset, min = - 1f32
            );
            self.r#image_offset = -1f32;
        }
        if self.r#image_offset > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_offset", value = self
                .r#image_offset, max = 1f32
            );
            self.r#image_offset = 1f32;
        }
        if self.r#length < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#length", value = self.r#length, min =
                0f32
            );
            self.r#length = 0f32;
        }
        if self.r#length > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#length", value = self.r#length, max =
                1f32
            );
            self.r#length = 1f32;
        }
        if self.r#offset_1 < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#offset_1", value = self.r#offset_1, min
                = 0f32
            );
            self.r#offset_1 = 0f32;
        }
        if self.r#offset_1 > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#offset_1", value = self.r#offset_1, max
                = 1f32
            );
            self.r#offset_1 = 1f32;
        }
        if self.r#offset_2 < (-1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#offset_2", value = self.r#offset_2, min
                = - 1f32
            );
            self.r#offset_2 = -1f32;
        }
        if self.r#offset_2 > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#offset_2", value = self.r#offset_2, max
                = 1f32
            );
            self.r#offset_2 = 1f32;
        }
        if self.r#angle_1 < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#angle_1", value = self.r#angle_1, min =
                0f32
            );
            self.r#angle_1 = 0f32;
        }
        if self.r#angle_1 > (180f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#angle_1", value = self.r#angle_1, max =
                180f32
            );
            self.r#angle_1 = 180f32;
        }
        if self.r#angle_2 < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#angle_2", value = self.r#angle_2, min =
                0f32
            );
            self.r#angle_2 = 0f32;
        }
        if self.r#angle_2 > (180f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#angle_2", value = self.r#angle_2, max =
                180f32
            );
            self.r#angle_2 = 180f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct GameObjectPrefabCircularSpriteObject {
    pub r#id: GameObjectPrefabId,
    pub r#image_1: String,
    pub r#image_scale: f32,
}
impl GameObjectPrefabCircularSpriteObject {
    fn new(r#id: GameObjectPrefabId) -> Self {
        Self {
            r#id,
            r#image_1: Default::default(),
            r#image_scale: 1f32,
        }
    }
    pub fn with_id(mut self, r#id: GameObjectPrefabId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_image_1(mut self, r#image_1: String) -> Self {
        self.r#image_1 = r#image_1;
        Self
    }
    pub fn with_image_scale(mut self, r#image_scale: f32) -> Self {
        self.r#image_scale = r#image_scale;
        Self
    }
}
impl DatabaseItem for GameObjectPrefabCircularSpriteObject {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#image_scale < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_scale", value = self
                .r#image_scale, min = 0f32
            );
            self.r#image_scale = 0f32;
        }
        if self.r#image_scale > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_scale", value = self
                .r#image_scale, max = 10f32
            );
            self.r#image_scale = 10f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct GameObjectPrefabCircularOutlineObject {
    pub r#id: GameObjectPrefabId,
    pub r#image_1: String,
    pub r#image_scale: f32,
    pub r#thickness: f32,
    pub r#aspect_ratio: f32,
}
impl GameObjectPrefabCircularOutlineObject {
    fn new(r#id: GameObjectPrefabId) -> Self {
        Self {
            r#id,
            r#image_1: Default::default(),
            r#image_scale: 1f32,
            r#thickness: 0.1f32,
            r#aspect_ratio: 1f32,
        }
    }
    pub fn with_id(mut self, r#id: GameObjectPrefabId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_image_1(mut self, r#image_1: String) -> Self {
        self.r#image_1 = r#image_1;
        Self
    }
    pub fn with_image_scale(mut self, r#image_scale: f32) -> Self {
        self.r#image_scale = r#image_scale;
        Self
    }
    pub fn with_thickness(mut self, r#thickness: f32) -> Self {
        self.r#thickness = r#thickness;
        Self
    }
    pub fn with_aspect_ratio(mut self, r#aspect_ratio: f32) -> Self {
        self.r#aspect_ratio = r#aspect_ratio;
        Self
    }
}
impl DatabaseItem for GameObjectPrefabCircularOutlineObject {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#image_scale < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_scale", value = self
                .r#image_scale, min = 0f32
            );
            self.r#image_scale = 0f32;
        }
        if self.r#image_scale > (10f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#image_scale", value = self
                .r#image_scale, max = 10f32
            );
            self.r#image_scale = 10f32;
        }
        if self.r#thickness < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#thickness", value = self.r#thickness,
                min = 0f32
            );
            self.r#thickness = 0f32;
        }
        if self.r#thickness > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#thickness", value = self.r#thickness,
                max = 1f32
            );
            self.r#thickness = 1f32;
        }
        if self.r#aspect_ratio < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#aspect_ratio", value = self
                .r#aspect_ratio, min = 0f32
            );
            self.r#aspect_ratio = 0f32;
        }
        if self.r#aspect_ratio > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#aspect_ratio", value = self
                .r#aspect_ratio, max = 100f32
            );
            self.r#aspect_ratio = 100f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct GameObjectPrefabUndefined {
    pub r#id: GameObjectPrefabId,
}
impl GameObjectPrefabUndefined {
    fn new(r#id: GameObjectPrefabId) -> Self {
        Self { r#id }
    }
    pub fn with_id(mut self, r#id: GameObjectPrefabId) -> Self {
        self.r#id = r#id;
        Self
    }
}
impl DatabaseItem for GameObjectPrefabUndefined {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Character.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: CharacterId) -> Self {
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
    pub fn with_id(mut self, r#id: CharacterId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_avatar_icon(mut self, r#avatar_icon: String) -> Self {
        self.r#avatar_icon = r#avatar_icon;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
    pub fn with_inventory(mut self, r#inventory: Option<LootId>) -> Self {
        self.r#inventory = r#inventory;
        Self
    }
    pub fn with_fleet(mut self, r#fleet: Option<FleetId>) -> Self {
        self.r#fleet = r#fleet;
        Self
    }
    pub fn with_relations(mut self, r#relations: i32) -> Self {
        self.r#relations = r#relations;
        Self
    }
    pub fn with_is_unique(mut self, r#is_unique: bool) -> Self {
        self.r#is_unique = r#is_unique;
        Self
    }
}
impl DatabaseItem for Character {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#relations < (-100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#relations", value = self.r#relations,
                min = - 100f32
            );
            self.r#relations = -100f32;
        }
        if self.r#relations > (100f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#relations", value = self.r#relations,
                max = 100f32
            );
            self.r#relations = 100f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/CombatRules.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    pub r#kill_them_all_button: bool,
    pub r#custom_soundtrack: Vec<SoundTrack>,
}
impl CombatRules {
    fn new(r#id: CombatRulesId) -> Self {
        Self {
            r#id,
            r#initial_enemy_ships: default,
            r#max_enemy_ships: default,
            r#battle_map_size: 200i32,
            r#time_limit: default,
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
    pub fn with_id(mut self, r#id: CombatRulesId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_initial_enemy_ships(mut self, r#initial_enemy_ships: String) -> Self {
        self.r#initial_enemy_ships = r#initial_enemy_ships;
        Self
    }
    pub fn with_max_enemy_ships(mut self, r#max_enemy_ships: String) -> Self {
        self.r#max_enemy_ships = r#max_enemy_ships;
        Self
    }
    pub fn with_battle_map_size(mut self, r#battle_map_size: i32) -> Self {
        self.r#battle_map_size = r#battle_map_size;
        Self
    }
    pub fn with_time_limit(mut self, r#time_limit: String) -> Self {
        self.r#time_limit = r#time_limit;
        Self
    }
    pub fn with_time_out_mode(mut self, r#time_out_mode: TimeOutMode) -> Self {
        self.r#time_out_mode = r#time_out_mode;
        Self
    }
    pub fn with_loot_condition(mut self, r#loot_condition: RewardCondition) -> Self {
        self.r#loot_condition = r#loot_condition;
        Self
    }
    pub fn with_exp_condition(mut self, r#exp_condition: RewardCondition) -> Self {
        self.r#exp_condition = r#exp_condition;
        Self
    }
    pub fn with_ship_selection(
        mut self,
        r#ship_selection: PlayerShipSelectionMode,
    ) -> Self {
        self.r#ship_selection = r#ship_selection;
        Self
    }
    pub fn with_disable_skill_bonuses(mut self, r#disable_skill_bonuses: bool) -> Self {
        self.r#disable_skill_bonuses = r#disable_skill_bonuses;
        Self
    }
    pub fn with_disable_random_loot(mut self, r#disable_random_loot: bool) -> Self {
        self.r#disable_random_loot = r#disable_random_loot;
        Self
    }
    pub fn with_disable_asteroids(mut self, r#disable_asteroids: bool) -> Self {
        self.r#disable_asteroids = r#disable_asteroids;
        Self
    }
    pub fn with_disable_planet(mut self, r#disable_planet: bool) -> Self {
        self.r#disable_planet = r#disable_planet;
        Self
    }
    pub fn with_next_enemy_button(mut self, r#next_enemy_button: bool) -> Self {
        self.r#next_enemy_button = r#next_enemy_button;
        Self
    }
    pub fn with_kill_them_all_button(mut self, r#kill_them_all_button: bool) -> Self {
        self.r#kill_them_all_button = r#kill_them_all_button;
        Self
    }
    pub fn with_custom_soundtrack(
        mut self,
        r#custom_soundtrack: Vec<SoundTrack>,
    ) -> Self {
        self.r#custom_soundtrack = r#custom_soundtrack;
        Self
    }
}
impl DatabaseItem for CombatRules {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#initial_enemy_ships < (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#initial_enemy_ships", value = self
                .r#initial_enemy_ships, min = 1f32
            );
            self.r#initial_enemy_ships = 1f32;
        }
        if self.r#max_enemy_ships < (1f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#max_enemy_ships", value = self
                .r#max_enemy_ships, min = 1f32
            );
            self.r#max_enemy_ships = 1f32;
        }
        if self.r#battle_map_size < (50f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#battle_map_size", value = self
                .r#battle_map_size, min = 50f32
            );
            self.r#battle_map_size = 50f32;
        }
        if self.r#time_limit < (0f32 as String) {
            tracing::warn!(
                "Field got truncated", field = "r#time_limit", value = self.r#time_limit,
                min = 0f32
            );
            self.r#time_limit = 0f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Fleet.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: FleetId) -> Self {
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
    pub fn with_id(mut self, r#id: FleetId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_factions(mut self, r#factions: FactionFilter) -> Self {
        self.r#factions = r#factions;
        Self
    }
    pub fn with_level_bonus(mut self, r#level_bonus: i32) -> Self {
        self.r#level_bonus = r#level_bonus;
        Self
    }
    pub fn with_no_random_ships(mut self, r#no_random_ships: bool) -> Self {
        self.r#no_random_ships = r#no_random_ships;
        Self
    }
    pub fn with_combat_time_limit(mut self, r#combat_time_limit: i32) -> Self {
        self.r#combat_time_limit = r#combat_time_limit;
        Self
    }
    pub fn with_loot_condition(mut self, r#loot_condition: RewardCondition) -> Self {
        self.r#loot_condition = r#loot_condition;
        Self
    }
    pub fn with_exp_condition(mut self, r#exp_condition: RewardCondition) -> Self {
        self.r#exp_condition = r#exp_condition;
        Self
    }
    pub fn with_specific_ships(mut self, r#specific_ships: Vec<ShipBuildId>) -> Self {
        self.r#specific_ships = r#specific_ships;
        Self
    }
    pub fn with_no_ship_changing(mut self, r#no_ship_changing: bool) -> Self {
        self.r#no_ship_changing = r#no_ship_changing;
        Self
    }
    pub fn with_player_has_one_ship(mut self, r#player_has_one_ship: bool) -> Self {
        self.r#player_has_one_ship = r#player_has_one_ship;
        Self
    }
    pub fn with_combat_rules(mut self, r#combat_rules: Option<CombatRulesId>) -> Self {
        self.r#combat_rules = r#combat_rules;
        Self
    }
}
impl DatabaseItem for Fleet {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#level_bonus < (-10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#level_bonus", value = self
                .r#level_bonus, min = - 10000f32
            );
            self.r#level_bonus = -10000f32;
        }
        if self.r#level_bonus > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#level_bonus", value = self
                .r#level_bonus, max = 10000f32
            );
            self.r#level_bonus = 10000f32;
        }
        if self.r#combat_time_limit < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#combat_time_limit", value = self
                .r#combat_time_limit, min = 0f32
            );
            self.r#combat_time_limit = 0f32;
        }
        if self.r#combat_time_limit > (999f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#combat_time_limit", value = self
                .r#combat_time_limit, max = 999f32
            );
            self.r#combat_time_limit = 999f32;
        }
        if self.r#combat_time_limit != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#combat_time_limit"
            );
        }
        if self.r#loot_condition != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#loot_condition"
            );
        }
        if self.r#exp_condition != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#exp_condition"
            );
        }
        if self.r#no_ship_changing != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#no_ship_changing"
            );
        }
        if self.r#player_has_one_ship != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#player_has_one_ship"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Loot.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct Loot {
    pub r#id: LootId,
    pub r#loot: LootContent,
}
impl Loot {
    fn new(r#id: LootId) -> Self {
        Self {
            r#id,
            r#loot: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: LootId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_loot(mut self, r#loot: LootContent) -> Self {
        self.r#loot = r#loot;
        Self
    }
}
impl DatabaseItem for Loot {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/Quest.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: QuestId) -> Self {
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
    pub fn with_id(mut self, r#id: QuestId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_quest_type(mut self, r#quest_type: QuestType) -> Self {
        self.r#quest_type = r#quest_type;
        Self
    }
    pub fn with_start_condition(mut self, r#start_condition: StartCondition) -> Self {
        self.r#start_condition = r#start_condition;
        Self
    }
    pub fn with_weight(mut self, r#weight: f32) -> Self {
        self.r#weight = r#weight;
        Self
    }
    pub fn with_origin(mut self, r#origin: QuestOrigin) -> Self {
        self.r#origin = r#origin;
        Self
    }
    pub fn with_requirement(mut self, r#requirement: Requirement) -> Self {
        self.r#requirement = r#requirement;
        Self
    }
    pub fn with_level(mut self, r#level: i32) -> Self {
        self.r#level = r#level;
        Self
    }
    pub fn with_use_random_seed(mut self, r#use_random_seed: bool) -> Self {
        self.r#use_random_seed = r#use_random_seed;
        Self
    }
    pub fn with_nodes(mut self, r#nodes: Vec<Node>) -> Self {
        self.r#nodes = r#nodes;
        Self
    }
}
impl DatabaseItem for Quest {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#weight < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, min =
                0f32
            );
            self.r#weight = 0f32;
        }
        if self.r#weight > (1000f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#weight", value = self.r#weight, max =
                1000f32
            );
            self.r#weight = 1000f32;
        }
        if self.r#level < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#level", value = self.r#level, min =
                0f32
            );
            self.r#level = 0f32;
        }
        if self.r#level > (1000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#level", value = self.r#level, max =
                1000f32
            );
            self.r#level = 1000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Quests/QuestItem.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct QuestItem {
    pub r#id: QuestItemId,
    pub r#name: String,
    pub r#description: String,
    pub r#icon: String,
    pub r#color: String,
    pub r#price: i32,
}
impl QuestItem {
    fn new(r#id: QuestItemId) -> Self {
        Self {
            r#id,
            r#name: Default::default(),
            r#description: Default::default(),
            r#icon: Default::default(),
            r#color: Default::default(),
            r#price: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: QuestItemId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_description(mut self, r#description: String) -> Self {
        self.r#description = r#description;
        Self
    }
    pub fn with_icon(mut self, r#icon: String) -> Self {
        self.r#icon = r#icon;
        Self
    }
    pub fn with_color(mut self, r#color: String) -> Self {
        self.r#color = r#color;
        Self
    }
    pub fn with_price(mut self, r#price: i32) -> Self {
        self.r#price = r#price;
        Self
    }
}
impl DatabaseItem for QuestItem {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, min =
                0f32
            );
            self.r#price = 0f32;
        }
        if self.r#price > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, max =
                1000000000f32
            );
            self.r#price = 1000000000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Satellite.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: SatelliteId) -> Self {
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
    pub fn with_id(mut self, r#id: SatelliteId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_model_image(mut self, r#model_image: String) -> Self {
        self.r#model_image = r#model_image;
        Self
    }
    pub fn with_model_scale(mut self, r#model_scale: f32) -> Self {
        self.r#model_scale = r#model_scale;
        Self
    }
    pub fn with_size_class(mut self, r#size_class: SizeClass) -> Self {
        self.r#size_class = r#size_class;
        Self
    }
    pub fn with_layout(mut self, r#layout: String) -> Self {
        self.r#layout = r#layout;
        Self
    }
    pub fn with_barrels(mut self, r#barrels: Vec<Barrel>) -> Self {
        self.r#barrels = r#barrels;
        Self
    }
}
impl DatabaseItem for Satellite {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#model_scale < (0.1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#model_scale", value = self
                .r#model_scale, min = 0.1f32
            );
            self.r#model_scale = 0.1f32;
        }
        if self.r#model_scale > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#model_scale", value = self
                .r#model_scale, max = 100f32
            );
            self.r#model_scale = 100f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/SatelliteBuild.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct SatelliteBuild {
    pub r#id: SatelliteBuildId,
    pub r#satellite_id: SatelliteId,
    pub r#not_available_in_game: bool,
    pub r#difficulty_class: DifficultyClass,
    pub r#components: Vec<InstalledComponent>,
}
impl SatelliteBuild {
    fn new(r#id: SatelliteBuildId, r#satellite_id: SatelliteId) -> Self {
        Self {
            r#id,
            r#satellite_id,
            r#not_available_in_game: Default::default(),
            r#difficulty_class: Default::default(),
            r#components: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: SatelliteBuildId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_satellite_id(mut self, r#satellite_id: SatelliteId) -> Self {
        self.r#satellite_id = r#satellite_id;
        Self
    }
    pub fn with_not_available_in_game(mut self, r#not_available_in_game: bool) -> Self {
        self.r#not_available_in_game = r#not_available_in_game;
        Self
    }
    pub fn with_difficulty_class(mut self, r#difficulty_class: DifficultyClass) -> Self {
        self.r#difficulty_class = r#difficulty_class;
        Self
    }
    pub fn with_components(mut self, r#components: Vec<InstalledComponent>) -> Self {
        self.r#components = r#components;
        Self
    }
}
impl DatabaseItem for SatelliteBuild {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#satellite_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#satellite_id"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Ship.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: ShipId) -> Self {
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
    pub fn with_id(mut self, r#id: ShipId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_ship_type(mut self, r#ship_type: ShipType) -> Self {
        self.r#ship_type = r#ship_type;
        Self
    }
    pub fn with_ship_rarity(mut self, r#ship_rarity: ShipRarity) -> Self {
        self.r#ship_rarity = r#ship_rarity;
        Self
    }
    pub fn with_size_class(mut self, r#size_class: SizeClass) -> Self {
        self.r#size_class = r#size_class;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_description(mut self, r#description: String) -> Self {
        self.r#description = r#description;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
    pub fn with_icon_image(mut self, r#icon_image: String) -> Self {
        self.r#icon_image = r#icon_image;
        Self
    }
    pub fn with_icon_scale(mut self, r#icon_scale: f32) -> Self {
        self.r#icon_scale = r#icon_scale;
        Self
    }
    pub fn with_model_image(mut self, r#model_image: String) -> Self {
        self.r#model_image = r#model_image;
        Self
    }
    pub fn with_model_scale(mut self, r#model_scale: f32) -> Self {
        self.r#model_scale = r#model_scale;
        Self
    }
    pub fn with_engine_color(mut self, r#engine_color: String) -> Self {
        self.r#engine_color = r#engine_color;
        Self
    }
    pub fn with_engines(mut self, r#engines: Vec<Engine>) -> Self {
        self.r#engines = r#engines;
        Self
    }
    pub fn with_layout(mut self, r#layout: String) -> Self {
        self.r#layout = r#layout;
        Self
    }
    pub fn with_barrels(mut self, r#barrels: Vec<Barrel>) -> Self {
        self.r#barrels = r#barrels;
        Self
    }
    pub fn with_features(mut self, r#features: ShipFeatures) -> Self {
        self.r#features = r#features;
        Self
    }
    pub fn with_collider_tolerance(mut self, r#collider_tolerance: f32) -> Self {
        self.r#collider_tolerance = r#collider_tolerance;
        Self
    }
    pub fn with_engine_position(mut self, r#engine_position: glam::f32::Vec2) -> Self {
        self.r#engine_position = r#engine_position;
        Self
    }
    pub fn with_engine_size(mut self, r#engine_size: f32) -> Self {
        self.r#engine_size = r#engine_size;
        Self
    }
    pub fn with_ship_category(mut self, r#ship_category: i32) -> Self {
        self.r#ship_category = r#ship_category;
        Self
    }
    pub fn with_energy_resistance(mut self, r#energy_resistance: f32) -> Self {
        self.r#energy_resistance = r#energy_resistance;
        Self
    }
    pub fn with_kinetic_resistance(mut self, r#kinetic_resistance: f32) -> Self {
        self.r#kinetic_resistance = r#kinetic_resistance;
        Self
    }
    pub fn with_heat_resistance(mut self, r#heat_resistance: f32) -> Self {
        self.r#heat_resistance = r#heat_resistance;
        Self
    }
    pub fn with_regeneration(mut self, r#regeneration: bool) -> Self {
        self.r#regeneration = r#regeneration;
        Self
    }
    pub fn with_builtin_devices(mut self, r#builtin_devices: Vec<DeviceId>) -> Self {
        self.r#builtin_devices = r#builtin_devices;
        Self
    }
    pub fn with_base_weight_modifier(mut self, r#base_weight_modifier: f32) -> Self {
        self.r#base_weight_modifier = r#base_weight_modifier;
        Self
    }
}
impl DatabaseItem for Ship {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#icon_scale < (0.1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#icon_scale", value = self.r#icon_scale,
                min = 0.1f32
            );
            self.r#icon_scale = 0.1f32;
        }
        if self.r#icon_scale > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#icon_scale", value = self.r#icon_scale,
                max = 100f32
            );
            self.r#icon_scale = 100f32;
        }
        if self.r#model_scale < (0.1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#model_scale", value = self
                .r#model_scale, min = 0.1f32
            );
            self.r#model_scale = 0.1f32;
        }
        if self.r#model_scale > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#model_scale", value = self
                .r#model_scale, max = 100f32
            );
            self.r#model_scale = 100f32;
        }
        if self.r#collider_tolerance < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#collider_tolerance", value = self
                .r#collider_tolerance, min = 0f32
            );
            self.r#collider_tolerance = 0f32;
        }
        if self.r#collider_tolerance > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#collider_tolerance", value = self
                .r#collider_tolerance, max = 1f32
            );
            self.r#collider_tolerance = 1f32;
        }
        if self.r#engine_position != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#engine_position"
            );
        }
        if self.r#engine_size != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#engine_size"
            );
        }
        if self.r#ship_category != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#ship_category"
            );
        }
        if self.r#energy_resistance != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#energy_resistance"
            );
        }
        if self.r#kinetic_resistance != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#kinetic_resistance"
            );
        }
        if self.r#heat_resistance != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#heat_resistance"
            );
        }
        if self.r#regeneration != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#regeneration"
            );
        }
        if self.r#builtin_devices != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#builtin_devices"
            );
        }
        if self.r#base_weight_modifier != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#base_weight_modifier"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/ShipBuild.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: ShipBuildId, r#ship_id: ShipId) -> Self {
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
    pub fn with_id(mut self, r#id: ShipBuildId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_ship_id(mut self, r#ship_id: ShipId) -> Self {
        self.r#ship_id = r#ship_id;
        Self
    }
    pub fn with_available_for_player(mut self, r#available_for_player: bool) -> Self {
        self.r#available_for_player = r#available_for_player;
        Self
    }
    pub fn with_available_for_enemy(mut self, r#available_for_enemy: bool) -> Self {
        self.r#available_for_enemy = r#available_for_enemy;
        Self
    }
    pub fn with_difficulty_class(mut self, r#difficulty_class: DifficultyClass) -> Self {
        self.r#difficulty_class = r#difficulty_class;
        Self
    }
    pub fn with_build_faction(mut self, r#build_faction: Option<FactionId>) -> Self {
        self.r#build_faction = r#build_faction;
        Self
    }
    pub fn with_custom_ai(mut self, r#custom_ai: Option<BehaviorTreeId>) -> Self {
        self.r#custom_ai = r#custom_ai;
        Self
    }
    pub fn with_components(mut self, r#components: Vec<InstalledComponent>) -> Self {
        self.r#components = r#components;
        Self
    }
    pub fn with_not_available_in_game(mut self, r#not_available_in_game: bool) -> Self {
        self.r#not_available_in_game = r#not_available_in_game;
        Self
    }
}
impl DatabaseItem for ShipBuild {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#ship_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#ship_id"
            );
        }
        if self.r#not_available_in_game != Default::default() {
            tracing::error!(
                "Obsolete field usage detected, generated code may not work", field =
                "r#not_available_in_game"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Skill.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: SkillId) -> Self {
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
    pub fn with_id(mut self, r#id: SkillId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_name(mut self, r#name: String) -> Self {
        self.r#name = r#name;
        Self
    }
    pub fn with_icon(mut self, r#icon: String) -> Self {
        self.r#icon = r#icon;
        Self
    }
    pub fn with_description(mut self, r#description: String) -> Self {
        self.r#description = r#description;
        Self
    }
    pub fn with_base_requirement(mut self, r#base_requirement: f32) -> Self {
        self.r#base_requirement = r#base_requirement;
        Self
    }
    pub fn with_requirement_per_level(mut self, r#requirement_per_level: f32) -> Self {
        self.r#requirement_per_level = r#requirement_per_level;
        Self
    }
    pub fn with_base_price(mut self, r#base_price: f32) -> Self {
        self.r#base_price = r#base_price;
        Self
    }
    pub fn with_price_per_level(mut self, r#price_per_level: f32) -> Self {
        self.r#price_per_level = r#price_per_level;
        Self
    }
    pub fn with_max_level(mut self, r#max_level: i32) -> Self {
        self.r#max_level = r#max_level;
        Self
    }
}
impl DatabaseItem for Skill {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#base_requirement < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_requirement", value = self
                .r#base_requirement, min = 0f32
            );
            self.r#base_requirement = 0f32;
        }
        if self.r#base_requirement > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_requirement", value = self
                .r#base_requirement, max = 100f32
            );
            self.r#base_requirement = 100f32;
        }
        if self.r#requirement_per_level < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#requirement_per_level", value = self
                .r#requirement_per_level, min = 0f32
            );
            self.r#requirement_per_level = 0f32;
        }
        if self.r#requirement_per_level > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#requirement_per_level", value = self
                .r#requirement_per_level, max = 100f32
            );
            self.r#requirement_per_level = 100f32;
        }
        if self.r#base_price < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_price", value = self.r#base_price,
                min = 0f32
            );
            self.r#base_price = 0f32;
        }
        if self.r#base_price > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#base_price", value = self.r#base_price,
                max = 100f32
            );
            self.r#base_price = 100f32;
        }
        if self.r#price_per_level < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#price_per_level", value = self
                .r#price_per_level, min = 0f32
            );
            self.r#price_per_level = 0f32;
        }
        if self.r#price_per_level > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#price_per_level", value = self
                .r#price_per_level, max = 100f32
            );
            self.r#price_per_level = 100f32;
        }
        if self.r#max_level < (1f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_level", value = self.r#max_level,
                min = 1f32
            );
            self.r#max_level = 1f32;
        }
        if self.r#max_level > (1000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#max_level", value = self.r#max_level,
                max = 1000f32
            );
            self.r#max_level = 1000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Technology.xml
#[derive(Debug, Clone, serde::Serialize)]
#[serde(tag = "Type")]
pub enum Technology {
    Component(TechnologyComponent),
    Ship(TechnologyShip),
    Satellite(TechnologySatellite),
}
impl From<TechnologyComponent> for Technology {
    fn from(item: TechnologyComponent) -> Self {
        Self::Component(item)
    }
}
impl From<TechnologyShip> for Technology {
    fn from(item: TechnologyShip) -> Self {
        Self::Ship(item)
    }
}
impl From<TechnologySatellite> for Technology {
    fn from(item: TechnologySatellite) -> Self {
        Self::Satellite(item)
    }
}
impl Technology {
    pub fn r#id(&self) {
        match self {
            Self::Component(x) => &x.r#id,
            Self::Ship(x) => &x.r#id,
            Self::Satellite(x) => &x.r#id,
        }
    }
    pub fn id_mut(&mut self) {
        match self {
            Self::Component(x) => &mut x.r#id,
            Self::Ship(x) => &mut x.r#id,
            Self::Satellite(x) => &mut x.r#id,
        }
    }
}
impl Technology {
    pub fn r#price(&self) {
        match self {
            Self::Component(x) => &x.r#price,
            Self::Ship(x) => &x.r#price,
            Self::Satellite(x) => &x.r#price,
        }
    }
    pub fn price_mut(&mut self) {
        match self {
            Self::Component(x) => &mut x.r#price,
            Self::Ship(x) => &mut x.r#price,
            Self::Satellite(x) => &mut x.r#price,
        }
    }
}
impl Technology {
    pub fn r#hidden(&self) {
        match self {
            Self::Component(x) => &x.r#hidden,
            Self::Ship(x) => &x.r#hidden,
            Self::Satellite(x) => &x.r#hidden,
        }
    }
    pub fn hidden_mut(&mut self) {
        match self {
            Self::Component(x) => &mut x.r#hidden,
            Self::Ship(x) => &mut x.r#hidden,
            Self::Satellite(x) => &mut x.r#hidden,
        }
    }
}
impl Technology {
    pub fn r#special(&self) {
        match self {
            Self::Component(x) => &x.r#special,
            Self::Ship(x) => &x.r#special,
            Self::Satellite(x) => &x.r#special,
        }
    }
    pub fn special_mut(&mut self) {
        match self {
            Self::Component(x) => &mut x.r#special,
            Self::Ship(x) => &mut x.r#special,
            Self::Satellite(x) => &mut x.r#special,
        }
    }
}
impl Technology {
    pub fn r#dependencies(&self) {
        match self {
            Self::Component(x) => &x.r#dependencies,
            Self::Ship(x) => &x.r#dependencies,
            Self::Satellite(x) => &x.r#dependencies,
        }
    }
    pub fn dependencies_mut(&mut self) {
        match self {
            Self::Component(x) => &mut x.r#dependencies,
            Self::Ship(x) => &mut x.r#dependencies,
            Self::Satellite(x) => &mut x.r#dependencies,
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
pub struct TechnologyShip {
    pub r#id: TechnologyId,
    pub r#item_id: ShipId,
    pub r#price: i32,
    pub r#hidden: bool,
    pub r#special: bool,
    pub r#dependencies: Vec<TechnologyId>,
}
impl TechnologyShip {
    fn new(r#id: TechnologyId, r#item_id: ShipId) -> Self {
        Self {
            r#id,
            r#item_id,
            r#price: Default::default(),
            r#hidden: Default::default(),
            r#special: Default::default(),
            r#dependencies: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: TechnologyId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_item_id(mut self, r#item_id: ShipId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
    pub fn with_price(mut self, r#price: i32) -> Self {
        self.r#price = r#price;
        Self
    }
    pub fn with_hidden(mut self, r#hidden: bool) -> Self {
        self.r#hidden = r#hidden;
        Self
    }
    pub fn with_special(mut self, r#special: bool) -> Self {
        self.r#special = r#special;
        Self
    }
    pub fn with_dependencies(mut self, r#dependencies: Vec<TechnologyId>) -> Self {
        self.r#dependencies = r#dependencies;
        Self
    }
}
impl DatabaseItem for TechnologyShip {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, min =
                0f32
            );
            self.r#price = 0f32;
        }
        if self.r#price > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, max =
                10000f32
            );
            self.r#price = 10000f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: TechnologyId, r#item_id: ComponentId) -> Self {
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
    pub fn with_id(mut self, r#id: TechnologyId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_item_id(mut self, r#item_id: ComponentId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
    pub fn with_price(mut self, r#price: i32) -> Self {
        self.r#price = r#price;
        Self
    }
    pub fn with_hidden(mut self, r#hidden: bool) -> Self {
        self.r#hidden = r#hidden;
        Self
    }
    pub fn with_special(mut self, r#special: bool) -> Self {
        self.r#special = r#special;
        Self
    }
    pub fn with_dependencies(mut self, r#dependencies: Vec<TechnologyId>) -> Self {
        self.r#dependencies = r#dependencies;
        Self
    }
}
impl DatabaseItem for TechnologyComponent {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, min =
                0f32
            );
            self.r#price = 0f32;
        }
        if self.r#price > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, max =
                10000f32
            );
            self.r#price = 10000f32;
        }
    }
}
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: TechnologyId, r#item_id: SatelliteId) -> Self {
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
    pub fn with_id(mut self, r#id: TechnologyId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_item_id(mut self, r#item_id: SatelliteId) -> Self {
        self.r#item_id = r#item_id;
        Self
    }
    pub fn with_faction(mut self, r#faction: Option<FactionId>) -> Self {
        self.r#faction = r#faction;
        Self
    }
    pub fn with_price(mut self, r#price: i32) -> Self {
        self.r#price = r#price;
        Self
    }
    pub fn with_hidden(mut self, r#hidden: bool) -> Self {
        self.r#hidden = r#hidden;
        Self
    }
    pub fn with_special(mut self, r#special: bool) -> Self {
        self.r#special = r#special;
        Self
    }
    pub fn with_dependencies(mut self, r#dependencies: Vec<TechnologyId>) -> Self {
        self.r#dependencies = r#dependencies;
        Self
    }
}
impl DatabaseItem for TechnologySatellite {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#item_id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#item_id"
            );
        }
        if self.r#price < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, min =
                0f32
            );
            self.r#price = 0f32;
        }
        if self.r#price > (10000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#price", value = self.r#price, max =
                10000f32
            );
            self.r#price = 10000f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/Ammunition.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct Ammunition {
    pub r#id: AmmunitionId,
    pub r#body: BulletBody,
    pub r#controller: BulletController,
    pub r#triggers: Vec<BulletTrigger>,
    pub r#impact_type: BulletImpactType,
    pub r#effects: Vec<ImpactEffect>,
}
impl Ammunition {
    fn new(r#id: AmmunitionId) -> Self {
        Self {
            r#id,
            r#body: Default::default(),
            r#controller: Default::default(),
            r#triggers: Default::default(),
            r#impact_type: Default::default(),
            r#effects: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: AmmunitionId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_body(mut self, r#body: BulletBody) -> Self {
        self.r#body = r#body;
        Self
    }
    pub fn with_controller(mut self, r#controller: BulletController) -> Self {
        self.r#controller = r#controller;
        Self
    }
    pub fn with_triggers(mut self, r#triggers: Vec<BulletTrigger>) -> Self {
        self.r#triggers = r#triggers;
        Self
    }
    pub fn with_impact_type(mut self, r#impact_type: BulletImpactType) -> Self {
        self.r#impact_type = r#impact_type;
        Self
    }
    pub fn with_effects(mut self, r#effects: Vec<ImpactEffect>) -> Self {
        self.r#effects = r#effects;
        Self
    }
}
impl DatabaseItem for Ammunition {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/BulletPerfab.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: BulletPrefabId) -> Self {
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
    pub fn with_id(mut self, r#id: BulletPrefabId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_shape(mut self, r#shape: BulletShape) -> Self {
        self.r#shape = r#shape;
        Self
    }
    pub fn with_image(mut self, r#image: String) -> Self {
        self.r#image = r#image;
        Self
    }
    pub fn with_size(mut self, r#size: f32) -> Self {
        self.r#size = r#size;
        Self
    }
    pub fn with_margins(mut self, r#margins: f32) -> Self {
        self.r#margins = r#margins;
        Self
    }
    pub fn with_deformation(mut self, r#deformation: f32) -> Self {
        self.r#deformation = r#deformation;
        Self
    }
    pub fn with_main_color(mut self, r#main_color: String) -> Self {
        self.r#main_color = r#main_color;
        Self
    }
    pub fn with_main_color_mode(mut self, r#main_color_mode: ColorMode) -> Self {
        self.r#main_color_mode = r#main_color_mode;
        Self
    }
    pub fn with_second_color(mut self, r#second_color: String) -> Self {
        self.r#second_color = r#second_color;
        Self
    }
    pub fn with_second_color_mode(mut self, r#second_color_mode: ColorMode) -> Self {
        self.r#second_color_mode = r#second_color_mode;
        Self
    }
}
impl DatabaseItem for BulletPrefab {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#size < (0.01f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, min =
                0.01f32
            );
            self.r#size = 0.01f32;
        }
        if self.r#size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#size", value = self.r#size, max =
                100f32
            );
            self.r#size = 100f32;
        }
        if self.r#margins < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#margins", value = self.r#margins, min =
                0f32
            );
            self.r#margins = 0f32;
        }
        if self.r#margins > (1f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#margins", value = self.r#margins, max =
                1f32
            );
            self.r#margins = 1f32;
        }
        if self.r#deformation < (-100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#deformation", value = self
                .r#deformation, min = - 100f32
            );
            self.r#deformation = -100f32;
        }
        if self.r#deformation > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#deformation", value = self
                .r#deformation, max = 100f32
            );
            self.r#deformation = 100f32;
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/VisualEffect.xml
#[derive(Debug, Clone, serde::Serialize)]
pub struct VisualEffect {
    pub r#id: VisualEffectId,
    pub r#elements: Vec<VisualEffectElement>,
}
impl VisualEffect {
    fn new(r#id: VisualEffectId) -> Self {
        Self {
            r#id,
            r#elements: Default::default(),
        }
    }
    pub fn with_id(mut self, r#id: VisualEffectId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_elements(mut self, r#elements: Vec<VisualEffectElement>) -> Self {
        self.r#elements = r#elements;
        Self
    }
}
impl DatabaseItem for VisualEffect {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
    }
}

// /home/juh9870/shared_projects/event-horizon-main/Assets/Modules/Database/.Schema/v1/Objects/Weapon/Weapon.xml
#[derive(Debug, Clone, serde::Serialize)]
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
    fn new(r#id: WeaponId) -> Self {
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
    pub fn with_id(mut self, r#id: WeaponId) -> Self {
        self.r#id = r#id;
        Self
    }
    pub fn with_weapon_class(mut self, r#weapon_class: WeaponClass) -> Self {
        self.r#weapon_class = r#weapon_class;
        Self
    }
    pub fn with_fire_rate(mut self, r#fire_rate: f32) -> Self {
        self.r#fire_rate = r#fire_rate;
        Self
    }
    pub fn with_spread(mut self, r#spread: f32) -> Self {
        self.r#spread = r#spread;
        Self
    }
    pub fn with_magazine(mut self, r#magazine: i32) -> Self {
        self.r#magazine = r#magazine;
        Self
    }
    pub fn with_activation_type(mut self, r#activation_type: ActivationType) -> Self {
        self.r#activation_type = r#activation_type;
        Self
    }
    pub fn with_shot_sound(mut self, r#shot_sound: String) -> Self {
        self.r#shot_sound = r#shot_sound;
        Self
    }
    pub fn with_charge_sound(mut self, r#charge_sound: String) -> Self {
        self.r#charge_sound = r#charge_sound;
        Self
    }
    pub fn with_shot_effect_prefab(mut self, r#shot_effect_prefab: String) -> Self {
        self.r#shot_effect_prefab = r#shot_effect_prefab;
        Self
    }
    pub fn with_visual_effect(
        mut self,
        r#visual_effect: Option<VisualEffectId>,
    ) -> Self {
        self.r#visual_effect = r#visual_effect;
        Self
    }
    pub fn with_effect_size(mut self, r#effect_size: f32) -> Self {
        self.r#effect_size = r#effect_size;
        Self
    }
    pub fn with_control_button_icon(mut self, r#control_button_icon: String) -> Self {
        self.r#control_button_icon = r#control_button_icon;
        Self
    }
}
impl DatabaseItem for Weapon {
    fn validate(&mut self) {
        if self.r#id.is_none() {
            tracing::error!(
                "Field is marked as notnull, but value is None", field = "r#id"
            );
        }
        if self.r#fire_rate < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#fire_rate", value = self.r#fire_rate,
                min = 0f32
            );
            self.r#fire_rate = 0f32;
        }
        if self.r#fire_rate > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#fire_rate", value = self.r#fire_rate,
                max = 100f32
            );
            self.r#fire_rate = 100f32;
        }
        if self.r#spread < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#spread", value = self.r#spread, min =
                0f32
            );
            self.r#spread = 0f32;
        }
        if self.r#spread > (360f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#spread", value = self.r#spread, max =
                360f32
            );
            self.r#spread = 360f32;
        }
        if self.r#magazine < (0f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#magazine", value = self.r#magazine, min
                = 0f32
            );
            self.r#magazine = 0f32;
        }
        if self.r#magazine > (1000000000f32 as i32) {
            tracing::warn!(
                "Field got truncated", field = "r#magazine", value = self.r#magazine, max
                = 1000000000f32
            );
            self.r#magazine = 1000000000f32;
        }
        if self.r#effect_size < (0f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#effect_size", value = self
                .r#effect_size, min = 0f32
            );
            self.r#effect_size = 0f32;
        }
        if self.r#effect_size > (100f32 as f32) {
            tracing::warn!(
                "Field got truncated", field = "r#effect_size", value = self
                .r#effect_size, max = 100f32
            );
            self.r#effect_size = 100f32;
        }
    }
}
