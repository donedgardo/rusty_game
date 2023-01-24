use bevy::app::App;
use bevy::prelude::{Added, Camera, Commands, Component, default, Entity, GlobalTransform, Plugin, Query, Res, SpriteBundle, Transform, Windows, With};
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
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform)>,
    player_q: Query<&GlobalTransform, With<Player>>,
    mut cursor_q: Query<&mut Transform, With<CursorIndicator>>
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let cursor_pos: Vec2 = world_pos.truncate();

        for player_transform in player_q.iter() {
            let player_pos = player_transform.translation().truncate();
            player_transform.forward();
            let angle = (cursor_pos - player_pos).angle_between(player_pos);
            let rotation = Quat::from_rotation_z(-angle);
            for mut cursor_transform in cursor_q.iter_mut() {
                cursor_transform.rotation = rotation;
            }
        }
    }
}

#[cfg(test)]
mod indicator_cursor_test {
    use super::*;
    use bevy::prelude::*;
    use crate::player::Player;
    use crate::test_utils::LoadTestPlugins;

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

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(CursorIndicatorPlugin);
        let _ = app.world.spawn(Player).id();
        app
    }
}
