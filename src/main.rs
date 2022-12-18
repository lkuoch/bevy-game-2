mod animation;
mod camera;
mod coordinator;
mod enemies;
mod graphics;
mod inspector;
mod physics;
mod player;
mod prelude;
mod sprites;

use crate::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugin(CoordinatorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GraphicsPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(InspectorPlugin)
        .add_plugin(PhysicsPlugin)
        .run();
}
