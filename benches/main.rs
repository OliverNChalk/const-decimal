use std::fmt::Debug;
use std::ops::Div;

use const_decimal::Integer;
use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use proptest::prelude::{Arbitrary, Strategy};

mod add;
mod div;
mod mul;
mod sub;

fn main() {
    let mut criterion = criterion::Criterion::default().configure_from_args();

    bench_integers::<9, u64>(&mut criterion.benchmark_group("u64_9"), 0..(u32::MAX as u64));
    bench_integers::<9, i64>(
        &mut criterion.benchmark_group("i64_9"),
        (i32::MIN as i64)..(i32::MAX as i64),
    );
    bench_integers::<18, u128>(&mut criterion.benchmark_group("u128_18"), 0..(u64::MAX as u128));
    bench_integers::<18, i128>(
        &mut criterion.benchmark_group("i128_18"),
        (i64::MIN as i128)..(i64::MAX as i128),
    );

    criterion.final_summary();
}

fn bench_integers<const D: u8, I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    mul_div_range: impl Strategy<Value = I> + Clone + Debug,
) where
    I: Integer<D> + Arbitrary + Div<Output = I>,
{
    add::bench_all::<D, I>(group);
    sub::bench_all::<D, I>(group);
    mul::bench_all::<D, I>(group, mul_div_range.clone());
    div::bench_all::<D, I>(group, mul_div_range);
}
