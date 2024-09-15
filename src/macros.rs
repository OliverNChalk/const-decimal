macro_rules! apply_to_common_variants {
    ($macro:ident) => {
        $macro!(u8, 1);
        $macro!(i8, 1);
        $macro!(u16, 2);
        $macro!(i16, 2);
        $macro!(u32, 5);
        $macro!(i32, 5);
        $macro!(u64, 9);
        $macro!(i64, 9);
        $macro!(u128, 18);
        $macro!(i128, 18);
    };
}

#[cfg(feature = "borsh")]
macro_rules! generate_tests_for_common_variants {
    ($f:ident) => {
        crate::macros::generate_test!($f, u8, 1);
        crate::macros::generate_test!($f, i8, 1);
        crate::macros::generate_test!($f, u16, 2);
        crate::macros::generate_test!($f, i16, 2);
        crate::macros::generate_test!($f, u32, 5);
        crate::macros::generate_test!($f, i32, 5);
        crate::macros::generate_test!($f, u64, 9);
        crate::macros::generate_test!($f, i64, 9);
        crate::macros::generate_test!($f, u128, 18);
        crate::macros::generate_test!($f, i128, 18);
    };
}

#[cfg(feature = "borsh")]
macro_rules! generate_test {
    ($f:ident, $underlying:ty, $decimals:literal) => {
        ::paste::paste! {
            #[test]
            fn [<$f _ $underlying _ $decimals >]() {
                $f::<$underlying, $decimals>();
            }
        }
    };
}

pub(crate) use apply_to_common_variants;
#[cfg(feature = "borsh")]
pub(crate) use {generate_test, generate_tests_for_common_variants};
