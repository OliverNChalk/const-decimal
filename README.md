# Const Decimal

`const-decimal` is a fixed precision decimal backed by an underlying integer.
This means it can compute Add/Sub/Mul/Div* operations within it's range without
precision loss. This crate was created as there was no pre-existing crate that
offered these features for decimal numbers (`fixed` cannot represent decimals
with precision).

This is a fairly simple crate and is mainly intended to serve as a lossless
representation of decimal values. While math operations are supported it is not
intended to be a general purpose maths library.

## Goals

The goals in order of priority are:

- Safety: Panic on over/underflow (even in release).
- Precision: Use integers instead of floats.
- Performance: Use primitive underlying types were possible.
