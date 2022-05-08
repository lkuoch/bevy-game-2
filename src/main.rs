mod animation;
mod camera;
mod coordinator;
mod enemies;
mod graphics;
mod inspector;
mod player;
mod prelude;
mod sprites;

#[macro_use]
extern crate smart_default;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(WindowDescriptor {
            width: 900. * 16.0 / 9.0,
            height: 900.,
            title: "Rusty".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugin(InspectorPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(CoordinatorPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(CameraPlugin)
        .run();
}
