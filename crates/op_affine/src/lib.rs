use std::{
    marker::PhantomData,
    ops::{Add, Mul},
};

use traits::{Identity, SemiGroup};

#[derive(Debug, Clone, Copy)]
pub struct OpAffine<T>(PhantomData<T>);

impl<T> SemiGroup for OpAffine<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    type Set = (T, T);

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        (lhs.0 * rhs.0, lhs.1 * rhs.0 + rhs.1)
    }
}

impl<T> Identity for OpAffine<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + From<bool>,
{
    fn id() -> Self::Set {
        (T::from(true), T::from(false))
    }
}

// macro_rules! identity_impl {
//     ($( $t:ty )*) => {$(
//         impl Identity for OpAffine<$t> {
//             fn id() -> Self::Set {
//                 // (1, 0)
//                 (<$t>::from(true), <$t>::from(false))
//             }
//         }
//     )*};
// }
// identity_impl!( i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize f32 f64 );
