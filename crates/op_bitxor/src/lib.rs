use std::{marker::PhantomData, ops::BitXor};

use traits::{Commutative, Identity, Inverse, SemiGroup};

pub struct OpBitXor<T>(PhantomData<T>);

impl<T> SemiGroup for OpBitXor<T>
where
    T: BitXor<Output = T> + Copy,
{
    type Set = T;

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs ^ rhs
    }
}

macro_rules! bitor_impl {
    ($( $t:ty )*) => {$(
        impl Identity for OpBitXor<$t> {
            fn id() -> Self::Set {
                0
            }
        }
    )*};
}
bitor_impl!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);

impl<T> Inverse for OpBitXor<T>
where
    Self: SemiGroup,
{
    fn inv(x: Self::Set) -> Self::Set {
        x
    }
}

impl<T> Commutative for OpBitXor<T> where Self: SemiGroup {}
