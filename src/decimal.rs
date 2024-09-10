use std::ops::{Add, Div, Mul, Sub};

use num_traits::{ConstOne, One};

use crate::const_traits::PrecisionFactor;

pub type Uint64 = Decimal<u64, 9>;
pub type Uint128 = Decimal<u128, 18>;
pub type Int64 = Decimal<i64, 9>;
pub type Int128 = Decimal<i128, 18>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal<I, const D: u8>(pub I);

pub trait Integer<const D: u8>:
    PrecisionFactor<D>
    + ConstOne
    + One
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Clone
    + Copy
{
}

impl<I, const D: u8> Integer<D> for I where
    I: PrecisionFactor<D>
        + ConstOne
        + One
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Clone
        + Copy
{
}

impl<I, const D: u8> Decimal<I, D>
where
    I: Integer<D>,
{
    pub const ONE: Decimal<I, D> = Decimal(I::PRECISION_FACTOR);
    pub const PRECISION_FACTOR: I = I::PRECISION_FACTOR;
}

impl<I, const D: u8> Add for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Decimal(self.0 + rhs.0)
    }
}

impl<I, const D: u8> Sub for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(self.0 - rhs.0)
    }
}

impl<I, const D: u8> Mul for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal(self.0 * rhs.0 / I::PRECISION_FACTOR)
    }
}

impl<I, const D: u8> Div for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Decimal(self.0 * I::PRECISION_FACTOR / rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use super::*;

    macro_rules! test_basic_ops {
        ($variant:ty) => {
            paste! {
                #[test]
                fn [<$variant:lower _add>]() {
                    assert_eq!(
                        $variant::ONE + $variant::ONE,
                        Decimal($variant::PRECISION_FACTOR * 2),
                    );
                }

                #[test]
                fn [<$variant:lower _sub>]() {
                    assert_eq!($variant::ONE - $variant::ONE, Decimal(0));
                }

                #[test]
                fn [<$variant:lower _mul>]() {
                    assert_eq!($variant::ONE * $variant::ONE, $variant::ONE);
                }

                #[test]
                fn [<$variant:lower _div>]() {
                    assert_eq!($variant::ONE / $variant::ONE, $variant::ONE);
                }
            }
        };
    }

    test_basic_ops!(Uint64);
    test_basic_ops!(Uint128);
    test_basic_ops!(Int64);
    test_basic_ops!(Int128);
}
