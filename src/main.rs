use bevy::ecs::system::{
    IntoSystem,
    Query,
    Commands,
    Res,
    ResMut
};
use bevy::input::{Input, keyboard::KeyCode};
use bevy::sprite::ColorMaterial;
use bevy::asset::{Assets, AssetServer};
use bevy::core::Time;
use bevy::math::{Vec2, Vec3};
use bevy::transform::components::Transform;
use bevy::render::entity::OrthographicCameraBundle;
use bevy::sprite::{Sprite, entity::SpriteBundle};
use bevy::ecs::bundle::Bundle;

#[derive(Bundle)]
struct HeroBundle {
    name: Name,
    health: Health,
    position: Vec3,
}

struct Car;

struct Health(usize);

struct Name(String);

enum Collider {
    Floor
}

fn main() {
    bevy::app::App::build()
        .add_plugins(bevy::DefaultPlugins)
        .add_startup_system(startup.system())
        .add_system(greet_heroes.system())
        .add_system(keyboard_control.system())
        .run()
}

fn keyboard_control(
    input: Res<Input<KeyCode>>,
    mut cars: Query<(&Car, &mut Transform)>
) {
    let shift = 10f32;
    let mut position_shift = Vec3::ZERO;
    if input.pressed(KeyCode::Up) {
        position_shift.y += shift;
    }
    if input.pressed(KeyCode::Down) {
        position_shift.y -= shift;
    }
    if input.pressed(KeyCode::Right) {
        position_shift.x += shift;
    }
    if input.pressed(KeyCode::Left) {
        position_shift.x -= shift;
    }
    for mut transform in cars.iter_mut() {
        transform.1.translation += position_shift;
    }
}

fn startup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let blue_car_handle = assets_server.load("blue_car.png");
    let green_car_handle = assets_server.load("green_car.png");
    let red_car_handle = assets_server.load("red_car.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(blue_car_handle.into()),
            ..Default::default()
        }).insert(Car);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(green_car_handle.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 30.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Car);
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(red_car_handle.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 60.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Car);
}

fn greet_heroes(time: Res<Time>, query: Query<&Name>) {
    for hero in query.iter() {
        println!("Hello {}! time_elapsed: {:?}", hero.0, time.delta())
    }
}
