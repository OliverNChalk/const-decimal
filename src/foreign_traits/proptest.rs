use std::fmt::Debug;

use proptest::arbitrary::Mapped;
use proptest::prelude::{any, Arbitrary, Strategy};

use crate::{Decimal, ScaledInteger};

impl<const D: u8, I> Arbitrary for Decimal<I, D>
where
    I: ScaledInteger<D> + Arbitrary + Debug,
{
    type Parameters = ();
    type Strategy = Mapped<I, Self>;

    fn arbitrary() -> Self::Strategy {
        Self::arbitrary_with(())
    }

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        any::<I>().prop_map(|integer| Decimal(integer))
    }
}
