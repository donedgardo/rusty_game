use bevy::prelude::{App, Plugin, SpriteSheetBundle, Bundle, Component, TextureAtlasSprite, Changed, Query};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_ecs_ldtk::ldtk::FieldValue;
use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;
use crate::interaction::Interaction;
use crate::physics_bundle::{ObjectPhysicsBundle};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_ldtk_entity::<DoorBundle>("Door")
            .add_system(update_door_sprites);
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

fn update_door_sprites(
   mut door_query: Query<(&Door, &mut TextureAtlasSprite), Changed<Door>>
) {
    for (door, mut sprite) in door_query.iter_mut() {
        match door.is_open() {
            true => { sprite.index = 1 }
            false => { sprite.index = 0 }
        }
    }
}


#[cfg(test)]
mod doors_test {
    use bevy::prelude::{App, TextureAtlasSprite, With};
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
    fn it_has_open_sprite_after_interacting() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let mut door = app.world
            .query::<&mut Door>()
            .single_mut(&mut app.world);
        door.interact();
        app.update();
        let sprite = app.world
            .query_filtered::<&TextureAtlasSprite, With<Door>>()
            .single(&app.world);
        assert_eq!(sprite.index, 0);
    }

    #[test]
    fn it_has_a_collider_with_a_sensor() {
        let mut app = setup();
        test_utils::update(&mut app, 3);
        let door_with_collider_sensor_count = app.world
            .query::<(&Door, &Sensor, &Collider, &RigidBody)>()
            .iter(&app.world)
            .len();
        assert_eq!(door_with_collider_sensor_count, 1);

    }

    fn setup() -> App {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin)
            .add_plugin(DoorPlugin);
        app
    }
}
