use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, Copy, EnumIter, Hash, PartialEq, Eq)]
pub enum PlayerState {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    Idle,
    Run,
}

#[derive(Debug, Deserialize, Clone, Copy, EnumIter, Hash, PartialEq, Eq)]
pub enum PlayerType {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}
