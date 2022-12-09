use crate::prelude::*;

pub struct CameraPlugin;
impl CameraPlugin {
    fn setup_camera(mut commands: Commands) {
        commands.spawn(Camera2dBundle::default());
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup_camera);
    }
}
