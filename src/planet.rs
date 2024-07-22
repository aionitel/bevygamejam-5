use bevy::prelude::*;

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
            ..default()
        },
        ..default()
    });
}
