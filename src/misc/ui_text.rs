use std::time::Duration;

use bevy::prelude::*;

use crate::states::GameState;

pub struct UITextPlugin;

impl Plugin for UITextPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(BottomTextUI {
            ui_data: TextUIData {
                duration_coef: 1. / 10.,
                content: String::from(""),
                fully_opened: false,
                is_opening: false,
                timer: Timer::from_seconds(4.0, false),
                knows_anim_start: true,
            },
            animation: TextUIAnimation {
                start_position: Vec2::ZERO,
                end_position: Vec2::ZERO,
                moving_speed: 5.0,
                start_time: 0.0,
            },
            window_size: Vec2::ZERO,
        })
        .add_startup_system(setup_bundle.system())
        .add_system_set(
            SystemSet::on_enter(GameState::MainGame).with_system(
                build_ui
                    .system()
                    .label("build_ui_text")
                    .before("build_terminal"),
            ),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainGame)
                .with_system(despawn_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(set_ui_text.system())
                .with_system(apply_animation.system().label("ui_bottom_text_animation")),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::ConsoleOpenedState)
                .with_system(hide_text.system()),
        );
    }
}

pub fn hide_text(
    mut ui_bundle: ResMut<BottomTextUI>,
    windows: Res<Windows>,
    mut console_query: Query<(&TextUIContainer, &mut Style)>,
) {
    let current_window = windows.get_primary().unwrap();

    ui_bundle.ui_data.fully_opened = false;
    ui_bundle.ui_data.is_opening = false;
    
    let init_pos = Vec2::new(0.2 * current_window.width(), -0.1 * current_window.height());
    ui_bundle.animation.start_position = init_pos;
    ui_bundle.animation.end_position = init_pos;

    if let Ok((_, mut style)) = console_query.single_mut() {
        style.position.bottom = Val::Px(init_pos.y);
        style.position.left = Val::Px(init_pos.x);
    }
}

pub fn setup_bundle(
    mut ui_bundle: ResMut<BottomTextUI>,
    windows: Res<Windows>,
) {
    let current_window = windows.get_primary().unwrap();

    ui_bundle.window_size = Vec2::new(
        current_window.width(),
        current_window.height(),
    );
}

#[derive(Bundle)]
pub struct BottomTextUI {
    ui_data: TextUIData,
    animation: TextUIAnimation,
    window_size: Vec2,
}

pub struct TextUIData {
    pub duration_coef: f32,
    pub content: String,
    pub is_opening: bool,
    pub fully_opened: bool,
    pub timer: Timer,
    pub knows_anim_start: bool,
}

impl BottomTextUI {
    pub fn show_text(
        &mut self,
        content: String,
    ) {
        // opening the ui
        self.ui_data.is_opening = true;
        self.ui_data.content = content.clone();

        // setting the open duration
        let duration = self.ui_data.duration_coef * content.len() as f32;
        self.ui_data.timer.set_duration(Duration::from_secs_f32(duration));

        // set the animation data
        self.animation.start_position =
            Vec2::new(0.2 * self.window_size.x, -0.1 * self.window_size.y);
        self.animation.end_position =
            Vec2::new(0.2 * self.window_size.x, 0.01 * self.window_size.y);
        self.ui_data.knows_anim_start = false;
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
    mut ui_bundle: ResMut<BottomTextUI>,
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
                value: ui_bundle.ui_data.content.clone(),
                style: TextStyle {
                    font_size: 25.0,
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    color: Color::WHITE,
                },
            }],
            alignment: TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            },
        },
        ..Default::default()
    };

    commands
        .spawn_bundle(container)
        .insert(TextUIContainer)
        .with_children(|parent| {
            parent.spawn_bundle(text).insert(TextUINode);
        });

    let init_position = Vec2::new(0.2 * current_window.width(), -0.1 * current_window.height());
    ui_bundle.animation.start_position = init_position;
    ui_bundle.animation.end_position = init_position;
}

pub fn apply_animation(
    mut console_query: Query<(&TextUIContainer, &mut Style)>,
    mut ui_bundle: ResMut<BottomTextUI>,
    time: Res<Time>,
    windows: Res<Windows>,
) {
    let current_window = windows.get_primary().unwrap();

    if !ui_bundle.ui_data.knows_anim_start {
        ui_bundle.animation.start_time = time.seconds_since_startup();
        ui_bundle.ui_data.knows_anim_start = true;
    }

    let delta_t = time.seconds_since_startup() - ui_bundle.animation.start_time;
    let value = 1.0 - (-(delta_t * ui_bundle.animation.moving_speed)).exp();
    let new_position = ui_bundle.animation
        .start_position
        .lerp(ui_bundle.animation.end_position, value as f32);

    if ui_bundle.ui_data.is_opening
        && new_position.abs_diff_eq(ui_bundle.animation.end_position, 1.0)
        && !ui_bundle.ui_data.fully_opened
    {
        ui_bundle.ui_data.timer.reset();
        ui_bundle.ui_data.fully_opened = true;
    }
    if ui_bundle.ui_data.fully_opened {
        ui_bundle.ui_data.timer.tick(time.delta());
        if ui_bundle.ui_data.timer.finished() {
            ui_bundle.ui_data.fully_opened = false;
            ui_bundle.ui_data.is_opening = false;

            ui_bundle.animation.start_position = ui_bundle.animation.end_position;
            ui_bundle.animation.end_position =
                Vec2::new(0.2 * current_window.width(), -0.1 * current_window.height());
            ui_bundle.animation.start_time = time.seconds_since_startup();
        }
    }

    if let Ok((_, mut style)) = console_query.single_mut() {
        style.position.bottom = Val::Px(new_position.y);
        style.position.left = Val::Px(new_position.x);
    }
}

pub fn set_ui_text(mut query: Query<&mut Text, With<TextUINode>>, bundle: Res<BottomTextUI>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = bundle.ui_data.content.clone();
    }
}

pub fn despawn_ui(
    mut commands: Commands,
    query: Query<Entity, Or<(With<TextUIContainer>, With<TextUINode>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}