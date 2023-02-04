use bevy::prelude::{App, Commands, Component, Entity, EventReader, Plugin, Query};
use bevy_rapier2d::pipeline::CollisionEvent;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(add_interaction_text_on_collision);
    }
}

pub trait Interaction {
    fn interact(&mut self);
}

#[derive(Component, Default)]
pub struct Interactor;

#[derive(Component)]
pub struct InteractionText;

pub fn add_interaction_text_on_collision(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    interactor_q: Query<&Interactor>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                add_interaction_text_to_interactor(
                    &mut commands,
                    &interactor_q,
                    vec![e1, e2],
                );
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                remove_interaction_text_from_interactor(
                    &mut commands,
                    &interactor_q,
                    vec![e1, e2],
                );
            }
        }
    }
}

fn remove_interaction_text_from_interactor(
    commands: &mut Commands,
    interactor_q: &Query<&Interactor>,
    entities: Vec<&Entity>,
) {
    for e in entities.into_iter() {
        if interactor_q.get(*e).is_ok() {
            commands.entity(*e).remove::<InteractionText>();
        }
    }
}

fn add_interaction_text_to_interactor(
    commands: &mut Commands,
    interactor_q: &Query<&Interactor>,
    entities: Vec<&Entity>,
) {
    for e in entities.into_iter() {
        if interactor_q.get(*e).is_ok() {
            commands.entity(*e).insert(InteractionText);
        }
    };
}

#[cfg(test)]
mod interaction_tests {
    use bevy::prelude::{App, Transform, Vec3, With};
    use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
    use crate::door::{Door, DoorPlugin};
    use crate::interaction::{InteractionPlugin, InteractionText};
    use crate::level::LevelPlugin;
    use crate::player::{Player, PlayerPlugin};
    use crate::test_utils;
    use crate::test_utils::LoadTestPlugins;

    #[test]
    fn it_adds_interaction_text_when_player_is_near_door() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        move_player_to_door(&mut app);
        test_utils::update(&mut app, 2);
        let interaction_text_count = app.world
            .query::<&InteractionText>()
            .iter(&app.world).len();
        assert_eq!(interaction_text_count, 1);
    }

    #[test]
    fn it_removes_interaction_text_when_player_moves_away_from_door() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let player_original_translation = app.world
            .query_filtered::<&Transform, With<Player>>()
            .single(&app.world)
            .translation;
        move_player_to_door(&mut app);
        test_utils::update(&mut app, 2);
        move_player(&mut app, player_original_translation);
        test_utils::update(&mut app, 2);
        let interaction_text_count = app.world
            .query::<&InteractionText>()
            .iter(&app.world).len();
        assert_eq!(interaction_text_count, 0);
    }

    fn move_player_to_door(app: &mut App) {
        let door_transform = app.world
            .query_filtered::<&Transform, With<Door>>()
            .single(&app.world);
        let door_translation = door_transform.translation;
        move_player(app, door_translation);
    }

    fn move_player(app: &mut App, translation: Vec3) {
        let mut player_transform = app.world
            .query_filtered::<&mut Transform, With<Player>>()
            .single_mut(&mut app.world);
        player_transform.translation = translation;
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugin(DoorPlugin)
            .add_plugin(InteractionPlugin);
        app
    }
}

