mod debug;
mod movement;
mod cow;
mod camera;
mod farmer;
mod asset_loader;
mod despawn;
mod collisions;
mod schedule;

use avian3d::prelude::*;
use bevy::prelude::*;
// use debug::DebugPlugin;
use movement::MovementPlugin;
use cow::CowPlugin;
use camera::CameraPlugin;
use farmer::FarmerPlugin;
use asset_loader::AssetLoaderPlugin;
use despawn::DespawnPlugin;
use collisions::CollisionsPlugin;

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::srgb(0.1, 0.4, 0.1)))
    .insert_resource(AmbientLight {
        color: Color::default(),
        brightness: 0.0
    })
    // Bevy built-ins
    .add_plugins(DefaultPlugins)
    // Avian3d physics engine
    .add_plugins(PhysicsPlugins::default())
    // Rapier3D physics engine
    // .add_plugins((
    //     RapierPhysicsPlugin::<NoUserData>::default(),
    //     RapierDebugRenderPlugin::default(),
    // ))
    // User Defined Plugins
    .add_plugins(CameraPlugin)
    .add_plugins(CowPlugin)
    .add_plugins(MovementPlugin)
    // .add_plugins(DebugPlugin)
    .add_plugins(FarmerPlugin)
    .add_plugins(AssetLoaderPlugin)
    .add_plugins(DespawnPlugin)
    .add_plugins(CollisionsPlugin)
    .add_plugins(schedule::SchedulePlugin)
    .run();
}

