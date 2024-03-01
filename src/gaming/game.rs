use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::app::AppExit;
use crate::AppState;
use crate::gaming::window::centred;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
            .add_system(exit_game)
            .add_system(handle_game_over);
    }
}

pub fn spawn_camera(mut commands: Commands,
                    window_query: Query<&Window, With<PrimaryWindow>>) {
    commands.spawn(Camera2dBundle {
        transform: centred(window_query),
        ..default()
    });
}

pub struct GameOver {
    pub score: u32,
}

pub fn exit_game(kb: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if kb.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn handle_game_over(mut commands: Commands, mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score);
        commands.insert_resource(NextState(Some(AppState::GameOver)));
    }
}
