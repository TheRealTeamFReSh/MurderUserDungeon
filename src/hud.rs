use crate::apartment::player::Hunger;
use crate::apartment::player::PeePeePooPoo;
use crate::apartment::player::Sleepiness;
use crate::misc::day_cycle::DayCycleResource;
use crate::states::GameState;
use bevy::prelude;
use bevy::prelude::*;

pub struct Plugin;

impl prelude::Plugin for Plugin {
    fn build(&self, app: &mut prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::MainGame)
                .with_system(build_stat_hud.before("build_terminal"))
                .with_system(build_time_display.before("build_terminal")),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MainGame)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        )
        .add_system_set(SystemSet::on_exit(GameState::MainGame).with_system(despawn_ui))
        .add_system_set(
            SystemSet::on_update(GameState::ConsoleOpenedState)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PeepholeOpenedState)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PlayerPeeingState)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PlayerSleepingState)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PlayerOrderingPizzaState)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PlayerHidingState)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        )
        .add_system_set(
            SystemSet::on_update(GameState::PlayerEatingState)
                .with_system(refresh_stat_hud)
                .with_system(update_time_display),
        );
    }
}

#[derive(Component)]
pub struct Hud;

#[derive(Clone, Copy, Component)]
enum StatDisplay {
    Hunger,
    Sleep,
    PeePoo,
}

fn build_stat_hud(
    mut commands: Commands,
    ass: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use StatDisplay::*;

    let font = ass.load("fonts/VT323-Regular.ttf");

    spawn_stat_bar(&mut commands, Hunger, Color::GREEN.into(), &font);
    spawn_stat_bar(&mut commands, Sleep, Color::CYAN.into(), &font);
    spawn_stat_bar(&mut commands, PeePoo, Color::YELLOW.into(), &font);
}

fn spawn_stat_bar(commands: &mut Commands, stat: StatDisplay, color: UiColor, font: &Handle<Font>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(100.0),
                    height: Val::Px(10.0),
                },
                position: Rect {
                    left: Val::Percent(85.0),
                    bottom: match stat {
                        StatDisplay::Hunger => Val::Percent(15.0),
                        StatDisplay::Sleep => Val::Percent(20.0),
                        StatDisplay::PeePoo => Val::Percent(25.0),
                    },
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            color,
            ..NodeBundle::default()
        })
        .insert(stat)
        .insert(Hud);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::default(),
                position: Rect {
                    left: Val::Percent(79.0),
                    bottom: match stat {
                        StatDisplay::Hunger => Val::Percent(14.5),
                        StatDisplay::Sleep => Val::Percent(19.5),
                        StatDisplay::PeePoo => Val::Percent(24.5),
                    },
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::with_section(
                match stat {
                    StatDisplay::Hunger => "HUNGER",
                    StatDisplay::Sleep => "SLEEP",
                    StatDisplay::PeePoo => "PISS",
                },
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(Hud);
}

fn refresh_stat_hud(
    hunger: Res<Hunger>,
    sleepiness: Res<Sleepiness>,
    peepeepoopoo: Res<PeePeePooPoo>,
    mut query: Query<(&mut Style, &StatDisplay)>,
) {
    query.for_each_mut(|(mut style, stat)| match stat {
        StatDisplay::Hunger => {
            if hunger.is_changed() {
                style.size.width = Val::Px(hunger.0);
            }
        }
        StatDisplay::Sleep => {
            if sleepiness.is_changed() {
                style.size.width = Val::Px(sleepiness.0)
            }
        }
        StatDisplay::PeePoo => {
            if peepeepoopoo.is_changed() {
                style.size.width = Val::Px(peepeepoopoo.0)
            }
        }
    });
}

#[derive(Component)]
struct TimeDisplay;

fn build_time_display(
    mut commands: Commands,
    time: Res<DayCycleResource>,
    ass: ResMut<AssetServer>,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::default(),
                position: Rect {
                    left: Val::Percent(79.0),
                    bottom: Val::Percent(30.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::with_section(
                "TIME",
                TextStyle {
                    font: ass.load("fonts/VT323-Regular.ttf"),
                    font_size: 18.,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(Hud);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::default(),
                position: Rect {
                    left: Val::Percent(85.0),
                    bottom: Val::Percent(30.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::with_section(
                format!("{:02}:{:02}", time.get_hour(), time.get_minute()),
                TextStyle {
                    font: ass.load("fonts/VT323-Regular.ttf"),
                    font_size: 24.,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(TimeDisplay)
        .insert(Hud);
}

fn update_time_display(
    time: Res<DayCycleResource>,
    mut query: Query<&mut Text, With<TimeDisplay>>,
) {
    if time.is_changed() {
        query.for_each_mut(|mut text| {
            if let Some(mut section) = text.sections.first_mut() {
                section.value = format!("{:02}:{:02}", time.get_hour(), time.get_minute());
            }
        });
    }
}

fn despawn_ui(mut commands: Commands, query: Query<Entity, With<Hud>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
