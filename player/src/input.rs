use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, Actionlike};

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlScheme::default())
            .add_plugin(InputManagerPlugin::<PlayerMoveAction>::default())
            .add_plugin(InputManagerPlugin::<PlayerFightAction>::default())
            .add_plugin(InputManagerPlugin::<CameraAction>::default());
    }
}

#[derive(Bundle, Default)]
pub struct Camera3dWithInputBundle {
    #[bundle]
    pub camera_3d: Camera3dBundle,
    #[bundle]
    pub input: InputManagerBundle<CameraAction>,
}

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerMoveAction {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
    Sprint,
    Yaw,
}

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerFightAction {
    Primary,
    Secondary,
    Hotbar1,
    Hotbar2,
    Hotbar3,
    Hotbar4,
    Hotbar5,
    Hotbar6,
    Hotbar7,
    Hotbar8,
    Hotbar9,
    Hotbar0,
    Pitch,
}

#[derive(Actionlike, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraAction {
    SwitchPerspective,
    // These two do nothing in First person view
    Freecam,
    SwitchShoulder,
}

#[derive(Resource, Debug)]
pub struct ControlScheme {
    pub move_input: InputMap<PlayerMoveAction>,
    pub fight_input: InputMap<PlayerFightAction>,
    pub camera_input: InputMap<CameraAction>,
}

impl Default for ControlScheme {
    fn default() -> Self {
        let mut move_input = InputMap::default();
        move_input
            .insert(KeyCode::W, PlayerMoveAction::Forward)
            .insert(KeyCode::S, PlayerMoveAction::Backward)
            .insert(KeyCode::A, PlayerMoveAction::Left)
            .insert(KeyCode::D, PlayerMoveAction::Right)
            .insert(KeyCode::Space, PlayerMoveAction::Jump)
            .insert(KeyCode::LControl, PlayerMoveAction::Crouch)
            .insert(KeyCode::LShift, PlayerMoveAction::Sprint)
            .insert(SingleAxis::mouse_motion_x(), PlayerMoveAction::Yaw);

        let mut fight_input = InputMap::default();
        fight_input
            .insert(MouseButton::Left, PlayerFightAction::Primary)
            .insert(MouseButton::Right, PlayerFightAction::Secondary)
            .insert(KeyCode::Key1, PlayerFightAction::Hotbar1)
            .insert(KeyCode::Key2, PlayerFightAction::Hotbar2)
            .insert(KeyCode::Key3, PlayerFightAction::Hotbar3)
            .insert(KeyCode::Key4, PlayerFightAction::Hotbar4)
            .insert(KeyCode::Key5, PlayerFightAction::Hotbar5)
            .insert(KeyCode::Key6, PlayerFightAction::Hotbar6)
            .insert(KeyCode::Key7, PlayerFightAction::Hotbar7)
            .insert(KeyCode::Key8, PlayerFightAction::Hotbar8)
            .insert(KeyCode::Key9, PlayerFightAction::Hotbar9)
            .insert(KeyCode::Key0, PlayerFightAction::Hotbar0)
            .insert(SingleAxis::mouse_motion_y(), PlayerFightAction::Pitch);

        let mut camera_input = InputMap::default();
        camera_input
            .insert(KeyCode::C, CameraAction::SwitchPerspective)
            .insert(KeyCode::F, CameraAction::Freecam)
            .insert(KeyCode::H, CameraAction::SwitchShoulder);

        Self {
            move_input,
            fight_input,
            camera_input,
        }
    }
}
