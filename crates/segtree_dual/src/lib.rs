use std::ops::RangeBounds;

use traits::Monoid;

/// A dual segment tree supporting range updates and point queries.
///
/// This is suitable for composing functions.
#[derive(Debug, Clone)]
pub struct DualSegtree<T>
where
    T: Monoid,
{
    // full binary tree
    data: Box<[T::Set]>,

    offset: usize,
    len: usize,
}

impl<T> DualSegtree<T>
where
    T: Monoid<Set: Copy>,
{
    /// Creates a new instance.
    ///
    /// # Panics
    ///
    /// `len` must be less than `isize::MAX`.
    #[must_use]
    pub fn new(len: usize) -> Self {
        static MSG: &str = "`len` must be less than `isize::MAX`";

        let offset = len.checked_next_power_of_two().expect(MSG);
        let new_len = offset.checked_add(len.next_multiple_of(2)).expect(MSG);

        Self {
            data: vec![T::id(); new_len].into_boxed_slice(),
            offset,
            len,
        }
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn propagate_above(&mut self, p: usize) {
        for p in (1..usize::BITS - p.leading_zeros()).rev().map(|d| p >> d) {
            self.data[p * 2] = T::op(self.data[p * 2], self.data[p]);
            self.data[p * 2 + 1] = T::op(self.data[p * 2 + 1], self.data[p]);
            self.data[p] = T::id();
        }
    }

    /// Returns a slice containing the updated values.
    ///
    /// # Time complexity
    ///
    /// Θ(N)
    #[must_use]
    pub fn as_slice(&mut self) -> &[T::Set] {
        for p in 1..self.data.len() / 2 {
            self.data[p * 2] = T::op(self.data[p * 2], self.data[p]);
            self.data[p * 2 + 1] = T::op(self.data[p * 2 + 1], self.data[p]);
            self.data[p] = T::id();
        }
        &self.data[self.offset..][..self.len]
    }

    /// Gets `i`-th element
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    #[must_use]
    pub fn point_query(&mut self, mut i: usize) -> T::Set {
        assert!(i < self.len, "index out of bounds");
        i += self.offset;

        let mut acc = self.data[i];
        while i > 1 {
            i /= 2;
            acc = T::op(acc, self.data[i]);
        }

        acc
    }

    /// Updates `i`-th element using [`T::op(*, update)`](traits::SemiGroup::op).
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    pub fn point_update(&mut self, mut i: usize, update: T::Set) {
        assert!(i < self.len, "index out of bounds");
        i += self.offset;

        // lazy propagation
        // FIXME: can be skipped for commutative ops
        self.propagate_above(i);
        self.data[i] = T::op(self.data[i], update);
    }

    /// Updates elements in the `range` using [`T::op(*, update)`](traits::SemiGroup::op).
    ///
    /// # Panics
    ///
    /// Panics if `range` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    pub fn range_update<R>(&mut self, range: R, update: T::Set)
    where
        R: RangeBounds<usize>,
    {
        static MSG: &str = "index out of bounds";

        let mut l = match range.start_bound() {
            std::ops::Bound::Included(l) => l.checked_add(self.offset).expect(MSG),
            std::ops::Bound::Excluded(l) => l.checked_add(self.offset + 1).expect(MSG),
            std::ops::Bound::Unbounded => self.offset,
        };
        let mut r = match range.end_bound() {
            std::ops::Bound::Included(r) => r.checked_add(self.offset + 1).expect(MSG),
            std::ops::Bound::Excluded(r) => r.checked_add(self.offset).expect(MSG),
            std::ops::Bound::Unbounded => self.len + self.offset,
        };

        if l >= r {
            return;
        }
        assert!(r <= self.data.len(), "{}", MSG);

        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();

        // step 1. lazy propagation
        // FIXME: can be skipped for commutative ops
        self.propagate_above(l);
        self.propagate_above(r - 1);

        // step 2. update
        while {
            if l >= r {
                self.data[l] = T::op(self.data[l], update);
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                self.data[r] = T::op(self.data[r], update);
                r >>= r.trailing_zeros();
            }

            l != r
        } {}
    }
}

impl<I, T> From<I> for DualSegtree<T>
where
    T: Monoid<Set: Copy>,
    I: IntoIterator<Item = T::Set>,
    I::IntoIter: ExactSizeIterator,
{
    /// Creates a new instance from initial values.
    ///
    /// # Panics
    ///
    /// `iter.len()` must be less than `isize::MAX`.
    fn from(iter: I) -> Self {
        static MSG: &str = "`iter.len()` must be less than `isize::MAX`.";

        let iter = iter.into_iter();
        let len = iter.len();
        let offset = len.checked_next_power_of_two().expect(MSG);
        let new_len = offset.checked_add(len.next_multiple_of(2)).expect(MSG);

        let mut data = Vec::with_capacity(new_len);
        data.resize(offset, T::id());
        data.extend(iter);
        if len & 1 == 1 {
            data.push(T::id());
        }

        Self {
            data: data.into_boxed_slice(),
            offset,
            len,
        }
    }
}
