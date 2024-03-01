use bevy::prelude::*;
use crate::ui::buttons::{make_play_button, make_quit_button};
use crate::ui::image::make_image;
use crate::ui::text::make_text;

#[derive(Component)]
pub struct MainMenu {}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands.spawn(
        (
            NodeBundle {
                style: main_menu_style(),
                ..default()
            },
            MainMenu {},
        )
    )
        .with_children(|parent| {
            make_title(parent, asset_server);
            make_play_button(asset_server, parent);
            make_quit_button(asset_server, parent);
        })
        .id()
}

fn make_title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            size: Size::new(Val::Px(300.0), Val::Px(120.0)),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        make_image("sprites/ball_blue_large.png", asset_server, parent);
        parent.spawn(make_text("Bevy Ball Game", 64.0, asset_server));
        make_image("sprites/ball_red_large.png", asset_server, parent);
    });
}

fn main_menu_style() -> Style {
    Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
        ..default()
    }
}
