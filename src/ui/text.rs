use bevy::prelude::*;

pub fn make_text(label: &str, font_size: f32, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: make_text_section(label, font_size, asset_server),
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

fn make_text_section(label: &str, font_size: f32, asset_server: &Res<AssetServer>) -> Vec<TextSection> {
    vec![TextSection::new(
        label,
        TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size,
            color: Color::WHITE,
            ..default()
        },
    )]
}
