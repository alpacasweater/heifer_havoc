use std::ops::Range;

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::movement::{MovingObjectBundle, Velocity, Acceleration};
use std::f32::consts::PI;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Y: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..0.0;
const NOMINAL_VELOCITY: f32 = 5.0;
const NOMINAL_ACCELERATION: f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 0.5;

#[derive(Component, Debug)]
pub struct Farmer;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct FarmerPlugin;
impl Plugin for FarmerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Update, spawn_farmer);
    }
}

fn spawn_farmer(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>) {
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.finished() {
        let random_vector = || Vec3::new(
            rand::random::<f32>() * (SPAWN_RANGE_X.end - SPAWN_RANGE_X.start) + SPAWN_RANGE_X.start,
            rand::random::<f32>() * (SPAWN_RANGE_Y.end - SPAWN_RANGE_Y.start) + SPAWN_RANGE_Y.start,
            rand::random::<f32>() * (SPAWN_RANGE_Z.end - SPAWN_RANGE_Z.start) + SPAWN_RANGE_Z.start,
        );

        let translation = random_vector();
        let velocity = random_vector().normalize_or_zero()*NOMINAL_VELOCITY; // Random velocity
        let acceleration = random_vector().normalize_or_zero()*NOMINAL_ACCELERATION; // Random velocity

        commands.spawn((MovingObjectBundle {
            velocity: Velocity::new(velocity),
            acceleration: Acceleration::new(acceleration),
            scene: SceneRoot(scene_assets.farmer.clone()), // Scale the farmer model down to size
            collider: Collider::capsule(1.0, 2.0),
            transform: Transform::from_translation(translation).with_scale(Vec3::splat(3.0)).with_rotation(Quat::from_euler(
                EulerRot::YXZ,
                0.0, // yaw
                PI/2.0, // pitch
                0.0, // roll
            )),
        }, 
        Farmer
    ));  

    }
}