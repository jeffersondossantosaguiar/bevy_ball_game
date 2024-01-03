use resources::*;
use systems::*;

use bevy::app::{App, Plugin, Startup, Update};

pub mod components;
mod resources;
mod systems;

pub const STAR_SIZE: f32 = 30.0; // This is the star sprite size.
pub const NUMBER_OF_STARS: usize = 10;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(Update, (tick_star_spawn_timer, spawn_stars_over_time));
    }
}
