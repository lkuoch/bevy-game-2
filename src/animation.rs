use crate::prelude::*;

#[derive(Component, Reflect)]
pub struct WithAnimation {
    pub timer: Timer,
    pub config: SpriteAnimation,
    pub texture: Handle<TextureAtlas>,

    curr_idx: usize,
}

impl WithAnimation {
    pub fn new(texture: Handle<TextureAtlas>, anim: SpriteAnimation) -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            config: anim,
            texture,

            curr_idx: 0,
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::frame_animation)
            .add_system(Self::watch_player_model);
    }
}

impl AnimationPlugin {
    fn watch_player_model(
        mut query: Query<
            (&mut Handle<TextureAtlas>, &PlayerType, &PlayerState),
            Or<(Changed<PlayerType>, Changed<PlayerState>)>,
        >,
        graphics: Res<GraphicsResource>,
    ) {
        for (mut atlas, ptype, pstate) in query.iter_mut() {
            // TODO: Mapping between player states
            let nstate = match ptype {
                PlayerType::MaskDude => PlayerState::MaskDude(MaskDudeState::Idle),
                PlayerType::NinjaFrog => PlayerState::NinjaFrog(NinjaFrogState::Idle),
                PlayerType::PinkMan => PlayerState::PinkMan(PinkManState::Idle),
                PlayerType::VirtualGuy => PlayerState::VirtualGuy(VirtualGuyState::Idle),
            };

            println!("{:?}, {:?}", ptype, nstate);
            if let Some((texture, _)) = graphics.get_player(*ptype, nstate) {
                *atlas = texture;
            }
        }
    }

    fn frame_animation(
        mut query: Query<(&mut TextureAtlasSprite, &mut WithAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut anim) in query.iter_mut() {
            anim.timer.tick(time.delta());

            if anim.timer.just_finished() {
                sprite.index = anim.curr_idx + anim.config.start;
                anim.curr_idx = (anim.curr_idx + 1).rem_euclid(anim.config.frames);
            }
        }
    }
}
