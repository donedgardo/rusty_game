use bevy::prelude::Bundle;
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, Sensor};

#[derive(Bundle, Default)]
pub struct CharacterPhysicsBundle {
    pub rigid_body: RigidBody,
    pub gravity: GravityScale,
    pub velocity: Velocity,
    pub collider: Collider,
    pub rotation_constraints: LockedAxes,
}

impl From<&EntityInstance> for CharacterPhysicsBundle {
    fn from(entity_instance: &EntityInstance) -> CharacterPhysicsBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        match entity_instance.identifier.as_ref() {
            "Player" => CharacterPhysicsBundle {
                collider: Collider::capsule_y(4., 7.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity: GravityScale(0.),
                ..Default::default()
            },
            _ => CharacterPhysicsBundle::default(),
        }
    }
}

#[derive(Bundle, Default)]
pub struct ObjectPhysicsBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub sensor: Sensor,
    pub events: ActiveEvents,
}

impl From<&EntityInstance> for ObjectPhysicsBundle {
    fn from(entity_instance: &EntityInstance) -> ObjectPhysicsBundle {
        match entity_instance.identifier.as_ref() {
            "Door" => ObjectPhysicsBundle {
                collider: Collider::cuboid(16., 16.),
                rigid_body: RigidBody::Fixed,
                sensor: Sensor,
                events: ActiveEvents::COLLISION_EVENTS,
            },
            _ => ObjectPhysicsBundle::default(),
        }
    }
}
