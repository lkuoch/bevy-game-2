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
            .add_system(Self::player_animation);
    }
}

impl AnimationPlugin {
    fn player_animation(
        mut query: Query<
            (
                &mut Handle<TextureAtlas>,
                &mut WithAnimation,
                &Player,
                &PlayerStates,
            ),
            Changed<PlayerStates>,
        >,
        graphics: Res<GraphicsResource>,
    ) {
        for (mut atlas, mut anim, player, state) in query.iter_mut() {
            if let Some((texture, animation)) = graphics.get_player(&player.variant, state) {
                anim.config = animation;
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
