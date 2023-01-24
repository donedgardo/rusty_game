use bevy::prelude::Bundle;
use bevy_rapier2d::dynamics::{GravityScale, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::Collider;
use bevy_ecs_ldtk::EntityInstance;

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub rigid_body: RigidBody,
    pub gravity: GravityScale,
    pub velocity: Velocity,
    pub collider: Collider,
    pub rotation_constraints: LockedAxes,
}

impl From<EntityInstance> for PhysicsBundle {
    fn from(entity_instance: EntityInstance) -> PhysicsBundle {
        let rotation_constraints = LockedAxes::ROTATION_LOCKED;
        match entity_instance.identifier.as_ref() {
            "Player" => PhysicsBundle {
                collider: Collider::capsule_y(4., 7.),
                rigid_body: RigidBody::Dynamic,
                rotation_constraints,
                gravity: GravityScale(0.),
                ..Default::default()
            },
            _ => PhysicsBundle::default(),
        }
    }
}
