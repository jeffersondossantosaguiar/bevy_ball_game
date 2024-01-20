pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use crate::events::GameOver;
use crate::game::systems::{pause_simulation, resume_simulation, toggle_simulation};
use crate::AppState;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
