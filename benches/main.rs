use std::ops::{Div, Neg};

use const_decimal::ScaledInteger;
use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use num_traits::ConstOne;
use proptest::prelude::{Arbitrary, Strategy};

mod add;
mod div;
mod mul;
mod sub;
mod to_f64;

fn to_sign<I>(positive: bool) -> I
where
    I: ConstOne + Neg<Output = I>,
{
    match positive {
        true => I::ONE,
        false => -I::ONE,
    }
}

fn main() {
    let mut criterion = criterion::Criterion::default().configure_from_args();

    // TODO: Our `hi` range for signed integers does not sample negative values.

    bench_integers::<9, u64>(
        &mut criterion.benchmark_group("u64_9"),
        0..u64::from(u32::MAX),
        // TODO: This upper bound is lower than necessary.
        (u64::from(u32::MAX) - 10u64.pow(9) + 1)..(u64::MAX / 10u64.pow(9)),
        (u64::MAX / 10u64.pow(9) + 1)..u64::MAX,
    );
    bench_integers::<9, i64>(
        &mut criterion.benchmark_group("i64_9"),
        i64::from(i32::MIN)..i64::from(i32::MAX),
        (bool::arbitrary(), (i64::from(u32::MAX) + 1)..(i64::MAX / 10i64.pow(9)))
            .prop_map(|(sign, unsigned)| to_sign::<i64>(sign) * unsigned),
        (bool::arbitrary(), (i64::MAX / 10i64.pow(9) + 1)..i64::MAX)
            .prop_map(|(sign, unsigned)| to_sign::<i64>(sign) * unsigned),
    );
    bench_integers::<18, u128>(
        &mut criterion.benchmark_group("u128_18"),
        0..u128::from(u64::MAX),
        // TODO: This upper bound is lower than necessary.
        (u128::from(u64::MAX) - 10u128.pow(18) + 1)..(u128::MAX / 10u128.pow(18)),
        (u128::MAX / 10u128.pow(18) + 1)..u128::MAX,
    );
    bench_integers::<18, i128>(
        &mut criterion.benchmark_group("i128_18"),
        i128::from(i64::MIN)..i128::from(i64::MAX),
        (bool::arbitrary(), (i128::from(u64::MAX) + 1)..(i128::MAX / 10i128.pow(18)))
            .prop_map(|(sign, unsigned)| to_sign::<i128>(sign) * unsigned),
        (bool::arbitrary(), (i128::MAX / 10i128.pow(18) + 1)..i128::MAX)
            .prop_map(|(sign, unsigned)| to_sign::<i128>(sign) * unsigned),
    );

    criterion.final_summary();
}

fn bench_integers<const D: u8, I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    lo_range: impl Strategy<Value = I> + Clone,
    hi_mul_range: impl Strategy<Value = I> + Clone,
    hi_div_range: impl Strategy<Value = I> + Clone,
) where
    I: ScaledInteger<D> + Arbitrary + Div<Output = I>,
{
    add::bench_all::<D, I>(group);
    sub::bench_all::<D, I>(group);
    mul::bench_all::<D, I>(group, lo_range.clone(), hi_mul_range);
    div::bench_all::<D, I>(group, lo_range, hi_div_range);
    to_f64::bench_all::<D, I>(group);
}
