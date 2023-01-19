use bevy::app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin};
use bevy::core::CorePlugin;
use bevy::time::TimePlugin;
use bevy::prelude::{App, ImagePlugin, WindowPlugin};
use bevy::asset::AssetPlugin;
use bevy::render::RenderPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::sprite::SpritePlugin;

pub struct LoadTestPlugins;

impl PluginGroup for LoadTestPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CorePlugin::default())
            .add(TimePlugin::default())
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



