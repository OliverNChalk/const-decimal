# Changelog

All notable changes to `const-decimal`.

## Unreleased

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
