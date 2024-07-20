use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game Jam 5".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }),
    );
    app.add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
