use bevy::{
    color::palettes::css::GOLD,
    prelude::*,
};
use avian3d::prelude::*;
use std::collections::HashSet;

use crate::schedule::InGameSet;
use crate::cow::Poop;
use crate::farmer::Farmer;

#[derive(Resource)]
struct Score(u32);

#[derive(Component)]
struct ScoreText;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(
            Update, 
            (poop_farmer_collisions, text_update_system).chain().in_set(InGameSet::DespawnEntities)
        );
    }
}

fn setup(mut commands: Commands) {

    // Text with multiple sections
    commands
        .spawn((
            // Create a Text with multiple child spans.
            Text::new("Score: "),
            TextFont {
                font_size: 33.0,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..default()
            }
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 33.0,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..default()
            },
            TextColor(GOLD.into()),
            ScoreText,
        ));
}

fn poop_farmer_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    farmers: Query<(), With<Farmer>>,
    poops: Query<(), With<Poop>>,
    mut score: ResMut<Score>,
) {
    // Create a HashSet to store a single reference to each entity to despawn
    let mut farmers_to_despawn: HashSet<Entity> = HashSet::new();
    let mut poops_to_despawn: HashSet<Entity> = HashSet::new();

    for Collision(contacts) in collision_event_reader.read() {
        // Check if the entity1 is a farmer and entity2 is a poop involved in the collision are a farmer and a poop
        if farmers.get(contacts.entity1).is_ok() && poops.get(contacts.entity2).is_ok() {
            farmers_to_despawn.insert(contacts.entity1);
            poops_to_despawn.insert(contacts.entity2);
        }
        else if poops.get(contacts.entity1).is_ok() && farmers.get(contacts.entity2).is_ok() {
            poops_to_despawn.insert(contacts.entity1);
            farmers_to_despawn.insert(contacts.entity2);
        }
    }

    score.0 += farmers_to_despawn.len() as u32;
    for farmer_entity in farmers_to_despawn {
        commands.entity(farmer_entity).despawn();
        
    }

    for poop_entity in poops_to_despawn {
        commands.entity(poop_entity).despawn();
        
    }
}

fn text_update_system(
    mut query: Query<&mut TextSpan, With<ScoreText>>,
    score: ResMut<Score>,
) {
    for mut span in &mut query {
        let value = score.0 as f32;
        **span = format!("{value:.2}");
    }
}
