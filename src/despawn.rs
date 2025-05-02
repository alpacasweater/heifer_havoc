use bevy::prelude::*;
use crate::cow::{Cow, Poop};
use crate::farmer::Farmer;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Or<(With<Poop>, With<Cow>, With<Farmer>)>>,
) {
   for (entity, transform) in query.iter() {
        if transform.translation().length() > DESPAWN_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}