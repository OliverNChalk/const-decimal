use std::fmt::Display;
use std::num::ParseIntError;
use std::ops::{Add, Div, Neg, Not, Rem, Sub};
use std::str::FromStr;

use num_traits::{Bounded, ConstOne, ConstZero, One, Pow, WrappingAdd};

use crate::cheats::Cheats;
use crate::full_mul_div::FullMulDiv;

pub trait Primitive:
    // `num-traits`
    ConstZero
    + ConstOne
    + One
    + Bounded
    // `std`
    + Add<Output = Self>
    + WrappingAdd<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Not<Output = Self>
    + Pow<usize, Output = Self>
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
        + Add<Output = Self>
        + WrappingAdd<Output = Self>
        + Sub<Output = Self>
        + Div<Output = Self>
        + Rem<Output = Self>
        + Not<Output = Self>
        + Pow<usize, Output = Self>
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

pub trait SignedInteger<const D: u8>: Integer<D> + Neg<Output = Self> {}

impl<I, const D: u8> SignedInteger<D> for I where I: Integer<D> + Neg<Output = Self> {}