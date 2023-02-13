use bevy::prelude::*;
use bevy_rapier3d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};
use input::{PlayerFightAction, PlayerMoveAction};
use leafwing_input_manager::prelude::ActionState;

// Everything is in SI units
const GRAVITY: f32 = -9.82;
const SPRINT_SPEED: f32 = 6.0;
const SPEED: f32 = 3.0;
const JUMP_FORCE: f32 = 5.0;
const TERMINAL_VELOCITY: f32 = -15.0;

pub mod camera;
pub mod input;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(input::InputPlugin)
            .add_plugin(camera::CameraPlugin)
            .add_system(player_movement)
            .add_system(player_jump)
            .add_system(player_aim);
    }
}

/// Marker component for the root entity of the player
#[derive(Component)]
pub struct PlayerBase;

/// Marker component for the focal point of the player aiming
#[derive(Component)]
pub struct PlayerAim;

fn player_jump(
    mut query: Query<(
        &ActionState<PlayerMoveAction>,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    time: Res<Time>,
    mut vertical_velocity: Local<f32>,
) {
    let (action_state, player, player_output) = match query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let delta_t = time.delta_seconds();

    if let Some(player_output) = player_output {
        if player_output.grounded {
            if action_state.pressed(PlayerMoveAction::Jump) {
                *vertical_velocity = JUMP_FORCE;
            } else {
                *vertical_velocity = 0.0;
            }
        }
    }

    let translation = Vec3::Y * *vertical_velocity * delta_t;
    *vertical_velocity = (*vertical_velocity + GRAVITY * delta_t).max(TERMINAL_VELOCITY);

    add_translation(player, translation);
}

fn player_movement(
    mut query: Query<
        (
            &ActionState<PlayerMoveAction>,
            &mut KinematicCharacterController,
            &Transform,
        ),
        With<PlayerBase>,
    >,
    time: Res<Time>,
) {
    let (action_state, player, transform) = match query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let forward = {
        let mut forward = transform.forward();
        forward.y = 0.0;
        forward.normalize()
    };

    let left = {
        let mut left = transform.left();
        left.y = 0.0;
        left.normalize()
    };

    let delta_t = time.delta_seconds();

    let speed = if action_state.pressed(PlayerMoveAction::Sprint) {
        SPRINT_SPEED
    } else {
        SPEED
    };

    let mut translation = Vec3::ZERO;
    if action_state.pressed(PlayerMoveAction::Forward) {
        translation += forward;
    }
    if action_state.pressed(PlayerMoveAction::Backward) {
        translation -= forward;
    }
    if action_state.pressed(PlayerMoveAction::Left) {
        translation += left;
    }
    if action_state.pressed(PlayerMoveAction::Right) {
        translation -= left;
    }
    translation = translation.normalize_or_zero() * delta_t * speed;

    add_translation(player, translation);
}

fn player_aim(
    windows: Res<Windows>,
    mut pitch_query: Query<
        (&ActionState<PlayerFightAction>, &mut Transform),
        (With<PlayerAim>, Without<PlayerBase>),
    >,
    mut yaw_query: Query<
        (&ActionState<PlayerMoveAction>, &mut Transform),
        (With<PlayerBase>, Without<PlayerAim>),
    >,
) {
    let (pitch_action_state, mut pitch_transform) = match pitch_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let (yaw_action_state, mut yaw_transform) = match yaw_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => return,
    };

    let rotation = Vec2::new(
        yaw_action_state.value(PlayerMoveAction::Yaw),
        pitch_action_state.value(PlayerFightAction::Pitch),
    );

    if rotation.length_squared() > 0.0 {
        let window = get_primary_window_size(&windows);

        let delta_x = rotation.x / window.x * std::f32::consts::PI * 2.0;
        let yaw = Quat::from_rotation_y(-delta_x);
        yaw_transform.rotation = yaw * yaw_transform.rotation; // rotate around global y axis

        let delta_y = rotation.y / window.y * std::f32::consts::PI;
        let pitch = Quat::from_rotation_x(-delta_y);
        pitch_transform.rotation = pitch_transform.rotation * pitch; // rotate around local x axis
    }
}

#[inline(always)]
fn add_translation(mut input: Mut<KinematicCharacterController>, new_translation: Vec3) {
    let old_translation = match input.translation {
        Some(value) => value,
        None => Vec3::ZERO,
    };

    input.translation = Some(old_translation + new_translation);
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
