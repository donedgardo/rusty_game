use bevy::prelude::{Bundle, Component};
use bevy_ecs_ldtk::{EntityInstance, LdtkEntity};
use bevy_ecs_ldtk::ldtk::{FieldInstance, FieldValue};

#[derive(Component, Default)]
pub struct Door {
    is_open: bool
}

impl Door {
    pub fn is_open(&self) -> bool {
        return self.is_open;
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
                   FieldValue::Bool(is_open) => { Door {is_open} }
                   _ => { default_door }
               }
            }
        }
    }
}

#[derive(Bundle, LdtkEntity, Default)]
pub struct DoorBundle {
    #[from_entity_instance]
    pub door: Door
}

#[cfg(test)]
mod doors_test {
    use bevy::prelude::App;
    use bevy_ecs_ldtk::prelude::RegisterLdtkObjects;
    use crate::door::{Door, DoorBundle};
    use crate::level::LevelPlugin;
    use crate::test_utils;
    use crate::test_utils::LoadTestPlugins;

    #[test]
    fn it_spawns_entity_with_door() {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin);
        app.register_ldtk_entity::<DoorBundle>("Door");
        test_utils::update(&mut app,3);
        assert!(app.world.query::<&Door>().iter(&app.world).len() > 0);
    }

    #[test]
    fn door_is_closed_by_default() {
        let mut app = App::new();
        app.add_plugins(LoadTestPlugins)
            .add_plugin(LevelPlugin);
        app.register_ldtk_entity::<DoorBundle>("Door");
        test_utils::update(&mut app,3);
        let door = app.world.query::<&Door>().single(&app.world);
        assert!(!door.is_open());
    }
}
