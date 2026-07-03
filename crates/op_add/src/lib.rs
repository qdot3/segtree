use std::marker::PhantomData;

use traits::{Commutative, Identity, Inverse, SemiGroup};

#[derive(Debug, Clone, Copy)]
pub struct OpAdd<T>(PhantomData<T>);

macro_rules! base_impl {
    ($( $t:ty )*) => {$(
        impl SemiGroup for OpAdd<$t> {
            type Set = $t;

            fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
                lhs + rhs
            }
        }

        impl Identity for OpAdd<$t> {
            fn id() -> Self::Set {
                0 as $t
            }
        }
    )*};
}
base_impl!( i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 );

macro_rules! patch_impl {
    ($( $t:ty )*) => {$(
        impl Inverse for OpAdd<$t> {
            fn inv(x: &Self::Set) -> Self::Set {
                -x
            }
        }
    )*};
}
patch_impl!( i8 i16 i32 i64 i128 isize f32 f64 );

impl<T> Commutative for OpAdd<T> where Self: SemiGroup {}
