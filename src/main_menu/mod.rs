use crate::states::GameState;
use bevy::{
    app::AppExit,
    prelude::{self, *},
};

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainMenu).with_system(build_main_menu.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainMenu).with_system(click_menu_item.system()),
        )
        .add_system_set(
            SystemSet::on_pause(GameState::MainMenu).with_system(despawn_menu_items.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MainMenu).with_system(despawn_menu_items.system()),
        );
    }
}

struct MainMenu;

#[derive(Clone, Copy)]
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
    commands.spawn_bundle(UiCameraBundle::default());

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
            visible: Visible {
                is_visible: false,
                ..Visible::default()
            },
            ..NodeBundle::default()
        })
        .insert(MainMenu)
        .with_children(|mut parent| {
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
            spawn_button(&mut parent, font.clone(), Play);
            spawn_button(&mut parent, font.clone(), Controls);
            spawn_button(&mut parent, font.clone(), Exit);
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
            Controls => todo!(),
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
