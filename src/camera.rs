use bevy::{ecs::query, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands, query: Query<Entity, With<Camera>>) {
    if query.is_empty(){
        // Spawn the camera and set its position
        commands.spawn(Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 10.0, 15.0), // Set the camera's position (X, Y, Z)
                rotation: Quat::from_rotation_y(3.14159), // Optional: Set camera rotation (optional)
                scale: Vec3::ONE, // Optional: Set scale (default is Vec3::ONE)
            },
            ..Default::default() // Default values for other fields in Camera3dBundle
        });
    }
}
