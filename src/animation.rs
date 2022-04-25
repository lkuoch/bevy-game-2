use crate::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct WithAnimation {
    pub timer: Timer,
    pub current_frame: usize,
    pub frames: Vec<usize>,
}

pub struct AnimationPlugin;

impl AnimationPlugin {
    fn frame_animation(
        mut sprites: Query<(&mut TextureAtlasSprite, &mut WithAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites.iter_mut() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Self::frame_animation);
    }
}

impl Default for WithAnimation {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, true),
            frames: Vec::new(),
            current_frame: 0,
        }
    }
}
