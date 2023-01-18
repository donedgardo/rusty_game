use bevy_ecs_ldtk::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Ground;

#[derive(Bundle, LdtkIntCell)]
pub struct GroundBundle {
    ground: Ground,
}

pub fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/levels.ldtk"),
        ..Default::default()
    });
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LdtkPlugin)
            .add_startup_system(load_level)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<GroundBundle>(1);
    }
}

#[cfg(test)]
mod level_tests {
    use bevy::prelude::*;
    use crate::level::{Ground, LevelPlugin};
    use crate::test_utils::{load_level_cycles, LoadTestPlugins};

    #[test]
    fn did_spawn_test_level() {
        let mut app = App::new();
        app
            .add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin);
        load_level_cycles(&mut app);
        assert_eq!(app.world.query::<&Ground>().iter(&app.world).len(), 24 * 24);
    }
}


