use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(load_level)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_int_cell::<GroundBundle>(1)
        .add_system(debug_level)
        .run();
}

fn debug_level(query: Query<&Ground>) {
    println!("{}", query.iter().len());
}


#[derive(Component, Default)]
struct Ground;

#[derive(Bundle, LdtkIntCell)]
struct GroundBundle {
    ground: Ground,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk/levels.ldtk"),
        ..Default::default()
    });
}

#[cfg(test)]
mod ldtk_tests {
    use super::*;
    use bevy::app::App;
    use bevy::core_pipeline::CorePipelinePlugin;
    use bevy::render::RenderPlugin;
    use bevy::sprite::SpritePlugin;

    #[test]
    fn did_spawn_test_level() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugin(WindowPlugin::default())
            .add_plugin(AssetPlugin::default())
            .add_plugin(RenderPlugin::default())
            .add_plugin(ImagePlugin::default())
            .add_plugin(CorePipelinePlugin::default())
            .add_plugin(SpritePlugin::default())
            .add_plugin(LdtkPlugin)
            .add_startup_system(load_level)
            .insert_resource(LevelSelection::Index(0))
            .register_ldtk_int_cell::<GroundBundle>(1);
        app.update();
        app.update();
        app.update();
        assert_eq!(app.world.query::<&Ground>().iter(&app.world).len(), 24 * 24);
    }
}
