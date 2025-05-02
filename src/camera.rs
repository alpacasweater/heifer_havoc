use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;

#[derive(Component, Debug)]
pub struct Camera;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
} 

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, CAMERA_DISTANCE))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, CAMERA_DISTANCE))
            .looking_at(Vec3::ZERO, Vec3::Y),
        Camera,
    ));
}