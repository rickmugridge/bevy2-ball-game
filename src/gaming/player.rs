use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::AppState;
use crate::gaming::SimulationState;
use crate::gaming::transform::confine_translation;
use crate::gaming::window::centred;

pub const PLAYER_DIAMETER: f32 = 64.0;
pub const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_system(kill_player.in_schedule(OnExit(AppState::Game)))
            .add_system(player_movement
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running))
            );
    }
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(mut commands: Commands,
                    window_query: Query<&Window, With<PrimaryWindow>>,
                    asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: centred(window_query),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {}
    ));
}

pub fn kill_player(mut commands: Commands,
                   player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(kb_input: Res<Input<KeyCode>>,
                       mut player_query: Query<&mut Transform, With<Player>>,
                       window_query: Query<&Window, With<PrimaryWindow>>,
                       time: Res<Time>) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let direction = kb_direction(kb_input);
        let mut translation = transform.translation + direction * PLAYER_SPEED * time.delta_seconds();
        let window = window_query.get_single().unwrap();
        confine_translation(&mut translation, window, PLAYER_DIAMETER);
        transform.translation = translation;
    }
}

fn kb_direction(kb_input: Res<Input<KeyCode>>) -> Vec3 {
    let mut direction = Vec3::ZERO;
    if kb_input.pressed(KeyCode::Left) || kb_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if kb_input.pressed(KeyCode::Right) || kb_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if kb_input.pressed(KeyCode::Up) || kb_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if kb_input.pressed(KeyCode::Down) || kb_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    direction
}

