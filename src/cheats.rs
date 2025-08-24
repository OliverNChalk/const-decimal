use paste::paste;

pub trait Cheats<const D: u8> {
    const MIN: Self;
    const MAX: Self;
    const TWO: Self;
    const TEN: Self;
    const SCALING_FACTOR: Self;
    const TWO_SCALING_FACTOR: Self;
}

macro_rules! impl_primitive {
    ($primitive:tt) => {
        impl<const D: u8> Cheats<D> for $primitive {
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;
            const TWO: Self = 2;
            const TEN: Self = 10;
            paste! {
                const SCALING_FACTOR: Self = [<10 $primitive>].pow(D as u32);
                const TWO_SCALING_FACTOR: Self = 2 * [<10 $primitive>].pow(D as u32);
            }
        }
    };
}

impl_primitive!(u8);
impl_primitive!(i8);
impl_primitive!(u16);
impl_primitive!(i16);
impl_primitive!(u32);
impl_primitive!(i32);
impl_primitive!(u64);
impl_primitive!(i64);
impl_primitive!(u128);
impl_primitive!(i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaling_factor() {
        assert_eq!(<i32 as Cheats<0>>::SCALING_FACTOR, 1);
        assert_eq!(<i32 as Cheats<1>>::SCALING_FACTOR, 10);
        assert_eq!(<i32 as Cheats<2>>::SCALING_FACTOR, 100);
        assert_eq!(<i32 as Cheats<3>>::SCALING_FACTOR, 1000);

        assert_eq!(<i64 as Cheats<0>>::SCALING_FACTOR, 1);
        assert_eq!(<i64 as Cheats<1>>::SCALING_FACTOR, 10);
        assert_eq!(<i64 as Cheats<2>>::SCALING_FACTOR, 100);
        assert_eq!(<i64 as Cheats<3>>::SCALING_FACTOR, 1000);
    }
}
