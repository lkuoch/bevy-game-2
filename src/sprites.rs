use crate::prelude::*;

#[derive(Debug, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SpriteModel<T, S> {
    Type(T),
    State(S),
}

#[derive(Copy, Clone, Debug, Reflect, Deserialize, Hash, PartialEq, Eq)]
pub enum Sprites {
    Player(SpriteModel<PlayerType, PlayerState>),
    Enemy(SpriteModel<EnemyType, EnemyState>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpritesDesc {
    pub location: String,
    pub tile_size: Vec2,
    pub cols: usize,
    pub rows: usize,
    pub sheets: HashMap<Sprites, SpriteSheetDesc>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpriteSheetDesc {
    pub start: usize,
    pub frames: usize,
}
