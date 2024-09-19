use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use decimal::ScalingFactor;
use decimal_shared::BasicInteger;
use proptest::prelude::Arbitrary;

mod to_f64;

fn main() {
    let mut criterion = criterion::Criterion::default().configure_from_args();

    bench_integers::<u64>(&mut criterion.benchmark_group("u64"));
    bench_integers::<i64>(&mut criterion.benchmark_group("i64"));
    bench_integers::<u128>(&mut criterion.benchmark_group("u128"));
    bench_integers::<i128>(&mut criterion.benchmark_group("i128"));

    criterion.final_summary();
}

fn bench_integers<I>(group: &mut BenchmarkGroup<'_, WallTime>)
where
    I: BasicInteger + ScalingFactor + Arbitrary,
{
    to_f64::bench_all::<I>(group);
}
