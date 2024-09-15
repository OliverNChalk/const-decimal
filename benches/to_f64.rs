use const_decimal::{Decimal, ScaledInteger};
use criterion::measurement::WallTime;
use criterion::{black_box, BatchSize, BenchmarkGroup};
use num_traits::PrimInt;
use prop::strategy::ValueTree;
use prop::test_runner::TestRunner;
use proptest::prelude::*;

pub fn bench_all<const D: u8, I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: ScaledInteger<D> + Arbitrary,
{
    bench_primitive_to_f64::<I>(group);
    bench_decimal_to_f64::<D, I>(group);
}

fn bench_primitive_to_f64<I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: PrimInt + Arbitrary,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input = (I::arbitrary(), I::arbitrary());

    group.bench_function("primitive/to_f64", |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a) + black_box(b)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_decimal_to_f64<const D: u8, I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: ScaledInteger<D> + Arbitrary,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input = I::arbitrary().prop_map(|a| Decimal::<_, D>(a));

    group.bench_function("decimal/to_f64", |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |a| black_box(a.to_f64()),
            BatchSize::SmallInput,
        )
    });
}
