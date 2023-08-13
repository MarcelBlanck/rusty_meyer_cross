use std::collections::HashSet;

use super::types::{Attack, Opening, Strike};
use rand::seq::SliceRandom;
use thiserror::Error;

pub type AttackSequence = Vec<Attack>;
pub struct AttackSequenceGenerator;

pub trait GenerateAttackSequence {
    fn gen_random_attack_sequence(
        length: AttackSequenceLength,
        allowed_strikes: &HashSet<Strike>,
        doppelfehler_enabled: bool,
    ) -> Result<AttackSequence, AttackSequenceError>;
}

#[derive(Error, Debug)]
pub enum AttackSequenceError {
    #[error("Attack sequences length must be {0} to {1} attacks.")]
    UnsupportedLength(usize, usize),
    #[error("No strikes available.")]
    NoStrikes,
    #[error("Only Fehler available.")]
    OnlyFehlerNotAllowed,
}
pub struct AttackSequenceLength {
    length: usize,
}

impl Into<usize> for AttackSequenceLength {
    fn into(self) -> usize {
        self.length
    }
}

impl AttackSequenceLength {
    pub const MIN_LENGTH: usize = 2;
    pub const MAX_LENGTH: usize = 8;

    pub fn new(length: usize) -> Result<Self, AttackSequenceError> {
        if (Self::MIN_LENGTH..=Self::MAX_LENGTH).contains(&length) {
            Ok(Self { length })
        } else {
            Err(AttackSequenceError::UnsupportedLength(
                Self::MIN_LENGTH,
                Self::MAX_LENGTH,
            ))
        }
    }
}

const BASE_SEQUENCES: [[Opening; 4]; 4] = [
    [
        Opening::TopRight,
        Opening::BottomLeft,
        Opening::BottomRight,
        Opening::TopLeft,
    ],
    [
        Opening::BottomRight,
        Opening::TopLeft,
        Opening::TopRight,
        Opening::BottomLeft,
    ],
    [
        Opening::TopLeft,
        Opening::BottomRight,
        Opening::BottomLeft,
        Opening::TopRight,
    ],
    [
        Opening::BottomLeft,
        Opening::TopRight,
        Opening::TopLeft,
        Opening::BottomRight,
    ],
];

impl GenerateAttackSequence for AttackSequenceGenerator {
    fn gen_random_attack_sequence(
        length: AttackSequenceLength,
        allowed_strikes: &HashSet<Strike>,
        doppelfehler_enabled: bool,
    ) -> Result<AttackSequence, AttackSequenceError> {
        match allowed_strikes.len() {
            0 => return Err(AttackSequenceError::NoStrikes),
            1 => {
                if allowed_strikes.contains(&Strike::Fehler) {
                    return Err(AttackSequenceError::OnlyFehlerNotAllowed);
                }
            }
            _ => (),
        }

        let allowed_strikes = allowed_strikes
            .iter()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        let true_strikes = allowed_strikes
            .iter()
            .filter(|&s| *s != Strike::Fehler)
            .cloned()
            .collect::<Vec<Strike>>();

        let strikes = |fehler_allowed| {
            if fehler_allowed {
                &allowed_strikes
            } else {
                &true_strikes
            }
        };

        let mut rnd = rand::thread_rng();
        let sequence_type = BASE_SEQUENCES
            .choose(&mut rnd)
            .unwrap_or(&BASE_SEQUENCES[0]);
        let mut sequence: Vec<Attack> = vec![];

        let length: usize = length.into();
        for n in 0..length {
            let last_attack = sequence.last();
            let last_was_fehler = last_attack.is_some_and(|a| a.strike == Strike::Fehler);
            let fehler_allowed = n != length - 1 && (!last_was_fehler || doppelfehler_enabled);

            sequence.push(Attack {
                opening: sequence_type[n % sequence_type.len()],
                strike: *strikes(fehler_allowed).choose(&mut rnd).unwrap(),
            });
        }
        Ok(sequence)
    }
}
