use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_inspector_egui::WorldInspectorPlugin;

pub const RESOLUTION: f32 = 16.0 / 9.0;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum AssetState {
    Loading,
    Done,
}

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct Player {
    speed: f32,
}

#[derive(AssetCollection)]
struct PlayerAssets {
    #[asset(
        path = "sprites/Players/Mask Dude/spritesheet.png",
        texture_atlas(tile_size_x = 32., tile_size_y = 32., columns = 12, rows = 5)
    )]
    mask_dude: Handle<TextureAtlas>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            sprite.index = (sprite.index + 1) % 11;
        }
    }
}

fn setup(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    // Setup camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);

    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_translation(Vec3::new(50., 30., 1.))
                .with_scale(Vec3::splat(2.)),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: player_assets.mask_dude.clone(),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player { speed: 10.0 });
}

fn main() {
    let mut app = App::new();

    AssetLoader::new(AssetState::Loading)
        .continue_to_state(AssetState::Done)
        .with_collection::<PlayerAssets>()
        .build(&mut app);

    app.add_state(AssetState::Loading)
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(AssetState::Done).with_system(setup))
        .add_system_set(SystemSet::on_update(AssetState::Done).with_system(animate_sprite))
        .run();
}
