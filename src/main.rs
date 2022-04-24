mod animation;
mod graphics;
mod player;
mod prelude;
mod sprites;

use crate::prelude::*;

pub const RESOLUTION: f32 = 16.0 / 9.0;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_players(mut commands: Commands, graphics: Res<Graphics>) {
    spawn_player_type(
        PlayerType::MaskDude,
        SpriteAnimation::MaskDude(PlayerState::Idle),
        &mut commands,
        &graphics,
        Vec3::new(-100., 0.0, 100.0),
    );
    spawn_player_type(
        PlayerType::NinjaFrog,
        SpriteAnimation::NinjaFrog(PlayerState::Idle),
        &mut commands,
        &graphics,
        Vec3::new(-50., 0.0, 100.0),
    );
    spawn_player_type(
        PlayerType::PinkMan,
        SpriteAnimation::PinkMan(PlayerState::Idle),
        &mut commands,
        &graphics,
        Vec3::new(0.0, 0.0, 100.0),
    );
    spawn_player_type(
        PlayerType::VirtualGuy,
        SpriteAnimation::VirtualGuy(PlayerState::Idle),
        &mut commands,
        &graphics,
        Vec3::new(50.0, 0.0, 100.0),
    );
}

fn spawn_player_type(
    player_type: PlayerType,
    sprite_animation: SpriteAnimation,
    commands: &mut Commands,
    graphics: &Res<Graphics>,
    translation: Vec3,
) {
    if let Some(player) = graphics.get(&Sprites::Player(player_type)) {
        if let Some(anim) = player.animations.get(&sprite_animation) {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(anim[0]),
                    texture_atlas: player.texture.clone(),
                    transform: Transform {
                        scale: Vec3::splat(2.0),
                        translation,
                        ..default()
                    },
                    ..default()
                })
                .insert(WithAnimation {
                    frames: anim.to_vec(),
                    ..default()
                });
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .insert_resource(WindowDescriptor {
            width: 900. * RESOLUTION,
            height: 900.,
            title: "Rusty".to_string(),
            present_mode: PresentMode::Fifo,
            resizable: false,
            ..default()
        })
        .add_plugin(GraphicsPlugin)
        .add_plugin(AnimationPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_players)
        .run();
}
