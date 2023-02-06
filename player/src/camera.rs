use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::input::CameraAction;

const RIGHT_SHOULDER: Vec3 = Vec3::new(1.5, 0.3, 5.0);
const LEFT_SHOULDER: Vec3 = Vec3::new(-1.5, 0.3, 5.0);
const FIRST_PERSON: Vec3 = Vec3::ZERO;

pub(crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cycle_camera);
    }
}

#[derive(Default)]
enum Location {
    #[default]
    FirstPerson,
    LeftShoulder,
    RightShoulder,
}

fn cycle_camera(
    mut query: Query<(&ActionState<CameraAction>, &mut Transform)>,
    mut location: Local<Location>,
) {
    let (action_state, mut transform) = query.single_mut();
    if action_state.just_pressed(CameraAction::SwitchPerspective) {
        match *location {
            Location::FirstPerson => {
                *location = Location::RightShoulder;
                transform.translation = RIGHT_SHOULDER;
            }
            _ => {
                *location = Location::FirstPerson;
                transform.translation = FIRST_PERSON;
            }
        }
    }

    if action_state.just_pressed(CameraAction::SwitchShoulder) {
        match *location {
            Location::LeftShoulder => {
                *location = Location::RightShoulder;
                transform.translation = RIGHT_SHOULDER;
            }
            Location::RightShoulder => {
                *location = Location::LeftShoulder;
                transform.translation = LEFT_SHOULDER;
            }
            _ => {}
        }
    }
}
