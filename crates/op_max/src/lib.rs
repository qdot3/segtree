use std::marker::PhantomData;

use traits::{Identity, SemiGroup};

pub struct OpMax<T>(PhantomData<T>);

impl<T> SemiGroup for OpMax<T>
where
    T: Ord,
{
    type Set = T;

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs.max(rhs)
    }
}

macro_rules! int_max_id_impl {
    ($( $t:ty )*) => {$(
        impl Identity for OpMax<$t> {
            fn id() -> Self::Set {
                <$t>::MIN
            }
        }
    )*};
}
int_max_id_impl!( i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize );
