use bevy::app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin};
use bevy::core::CorePlugin;
use bevy::time::TimePlugin;
use bevy::prelude::{App, HierarchyPlugin, ImagePlugin, TransformPlugin, Window, WindowPlugin, Windows};
use bevy::asset::AssetPlugin;
use bevy::render::RenderPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::sprite::SpritePlugin;
use bevy::math::DVec2;

pub struct LoadTestPlugins;

impl PluginGroup for LoadTestPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin::default())
            .add(TimePlugin::default())
            .add(TransformPlugin::default())
            .add(HierarchyPlugin::default())
            .add(ScheduleRunnerPlugin::default())
            .add(WindowPlugin::default())
            .add(AssetPlugin::default())
            .add(RenderPlugin::default())
            .add(ImagePlugin::default())
            .add(CorePipelinePlugin::default())
            .add(SpritePlugin::default())
    }
}

pub fn update(app: &mut App, cycles: usize) {
    for _ in 1..=cycles {
        app.update();
    }
}

pub fn create_test_windows() -> Windows {
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



