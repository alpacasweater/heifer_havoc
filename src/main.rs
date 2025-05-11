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
use bevy::window::{PresentMode, WindowTheme};
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
    // Bevy built-ins
    // .add_plugins(DefaultPlugins)
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Heifer Havoc".into(),
                name: Some("bevy.app".into()),
                // resolution: (500., 300.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells Wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells Wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: true,
                    ..Default::default()
                },
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                visible: true,
                ..default()
            }),
            ..default()
        }),
    ))
    // Avian3d physics engine
    .add_plugins(PhysicsPlugins::default())
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

