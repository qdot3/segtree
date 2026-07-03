use std::marker::PhantomData;

use traits::{Commutative, Idempotent, Identity, SemiGroup};

pub struct OpBitOr<T>(PhantomData<T>);

macro_rules! trait_impl {
    ($( $t:ty )*) => {$(
        impl SemiGroup for OpBitOr<$t> {
            type Set = $t;

            fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                lhs | rhs
            }
        }

        impl Identity for OpBitOr<$t> {
            fn id() -> Self::Set {
                0 as $t
            }
        }
    )*};
}
trait_impl!(i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize);

impl<T> Commutative for OpBitOr<T> where Self: SemiGroup {}
impl<T> Idempotent for OpBitOr<T> where Self: SemiGroup {}
