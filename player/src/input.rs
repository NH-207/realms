use bevy::prelude::*;
use leafwing_input_manager::{prelude::*, Actionlike};

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ControlScheme::default())
            .add_plugin(InputManagerPlugin::<CharacterAction>::default())
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
pub enum CharacterAction {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
    Sprint,
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
    Rotate,
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
    pub character_input: InputMap<CharacterAction>,
    pub camera_input: InputMap<CameraAction>,
}

impl Default for ControlScheme {
    fn default() -> Self {
        let mut character_input = InputMap::default();
        character_input
            .insert(KeyCode::W, CharacterAction::Forward)
            .insert(KeyCode::S, CharacterAction::Backward)
            .insert(KeyCode::A, CharacterAction::Left)
            .insert(KeyCode::D, CharacterAction::Right)
            .insert(KeyCode::Space, CharacterAction::Jump)
            .insert(KeyCode::LControl, CharacterAction::Crouch)
            .insert(KeyCode::LShift, CharacterAction::Sprint)
            .insert(MouseButton::Left, CharacterAction::Primary)
            .insert(MouseButton::Right, CharacterAction::Secondary)
            .insert(KeyCode::Key1, CharacterAction::Hotbar1)
            .insert(KeyCode::Key2, CharacterAction::Hotbar2)
            .insert(KeyCode::Key3, CharacterAction::Hotbar3)
            .insert(KeyCode::Key4, CharacterAction::Hotbar4)
            .insert(KeyCode::Key5, CharacterAction::Hotbar5)
            .insert(KeyCode::Key6, CharacterAction::Hotbar6)
            .insert(KeyCode::Key7, CharacterAction::Hotbar7)
            .insert(KeyCode::Key8, CharacterAction::Hotbar8)
            .insert(KeyCode::Key9, CharacterAction::Hotbar9)
            .insert(KeyCode::Key0, CharacterAction::Hotbar0)
            .insert(DualAxis::mouse_motion(), CharacterAction::Rotate);

        let mut camera_input = InputMap::default();
        camera_input
            .insert(KeyCode::C, CameraAction::SwitchPerspective)
            .insert(KeyCode::F, CameraAction::Freecam)
            .insert(KeyCode::H, CameraAction::SwitchShoulder);

        Self {
            character_input,
            camera_input,
        }
    }
}
