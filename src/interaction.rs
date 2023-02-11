use bevy::prelude::{App, Commands, Component, Entity, EventReader, Plugin, Query, Text, With};
use bevy_rapier2d::pipeline::CollisionEvent;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_interactive_collisions);
    }
}

pub trait Interaction {
    fn interact(&mut self);
}

#[derive(Component, Default)]
pub struct Interactor;

#[derive(Component, Default)]
pub struct InteractiveText;

#[derive(Component)]
pub struct Interactive;

pub fn handle_interactive_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    interactor_q: Query<&Interactor>,
    mut interactive_text_q: Query<&mut Text, With<InteractiveText>>
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::Started(e1, e2, _) => {
                add_interactive_to_object(
                    &mut commands,
                    &interactor_q,
                    vec![e1, e2],
                );
            }
            CollisionEvent::Stopped(e1, e2, _) => {
                remove_interactive_from_object(
                    &mut commands,
                    &interactor_q,
                    vec![e1, e2],
                );
                remove_interactive_text(&mut interactive_text_q);
            }
        }
    }
}

fn remove_interactive_text(interactive_text_q: &mut Query<&mut Text, With<InteractiveText>>) {
    for mut text in interactive_text_q.iter_mut() {
        text.sections[0].value = "".to_string();
    }
}

fn remove_interactive_from_object(
    commands: &mut Commands,
    interactor_q: &Query<&Interactor>,
    entities: Vec<&Entity>,
) {
    for e in entities.into_iter() {
        if interactor_q.get(*e).is_err() {
            commands.entity(*e).remove::<Interactive>();
        }
    }
}

fn add_interactive_to_object(
    commands: &mut Commands,
    interactor_q: &Query<&Interactor>,
    entities: Vec<&Entity>,
) {
    for e in entities.into_iter() {
        if interactor_q.get(*e).is_err() {
            commands.entity(*e).insert(Interactive);
        }
    };
}

#[cfg(test)]
mod interaction_tests {
    use bevy::prelude::{App, Entity, Text, TextBundle, Transform};
    use bevy::text::TextStyle;
    use bevy_rapier2d::dynamics::LockedAxes;
    use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, GravityScale, NoUserData, RapierPhysicsPlugin, RigidBody, Sensor};
    use bevy_rapier2d::rapier::prelude::CollisionEventFlags;
    use crate::interaction::{InteractionPlugin, Interactive, InteractiveText, Interactor};
    use crate::physics_bundle::{CharacterPhysicsBundle, ObjectPhysicsBundle};
    use crate::test_utils::LoadTestPlugins;

    #[test]
    fn it_adds_interactive_to_object_when_near_interactor() {
        let (mut app, player, object) = setup();
        app.update();
        app.world.send_event(CollisionEvent::Started(player, object, CollisionEventFlags::SENSOR));
        app.update();
        let interactive = app.world.get::<Interactive>(object);
        assert!(interactive.is_some());
    }

    #[test]
    fn it_removes_interactive_from_door_when_player_moves_away() {
        let (mut app, player, object) = setup();
        app.update();
        app.world.send_event(CollisionEvent::Started(player, object, CollisionEventFlags::SENSOR));
        app.update();
        app.world.send_event(CollisionEvent::Stopped(player, object, CollisionEventFlags::SENSOR));
        app.update();
        let interactive = app.world.get::<Interactive>(object);
        assert!(interactive.is_none());
    }

    #[test]
    fn it_removes_interactive_text_when_player_moves_away_from_door() {
        let (mut app, player, object) = setup();
        app.update();
        app.world.send_event(CollisionEvent::Stopped(player, object, CollisionEventFlags::SENSOR));
        app.update();
        let text = app.world.get::<Text>(player).unwrap();
        assert_eq!(text.sections[0].value, "");
    }

    fn setup() -> (App, Entity, Entity) {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
            .add_plugin(InteractionPlugin);
        let interactor = app.world.spawn((
            Interactor,
            CharacterPhysicsBundle {
                collider: Collider::capsule_y(4., 7.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                gravity: GravityScale(0.),
                ..Default::default()
            },
            TextBundle::from_section("text", TextStyle::default()),
            InteractiveText
        )).id();
        let object = app.world.spawn((
            ObjectPhysicsBundle {
                collider: Collider::cuboid(16., 16.),
                rigid_body: RigidBody::Fixed,
                sensor: Sensor,
                events: ActiveEvents::COLLISION_EVENTS,
            },
            Transform::default(),
        )).id();
        (app, interactor, object)
    }
}

