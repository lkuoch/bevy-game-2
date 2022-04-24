use crate::prelude::*;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Graphics(HashMap<Sprites, GraphicsDesc>);

#[derive(Debug, Clone)]
pub struct GraphicsDesc {
    pub texture: Handle<TextureAtlas>,
    pub animations: HashMap<SpriteAnimation, Vec<usize>>,
}

#[derive(Debug, Deserialize, Clone)]
struct GraphicsConfig {
    sprite_map: HashMap<Sprites, SpritesDesc>,
}

pub struct GraphicsPlugin;
impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let sprite_config: GraphicsConfig = match ron::from_str(
            &fs::read_to_string("config/graphics.ron").expect("sprite config file"),
        ) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load 'config/graphics.ron': {}", e);
                std::process::exit(1);
            }
        };

        // This will be injected as a world resource
        let mut sprite_map = HashMap::default();

        // Load all sprites
        for (sprite, sprite_desc) in sprite_config.sprite_map.iter() {
            let texture_handle = texture_atlases.add(TextureAtlas::from_grid(
                assets.load(&sprite_desc.location),
                sprite_desc.tile_size,
                sprite_desc.cols,
                sprite_desc.rows,
            ));

            let mut animation_map = HashMap::default();

            // Load all animation clips of texture
            for (anim, anim_desc) in sprite_desc.sheets.iter() {
                animation_map.insert(
                    *anim,
                    (anim_desc.start..(anim_desc.start + anim_desc.frames)).collect::<Vec<_>>(),
                );
            }

            sprite_map.insert(
                *sprite,
                GraphicsDesc {
                    texture: texture_handle.clone(),
                    animations: animation_map,
                },
            );
        }

        commands.insert_resource(Graphics(sprite_map));
    }
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, Self::load_graphics);
    }
}
