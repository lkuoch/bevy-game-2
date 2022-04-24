use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PlayerAnimation {
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
