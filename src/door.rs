use bevy::prelude::{App, Plugin, SpriteSheetBundle, Bundle, Component, TextureAtlasSprite, Changed, Query, Commands, Entity, BuildChildren, Mut, Children};
use bevy_ecs_ldtk::prelude::{EntityInstance, LdtkEntity, RegisterLdtkObjects};
use bevy_ecs_ldtk::ldtk::FieldValue;
use bevy_rapier2d::prelude::Collider;
use crate::interaction::Interaction;
use crate::physics_bundle::{ObjectPhysicsBundle};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<DoorBundle>("Door")
            .add_system(update_changed_doors);
    }
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct DoorBundle {
    #[from_entity_instance]
    pub door: Door,
    #[bundle]
    #[sprite_sheet_bundle("dungeon/doors.png", 32.0, 32.0, 2, 1, 0.0, 0.0, 1)]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    #[bundle]
    pub physics: ObjectPhysicsBundle,
}

#[derive(Component, Default)]
pub struct Door {
    is_open: bool,
}

impl Door {
    pub fn is_open(&self) -> bool {
        return self.is_open;
    }
}

impl Interaction for Door {
    fn interact(&mut self) {
        self.is_open = !self.is_open;
    }
}

impl From<EntityInstance> for Door {
    fn from(value: EntityInstance) -> Self {
        let field_instances = &value.field_instances;
        let default_door = Door { is_open: false };
        match field_instances.iter()
            .find(|f| f.identifier == *"is_open") {
            None => { default_door }
            Some(field_instance) => {
                match field_instance.value {
                    FieldValue::Bool(is_open) => { Door { is_open } }
                    _ => { default_door }
                }
            }
        }
    }
}

fn update_changed_doors(
    mut commands: Commands,
    mut door_query: Query<(&Door, &mut TextureAtlasSprite, Entity, Option<&Children>), Changed<Door>>,
) {
    for (door, mut sprite, entity, children) in door_query.iter_mut() {
        match door.is_open() {
            true => {
                set_open_door_sprite(&mut sprite);
                remove_blocking_collider(&mut commands, entity, children);
            }
            false => {
                set_closed_door_sprite(&mut sprite);
                add_blocking_collider(&mut commands, entity);
            }
        }
    }
}

fn remove_blocking_collider(commands: &mut Commands, entity: Entity, children_r: Option<&Children>) {
    if let Some(children) = children_r {
        for &child in children.iter() {
            commands.entity(entity).remove_children(&[child]);
            commands.entity(child).despawn();
        }
    }
}

fn set_closed_door_sprite(sprite: &mut Mut<TextureAtlasSprite>) {
    sprite.index = 0;
}

fn set_open_door_sprite(sprite: &mut Mut<TextureAtlasSprite>) {
    sprite.index = 1;
}

fn add_blocking_collider(commands: &mut Commands, entity: Entity) {
    commands.entity(entity).with_children(|parent| {
        parent.spawn(Collider::cuboid(3.0, 1.0)); });
}


#[cfg(test)]
mod doors_test {
    use bevy::ecs::query::QueryEntityError;
    use bevy::prelude::{App, Children, Entity, TextureAtlasSprite, With, Without};
    use bevy_rapier2d::prelude::*;
    use crate::door::{Door, DoorPlugin};
    use crate::interaction::Interaction;
    use crate::level::LevelPlugin;
    use crate::test_utils;
    use crate::test_utils::LoadTestPlugins;

    #[test]
    fn door_has_interaction_to_open() {
        let mut door = Door { is_open: false };
        door.interact();
        assert!(door.is_open())
    }

    #[test]
    fn it_spawns_entity_with_door() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        assert!(app.world.query::<&Door>().iter(&app.world).len() > 0);
    }

    #[test]
    fn door_is_open_by_default() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let door = app.world.query::<&Door>().single(&app.world);
        assert!(door.is_open());
    }

    #[test]
    fn it_has_a_sprite_sheet() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let door_sprite_sheet_count = app.world
            .query_filtered::<&TextureAtlasSprite, With<Door>>()
            .iter(&app.world)
            .len();
        assert_eq!(door_sprite_sheet_count, 1);
    }

    #[test]
    fn it_has_open_sprite_by_default() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let sprite = app.world
            .query_filtered::<&TextureAtlasSprite, With<Door>>()
            .single(&app.world);
        assert_eq!(sprite.index, 1);
    }

    #[test]
    fn it_has_a_sensor_collider_by_default() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let door_with_collider_sensor_count = app.world
            .query::<(&Door, &Sensor, &Collider, &RigidBody)>()
            .iter(&app.world)
            .len();
        assert_eq!(door_with_collider_sensor_count, 1);
    }

    #[test]
    fn it_has_open_sprite_after_interacting() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        interact_with_door(&mut app);
        app.update();
        let sprite = app.world
            .query_filtered::<&TextureAtlasSprite, With<Door>>()
            .single(&app.world);
        assert_eq!(sprite.index, 0);
    }

    #[test]
    fn it_doesnt_have_blocking_collider_when_open() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let blocking_collider = get_blocking_collider(&mut app);
        assert!(blocking_collider.is_err());
    }

    #[test]
    fn it_has_blocking_collider_when_closed() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        interact_with_door(&mut app);
        app.update();
        let blocking_collider = get_blocking_collider(&mut app);
        assert!(blocking_collider.is_ok());
    }

    #[test]
    fn it_doesnt_have_blocking_collider_when_reopened() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        //close
        interact_with_door(&mut app);
        app.update();
        //open
        interact_with_door(&mut app);
        app.update();
        let blocking_collider = get_blocking_collider(&mut app);
        assert!(blocking_collider.is_err());
    }

    fn interact_with_door(app: &mut App) {
        let mut door = app.world
            .query::<&mut Door>()
            .single_mut(&mut app.world);
        door.interact();
    }

    fn get_blocking_collider(app: &mut App) -> Result<&Collider, QueryEntityError> {
        let mut blocking_collider = Err(QueryEntityError::NoSuchEntity(Entity::from_raw(0)));
        let mut children_q = app.world
            .query_filtered::<&Children, With<Door>>();
        let mut collider_q = app.world
            .query_filtered::<&Collider, Without<Sensor>>();
        for children in children_q.iter(&app.world) {
            for &child in children.iter() {
                blocking_collider = collider_q
                    .get(&app.world, child);
            }
        }
        blocking_collider
    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(DoorPlugin);
        app
    }
}
