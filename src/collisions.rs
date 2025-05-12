use bevy::{
    color::palettes::css::GOLD,
    prelude::*,
};
use avian3d::prelude::*;
use std::collections::HashSet;

use crate::schedule::InGameSet;
use crate::cow::{Cow, Poop};
use crate::farmer::Farmer;

#[derive(Resource)]
struct Score(u32);

#[derive(Resource)]
struct HighScore(u32);

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct HighScoreText;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
        .insert_resource(HighScore(0))
        .add_systems(Startup, setup)
        .add_systems(
            Update, 
            (poop_farmer_collisions, cow_farmer_collisions, text_update_system).chain().in_set(InGameSet::DespawnEntities)
        );
    }
}

fn setup(mut commands: Commands) {

    // Text with multiple sections
    commands.spawn((
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

    commands.spawn((
            // Create a Text with multiple child spans.
            Text::new("High Score: "),
            TextFont {
                font_size: 33.0,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(15.0),
                left: Val::Px(0.0),
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: 33.0,
                // If no font is specified, the default font (a minimal subset of FiraMono) will be used.
                ..default()
            },
            TextColor(GOLD.into()),
            HighScoreText,
        ));

}

fn poop_farmer_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    farmers: Query<(), With<Farmer>>,
    poops: Query<(), With<Poop>>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
) {
    // Create a HashSet to store a single reference to each entity to despawn
    let mut farmers_to_despawn: HashSet<Entity> = HashSet::new();
    let mut poops_to_despawn: HashSet<Entity> = HashSet::new();

    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        // Check if the entity1 is a farmer and entity2 is a poop involved in the collision are a farmer and a poop
        if farmers.get(*entity1).is_ok() && poops.get(*entity2).is_ok() {
            farmers_to_despawn.insert(*entity1);
            poops_to_despawn.insert(*entity2);
        }
        else if poops.get(*entity1).is_ok() && farmers.get(*entity2).is_ok() {
            poops_to_despawn.insert(*entity1);
            farmers_to_despawn.insert(*entity2);
        }
    }

    score.0 += farmers_to_despawn.len() as u32;
    high_score.0 = high_score.0.max(score.0);
    for farmer_entity in farmers_to_despawn {
        commands.entity(farmer_entity).despawn();
        
    }

    for poop_entity in poops_to_despawn {
        commands.entity(poop_entity).despawn();
        
    }
}

fn cow_farmer_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<CollisionStarted>,
    farmers: Query<(), With<Farmer>>,
    cows: Query<(), With<Cow>>,
    mut score: ResMut<Score>,
) {
    // Create a HashSet to store a single reference to each entity to despawn
    let mut farmers_to_despawn: HashSet<Entity> = HashSet::new();
    let mut cows_to_despawn: HashSet<Entity> = HashSet::new();

    for CollisionStarted(entity1, entity2) in collision_event_reader.read() {
        // Check if the entity1 is a farmer and entity2 is a poop involved in the collision are a farmer and a poop
        if farmers.get(*entity1).is_ok() && cows.get(*entity2).is_ok() {
            farmers_to_despawn.insert(*entity1);
            cows_to_despawn.insert(*entity2);
        }
        else if cows.get(*entity1).is_ok() && farmers.get(*entity2).is_ok() {
            cows_to_despawn.insert(*entity1);
            farmers_to_despawn.insert(*entity2);
        }
    }

   
    for farmer_entity in farmers_to_despawn {
        commands.entity(farmer_entity).despawn();
        
    }

    for _cow_entity in cows_to_despawn {
        // commands.entity(cow_entity).despawn();
        score.0 = 0.0 as u32;
    }
}

fn text_update_system(
    mut score_query: Query<&mut TextSpan, (With<ScoreText>, Without<HighScoreText>)>,
    mut high_score_query: Query<&mut TextSpan, (With<HighScoreText>, Without<ScoreText>)>,
    score: ResMut<Score>,
    high_score: ResMut<HighScore>,
) {
    for mut span in &mut score_query {
        let value = score.0 as f32;
        **span = format!("{value:.2}");
    }

    for mut span in &mut high_score_query {
        let value = high_score.0 as f32;
        **span = format!("{value:.2}");
    }
}
