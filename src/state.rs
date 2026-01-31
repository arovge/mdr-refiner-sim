use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Leaderboard,
}

pub fn play(_click: On<Pointer<Click>>, mut commands: Commands) {
    commands.set_state(GameState::Playing);
}
