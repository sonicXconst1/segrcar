use bevy::app::{EventReader, Plugin, AppBuilder};
use bevy::math::Vec2;
use bevy::window::{CursorMoved, Windows};
use bevy::ecs::system::{Query, Res, Commands, IntoSystem};
use bevy::transform::components::Transform;
use bevy::render::camera::Camera;
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
                setup.system().label(CursorPluginSystem::Setup));
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
    camera: Query<&Transform, With<Camera>>,
) {
}
