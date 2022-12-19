use crate::prelude::*;

pub struct PhysicsPlugin;

impl PhysicsPlugin {
    pub fn init_physics(mut rapier_config: ResMut<RapierConfiguration>) {
        rapier_config.gravity = Vec2::new(0.0, -200.0);
    }

    pub fn player_movement(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_info: Query<(&Player, &mut TextureAtlasSprite, &mut Velocity)>,
    ) {
        for (player, mut sprite, mut velocity) in &mut player_info {
            let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
            let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);
            let dir = (-(left as i8) + right as i8) as f32;

            sprite.flip_x = left | (!right & sprite.flip_x);
            velocity.linvel = Vec2::new(dir * player.move_speed, velocity.linvel.y)
        }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_startup_system(Self::init_physics)
            .add_system(Self::player_movement);
    }
}
