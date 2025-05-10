use std::ops::Range;

use bevy::prelude::*;
use avian3d::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::movement::{MovingObjectBundle, Velocity, Acceleration};
use crate::schedule::InGameSet;
use std::f32::consts::PI;
use crate::cow::Cow;

const SPAWN_RANGE_DISTANCE: Range<f32> = 15.0..90.0;
const NOMINAL_VELOCITY: f32 = 8.0;
const NOMINAL_ACCELERATION: f32 = 1.0;
const SPAWN_TIME_SECONDS: f32 = 0.1;

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
        .add_systems(
            Update, 
            (spawn_farmer, update_velocity).in_set(InGameSet::EntityUpdates)
        );
    }
}

fn spawn_farmer(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>, cow_query: Query<&Transform, With<Cow>>) {
    let Ok(cow_transform) = cow_query.get_single() else {
        return;
    };
    
    spawn_timer.timer.tick(time.delta());
    if spawn_timer.timer.finished() {
        let random_vector = || {
            let theta = rand::random::<f32>() * 2.0*PI;
            Vec3::new(
                theta.cos(),
                theta.sin(),
                0.0,
            )
        };
            
        let random_distance = || {
            rand::random::<f32>() * (SPAWN_RANGE_DISTANCE.end - SPAWN_RANGE_DISTANCE.start) + SPAWN_RANGE_DISTANCE.start
        };
        let translation = random_vector()*random_distance() + cow_transform.translation;
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

fn update_velocity(mut query: Query<(&mut Velocity, &Transform), With<Farmer>>, cows: Query<&Transform, With<Cow>>) {
    let cow_positions: Vec<Vec3> = cows.iter().map(|cow| cow.translation).collect();
    for (mut velocity, transform) in query.iter_mut() {
        velocity.value =   (cow_positions.first().unwrap_or(&Vec3::ZERO) - transform.translation).normalize()*NOMINAL_VELOCITY; // Update velocity based on cow positions
    }
}