use crate::prelude::*;

#[derive(Component, Debug, Inspectable)]
pub struct Player {
    pub player_type: PlayerType,
    pub player_state: PlayerState,
}

#[derive(Debug, Deserialize, Clone, Copy, EnumIter, Eq, Inspectable, Hash, PartialEq)]
pub enum PlayerState {
    DoubleJump,
    Fall,
    Jump,
    Hit,
    WallJump,
    Idle,
    Run,
}

#[derive(Debug, Deserialize, Clone, Copy, EnumIter, Eq, Inspectable, Hash, PartialEq)]
pub enum PlayerType {
    MaskDude,
    NinjaFrog,
    PinkMan,
    VirtualGuy,
}

impl fmt::Display for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PlayerState::DoubleJump => write!(f, "DoubleJump"),
            PlayerState::Fall => write!(f, "Fall"),
            PlayerState::Jump => write!(f, "Jump"),
            PlayerState::Hit => write!(f, "Hit"),
            PlayerState::WallJump => write!(f, "WallJump"),
            PlayerState::Idle => write!(f, "Idle"),
            PlayerState::Run => write!(f, "Run"),
        }
    }
}

impl fmt::Display for PlayerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PlayerType::MaskDude => write!(f, "MaskDude"),
            PlayerType::NinjaFrog => write!(f, "NinjaFrog"),
            PlayerType::PinkMan => write!(f, "PinkMan"),
            PlayerType::VirtualGuy => write!(f, "VirtualGuy"),
        }
    }
}
