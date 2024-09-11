# Const Decimal

Fixed precision decimal type using generics & compile time constants where
possible.

The goal of this crate is to provide a numeric type that is suitable for use in
financial applications. More specifically, the type has the following
properties:

- Can losslessly represent a decimal with `D` precision.
- Add/Sub/Mul are lossless.
- Division truncates the remainder deterministically like integer division.
- All overflows/underflows are panics.
  - TODO: Add wrapping/unchecked operations.
