use crate::prelude::*;

#[derive(Component, Debug, Inspectable)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub enemy_state: EnemyState,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum EnemyType {
    #[default]
    AngryPig,
    Bat,
    Bee,
    BlueBird,
    Bunny,
    Chameleon,
    Chicken,
    Duck,
    FatBird,
    Ghost,
    Mushroom,
    // Plant,
    // Radish,
    // Rhino,
    // Rock1,
    // Rock2,
    // Rock3,
    // Skull,
    // Slime,
    // Snail,
    // Trunk,
    // Turtle
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum EnemyState {
    #[default]
    AngryPig(AngryPigState),
    Bat(BatState),
    Bee(BeeState),
    BlueBird(BlueBirdState),
    Bunny(BunnyState),
    Chameleon(ChameleonState),
    Chicken(ChickenState),
    Duck(DuckState),
    FatBird(FatBirdState),
    Ghost(GhostState),
    Mushroom(MushroomState),
    // Plant,
    // Radish,
    // Rhino,
    // Rock1,
    // Rock2,
    // Rock3,
    // Skull,
    // Slime,
    // Snail,
    // Trunk,
    // Turtle
}

impl EnemyState {
    pub fn get_enemy_type(&self) -> EnemyType {
        match *self {
            EnemyState::AngryPig(_) => EnemyType::AngryPig,
            EnemyState::Bat(_) => EnemyType::Bat,
            EnemyState::Bee(_) => EnemyType::Bee,
            EnemyState::BlueBird(_) => EnemyType::BlueBird,
            EnemyState::Bunny(_) => EnemyType::Bunny,
            EnemyState::Chameleon(_) => EnemyType::Chameleon,
            EnemyState::Chicken(_) => EnemyType::Chicken,
            EnemyState::Duck(_) => EnemyType::Duck,
            EnemyState::FatBird(_) => EnemyType::FatBird,
            EnemyState::Ghost(_) => EnemyType::Ghost,
            EnemyState::Mushroom(_) => EnemyType::Mushroom,
        }
    }
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum AngryPigState {
    Hit1,
    Hit2,
    #[default]
    Idle,
    Run,
    Walk,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum BatState {
    CeilingIn,
    CeilingOut,
    Flying,
    Hit,
    #[default]
    Idle,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum BeeState {
    Attack,
    Hit,
    #[default]
    Idle,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum BlueBirdState {
    #[default]
    Flying,
    Hit,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum BunnyState {
    Fall,
    Hit,
    #[default]
    Idle,
    Jump,
    Run,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum ChameleonState {
    Attack,
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum ChickenState {
    Hit,
    #[default]
    Idle,
    Run,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum DuckState {
    Fall,
    Hit,
    #[default]
    Idle,
    Jump,
    JumpAnticipation,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum FatBirdState {
    Fall,
    Ground,
    Hit,
    #[default]
    Idle,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum GhostState {
    Appear,
    Disappear,
    Hit,
    #[default]
    Idle,
}

#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Display,
    EnumIter,
    Eq,
    Inspectable,
    Hash,
    PartialEq,
    SmartDefault,
)]
pub enum MushroomState {
    Hit,
    #[default]
    Idle,
    Run,
}
