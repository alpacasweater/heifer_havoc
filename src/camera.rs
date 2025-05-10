use bevy::prelude::*;
use crate::cow::Cow;
use crate::schedule::InGameSet;

const CAMERA_DISTANCE: f32 = 80.0;

#[derive(Component, Debug)]
pub struct Camera;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
        .add_systems(Update, track_cow.in_set(InGameSet::EntityUpdates));
    }
} 

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, CAMERA_DISTANCE))
            .looking_at(Vec3::ZERO, Vec3::Y),
            Camera,
    ));
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, CAMERA_DISTANCE))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn track_cow(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    cow_query: Query<&Transform, (With<Cow>, Without<Camera>)>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        if let Ok(cow_transform) = cow_query.get_single() {
            camera_transform.translation = cow_transform.translation + Vec3::new(0.0, 0.0, CAMERA_DISTANCE);
            camera_transform.look_at(cow_transform.translation, Vec3::Y);
        } else {
            return;
        }
    } else {
        return;
    }
}