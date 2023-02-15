use bevy::prelude::{App, Commands, EventReader, Gamepad, GamepadEvent, GamepadEventType, Plugin, Res, Resource};

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
