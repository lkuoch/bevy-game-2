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
                                        (state_idx as f32 * 75.) - 175.,
                                        100.0,
                                    ),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(WithAnimation {
                                frames: anim.frames.to_vec(),
                                ..default()
                            });
                    }
                }
            }
        }
    }
}

impl Plugin for CoordinatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_player_variants);
    }
}
