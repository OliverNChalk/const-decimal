use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Not, Shr};
use std::str::FromStr;

use num_traits::{
    Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedNeg, CheckedRem, CheckedSub, ConstOne,
    ConstZero, One, Pow, WrappingAdd,
};

use crate::cheats::Cheats;
use crate::full_mul_div::FullMulDiv;

pub trait Primitive:
    // `num-traits`
    ConstZero
    + ConstOne
    + One
    + Bounded
    // `std`
    + CheckedAdd<Output = Self>
    + WrappingAdd<Output = Self>
    + CheckedSub<Output = Self>
    + CheckedMul<Output = Self>
    + CheckedDiv<Output = Self>
    + CheckedRem<Output = Self>
    + Not<Output = Self>
    + Pow<usize, Output = Self>
    + Shr<u32, Output = Self>
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Display
    + FromStr<Err = ParseIntError>
{
}

impl<T> Primitive for T where
    T: ConstZero
        + ConstOne
        + One
        + Bounded
        + CheckedAdd<Output = Self>
        + WrappingAdd<Output = Self>
        + CheckedSub<Output = Self>
        + CheckedMul<Output = Self>
        + CheckedDiv<Output = Self>
        + CheckedRem<Output = Self>
        + Not<Output = Self>
        + Pow<usize, Output = Self>
        + Shr<u32, Output = Self>
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Display
        + FromStr<Err = ParseIntError>
{
}

pub trait Integer<const D: u8>: Cheats<D> + FullMulDiv + Primitive {}

impl<I, const D: u8> Integer<D> for I where I: Cheats<D> + FullMulDiv + Primitive {}

pub trait SignedInteger<const D: u8>: Integer<D> + CheckedNeg {}

impl<I, const D: u8> SignedInteger<D> for I where I: Integer<D> + CheckedNeg {}
