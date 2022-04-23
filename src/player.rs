use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, Default, Hash, PartialEq, Eq)]
pub enum PlayerAnimation {
    #[default]
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
}
