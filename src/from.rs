use crate::{Decimal, Int128_18, Int64_9, Uint128_18, Uint64_9};

// TODO: Implement From generically where the result cannot overflow.
// TODO: Implement TryFrom generically where the result can overflow.

impl From<Uint64_9> for Uint128_18 {
    fn from(value: Uint64_9) -> Self {
        Decimal((value.0 as u128) * 10u128.pow(9))
    }
}

impl From<Int64_9> for Int128_18 {
    fn from(value: Int64_9) -> Self {
        Decimal((value.0 as i128) * 10i128.pow(9))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use proptest::prelude::Arbitrary;
    use proptest::test_runner::TestRunner;

    use super::*;

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
}
