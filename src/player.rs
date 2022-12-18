use crate::prelude::*;

#[derive(
    Clone,
    Component,
    Copy,
    Derivative,
    Deserialize,
    FromReflect,
    Inspectable,
    PartialEq,
    Reflect,
    Serialize,
)]
#[derivative(Debug, Default)]
pub struct Player {
    #[derivative(Default(value = "500.0"))]
    pub move_speed: f32,
}

#[derive(
    Clone,
    Component,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    FromReflect,
    Inspectable,
    Hash,
    PartialEq,
    Reflect,
    Serialize,
)]
pub enum PlayerType {
    #[default]
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

#[derive(
    Clone,
    Component,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    FromReflect,
    Inspectable,
    Hash,
    PartialEq,
    Reflect,
    Serialize,
)]
pub enum PlayerState {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    #[default]
    Idle,
    Run,
}
