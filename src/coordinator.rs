use crate::prelude::*;

pub struct CoordinatorPlugin;
impl CoordinatorPlugin {
    pub fn spawn_player_variants(mut commands: Commands, graphics: Res<Graphics>) {
        // Render models
        for (player_idx, player_type) in PlayerType::iter().enumerate() {
            if let Some(player) = graphics.get(&Sprites::Player(SpriteModel::Type(player_type))) {
                // Render animations
                for (state_idx, player_state) in PlayerState::iter().enumerate() {
                    if let Some(anim) = player
                        .animations
                        .get(&Sprites::Player(SpriteModel::State(player_state)))
                    {
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                sprite: TextureAtlasSprite::new(anim.frames[0]),
                                texture_atlas: player.texture.clone(),
                                transform: Transform {
                                    scale: Vec3::splat(2.0),
                                    translation: Vec3::new(
                                        (player_idx as f32 * 75.) - 75.,
                                        -(state_idx as f32 * 75.) + 250.,
                                        100.0,
                                    ),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(Player {
                                player_type,
                                player_state,
                            })
                            .insert(WithAnimation {
                                frames: anim.frames.to_vec(),
                                ..default()
                            })
                            .insert(Name::new(format!("{player_type}")));
                    }
                }
            }
        }
    }

    pub fn spawn_enemy_variants(mut commands: Commands, graphics: Res<Graphics>) {
        let mut spawn = |enemy_graphic: &GraphicsDesc,
                         enemy_state: EnemyState,
                         outer_idx: usize,
                         inner_idx: usize| {
            if let Some(anim) = enemy_graphic
                .animations
                .get(&Sprites::Enemy(SpriteModel::State(enemy_state)))
            {
                let enemy_type = enemy_state.get_enemy_type();

                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite::new(anim.frames[0]),
                        texture_atlas: enemy_graphic.texture.clone(),
                        transform: Transform {
                            scale: Vec3::splat(2.0),
                            translation: Vec3::new(
                                (outer_idx as f32 * 75.) - 75.,
                                -(inner_idx as f32 * 75.) + 250.,
                                100.0,
                            ),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Enemy {
                        enemy_type,
                        enemy_state,
                    })
                    .insert(WithAnimation {
                        frames: anim.frames.to_vec(),
                        ..default()
                    })
                    .insert(Name::new(format!("{enemy_type}")));
            }
        };

        EnemyType::iter()
            .enumerate()
            .for_each(|(outer_idx, enemy_type)| {
                if let Some(enemy_graphic) =
                    graphics.get(&Sprites::Enemy(SpriteModel::Type(enemy_type)))
                {
                    match enemy_type {
                        EnemyType::AngryPig => {
                            AngryPigState::iter()
                                .enumerate()
                                .for_each(|(inner_idx, variant)| {
                                    spawn(
                                        enemy_graphic,
                                        EnemyState::AngryPig(variant),
                                        outer_idx,
                                        inner_idx,
                                    );
                                })
                        }
                        EnemyType::Bat => {
                            BatState::iter()
                                .enumerate()
                                .for_each(|(inner_idx, variant)| {
                                    spawn(
                                        enemy_graphic,
                                        EnemyState::Bat(variant),
                                        outer_idx,
                                        inner_idx,
                                    );
                                })
                        }
                        EnemyType::Bee => {
                            BeeState::iter()
                                .enumerate()
                                .for_each(|(inner_idx, variant)| {
                                    spawn(
                                        enemy_graphic,
                                        EnemyState::Bee(variant),
                                        outer_idx,
                                        inner_idx,
                                    );
                                })
                        }
                        EnemyType::BlueBird => {
                            BlueBirdState::iter()
                                .enumerate()
                                .for_each(|(inner_idx, variant)| {
                                    spawn(
                                        enemy_graphic,
                                        EnemyState::BlueBird(variant),
                                        outer_idx,
                                        inner_idx,
                                    );
                                })
                        }
                    }
                }
            });
    }
}

impl Plugin for CoordinatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_enemy_variants);
    }
}
