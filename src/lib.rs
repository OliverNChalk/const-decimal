use std::ops::{Add, Div, Mul, Sub};

pub const PRECISION_FACTOR: u64 = 10u64.pow(9);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Decimal64(pub u64);

impl Decimal64 {
    pub const ONE: Decimal64 = Decimal64(PRECISION_FACTOR);
}

impl Add for Decimal64 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Decimal64(self.0 + rhs.0)
    }
}

impl Sub for Decimal64 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Decimal64(self.0 - rhs.0)
    }
}

impl Mul for Decimal64 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        Decimal64(self.0 * rhs.0 / PRECISION_FACTOR)
    }
}

impl Div for Decimal64 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        Decimal64(self.0 * PRECISION_FACTOR / rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Decimal64::ONE + Decimal64::ONE, Decimal64(PRECISION_FACTOR * 2));
    }

    #[test]
    fn sub() {
        assert_eq!(Decimal64::ONE - Decimal64::ONE, Decimal64(0));
    }

    #[test]
    fn mul() {
        assert_eq!(Decimal64::ONE * Decimal64::ONE, Decimal64::ONE);
    }

    #[test]
    fn div() {
        assert_eq!(Decimal64::ONE / Decimal64::ONE, Decimal64::ONE);
    }
}
