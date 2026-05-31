use std::marker::PhantomData;

use traits::{Idempotent, Identity, SemiGroup};

pub struct OpMin<T>(PhantomData<T>);

impl<T> SemiGroup for OpMin<T>
where
    T: Ord,
{
    type Set = T;

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs.min(rhs)
    }
}

impl<T> Idempotent for OpMin<T> where T: Ord {}

macro_rules! int_min_id_impl {
    ($( $t:ty )*) => {$(
        impl Identity for OpMin<$t> {
            fn id() -> Self::Set {
                <$t>::MAX
            }
        }
    )*};
}
int_min_id_impl!( i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize );
