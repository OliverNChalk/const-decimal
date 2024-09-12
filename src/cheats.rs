pub trait Cheats<const D: u8> {
    const TWO: Self;
    const TEN: Self;
    const SCALING_FACTOR: Self;
}

impl<const D: u8> Cheats<D> for u8 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10u8.pow(D as u32);
}

impl<const D: u8> Cheats<D> for i8 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10i8.pow(D as u32);
}

impl<const D: u8> Cheats<D> for u16 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10u16.pow(D as u32);
}

impl<const D: u8> Cheats<D> for i16 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10i16.pow(D as u32);
}

impl<const D: u8> Cheats<D> for u32 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10u32.pow(D as u32);
}

impl<const D: u8> Cheats<D> for i32 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10i32.pow(D as u32);
}

impl<const D: u8> Cheats<D> for u64 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10u64.pow(D as u32);
}

impl<const D: u8> Cheats<D> for i64 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10i64.pow(D as u32);
}

impl<const D: u8> Cheats<D> for u128 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10u128.pow(D as u32);
}

impl<const D: u8> Cheats<D> for i128 {
    const TWO: Self = 2;
    const TEN: Self = 10;
    const SCALING_FACTOR: Self = 10i128.pow(D as u32);
}
