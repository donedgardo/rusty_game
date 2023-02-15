use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use level::LevelPlugin;
use player::PlayerPlugin;
use crate::camera::CameraPlugin;
use crate::cursor_indicator::CursorIndicatorPlugin;
use crate::door::DoorPlugin;
use crate::game_logs::UIPlugin;
use crate::gamepad::GamepadPlugin;
use crate::movement::MyInputPlugin;
use crate::interaction::InteractionPlugin;

mod level;
mod camera;
mod test_utils;
mod player;
mod movement;
mod cursor_indicator;
mod physics_bundle;
mod game_logs;
mod door;
mod interaction;
mod gamepad;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(GamepadPlugin)
        .add_plugin(MyInputPlugin)
        .add_plugin(CursorIndicatorPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(DoorPlugin)
        .add_plugin(InteractionPlugin)
        .run();
}
