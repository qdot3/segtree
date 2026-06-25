pub trait SemiGroup {
    type Set;

    /// Performs associative binary operation.
    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set;
}

pub trait Identity: SemiGroup {
    fn id() -> Self::Set;
}

pub trait Inverse: SemiGroup {
    fn inv(x: Self::Set) -> Self::Set;
}

pub trait Monoid: Identity {}

impl<T> Monoid for T where T: Identity {}

pub trait Group: Monoid + Inverse {}

impl<T> Group for T where T: Monoid + Inverse {}

macro_rules! tuple_impl {
    ($( $t:tt, $i:tt )|*) => {
        impl<$( $t: SemiGroup ),*> SemiGroup for ($( $t ),*) {
            type Set = ($( <$t>::Set ),*);

            fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
                ($( <$t>::op(lhs.$i, rhs.$i) ),*)
            }
        }

        impl<$( $t: Identity ),*> Identity for ($( $t ),*) {
            fn id() -> Self::Set {
                ($( <$t>::id() ),*)
            }
        }

        impl<$( $t: Inverse ),*> Inverse for ($( $t ),*) {
            fn inv(x: Self::Set) -> Self::Set {
                ($( <$t>::inv(x.$i) ),*)
            }
        }

        impl<$( $t: Commutative ),*> Commutative for ($( $t ),*) {}

        impl<$( $t: Idempotent ),*> Idempotent for ($( $t ),*) {}
    };
}
tuple_impl!(T0, 0 | T1, 1);
tuple_impl!(T0, 0 | T1, 1 | T2, 2);
tuple_impl!(T0, 0 | T1, 1 | T2, 2 | T3, 3);

pub trait Idempotent: SemiGroup {}

pub trait Commutative: SemiGroup {}
