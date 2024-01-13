pub mod events;
mod game;
mod main_menu;
mod systems;

use bevy::prelude::*;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MainMenuPlugin, GamePlugin))
        .add_state::<AppState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over, transition_states))
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
