use crate::prelude::*;

#[derive(Debug, Deserialize, Clone)]
pub struct SpriteConfig {
    pub players: Vec<Sprites<PlayerVariant, PlayerStates>>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Sprites<T, S>
where
    T: std::hash::Hash + Eq,
    S: std::hash::Hash + Eq,
{
    pub entity_type: T,
    pub texture: SpriteTextureDesc,
    pub animation: HashMap<S, SpriteAnimation>,
}

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct SpriteTextureDesc {
    pub location: String,
    pub tile_size: Vec2,
    pub row_cols: [u32; 2],
}

#[derive(Deserialize, Copy, Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct SpriteAnimation {
    pub start: usize,
    pub frames: usize,
}
