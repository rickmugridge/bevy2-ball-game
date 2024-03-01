use bevy::prelude::{Transform, Vec3, Window};

pub fn confine_translation(translation: &mut Vec3,
                           window: &Window, diameter: f32) {
    let radius = diameter / 2.0;
    let x_min = 0.0 + radius;
    let x_max = window.width() - radius;
    let y_min = 0.0 + radius;
    let y_max = window.height() - radius;

    if translation.x < x_min {
        translation.x = x_min;
    } else if translation.x > x_max {
        translation.x = x_max;
    }
    if translation.y < y_min {
        translation.y = y_min;
    } else if translation.y > y_max {
        translation.y = y_max;
    }
}

pub fn collided(transform1: &Transform, diameter1: f32,
                transform2: &Transform, diameter2: f32) -> bool {
    let distance = transform1.translation.distance(transform2.translation);
    let player_radius = diameter1 / 2.0;
    let star_radius = diameter2 / 2.0;
    distance < player_radius + star_radius
}

