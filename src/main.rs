use bevy::prelude::*;
use crate::gaming::enemy::{EnemySpawnTimer};
use crate::gaming::game::{GameOver};
use crate::gaming::{GamingPlugin, SimulationState};
use crate::gaming::stars::{StarSpawnTimer};
use crate::ui::UiPlugin;

mod gaming;
mod ui;

fn main() {
    App::new()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamingPlugin)
        .add_plugin(UiPlugin)
        .add_system(transition_to_game_state)
        .add_system(transition_to_ui_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

pub fn transition_to_game_state(mut commands: Commands,
                                kb_input: Res<Input<KeyCode>>,
                                app_state: Res<State<AppState>>) {
    if kb_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            commands.insert_resource(NextState(Some(AppState::Game)));
            println!("Entered AppState::Playing");
        }
    }
}

pub fn transition_to_ui_state(mut commands: Commands,
                              kb_input: Res<Input<KeyCode>>,
                              app_state: Res<State<AppState>>) {
    if kb_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            commands.insert_resource(NextState(Some(AppState::MainMenu)));
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Entered AppState::MainMenu");
        }
    }
}
