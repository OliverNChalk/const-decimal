use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::integer::{Integer, SignedInteger};

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

    pub fn is_zero(&self) -> bool {
        self.0 == I::ZERO
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
    use std::ops::Shr;

    use paste::paste;
    use proptest::prelude::*;

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

    macro_rules! fuzz_against_primitive {
        ($primitive:tt, $decimals:literal) => {
            paste! {
                proptest! {
                    /// Addition functions the same as regular unsigned integer addition.
                    #[test]
                    fn [<$primitive _ $decimals _add>](
                        x in $primitive::MIN..$primitive::MAX,
                        y in $primitive::MIN..$primitive::MAX,
                    ) {
                        let decimal = std::panic::catch_unwind(
                            || Decimal::<_, $decimals>(x) + Decimal(y)
                        );
                        let primitive = std::panic::catch_unwind(|| x.checked_add(y).unwrap());

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
                    fn [<$primitive _ $decimals _sub>](
                        x in $primitive::MIN..$primitive::MAX,
                        y in $primitive::MIN..$primitive::MAX,
                    ) {
                        let decimal = std::panic::catch_unwind(
                            || Decimal::<_, $decimals>(x) - Decimal(y)
                        );
                        let primitive = std::panic::catch_unwind(|| x.checked_sub(y).unwrap());

                        match (decimal, primitive) {
                            (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                            (Err(_), Err(_)) => {}
                            (decimal, primitive) => panic!(
                                "Mismatch; decimal={decimal:?}; primitive={primitive:?}",
                            )
                        }
                    }

                    /// Multiplication requires the result to be divided by the scaling factor.
                    #[test]
                    fn [<$primitive _ $decimals _mul>](
                        x in ($primitive::MIN.shr($primitive::BITS / 2))
                            ..($primitive::MAX.shr($primitive::BITS / 2)),
                        y in ($primitive::MIN.shr($primitive::BITS / 2))
                            ..($primitive::MAX.shr($primitive::BITS / 2)),
                    ) {
                        let decimal = std::panic::catch_unwind(
                            || Decimal::<_, $decimals>(x) * Decimal(y)
                        );
                        let primitive = std::panic::catch_unwind(
                            || x
                                .checked_mul(y)
                                .unwrap()
                                .checked_div($primitive::pow(10, $decimals))
                                .unwrap()
                        );

                        match (decimal, primitive) {
                            (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                            (Err(_), Err(_)) => {}
                            (decimal, primitive) => panic!(
                                "Mismatch; decimal={decimal:?}; primitive={primitive:?}"
                            )
                        }
                    }

                    /// Division requires the numerator to first be scaled by the scaling factor.
                    #[test]
                    fn [<$primitive _ $decimals _div>](
                        x in ($primitive::MIN / $primitive::pow(10, $decimals))
                            ..($primitive::MAX / $primitive::pow(10, $decimals)),
                        y in ($primitive::MIN / $primitive::pow(10, $decimals))
                            ..($primitive::MAX / $primitive::pow(10, $decimals)),
                    ) {
                        let decimal = std::panic::catch_unwind(
                            || Decimal::<_, $decimals>(x) / Decimal(y)
                        );
                        let primitive = std::panic::catch_unwind(
                            || x
                                .checked_mul($primitive::pow(10, $decimals))
                                .unwrap()
                                .checked_div(y)
                                .unwrap()
                        );

                        match (decimal, primitive) {
                            (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                            (Err(_), Err(_)) => {}
                            (decimal, primitive) => panic!(
                                "Mismatch; decimal={decimal:?}; primitive={primitive:?}"
                            )
                        }
                    }
                }
            }
        };
    }

    fuzz_against_primitive!(u8, 1);
    fuzz_against_primitive!(i8, 1);
    fuzz_against_primitive!(u16, 2);
    fuzz_against_primitive!(i16, 2);
    fuzz_against_primitive!(u32, 5);
    fuzz_against_primitive!(i32, 5);
    fuzz_against_primitive!(u64, 9);
    fuzz_against_primitive!(i64, 9);
    fuzz_against_primitive!(u128, 18);
    fuzz_against_primitive!(i128, 18);
}
