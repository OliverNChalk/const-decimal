use std::cmp::Ordering;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::FromStr;

use thiserror::Error;

use crate::{Decimal, ScaledInteger};

impl<I, const D: u8> Display for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sign, unsigned) = match self.0 < I::ONE {
            // NB: Integers do not implement negation, so lets use two's complement to flip the sign
            // of the signed integer (modelled as an unsigned integer).
            true => ("-", (!self.0).wrapping_add(&I::ONE)),
            false => ("", self.0),
        };
        // `SCALING_FACTOR` cannot be zero.
        #[allow(clippy::arithmetic_side_effects)]
        let integer = unsigned / I::SCALING_FACTOR;
        // `SCALING_FACTOR` cannot be zero.
        #[allow(clippy::arithmetic_side_effects)]
        let fractional = unsigned % I::SCALING_FACTOR;

        write!(f, "{sign}{integer}.{fractional:0>decimals$}", decimals = D as usize)
    }
}

impl<I, const D: u8> FromStr for Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    type Err = ParseDecimalError<I>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Strip the sign (-0 would parse to 0 and break our output).
        let unsigned_s = s.strip_prefix('-').unwrap_or(s);

        // Parse the unsigned representation.
        let (integer_s, fractional_s) = unsigned_s
            .split_once('.')
            .ok_or(ParseDecimalError::MissingDecimalPoint)?;
        let integer = I::from_str(integer_s)?;
        let fractional = I::from_str(fractional_s)?;

        let scaled_integer = integer
            .checked_mul(&I::SCALING_FACTOR)
            .ok_or(ParseDecimalError::Overflow(integer, fractional))?;

        let fractional_s_len = fractional_s.len();
        let fractional = match fractional_s_len.cmp(&(D as usize)) {
            Ordering::Equal => fractional,
            Ordering::Less => {
                // `fractional_s_len` guaranteed to be less than D.
                #[allow(clippy::arithmetic_side_effects)]
                let shortfall = D as usize - fractional_s_len;

                // TODO: Remove the `checked_mul` in favor of ensuring `D` cannot overflow.
                fractional
                    .checked_mul(&I::pow(I::TEN, shortfall.try_into().unwrap()))
                    .unwrap()
            }
            Ordering::Greater => return Err(ParseDecimalError::PrecisionLoss(fractional_s.len())),
        };
        let unsigned = scaled_integer
            .checked_add(&fractional)
            .ok_or(ParseDecimalError::Overflow(integer, fractional))?;

        // Use two's complement to convert to the signed representation.
        Ok(match unsigned_s.len() == s.len() {
            true => Decimal(unsigned),
            false => {
                debug_assert_eq!(unsigned_s.len().checked_add(1).unwrap(), s.len());

                Decimal((!unsigned).wrapping_add(&I::ONE))
            }
        })
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum ParseDecimalError<I>
where
    I: Display,
{
    #[error("Missing decimal point")]
    MissingDecimalPoint,
    #[error("Resultant decimal overflowed; integer={0}; fractional={1}")]
    Overflow(I, I),
    #[error("Failed to parse integer; err={0}")]
    ParseInt(#[from] ParseIntError),
    #[error("Could not parse without precision loss; decimals={0}")]
    PrecisionLoss(usize),
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use proptest::prelude::Arbitrary;
    use proptest::proptest;
    use proptest::test_runner::TestRunner;

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
        expect![[r#"
            Err(
                ParseInt(
                    ParseIntError {
                        kind: Empty,
                    },
                ),
            )
        "#]]
        .assert_debug_eq(&"1.".parse::<Uint64_9>());
        assert_eq!("1.0".parse::<Uint64_9>(), Ok(Uint64_9::ONE));
        assert_eq!("0.1".parse::<Uint64_9>(), Ok(Decimal(10u64.pow(8))));
        assert_eq!("0.123456789".parse::<Uint64_9>(), Ok(Decimal(123456789)));
        assert_eq!("0.012345678".parse::<Uint64_9>(), Ok(Decimal(12345678)));
        assert_eq!("0.000000001".parse::<Uint64_9>(), Ok(Decimal(1)));

        assert_eq!("0.0000000001".parse::<Uint64_9>(), Err(ParseDecimalError::PrecisionLoss(10)));
        assert_eq!(
            format!("{}.0", u64::MAX).parse::<Uint64_9>(),
            Err(ParseDecimalError::Overflow(u64::MAX, 0))
        );
        assert_eq!(
            format!("{}.0", u64::MAX / Uint64_9::SCALING_FACTOR).parse::<Uint64_9>(),
            Ok(Decimal(u64::MAX / Uint64_9::SCALING_FACTOR * Uint64_9::SCALING_FACTOR))
        );
        assert_eq!(format!("18446744073.709551615").parse::<Uint64_9>(), Ok(Decimal::max()),);
        assert_eq!(
            format!("18446744073.709551616").parse::<Uint64_9>(),
            Err(ParseDecimalError::Overflow(18446744073, 709551616)),
        );
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
        expect![[r#"
            Err(
                ParseInt(
                    ParseIntError {
                        kind: Empty,
                    },
                ),
            )
        "#]]
        .assert_debug_eq(&"1.".parse::<Int64_9>());
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

    // TODO: Round trip fuzz test does not cover strings with precision greater/less
    // than target precision.

    #[test]
    fn uint64_9_round_trip() {
        decimal_round_trip::<9, u64>();
    }

    #[test]
    fn int64_9_round_trip() {
        decimal_round_trip::<9, i64>();
    }

    #[test]
    fn uint128_18_round_trip() {
        decimal_round_trip::<9, u64>();
    }

    #[test]
    fn int128_18_round_trip() {
        decimal_round_trip::<9, i64>();
    }

    fn decimal_round_trip<const D: u8, I>()
    where
        I: ScaledInteger<D> + Arbitrary,
    {
        let mut runner = TestRunner::default();
        let input = Decimal::arbitrary();

        runner
            .run(&input, |decimal: Decimal<I, D>| {
                let round_trip = decimal.to_string().parse().unwrap();

                assert_eq!(decimal, round_trip);

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn uint64_9_parse_no_panic() {
        decimal_parse_no_panic::<9, u64>();
    }

    #[test]
    fn int64_9_parse_no_panic() {
        decimal_parse_no_panic::<9, i64>();
    }

    #[test]
    fn uint128_18_parse_no_panic() {
        decimal_parse_no_panic::<9, u64>();
    }

    #[test]
    fn int128_18_parse_no_panic() {
        decimal_parse_no_panic::<9, i64>();
    }

    fn decimal_parse_no_panic<const D: u8, I>()
    where
        I: ScaledInteger<D>,
    {
        proptest!(|(decimal_s: String)| {
            let _ = decimal_s.parse::<Decimal<I, D>>();
        });
    }

    #[test]
    fn uint64_9_parse_numeric_no_panic() {
        decimal_parse_numeric_no_panic::<9, u64>();
    }

    #[test]
    fn int64_9_parse_numeric_no_panic() {
        decimal_parse_numeric_no_panic::<9, i64>();
    }

    #[test]
    fn uint128_18_parse_numeric_no_panic() {
        decimal_parse_numeric_no_panic::<9, u64>();
    }

    #[test]
    fn int128_18_parse_numeric_no_panic() {
        decimal_parse_numeric_no_panic::<9, i64>();
    }

    fn decimal_parse_numeric_no_panic<const D: u8, I>()
    where
        I: ScaledInteger<D>,
    {
        proptest!(|(decimal_s in "[0-9]{0,24}\\.[0-9]{0,24}")| {
            let _ = decimal_s.parse::<Decimal<I, D>>();
        });
    }
}
