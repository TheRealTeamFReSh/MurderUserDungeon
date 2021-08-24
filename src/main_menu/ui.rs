use bevy::prelude::*;

pub struct MainMenuUI;

pub fn build_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(
                Val::Percent(100.0),
                Val::Percent(100.0),
            ),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            text: Text{
                sections: vec![TextSection {
                    style: TextStyle {
                        font: asset_server.load("fonts/VT323-Regular.ttf"),
                        font_size: 40.,
                        color: Color::rgba_u8(102, 255, 102, 255),
                    },
                    value: "Rusty Jam".to_string(),
                }],
                ..Default::default()
            },
            ..Default::default()
        });
    })
    .insert(MainMenuUI);
}