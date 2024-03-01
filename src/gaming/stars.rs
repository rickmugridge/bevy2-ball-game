use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::AppState;
use crate::gaming::player::{Player, PLAYER_DIAMETER};
use crate::gaming::score::Score;
use crate::gaming::SimulationState;
use crate::gaming::transform::collided;
use crate::gaming::window::random_within_window;

pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            .add_system(kill_stars.in_schedule(OnExit(AppState::Game)))
            .add_systems((
                star_hit_player,
                tick_star_spawn_timer,
                spawn_stars_over_time
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            );
    }
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        StarSpawnTimer { timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating) }
    }
}

pub fn spawn_stars(mut commands: Commands,
                   window_query: Query<&Window, With<PrimaryWindow>>,
                   asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_STARS {
        make_star(&mut commands, &asset_server, window);
    }
}

pub fn kill_stars(mut commands: Commands,
                  star_query: Query<Entity, With<Star>>) {
    for star_entity in star_query.iter() {
        commands.entity(star_entity).despawn();
    }
}

pub fn star_hit_player(mut commands: Commands,
                       player_query: Query<&Transform, With<Player>>,
                       star_query: Query<(Entity, &Transform), With<Star>>,
                       asset_server: Res<AssetServer>,
                       audio: Res<Audio>,
                       mut score: ResMut<Score>) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            if collided(player_transform, PLAYER_DIAMETER, star_transform, STAR_SIZE) {
                score.value += 1;
                let sound = asset_server.load("audio/impactMetal_heavy_001.ogg");
                audio.play(sound);
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(mut commands: Commands,
                             window_query: Query<&Window, With<PrimaryWindow>>,
                             asset_server: Res<AssetServer>,
                             star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        make_star(&mut commands, &asset_server, window);
    }
}


fn make_star(commands: &mut Commands, asset_server: &Res<AssetServer>, window: &Window) {
    commands.spawn((
        SpriteBundle {
            transform: random_within_window(window),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        },
        Star {}
    ));
}
