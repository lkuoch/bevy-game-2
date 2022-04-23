use std::fs;

use crate::player::*;
use bevy::{prelude::*, utils::HashMap};
use ron::from_str;
use serde::Deserialize;

#[derive(Copy, Clone, Debug, Reflect, Deserialize, Hash, PartialEq, Eq)]
pub enum Sprites {
    Player(PlayerType),
}

#[derive(Copy, Clone, Debug, Reflect, Deserialize, Hash, PartialEq, Eq)]
pub enum SpriteAnimation {
    MaskDude(PlayerAnimation),
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpriteAnimationDesc {
    start: usize,
    frames: usize,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SpritesDesc {
    pub location: String,
    pub tile_size: Vec2,
    pub cols: usize,
    pub rows: usize,
    pub sheets: HashMap<SpriteAnimation, SpriteAnimationDesc>,
}

#[derive(Debug, Deserialize, Clone)]
struct GraphicsConfig {
    sprite_map: HashMap<Sprites, SpritesDesc>,
}

#[derive(Debug, Clone)]
struct Graphics(HashMap<SpriteAnimation, GraphicsDesc>);

#[derive(Debug, Clone)]
struct GraphicsDesc {
    pub texture: Handle<TextureAtlas>,
    pub frames: Vec<usize>,
}

pub struct GraphicsPlugin;
impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let sprite_config: GraphicsConfig =
            match from_str(&fs::read_to_string("config/graphics.ron").expect("sprite config file"))
            {
                Ok(x) => x,
                Err(e) => {
                    println!("Failed to load config: {}", e);
                    std::process::exit(1);
                }
            };

        // This will be injected as a world resource
        let mut graphics = HashMap::default();

        // Load all sprites
        for (sprite, sprite_desc) in sprite_config.sprite_map.iter() {
            let texture_handle = texture_atlases.add(TextureAtlas::from_grid(
                assets.load(&sprite_desc.location),
                sprite_desc.tile_size,
                sprite_desc.cols,
                sprite_desc.rows,
            ));

            // Load all animation clips of texture
            for (anim, anim_desc) in sprite_desc.sheets.iter() {
                let frames =
                    (anim_desc.start..(anim_desc.start + anim_desc.frames - 1)).collect::<Vec<_>>();

                graphics.insert(
                    *anim,
                    GraphicsDesc {
                        texture: texture_handle.clone(),
                        frames,
                    },
                );
            }
        }

        commands.insert_resource(Graphics(graphics));
    }
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_graphics);
    }
}
