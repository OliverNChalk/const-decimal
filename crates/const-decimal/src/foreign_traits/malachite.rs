use malachite::num::basic::integers::PrimitiveInt;
use malachite::Rational;

use crate::{Decimal, ScaledInteger};

impl<I, const D: u8> From<Decimal<I, D>> for Rational
where
    I: ScaledInteger<D> + PrimitiveInt,
    malachite::Integer: From<I>,
{
    fn from(value: Decimal<I, D>) -> Self {
        Rational::from_integers(
            malachite::Integer::from(value.0),
            malachite::Integer::from(I::SCALING_FACTOR),
        )
    }
}
