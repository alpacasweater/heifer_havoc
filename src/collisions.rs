use bevy::{
    color::palettes::css::GOLD,
    prelude::*,
};
use avian3d::prelude::*;


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
        .add_systems(Update, print_collisions)
        .add_systems(Update, text_update_system);
        
    }
}

fn print_collisions(
    mut commands: Commands,
    mut collision_event_reader: EventReader<Collision>,
    farmers: Query<(), With<Farmer>>,
    poops: Query<(), With<Poop>>,
    mut score: ResMut<Score>,
) {
    for Collision(contacts) in collision_event_reader.read() {
        let a = contacts.entity1;
        let b = contacts.entity2;

        let is_farmer_and_poop =
            (farmers.get(a).is_ok() && poops.get(b).is_ok()) ||
            (poops.get(a).is_ok() && farmers.get(b).is_ok());

        if is_farmer_and_poop {
            commands.entity(a).despawn();
            commands.entity(b).despawn();

            // Update score
            score.0 += 1;
        }
    }
}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {

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

fn text_update_system(
    mut query: Query<&mut TextSpan, With<ScoreText>>,
    score: ResMut<Score>,
) {
    for mut span in &mut query {
        let value = score.0 as f32;
        **span = format!("{value:.2}");
    }
}
