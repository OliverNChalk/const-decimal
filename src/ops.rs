use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Decimal;
use crate::traits::{Integer, SignedInteger};

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
        Decimal(I::full_mul_div(self.0, rhs.0, I::PRECISION_FACTOR))
    }
}

impl<I, const D: u8> Div for Decimal<I, D>
where
    I: Integer<D>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Decimal(I::full_mul_div(self.0, I::PRECISION_FACTOR, rhs.0))
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
}
