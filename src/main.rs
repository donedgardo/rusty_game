use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use level::LevelPlugin;
use player::PlayerPlugin;
use crate::camera::CameraPlugin;
use crate::cursor_indicator::CursorIndicatorPlugin;
use crate::input::MyInputPlugin;

mod level;
mod camera;
mod test_utils;
mod player;
mod input;
mod cursor_indicator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(MyInputPlugin)
        .add_plugin(CursorIndicatorPlugin)
        //.add_plugin(RapierDebugRenderPlugin::default())
        .run();
}


