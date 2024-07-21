use bevy::prelude::*;
use std::collections::HashMap;
use crate::player::Player;

#[derive(Component)]
pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAnimations>()
            .add_systems(Update, update_player_animation)
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
}

#[derive(Component, Clone, Debug, Default)]
pub struct Animation {
    pub index: usize,
    pub len: usize,
    pub frame_time: f32,
}

#[derive(Resource)]
struct PlayerAnimations {
    map: HashMap<AnimationType, Animation>,
}

impl FromWorld for PlayerAnimations {
    fn from_world(_world: &mut World) -> Self {
        const IDLE_FRAME_TIME: f32 = 0.13;
        const WALK_FRAME_TIME: f32 = 0.08;

        let mut animations = PlayerAnimations {
            map: HashMap::new(),
        };

        animations.map.insert(
            AnimationType::Idle,
            Animation {
                index: 0,
                len: 4,
                frame_time: IDLE_FRAME_TIME,
            }
        );
        animations.map.insert(
            AnimationType::Walk,
            Animation {
                index: 14,
                len: 14,
                frame_time: WALK_FRAME_TIME,
            }
        );

        animations
    }
}

fn update_player_animation(
    mut animation_q: Query<&mut AnimationType, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut animation = animation_q.single_mut();

    let new_animation = if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::KeyD, KeyCode::ArrowRight, KeyCode::ArrowLeft]) {
        AnimationType::Walk
    } else {
        AnimationType::Idle
    };

    *animation = new_animation;
}

fn animate_player(
    animation_q: Query<&AnimationType, With<Player>>,
    animation_res: Res<PlayerAnimations>,
    mut frame_time_q: Query<&mut FrameTime, With<Player>>,
    mut sprite_q: Query<&mut TextureAtlas, With<Player>>,
    time: Res<Time>,
) {
    let mut frame_time = frame_time_q.single_mut();
    let mut sprite = sprite_q.single_mut();
    let Some(animation) = animation_res.map.get(animation_q.single()) else {
        return;
    };

    if sprite.index < animation.index { sprite.index = animation.index; }

    frame_time.0 += time.delta_seconds();

    if frame_time.0 >= animation.frame_time {
        info!("{:?}", sprite.index);

        let frames_elapsed = frame_time.0 / animation.frame_time;

        // Animate!
        sprite.index += frames_elapsed as usize;

        // If current animation index becomes greater than or equal to size of animation, reset sprite index. (Restart animation)
        if sprite.index >= (animation.len + animation.index) { sprite.index = animation.index; }

        // Subtract total frames from frame_time as to not accumulate in frame_time.
        frame_time.0 -= animation.frame_time * frames_elapsed as f32;
    }
}
