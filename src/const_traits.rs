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
