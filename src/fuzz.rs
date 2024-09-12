use std::ops::Shr;

use paste::paste;
use proptest::prelude::*;

use super::*;

// TODO:
//
// - Is there a reasonable way to fuzz test the full mul/div range (i.e. check
//   overflow handling?). It's tricky because Decimal uses MulDiv internally
//   which does a full multiplication then division.
// - Actually test the negative range.

macro_rules! fuzz_against_primitive {
    ($primitive:tt, $decimals:literal) => {
        paste! {
            proptest! {
                /// Addition functions the same as regular unsigned integer addition.
                #[test]
                fn [<$primitive _ $decimals _add>](
                    x in 0..$primitive::MAX,
                    y in 0..$primitive::MAX,
                ) {
                    let decimal = std::panic::catch_unwind(
                        || Decimal::<_, $decimals>(x) + Decimal(y)
                    );
                    let primitive = std::panic::catch_unwind(|| x + y);

                    match (decimal, primitive) {
                        (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                        (Err(_), Err(_)) => {}
                        (decimal, primitive) => panic!(
                            "Mismatch; decimal={decimal:?}; primitive={primitive:?}"
                        )
                    }
                }

                /// Subtraction functions the same as regular unsigned integer addition.
                #[test]
                fn [<$primitive _ $decimals _sub>](
                    x in 0..$primitive::MAX,
                    y in 0..$primitive::MAX,
                ) {
                    let decimal = std::panic::catch_unwind(
                        || Decimal::<_, $decimals>(x) - Decimal(y)
                    );
                    let primitive = std::panic::catch_unwind(|| x - y);

                    match (decimal, primitive) {
                        (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                        (Err(_), Err(_)) => {}
                        (decimal, primitive) => panic!(
                            "Mismatch; decimal={decimal:?}; primitive={primitive:?}",
                        )
                    }
                }

                /// Multiplication requires the result to be divided by the scaling factor.
                #[test]
                fn [<$primitive _ $decimals _mul>](
                    // TODO: Could limit shifting to SCALING_FACTOR BITS?
                    x in 0..($primitive::MAX.shr($primitive::BITS / 2)),
                    y in 0..($primitive::MAX.shr($primitive::BITS / 2)),
                ) {
                    let decimal = std::panic::catch_unwind(
                        || Decimal::<_, $decimals>(x) * Decimal(y)
                    );
                    let primitive = std::panic::catch_unwind(
                        || x * y / $primitive::pow(10, $decimals)
                    );

                    match (decimal, primitive) {
                        (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                        (Err(_), Err(_)) => {}
                        (decimal, primitive) => panic!(
                            "Mismatch; decimal={decimal:?}; primitive={primitive:?}"
                        )
                    }
                }

                /// Division requires the numerator to first be scaled by the scaling factor.
                #[test]
                fn [<$primitive _ $decimals _div>](
                    x in 0..($primitive::MAX / $primitive::pow(10, $decimals)),
                    y in 0..($primitive::MAX / $primitive::pow(10, $decimals)),
                ) {
                    let decimal = std::panic::catch_unwind(
                        || Decimal::<_, $decimals>(x) / Decimal(y)
                    );
                    let primitive = std::panic::catch_unwind(
                        || x * $primitive::pow(10, $decimals) / y
                    );

                    match (decimal, primitive) {
                        (Ok(decimal), Ok(primitive)) => assert_eq!(decimal.0, primitive),
                        (Err(_), Err(_)) => {}
                        (decimal, primitive) => panic!(
                            "Mismatch; decimal={decimal:?}; primitive={primitive:?}"
                        )
                    }
                }
            }
        }
    };
}

fuzz_against_primitive!(u8, 1);
fuzz_against_primitive!(i8, 1);
fuzz_against_primitive!(u16, 2);
fuzz_against_primitive!(i16, 2);
fuzz_against_primitive!(u32, 5);
fuzz_against_primitive!(i32, 5);
fuzz_against_primitive!(u64, 9);
fuzz_against_primitive!(i64, 9);
fuzz_against_primitive!(u128, 18);
fuzz_against_primitive!(i128, 18);
