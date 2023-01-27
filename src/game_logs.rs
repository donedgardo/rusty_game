use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameLogEvent>()
            .add_startup_system(setup_ui)
            .add_system(game_log_events)
            .add_system(mouse_scroll);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::new(
                Val::Percent(100.),
                Val::Percent(20.),
            ),
            overflow: Overflow::Hidden,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        max_size: Size::UNDEFINED,
                        ..default()
                    },
                    ..default()
                },
                ScrollingList::default(),
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/kongtext/kongtext.ttf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    ),
                    GameLogs
                ));
            });
    });
}

fn game_log_events(
    mut log_event_reader: EventReader<GameLogEvent>,
    mut text_query: Query<&mut Text, With<GameLogs>>,
) {
    for event in log_event_reader.iter() {
        let log = event.0.clone() + "\n";
        for mut text in text_query.iter_mut() {
            text.sections[0].value += &*log;
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Node)>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, uinode) in &mut query_list {
            let panel_height = uinode.size().y;
            let max_scroll = panel_height.max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            println!("{}", scrolling_list.position);
            style.position.top = Val::Px(scrolling_list.position);
        }
    }
}


#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component)]
struct GameLogs;

pub struct GameLogEvent(String);

#[cfg(test)]
mod log_test {
    use bevy::input::InputPlugin;
    use super::*;
    use crate::test_utils;

    #[test]
    fn it_shows_log_text() {
        let mut app = App::new();
        app.add_plugins(test_utils::LoadTestPlugins);
        app.add_plugin(InputPlugin);
        app.add_plugin(UIPlugin);

        app.update();
        let log_text = app.world
            .query_filtered::<&Text, With<GameLogs>>().single(&app.world);
        assert_eq!(log_text.sections[0].value, String::new());
    }

    #[test]
    fn it_updates_log_text() {
        let mut app = App::new();
        app.add_plugins(test_utils::LoadTestPlugins);
        app.add_plugin(InputPlugin);
        app.add_plugin(UIPlugin);

        app.world.send_event(GameLogEvent("Hello World!".to_string()));
        app.update();
        let log_text = app.world
            .query_filtered::<&Text, With<GameLogs>>().single(&app.world);
        assert_eq!(log_text.sections[0].value, "Hello World!\n".to_string());
    }

    #[test]
    fn it_scroll_to_the_bottom_as_new_text_comes_in() {
       let testLog = "Testing long text for helping me.\nTesting long text for helping me. longer than expected.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\n20!\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\nTesting long text for helping me.\n";
    }
}
