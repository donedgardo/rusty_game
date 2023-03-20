use bevy::app::{PluginGroup, PluginGroupBuilder, ScheduleRunnerPlugin};
use bevy::time::TimePlugin;
use bevy::prelude::{App, FrameCountPlugin, Gamepad, HierarchyPlugin, ImagePlugin, TaskPoolPlugin,
                    TransformPlugin, TypeRegistrationPlugin, WindowPlugin};
use bevy::asset::AssetPlugin;
use bevy::render::RenderPlugin;
use bevy::core_pipeline::CorePipelinePlugin;
use bevy::sprite::SpritePlugin;
use bevy::input::gamepad::{GamepadConnection, GamepadConnectionEvent, GamepadInfo};

pub struct LoadTestPlugins;

impl PluginGroup for LoadTestPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TaskPoolPlugin::default())
            .add(TypeRegistrationPlugin::default())
            .add(FrameCountPlugin::default())
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

#[allow(dead_code)]
pub fn connect_test_gamepad(app: &mut App) {
    app.world.send_event(
        GamepadConnectionEvent::new(
            Gamepad { id: 1 },
            GamepadConnection::Connected(
                GamepadInfo { name: "test_gamepad".to_string() }
            )));
    app.update();
}



