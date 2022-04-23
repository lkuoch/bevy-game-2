mod graphics;
mod player;

use std::fs;

use bevy::{prelude::*, window::PresentMode};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use graphics::GraphicsPlugin;
use ron::from_str;
use serde::{Deserialize, Serialize};

pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct Player {
    speed: f32,
}

fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_assets: ResMut<Assets<TextureAtlas>>,
) {
    let sprite_assets = fs::read_to_string("config/sprites.ron").expect("sprite config file");
}

fn setup(mut commands: Commands) {
    // Setup camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(WindowDescriptor {
            width: 900. * RESOLUTION,
            height: 900.,
            title: "Rusty".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugin(GraphicsPlugin)
        .run();
}
