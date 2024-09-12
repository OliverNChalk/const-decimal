use std::fmt::Debug;
use std::ops::{Div, Neg};

use const_decimal::Integer;
use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use num_traits::ConstOne;
use proptest::prelude::{Arbitrary, Strategy};

mod add;
mod div;
mod mul;
mod sub;

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
        0..(u32::MAX as u64),
        ((u32::MAX as u64) - 10u64.pow(9) + 1)..(u64::MAX / 10u64.pow(9)),
        (u64::MAX / 10u64.pow(9) + 1)..u64::MAX,
    );
    bench_integers::<9, i64>(
        &mut criterion.benchmark_group("i64_9"),
        (i32::MIN as i64)..(i32::MAX as i64),
        (bool::arbitrary(), ((u32::MAX as i64) + 1)..(i64::MAX / 10i64.pow(9)))
            .prop_map(|(sign, unsigned)| to_sign::<i64>(sign) * unsigned),
        (bool::arbitrary(), (i64::MAX / 10i64.pow(9) + 1)..i64::MAX)
            .prop_map(|(sign, unsigned)| to_sign::<i64>(sign) * unsigned),
    );
    bench_integers::<18, u128>(
        &mut criterion.benchmark_group("u128_18"),
        0..(u64::MAX as u128),
        ((u64::MAX as u128) - 10u128.pow(18) + 1)..(u128::MAX / 10u128.pow(18)),
        (u128::MAX / 10u128.pow(18) + 1)..u128::MAX,
    );
    bench_integers::<18, i128>(
        &mut criterion.benchmark_group("i128_18"),
        (i64::MIN as i128)..(i64::MAX as i128),
        (bool::arbitrary(), ((u64::MAX as i128) + 1)..(i128::MAX / 10i128.pow(18)))
            .prop_map(|(sign, unsigned)| to_sign::<i128>(sign) * unsigned),
        (bool::arbitrary(), (i128::MAX / 10i128.pow(18) + 1)..i128::MAX)
            .prop_map(|(sign, unsigned)| to_sign::<i128>(sign) * unsigned),
    );

    criterion.final_summary();
}

fn bench_integers<const D: u8, I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    lo_range: impl Strategy<Value = I> + Clone + Debug,
    hi_mul_range: impl Strategy<Value = I> + Clone + Debug,
    hi_div_range: impl Strategy<Value = I> + Clone + Debug,
) where
    I: Integer<D> + Arbitrary + Div<Output = I>,
{
    add::bench_all::<D, I>(group);
    sub::bench_all::<D, I>(group);
    mul::bench_all::<D, I>(group, lo_range.clone(), hi_mul_range);
    div::bench_all::<D, I>(group, lo_range, hi_div_range);
}
