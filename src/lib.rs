/// Some balanced [`Decimal`] variants.
mod aliases;
/// Stuff that should be done generically if I had more time.
mod cheats;
/// Casts between cost-decimals.
mod conversion;
/// Core decimal type & operations.
mod decimal;
/// [`Display`] and [`FromStr`] implementation.
mod display;
/// Implementations of foreign traits.
mod foreign_traits;
/// Full multiplication implementations for underlying integers.
mod full_mul_div;
/// Trait definition for underlying integer.
mod integer;
/// Macros used in tests.
#[cfg(test)]
#[macro_use]
pub(crate) mod macros;
/// Underlying algorithms used by other operations.
#[cfg(test)]
pub(crate) mod algorithms;

pub use aliases::*;
pub use decimal::*;
pub use integer::*;
