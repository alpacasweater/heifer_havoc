use bevy::prelude::*;
use avian3d::prelude::*;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_collisions);
    }
}

fn print_collisions(mut commands: Commands, mut collision_event_reader: EventReader<Collision>) {
    for Collision(contacts) in collision_event_reader.read() {
        commands.entity(contacts.entity1).despawn();
        commands.entity(contacts.entity2).despawn();
    }
}