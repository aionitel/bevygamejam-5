use bevy::prelude::*;
use crate::animation::{AnimationType, FrameTime};
use crate::planet::Planet;

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement)
            .add_systems(Update, update_orbiting_planet);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct OrbitingPlanet(Planet);

fn update_orbiting_planet(
    mut commands: Commands,
    planets_q: Query<&Planet>,
) {
    for planet in planets_q.iter() {
        info!("{:?}", planet.position());
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Cut out player sprites from sheet.
    let texture = asset_server.load("player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 14, 4,  None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(
        SpriteBundle {
            texture,
            transform: Transform {
                scale: Vec3::splat(5.),
                translation: Vec3::new(0., -70., 1.),
                ..default()
            },
            ..default()
    })
    .insert(TextureAtlas {
        layout: texture_atlas_layout,
        ..default()
    })
    .insert(Player)
    .insert(AnimationType::Idle)
    .insert(FrameTime(0.5));
}

fn player_movement(
    mut player_pos_q: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.;
    let mut player_pos = player_pos_q.single_mut();

    let mut direction = Vec3::ZERO;
    if keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]) {
        direction += Vec3::new(-1., 0., 0.);
    }
    if keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]) {
        direction += Vec3::new(1., 0., 0.);
    }

    // Setting translation property to our own updated direction vector.
    // delta_seconds() returns time elapsed since last frame, used to make movement frame-rate independent.
    player_pos.translation += direction * SPEED * time.delta_seconds();
}
