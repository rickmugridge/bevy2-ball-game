use bevy::prelude::*;
use crate::AppState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)))
            .add_system(update_score.run_if(in_state(AppState::Game)));
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

pub fn insert_score(mut commands:Commands){
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands:Commands) {
    commands.remove_resource::<Score>()
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}