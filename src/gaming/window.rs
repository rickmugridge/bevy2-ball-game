use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn centred(window_query: Query<&Window, With<PrimaryWindow>>) -> Transform {
    let window = window_query.get_single().unwrap();
    Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
}

pub fn random_within_window(window: &Window) -> Transform {
    let x = random::<f32>() * window.width();
    let y = random::<f32>() * window.height();
    Transform::from_xyz(x, y, 0.0)
}