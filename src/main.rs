pub mod enemy;
pub mod events;
mod player;
pub mod score;
pub mod star;
mod systems;

use events::*;
use systems::*;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
        ))
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, (exit_game, handle_game_over))
        .run();
}
