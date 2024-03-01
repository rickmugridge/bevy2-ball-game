use bevy::prelude::*;

pub fn make_image(image: &str, asset_server: &Res<AssetServer>, parent: &mut ChildBuilder) {
    parent.spawn(ImageBundle {
        style: Style {
            size: Size::new(Val::Px(64.0), Val::Px(84.0)),
            margin: UiRect::new(Val::Px(8.0), Val::Px(8.0), Val::Px(8.0), Val::Px(8.0)),
            ..default()
        },
        image: asset_server.load(image).into(),
        ..default()
    });
}
