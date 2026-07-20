# Changelog

All notable changes to `const-decimal`.

## Unreleased

## 0.4.0

- Update to Rust edition 2024 (requires Rust 1.85+).
- Add `fpdec_comparison` benchmark.
- Parse strings without a decimal point.
- Add `quantize_round_to_zero`.
- Impl `num_traits::Zero`, `num_traits::One`, and `std::ops::Rem`.
- Impl `num_traits::Num` and `num_traits::Signed`.
- Add `Decimal::to_f32(&self)`.
- Add `Decimal::MIN` and `Decimal::MAX`, deprecating `Decimal::min()` and
  `Decimal::max()`.
- Improve panic messages in arithmetic operations.
- Correctly format `Decimal::MIN`, e.g. `Decimal::<i8, 1>::MIN` as `-12.8`.

## 0.3.0

- BREAKING: Remove `from_scaled` in favor of `try_from_scaled`.

## 0.2.2

- Correctly format `Decimal::ZERO` as `0.0...` not `-0.0...`.

## 0.2.1

- Added `Decimal::to_f64(&self)`.

## 0.2.0

- Added `BorshSerialize` and `BorshDeserialize` behind `borsh` feature flag.
- Added `AddAssign`, `SubAssign`, `MulAssign`, and `DivAssign`.
- Implemented `Decimal::from_scaled(integer: I, decimals: u8)`.

## 0.1.0

- Initialize release.
