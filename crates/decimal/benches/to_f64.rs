use criterion::measurement::WallTime;
use criterion::{black_box, BatchSize, BenchmarkGroup};
use decimal::{Decimal, ScalingFactor};
use decimal_shared::BasicInteger;
use prop::strategy::ValueTree;
use prop::test_runner::TestRunner;
use proptest::prelude::*;

pub fn bench_all<I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: BasicInteger + ScalingFactor + Arbitrary,
{
    bench_primitive_to_f64::<I>(group);
    bench_decimal_to_f64::<I>(group);
}

fn bench_primitive_to_f64<I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: BasicInteger + Arbitrary,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input = I::arbitrary();

    group.bench_function("primitive/to_f64", |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |a| black_box(black_box(a).to_f64()),
            BatchSize::SmallInput,
        )
    });
}

fn bench_decimal_to_f64<I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: BasicInteger + ScalingFactor + Arbitrary,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input =
        (I::arbitrary(), 0..18u8).prop_map(|(integer, decimals)| Decimal::new(integer, decimals));

    group.bench_function("decimal/to_f64", |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |a| black_box(a.to_f64()),
            BatchSize::SmallInput,
        )
    });
}
