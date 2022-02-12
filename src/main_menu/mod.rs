use crate::states::GameState;
use bevy::{
    app::AppExit,
    prelude::{self, *},
};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(build_main_menu))
            .add_system_set(SystemSet::on_resume(GameState::MainMenu).with_system(build_main_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(click_menu_item))
            .add_system_set(
                SystemSet::on_pause(GameState::MainMenu).with_system(despawn_menu_items),
            )
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(despawn_menu_items))
            .add_system_set(
                SystemSet::on_enter(GameState::ControlMenu).with_system(build_control_menu),
            )
            .add_system_set(SystemSet::on_update(GameState::ControlMenu).with_system(return_button))
            .add_system_set(
                SystemSet::on_exit(GameState::ControlMenu).with_system(despawn_control_menu),
            );
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Clone, Copy, Component)]
enum MenuItem {
    Play,
    Controls,
    Exit,
}

fn build_main_menu(
    mut commands: Commands,
    ass: ResMut<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    let font = ass.load("fonts/VT323-Regular.ttf");
    clear_color.0 = Color::BLACK;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            visibility: Visibility {
                is_visible: false,
                ..Visibility::default()
            },
            ..NodeBundle::default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style { ..Style::default() },
                text: Text::with_section(
                    "MURDER-USER DUNGEON",
                    TextStyle {
                        font: font.clone(),
                        font_size: 36.0,
                        color: Color::ORANGE_RED,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });

            use MenuItem::*;
            spawn_button(parent, font.clone(), Play);
            spawn_button(parent, font.clone(), Controls);
            spawn_button(parent, font.clone(), Exit);
        });
}

fn spawn_button(parent: &mut ChildBuilder, font: Handle<Font>, item: MenuItem) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(10.),
                    height: Val::Px(30.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            ..ButtonBundle::default()
        })
        .insert(item)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style::default(),
                text: Text::with_section(
                    match item {
                        MenuItem::Play => "PLAY",
                        MenuItem::Controls => "CONTROLS",
                        MenuItem::Exit => "EXIT",
                    },
                    TextStyle {
                        font,
                        font_size: 20.0,
                        color: Color::DARK_GRAY,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });
        });
}

fn click_menu_item(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    query: Query<(&Interaction, &MenuItem)>,
) {
    use Interaction::*;
    use MenuItem::*;
    query.for_each(|(interaction, item)| match interaction {
        Clicked => match item {
            Play => {
                app_state
                    .push(GameState::MainGame)
                    .map_err(|err| error!("Failed to start game: {}", err))
                    .unwrap();
            }
            Controls => {
                app_state
                    .push(GameState::ControlMenu)
                    .map_err(|err| error!("Failed to open control menu: {}", err))
                    .unwrap();
            }
            Exit => app_exit_events.send(AppExit),
        },
        Hovered => {
            // Here you can add an effect on hover if you want c:
        }
        _ => {}
    });
}

fn despawn_menu_items(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}

#[derive(Component)]
struct ControlMenu;

#[derive(Component)]
struct ReturnButton;

fn build_control_menu(mut commands: Commands, ass: ResMut<AssetServer>) {
    let font = ass.load("fonts/VT323-Regular.ttf");

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                },
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                ..Style::default()
            },
            visibility: Visibility {
                is_visible: false,
                ..Visibility::default()
            },
            ..NodeBundle::default()
        })
        .insert(ControlMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                style: Style { ..Style::default() },
                text: Text::with_section(
                    "USE \"WASD\" TO MOVE AND \"E\" TO INTERACT WITH THINGS.\nUSE \"C\" AT THE DOOR TO LOOK THROUGH THE PEEPHOLE.\nUSE \"C\" AT THE BED TO HIDE UNDER IT.\n\n BE CAREFUL.\n\n YOUR APARTMENT SEEMS SAFE.",
                    TextStyle {
                        font: font.clone(),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..TextBundle::default()
            });
            parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(10.),
                        height: Val::Px(30.),
                    },
                    flex_direction: FlexDirection::ColumnReverse,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..Style::default()
                },
                ..ButtonBundle::default()
            })
            .insert(ReturnButton)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style::default(),
                    text: Text::with_section(
                        "RETURN",
                        TextStyle {
                            font,
                            font_size: 20.0,
                            color: Color::DARK_GRAY,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..TextBundle::default()
                });
            });
        });
}

fn despawn_control_menu(mut commands: Commands, query: Query<Entity, With<ControlMenu>>) {
    query.for_each(|entity| commands.entity(entity).despawn_recursive());
}

fn return_button(
    mut app_state: ResMut<State<GameState>>,
    query: Query<&Interaction, With<ReturnButton>>,
) {
    query.for_each(|interaction| match interaction {
        Interaction::Clicked => {
            #[cfg(debug_assertions)]
            info!("Popped game state");
            app_state
                .pop()
                .map_err(|err| error!("Failed to return to main menu: {}", err))
                .unwrap();
        }
        Interaction::Hovered => {}
        Interaction::None => {}
    });
}
