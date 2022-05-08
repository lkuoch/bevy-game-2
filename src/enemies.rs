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
    // Bunny,
    // Chameleon,
    // Chicken,
    // Duck,
    // FatBird,
    // Ghost,
    // Mushroom,
    // Plant,
    // Radish,
    // Rhino,
    // Rocks,
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
    // Bunny,
    // Chameleon,
    // Chicken,
    // Duck,
    // FatBird,
    // Ghost,
    // Mushroom,
    // Plant,
    // Radish,
    // Rhino,
    // Rocks,
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
