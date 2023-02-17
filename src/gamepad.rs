use bevy::prelude::{App, Commands, EventReader, Gamepad, GamepadAxis, GamepadAxisType, GamepadEvent, GamepadEventType, Plugin, Res, Resource};
use bevy::input::Axis;
use bevy::math::Vec2;

pub struct GamepadPlugin;

impl Plugin for GamepadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(gamepad_detection);
    }
}

#[derive(Resource)]
pub struct MyGamepad(pub Gamepad);

pub fn gamepad_detection(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.iter() {
        let id = ev.gamepad;
        match &ev.event_type {
            GamepadEventType::Connected(_) => {
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(id));
                }
            }
            GamepadEventType::Disconnected => {
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if *old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn get_right_axis_direction(axes: Res<Axis<GamepadAxis>>, gamepad: Gamepad) -> Vec2 {
    let axis_rx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::RightStickX,
    };
    let axis_ry = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::RightStickY,
    };
    get_axis_direction(axes, axis_rx, axis_ry)
}


pub fn get_left_axis_direction(axes: Res<Axis<GamepadAxis>>, gamepad: Gamepad) -> Vec2 {
    let axis_lx = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickX,
    };
    let axis_ly = GamepadAxis {
        gamepad,
        axis_type: GamepadAxisType::LeftStickY,
    };
    get_axis_direction(axes, axis_lx, axis_ly)
}

fn get_axis_direction(axes: Res<Axis<GamepadAxis>>, axis_rx: GamepadAxis, axis_ry: GamepadAxis) -> Vec2 {
    let mut direction = Vec2::ZERO;
    if let Some(x) = axes.get(axis_rx) {
        direction.x = x;
    }
    if let Some(y) = axes.get(axis_ry) {
        direction.y = y;
    }
    direction
}

#[cfg(test)]
mod gamepad_test {
    use bevy::input::InputPlugin;
    use bevy::prelude::{App, Gamepad, GamepadEvent, GamepadEventType};
    use crate::gamepad::{GamepadPlugin, MyGamepad};
    use crate::test_utils::connect_test_gamepad;

    #[test]
    fn primary_game_pad_is_assigned_on_connect() {
        let mut app = setup();
        connect_test_gamepad(&mut app);
        app.update();
        let gamepad = app.world.get_resource::<MyGamepad>().unwrap();
        assert_eq!(gamepad.0.id, 1);
    }

    #[test]
    fn primary_game_pad_is_unassigned_on_disconnect() {
        let mut app = setup();
        connect_test_gamepad(&mut app);
        app.update();
        app.world.send_event(
            GamepadEvent::new(
                Gamepad { id: 1 },
                GamepadEventType::Disconnected));
        app.update();
        assert!(app.world.get_resource::<MyGamepad>().is_none());
    }

    fn setup() -> App {
        let mut app = App::new();
        app
            .add_plugin(InputPlugin)
            .add_plugin(GamepadPlugin);
        app
    }
}

