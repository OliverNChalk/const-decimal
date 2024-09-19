use decimal_shared::BasicInteger;

use crate::ScalingFactor;

#[derive(Debug)]
pub struct Decimal<I> {
    integer: I,
    decimals: u8,
}

impl<I> Decimal<I>
where
    I: BasicInteger + ScalingFactor,
{
    pub fn new(integer: I, decimals: u8) -> Self {
        Decimal { integer, decimals }
    }

    pub fn integer(&self) -> I {
        self.integer
    }

    pub fn decimals(&self) -> u8 {
        self.decimals
    }

    // SAFETY: `num_traits::to_f64` does not panic on primitive types.
    #[allow(clippy::missing_panics_doc)]
    pub fn to_f64(&self) -> f64 {
        self.integer.to_f64().unwrap() / I::scaling_factor(self.decimals).to_f64().unwrap()
    }
}
