use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameLogEvent>()
            .add_startup_system(setup_ui)
            .add_system(debug_event)
            .add_system(game_log_events)
            .add_system(mouse_scroll);
    }
}

fn debug_event(
    mut ev: EventWriter<GameLogEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let test_log = "Testing long text for helping me. Long Long long long long long long long long!";
        ev.send(GameLogEvent(test_log.to_string()));
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::FlexEnd,
            size: Size::new(
                Val::Percent(35.),
                Val::Percent(10.),
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
            ));
    });
}


fn game_log_events(
    mut commands: Commands,
    mut log_event_reader: EventReader<GameLogEvent>,
    asset_server: Res<AssetServer>,
    list_query: Query<(Entity, &Node), With<ScrollingList>>,
) {
    for event in log_event_reader.iter() {
        let log = event.0.clone() + "\n";
        for (list, node) in list_query.iter() {
            commands.entity(list).with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        log.to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/kongtext/kongtext.ttf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                        },
                    ).with_style(Style {
                        flex_shrink: 0.,
                        size: Size::new(
                            Val::Px(node.size().x),
                            Val::Undefined,
                        ),
                        ..default()
                    }),
                    GameLogs
                ));
            });
        }
    }
}

fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Children, &Node)>,
    query_item: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, children, uinode) in &mut query_list {
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size().y)
                .sum();
            let panel_height = uinode.size().y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };
            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
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
    use bevy::text::TextPlugin;
    use bevy::ui::UiPlugin;
    use super::*;
    use crate::test_utils;
    use crate::test_utils::update;

    #[test]
    fn it_updates_log_text() {
        let mut app = setup();
        app.world.send_event(GameLogEvent("Hello World!".to_string()));
        app.update();
        let log_text = app.world
            .query_filtered::<&Text, With<GameLogs>>().single(&app.world);
        assert_eq!(log_text.sections[0].value, "Hello World!\n".to_string());
    }

    #[test]
    fn it_scrolls_on_mouse_scroll() {
        let mut app = setup();
        app.world.send_event(GameLogEvent("Hello World!Long LOng Long \n Long \n Long \n LOng\n very long! \n Very Long \n Very LOng \n Very Long".to_string()));
        update(&mut app, 2);
        app.world.send_event(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: -1.,
        });
        update(&mut app, 1);
        let scroll_list_style = app.world
            .query_filtered::<&Style, With<ScrollingList>>().single(&app.world);
        assert_eq!(scroll_list_style.position.top, Val::Px(-20.));
    }

    fn setup() -> App {
        let windows = test_utils::create_test_windows();
        let mut app = App::new();
        app.add_plugins(test_utils::LoadTestPlugins);
        app.add_plugin(TextPlugin::default());
        app.add_plugin(UiPlugin::default());
        app.add_plugin(InputPlugin);
        app.add_plugin(UIPlugin);
        app.insert_resource(windows);
        app
    }
}
