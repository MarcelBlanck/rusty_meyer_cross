mod meyer_cross;
mod plugins;
mod resources;

use std::collections::HashSet;

use bevy::prelude::*;
use meyer_cross::types::Opening;
use resources::meyer_cross::{MeyerCross, Preparation};

use crate::meyer_cross::{attack_sequence::AttackSequenceLength, types::Strike};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::meyer_cross::MeyerCrossPlugin))
        .add_systems(Startup, setup)
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

static mut COUNTER: usize = 0;
static mut NEXT_SWITCH: usize = 0;
const DIST: f32 = 250.0;

fn position_attack_indicators(
    mut meyer_cross: ResMut<MeyerCross>,
    mut query: Query<(&mut Transform, &mut Text, &mut Visibility, &AttackIndicator)>,
) {
    unsafe {
        if COUNTER == NEXT_SWITCH {
            let _ = meyer_cross.randomize(
                AttackSequenceLength::randomized(),
                &vec![Preparation::Static(meyer_cross::types::Guard::Alber)]
                    .into_iter()
                    .collect::<HashSet<Preparation>>(),
                &vec![Strike::Long, Strike::Short, Strike::Flat, Strike::Fehler]
                    .into_iter()
                    .collect::<HashSet<Strike>>(),
                true,
            );
            println!("Updated Meyer Cross {:?}", meyer_cross);
            if let Some(attack_sequence) = meyer_cross.attack_sequence.as_ref() {
                NEXT_SWITCH = COUNTER + 30 * attack_sequence.len();
            } else {
                NEXT_SWITCH = COUNTER + 100;
            }
        };
        COUNTER += 1;

        if let Some(attack_sequence) = meyer_cross.attack_sequence.as_ref() {
            for (mut transform, mut text, mut visibility, attack_indicator) in query.iter_mut() {
                if let Some(attack) = attack_sequence.get(attack_indicator.0 - 1) {
                    *visibility = Visibility::Visible;
                    let inner_pos_factor = if attack_indicator.0 > 4 { 0.5 } else { 1.0 };
                    (*transform).translation = match attack.opening {
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
}
