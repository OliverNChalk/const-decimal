use std::cmp::Ordering;
use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Div, Rem};
use std::str::FromStr;

use num_traits::Pow;
use thiserror::Error;

use crate::{Decimal, Integer};

impl<I, const D: u8> Display for Decimal<I, D>
where
    I: Integer<D> + Div<Output = I> + Rem<Output = I> + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sign, unsigned) = match self.0 < I::ONE {
            // NB: Integers do not implement negation, so lets use two's complement to flip the sign
            // of the signed integer (modelled as an unsigned integer).
            true => ("-", (!self.0).wrapping_add(&I::ONE)),
            false => ("", self.0),
        };
        let integer = unsigned / I::PRECISION_FACTOR;
        let fractional = unsigned % I::PRECISION_FACTOR;

        write!(f, "{sign}{integer}.{fractional:0>decimals$}", decimals = D as usize)
    }
}

impl<I, const D: u8> FromStr for Decimal<I, D>
where
    I: Integer<D> + FromStr<Err = ParseIntError> + Pow<usize, Output = I>,
{
    type Err = ParseDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip the sign (-0 would parse to 0 and break our output).
        let unsigned_s = s.strip_prefix('-').unwrap_or(s);

        // Parse the unsigned representation.
        let (integer_s, fractional_s) = unsigned_s
            .split_once('.')
            .ok_or(ParseDecimalError::MissingDecimalPoint)?;
        let integer = I::from_str(integer_s)?;
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
        let unsigned = integer + fractional;

        // Use two's complement to convert to the signed representation.
        Ok(match unsigned_s.len() == s.len() {
            true => Decimal(unsigned),
            false => {
                debug_assert_eq!(unsigned_s.len() + 1, s.len());

                Decimal((!unsigned).wrapping_add(&I::ONE))
            }
        })
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
    use super::*;
    use crate::{Int64_9, Uint64_9};

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

    #[test]
    fn int64_9_to_string() {
        assert_eq!(Int64_9::ONE.to_string(), "1.000000000");
        assert_eq!(Int64_9::from_scaled(123, 9).to_string(), "0.000000123");
        assert_eq!((Int64_9::ONE + Int64_9::from_scaled(123, 9)).to_string(), "1.000000123");
        assert_eq!((-Int64_9::ONE).to_string(), "-1.000000000");
        assert_eq!((-Int64_9::from_scaled(123, 9)).to_string(), "-0.000000123");
        assert_eq!((-Int64_9::ONE + -Int64_9::from_scaled(123, 9)).to_string(), "-1.000000123");
    }

    #[test]
    fn int64_9_from_str() {
        assert_eq!("".parse::<Int64_9>(), Err(ParseDecimalError::MissingDecimalPoint));
        assert_eq!("1.0".parse::<Int64_9>(), Ok(Int64_9::ONE));
        assert_eq!("0.1".parse::<Int64_9>(), Ok(Decimal(10i64.pow(8))));
        assert_eq!("0.123456789".parse::<Int64_9>(), Ok(Decimal(123456789)));
        assert_eq!("0.012345678".parse::<Int64_9>(), Ok(Decimal(12345678)));
        assert_eq!("0.000000001".parse::<Int64_9>(), Ok(Decimal(1)));
        assert_eq!("0.0000000001".parse::<Int64_9>(), Err(ParseDecimalError::PrecisionLoss(10)));
        assert_eq!("-1.0".parse::<Int64_9>(), Ok(-Int64_9::ONE));
        assert_eq!("-0.1".parse::<Int64_9>(), Ok(-Decimal(10i64.pow(8))));
        assert_eq!("-0.123456789".parse::<Int64_9>(), Ok(-Decimal(123456789)));
        assert_eq!("-0.012345678".parse::<Int64_9>(), Ok(-Decimal(12345678)));
        assert_eq!("-0.000000001".parse::<Int64_9>(), Ok(-Decimal(1)));
        assert_eq!("-0.0000000001".parse::<Int64_9>(), Err(ParseDecimalError::PrecisionLoss(10)));
    }

    // TODO: Proptest that round tripping a random decimal produces the same
    // value.
}
