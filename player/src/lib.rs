use bevy::prelude::*;
use bevy_rapier3d::prelude::KinematicCharacterController;
use input::CharacterAction;
use leafwing_input_manager::{axislike::DualAxisData, prelude::ActionState};

const GRAVITY: Vec3 = Vec3::new(0.0, -0.3, 0.0);
const SPRINT_SPEED: f32 = 6.0;
const SPEED: f32 = 3.0;

pub mod camera;
pub mod input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(input::InputPlugin)
            .add_plugin(camera::CameraPlugin)
            .add_system(player_movement);
    }
}

fn player_movement(
    windows: Res<Windows>,
    mut query: Query<(
        &ActionState<CharacterAction>,
        &mut KinematicCharacterController,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    for (action_state, mut player, mut transform) in query.iter_mut() {
        let forward = {
            let mut forward = transform.forward();
            forward.y = 0.0;
            forward.normalize()
        };

        let mut left = transform.left();
        left.y = 0.0;
        left = left.normalize();

        let speed = if action_state.pressed(CharacterAction::Sprint) {
            SPRINT_SPEED
        } else {
            SPEED
        };

        let mut translation = GRAVITY;
        if action_state.pressed(CharacterAction::Forward) {
            translation += forward * time.delta_seconds() * speed;
        }
        if action_state.pressed(CharacterAction::Backward) {
            translation -= forward * time.delta_seconds() * speed;
        }
        if action_state.pressed(CharacterAction::Left) {
            translation += left * time.delta_seconds() * speed;
        }
        if action_state.pressed(CharacterAction::Right) {
            translation -= left * time.delta_seconds() * speed;
        }

        player.translation = Some(translation);

        let rotation_move: Vec2 = action_state
            .axis_pair(CharacterAction::Rotate)
            .unwrap_or(DualAxisData::from_xy(Vec2::ZERO))
            .into();

        if rotation_move.length_squared() > 0.0 {
            let window = get_primary_window_size(&windows);
            let delta_x = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
