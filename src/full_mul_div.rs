use ruint::aliases::U256;
use ruint::Uint;

pub trait FullMulDiv {
    /// Implements `a * b / c` with full width on the intermediate `a * b`
    /// state.
    fn full_mul_div(self, rhs: Self, div: Self) -> Self;
}

macro_rules! impl_primitive {
    ($primary:ty, $intermediate:ty) => {
        impl FullMulDiv for $primary {
            fn full_mul_div(self, rhs: Self, div: Self) -> Self {
                let numer = (<$intermediate>::from(self))
                    .checked_mul(<$intermediate>::from(rhs))
                    .unwrap_or_else(|| panic!("Mul overflowed; lhs={self}; rhs={rhs}"));
                let denom = <$intermediate>::from(div);
                let out = numer
                    .checked_div(denom)
                    .unwrap_or_else(|| panic!("Division by zero; numer={numer}; denom={denom}"));

                out.try_into()
                    .unwrap_or_else(|err| panic!("Cast failed; err={err}"))
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
    fn full_mul_div(self, rhs: Self, div: Self) -> Self {
        let out: U256 = Uint::from(self)
            .checked_mul(Uint::from(rhs))
            .unwrap()
            .checked_div(Uint::from(div))
            .unwrap();

        out.try_into().unwrap()
    }
}

impl FullMulDiv for i128 {
    fn full_mul_div(self, rhs: Self, div: Self) -> Self {
        // If we can compute the output using only an i128, then we should.
        if let Some(out) = self
            .checked_mul(rhs)
            // NB: Panic early on division by 0.
            .map(|numer| numer.checked_div(div).unwrap())
        {
            return out;
        }

        // Determine the sign of the output. Signum returns -1, 0, +1, therefore
        // overflow is not possible. Additionally, if `self` or `rhs` are zero then we
        // will have already returned. If `div` is zero then our next checked_div will
        // catch this (and panic).
        #[allow(clippy::arithmetic_side_effects)]
        let sign = self.signum() * rhs.signum() * div.signum();

        // Get the unsigned u256 representation of the integer (we'll later recover the
        // signed representation using two's complement).
        let this = U256::from(self.unsigned_abs());
        let rhs = U256::from(rhs.unsigned_abs());
        let div = U256::from(div.unsigned_abs());

        // Compute the unsigned output.
        let unsigned = this.checked_mul(rhs).unwrap().checked_div(div).unwrap();

        // Convert back to the signed output.
        match sign {
            1 => i128::try_from(unsigned).unwrap(),
            -1 => {
                // Take two's complement (!unsigned + 1).
                // https://en.wikipedia.org/wiki/Two%27s_complement.
                let unsigned = u128::try_from(unsigned).unwrap();
                let twos_complement = (!unsigned).overflowing_add(1).0;

                i128::from_le_bytes(twos_complement.to_le_bytes())
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
            match u128::try_from(&reference) {
                Ok(reference) => assert_eq!(u128::full_mul_div(a, b, div), reference),
                Err(_) => {}
            };
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
            match i128::try_from(&reference) {
                Ok(reference) => assert_eq!(i128::full_mul_div(a, b, div), reference),
                Err(_) => {}
            };
        });
    }
}
