use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use crate::gamepad;
use crate::gamepad::MyGamepad;
use crate::player::Player;

pub struct MyInputPlugin;

impl Plugin for MyInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_input);
    }
}

fn movement_input(
    mut player_q: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    axes: Res<Axis<GamepadAxis>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let velocity_res = player_q.get_single_mut();
    if velocity_res.is_err() { return; }
    let mut velocity = velocity_res.unwrap();
    let mut direction = Vec2::default();
    let max_speed = 70.0;
    handle_gamepad_input(axes, my_gamepad, &mut direction);
    handle_keyboard_input(keyboard_input, &mut direction);
    velocity.linvel = direction.normalize_or_zero() * max_speed;
}

fn handle_keyboard_input(keyboard_input: Res<Input<KeyCode>>, direction: &mut Vec2) {
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.0;
    }
}

fn handle_gamepad_input(axes: Res<Axis<GamepadAxis>>, my_gamepad: Option<Res<MyGamepad>>, direction: &mut Vec2) {
    if my_gamepad.is_none() { return; }
    let gamepad = my_gamepad.unwrap().0;
    let new_direction = gamepad::get_left_axis_direction(axes, gamepad);
    *direction = new_direction;
}


#[cfg(test)]
mod input_tests {
    use super::*;
    use bevy_rapier2d::prelude::*;
    use bevy::time::TimePlugin;
    use bevy::input::{ButtonState, InputPlugin};
    use bevy::input::keyboard::KeyboardInput;
    use bevy::prelude::GamepadAxisType::{LeftStickX, LeftStickY};
    use crate::gamepad::GamepadPlugin;
    use crate::test_utils;
    use crate::player::*;

    #[test]
    fn moves_player_up() {
        let (mut app, player_entity) = setup();
        press_key(&mut app, KeyCode::W);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.y > 0.);
    }

    #[test]
    fn moves_player_down() {
        let (mut app, player_entity) = setup();
        press_key(&mut app, KeyCode::S);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.y < 0.);
    }

    #[test]
    fn moves_player_left() {
        let (mut app, player_entity) = setup();
        press_key(&mut app, KeyCode::A);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.x < 0.);
    }

    #[test]
    fn moves_player_right() {
        let (mut app, player_entity) = setup();
        press_key(&mut app, KeyCode::D);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.x > 0.);
    }

    #[test]
    fn gamepad_moves_player_up() {
        let (mut app, player_entity) = setup();
        test_utils::connect_test_gamepad(&mut app);
        move_gamepad_left_axis(&mut app, 0., 1.);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.y > 0.);
    }

    #[test]
    fn gamepad_moves_player_down() {
        let (mut app, player_entity) = setup();
        test_utils::connect_test_gamepad(&mut app);
        move_gamepad_left_axis(&mut app, 0., -1.);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.y < 0.);
    }

    #[test]
    fn gamepad_moves_player_left() {
        let (mut app, player_entity) = setup();
        test_utils::connect_test_gamepad(&mut app);
        move_gamepad_left_axis(&mut app, -1., 0.);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.x < 0.);
    }

    #[test]
    fn gamepad_moves_player_right() {
        let (mut app, player_entity) = setup();
        test_utils::connect_test_gamepad(&mut app);
        move_gamepad_left_axis(&mut app, 1., 0.);
        test_utils::update(&mut app, 2);
        let new_transform = get_player_transform(&app, player_entity);
        assert!(new_transform.translation.x > 0.);
    }

    #[test]
    fn gamepad_moves_player_at_an_angle() {
        let (mut app, player_entity) = setup();
        test_utils::connect_test_gamepad(&mut app);
        move_gamepad_left_axis(&mut app, 0.7, 0.7);
        test_utils::update(&mut app, 3);
        let velocity = app.world.get::<Velocity>(player_entity).unwrap();
        assert_eq!(velocity.linvel.floor(),
                   (Vec2::new(0.7, 0.7).normalize() * 70.).floor());
    }

    fn setup() -> (App, Entity) {
        let mut app = App::new();
        app
            .add_plugin(TimePlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(GamepadPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(MyInputPlugin);
        let player_entity = app.world.spawn(PlayerBundle::default()).id();
        (app, player_entity)
    }

    fn get_player_transform(app: &App, player_entity: Entity) -> &Transform {
        app.world.get::<Transform>(player_entity).unwrap()
    }

    fn press_key(app: &mut App, key: KeyCode) {
        app.world.send_event(KeyboardInput {
            scan_code: 0,
            key_code: Option::from(key),
            state: ButtonState::Pressed,
        });
    }

    fn move_gamepad_left_axis(app: &mut App, x_pos: f32, y_pos: f32) {
        let mut gamepad_axis = Axis::<GamepadAxis>::default();
        let gamepad_x_axis = GamepadAxis {
            gamepad: Gamepad { id: 1 },
            axis_type: LeftStickX,
        };
        let gamepad_y_axis = GamepadAxis {
            gamepad: Gamepad { id: 1 },
            axis_type: LeftStickY,
        };
        gamepad_axis.set(gamepad_x_axis, x_pos);
        gamepad_axis.set(gamepad_y_axis, y_pos);
        app.insert_resource(gamepad_axis);
    }
}
