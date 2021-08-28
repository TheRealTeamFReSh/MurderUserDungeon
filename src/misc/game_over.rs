use bevy::prelude::*;

use crate::{hud::Hud, states::GameState};

pub struct GameOverPlugin;

pub const DEATH_SOUND_DELAY: f32 = 3.6;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let mut game_over_sound_timer = Timer::from_seconds(DEATH_SOUND_DELAY, false);
        game_over_sound_timer.pause();
        app.insert_resource(GameOverAnimation {
            game_over_sound_timer,
            ..Default::default()
        });
        app.insert_resource(GameOverData {
            reason: None,
            hide_player_sprite: false,
        });
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
        app.add_system(set_game_over_message.system());
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GameOverReason {
    DoorLeftOpen,
    TooManyRageQuit,
    TicTacToeLosing,
    LabyrinthLosing,
    LabyrinthWinning,
    LetThemIn,
    NeedsDepleted,
}

impl GameOverReason {
    pub fn get_message(&self) -> &str {
        match self {
            GameOverReason::DoorLeftOpen => "Don't forget to close the door when focusing on something else.",
            GameOverReason::TooManyRageQuit => "You ragequitted too many times, \nyou went in fury mode and made an heart attack.",
            GameOverReason::TicTacToeLosing => "How dare you... lose a game of TicTacToe...\n(There is no AI, it's like losing to a baby...)",
            GameOverReason::LabyrinthLosing => "It's hard to stay alive there...\nMaybe in another life you will thrive and be a real MLG!",
            GameOverReason::LabyrinthWinning => "Congrats! What?! You really thought you could win?\n'Time to learn a lesson kiddo...",
            GameOverReason::LetThemIn => "Why break in when you can use the front door?",
            GameOverReason::NeedsDepleted => "You are a human too with needs.\n\"Next time be sure to eat enough\" would say grandma...",
        }
    }
}
pub struct GameOverData {
    pub reason: Option<GameOverReason>,
    pub hide_player_sprite: bool,
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
                Val::Px(current_window.height()),
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
            size: Size::new(Val::Percent(70.), Val::Percent(50.0)),
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
                vertical: VerticalAlign::Center,
            },
            sections: vec![TextSection {
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
                vertical: VerticalAlign::Center,
            },
            sections: vec![TextSection {
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

    commands
        .spawn_bundle(background)
        .insert(GameOverBackground)
        .insert(GameOverAnimationComponent)
        .with_children(|parent| {
            parent.spawn_bundle(container).with_children(|parent| {
                parent
                    .spawn_bundle(title_text_container)
                    .insert(TextTitleContainer)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(title_text)
                            .insert(TextTitle)
                            .insert(GameOverAnimationComponent);
                    });

                parent
                    .spawn_bundle(reason_text_container)
                    .insert(TextReasonContainer)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(reason_text)
                            .insert(GameOverAnimationComponent)
                            .insert(TextReason);
                    });
            });
        });
}

fn on_enter_game_over(mut anim_data: ResMut<GameOverAnimation>, time: Res<Time>) {
    #[cfg(debug_assertions)]
    info!("Enter game over state");
    anim_data.start_opacity = 0.0;
    anim_data.end_opacity = 1.0;
    anim_data.start_time = time.seconds_since_startup();
    anim_data.speed = 1.4;
    anim_data.game_over_sound_timer.unpause();
}

fn show_game_over_screen(mut query: Query<(&Transform, With<GameOverBackground>)>) {
    let (_transform, _) = query.single_mut().unwrap();
}

fn set_game_over_message(
    mut query: Query<(&mut Text, With<TextReason>)>,
    go_data: Res<GameOverData>,
) {
    for (mut text, _) in query.iter_mut() {
        if let Some(reason) = go_data.reason {
            text.sections[0].value = reason.get_message().to_string();
        }
    }
}

#[derive(Default)]
pub struct GameOverAnimation {
    pub start_opacity: f32,
    pub end_opacity: f32,
    pub start_time: f64,
    pub speed: f64,
    pub game_over_sound_timer: Timer,
}

pub fn apply_animation(
    mut mat_query: QuerySet<(
        Query<(
            &Node,
            &mut Handle<ColorMaterial>,
            With<GameOverAnimationComponent>,
        )>,
        Query<(
            &Node,
            &mut Handle<ColorMaterial>,
            With<Hud>,
        )>,
    )>,
    mut font_query: QuerySet<(
        Query<(
            &mut Text, 
            With<GameOverAnimationComponent>,
        )>,
        Query<(
            &mut Text, 
            With<Hud>,
        )>,
    )>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut anim_data: ResMut<GameOverAnimation>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let delta_t = time.seconds_since_startup() - anim_data.start_time;
    let background_value = 1.0 - (-(delta_t * anim_data.speed)).exp();

    // text fades in after fade to black
    let mut text_value = delta_t - 3.0;
    if text_value < 0.0 {
        text_value = 0.0;
    }
    if text_value > 1.0 {
        text_value = 1.0;
    }

    // changing material opacity
    for (_, color, _) in mat_query.q0_mut().iter_mut() {
        let color_mat = materials.get_mut(color.id).unwrap();
        color_mat.color.set_a(background_value as f32);
    }

    // changin font opacity
    for (mut text, _) in font_query.q0_mut().iter_mut() {
        for section in text.sections.iter_mut() {
            section.style.color.set_a(text_value as f32);
        }
    }

    // changing material opacity
    for (_, color, _) in mat_query.q1_mut().iter_mut() {
        let color_mat = materials.get_mut(color.id).unwrap();
        let col_val = 1.0 - background_value as f32;
        let prev_color = color_mat.color;
        color_mat.color = Color::rgba(
            prev_color.r() * col_val,
            prev_color.g() * col_val,
            prev_color.b() * col_val,
            prev_color.a() * col_val,
        );
    }

    // changin font opacity
    for (mut text, _) in font_query.q1_mut().iter_mut() {
        for section in text.sections.iter_mut() {
            let col_val = 1.0 - background_value as f32;
            let prev_color = section.style.color;
            section.style.color = Color::rgba(
                prev_color.r() * col_val,
                prev_color.g() * col_val,
                prev_color.b() * col_val,
                prev_color.a() * col_val,
            );
        }
    }

    // update timer to play death sound effect
    anim_data.game_over_sound_timer.tick(time.delta());
    if anim_data.game_over_sound_timer.just_finished() {
        audio.play(asset_server.load("audio/death.mp3"));
    }
}
