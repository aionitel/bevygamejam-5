use bevy::prelude::*;
use std::collections::HashMap;
use crate::player::Player;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_systems(Update, animate_player);
    }
}

#[derive(Component)]
pub struct FrameTime(pub f32);

// Eq, PartialEq, and Hash necessary for animation to be inserted into HashMap world resource.
#[derive(Component, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum AnimationType {
    Idle,
    Walk,
    Run,
    Attack,
}

#[derive(Component, Clone, Debug)]
pub struct Animation {
    pub len: usize,
    pub frame_time: f32,
    pub path: String,
}

#[derive(Resource, Default)]
struct PlayerAnimations {
}

fn animate_player(
    mut frame_time_q: Query<&mut FrameTime, With<Player>>,
    mut sprite_q: Query<&mut TextureAtlas, With<Player>>,
    time: Res<Time>,
) {
    const ANIMATION_FRAME_TIME: f32 = 0.2;
    const LEN: usize = 4;

    let mut frame_time = frame_time_q.single_mut();
    let mut sprite = sprite_q.single_mut();

    frame_time.0 += time.delta_seconds();

    if frame_time.0 >= ANIMATION_FRAME_TIME {
        let frames_elapsed = frame_time.0 / ANIMATION_FRAME_TIME;

        // Animate!
        sprite.index += frames_elapsed as usize;

        // If current animation index becomes greater than or equal to size of animation, reset sprite index. (Restart animation)
        if sprite.index >= LEN { sprite.index = 0; }

        // Subtract total frames from frame_time as to not accumulate in frame_time.
        frame_time.0 -= ANIMATION_FRAME_TIME * frames_elapsed as f32;
    }
}
