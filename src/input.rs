use bevy::prelude::{Bundle, KeyCode, Query, Res, With};
use bevy_rapier2d::dynamics::{RigidBody, Velocity};
use bevy::input::Input;
use bevy::math::Vec2;
use crate::player::Player;

pub fn input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_q: Query<&mut Velocity, With<Player>>,
) {
    let velocity_res = player_q.get_single_mut();
    if velocity_res.is_err() { return; }
    let mut velocity = velocity_res.unwrap();
    let mut direction = Vec2::default();
    let speed = 100.0;
    if keyboard_input.pressed(KeyCode::W) {
        direction.y += speed;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.y -= speed;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x -= speed;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += speed;
    }
    velocity.linvel = direction;
}

#[derive(Bundle, Default)]
pub struct Controllable {
    rigid_body: RigidBody,
    velocity: Velocity,
}

#[cfg(test)]
mod input_tests {
    use super::*;
    use bevy::prelude::*;
    use bevy_rapier2d::prelude::*;
    use bevy::time::TimePlugin;
    use bevy::input::{ButtonState, InputPlugin};
    use bevy::input::keyboard::KeyboardInput;
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

    fn press_key(app: &mut App, key: KeyCode) {
        app.world.send_event(KeyboardInput {
            scan_code: 0,
            key_code: Option::from(key),
            state: ButtonState::Pressed,
        });
    }

    fn get_player_transform(app: &App, player_entity: Entity) -> &Transform {
        app.world.get::<Transform>(player_entity).unwrap()
    }

    fn setup() -> (App, Entity) {
        let mut app = App::new();
        app
            .add_plugin(TimePlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(InputPlugin)
            .add_system(input_system);
        let player_entity = app.world.spawn(PlayerBundle::default()).id();
        (app, player_entity)
    }
}
