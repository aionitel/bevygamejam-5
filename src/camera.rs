use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, player_camera);
    }
}

#[derive(Component)]
struct Camera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);
}

// Camera follows player, always centered on screen.
fn player_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player_q: Query<&Transform, With<Player>>,
    mut camera_q: Query<(&Camera, &mut Transform), Without<Player>>,
) {
    let player = player_q.single();
    let (_, mut camera) = camera_q.single_mut();

    // Set camera's coordinates to player's coordinates.
    camera.translation = player.translation;

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        camera.rotate_z(45.);
    }
}
