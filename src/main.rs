use bevy::prelude::*;
use level::LevelPlugin;
use player::PlayerPlugin;
use crate::camera::CameraPlugin;

mod level;
mod camera;
mod test_utils;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .run();
}


