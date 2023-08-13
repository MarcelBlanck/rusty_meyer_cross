use std::collections::HashSet;

use rand::seq::SliceRandom;
use thiserror::Error;

use super::types::Guard;

#[derive(Error, Debug)]
pub enum PreparationError {
    #[error("No allowed preparation given.")]
    NoPreparation,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum Preparation {
    Static(Guard),
    Streichen(StreichenCount),
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum StreichenCount {
    One,
    Two,
    Three,
}

pub struct PreparationRandomizer;

pub trait RandomizePreparation {
    fn gen_random_preparation(
        allowed_preparations: &HashSet<Preparation>,
    ) -> Result<Preparation, PreparationError>;
}

impl RandomizePreparation for PreparationRandomizer {
    fn gen_random_preparation(
        allowed_preparations: &HashSet<Preparation>,
    ) -> Result<Preparation, PreparationError> {
        if allowed_preparations.is_empty() {
            Err(PreparationError::NoPreparation)
        } else {
            Ok(allowed_preparations
                .iter()
                .map(|v| v.to_owned())
                .collect::<Vec<_>>()
                .choose(&mut rand::thread_rng())
                .unwrap_or(&Preparation::Streichen(StreichenCount::Three))
                .to_owned())
        }
    }
}

impl Into<usize> for StreichenCount {
    fn into(self) -> usize {
        match self {
            StreichenCount::One => 1,
            StreichenCount::Two => 2,
            StreichenCount::Three => 3,
        }
    }
}
