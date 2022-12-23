use crate::prelude::*;

pub struct CoordinatorPlugin;

impl Plugin for CoordinatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_level);
    }
}

impl CoordinatorPlugin {
    pub fn spawn_level(mut commands: Commands, graphics: Res<GraphicsResource>) {
        let ground_size = 500.0;
        let ground_height = 1.0;
        let player = Player::default();

        if let Some((texture, animation)) =
            graphics.get_player(&player.variant, &PlayerStates::Idle)
        {
            commands
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture.clone(),
                    sprite: TextureAtlasSprite::new(animation.start),
                    transform: Transform {
                        translation: Vec3::new(250.0, 250.0, 100.0),
                        scale: Vec3::splat(2.5),
                        ..default()
                    },
                    ..default()
                })
                .insert((
                    player,
                    player.new_state_machine(),
                    WithGravity,
                    WithAnimation::new(texture, animation),
                    RigidBody::KinematicPositionBased,
                    Collider::cuboid(13.0, 15.0),
                    KinematicCharacterController {
                        offset: CharacterLength::Absolute(0.01),
                        snap_to_ground: Some(CharacterLength::Relative(0.1)),
                        apply_impulse_to_dynamic_bodies: true,
                        ..default()
                    },
                    Name::new("Player"),
                ));
        }

        commands.spawn((
            TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
            Collider::cuboid(ground_size, ground_height),
        ));
    }
}
