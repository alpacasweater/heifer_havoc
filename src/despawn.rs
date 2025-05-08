use bevy::prelude::*;
use crate::cow::{Cow, Poop};
use crate::farmer::Farmer;
use crate::schedule::InGameSet;

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            despawn_far_away_entities.in_set(InGameSet::DespawnEntities)
    );
    }
}

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform), Or<(With<Poop>, With<Cow>, With<Farmer>)>>,
) {
   for (entity, transform) in query.iter() {
        if transform.translation().length() > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}