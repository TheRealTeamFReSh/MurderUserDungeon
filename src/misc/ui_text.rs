use std::time::Duration;

use bevy::prelude::*;

use crate::states::GameState;

pub struct UITextPlugin;

impl Plugin for UITextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(TextUIData {
                duration_coef: 1./10.,
                content: String::from(""),
                fully_opened: false,
                is_opening: false,
                timer: Timer::from_seconds(4.0, false),
            })
            .insert_resource(TextUIAnimation {
                start_position: Vec2::ZERO,
                end_position: Vec2::ZERO,
                moving_speed: 5.0,
                start_time: 0.0,
            })
            .add_system_set(
                SystemSet::on_enter(GameState::MainGame)
                    .with_system(
                        build_ui.system()
                        .label("build_ui_text")
                        .before("build_terminal")
                    ),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainGame)
                    .with_system(set_ui_text.system())
                    .with_system(
                        apply_animation.system()
                        .label("ui_bottom_text_animation")
                    )
                    .with_system(
                        debug_open.system()
                        .after("ui_bottom_text_animation")
                    ),
            );
    }
}

pub struct TextUIData {
    pub duration_coef: f32,
    pub content: String,
    pub is_opening: bool,
    pub fully_opened: bool,
    pub timer: Timer,
}

impl TextUIData {
    pub fn show_text(
        &mut self, 
        anim_data: &mut ResMut<TextUIAnimation>,
        windows: &Res<Windows>,
        time: &Res<Time>, 
        content: String
    ) {
        let current_window = windows.get_primary().unwrap();

        // opening the ui
        self.is_opening = true;
        self.content = content.clone();
        
        // setting the open duration
        let duration = self.duration_coef * content.len() as f32;
        self.timer.set_duration(Duration::from_secs_f32(duration));

        // set the animation data
        anim_data.start_position = Vec2::new(
            0.2 * current_window.width(),
            -0.1 * current_window.height(),
        );
        anim_data.end_position = Vec2::new(
            0.2 * current_window.width(),
            0.01 * current_window.height(),
        );
        anim_data.start_time = time.seconds_since_startup();
    }
}

pub struct TextUIAnimation {
    pub start_position: Vec2,
    pub end_position: Vec2,
    pub start_time: f64,
    pub moving_speed: f64,
}

pub struct TextUIContainer;
pub struct TextUINode;

fn build_ui(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut anim_data: ResMut<TextUIAnimation>,
    data: Res<TextUIData>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let current_window = windows.get_primary().unwrap();
    let tran_col_h = materials.add(Color::rgba_u8(0, 0, 0, 0).into()); 

    let container = NodeBundle {
        material: tran_col_h,
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(0.6 * current_window.width()),
                Val::Px(0.1 * current_window.height()),
            ),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    };

    let text = TextBundle {
        text: Text {
            sections: vec![TextSection {
                value: data.content.clone(),
                style: TextStyle {
                    font_size: 25.0,
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    color: Color::WHITE,
                }
            }],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }
        },
        ..Default::default()
    };

    commands.spawn_bundle(container)
        .insert(TextUIContainer)
        .with_children(|parent| {
            parent.spawn_bundle(text)
                .insert(TextUINode);
        });

    let init_position = Vec2::new(
        0.2 * current_window.width(),
        -0.1 * current_window.height(),
    );
    anim_data.start_position = init_position;
    anim_data.end_position = init_position;
}

pub fn apply_animation(
    mut console_query: Query<(&TextUIContainer, &mut Style)>,
    mut anim_data: ResMut<TextUIAnimation>,
    mut data: ResMut<TextUIData>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    let current_window = windows.get_primary().unwrap();

    let delta_t = time.seconds_since_startup() - anim_data.start_time;
    let value = 1.0 - (-(delta_t * anim_data.moving_speed)).exp();
    let new_position = anim_data
        .start_position
        .lerp(anim_data.end_position, value as f32);

    if data.is_opening && new_position.abs_diff_eq(anim_data.end_position, 1.0) && !data.fully_opened {
        data.timer.reset();
        data.fully_opened = true;
    }
    if data.fully_opened {
        data.timer.tick(time.delta());
        if data.timer.finished() {
            data.fully_opened = false;
            data.is_opening = false;
            
            anim_data.start_position = anim_data.end_position;
            anim_data.end_position = Vec2::new(
                0.2 * current_window.width(),
                -0.1 * current_window.height(),
            );
            anim_data.start_time = time.seconds_since_startup();
        }
    }

    if let Ok((_, mut style)) = console_query.single_mut() {
        style.position.bottom = Val::Px(new_position.y);
        style.position.left = Val::Px(new_position.x);
    }
}

pub fn set_ui_text(
    mut query: Query<&mut Text, With<TextUINode>>,
    data: Res<TextUIData>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = data.content.clone();
    }
}

pub fn debug_open(
    keyboard_input: Res<Input<KeyCode>>,
    mut ui_data: ResMut<TextUIData>,
    mut anim_data: ResMut<TextUIAnimation>,
    windows: Res<Windows>,
    time: Res<Time>,
) {
    if keyboard_input.just_pressed(KeyCode::T) {
        ui_data.show_text(&mut anim_data, &windows, &time, String::from("Hello this is a text showing on screen"));
    }
}