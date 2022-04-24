use std::fs;

use crate::player::*;
use bevy::{prelude::*, utils::HashMap};
use ron::from_str;
use serde::Deserialize;

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

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

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Graphics(HashMap<Sprites, GraphicsDesc>);

#[derive(Debug, Clone)]
pub struct GraphicsDesc {
    pub texture: Handle<TextureAtlas>,
    pub animations: HashMap<SpriteAnimation, GraphicsSpriteDesc>,
}

#[derive(Debug, Clone)]
pub struct GraphicsSpriteDesc {
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
        let mut sprite_map = HashMap::default();

        // Load all sprites
        for (sprite, sprite_desc) in sprite_config.sprite_map.iter() {
            let texture_handle = texture_atlases.add(TextureAtlas::from_grid(
                assets.load(&sprite_desc.location),
                sprite_desc.tile_size,
                sprite_desc.cols,
                sprite_desc.rows,
            ));

            let mut animation_map = HashMap::default();

            // Load all animation clips of texture
            for (anim, anim_desc) in sprite_desc.sheets.iter() {
                let frames =
                    (anim_desc.start..(anim_desc.start + anim_desc.frames - 1)).collect::<Vec<_>>();

                animation_map.insert(*anim, GraphicsSpriteDesc { frames });
            }

            sprite_map.insert(
                *sprite,
                GraphicsDesc {
                    texture: texture_handle.clone(),
                    animations: animation_map,
                },
            );
        }

        commands.insert_resource(Graphics(sprite_map));
    }

    fn frame_animation(
        mut sprites: Query<(&mut TextureAtlasSprite, &mut FrameAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites.iter_mut() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_graphics)
            .add_system(Self::frame_animation);
    }
}
