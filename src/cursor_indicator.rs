use bevy::app::App;
use bevy::prelude::{Added, Commands, Component, default, Entity, Plugin, Query, Res, SpriteBundle};
use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use crate::player::Player;

#[derive(Component)]
struct CursorIndicator;

pub struct CursorIndicatorPlugin;

impl Plugin for CursorIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_cursor_indicator);
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
