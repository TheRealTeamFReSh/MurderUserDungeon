use bevy::prelude::*;

use crate::states::GameState;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GameOverAnimation::default());
        app.add_system_set(
            SystemSet::on_enter(GameState::GameOverState)
                .with_system(on_enter_game_over.system())
                .with_system(build_ui.system()),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::GameOverState)
                .with_system(show_game_over_screen.system())
                .with_system(apply_animation.system()),
        );    
    }
}

pub struct GameOverBackground;
pub struct TextTitleContainer;
pub struct TextTitle;
pub struct TextReasonContainer;
pub struct TextReason;
pub struct GameOverAnimationComponent;

fn build_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    window: Res<Windows>,
) {
    let current_window = window.get_primary().unwrap();
    let transparent_col = Color::rgba_u8(0, 0, 0, 0);

    let background = NodeBundle {
        material: materials.add(Color::rgba_u8(0, 0, 0, 255).into()),
        style: Style {
            size: Size::new(
                Val::Px(current_window.width()), 
                Val::Px(current_window.height())
            ),
            position: Rect {
                top: Val::Px(0.),
                left: Val::Px(0.),
                ..Default::default()
            },
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..Default::default()
    };
    let container = NodeBundle {
        material: materials.add(transparent_col.into()),
        style: Style {
            size: Size::new(
                Val::Percent(70.),
                Val::Percent(50.0),
            ),
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..Default::default()
    };

    let title_text_container = NodeBundle {
        style: Style {
            size: Size {
                height: Val::Percent(50.),
                width: Val::Percent(100.),
            },
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexEnd,
            ..Default::default()
        },
        material: materials.add(transparent_col.into()),
        ..Default::default()
    };
    let title_text = TextBundle {
        text: Text {
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center, 
                vertical: VerticalAlign::Center
            },
            sections: vec![TextSection{
                style: TextStyle {
                    color: Color::RED,
                    font_size: 40.0,
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                },
                value: "Game Over".to_string(),
            }],
        },
        ..Default::default()
    };

    let reason_text_container = NodeBundle {
        material: materials.add(transparent_col.into()),
        style: Style {
            padding: Rect {
                top: Val::Percent(10.),
                ..Default::default()
            },
            size: Size {
                height: Val::Percent(50.),
                width: Val::Percent(100.),
            },
            flex_direction: FlexDirection::ColumnReverse,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            ..Default::default()
        },
        ..Default::default()
    };
    let reason_text = TextBundle {
        text: Text {
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center, 
                vertical: VerticalAlign::Center
            },
            sections: vec![TextSection{
                style: TextStyle {
                    color: Color::GRAY,
                    font_size: 30.0,
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                },
                value: "You died because of <insert reason here>".to_string(),
            }],
        },
        ..Default::default()
    };

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(background)
        .insert(GameOverBackground)
        .insert(GameOverAnimationComponent)
        .with_children(|parent| {
            parent.spawn_bundle(container).with_children(|parent| {
                parent.spawn_bundle(title_text_container)
                    .insert(TextTitleContainer)
                    .with_children(|parent| {
                        parent.spawn_bundle(title_text)
                            .insert(TextTitle)
                            .insert(GameOverAnimationComponent);
                    });

                parent.spawn_bundle(reason_text_container)
                    .insert(TextReasonContainer)
                    .with_children(|parent| {
                        parent.spawn_bundle(reason_text)
                            .insert(GameOverAnimationComponent)
                            .insert(TextReason);
                    });
            });
        });
}

fn on_enter_game_over(
    mut anim_data: ResMut<GameOverAnimation>,
    time: Res<Time>,
) {
    info!("Enter game over state");
    anim_data.start_opacity = 0.0;
    anim_data.end_opacity = 1.0;
    anim_data.start_time = time.seconds_since_startup();
    anim_data.speed = 0.4;
}

fn show_game_over_screen(
    mut query: Query<(&Transform, With<GameOverBackground>)>,
) {
    let (_transform, _) = query.single_mut().unwrap();
    //info!("{:?}", transform.translation);
}

#[derive(Default)]
pub struct GameOverAnimation {
    pub start_opacity: f32,
    pub end_opacity: f32,
    pub start_time: f64,
    pub speed: f64,
}

pub fn apply_animation(
    mut mat_query: Query<(&Node, &mut Handle<ColorMaterial>, With<GameOverAnimationComponent>)>,
    mut font_query: Query<(&mut Text, With<GameOverAnimationComponent>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    anim_data: Res<GameOverAnimation>,
    time: Res<Time>,
) {
    let delta_t = time.seconds_since_startup() - anim_data.start_time;
    let value = 1.0 - (-(delta_t * anim_data.speed)).exp();

    // changing material opacity
    for (_, color, _) in mat_query.iter_mut() {
        let color_mat = materials.get_mut(color.id).unwrap();
        color_mat.color.set_a(value as f32);
    }

    // changin font opacity
    for (mut text, _) in font_query.iter_mut() {
        for section in text.sections.iter_mut() {
            section.style.color.set_a(value as f32);
        }
    }
}