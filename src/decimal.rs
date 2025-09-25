use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

use crate::display::ParseDecimalError;
use crate::integer::{ScaledInteger, SignedScaledInteger};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "borsh", derive(borsh::BorshSerialize, borsh::BorshDeserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Decimal<I, const D: u8>(pub I);

impl<I, const D: u8> Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    pub const ZERO: Decimal<I, D> = Decimal(I::ZERO);
    pub const ONE: Decimal<I, D> = Decimal(I::SCALING_FACTOR);
    pub const TWO: Decimal<I, D> = Decimal(I::TWO_SCALING_FACTOR);
    pub const MIN: Decimal<I, D> = Decimal(I::MIN);
    pub const MAX: Decimal<I, D> = Decimal(I::MAX);
    pub const DECIMALS: u8 = D;
    pub const SCALING_FACTOR: I = I::SCALING_FACTOR;

    #[deprecated(note = "use Self::MIN")]
    #[must_use]
    pub const fn min() -> Self {
        Self::MIN
    }

    #[deprecated(note = "use Self::MAX")]
    #[must_use]
    pub const fn max() -> Self {
        Self::MAX
    }

    /// Losslessly converts a scaled integer to this type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use const_decimal::Decimal;
    ///
    /// let five = Decimal::<u64, 3>::try_from_scaled(5, 0).unwrap();
    /// assert_eq!(five, Decimal::TWO + Decimal::TWO + Decimal::ONE);
    /// assert_eq!(five.0, 5000);
    /// ```
    pub fn try_from_scaled(integer: I, scale: u8) -> Option<Self> {
        match scale.cmp(&D) {
            Ordering::Greater => {
                // SAFETY: We know `scale > D` so this cannot underflow.
                #[allow(clippy::arithmetic_side_effects)]
                let divisor = I::TEN.pow(u32::from(scale - D));

                // SAFETY: `divisor` cannot be zero as `x.pow(y)` cannot return 0.
                #[allow(clippy::arithmetic_side_effects)]
                let remainder = integer % divisor;
                if remainder != I::ZERO {
                    // NB: Cast would lose precision.
                    return None;
                }

                integer.checked_div(&divisor).map(Decimal)
            }
            Ordering::Less => {
                // SAFETY: We know `scale < D` so this cannot underflow.
                #[allow(clippy::arithmetic_side_effects)]
                let multiplier = I::TEN.pow(u32::from(D - scale));

                integer.checked_mul(&multiplier).map(Decimal)
            }
            Ordering::Equal => Some(Decimal(integer)),
        }
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.0 == I::ZERO
    }

    /// Round a number to a multiple of a given `quantum` toward zero.
    /// general ref: <https://en.wikipedia.org/wiki/Quantization_(signal_processing)>
    ///
    /// By default, rust is rounding towards zero and so does this method.
    ///
    /// # Example:
    /// ```rust
    /// use const_decimal::Decimal;
    /// // 11.65
    /// let d = Decimal::<i64, 5>::try_from_scaled(1165, 2).unwrap();
    /// // Allow only increments of 0.5
    /// let quantum = Decimal::<i64, 5>::try_from_scaled(5, 1).unwrap();
    /// let q = d.quantize_round_to_zero(quantum);
    /// // 11.5 rounded down to the nearest `quantum`.
    /// assert_eq!(q, Decimal::try_from_scaled(115, 1).unwrap());
    /// ```
    #[inline]
    #[must_use]
    pub fn quantize_round_to_zero(&self, quantum: Self) -> Self {
        // SAFETY: We know the multiplication cannot overflow as we previously divided
        // by the same number (and rust is rounding towards zero by default).
        #[allow(clippy::arithmetic_side_effects)]
        Self((self.0 / quantum.0) * quantum.0)
    }
}

impl<I, const D: u8> num_traits::Zero for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    #[inline]
    fn zero() -> Self {
        Self(I::zero())
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<I, const D: u8> num_traits::One for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    #[inline]
    fn one() -> Self {
        Self(I::SCALING_FACTOR)
    }
}

impl<I, const D: u8> num_traits::Num for Decimal<I, D>
where
    I: SignedScaledInteger<D>,
{
    type FromStrRadixErr = ParseDecimalError<I>;

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if radix != 10 {
            return Err(ParseDecimalError::RadixMustBe10);
        }

        Self::from_str(str)
    }
}

impl<I, const D: u8> num_traits::Signed for Decimal<I, D>
where
    I: SignedScaledInteger<D>,
{
    fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    fn abs_sub(&self, other: &Self) -> Self {
        Self(self.0.abs_sub(&other.0))
    }

    fn signum(&self) -> Self {
        Self(self.0.signum())
    }

    fn is_positive(&self) -> bool {
        self.0.is_positive()
    }

    fn is_negative(&self) -> bool {
        self.0.is_negative()
    }
}

impl<I, const D: u8> Add for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Decimal(self.0.checked_add(&rhs.0).unwrap())
    }
}

impl<I, const D: u8> Sub for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Decimal(self.0.checked_sub(&rhs.0).unwrap())
    }
}

impl<I, const D: u8> Mul for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal(I::full_mul_div(self.0, rhs.0, I::SCALING_FACTOR))
    }
}

impl<I, const D: u8> Div for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Decimal(I::full_mul_div(self.0, I::SCALING_FACTOR, rhs.0))
    }
}

impl<I, const D: u8> std::ops::Rem for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    type Output = Self;

    #[inline]
    fn rem(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_rem(&rhs.0).unwrap())
    }
}

impl<I, const D: u8> Neg for Decimal<I, D>
where
    I: SignedScaledInteger<D>,
{
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Decimal(self.0.checked_neg().unwrap())
    }
}

impl<I, const D: u8> AddAssign for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Decimal(self.0.checked_add(&rhs.0).unwrap());
    }
}

impl<I, const D: u8> SubAssign for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Decimal(self.0.checked_sub(&rhs.0).unwrap());
    }
}

impl<I, const D: u8> MulAssign for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Decimal(I::full_mul_div(self.0, rhs.0, I::SCALING_FACTOR));
    }
}

impl<I, const D: u8> DivAssign for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = Decimal(I::full_mul_div(self.0, I::SCALING_FACTOR, rhs.0));
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::ops::Shr;

    use malachite::num::basic::traits::Zero;
    use malachite::{Integer, Rational};
    use paste::paste;
    use proptest::prelude::*;

    use super::*;

    macro_rules! test_basic_ops {
        ($underlying:ty, $decimals:literal) => {
            paste! {
                #[test]
                fn [<num_traits_one_ $underlying _ $decimals _add>]() {
                    use num_traits::One;
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::one(),
                        Decimal::try_from_scaled(1, 0).unwrap(),
                    );
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::one(),
                        Decimal::try_from_scaled(10, 1).unwrap(),
                    );
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::one(),
                        Decimal::try_from_scaled(100, 2).unwrap(),
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _add>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::ONE + Decimal::ONE,
                        Decimal::TWO,
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _sub>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::ONE - Decimal::ONE,
                        Decimal::ZERO,
                    )
                }

                #[test]
                fn [<$underlying _ $decimals _mul>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::ONE * Decimal::ONE,
                        Decimal::ONE,
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _div>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::ONE / Decimal::ONE,
                        Decimal::ONE,
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _mul_min_by_one>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::MIN
                            * Decimal::<$underlying, $decimals>::ONE,
                        Decimal::MIN
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _div_min_by_one>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::MIN
                            / Decimal::<$underlying, $decimals>::ONE,
                        Decimal::MIN
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _mul_max_by_one>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::MAX
                            * Decimal::<$underlying, $decimals>::ONE,
                        Decimal::MAX,
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _div_max_by_one>]() {
                    assert_eq!(
                        Decimal::<$underlying, $decimals>::MAX
                            / Decimal::<$underlying, $decimals>::ONE,
                        Decimal::MAX,
                    );
                }

                #[test]
                fn [<$underlying _ $decimals _add_assign>]() {
                    let mut out = Decimal::<$underlying, $decimals>::ONE;
                    out += Decimal::ONE;

                    assert_eq!(out, Decimal::ONE + Decimal::ONE);
                }

                #[test]
                fn [<$underlying _ $decimals _sub_assign>]() {
                    let mut out = Decimal::<$underlying, $decimals>::ONE;
                    out -= Decimal::<$underlying, $decimals>::ONE;

                    assert_eq!(out, Decimal::ZERO);
                }

                #[test]
                fn [<$underlying _ $decimals _mul_assign>]() {
                    let mut out = Decimal::<$underlying, $decimals>::ONE;
                    out *= Decimal::TWO;

                    assert_eq!(out, Decimal::ONE + Decimal::ONE);
                }

                #[test]
                fn [<$underlying _ $decimals _div_assign>]() {
                    let mut out = Decimal::<$underlying, $decimals>::ONE;
                    out /= Decimal::TWO;

                    assert_eq!(out, Decimal::ONE / Decimal::TWO);
                }

                #[test]
                fn [<$underlying _ $decimals _quantize_toward_zero_0>]() {
                    let quantum = Decimal::<$underlying, $decimals>::try_from_scaled(5, 1).unwrap();
                    let original = Decimal::<$underlying, $decimals>::try_from_scaled(61, 1)
                        .unwrap();
                    assert_eq!(
                        original.quantize_round_to_zero(quantum),
                        Decimal::try_from_scaled(60, 1).unwrap(),
                    );
                    let original = Decimal::<$underlying, $decimals>::try_from_scaled(49, 1)
                        .unwrap();
                    assert_eq!(
                        original.quantize_round_to_zero(quantum),
                        Decimal::try_from_scaled(45, 1).unwrap(),
                    );
                    let original = Decimal::<$underlying, $decimals>::try_from_scaled(44, 1)
                        .unwrap();
                    assert_eq!(
                        original.quantize_round_to_zero(quantum),
                        Decimal::try_from_scaled(40, 1).unwrap(),
                    );

                    let quantum = Decimal::<$underlying, $decimals>::try_from_scaled(2, 1).unwrap();
                    let original = Decimal::<$underlying, $decimals>::try_from_scaled(61, 1)
                        .unwrap();
                    assert_eq!(
                        original.quantize_round_to_zero(quantum),
                        Decimal::try_from_scaled(60, 1).unwrap(),
                    );
                    let original = Decimal::<$underlying, $decimals>::try_from_scaled(49, 1)
                        .unwrap();
                    assert_eq!(
                        original.quantize_round_to_zero(quantum),
                        Decimal::try_from_scaled(48, 1).unwrap(),
                    );
                    let original = Decimal::<$underlying, $decimals>::try_from_scaled(44, 1)
                        .unwrap();
                    assert_eq!(
                        original.quantize_round_to_zero(quantum),
                        Decimal::try_from_scaled(44, 1).unwrap(),
                    );

                    let quantum = Decimal::<$underlying, $decimals>::try_from_scaled(4, 1).unwrap();
                    let original = Decimal::<$underlying, $decimals>::try_from_scaled(123, 1)
                        .unwrap();
                    assert_eq!(
                        original.quantize_round_to_zero(quantum),
                        Decimal::try_from_scaled(120, 1).unwrap(),
                    );
                }
            }
        };
    }

    macro_rules! fuzz_against_primitive {
        ($primitive:tt, $decimals:literal) => {
            paste! {
                proptest! {
                    /// Addition functions the same as regular unsigned integer addition.
                    #[test]
                    fn [<fuzz_primitive_ $primitive _ $decimals _add>](
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
                    fn [<fuzz_primitive_ $primitive _ $decimals _sub>](
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
                    fn [<fuzz_primitive_ $primitive _ $decimals _mul>](
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
                    fn [<fuzz_primitive_ $primitive _ $decimals _div>](
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

    macro_rules! differential_fuzz {
        ($underlying:ty, $decimals:literal) => {
            paste! {
                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _add>]() {
                    differential_fuzz_add::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _sub>]() {
                    differential_fuzz_sub::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _mul>]() {
                    differential_fuzz_mul::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _div>]() {
                    differential_fuzz_div::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _add_assign>]() {
                    differential_fuzz_add_assign::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _sub_assign>]() {
                    differential_fuzz_sub_assign::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _mul_assign>]() {
                    differential_fuzz_mul_assign::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _div_assign>]() {
                    differential_fuzz_div_assign::<$underlying, $decimals>();
                }

                #[test]
                fn [<differential_fuzz_ $underlying _ $decimals _from_scaled>]() {
                    differential_fuzz_from_scaled::<$underlying, $decimals>();
                }
            }
        };
    }

    fn differential_fuzz_add<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            let Ok(out) = std::panic::catch_unwind(|| a + b) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) + Rational::from(b);

            assert_eq!(Rational::from(out), reference_out);
        });
    }

    fn differential_fuzz_sub<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            let Ok(out) = std::panic::catch_unwind(|| a - b) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) - Rational::from(b);

            assert_eq!(Rational::from(out), reference_out);
        });
    }

    fn differential_fuzz_mul<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe + Into<Integer>,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            let Ok(out) = std::panic::catch_unwind(|| a * b) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) * Rational::from(b);

            // If the multiplication contains truncation ignore it.
            let scaling: Integer = Decimal::<I, D>::SCALING_FACTOR.into();
            let divisor = Integer::from(reference_out.denominator_ref());
            if scaling % divisor != Integer::ZERO {
                // TODO: Can we assert they are within N of each other?
                return Ok(());
            }

            assert_eq!(Rational::from(out), reference_out, "{} {a:?} {b:?} {out:?} {reference_out:?}", I::SCALING_FACTOR);
        });
    }

    fn differential_fuzz_div<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe + Into<Integer>,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            if b == Decimal::ZERO {
                return Ok(());
            }

            let Ok(out) = std::panic::catch_unwind(|| a / b) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) / Rational::from(b);

            // If the division contains truncation ignore it.
            let scaling: Integer = Decimal::<I, D>::SCALING_FACTOR.into();
            let divisor = Integer::from(reference_out.denominator_ref());
            if scaling % divisor != Integer::ZERO {
                // TODO: Can we assert they are within N of each other?
                return Ok(());
            }

            assert_eq!(Rational::from(out), reference_out);
        });
    }

    fn differential_fuzz_add_assign<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            let Ok(out) = std::panic::catch_unwind(|| {
                let mut out = a;
                out += b;

                out
            }) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) + Rational::from(b);

            assert_eq!(Rational::from(out), reference_out);
        });
    }

    fn differential_fuzz_sub_assign<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            let Ok(out) = std::panic::catch_unwind(|| {
                let mut out = a;
                out -= b;

                out
            }) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) - Rational::from(b);

            assert_eq!(Rational::from(out), reference_out);
        });
    }

    fn differential_fuzz_mul_assign<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe + Into<Integer>,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            let Ok(out) = std::panic::catch_unwind(|| {
                let mut out = a;
                out *= b;

                out
            }) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) * Rational::from(b);

            // If the multiplication contains truncation ignore it.
            let scaling: Integer = Decimal::<I, D>::SCALING_FACTOR.into();
            let divisor = Integer::from(reference_out.denominator_ref());
            if scaling % divisor != Integer::ZERO {
                // TODO: Can we assert they are within N of each other?
                return Ok(());
            }

            assert_eq!(Rational::from(out), reference_out);
        });
    }

    fn differential_fuzz_div_assign<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe + Into<Integer>,
        Rational: From<Decimal<I, D>>,
    {
        proptest!(|(a: Decimal<I, D>, b: Decimal<I, D>)| {
            let Ok(out) = std::panic::catch_unwind(|| {
                let mut out = a;
                out /= b;

                out
            }) else {
                return Ok(());
            };
            let reference_out = Rational::from(a) / Rational::from(b);

            // If the division contains truncation ignore it.
            let scaling: Integer = Decimal::<I, D>::SCALING_FACTOR.into();
            let divisor = Integer::from(reference_out.denominator_ref());
            if scaling % divisor != Integer::ZERO {
                // TODO: Can we assert they are within N of each other?
                return Ok(());
            }

            assert_eq!(Rational::from(out), reference_out);
        });
    }

    fn differential_fuzz_from_scaled<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + std::panic::RefUnwindSafe + Into<Integer> + TryInto<u64>,
        Rational: From<I> + From<Decimal<I, D>>,
        <I as TryInto<u64>>::Error: Debug,
    {
        proptest!(|(integer: I, decimals_percent in 0..100u64)| {
            let max_decimals: u64 = crate::algorithms::log10(I::max_value()).try_into().unwrap();
            let decimals = u8::try_from(decimals_percent * max_decimals / 100).unwrap();
            let scaling = I::TEN.pow(u32::from(decimals));

            let out = Decimal::try_from_scaled(integer, decimals);
            let reference_out = Rational::from_integers(integer.into(), scaling.into());

            match out {
                Some(out) => assert_eq!(Rational::from(out), reference_out),
                None => {
                    let scaling: Integer = Decimal::<I, D>::SCALING_FACTOR.into();
                    let remainder = &scaling % Integer::from(reference_out.denominator_ref());
                    let information = &reference_out * Rational::from(scaling);

                    assert!(
                        remainder != 0
                            || information > Rational::from(I::max_value())
                            || information < Rational::from(I::min_value()) ,
                        "Failed to parse valid input; integer={integer}; input_scale={decimals}; \
                        output_scale={D}",
                    );
                }
            }
        });
    }

    crate::macros::apply_to_common_variants!(test_basic_ops);
    crate::macros::apply_to_common_variants!(fuzz_against_primitive);
    crate::macros::apply_to_common_variants!(differential_fuzz);
}
