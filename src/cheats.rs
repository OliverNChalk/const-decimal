use crate::{Int128_18, Int64_9, Uint128_18, Uint64_9};

pub trait PrecisionFactor<const D: u8> {
    const PRECISION_FACTOR: Self;
}

impl<const D: u8> PrecisionFactor<D> for u64 {
    const PRECISION_FACTOR: Self = 10u64.pow(D as u32);
}

impl<const D: u8> PrecisionFactor<D> for i64 {
    const PRECISION_FACTOR: Self = 10i64.pow(D as u32);
}

impl<const D: u8> PrecisionFactor<D> for u128 {
    const PRECISION_FACTOR: Self = 10u128.pow(D as u32);
}

impl<const D: u8> PrecisionFactor<D> for i128 {
    const PRECISION_FACTOR: Self = 10i128.pow(D as u32);
}

impl From<Uint64_9> for Uint128_18 {
    fn from(value: Uint64_9) -> Self {
        todo!("Cast value then shift by 1e9")
    }
}

impl From<Int64_9> for Int128_18 {
    fn from(value: Int64_9) -> Self {
        todo!("Cast value then shift by 1e9")
    }
}
