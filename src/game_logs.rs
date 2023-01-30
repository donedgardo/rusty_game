use bevy::prelude::*;
use bevy::asset::AssetServer;
use bevy::ecs::query::QueryIter;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<GameLogEvent>()
            .add_startup_system(setup_ui)
            .add_system(debug_event)
            .add_system(add_game_logs)
            .add_system(scroll_to_bottom_on_new_log)
            .add_system(scroll_logs_on_mouse_scroll);
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


fn add_game_logs(
    mut commands: Commands,
    mut log_event_reader: EventReader<GameLogEvent>,
    asset_server: Res<AssetServer>,
    list_query: Query<(Entity, &Node), With<ScrollingList>>,
) {
    for event in log_event_reader.iter() {
        let log_text = event.0.clone() + "\n";
        let list_query_result = list_query.get_single();
        if list_query_result.is_err() { return; };
        let (list, node) = list_query_result.unwrap();
        commands.entity(list).with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    log_text.to_string(),
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
                GameLog
            ));
        });
    }
}

fn scroll_to_bottom_on_new_log(
    log_node_changed_query: Query<Entity, (With<GameLog>, Changed<Node>)>,
    mut query_list: Query<(&mut Style, &Node, &mut ScrollingList)>,
    logs_query: Query<&Node, With<GameLog>>,
) {
    for _ in log_node_changed_query.iter() {
        let logs_height = get_game_logs_height(logs_query.iter());
        let query_list_result = query_list.get_single_mut();
        if query_list_result.is_err() { continue; }
        let (mut style, node, mut scroll) = query_list_result.unwrap();
        let panel_height = node.size().y;
        scroll.position = panel_height - logs_height;
        style.position.top = Val::Px(panel_height - logs_height);
    }
}


fn scroll_logs_on_mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Node)>,
    logs_query: Query<&Node, With<GameLog>>,
) {
    for mouse_wheel_event in mouse_wheel_events.iter() {
        for (mut scrolling_list, mut style, uinode) in &mut query_list {
            let logs_height = get_game_logs_height(logs_query.iter());
            let panel_height = uinode.size().y;
            let max_scroll = (logs_height - panel_height).max(0.);
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

fn get_game_logs_height(logs_iter: QueryIter<&Node, With<GameLog>>) -> f32 {
    let logs_height: f32 = logs_iter
        .map(|node| node.size().y)
        .sum();
    logs_height
}


#[derive(Component, Default)]
struct ScrollingList {
    position: f32,
}

#[derive(Component)]
struct GameLog;

pub struct GameLogEvent(String);

#[cfg(test)]
mod log_test {
    use bevy::input::InputPlugin;
    use bevy::text::TextPlugin;
    use bevy::ui::UiPlugin;
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn it_updates_log_text() {
        let mut app = setup();
        app.world.send_event(GameLogEvent("Hello World!".to_string()));
        app.update();
        let log_text = app.world
            .query_filtered::<&Text, With<GameLog>>().single(&app.world);
        assert_eq!(log_text.sections[0].value, "Hello World!\n".to_string());
    }

    #[test]
    fn it_scrolls_on_mouse_scroll() {
        let mut app = setup();
        app.world.send_event(GameLogEvent("Hello World!Long LOng Long \n Long \n Long \n LOng\n very long! \n Very Long \n Very LOng \n Very Long".to_string()));
        update(&mut app, 3);
        let original_top = get_scroll_list_top(&mut app);
        scroll_mouse(&mut app, 1.);
        update(&mut app, 2);
        let top_after_mouse_scroll = get_scroll_list_top(&mut app);
        let expected_top = original_top.try_add(Val::Px(20.)).unwrap();
        assert_eq!(top_after_mouse_scroll, expected_top);
        scroll_mouse(&mut app, -1.);
        update(&mut app, 2);
        let top_after_scrolling_back = get_scroll_list_top(&mut app);
        assert_eq!(original_top, top_after_scrolling_back);
    }

    #[test]
    fn it_scrolls_to_bottom_on_log_event() {
        let mut app = setup();
        app.world.send_event(GameLogEvent("Hello World!Long LOng Long \n Long \n Long \n LOng\n very long! \n Very Long \n Very LOng \n Very Long".to_string()));
        app.world.send_event(GameLogEvent("Hello World!Long LOng Long \n Long \n Long \n LOng\n very long! \n Very Long \n Very LOng \n Very Long".to_string()));
        update(&mut app, 3);
        let item_height: f32 = get_logs_height(&mut app);
        let (node, style, scroll) = app.world
            .query::<(&Node, &Style, &ScrollingList)>()
            .single(&app.world);
        let panel_height = node.size().y;
        let expected_top_value = item_height - panel_height;
        assert_eq!(style.position.top, Val::Px(-expected_top_value));
        assert_eq!(scroll.position, -expected_top_value);
    }

    fn get_logs_height(app: &mut App) -> f32 {
        app.world.query_filtered::<&Node, With<GameLog>>()
            .iter(&app.world)
            .map(|node| node.size().y)
            .sum()
    }

    fn get_scroll_list_top(app: &mut App) -> Val {
        app.world
            .query_filtered::<&Style, With<ScrollingList>>().single(&app.world)
            .position
            .top
    }

    fn scroll_mouse(app: &mut App, y: f32) {
        app.world.send_event(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y,
        });
    }

    fn setup() -> App {
        let windows = create_test_windows();
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins);
        app.add_plugin(TextPlugin::default());
        app.add_plugin(UiPlugin::default());
        app.add_plugin(InputPlugin);
        app.add_plugin(UIPlugin);
        app.insert_resource(windows);
        app
    }
}
