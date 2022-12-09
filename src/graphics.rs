use crate::prelude::*;

#[derive(Debug, Resource)]
pub struct GraphicsResource {
    pub players: HashMap<PlayerType, GraphicsDescription<PlayerState>>,
}

impl GraphicsResource {
    pub fn get_player(
        &self,
        player: PlayerType,
        state: PlayerState,
    ) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
        if let Some(player) = self.players.get(&player) {
            if let Some(animation) = player.animation.get(&state) {
                return Some((player.texture.clone(), *animation));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct GraphicsDescription<S>
where
    S: std::hash::Hash + Eq,
{
    pub animation: HashMap<S, SpriteAnimation>,
    pub texture: Handle<TextureAtlas>,
}

pub struct GraphicsPlugin;
impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let sprite_config: SpriteConfig = match ron::from_str(
            &fs::read_to_string("config/sprites.ron").expect("sprite config file"),
        ) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load 'config/sprites.ron': {}", e);
                std::process::exit(1);
            }
        };

        // This will be injected as a world resource
        let mut players_map = HashMap::default();

        // Load players
        for player in sprite_config.players.iter() {
            let [rows, cols] = player.texture.row_cols;

            let texture_handle = texture_atlases.add(TextureAtlas::from_grid(
                assets.load(&player.texture.location),
                player.texture.tile_size,
                cols as usize,
                rows as usize,
                None,
                None,
            ));

            players_map.insert(
                player.entity_type,
                GraphicsDescription {
                    animation: player.animation.to_owned(),
                    texture: texture_handle,
                },
            );
        }

        commands.insert_resource(GraphicsResource {
            players: players_map,
        });
    }
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_graphics);
    }
}
