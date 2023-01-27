use bevy::prelude::*;
use bevy::asset::AssetServer;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/kongtext/kongtext.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ),
        GameLogs
    ));
}

#[derive(Component)]
struct GameLogs;

#[cfg(test)]
mod log_test {
    use super::*;
    use crate::test_utils;

    #[test]
    fn it_shows_log_text() {
        let mut app = App::new();
        app.add_plugins(test_utils::LoadTestPlugins);
        app.add_plugin(UIPlugin);

        app.update();
        let log_text = app.world
            .query_filtered::<&Text, With<GameLogs>>().single(&app.world);
        assert_eq!(log_text.sections[0].value, String::new());
    }
}
