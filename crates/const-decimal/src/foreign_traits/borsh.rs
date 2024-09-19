#[cfg(test)]
mod tests {
    use borsh::{BorshDeserialize, BorshSerialize};
    use proptest::prelude::*;

    use crate::macros::generate_tests_for_common_variants;
    use crate::{Decimal, ScaledInteger};

    generate_tests_for_common_variants!(round_trip_borsh);

    fn round_trip_borsh<I, const D: u8>()
    where
        I: ScaledInteger<D> + Arbitrary + BorshSerialize + BorshDeserialize,
    {
        proptest!(|(input: Decimal<I, D>)| {
            let serialized = borsh::to_vec(&input).unwrap();
            let recovered = borsh::from_slice(&serialized).unwrap();

            prop_assert_eq!(input, recovered);
        });
    }
}
