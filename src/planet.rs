use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_planet);
    }
}

fn spawn_planet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("fire_planet.png");

    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            scale: Vec3::splat(10.),
            translation: Vec3::new(0., -1100., 0.),
            ..default()
        },
        ..default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::ball(100.))
    .insert(AdditionalMassProperties::Mass(0.))
    .insert(GravityScale(1.));
}
