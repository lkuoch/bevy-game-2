use crate::prelude::*;

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
