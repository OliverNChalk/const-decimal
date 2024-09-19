use decimal_shared::BasicInteger;
use num_traits::CheckedNeg;

use crate::cheats::Cheats;
use crate::full_mul_div::FullMulDiv;

pub trait ScaledInteger<const D: u8>: BasicInteger + FullMulDiv + Cheats<D> {}

impl<I, const D: u8> ScaledInteger<D> for I where I: BasicInteger + FullMulDiv + Cheats<D> {}

pub trait SignedScaledInteger<const D: u8>: ScaledInteger<D> + CheckedNeg {}

impl<I, const D: u8> SignedScaledInteger<D> for I where I: ScaledInteger<D> + CheckedNeg {}
