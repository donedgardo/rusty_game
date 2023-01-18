use bevy::prelude::{Bundle, Component, Plugin, App, SpriteSheetBundle};
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Default)]
struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
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
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<PlayerBundle>("Player");
    }
}

#[cfg(test)]
mod player_tests {
    use bevy::prelude::*;
    use super::*;
    use crate::level::LevelPlugin;
    use crate::test_utils;
    use crate::test_utils::LoadTestPlugins;

    #[test]
    fn player_spawns() {
        let mut app = App::new();
        app
            .add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(PlayerPlugin);
        test_utils::load_level_cycles(&mut app);
        assert_eq!(app.world.query::<&Player>().iter(&app.world).len(), 1)
    }

    #[test]
    fn player_has_sprite() {
        let mut app = App::new();
        app
            .add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(PlayerPlugin);
        test_utils::load_level_cycles(&mut app);
        assert_eq!(app.world.query::<(&Player, &TextureAtlasSprite)>().iter(&app.world).len(), 1);
    }
}
