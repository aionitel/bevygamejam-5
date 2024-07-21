use bevy::prelude::*;
use crate::animation::{AnimationType, FrameTime};

#[derive(Component)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    const SCALE: f32 = 5.;

    // Cut out player sprites from sheet.
    let texture = asset_server.load("player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 14, 4,  None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn(
            SpriteBundle {
                texture,
                transform: Transform::from_scale(Vec3::splat(SCALE)),
                ..default()
        })
        .insert(TextureAtlas {
            layout: texture_atlas_layout,
            ..default()
        })
        .insert(AnimationType::Idle)
        .insert(FrameTime(0.5))
        .insert(Player);
}
