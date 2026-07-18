use ruint::Uint;
use ruint::aliases::U256;

pub trait FullMulDiv: Sized {
    /// Implements `a * b / c` with full width on the intermediate `a * b`
    /// state.
    ///
    /// # Panics
    ///
    /// Panics when `div` is zero or when the result does not fit `Self`.
    #[track_caller]
    fn full_mul_div(self, rhs: Self, div: Self) -> Self;

    /// Implements `a * b / c` with full width on the intermediate `a * b`
    /// state, returning `None` when the result does not fit `Self`.
    ///
    /// # Panics
    ///
    /// Panics when `div` is zero.
    #[track_caller]
    fn try_full_mul_div(self, rhs: Self, div: Self) -> Option<Self>;
}

macro_rules! impl_primitive {
    ($primary:ty, $intermediate:ty) => {
        impl FullMulDiv for $primary {
            #[track_caller]
            fn full_mul_div(self, rhs: Self, div: Self) -> Self {
                match self.try_full_mul_div(rhs, div) {
                    Some(out) => out,
                    None => panic!(
                        "Result out of range; ({self} * {rhs}) / {div} does not fit the backing \
                         integer type"
                    ),
                }
            }

            #[track_caller]
            fn try_full_mul_div(self, rhs: Self, div: Self) -> Option<Self> {
                assert!(div != 0, "Division by zero; lhs={self}; rhs={rhs}");

                // The intermediate type has twice the width of the primary
                // type, so neither the product nor the division can overflow.
                let numer = <$intermediate>::from(self)
                    .checked_mul(<$intermediate>::from(rhs))
                    .expect("doubled-width product cannot overflow");
                let out = numer
                    .checked_div(<$intermediate>::from(div))
                    .expect("divisor checked non-zero above");

                out.try_into().ok()
            }
        }
    };
}

impl_primitive!(u8, u16);
impl_primitive!(i8, i16);
impl_primitive!(u16, u32);
impl_primitive!(i16, i32);
impl_primitive!(u32, u64);
impl_primitive!(i32, i64);
impl_primitive!(u64, u128);
impl_primitive!(i64, i128);

impl FullMulDiv for u128 {
    #[track_caller]
    fn full_mul_div(self, rhs: Self, div: Self) -> Self {
        match self.try_full_mul_div(rhs, div) {
            Some(out) => out,
            None => panic!(
                "Result out of range; ({self} * {rhs}) / {div} does not fit the backing integer \
                 type"
            ),
        }
    }

    #[track_caller]
    fn try_full_mul_div(self, rhs: Self, div: Self) -> Option<Self> {
        assert!(div != 0, "Division by zero; lhs={self}; rhs={rhs}");

        let out: U256 = Uint::from(self)
            .checked_mul(Uint::from(rhs))
            .expect("two u128 always fit U256")
            .checked_div(Uint::from(div))
            .expect("divisor checked non-zero above");

        out.try_into().ok()
    }
}

impl FullMulDiv for i128 {
    #[track_caller]
    fn full_mul_div(self, rhs: Self, div: Self) -> Self {
        match self.try_full_mul_div(rhs, div) {
            Some(out) => out,
            None => panic!(
                "Result out of range; ({self} * {rhs}) / {div} does not fit the backing integer \
                 type"
            ),
        }
    }

    #[track_caller]
    fn try_full_mul_div(self, rhs: Self, div: Self) -> Option<Self> {
        assert!(div != 0, "Division by zero; lhs={self}; rhs={rhs}");

        // If we can compute the output using only an i128, then we should.
        // NB: `checked_div` also fails on `i128::MIN / -1`, which falls
        // through to the wide path and is reported as out of range there.
        if let Some(out) = self
            .checked_mul(rhs)
            .and_then(|numer| numer.checked_div(div))
        {
            return Some(out);
        }

        // Determine the sign of the output. Signum returns -1, 0, +1, therefore
        // overflow is not possible. Additionally, if `self` or `rhs` are zero then we
        // will have already returned, and `div` was checked non-zero above.
        #[allow(clippy::arithmetic_side_effects)]
        let sign = self.signum() * rhs.signum() * div.signum();

        // Get the unsigned u256 representation of the integer (we'll later recover the
        // signed representation using two's complement).
        let this = U256::from(self.unsigned_abs());
        let rhs = U256::from(rhs.unsigned_abs());
        let div = U256::from(div.unsigned_abs());

        // Compute the unsigned output.
        let unsigned = this
            .checked_mul(rhs)
            .expect("two i128 magnitudes always fit U256")
            .checked_div(div)
            .expect("divisor checked non-zero above");

        // Convert back to the signed output.
        match sign {
            1 => i128::try_from(unsigned).ok(),
            -1 => {
                let unsigned = u128::try_from(unsigned).ok()?;
                // The most negative representable magnitude is 2^127; larger
                // magnitudes would silently wrap through two's complement.
                if unsigned > 1u128 << 127 {
                    return None;
                }

                // Take two's complement (!unsigned + 1).
                // https://en.wikipedia.org/wiki/Two%27s_complement.
                let twos_complement = (!unsigned).overflowing_add(1).0;

                Some(i128::from_le_bytes(twos_complement.to_le_bytes()))
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use malachite::Integer;
    use proptest::prelude::*;

    use super::*;

    #[test]
    fn u128_full_mul_div() {
        proptest!(|(a: u128, b: u128, div: u128)| {
            if div == 0 {
                return Ok(());
            }

            // Compute reference value.
            let reference = Integer::from(a) * Integer::from(b) / Integer::from(div);

            // If the output fits in a u128 then ours should match.
            if let Ok(reference) = u128::try_from(&reference) {
                assert_eq!(u128::full_mul_div(a, b, div), reference);
            }
        });
    }

    #[test]
    fn i128_full_mul_div() {
        proptest!(|(a: i128, b: i128, div: i128)| {
            if div == 0 {
                return Ok(());
            }

            // Compute reference value.
            let reference = Integer::from(a) * Integer::from(b) / Integer::from(div);

            // If the output fits in an i128 then ours should match.
            if let Ok(reference) = i128::try_from(&reference) {
                assert_eq!(i128::full_mul_div(a, b, div), reference);
            }
        });
    }

    #[test]
    fn i128_try_full_mul_div_rejects_out_of_range_results() {
        proptest!(|(a: i128, b: i128, div: i128)| {
            if div == 0 {
                return Ok(());
            }

            let reference = Integer::from(a) * Integer::from(b) / Integer::from(div);
            match i128::try_from(&reference) {
                Ok(reference) => assert_eq!(i128::try_full_mul_div(a, b, div), Some(reference)),
                Err(_) => assert_eq!(i128::try_full_mul_div(a, b, div), None),
            }
        });
    }

    /// Regression: a negative result with magnitude in `(2^127, 2^128)`
    /// previously wrapped through two's complement to a silently wrong value.
    #[test]
    fn i128_full_mul_div_negative_out_of_range() {
        assert_eq!(i128::try_full_mul_div(i128::MIN, 3, 2), None);
    }

    #[test]
    #[should_panic(expected = "Result out of range")]
    fn i128_full_mul_div_negative_out_of_range_panics() {
        i128::full_mul_div(i128::MIN, 3, 2);
    }

    /// The panic message contains all operands so that callers (and the
    /// humans or agents reading their logs) can identify the failing
    /// expression.
    #[test]
    #[should_panic(expected = "(9223372036854775807 * 100000000) / 1")]
    fn i64_full_mul_div_out_of_range_message_contains_operands() {
        i64::full_mul_div(i64::MAX, 100_000_000, 1);
    }

    #[test]
    #[should_panic(expected = "Division by zero; lhs=1; rhs=2")]
    fn i64_full_mul_div_division_by_zero_message_contains_operands() {
        i64::full_mul_div(1, 2, 0);
    }
}
