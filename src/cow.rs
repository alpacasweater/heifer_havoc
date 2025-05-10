
use bevy::prelude::*;
use avian3d::prelude::*;
use std::f32::consts::PI;

use crate::asset_loader::SceneAssets;
use crate::movement::{MovingObjectBundle, Velocity, Acceleration};
use crate::schedule::InGameSet;

const START_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const COW_SPEED: f32 = 15.0;
// const COW_ROLL_RATE: f32 = 1.0; // radians per second
// const COW_PITCH_RATE: f32 = 1.0; // radians per second
const COW_YAW_RATE: f32 = 3.0; // radians per second
const COW_PROJECTILE_SPEED: f32 = 20.0;
const POOP_OFFSET_Z: f32 = -3.0; // Offset to spawn the projectile in front of the cow
// const POOP_OFFSET_Y: f32 = 3.0; // Offset to spawn the projectile at the same height as the cow
// const POOP_OFFSET_Z: f32 = 0.0; // Offset to spawn the projectile in front of the cow
const POOP_OFFSET_Y: f32 = 0.0; // Offset to spawn the projectile at the same height as the cow

#[derive(Component, Debug)]
pub struct Cow;

#[derive(Component, Debug)]
pub struct CowShield;

#[derive(Component, Debug)]
pub struct Poop;

pub struct CowPlugin;
impl Plugin for CowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_cow)
        .add_systems(
            Update, 
            (cow_movement_controls, cow_weapon_controls, cow_shield_controls).chain().in_set(InGameSet::UserInput)
        );
    }
}

fn spawn_cow(mut commands: Commands, scene_assets: Res<SceneAssets>) {

    commands.spawn((MovingObjectBundle{
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        scene: SceneRoot(scene_assets.cow.clone()),
        collider: Collider::sphere(1.0),
        transform: Transform::from_translation(START_TRANSLATION).with_rotation(Quat::from_euler(
            EulerRot::YXZ,
            PI, // yaw
            -PI/2.0, // pitch
            0.0, // roll
        )),
    }, 
    Cow
));
}

fn cow_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Cow>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    time: Res<Time>
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };

    let (_roll_rate, _pitch_rate, mut yaw_rate) = (0.0f32, 0.0f32, 0.0f32);
    let mut speed = 0.0f32;

    // Translation
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        speed = -COW_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        speed = COW_SPEED;
    }

    // Rotation 
    // yaw
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        yaw_rate = COW_YAW_RATE;
    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        yaw_rate = -COW_YAW_RATE;
    }

    // roll
    // if keyboard_input.pressed(KeyCode::KeyQ) {
    //     roll_rate = COW_ROLL_RATE;
    // } else if keyboard_input.pressed(KeyCode::KeyE) {
    //     roll_rate = -COW_ROLL_RATE;
    // }

    // pitch
    // if keyboard_input.pressed(KeyCode::KeyZ) {
    //     pitch_rate = COW_PITCH_RATE;
    // } else if keyboard_input.pressed(KeyCode::KeyC) {
    //     pitch_rate = -COW_PITCH_RATE;
    // }

    // transform.rotate_local_x(pitch_rate * time.delta_secs());
    transform.rotate_local_y(yaw_rate * time.delta_secs());
    // transform.rotate_local_z(roll_rate * time.delta_secs());

    velocity.value = transform.forward() * speed; // transform.forward() is equivalent to -local_z()
}

fn cow_weapon_controls(
    mut commands: Commands,
    query: Query<&mut Transform, With<Cow>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    scene_assets: Res<SceneAssets>,
) {
    let Ok(transform) = query.get_single() else {
        return;
    };

    // pitch
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((MovingObjectBundle{
            velocity: Velocity::new(transform.forward() * COW_PROJECTILE_SPEED), // Adjust speed as needed
            acceleration: Acceleration::new(Vec3::ZERO),
            scene: SceneRoot(scene_assets.poop.clone()), // Assuming poop is the projectile
            collider: Collider::sphere(0.1),
            transform: Transform::from_translation(transform.translation + transform.local_z() * POOP_OFFSET_Z + transform.local_y() * POOP_OFFSET_Y).with_scale(Vec3::splat(3.0)),
        }, Poop
    ));
    }
}

fn cow_shield_controls(
    mut commands: Commands,
    query: Query<Entity, With<Cow>>, 
    keyboard_input: Res<ButtonInput<KeyCode>>, 
) {
    let Ok(cow) = query.get_single() else {
        return;
    };

    if keyboard_input.pressed(KeyCode::Tab) {
        commands.entity(cow).insert(CowShield);
    }
}