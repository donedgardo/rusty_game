use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::input::Controllable;

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle, LdtkEntity)]
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
    #[sprite_sheet_bundle("dungeon/characters.png", 16.0, 32.0, 9, 8, 0.0, 0.0, 45)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[bundle]
    pub controller: Controllable,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player::default(),
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            controller: Controllable::default(),
        }
    }
}

pub struct PlayerPlugin;

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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<PlayerBundle>("Player")
            .add_system(camera_follow);
    }
}

#[cfg(test)]
mod player_tests {
    use bevy::prelude::*;
    use super::*;
    use crate::level::LevelPlugin;
    use crate::{test_utils};
    use crate::camera::CameraPlugin;
    use crate::test_utils::LoadTestPlugins;

    #[test]
    fn player_spawns() {
        let mut app = App::new();
        app
            .add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(PlayerPlugin);
        test_utils::update(&mut app, 3);
        assert_eq!(app.world.query::<&Player>().iter(&app.world).len(), 1)
    }

    #[test]
    fn player_has_sprite() {
        let mut app = App::new();
        app
            .add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(PlayerPlugin);
        test_utils::update(&mut app, 3);
        assert_eq!(app.world.query::<(&Player, &TextureAtlasSprite)>().iter(&app.world).len(), 1);
    }

    #[test]
    fn player_has_camara_as_child() {
        let mut app = App::new();
        app
            .add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CameraPlugin);
        test_utils::update(&mut app, 9);
        assert_eq!(app.world.query_filtered::<&Parent, With<Camera>>()
                       .iter(&app.world).len(), 1);
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
}
