use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use player::{
    input::{Camera3dWithInputBundle, CameraAction, CharacterAction, ControlScheme},
    PlayerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    control_scheme: Res<ControlScheme>,
) {
    commands
        .spawn(RigidBody::Fixed)
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(25.0, 0.0, 25.0));

    // Add Player
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule::default())),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 3.0, 0.0),
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default())
        .insert(InputManagerBundle::<CharacterAction> {
            input_map: control_scheme.character_input.clone(),
            ..default()
        })
        // Add Camera
        .with_children(|commands| {
            commands.spawn(Camera3dWithInputBundle {
                camera_3d: Camera3dBundle::default(),
                input: InputManagerBundle::<CameraAction> {
                    input_map: control_scheme.camera_input.clone(),
                    ..default()
                },
            });
        });
}
