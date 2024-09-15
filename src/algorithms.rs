use crate::ScaledInteger;

pub(crate) fn log10<const D: u8, I: ScaledInteger<D>>(mut input: I) -> I {
    let mut result = I::ZERO;
    while input > I::ZERO {
        input /= I::TEN;
        result += I::ONE;
    }

    result
}
