use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;
use crate::{AppState, GameOver};
use crate::gaming::player::{Player, PLAYER_DIAMETER};
use crate::gaming::score::Score;
use crate::gaming::SimulationState;
use crate::gaming::transform::{collided, confine_translation};
use crate::gaming::window::random_within_window;

pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_DIAMETER: f32 = 64.0;
pub const ENEMY_SPAWN_TIME: f32 = 5.0;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            // .add_startup_system(spawn_enemies)
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_system(kill_enemies.in_schedule(OnExit(AppState::Game)))
            .add_systems((
                enemy_movement,
                update_enemy_direction,
                confine_enemies_movement,
                enemy_hit_player,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
            );
    }
}

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

pub struct EnemyPlugin;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        EnemySpawnTimer { timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating) }
    }
}

pub fn spawn_enemies(mut commands: Commands,
                     window_query: Query<&Window, With<PrimaryWindow>>,
                     asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUMBER_OF_ENEMIES {
        make_enemy(&mut commands, &asset_server, window);
    }
}

pub fn kill_enemies(mut commands: Commands,
                     enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
       commands.entity(enemy_entity).despawn();
    }
}

pub fn update_enemy_direction(mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
                              window_query: Query<&Window, With<PrimaryWindow>>,
                              audio: Res<Audio>,
                              asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();
    let half_enemy_size = ENEMY_DIAMETER / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }
        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            let sound_effect = if random::<f32>() > 0.5 { sound_effect_1 } else { sound_effect_2 };
            audio.play(sound_effect);
        }
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>,
                      time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn confine_enemies_movement(mut enemy_query: Query<&mut Transform, With<Enemy>>,
                                window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    for mut enemy_transform in enemy_query.iter_mut() {
        let mut translation = enemy_transform.translation;

        confine_translation(&mut translation, window, ENEMY_DIAMETER);
        enemy_transform.translation = translation;
    }
}

pub fn enemy_hit_player(mut commands: Commands,
                        mut game_over_event_writer: EventWriter<GameOver>,
                        mut player_query: Query<(Entity, &Transform), With<Player>>,
                        enemy_query: Query<&Transform, With<Enemy>>,
                        asset_server: Res<AssetServer>,
                        audio: Res<Audio>,
                        score: Res<Score>) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            if collided(player_transform, PLAYER_DIAMETER, enemy_transform, ENEMY_DIAMETER) {
                println!("Enemy hit player: Game over");
                let sound = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound);
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(mut commands: Commands,
                               window_query: Query<&Window, With<PrimaryWindow>>,
                               asset_server: Res<AssetServer>,
                               enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        make_enemy(&mut commands, &asset_server, window);
    }
}


fn make_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, window: &Window) {
    let direction = Vec2::new(random::<f32>(), random::<f32>()).normalize();
    commands.spawn((
        SpriteBundle {
            transform: random_within_window(window),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        },
        Enemy { direction }
    ));
}
