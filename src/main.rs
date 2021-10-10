use bevy::window::Windows;
use bevy::sprite::collide_aabb::collide;
use bevy::ecs::system::{
    IntoSystem,
    Query,
    Commands,
    Res,
    ResMut
};
use bevy::input::{Input, keyboard::KeyCode};
use bevy::sprite::ColorMaterial;
use bevy::render::color::Color;
use bevy::asset::{Assets, AssetServer};
use bevy::core::Time;
use bevy::math::{Vec2, Vec3, Quat};
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

#[derive(Default)]
struct Car {
    crashed: bool
}

struct Health(usize);

struct Name(String);

enum Collider {
    Wall
}

fn main() {
    bevy::app::App::build()
        .add_plugins(bevy::DefaultPlugins)
        .add_startup_system(startup.system())
        .add_system(greet_heroes.system())
        .add_system(keyboard_control.system())
        .add_system(car_collision_system.system())
        .run()
}

fn keyboard_control(
    input: Res<Input<KeyCode>>,
    window: Res<Windows>,
    time: Res<Time>,
    mut cars: Query<(&Car, &mut Transform)>,
) {
    if input.pressed(KeyCode::R) {
        let shift_step = 30f32;
        let mut shift = Vec3::ZERO;
        for (_car, mut transform) in cars.iter_mut() {
            transform.translation = shift;
            transform.rotation = Quat::IDENTITY;
            shift.y += shift_step;
        }
        return;
    }
    let primary_window = window.get_primary().unwrap();
    let height = primary_window.height() / 10f32;
    let width = primary_window.width() / 10f32;
    let vertical_speed = height * time.delta_seconds(); 
    let horizontal_speed = width * time.delta_seconds();
    let mut position_shift = Vec3::ZERO;
    if input.pressed(KeyCode::Up) {
        position_shift.y += vertical_speed;
    }
    if input.pressed(KeyCode::Down) {
        position_shift.y -= vertical_speed;
    }
    if input.pressed(KeyCode::Right) {
        position_shift.x += horizontal_speed;
    }
    if input.pressed(KeyCode::Left) {
        position_shift.x -= horizontal_speed;
    }
    for (car, mut transform) in cars.iter_mut() {
        if car.crashed {
            transform.translation = Vec3::ZERO;
            transform.rotation = Quat::IDENTITY;
        }
        else {
            transform.translation += position_shift;
            if let Some(normalized_shift) = position_shift.try_normalize() {
                let angle = normalized_shift.angle_between(Vec3::ONE) / 360f32;
                println!("Angle {}", angle);
                let rotation = Quat::from_axis_angle(Vec3::new(0f32, 0f32, 1f32), angle);
                transform.rotation *= rotation;
            }
        }
    }
}

fn car_collision_system(
    mut cars: Query<(&mut Car, &Transform)>,
    colliders: Query<(&Collider, &Transform)>,
) {
    let car_size = Vec2::new(50f32, 30f32);
    for (mut car, car_transform) in cars.iter_mut() {
        let mut collision_happened = false;
        for (_collider, collider_transform) in colliders.iter() {
            let collision = collide(
                car_transform.translation,
                car_size,
                collider_transform.translation,
                collider_transform.scale.truncate() * Vec2::ONE);
            if let Some(collision) = collision {
                println!("Collision {:?}", collision);
                collision_happened = true;
                break;
            } 
        }
        car.crashed = collision_happened;
    }
}

fn startup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let blue_car_handle = assets_server.load("blue_car.png");
    let green_car_handle = assets_server.load("green_car.png");
    let red_car_handle = assets_server.load("red_car.png");

    let window = windows.get_primary().unwrap();
    let width = window.width();
    let height = window.height();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(blue_car_handle.into()),
            ..Default::default()
        }).insert(Car::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(green_car_handle.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 30.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Car::default());
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(red_car_handle.into()),
            transform: Transform {
                translation: Vec3::new(0.0, 60.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        }).insert(Car::default());

    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 1.0, 0.2).into()),
        sprite: Sprite {
            size: Vec2::new(1f32, 1f32),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(-width / 2f32, 0f32, 0f32),
            scale: Vec3::new(10f32, height, 0f32),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Collider::Wall);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 1.0, 0.2).into()),
        sprite: Sprite {
            size: Vec2::new(1f32, 1f32),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(width / 2f32, 0f32, 0f32),
            scale: Vec3::new(10f32, height, 0f32),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Collider::Wall);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 1.0, 0.2).into()),
        sprite: Sprite {
            size: Vec2::new(1f32, 1f32),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0f32, -height / 2f32, 0f32),
            scale: Vec3::new(width, 10f32, 0f32),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Collider::Wall);
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 1.0, 0.2).into()),
        sprite: Sprite {
            size: Vec2::new(1f32, 1f32),
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0f32, height / 2f32, 0f32),
            scale: Vec3::new(width, 10f32, 0f32),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Collider::Wall);
}

fn greet_heroes(time: Res<Time>, query: Query<&Name>) {
    for hero in query.iter() {
        println!("Hello {}! time_elapsed: {:?}", hero.0, time.delta())
    }
}
