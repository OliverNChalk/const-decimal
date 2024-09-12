use std::cmp::Ordering;

use crate::traits::Integer;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Decimal<I, const D: u8>(pub I);

impl<I, const D: u8> Decimal<I, D>
where
    I: Integer<D>,
{
    pub const ZERO: Decimal<I, D> = Decimal(I::ZERO);
    pub const ONE: Decimal<I, D> = Decimal(I::PRECISION_FACTOR);
    pub const PRECISION: u8 = D;
    pub const PRECISION_FACTOR: I = I::PRECISION_FACTOR;

    // TODO: See if we can generate a constant.
    pub fn min() -> Self {
        Decimal(I::min_value())
    }

    // TODO: See if we can generate a constant.
    pub fn max() -> Self {
        Decimal(I::max_value())
    }

    pub fn from_scaled(integer: I, scale: u8) -> Self {
        match scale.cmp(&D) {
            Ordering::Greater => todo!(),
            Ordering::Less => todo!(),
            Ordering::Equal => Decimal(integer),
        }
    }
}
