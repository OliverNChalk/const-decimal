use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Not, Shr};
use std::str::FromStr;

use num_traits::{CheckedNeg, CheckedRem, ConstOne, ConstZero, One, PrimInt, WrappingAdd};

use crate::cheats::Cheats;
use crate::full_mul_div::FullMulDiv;

pub trait Integer<const D: u8>:
    PrimInt
    + ConstZero
    + ConstOne
    + One
    + WrappingAdd<Output = Self>
    + CheckedRem<Output = Self>
    + Not<Output = Self>
    + Shr<u32, Output = Self>
    + Display
    + FromStr<Err = ParseIntError>
    + Cheats<D>
    + FullMulDiv
{
}

impl<I, const D: u8> Integer<D> for I where
    I: PrimInt
        + ConstZero
        + ConstOne
        + One
        + WrappingAdd<Output = Self>
        + CheckedRem<Output = Self>
        + Not<Output = Self>
        + Shr<u32, Output = Self>
        + Display
        + FromStr<Err = ParseIntError>
        + Cheats<D>
        + FullMulDiv
{
}

pub trait SignedInteger<const D: u8>: Integer<D> + CheckedNeg {}

impl<I, const D: u8> SignedInteger<D> for I where I: Integer<D> + CheckedNeg {}
