use const_decimal::{Decimal, Integer, Primitive};
use criterion::measurement::WallTime;
use criterion::{black_box, BatchSize, BenchmarkGroup};
use prop::strategy::ValueTree;
use prop::test_runner::TestRunner;
use proptest::prelude::*;

pub fn bench_all<const D: u8, I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: Integer<D> + Arbitrary,
{
    bench_primitive_add::<I>(group);
    bench_decimal_add::<D, I>(group);
}

fn bench_primitive_add<I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: Primitive + Arbitrary,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input = (I::arbitrary(), I::arbitrary());

    group.bench_function("primitive/add", |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a) + black_box(b)),
            BatchSize::SmallInput,
        )
    });
}

fn bench_decimal_add<const D: u8, I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: Integer<D> + Arbitrary,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input = (I::arbitrary(), I::arbitrary())
        .prop_map(|(a, b)| (Decimal::<_, D>(a), Decimal::<_, D>(b)));

    group.bench_function("decimal/add", |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a) + black_box(b)),
            BatchSize::SmallInput,
        )
    });
}
