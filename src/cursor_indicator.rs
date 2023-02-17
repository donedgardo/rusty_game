use bevy::app::App;
use bevy::prelude::{Added, Axis, Camera, Commands, Component, CursorMoved, default, Entity, EventReader, Gamepad, GamepadAxis, GlobalTransform, Plugin, Query, Res, SpriteBundle, Transform, Vec3, Window, Windows, With};
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::math::{Quat, Vec2};
use bevy::render::camera::RenderTarget;
use crate::gamepad;
use crate::gamepad::MyGamepad;
use crate::player::Player;

#[derive(Component)]
struct CursorIndicator;

pub struct CursorIndicatorPlugin;

impl Plugin for CursorIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_cursor_indicator)
            .add_system(my_gamepad_system)
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

fn my_gamepad_system(
    axes: Res<Axis<GamepadAxis>>,
    gamepad_res: Option<Res<MyGamepad>>,
    mut indicator_q: Query<&mut Transform, With<CursorIndicator>>,
) {
    if gamepad_res.is_none() { return; }
    let my_gamepad = gamepad_res.unwrap().0;
    let direction = gamepad::get_right_axis_direction(axes, my_gamepad);
    if direction.length() == 0. { return; }
    for mut transform in indicator_q.iter_mut() {
        transform.rotation = get_rotation_from_to(Vec2::ZERO, direction);
    }
}

fn my_cursor_system(
    windows: Res<Windows>,
    cursor_evr: EventReader<CursorMoved>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut indicator_q: Query<(&mut Transform, &GlobalTransform), With<CursorIndicator>>,
) {
    if cursor_evr.len() == 0 { return; }
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
    Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)
}

#[cfg(test)]
mod indicator_cursor_test {
    use bevy::input::InputPlugin;
    use super::*;
    use bevy::prelude::*;
    use bevy::prelude::GamepadAxisType::{RightStickX, RightStickY};
    use crate::camera::CameraPlugin;
    use crate::gamepad::GamepadPlugin;
    use crate::player::Player;
    use crate::test_utils::{connect_test_gamepad, create_test_windows, LoadTestPlugins, update};

    #[test]
    fn it_spawns_indicator_as_child_of_player() {
        let mut app = setup();
        let cursor = app.world
            .query_filtered::<&Parent, With<CursorIndicator>>().get_single(&app.world);
        assert!(cursor.is_ok());
    }

    #[test]
    fn it_has_a_sprite_sheet() {
        let mut app = setup();
        let sprite = app.world
            .query_filtered::<&Sprite, With<CursorIndicator>>()
            .get_single(&app.world);
        assert!(sprite.is_ok());
    }
    #[test]
    fn indicator_doesnt_looks_at_mouse_when_unmoved() {
        let mut app = setup();
        let cursor_transform = get_cursor_transform(&mut app);
        assert_eq!(cursor_transform.rotation.xyz(), Vec3::new(0., 0., 0.));
    }

    #[test]
    fn indicator_looks_at_mouse_when_moved() {
        let mut app = setup();
        app.world.send_event(CursorMoved {
            id: Default::default(),
            position: Vec2::new(0., 50.),
        });
        update(&mut app, 1);
        let cursor_transform = get_cursor_transform(&mut app);
        assert_eq!(cursor_transform.rotation.xyz(), Vec3::new(0., 0., 1.));
    }

    #[test]
    fn it_looks_towards_right_joystick_direction() {
        let mut app = setup();
        connect_test_gamepad(&mut app);
        update(&mut app, 1);
        move_gamepad_right_axis(&mut app, -1., 0.);
        update(&mut app, 1);
        let cursor_transform = get_cursor_transform(&mut app);
        assert_eq!(cursor_transform.rotation.xyz(), Vec3::new(0., 0., 1.));
    }

    #[test]
    fn it_ignores_right_joystick_when_its_zero() {
        let mut app = setup();
        connect_test_gamepad(&mut app);
        update(&mut app, 1);
        app.world.send_event(CursorMoved {
            id: Default::default(),
            position: Vec2::new(0., 50.),
        });
        update(&mut app, 1);
        move_gamepad_right_axis(&mut app, 0., 0.);
        update(&mut app, 1);
        let cursor_transform = get_cursor_transform(&mut app);
        assert_eq!(cursor_transform.rotation.xyz(), Vec3::new(0., 0., 1.));
    }

    fn move_gamepad_right_axis(app: &mut App, x_pos: f32, y_pos: f32) {
        let mut gamepad_axis = Axis::<GamepadAxis>::default();
        let gamepad_x_axis = GamepadAxis {
            gamepad: Gamepad { id: 1 },
            axis_type: RightStickX,
        };
        let gamepad_y_axis = GamepadAxis {
            gamepad: Gamepad { id: 1 },
            axis_type: RightStickY,
        };
        gamepad_axis.set(gamepad_x_axis, x_pos);
        gamepad_axis.set(gamepad_y_axis, y_pos);
        app.insert_resource(gamepad_axis);
    }

    fn get_cursor_transform(app: &mut App) -> &Transform {
        app.world
            .query_filtered::<&Transform, With<CursorIndicator>>()
            .single(&app.world)
    }

    fn setup() -> App {
        let mut app = App::new();
        let window = create_test_windows();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(InputPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(GamepadPlugin)
            .add_plugin(CursorIndicatorPlugin)
            .insert_resource(window);
        app.world.spawn(Player);
        app.update();
        app
    }
}
