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
use resources::meyer_cross::{MeyerCross, Preparation};

use crate::meyer_cross::{attack_sequence::AttackSequenceLength, types::Strike};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::meyer_cross::MeyerCrossPlugin))
        .add_plugins(MeyerUiPlugin)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, position_attack_indicators)
        .add_systems(Update, apply_ui_selections)
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

fn apply_ui_selections(mut meyer_cross: ResMut<MeyerCross>, mut ui_state: ResMut<UiState>) {
    let length = AttackSequenceLength::randomized_range(
        ui_state.attack_count_min,
        ui_state.attack_count_max,
    )
    .expect("Attack Sequence Length out of bounds");

    let mut prepatations = HashSet::new();

    if ui_state.random_streichen {
        prepatations.insert(Preparation::Streichen(StreichenCount::One));
        prepatations.insert(Preparation::Streichen(StreichenCount::Two));
        prepatations.insert(Preparation::Streichen(StreichenCount::Three));
    } else if ui_state.streichen {
        prepatations.insert(Preparation::Streichen(StreichenCount::Three));
    }

    if ui_state.tag_left {
        prepatations.insert(Preparation::Static(Guard::TagLeft));
    }
    if ui_state.tag_right {
        prepatations.insert(Preparation::Static(Guard::TagRight));
    }
    if ui_state.tag_above {
        prepatations.insert(Preparation::Static(Guard::TagAbove));
    }
    if ui_state.pflug_left {
        prepatations.insert(Preparation::Static(Guard::PflugLeft));
    }
    if ui_state.pflug_right {
        prepatations.insert(Preparation::Static(Guard::PflugRight));
    }
    if ui_state.ochs_left {
        prepatations.insert(Preparation::Static(Guard::OchsLeft));
    }
    if ui_state.ochs_right {
        prepatations.insert(Preparation::Static(Guard::OchsRight));
    }
    if ui_state.eisenport {
        prepatations.insert(Preparation::Static(Guard::Eisenport));
    }
    if ui_state.langort {
        prepatations.insert(Preparation::Static(Guard::Langort));
    }
    if ui_state.alber {
        prepatations.insert(Preparation::Static(Guard::Alber));
    }

    let mut strikes = HashSet::new();
    if ui_state.long_edge_allowed {
        strikes.insert(Strike::Long);
    }
    if ui_state.short_edge_allowed {
        strikes.insert(Strike::Short);
    }
    if ui_state.flat_allowed {
        strikes.insert(Strike::Flat);
    }
    if ui_state.fehler_allowed {
        strikes.insert(Strike::Fehler);
    }

    if ui_state.apply_pending {
        let _ = meyer_cross.randomize(
            length,
            &prepatations,
            &strikes,
            ui_state.doppelfehler_allowed,
        );

        ui_state.apply_pending = false;
    }
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
