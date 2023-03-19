use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(set_zoom);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode =  ScalingMode::FixedVertical(1000.);
    commands.spawn(camera_bundle);
}

pub fn set_zoom(mut query: Query<&mut OrthographicProjection, Added<Camera>>) {
    for mut projection in query.iter_mut() {
        projection.scale = 0.35;
    }
}


#[cfg(test)]
mod camera_test {
    use super::*;
    use crate::test_utils::{LoadTestPlugins, update};

    #[test]
    fn spawns_camera() {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(CameraPlugin);
        update(&mut app, 3);
        assert_eq!(app.world.query::<&Camera>().iter(&app.world).len(), 1)
    }
}

pub fn get_default_window() -> Window {
    Window {
        fit_canvas_to_parent: true,
        ..default()
    }
}
