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
            title: "Rusty".to_string(),
            ..default()
        })
        .add_plugin(InspectorPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(CoordinatorPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(CameraPlugin)
        .run();
}
