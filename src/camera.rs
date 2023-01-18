use bevy::prelude::{Camera2dBundle, Commands};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[cfg(test)]
mod camera_test {
    use bevy::prelude::*;
    use crate::camera::spawn_camera;

    #[test]
    fn spawns_camera() {
        let mut app = App::new();
        app.add_startup_system(spawn_camera);
        app.update();
        assert_eq!(app.world.query::<&Camera>().iter(&app.world).len(), 1)
    }
}
