use std::fmt::Debug;
use std::ops::Div;

use const_decimal::{Decimal, Integer, Primitive};
use criterion::measurement::WallTime;
use criterion::{black_box, BatchSize, BenchmarkGroup};
use prop::strategy::ValueTree;
use prop::test_runner::TestRunner;
use proptest::prelude::*;

pub fn bench_all<const D: u8, I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    strategy: impl Strategy<Value = I> + Clone,
) where
    I: Integer<D> + Debug + Div<Output = I>,
{
    primitive_div::<I>(group, strategy.clone());
    decimal_div::<D, I>(group, strategy);
}

fn primitive_div<I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    strategy: impl Strategy<Value = I> + Clone,
) where
    I: Primitive + Div<Output = I>,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input = (strategy.clone(), strategy);

    group.bench_function("primitive/div", |bencher| {
        bencher.iter_batched(
            || {
                let (numer, denom) = input.new_tree(&mut runner).unwrap().current();
                // Avoid division by zero.
                let denom = std::cmp::max(denom, I::ONE);

                (numer, denom)
            },
            |(a, b)| black_box(black_box(a) / black_box(b)),
            BatchSize::SmallInput,
        )
    });
}

// TODO: Split into lo/hi range (based on whether the intermediate result fits
// in one word).
fn decimal_div<const D: u8, I>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    strategy: impl Strategy<Value = I> + Clone,
) where
    I: Integer<D> + Debug,
{
    // Use proptest to generate arbitrary input values.
    let mut runner = TestRunner::deterministic();
    let input =
        (strategy.clone(), strategy).prop_map(|(a, b)| (Decimal::<_, D>(a), Decimal::<_, D>(b)));

    group.bench_function("decimal/div", |bencher| {
        bencher.iter_batched(
            || input.new_tree(&mut runner).unwrap().current(),
            |(a, b)| black_box(black_box(a) / black_box(b)),
            BatchSize::SmallInput,
        )
    });
}
