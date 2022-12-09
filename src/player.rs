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
    MaskDude(MaskDudeState),
    NinjaFrog(NinjaFrogState),
    PinkMan(PinkManState),
    VirtualGuy(VirtualGuyState),
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::MaskDude(MaskDudeState::Idle)
    }
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
pub enum MaskDudeState {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    #[default]
    Idle,
    Run,
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
pub enum NinjaFrogState {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    #[default]
    Idle,
    Run,
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
pub enum PinkManState {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    #[default]
    Idle,
    Run,
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
pub enum VirtualGuyState {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    #[default]
    Idle,
    Run,
}
