use bevy::prelude::*;
use crate::ui::text::make_text;

#[derive(Component)]
pub struct PlayButton {}

#[derive(Component)]
pub struct QuitButton {}

const BUTTON_SIZE: Size = Size::new(Val::Px(200.0), Val::Px(80.0));
const BUTTON_COLOUR: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_COLOUR: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_COLOUR: Color = Color::rgb(0.35, 0.75, 0.35);


pub fn make_play_button(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn(
        (button_bundle(), PlayButton {})
    )
        .with_children(|parent| {
            parent.spawn(make_text("Play", 32.0, asset_server));
        });
}

pub fn make_quit_button(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn(
        (button_bundle(), QuitButton {})
    )
        .with_children(|parent| {
            parent.spawn(make_text("Quit", 32.0, asset_server));
        });
}

fn button_bundle() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: BUTTON_SIZE,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BUTTON_COLOUR.into(),
        ..default()
    }
}
