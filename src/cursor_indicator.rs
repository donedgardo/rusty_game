use bevy::app::App;
use bevy::prelude::{Added, Camera, Commands, Component, default, Entity, GlobalTransform, Plugin, Query, Res, SpriteBundle, Transform, Vec3, Window, Windows, With};
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::math::{Quat, Vec2};
use bevy::render::camera::RenderTarget;
use crate::player::Player;

#[derive(Component)]
struct CursorIndicator;

pub struct CursorIndicatorPlugin;

impl Plugin for CursorIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_cursor_indicator)
            .add_system(my_cursor_system);
    }
}

fn spawn_cursor_indicator(
    mut commands: Commands,
    player_q: Query<Entity, Added<Player>>,
    asset_server: Res<AssetServer>,
) {
    for player in player_q.iter() {
        commands.entity(player).with_children(|parent| {
            parent.spawn(CursorIndicator)
                .insert(SpriteBundle {
                    texture: asset_server.load("dungeon/cursor_indicator.png"),
                    ..default()
                });
        });
    }
}

fn my_cursor_system(
    windows: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut indicator_q: Query<(&mut Transform, &GlobalTransform), With<CursorIndicator>>,
) {
    let (camera, camera_transform) = q_camera.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let cursor_pos = get_cursor_translation(camera, camera_transform, wnd, screen_pos);
        for (mut indicator_transform, cursor_global_transform) in indicator_q.iter_mut() {
            let player_pos = cursor_global_transform.translation().truncate();
            let rotation = get_rotation_from_to(player_pos, cursor_pos);
            indicator_transform.rotation = rotation;
        }
    }
}

fn get_cursor_translation(camera: &Camera, camera_transform: &GlobalTransform, wnd: &Window, screen_pos: Vec2) -> Vec2 {
    let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
}

fn get_rotation_from_to(from: Vec2, to: Vec2) -> Quat {
    let diff = to - from;
    let angle = diff.y.atan2(diff.x);
    let rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
    rotation
}

#[cfg(test)]
mod indicator_cursor_test {
    use bevy::math::DVec2;
    use super::*;
    use bevy::prelude::*;
    use crate::camera::CameraPlugin;
    use crate::player::{Player};
    use crate::test_utils::{LoadTestPlugins, update};

    #[test]
    fn it_spawns_indicator_as_child_of_player() {
        let mut app = setup();
        app.update();
        let cursor = app.world
            .query_filtered::<&Parent, With<CursorIndicator>>().get_single(&app.world);
        assert!(cursor.is_ok());
    }

    #[test]
    fn it_has_a_sprite_sheet() {
        let mut app = setup();
        app.update();
        let sprite = app.world
            .query_filtered::<&Sprite, With<CursorIndicator>>()
            .get_single(&app.world);
        assert!(sprite.is_ok());
    }

    #[test]
    fn indicator_looks_at_mouse() {
        let mut app = setup();
        update(&mut app, 2);
        let cursor_transform = app.world
            .query_filtered::<&Transform, With<CursorIndicator>>()
            .single(&app.world);
        assert_eq!(cursor_transform.forward(), Vec3::new(0., 0., -1.));
    }

    fn setup() -> App {
        let windows = create_test_windows();
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(CameraPlugin)
            .add_plugin(CursorIndicatorPlugin)
            .insert_resource(windows);
        let _ = app.world.spawn(Player).id();
        app
    }

    fn create_test_windows() -> Windows {
        let mut windows = Windows::default();
        let mut test_window = Window::new(
            Default::default(),
            &Default::default(),
            100,
            100,
            1.0,
            None,
            None,
        );
        test_window.update_cursor_physical_position_from_backend(
            // position from bottom left of windows
            Option::from(DVec2::new(0., 50.))
        );
        windows.add(test_window);
        windows
    }
}
