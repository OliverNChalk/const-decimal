use crate::{Decimal, Int128_18, Int64_9, ScaledInteger, Uint128_18, Uint64_9};

// TODO: Implement From generically where the result cannot overflow.
// TODO: Implement TryFrom generically where the result can overflow.

impl From<Uint64_9> for Uint128_18 {
    fn from(value: Uint64_9) -> Self {
        // We know this multiplication can never overflow.
        #[allow(clippy::arithmetic_side_effects)]
        Decimal((u128::from(value.0)) * 10u128.pow(9))
    }
}

impl From<Int64_9> for Int128_18 {
    fn from(value: Int64_9) -> Self {
        // We know this multiplication can never overflow.
        #[allow(clippy::arithmetic_side_effects)]
        Decimal((i128::from(value.0)) * 10i128.pow(9))
    }
}

impl<I, const D: u8> Decimal<I, D>
where
    I: ScaledInteger<D>,
{
    // SAFETY: `num_traits::to_f64` does not panic on primitive types.
    #[allow(clippy::missing_panics_doc)]
    pub fn to_f64(&self) -> f64 {
        self.0.to_f64().unwrap() / I::SCALING_FACTOR.to_f64().unwrap()
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn to_f32(&self) -> f32 {
        self.0.to_f32().unwrap() / I::SCALING_FACTOR.to_f32().unwrap()
    }
}

#[allow(clippy::float_cmp)]
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proptest::prelude::Arbitrary;
    use proptest::proptest;
    use proptest::test_runner::TestRunner;

    use super::*;
    use crate::macros::generate_tests_for_common_variants;

    #[test]
    fn uint128_18_from_uint64_9() {
        let mut runner = TestRunner::default();
        let input = Decimal::arbitrary();

        runner
            .run(&input, |decimal: Decimal<u64, 9>| {
                let out = Uint128_18::from(decimal);
                let out_f = f64::from_str(&out.to_string()).unwrap();
                let decimal_f = f64::from_str(&decimal.to_string()).unwrap();

                assert_eq!(out_f, decimal_f);

                Ok(())
            })
            .unwrap();
    }

    #[test]
    fn int128_18_from_int64_9() {
        let mut runner = TestRunner::default();
        let input = Decimal::arbitrary();

        runner
            .run(&input, |decimal: Decimal<i64, 9>| {
                let out = Int128_18::from(decimal);
                let out_f = f64::from_str(&out.to_string()).unwrap();
                let decimal_f = f64::from_str(&decimal.to_string()).unwrap();

                assert_eq!(out_f, decimal_f);

                Ok(())
            })
            .unwrap();
    }

    generate_tests_for_common_variants!(to_f64_does_not_panic);

    fn to_f64_does_not_panic<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary,
    {
        proptest!(|(a: Decimal<I, D>)| {
            a.to_f64();
        });
    }
}
