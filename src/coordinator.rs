use crate::prelude::*;

pub struct CoordinatorPlugin;

impl Plugin for CoordinatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::spawn_level)
            .add_startup_system(Self::spawn_player);
    }
}

impl CoordinatorPlugin {
    pub fn spawn_level(mut commands: Commands) {
        let ground_size = 500.0;
        let ground_height = 1.0;

        commands.spawn((
            TransformBundle::from(Transform::from_xyz(0.0, -ground_height, 0.0)),
            Collider::cuboid(ground_size, ground_height),
        ));
    }

    pub fn spawn_player(mut commands: Commands, graphics: Res<GraphicsResource>) {
        let player_type = PlayerType::MaskDude;
        let player_state = PlayerState::Idle;

        if let Some((texture, animation)) = graphics.get_player(player_type, player_state) {
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
                    player_type,
                    player_state,
                    WithAnimation::new(texture, animation),
                    RigidBody::Dynamic,
                    Velocity::zero(),
                    Collider::cuboid(13.0, 15.0),
                    Restitution::coefficient(0.7),
                    Player::default(),
                    Name::new("Player"),
                ));
        }
    }

    // pub fn spawn_enemy_variants(mut commands: Commands, graphics: Res<GraphicsMap>) {
    // let mut spawn = |enemy_graphic: &Graphics,
    //                  enemy_state: EnemyState,
    //                  outer_idx: usize,
    //                  inner_idx: usize| {
    //     if let Some(anim) = enemy_graphic
    //         .animations
    //         .get(&Sprites::Enemy(SpriteModel::State(enemy_state)))
    //     {
    //         let enemy_type = enemy_state.get_enemy_type();

    //         commands
    //             .spawn_bundle(SpriteSheetBundle {
    //                 sprite: TextureAtlasSprite::new(anim.frames[0]),
    //                 texture_atlas: enemy_graphic.texture.clone(),
    //                 transform: Transform {
    //                     scale: Vec3::splat(2.0),
    //                     translation: Vec3::new(
    //                         ((outer_idx % 19) as f32 * 125.) - 1200.,
    //                         (inner_idx as f32 * 100. + 200.) - (800 * (outer_idx / 19)) as f32,
    //                         100.0,
    //                     ),
    //                     ..default()
    //                 },
    //                 ..default()
    //             })
    //             .insert(Enemy {
    //                 enemy_type,
    //                 enemy_state,
    //             })
    //             .insert(WithAnimation {
    //                 frames: anim.frames.to_vec(),
    //                 ..default()
    //             })
    //             .insert(Name::new(format!("{enemy_type}")));
    //     }
    // };

    // EnemyType::iter()
    //     .enumerate()
    //     .for_each(|(outer_idx, enemy_type)| {
    //         if let Some(enemy_graphic) =
    //             graphics.get(&Sprites::Enemy(SpriteModel::Type(enemy_type)))
    //         {
    //             match enemy_type {
    //                 EnemyType::AngryPig => {
    //                     AngryPigState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::AngryPig(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Bat => {
    //                     BatState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Bat(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Bee => {
    //                     BeeState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Bee(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::BlueBird => {
    //                     BlueBirdState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::BlueBird(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Bunny => {
    //                     BunnyState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Bunny(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Chameleon => {
    //                     ChameleonState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Chameleon(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Chicken => {
    //                     ChickenState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Chicken(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Duck => {
    //                     DuckState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Duck(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::FatBird => {
    //                     FatBirdState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::FatBird(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Ghost => {
    //                     GhostState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Ghost(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Mushroom => {
    //                     MushroomState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Mushroom(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Plant => {
    //                     PlantState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Plant(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Radish => {
    //                     RadishState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Radish(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Rhino => {
    //                     RhinoState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Rhino(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Rock1 => {
    //                     Rock1State::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Rock1(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Rock2 => {
    //                     Rock2State::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Rock2(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Rock3 => {
    //                     Rock3State::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Rock3(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Skull => {
    //                     SkullState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Skull(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Slime => {
    //                     SlimeState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Slime(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Snail => {
    //                     SnailState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Snail(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Trunk => {
    //                     TrunkState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Trunk(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //                 EnemyType::Turtle => {
    //                     TurtleState::iter()
    //                         .enumerate()
    //                         .for_each(|(inner_idx, variant)| {
    //                             spawn(
    //                                 enemy_graphic,
    //                                 EnemyState::Turtle(variant),
    //                                 outer_idx,
    //                                 inner_idx,
    //                             );
    //                         })
    //                 }
    //             }
    //         }
    //     });
    // }
}
