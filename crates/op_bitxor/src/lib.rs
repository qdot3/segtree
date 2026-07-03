use std::marker::PhantomData;

use traits::{Commutative, Identity, Inverse, SemiGroup};

pub struct OpBitXor<T>(PhantomData<T>);

macro_rules! trait_impl {
    ($( $t:ty )*) => {$(
        impl SemiGroup for OpBitXor<$t> {
            type Set = $t;

            fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                lhs ^ rhs
            }
        }

        impl Identity for OpBitXor<$t> {
            fn id() -> Self::Set {
                0 as $t
            }
        }

        impl Inverse for OpBitXor<$t> {
            fn inv(x: &Self::Set) -> Self::Set {
                *x
            }
        }
    )*};
}
trait_impl!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);

impl<T> Commutative for OpBitXor<T> where Self: SemiGroup {}
