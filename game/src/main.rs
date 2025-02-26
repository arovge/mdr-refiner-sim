use bevy::{prelude::*, window::WindowResolution};
use game::GamePlugin;
use leaderboard::LeaderboardPlugin;
use menu::MenuPlugin;

mod game;
mod leaderboard;
mod menu;
mod state;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(game::WIDTH, game::HEIGHT)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin,
            MenuPlugin,
            LeaderboardPlugin,
            GamePlugin,
        ))
        .init_state::<state::GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
