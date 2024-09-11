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
