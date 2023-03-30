use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_rapier2d::prelude::*;
use game_state::GameState;
use level::LevelPlugin;
use player::PlayerPlugin;
use crate::animation::animation_system;

use crate::camera::CameraPlugin;
use crate::cursor_indicator::CursorIndicatorPlugin;
use crate::door::DoorPlugin;
use crate::game_logs::UIPlugin;
use crate::gamepad::GamepadPlugin;
use crate::movement::MyInputPlugin;
use crate::interaction::InteractionPlugin;

mod level;
mod camera;
mod player;
mod movement;
mod cursor_indicator;
mod physics_bundle;
mod game_logs;
mod door;
mod interaction;
mod gamepad;
mod game_state;
mod test_utils;
mod animation;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Option::from(camera::get_default_window()),
            ..default()
        })
        .set(ImagePlugin::default_nearest())
        .set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        .add_state::<GameState>()
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(GamepadPlugin)
        .add_plugin(MyInputPlugin)
        .add_plugin(CursorIndicatorPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(DoorPlugin)
        .add_plugin(InteractionPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(LevelSelection::Index(1))
        .add_system(animation_system);

    #[cfg(feature = "debug-mode")]
    {
        use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
        use bevy_inspector_egui::quick::WorldInspectorPlugin;

        app.add_plugin(WorldInspectorPlugin::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    app.run();
}


