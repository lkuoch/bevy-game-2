mod graphics;
mod player;

use bevy::{prelude::*, render::camera::ScalingMode, window::PresentMode};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use graphics::*;
use player::*;

pub const RESOLUTION: f32 = 16.0 / 9.0;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn test_spawn(mut commands: Commands, graphics: Res<Graphics>) {
    if let Some(mask_dude) = graphics.get(&Sprites::Player(PlayerType::MaskDude)) {
        if let Some(idle) = mask_dude
            .animations
            .get(&SpriteAnimation::MaskDude(PlayerAnimation::Idle))
        {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(idle.frames[0]),
                    texture_atlas: mask_dude.texture.clone(),
                    transform: Transform::from_scale(Vec3::splat(2.0)),
                    ..default()
                })
                .insert(FrameAnimation {
                    timer: Timer::from_seconds(0.1, true),
                    frames: idle.frames.to_vec(),
                    current_frame: 0,
                });
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::DARK_GREEN))
        .insert_resource(WindowDescriptor {
            width: 900. * RESOLUTION,
            height: 900.,
            title: "Rusty".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugin(GraphicsPlugin)
        .add_startup_system(setup)
        .add_startup_system(test_spawn)
        .run();
}
