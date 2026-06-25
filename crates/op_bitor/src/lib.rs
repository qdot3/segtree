use std::{marker::PhantomData, ops::BitOr};

use traits::{Commutative, Idempotent, Identity, SemiGroup};

pub struct OpBitOr<T>(PhantomData<T>);

impl<T> SemiGroup for OpBitOr<T>
where
    T: BitOr<Output = T> + Copy,
{
    type Set = T;

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs | rhs
    }
}

macro_rules! bitor_impl {
    ($( $t:ty )*) => {$(
        impl Identity for OpBitOr<$t> {
            fn id() -> Self::Set {
                0
            }
        }
    )*};
}
bitor_impl!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);

impl<T> Commutative for OpBitOr<T> where Self: SemiGroup {}
impl<T> Idempotent for OpBitOr<T> where Self: SemiGroup {}
