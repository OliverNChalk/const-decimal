use std::cmp::Ordering;
use std::fmt::Display;
use std::num::ParseFloatError;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use num_traits::{Bounded, ConstOne, ConstZero, One};

use crate::cheats::PrecisionFactor;
use crate::mul_div::MulDiv;

pub type Uint64_9 = Decimal<u64, 9>;
pub type Uint128_18 = Decimal<u128, 18>;
pub type Int64_9 = Decimal<i64, 9>;
pub type Int128_18 = Decimal<i128, 18>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Decimal<I, const D: u8>(pub I);

pub trait Integer<const D: u8>:
    // `const_decimal`
    PrecisionFactor<D>
    + MulDiv
    // `num-traits`
    + ConstZero
    + ConstOne
    + One
    + Bounded
    // `std`
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
        + MulDiv
        + ConstZero
        + ConstOne
        + One
        + Bounded
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Clone
        + Copy
{
}

pub trait SignedInteger<const D: u8>: Integer<D> + Neg<Output = Self> {}

impl<I, const D: u8> SignedInteger<D> for I where I: Integer<D> + Neg<Output = Self> {}

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
        Decimal(I::mul_div(self.0, rhs.0, I::PRECISION_FACTOR))
    }
}

impl<I, const D: u8> Div for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Decimal(I::mul_div(self.0, I::PRECISION_FACTOR, rhs.0))
    }
}

impl<I, const D: u8> Neg for Decimal<I, D>
where
    I: SignedInteger<D>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Decimal(-self.0)
    }
}

impl<I, const D: u8> Display for Decimal<I, D>
where
    I: Integer<D>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Display as a decimal; i.e. [125~2] -> 1.25");
    }
}

impl<I, const D: u8> FromStr for Decimal<I, D>
where
    I: Integer<D>,
{
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Parse from a string containing a floating point number; refuse to parse an integer");
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

                #[test]
                fn [<$variant:lower _mul_min_by_one>]() {
                    assert_eq!($variant::min() * $variant::ONE, $variant::min());
                }

                #[test]
                fn [<$variant:lower _div_min_by_one>]() {
                    assert_eq!($variant::min() / $variant::ONE, $variant::min());
                }

                #[test]
                fn [<$variant:lower _mul_max_by_one>]() {
                    assert_eq!($variant::max() * $variant::ONE, $variant::max());
                }

                #[test]
                fn [<$variant:lower _div_max_by_one>]() {
                    assert_eq!($variant::max() / $variant::ONE, $variant::max());
                }
            }
        };
    }

    test_basic_ops!(Uint64_9);
    test_basic_ops!(Uint128_18);
    test_basic_ops!(Int64_9);
    test_basic_ops!(Int128_18);

    mod fuzz {
        use proptest::prelude::*;

        use super::*;

        proptest! {
            /// Addition functions the same as regular unsigned integer addition.
            #[test]
            fn uint64_9_add(x in 0..u64::MAX, y in 0..u64::MAX) {
                let decimal = std::panic::catch_unwind(|| Decimal::<_, 9>(x) + Decimal(y));
                let primitive = std::panic::catch_unwind(|| x + y);

                match (decimal, primitive) {
                    (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                    (Err(_), Err(_)) => {}
                    (decimal, primitive) => panic!(
                        "Mismatch; decimal={decimal:?}; primitive={primitive:?}"
                    )
                }
            }

            /// Subtraction functions the same as regular unsigned integer addition.
            #[test]
            fn uint64_9_sub(x in 0..u64::MAX, y in 0..u64::MAX) {
                let decimal = std::panic::catch_unwind(|| Decimal::<_, 9>(x) - Decimal(y));
                let primitive = std::panic::catch_unwind(|| x - y);

                match (decimal, primitive) {
                    (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                    (Err(_), Err(_)) => {}
                    (decimal, primitive) => panic!(
                        "Mismatch; decimal={decimal:?}; primitive={primitive:?}",
                    )
                }
            }
        }
    }
}
