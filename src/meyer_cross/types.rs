use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(FromPrimitive, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Strike {
    Flat,
    Long,
    Short,
    Fehler,
}

#[derive(FromPrimitive, Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Guard {
    TagLeft,
    TagRight,
    TagAbove,
    PflugLeft,
    PflugRight,
    OchsLeft,
    OchsRight,
    Eisenport,
    Langort,
    Alber,
}

#[derive(FromPrimitive, Clone, Copy, Hash, PartialEq, Debug)]
pub enum Opening {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Attack {
    pub opening: Opening,
    pub strike: Strike,
}

macro_rules! impl_standard_distribution {
    ($enum_type:ty, $last_index:expr) => {
        impl Distribution<$enum_type> for Standard {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $enum_type {
                let index = rng.gen_range(0..=$last_index);
                FromPrimitive::from_usize(index).unwrap()
            }
        }
    };
}

impl_standard_distribution!(Opening, Opening::BottomRight as usize);
impl_standard_distribution!(Guard, Guard::Alber as usize);
