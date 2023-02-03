use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::physics_bundle::CharacterPhysicsBundle;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_system(camera_follow);
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, LdtkEntity, Default)]
pub struct PlayerBundle {
    pub player: Player,
    /// ### `#[sprite_sheet_bundle...]`
    /// Similar to `#[sprite_bundle...]`, indicates that a [SpriteSheetBundle] field should be created
    /// with an actual material/image.
    /// There are two forms for this attribute:
    /// - `#[sprite_sheet_bundle("path/to/asset.png", tile_width, tile_height, columns, rows, padding,
    /// offset, index)]` will create the field using all of the information provided.
    /// Similar to using [TextureAtlas::from_grid()].
    /// - `#[sprite_sheet_bundle]` will create the field using information from the LDtk Editor visual,
    /// if it has one.
    #[bundle]
    #[sprite_sheet_bundle("dungeon/wizzard.png", 16.0, 32.0, 9, 1, 0.0, 0.0, 0)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub physics: CharacterPhysicsBundle,
}

fn camera_follow(
    mut commands: Commands,
    mut camera_q: Query<(Entity, &mut Transform), With<Camera>>,
    player_q: Query<(Entity, &Transform), (Added<Player>, Without<Camera>)>,
) {
    for (player, p_transform) in player_q.iter() {
        for (camera, mut transform) in camera_q.iter_mut() {
            commands.entity(player).add_child(camera);
            transform.translation.z -= p_transform.translation.z;
        }
    }
}

#[cfg(test)]
mod player_tests {
    use super::*;
    use bevy_rapier2d::prelude::*;
    use crate::level::LevelPlugin;
    use crate::{test_utils};
    use crate::camera::CameraPlugin;
    use crate::test_utils::LoadTestPlugins;

    #[test]
    fn player_spawns() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        assert_eq!(app.world.query::<&Player>().iter(&app.world).len(), 1)
    }

    #[test]
    fn player_has_sprite() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        assert_eq!(app.world.query::<(&Player, &TextureAtlasSprite)>().iter(&app.world).len(), 1);
    }

    #[test]
    fn player_has_camara_as_child() {
        let mut app = setup();
        test_utils::update(&mut app, 9);
        assert_eq!(app.world.query_filtered::<&Parent, With<Camera>>()
                       .iter(&app.world).len(), 1);
    }

    #[test]
    fn camera_does_not_exceed_default_clipping() {
        let mut app = setup();
        test_utils::update(&mut app, 9);
        let mut player_query = app.world.query::<(&Camera, &Transform)>();
        let mut camera_query = app.world.query::<(&Player, &Transform)>();
        for (_, p_transform) in player_query.iter(&app.world) {
            for (_, c_transform) in camera_query.iter(&app.world) {
                assert!(
                    p_transform.translation.z + c_transform.translation.z < 1000.,
                    "Camera global transform is not above 1000 (clipping issues)"
                );
            }
        }
    }

    #[test]
    fn has_a_collider() {
        let mut app = setup();
        test_utils::update(&mut app, 9);
        let collider_count = app.world.query_filtered::<&Collider, With<Player>>()
            .iter(&app.world).len();
        assert_eq!(collider_count, 1);
    }

    #[test]
    fn has_locked_rotation() {
        let mut app = setup();
        test_utils::update(&mut app, 9);
        let rotation_constraint = app.world.query_filtered::<&LockedAxes, With<Player>>().single(&app.world);
        assert_eq!(rotation_constraint, &LockedAxes::ROTATION_LOCKED);
    }

    #[test]
    fn has_zero_gravity() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let gravity = app.world.query_filtered::<&GravityScale, With<Player>>().single(&app.world);
        assert_eq!(gravity.0, 0.);
    }

    fn setup() -> App {
        let mut app = App::new();
        app
            .add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CameraPlugin);
        app
    }
}
