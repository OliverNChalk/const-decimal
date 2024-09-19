mod decimal;
mod scaling_factor;

pub use decimal::*;
pub use scaling_factor::*;

#[inline(never)]
pub fn temp_scaling_factor_u64(decimals: u8) -> u64 {
    u64::scaling_factor(decimals)
}
