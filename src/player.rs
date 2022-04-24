use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Jump,
    DoubleJump,
    WallJump,
    Fall,
    Hit,
    Run,
}

#[derive(Debug, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PlayerType {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}
