use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{AddAssign, DivAssign, Not, Shr};
use std::str::FromStr;

use num_traits::{CheckedNeg, CheckedRem, ConstOne, ConstZero, One, PrimInt, WrappingAdd};

use crate::cheats::Cheats;
use crate::full_mul_div::FullMulDiv;

pub trait ScaledInteger<const D: u8>:
    PrimInt
    + ConstZero
    + ConstOne
    + One
    + WrappingAdd<Output = Self>
    + CheckedRem<Output = Self>
    + Not<Output = Self>
    + Shr<u32, Output = Self>
    + AddAssign
    + DivAssign
    + Display
    + FromStr<Err = ParseIntError>
    + Cheats<D>
    + FullMulDiv
{
}

impl<I, const D: u8> ScaledInteger<D> for I where
    I: PrimInt
        + ConstZero
        + ConstOne
        + One
        + WrappingAdd<Output = Self>
        + CheckedRem<Output = Self>
        + Not<Output = Self>
        + Shr<u32, Output = Self>
        + AddAssign
        + DivAssign
        + Display
        + FromStr<Err = ParseIntError>
        + Cheats<D>
        + FullMulDiv
{
}

pub trait SignedScaledInteger<const D: u8>: ScaledInteger<D> + CheckedNeg {}

impl<I, const D: u8> SignedScaledInteger<D> for I where I: ScaledInteger<D> + CheckedNeg {}
