use std::{marker::PhantomData, ops::BitAnd};

use traits::{Commutative, Idempotent, Identity, SemiGroup};

pub struct OpBitAnd<T>(PhantomData<T>);

impl<T> SemiGroup for OpBitAnd<T>
where
    T: BitAnd<Output = T> + Copy,
{
    type Set = T;

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs & rhs
    }
}

macro_rules! bitand_impl {
    ($( $t:ty )*) => {$(
        impl Identity for OpBitAnd<$t> {
            fn id() -> Self::Set {
                !0
            }
        }
    )*};
}
bitand_impl!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);

impl<T> Commutative for OpBitAnd<T> where Self: SemiGroup {}
impl<T> Idempotent for OpBitAnd<T> where Self: SemiGroup {}
