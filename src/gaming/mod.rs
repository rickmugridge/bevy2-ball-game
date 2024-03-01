use bevy::prelude::*;
use crate::AppState;
use crate::gaming::enemy::EnemyPlugin;
use crate::gaming::game::{GameOver, GamePlugin};
use crate::gaming::player::PlayerPlugin;
use crate::gaming::score::ScorePlugin;
use crate::gaming::stars::StarPlugin;

pub mod enemy;
pub mod player;
pub mod score;
pub mod stars;
pub mod game;
mod transform;
mod window;


pub struct GamingPlugin;

impl Plugin for GamingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugin(PlayerPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(StarPlugin)
            .add_plugin(ScorePlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}

pub fn pause_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Paused);
}

pub fn resume_simulation(mut simulation_state_next_state: ResMut<NextState<SimulationState>>) {
    simulation_state_next_state.set(SimulationState::Running);
}

pub fn toggle_simulation(mut commands: Commands,
                         kb: Res<Input<KeyCode>>,
                         simulation_state: Res<State<SimulationState>>) {
    if kb.just_pressed(KeyCode::Space) {
        match simulation_state.0 {
            SimulationState::Running => {
                commands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("Simulation Paused.");
            }
            SimulationState::Paused => {
                commands.insert_resource(NextState(Some(SimulationState::Running)));
                println!("Simulation Running.");
            }
        }
    }
}