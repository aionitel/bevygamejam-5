use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component, Debug)]
pub struct Planet {
    radius: f32,
    gravity: f32,
    position: Vec3,
}

impl Planet {
    pub fn position(&self) -> &Vec3 {
        &self.position
    }
}

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius: f32 = 200.;
    let position = Vec3::new(0., -1100., 0.);
    let texture = asset_server.load("fire_planet.png");
    let color = Color::hsl(360., 0.95, 0.7);

    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            scale: Vec3::splat(10.),
            translation: position,
            ..default()
        },
        ..default()
    })
    .insert(Planet { radius, gravity: 9.8, position })
    .with_children(|parent| {
        parent.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Annulus::new(radius / 2. - 1., radius / 2. ))),
            material: materials.add(color),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        });
    });
}
