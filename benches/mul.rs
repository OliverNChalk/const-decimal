use std::fmt::Debug;

use const_decimal::{Decimal, ScaledInteger};
use criterion::measurement::WallTime;
use criterion::{black_box, BatchSize, BenchmarkGroup};
use num_traits::PrimInt;
use prop::strategy::ValueTree;
use prop::test_runner::TestRunner;
use proptest::prelude::*;

pub fn bench_all<const D: u8, I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    lo_strategy: impl Strategy<Value = I> + Clone,
    hi_strategy: impl Strategy<Value = I> + Clone,
) where
    I: ScaledInteger<D> + Debug,
{
    primitive_mul::<I>(group, lo_strategy.clone(), "lo");
    decimal_mul::<D, I>(group, lo_strategy, "lo");
    primitive_mul::<I>(group, hi_strategy.clone(), "hi");
    decimal_mul::<D, I>(group, hi_strategy, "hi");
}

fn primitive_mul<I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    strategy: impl Strategy<Value = I> + Clone,
    strategy_label: &str,
) where
    I: PrimInt,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input = (strategy.clone(), strategy);

    group.bench_function(format!("primitive/mul/{strategy_label}"), |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a) * black_box(b)),
            BatchSize::SmallInput,
        );
    });
}

fn decimal_mul<const D: u8, I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    strategy: impl Strategy<Value = I> + Clone,
    strategy_label: &str,
) where
    I: ScaledInteger<D> + Debug,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input =
        (strategy.clone(), strategy).prop_map(|(a, b)| (Decimal::<_, D>(a), Decimal::<_, D>(b)));

    group.bench_function(format!("decimal/mul/{strategy_label}"), |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a) * black_box(b)),
            BatchSize::SmallInput,
        );
    });
}
