use crate::prelude::*;

#[derive(Component, Debug, Inspectable)]
pub struct Player {
    pub player_type: PlayerType,
    pub player_state: PlayerState,
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
pub enum PlayerType {
    #[default]
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}
