use crate::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct InspectorPlugin;
impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .register_type::<WithAnimation>()
            .register_inspectable::<Player>()
            .register_inspectable::<Enemy>();
    }
}
