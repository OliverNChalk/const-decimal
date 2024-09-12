use ruint::aliases::U256;
use ruint::Uint;

pub trait FullMulDiv {
    fn full_mul_div(self, rhs: Self, div: Self) -> Self;
}

macro_rules! impl_primitive_mul_div {
    ($primary:ty, $intermediate:ty) => {
        impl FullMulDiv for $primary {
            fn full_mul_div(self, rhs: Self, div: Self) -> Self {
                let numer = self as $intermediate * rhs as $intermediate;
                let denom = div as $intermediate;
                let out = numer / denom;

                out.try_into()
                    .unwrap_or_else(|_| panic!("Cast failed; {out}"))
            }
        }
    };
}

impl_primitive_mul_div!(u8, u16);
impl_primitive_mul_div!(i8, i16);
impl_primitive_mul_div!(u16, u32);
impl_primitive_mul_div!(i16, i32);
impl_primitive_mul_div!(u32, u64);
impl_primitive_mul_div!(i32, i64);
impl_primitive_mul_div!(u64, u128);
impl_primitive_mul_div!(i64, i128);

impl FullMulDiv for u128 {
    fn full_mul_div(self, rhs: Self, div: Self) -> Self {
        let out: U256 = Uint::from(self) * Uint::from(rhs) / Uint::from(div);

        out.try_into().unwrap()
    }
}

impl FullMulDiv for i128 {
    fn full_mul_div(self, rhs: Self, div: Self) -> Self {
        // If we can compute the output using only an i128, then we should.
        if let Some(out) = self
            .checked_mul(rhs)
            // NB: Panic early on division by 0.
            .map(|numer| numer.checked_div(div).unwrap())
        {
            return out;
        }

        // Determine the sign of the output.
        let sign = self.signum() * rhs.signum() * div.signum();

        // Get the unsigned u256 representation of the integer (we'll later recover the
        // signed representation using two's complement).
        let this = U256::from(self.unsigned_abs());
        let rhs = U256::from(rhs.unsigned_abs());
        let div = U256::from(div.unsigned_abs());

        // Compute the unsigned output.
        let unsigned = this * rhs / div;

        // Convert back to the signed output.
        match sign {
            1 => i128::try_from(unsigned).unwrap(),
            -1 => {
                // Take two's complement (!unsigned + 1).
                // https://en.wikipedia.org/wiki/Two%27s_complement.
                let unsigned = u128::try_from(unsigned).unwrap();
                let twos_complement = (!unsigned).overflowing_add(1).0;

                i128::from_le_bytes(twos_complement.to_le_bytes())
            }
            _ => unreachable!(),
        }
    }
}
