use crate::events::GameOver;

use crate::game::SimulationState;
use crate::AppState;
use bevy::app::AppExit;
use bevy::input::Input;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn((Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    },));
}

pub fn transition_states(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if (keyboard_input.just_pressed(KeyCode::G)) & (*app_state.get() != AppState::Game) {
        next_app_state.set(AppState::Game);
        println!("Entered AppState::Game");
    }

    if (keyboard_input.just_pressed(KeyCode::M)) & (*app_state.get() != AppState::MainMenu) {
        next_app_state.set(AppState::MainMenu);
        next_simulation_state.set(SimulationState::Paused);
        println!("Entered AppState::MainMenu");
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}
