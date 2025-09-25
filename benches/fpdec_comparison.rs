//! Compare `const-decimal` with [`fpdec.rs`](https://github.com/mamrhein/fpdec.rs)

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fpdec::{Dec, Decimal};

// Lots of black boxes, maybe overkill.
fn criterion_benchmark(c: &mut Criterion) {
    // Create instances here to avoid benchmarking the constructor.
    let fpdec_0 = fpdec::Decimal::new_raw(100, 0);
    let fpdec_1 = fpdec::Decimal::new_raw(110, 0);

    let const_decimal_i32_0 = const_decimal::Decimal::<i32, 4>::try_from_scaled(100, 0).unwrap();
    let const_decimal_i32_1 = const_decimal::Decimal::<i32, 4>::try_from_scaled(110, 0).unwrap();

    let const_decimal_i64_0 = const_decimal::Decimal::<i64, 4>::try_from_scaled(100, 0).unwrap();
    let const_decimal_i64_1 = const_decimal::Decimal::<i64, 4>::try_from_scaled(110, 0).unwrap();

    c.bench_function("fpdec_add", |b| {
        b.iter(|| {
            let _ = black_box(black_box(fpdec_0) + black_box(fpdec_1));
        });
    });
    c.bench_function("const_decimal_i32_add", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i32_0) + black_box(const_decimal_i32_1));
        });
    });
    c.bench_function("const_decimal_i64_add", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i64_0) + black_box(const_decimal_i64_1));
        });
    });

    c.bench_function("fpdec_sub", |b| {
        b.iter(|| {
            let _ = black_box(black_box(fpdec_0) - black_box(fpdec_1));
        });
    });
    c.bench_function("const_decimal_i32_sub", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i32_0) - black_box(const_decimal_i32_1));
        });
    });
    c.bench_function("const_decimal_i64_sub", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i64_0) - black_box(const_decimal_i64_1));
        });
    });

    c.bench_function("fpdec_mul", |b| {
        b.iter(|| {
            let _ = black_box(black_box(fpdec_0) * black_box(fpdec_1));
        });
    });
    c.bench_function("const_decimal_i32_mul", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i32_0) * black_box(const_decimal_i32_1));
        });
    });
    c.bench_function("const_decimal_i64_mul", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i64_0) * black_box(const_decimal_i64_1));
        });
    });

    c.bench_function("fpdec_div", |b| {
        b.iter(|| {
            let _ = black_box(black_box(fpdec_0) / black_box(fpdec_1));
        });
    });
    c.bench_function("const_decimal_i32_div", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i32_0) / black_box(const_decimal_i32_1));
        });
    });
    c.bench_function("const_decimal_i64_div", |b| {
        b.iter(|| {
            black_box(black_box(const_decimal_i64_0) / black_box(const_decimal_i64_1));
        });
    });

    // Some real-world use case examples. Linear futures profit and loss
    // calculations. Quantity denoted in BaseCurrency.
    let fpdec_entry_price = Dec!(100);
    let fpdec_exit_price = Dec!(110);
    let fpdec_qty = Dec!(5);

    let const_decimal_i32_entry_price =
        const_decimal::Decimal::<i32, 2>::try_from_scaled(100, 0).unwrap();
    let const_decimal_i32_exit_price =
        const_decimal::Decimal::<i32, 2>::try_from_scaled(110, 0).unwrap();
    let const_decimal_i32_qty = const_decimal::Decimal::<i32, 2>::try_from_scaled(5, 0).unwrap();

    let const_decimal_i64_entry_price =
        const_decimal::Decimal::<i64, 2>::try_from_scaled(100, 0).unwrap();
    let const_decimal_i64_exit_price =
        const_decimal::Decimal::<i64, 2>::try_from_scaled(110, 0).unwrap();
    let const_decimal_i64_qty = const_decimal::Decimal::<i64, 2>::try_from_scaled(5, 0).unwrap();

    c.bench_function("fpdec_linear_futures_pnl", |b| {
        b.iter(|| {
            let _ = black_box(fpdec_exit_price * fpdec_qty - fpdec_entry_price * fpdec_qty);
        });
    });
    c.bench_function("const_decimal_i32_linear_futures_pnl", |b| {
        b.iter(|| {
            black_box(
                const_decimal_i32_exit_price * const_decimal_i32_qty
                    - const_decimal_i32_entry_price * const_decimal_i32_qty,
            );
        });
    });
    c.bench_function("const_decimal_i64_linear_futures_pnl", |b| {
        b.iter(|| {
            black_box(
                const_decimal_i64_exit_price * const_decimal_i64_qty
                    - const_decimal_i64_entry_price * const_decimal_i64_qty,
            );
        });
    });

    // Inverse futures profit and loss calculation. Quantity is denoted in
    // QuoteCurrency.
    let fpdec_qty = Dec!(500);
    let const_decimal_i32_qty = const_decimal::Decimal::<i32, 2>::try_from_scaled(500, 0).unwrap();
    let const_decimal_i64_qty = const_decimal::Decimal::<i64, 2>::try_from_scaled(500, 0).unwrap();

    c.bench_function("fpdec_inverse_futures_pnl", |b| {
        b.iter(|| {
            let _ = black_box(
                black_box(fpdec_qty) / black_box(fpdec_entry_price)
                    - black_box(fpdec_qty) / black_box(fpdec_exit_price),
            );
        });
    });
    c.bench_function("const_decimal_i32_inverse_futures_pnl", |b| {
        b.iter(|| {
            black_box(
                black_box(const_decimal_i32_qty) / black_box(const_decimal_i32_entry_price)
                    - black_box(const_decimal_i32_qty) / black_box(const_decimal_i32_exit_price),
            );
        });
    });
    c.bench_function("const_decimal_i64_inverse_futures_pnl", |b| {
        b.iter(|| {
            black_box(
                black_box(const_decimal_i64_qty) / black_box(const_decimal_i64_entry_price)
                    - black_box(const_decimal_i64_qty) / black_box(const_decimal_i64_exit_price),
            );
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
