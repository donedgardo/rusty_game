use bevy::prelude::{Added, Changed, Component, Mut, Or, Query, Res, Time, Timer};
use bevy::sprite::TextureAtlasSprite;
use bevy::time::TimerMode;
use bevy::utils::petgraph::matrix_graph::Zero;
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::prelude::Velocity;

#[derive(Debug, PartialEq, Default)]
pub enum AnimationState {
    #[default]
    Idle,
    Moving,
}

#[derive(Component, Default)]
pub struct CharacterAnimation {
    pub state: AnimationState,
    pub timer: Timer,
    first: usize,
    last: usize,
}

impl From<&EntityInstance> for CharacterAnimation {
    fn from(entity_instance: &EntityInstance) -> CharacterAnimation {
        match entity_instance.identifier.as_ref() {
            "Player" => CharacterAnimation {
                state: AnimationState::Moving,
                timer: Timer::from_seconds(0.25, TimerMode::Repeating),
                first: 0,
                last: 7,
            },
            _ => CharacterAnimation::default(),
        }
    }
}

pub fn animation_system(
    time: Res<Time>,
    mut query: Query<(&mut CharacterAnimation,
                      &mut TextureAtlasSprite,
                      &Velocity),
        Or<(Added<Velocity>, Changed<Velocity>)>>,
) {
    for (
        mut animation,
        mut sprite,
        velocity
    ) in query.iter_mut() {
        update_animation_state(&mut animation, velocity);
        update_sprite_sheet(&time, &mut animation, &mut sprite, velocity);
    }
}

fn update_sprite_sheet(time: &Res<Time>, animation: &mut Mut<CharacterAnimation>, sprite: &mut Mut<TextureAtlasSprite>, velocity: &Velocity) {
    if time.delta().is_zero() || velocity.linvel.length().is_zero() { return; }
    animation.timer.tick(time.delta());
    if animation.timer.just_finished() {
        sprite.index = if sprite.index == animation.last {
            animation.first
        } else {
            sprite.index + 1
        };
    }
}

fn update_animation_state(animation: &mut Mut<CharacterAnimation>, velocity: &Velocity) {
    if velocity.linvel.length() == 0. {
        animation.state = AnimationState::Idle;
    } else {
        animation.state = AnimationState::Moving;
    }
}

#[cfg(test)]
mod test_player_animation {
    use std::time::Duration;
    use bevy::prelude::{App, Entity, Time, Vec2};
    use bevy::sprite::{SpriteSheetBundle, TextureAtlasSprite};
    use bevy::time::{Timer, TimerMode};
    use bevy::utils::default;
    use bevy_rapier2d::prelude::Velocity;
    use crate::animation::{AnimationState, animation_system, CharacterAnimation};
    use crate::physics_bundle::CharacterPhysicsBundle;

    #[test]
    fn it_has_idle_animation_by_default() {
        let (app, player) = setup();
        let animation = app.world.entity(player).get::<CharacterAnimation>().unwrap();
        assert_eq!(animation.state, AnimationState::Idle);
    }

    #[test]
    fn it_changes_to_moving_animation_when_moving() {
        let (mut app, player) = setup();
        add_velocity(&mut app, player, Vec2::new(10., 0.));
        app.update();
        let animation = app.world.entity(player).get::<CharacterAnimation>().unwrap();
        assert_eq!(animation.state, AnimationState::Moving);
    }

    #[test]
    fn updates_sprite_by_frame() {
        let (mut app, player) = setup();
        let sprite_sheet = app.world.entity(player).get::<TextureAtlasSprite>().unwrap();
        assert_eq!(sprite_sheet.index, 0);

        // test to check it doesnt update sprite if velocity is 0.
        add_velocity(&mut app, player, Vec2::new(0., 0.));
        update_time_resource(&mut app, Duration::from_secs(1));
        app.update();
        let sprite_sheet = app.world.entity(player).get::<TextureAtlasSprite>().unwrap();
        assert_eq!(sprite_sheet.index, 0);


        add_velocity(&mut app, player, Vec2::new(10., 0.));
        update_time_resource(&mut app, Duration::from_secs(1));
        app.update();

        let sprite_sheet = app.world.entity(player).get::<TextureAtlasSprite>().unwrap();
        assert_eq!(sprite_sheet.index, 1);
    }

    fn setup() -> (App, Entity) {
        let mut app = App::new();
        initialize_time(&mut app);
        let player = app.world
            .spawn((
                SpriteSheetBundle::default(),
                CharacterAnimation {
                    timer: Timer::from_seconds(1., TimerMode::Repeating),
                    state: Default::default(),
                    first: 0,
                    last: 7,
                },
            )).id();
        app.add_system(animation_system);
        app.update();
        (app, player)
    }

    fn initialize_time(app: &mut App) {
        app.init_resource::<Time>();
        let mut time = Time::default();
        time.update();
        app.world.insert_resource(time);
    }

    fn add_velocity(app: &mut App, player: Entity, linvel: Vec2) {
        app.world
            .entity_mut(player)
            .insert(Velocity {
                linvel,
                ..default()
            });
    }

    fn update_time_resource(app: &mut App, duration: Duration) {
        let mut time = app.world.resource_mut::<Time>();
        let last_update = time.last_update().unwrap();
        time.update_with_instant(last_update + duration);
    }
}
