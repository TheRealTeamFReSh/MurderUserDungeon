use crate::apartment::LIGHTING_Z;
use crate::states::GameState;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

pub const DAY_LENGTH: f32 = 240.0;
pub const STARTING_HOUR: f32 = 6.0;
pub const MAX_ALPHA: f32 = 0.92;
pub const MIN_ALPHA: f32 = 0.5;
pub const SLEEP_HOURS: f32 = 8.0;
pub struct DayCyclePlugin;

impl Plugin for DayCyclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(DayCycleResource {
            day_timer: Timer::from_seconds(DAY_LENGTH, true),
            day_length: DAY_LENGTH,
            max_alpha: MAX_ALPHA,
            min_alpha: MIN_ALPHA,
            sleep_hours: SLEEP_HOURS,
            days_passed: 0,
        })
        .add_system_set(
            SystemSet::on_enter(GameState::MainGame).with_system(setup_lighting_system.system()),
        )
        .add_system(day_cycle_system.system());
    }
}

pub struct DayCycleResource {
    pub day_timer: Timer,
    pub day_length: f32,
    pub max_alpha: f32,
    pub min_alpha: f32,
    pub sleep_hours: f32,
    pub days_passed: u8,
}

impl DayCycleResource {
    pub fn get_alpha(&self) -> f32 {
        ((1.0 - ((2.0 * PI * self.day_timer.elapsed_secs()) / (self.day_length * 2.0)).sin())
            * (self.max_alpha - self.min_alpha))
            + self.min_alpha
    }

    pub fn get_hour(&self) -> u8 {
        ((self.day_timer.elapsed_secs() / self.day_length) * 24.0) as u8
    }

    pub fn get_minute(&self) -> u8 {
        ((((self.day_timer.elapsed_secs() / self.day_length) * 24.0) - (self.get_hour() as f32))
            * 60.0) as u8
    }

    pub fn add_hours(&mut self, hours: f32) {
        let timer_length = (hours / 24.0) * self.day_length;
        if self.day_timer.elapsed_secs() + timer_length >= self.day_length {
            self.days_passed += 1;
            self.day_timer.set_elapsed(Duration::from_secs_f32(
                self.day_timer.elapsed_secs() + timer_length - self.day_length,
            ));
        } else {
            self.day_timer.set_elapsed(Duration::from_secs_f32(
                self.day_timer.elapsed_secs() + timer_length,
            ));
        }
    }

    pub fn sleep(&mut self) {
        self.add_hours(self.sleep_hours);
    }
}

fn day_cycle_system(
    mut lighting_query: Query<&mut Handle<ColorMaterial>, With<LightingComponent>>,
    mut day_cycle_resource: ResMut<DayCycleResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    day_cycle_resource.day_timer.tick(time.delta());
    if day_cycle_resource.day_timer.just_finished() {
        day_cycle_resource.days_passed += 1;
    }
    for color in lighting_query.iter_mut() {
        let color_mat = materials.get_mut(color.id).unwrap();
        /*
        println!(
            "day {} hour: {} minute: {}",
            day_cycle_resource.days_passed,
            day_cycle_resource.get_hour(),
            day_cycle_resource.get_minute()
        );
        */
        color_mat.color.set_a(day_cycle_resource.get_alpha());
    }
}

pub struct LightingComponent;

fn setup_lighting_system(
    mut commands: Commands,
    mut day_cycle_resource: ResMut<DayCycleResource>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    day_cycle_resource
        .day_timer
        .set_elapsed(Duration::from_secs_f32((STARTING_HOUR / 24.0) * DAY_LENGTH));

    let texture_handle: Handle<Texture> = asset_server.load("textures/lighting.png");
    let color_material = ColorMaterial {
        color: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: day_cycle_resource.get_alpha(),
        },
        texture: Some(texture_handle),
    };

    commands
        .spawn()
        .insert(LightingComponent)
        .insert_bundle(SpriteBundle {
            material: materials.add(color_material),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, LIGHTING_Z)),
            ..Default::default()
        })
        .insert(Name::new("Lighting"));
}
