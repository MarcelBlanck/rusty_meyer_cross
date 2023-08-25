pub mod attack_sequence;
pub mod preparation;
pub mod types;

use std::collections::HashSet;

use thiserror::Error;

use attack_sequence::{
    AttackSequenceError, AttackSequenceGenerator, AttackSequenceLength, GenerateAttackSequence,
};
use preparation::{Preparation, PreparationError, RandomizePreparation};
use types::Strike;

use self::{attack_sequence::AttackSequence, preparation::PreparationRandomizer};

#[derive(Error, Debug)]
pub enum MeyerCrossError {
    #[error(transparent)]
    AttackSequence(#[from] AttackSequenceError),
    #[error(transparent)]
    Preparation(#[from] PreparationError),
}

#[derive(Debug)]
pub struct MeyerCross {
    pub preparation: Option<Preparation>,
    pub attack_sequence: Option<AttackSequence>,
}

impl MeyerCross {
    pub fn new() -> Self {
        MeyerCross {
            preparation: None,
            attack_sequence: None,
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.preparation = None;
        self.attack_sequence = None;
    }

    pub fn randomize(
        &mut self,
        length: AttackSequenceLength,
        preparations: &HashSet<Preparation>,
        strikes: &HashSet<Strike>,
        doppelfehler_enabled: bool,
    ) -> Result<(), MeyerCrossError> {
        self.preparation = Some(PreparationRandomizer::gen_random_preparation(preparations)?);
        self.attack_sequence = Some(AttackSequenceGenerator::gen_random_attack_sequence(
            length,
            strikes,
            doppelfehler_enabled,
        )?);
        Ok(())
    }
}
