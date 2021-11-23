use bevy::log::info;
use bevy::app::{EventReader, Plugin, AppBuilder};
use bevy::math::{Vec2, vec2};
use bevy::window::{CursorMoved, Windows};
use bevy::ecs::system::{Query, Res, Commands, IntoSystem};
use bevy::transform::components::Transform;
use bevy::render::camera::Camera;
use bevy::render::color::Color;
use bevy::ecs::query::With;
use bevy::ecs::schedule::SystemLabel;
use bevy::prelude::ParallelSystemDescriptorCoercion;

#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemLabel)]
pub enum CursorPluginSystem {
    Setup,
    CursorState,
}

pub struct CursorPlugin;

#[derive(Default)]
pub struct CursorState {
    pub position: Vec2
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(
                setup.system().label(CursorPluginSystem::Setup))
            .add_system(cursor_state.system().label(CursorPluginSystem::CursorState));
    }
}

fn setup(
    mut commands: Commands
) {
    commands.spawn().insert(CursorState::default());
}

fn cursor_state(
    mut commands: Commands,
    windows: Res<Windows>,
    mut events: EventReader<CursorMoved>,
    // camera is required to calculate world position.
    // camera is static right now, so we will use it later.
    _camera: Query<&Transform, With<Camera>>,
    mut cursor_state: Query<&mut CursorState>
) {
    let window = windows.get_primary().expect("Failed to get window.");
    let mut cursor_state = cursor_state.single_mut()
        .expect("More then 1 cursor");
    for event in events.iter() {
        let window_size = vec2(window.width(), window.height());
        let mouse_position_screen = event.position - window_size / 2f32;
        info!("Mouse position {:?}", mouse_position_screen);
        commands.spawn_bundle(crate::line::LineBundle::from_points(
            vec![
                crate::line::Point(
                    cursor_state.position.extend(0f32),
                    mouse_position_screen.extend(0f32)),
            ],
            Color::DARK_GREEN));
        cursor_state.position = mouse_position_screen;
    }
}
