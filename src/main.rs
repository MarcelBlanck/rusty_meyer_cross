mod gui;
mod meyer_cross;
mod plugins;
mod resources;

use std::collections::HashSet;

use bevy::prelude::*;
use gui::setup_egui::{MeyerUiPlugin, UiState};
use meyer_cross::{
    preparation::StreichenCount,
    types::{Guard, Opening},
};
use resources::meyer_cross::{
    MeyerCross, Preparation,
    Preparation::{Static, Streichen},
};

use crate::meyer_cross::{attack_sequence::AttackSequenceLength, types::Strike};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::meyer_cross::MeyerCrossPlugin))
        .add_plugins(MeyerUiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, apply_ui_selections)
        .add_systems(Update, position_attack_indicators)
        .run();
}

#[derive(Component)]
struct AttackIndicator(usize);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("OpenSans-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 150.0,
        color: Color::WHITE,
    };

    commands.spawn(Camera2dBundle::default());

    for id in 1..=8 {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(id.to_string(), text_style.clone())
                    .with_alignment(TextAlignment::Center),
                ..default()
            },
            AttackIndicator(id),
        ));
    }
}

const DIST: f32 = 250.0;

fn apply_ui_selections(mut meyer_cross: ResMut<MeyerCross>, mut ui: ResMut<UiState>) {
    if ui.apply_pending {
        ui.apply_pending = false;
    } else {
        return;
    }

    let mut preps = HashSet::new();
    fn insert_prep(preps: &mut HashSet<Preparation>, prep: Preparation, cond: bool) {
        if cond {
            preps.insert(prep);
        }
    }

    if ui.random_streichen {
        preps.insert(Streichen(StreichenCount::One));
        preps.insert(Streichen(StreichenCount::Two));
        preps.insert(Streichen(StreichenCount::Three));
    } else if ui.streichen {
        preps.insert(Streichen(StreichenCount::Three));
    }

    insert_prep(&mut preps, Static(Guard::TagLeft), ui.tag_left);
    insert_prep(&mut preps, Static(Guard::TagRight), ui.tag_right);
    insert_prep(&mut preps, Static(Guard::TagAbove), ui.tag_above);
    insert_prep(&mut preps, Static(Guard::PflugLeft), ui.pflug_left);
    insert_prep(&mut preps, Static(Guard::PflugRight), ui.pflug_right);
    insert_prep(&mut preps, Static(Guard::OchsLeft), ui.ochs_left);
    insert_prep(&mut preps, Static(Guard::OchsRight), ui.ochs_right);
    insert_prep(&mut preps, Static(Guard::Eisenport), ui.eisenport);
    insert_prep(&mut preps, Static(Guard::Langort), ui.langort);
    insert_prep(&mut preps, Static(Guard::Alber), ui.alber);

    let mut strikes = HashSet::new();
    fn insert_strike(strikes: &mut HashSet<Strike>, strike: Strike, condition: bool) {
        if condition {
            strikes.insert(strike);
        }
    }

    insert_strike(&mut strikes, Strike::Long, ui.long_edge_allowed);
    insert_strike(&mut strikes, Strike::Short, ui.short_edge_allowed);
    insert_strike(&mut strikes, Strike::Flat, ui.flat_allowed);
    insert_strike(&mut strikes, Strike::Fehler, ui.fehler_allowed);

    let _ = meyer_cross.randomize(
        AttackSequenceLength::randomized_range(ui.attack_count_min, ui.attack_count_max)
            .expect("Attack Sequence Length out of bounds"),
        &preps,
        &strikes,
        ui.doppelfehler_allowed,
    );
}

fn position_attack_indicators(
    meyer_cross: Res<MeyerCross>,
    mut query: Query<(&mut Transform, &mut Text, &mut Visibility, &AttackIndicator)>,
) {
    if let Some(attack_sequence) = meyer_cross.attack_sequence.as_ref() {
        for (mut transform, mut text, mut visibility, attack_indicator) in query.iter_mut() {
            if let Some(attack) = attack_sequence.get(attack_indicator.0 - 1) {
                *visibility = Visibility::Visible;
                let inner_pos_factor = if attack_indicator.0 > 4 { 0.5 } else { 1.0 };
                transform.translation = match attack.opening {
                    Opening::TopLeft => Vec3::new(-DIST, DIST, 0.0),
                    Opening::TopRight => Vec3::new(DIST, DIST, 0.0),
                    Opening::BottomLeft => Vec3::new(-DIST, -DIST, 0.0),
                    Opening::BottomRight => Vec3::new(DIST, -DIST, 0.0),
                } * inner_pos_factor;
                text.sections[0].style.color = match attack.strike {
                    Strike::Flat => Color::RED,
                    Strike::Long => Color::WHITE,
                    Strike::Short => Color::BLUE,
                    Strike::Fehler => Color::GREEN,
                };
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
