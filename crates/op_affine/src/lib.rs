use std::marker::PhantomData;

use traits::{Identity, SemiGroup};

#[derive(Debug, Clone, Copy)]
pub struct OpAffine<T>(PhantomData<T>);

macro_rules! trait_impl {
    ($( $t:ty )*) => {$(
        impl SemiGroup for OpAffine<$t> {
            type Set = ($t, $t);

            fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                (lhs.0 * rhs.0, lhs.1 * rhs.0 + rhs.1)
            }
        }

        impl Identity for OpAffine<$t> {
            fn id() -> Self::Set {
                (1 as $t, 0 as $t)
            }
        }
    )*};
}
trait_impl!( i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 );
