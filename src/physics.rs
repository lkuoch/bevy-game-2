use crate::prelude::*;

#[derive(Component, Debug)]
pub struct WithGravity;

pub struct PhysicsPlugin;

impl PhysicsPlugin {
    pub fn init_physics(mut rapier_config: ResMut<RapierConfiguration>) {
        rapier_config.gravity = Vec2::new(0.0, -200.0);
    }

    pub fn gravity(mut query: Query<(&WithGravity, &mut KinematicCharacterController)>) {
        for (_, mut controller) in query.iter_mut() {
            let translation = controller.translation.unwrap_or_default();
            controller.translation = Some(Vec2::new(translation.x, -10.0));
        }
    }

    pub fn player_movement(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(
            &Player,
            &mut TextureAtlasSprite,
            &mut KinematicCharacterController,
        )>,
    ) {
        for (player, mut sprite, mut controller) in &mut query.iter_mut() {
            let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
            let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
            let dir = (-(left as i8) + right as i8) as f32;
            let translation = controller.translation.unwrap_or_default();

            sprite.flip_x = left | (!right & sprite.flip_x);
            controller.translation = Some(Vec2::new(dir * player.move_speed, translation.y));
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(Self::init_physics)
            .add_system(Self::gravity)
            .add_system(Self::player_movement);
    }
}
