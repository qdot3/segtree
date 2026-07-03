use std::marker::PhantomData;

use traits::{Commutative, Idempotent, Identity, SemiGroup};

pub struct OpMax<T>(PhantomData<T>);

macro_rules! trait_impl_int {
    ($( $t:ty )*) => {$(
        impl SemiGroup for OpMax<$t> {
            type Set = $t;

            fn op(&lhs: &Self::Set, &rhs: &Self::Set) -> Self::Set {
                lhs.max(rhs)
            }
        }

        impl Identity for OpMax<$t> {
            fn id() -> Self::Set {
                <$t>::MIN
            }
        }
    )*};
}
trait_impl_int!( i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize );

macro_rules! trait_impl_float {
    ($( $t:ty )*) => {$(
        impl SemiGroup for OpMax<$t> {
            type Set = $t;

            fn op(&lhs: &Self::Set, &rhs: &Self::Set) -> Self::Set {
                lhs.max(rhs)
            }
        }

        impl Identity for OpMax<$t> {
            fn id() -> Self::Set {
                <$t>::NEG_INFINITY
            }
        }
    )*};
}
trait_impl_float!( f32 f64 );

impl<T> Idempotent for OpMax<T> where Self: SemiGroup {}
impl<T> Commutative for OpMax<T> where Self: SemiGroup {}
