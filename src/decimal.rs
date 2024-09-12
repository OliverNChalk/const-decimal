use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::traits::{Integer, SignedInteger};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Decimal<I, const D: u8>(pub I);

impl<I, const D: u8> Decimal<I, D>
where
    I: Integer<D>,
{
    pub const ZERO: Decimal<I, D> = Decimal(I::ZERO);
    pub const ONE: Decimal<I, D> = Decimal(I::SCALING_FACTOR);
    pub const DECIMALS: u8 = D;
    pub const SCALING_FACTOR: I = I::SCALING_FACTOR;

    // TODO: See if we can generate a constant.
    #[must_use]
    pub fn min() -> Self {
        Decimal(I::min_value())
    }

    // TODO: See if we can generate a constant.
    #[must_use]
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
        Decimal(self.0.checked_add(&rhs.0).unwrap())
    }
}

impl<I, const D: u8> Sub for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(self.0.checked_sub(&rhs.0).unwrap())
    }
}

impl<I, const D: u8> Mul for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal(I::full_mul_div(self.0, rhs.0, I::SCALING_FACTOR))
    }
}

impl<I, const D: u8> Div for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Decimal(I::full_mul_div(self.0, I::SCALING_FACTOR, rhs.0))
    }
}

impl<I, const D: u8> Neg for Decimal<I, D>
where
    I: SignedInteger<D>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Decimal(self.0.checked_neg().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use paste::paste;

    use super::*;
    use crate::{Int128_18, Int64_9, Uint128_18, Uint64_9};

    macro_rules! test_basic_ops {
        ($variant:ty) => {
            paste! {
                #[test]
                fn [<$variant:lower _add>]() {
                    assert_eq!(
                        $variant::ONE + $variant::ONE,
                        Decimal($variant::SCALING_FACTOR * 2),
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
}
