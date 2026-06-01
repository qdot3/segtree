use std::{
    marker::PhantomData,
    ops::{Add, Neg},
};

use traits::{Commutative, Identity, Inverse, SemiGroup};

#[derive(Debug, Clone, Copy)]
pub struct OpAdd<T>(PhantomData<T>);

impl<T> SemiGroup for OpAdd<T>
where
    T: Add<Output = T> + Copy,
{
    type Set = T;

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        lhs + rhs
    }
}

impl<T> Identity for OpAdd<T>
where
    T: Add<Output = T> + Copy + From<bool>,
{
    fn id() -> Self::Set {
        T::from(false)
    }
}

impl<T> Inverse for OpAdd<T>
where
    T: Add<Output = T> + Neg<Output = T> + Copy,
{
    fn inv(x: Self::Set) -> Self::Set {
        -x
    }
}

impl<T> Commutative for OpAdd<T> where Self: SemiGroup {}
