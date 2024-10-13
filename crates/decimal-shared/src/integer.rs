use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{AddAssign, DivAssign, Not, Shr};
use std::str::FromStr;

use num_traits::{CheckedNeg, CheckedRem, ConstOne, ConstZero, One, PrimInt, WrappingAdd};

pub trait BasicInteger:
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
{
}

impl<I> BasicInteger for I where
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
{
}

pub trait SignedBasicInteger: BasicInteger + CheckedNeg {}

impl<I> SignedBasicInteger for I where I: BasicInteger + CheckedNeg {}
