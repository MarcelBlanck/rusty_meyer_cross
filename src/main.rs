mod meyer_cross;
mod plugins;
mod resources;

use std::collections::HashSet;

use bevy::prelude::*;
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
        font_size: 60.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::Center;

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("1", text_style.clone()).with_alignment(TextAlignment::Center),
            ..default()
        },
        AttackIndicator(0),
    ));
}

fn position_attack_indicators(mut meyer_cross: ResMut<MeyerCross>) {
    println!("Found bundle {:?}", meyer_cross);
    let _ = meyer_cross.randomize(
        AttackSequenceLength::new(4).unwrap(),
        &vec![Preparation::Static(meyer_cross::types::Guard::Alber)]
            .into_iter()
            .collect::<HashSet<Preparation>>(),
        &vec![Strike::Long, Strike::Fehler].into_iter().collect::<HashSet<Strike>>(),
        true,
    );
}
