use ruint::aliases::U256;
use ruint::Uint;

pub trait MulDiv {
    fn mul_div(self, rhs: Self, div: Self) -> Self;
}

impl MulDiv for u64 {
    fn mul_div(self, rhs: Self, div: Self) -> Self {
        ((self as u128 * rhs as u128) / div as u128)
            .try_into()
            .unwrap()
    }
}

impl MulDiv for i64 {
    fn mul_div(self, rhs: Self, div: Self) -> Self {
        ((self as i128 * rhs as i128) / div as i128)
            .try_into()
            .unwrap()
    }
}

impl MulDiv for u128 {
    fn mul_div(self, rhs: Self, div: Self) -> Self {
        let out: U256 = Uint::from(self) * Uint::from(rhs) / Uint::from(div);

        out.try_into().unwrap()
    }
}

impl MulDiv for i128 {
    fn mul_div(self, rhs: Self, div: Self) -> Self {
        // Determine the sign of the output.
        let sign = self.signum() * rhs.signum() * div.signum();

        // Get the unsigned u256 representation of the integer.
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
