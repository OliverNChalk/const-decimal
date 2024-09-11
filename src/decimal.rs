use std::cmp::Ordering;
use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str::FromStr;

use num_traits::{Bounded, ConstOne, ConstZero, One, Pow};
use thiserror::Error;

use crate::cheats::Cheats;
use crate::full_mul_div::FullMulDiv;

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
    Cheats<D>
    + FullMulDiv
    // `num-traits`
    + ConstZero
    + ConstOne
    + One
    + Bounded
    // `std`
    + Add<Output = Self>
    + Sub<Output = Self>
    + Clone
    + Copy
{
}

impl<I, const D: u8> Integer<D> for I where
    I: Cheats<D>
        + FullMulDiv
        + ConstZero
        + ConstOne
        + One
        + Bounded
        + Add<Output = Self>
        + Sub<Output = Self>
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

impl<I, const D: u8> Display for Decimal<I, D>
where
    I: Integer<D> + Div<Output = I> + Rem<Output = I> + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let integer = self.0 / I::PRECISION_FACTOR;
        let fractional = self.0 % I::PRECISION_FACTOR;

        write!(f, "{integer}.{fractional:0>decimals$}", decimals = D as usize)
    }
}

impl<I, const D: u8> FromStr for Decimal<I, D>
where
    I: Integer<D> + FromStr<Err = ParseIntError> + Pow<usize, Output = I>,
{
    type Err = ParseDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (integer, fractional_s) = s
            .split_once('.')
            .ok_or(ParseDecimalError::MissingDecimalPoint)?;
        let integer = I::from_str(integer)?;
        let integer = integer * I::PRECISION_FACTOR;
        let fractional = I::from_str(fractional_s)?;
        let fractional = match fractional_s.len().cmp(&(D as usize)) {
            Ordering::Equal => fractional,
            Ordering::Less => {
                let shortfall = D as usize - fractional_s.len();

                fractional * I::pow(I::TEN, shortfall)
            }
            Ordering::Greater => return Err(ParseDecimalError::PrecisionLoss(fractional_s.len())),
        };

        Ok(Decimal(integer + fractional))
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseDecimalError {
    #[error("Missing decimal point")]
    MissingDecimalPoint,
    #[error("Failed to parse integer; err={0}")]
    ParseInt(#[from] ParseIntError),
    #[error("Could not parse without precision loss; decimals={0}")]
    PrecisionLoss(usize),
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

    #[test]
    fn uint64_9_to_string() {
        assert_eq!(Uint64_9::ONE.to_string(), "1.000000000");
        assert_eq!(Uint64_9::from_scaled(123, 9).to_string(), "0.000000123");
        assert_eq!((Uint64_9::ONE + Uint64_9::from_scaled(123, 9)).to_string(), "1.000000123");
    }

    #[test]
    fn uint64_9_from_str() {
        assert_eq!("".parse::<Uint64_9>(), Err(ParseDecimalError::MissingDecimalPoint));
        assert_eq!("1.0".parse::<Uint64_9>(), Ok(Uint64_9::ONE));
        assert_eq!("0.1".parse::<Uint64_9>(), Ok(Decimal(10u64.pow(8))));
        assert_eq!("0.123456789".parse::<Uint64_9>(), Ok(Decimal(123456789)));
        assert_eq!("0.012345678".parse::<Uint64_9>(), Ok(Decimal(12345678)));
        assert_eq!("0.000000001".parse::<Uint64_9>(), Ok(Decimal(1)));
        assert_eq!("0.0000000001".parse::<Uint64_9>(), Err(ParseDecimalError::PrecisionLoss(10)));
    }
}
