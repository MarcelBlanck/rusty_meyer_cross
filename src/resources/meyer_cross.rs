use bevy::prelude::*;

pub use crate::meyer_cross::{
    attack_sequence::AttackSequence, preparation::Preparation, MeyerCross,
};

impl Resource for MeyerCross {}
