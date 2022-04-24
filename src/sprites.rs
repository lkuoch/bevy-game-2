use crate::prelude::*;

#[derive(Copy, Clone, Debug, Reflect, Deserialize, Hash, PartialEq, Eq)]
pub enum Sprites {
    Player(PlayerType),
}

#[derive(Copy, Clone, Debug, Reflect, Deserialize, Hash, PartialEq, Eq)]
pub enum SpriteAnimation {
    // Use `PlayerType` variants
    MaskDude(PlayerState),
    NinjaFrog(PlayerState),
    PinkMan(PlayerState),
    VirtualGuy(PlayerState),
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpritesDesc {
    pub location: String,
    pub tile_size: Vec2,
    pub cols: usize,
    pub rows: usize,
    pub sheets: HashMap<SpriteAnimation, SpriteAnimationDesc>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpriteAnimationDesc {
    pub start: usize,
    pub frames: usize,
}
