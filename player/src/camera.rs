use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::input::CameraAction;

const RIGHT_SHOULDER: Vec3 = Vec3::new(1.0, 1.0, 2.0);
const LEFT_SHOULDER: Vec3 = Vec3::new(-1.0, 1.0, 2.0);
const FIRST_PERSON: Vec3 = Vec3::new(0.0, 1.0, 0.0);

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enter_first_person)
            .add_system(enter_third_person)
            .add_system(swap_shoulder);
    }
}

enum Shoulder {
    Left,
    Right,
}

#[derive(Component)]
struct FirstPerson;

/// Tags an entity as capable of orbiting.
#[derive(Component)]
struct ThirdPerson {
    pub shoulder: Shoulder,
}

impl Default for ThirdPerson {
    fn default() -> Self {
        ThirdPerson {
            shoulder: Shoulder::Right,
        }
    }
}

fn enter_first_person(
    mut commands: Commands,
    mut query: Query<(Entity, &ActionState<CameraAction>, &mut Transform), Without<FirstPerson>>,
) {
    for (entity, action_state, mut transform) in query.iter_mut() {
        if action_state.just_pressed(CameraAction::SwitchPerspective) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<ThirdPerson>();

                entity_commands.insert(FirstPerson);

                transform.translation = FIRST_PERSON;
            }
        }
    }
}

fn enter_third_person(
    mut commands: Commands,
    mut query: Query<(Entity, &ActionState<CameraAction>, &mut Transform), Without<ThirdPerson>>,
) {
    for (entity, action_state, mut transform) in query.iter_mut() {
        if action_state.just_pressed(CameraAction::SwitchPerspective) {
            if let Some(mut entity_commands) = commands.get_entity(entity) {
                entity_commands.remove::<FirstPerson>();

                entity_commands.insert(ThirdPerson {
                    shoulder: Shoulder::Right,
                });

                transform.translation = RIGHT_SHOULDER;
            }
        }
    }
}

fn swap_shoulder(mut query: Query<(&ActionState<CameraAction>, &mut ThirdPerson, &mut Transform)>) {
    for (action_state, mut third_person, mut transform) in query.iter_mut() {
        let old_location = &third_person.shoulder;

        if action_state.just_pressed(CameraAction::SwitchShoulder) {
            match old_location {
                Shoulder::Left => {
                    third_person.shoulder = Shoulder::Right;
                    transform.translation = RIGHT_SHOULDER;
                }
                Shoulder::Right => {
                    third_person.shoulder = Shoulder::Left;
                    transform.translation = LEFT_SHOULDER;
                }
            }
        }
    }
}
