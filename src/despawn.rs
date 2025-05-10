use bevy::prelude::*;
use crate::cow::{Cow, Poop};
use crate::farmer::Farmer;
use crate::schedule::InGameSet;
use std::collections::HashSet;

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
    despawn_query: Query<(Entity, &GlobalTransform), (Or<(With<Poop>, With<Farmer>)>, Without<Cow>)>,
    cow_query: Query<(Entity, &GlobalTransform), With<Cow>>,
) {
    // Create a HashSet to store a single reference to each entity to despawn
    let mut entities_to_despawn: HashSet<Entity> = HashSet::new();
    for (entity, entity_transform) in despawn_query.iter() {
        let mut despawn = true;
        for (_cow_entity, cow_transform) in cow_query.iter() {
            if (entity_transform.translation() - cow_transform.translation()).length() < DESPAWN_DISTANCE {
                despawn = false;
                break;
            }
        }
        
        if despawn {
            entities_to_despawn.insert(entity);
        }
    }

    for entity in entities_to_despawn {
        commands.entity(entity).despawn();
    }
}



