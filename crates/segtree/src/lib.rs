use std::ops::RangeBounds;

use traits::Monoid;

#[derive(Debug, Clone)]
pub struct Segtree<T>
where
    T: Monoid,
{
    // full binary tree
    data: Box<[T::Set]>,

    offset: usize,
    len: usize,
}

impl<T> Segtree<T>
where
    T: Monoid<Set: Copy>,
{
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

    /// Returns a slice of updated values.
    #[must_use]
    pub fn as_slice(&self) -> &[T::Set] {
        &self.data[self.offset..][..self.len]
    }

    /// Get `i`-th element.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    #[must_use]
    pub fn point_query(&self, i: usize) -> T::Set {
        assert!(i < self.len, "index out of bounds");

        self.data[i + self.offset]
    }

    /// Reduces the elements in the `range` using `Query::op`.
    ///
    /// See also: [`Iterator::reduce`].
    ///
    /// # Panics
    ///
    /// Panics if `range` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    #[must_use]
    pub fn range_query<R>(&self, range: R) -> T::Set
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
            return T::id();
        }
        assert!(r <= self.data.len(), "{}", MSG);

        l >>= l.trailing_zeros();
        r >>= r.trailing_zeros();

        let mut acc_l = T::id();
        let mut acc_r = T::id();
        while {
            if l >= r {
                acc_l = T::op(acc_l, self.data[l]);
                l += 1;
                l >>= l.trailing_zeros();
            } else {
                r -= 1;
                acc_r = T::op(self.data[r], acc_r);
                r >>= r.trailing_zeros();
            }

            l != r
        } {}

        T::op(acc_l, acc_r)
    }

    /// Updates `i`-th element using `f`,
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    pub fn point_update<F>(&mut self, mut i: usize, f: F)
    where
        F: FnOnce(T::Set) -> T::Set,
    {
        assert!(i < self.len, "index out of bounds");
        i += self.offset;
        // step 1. update
        self.data[i] = f(self.data[i]);
        // step 2. recalculate ancestors
        while i > 1 {
            i /= 2;
            self.data[i] = T::op(self.data[i * 2], self.data[i * 2 + 1]);
        }
    }

    #[deprecated = "this api is not tested and may contains bugs. please tell me a problem to verify this."]
    /// Performs an operation similar to [`slice::partition_point`].
    ///
    /// The `LazySegtree` is assumed to be partitioned according to the given predicate.
    /// That is, once the predicate returns `false`, it must remain `false` for all larger ranges.
    /// Otherwise, the returned value is unspecified.
    ///
    /// ```no_run
    /// // Illustrative example of the behavior.
    /// let (v, p) = lst.right_partition(l, pred);
    /// assert_eq!(v, lst.range_query(l..p));
    ///
    /// for r in l..p {
    ///     let v = lst.range_query(l..r);
    ///     assert!(pred(v));
    /// }
    /// for r in p.. {
    ///     let v = lst.range_query(l..r);
    ///     assert!(!pred(v));
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `l` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    #[must_use]
    pub fn right_partition<P>(&self, mut l: usize, mut pred: P) -> (T::Set, usize)
    where
        P: FnMut(T::Set) -> bool,
    {
        assert!(l < self.len, "index out of bounds");
        l += self.offset;
        l >>= l.trailing_zeros();

        let mut acc = T::id();
        loop {
            let v = T::op(acc, self.data[l]);
            if !pred(v) {
                break;
            }
            acc = v;

            l += 1;
            if l.count_ones() <= 1 {
                return (acc, self.len);
            }
            l >>= l.trailing_zeros();
        }

        while l < self.data.len() / 2 {
            l *= 2;

            let v = T::op(acc, self.data[l]);
            if pred(v) {
                acc = v;
                l += 1;
            }
        }

        (acc, l.wrapping_sub(self.offset).min(self.len))
    }

    #[deprecated = "this api is not tested and may contains bugs. please tell me a problem to verify this."]
    /// Performs an operation similar to [`slice::partition_point`].
    ///
    /// The `LazySegtree` is assumed to be partitioned according to the given predicate.
    /// That is, once the predicate returns `false`, it must remain `false` for all larger ranges.
    /// Otherwise, the returned value is unspecified.
    ///
    /// ```no_run
    /// // Illustrative example of the behavior.
    /// let (v, p) = lst.left_partition(r, pred);
    /// assert_eq!(v, lst.range_query(p..r));
    ///
    /// for l in 0..p {
    ///     let v = lst.range_query(l..r);
    ///     assert!(!pred(v));
    /// }
    /// for l in p..r {
    ///     let v = lst.range_query(l..r);
    ///     assert!(pred(v));
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if `r` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    #[must_use]
    pub fn left_partition<P>(&self, mut r: usize, mut pred: P) -> (T::Set, usize)
    where
        P: FnMut(T::Set) -> bool,
    {
        assert!(r <= self.len, "index out of bounds");
        r += self.offset;
        r >>= r.trailing_zeros();

        let mut acc = T::id();
        while !r.is_power_of_two() {
            r -= 1;

            let v = T::op(acc, self.data[r]);
            if !pred(v) {
                break;
            }
            acc = v;

            r >>= r.trailing_zeros();
        }

        r = (r - 1).max(1);
        while r < self.data.len() / 2 {
            r = r * 2 + 1;

            let v = self.data[r];
            if pred(v) {
                acc = v;
                r -= 1;
            }
        }

        (acc, r + 1 - self.offset)
    }
}

impl<I, T> From<I> for Segtree<T>
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
        {
            let uninit = data.spare_capacity_mut();
            assert!(uninit.len() >= new_len, "bug");

            for uninit in uninit.iter_mut().take(offset).skip(new_len / 2) {
                uninit.write(T::id());
            }
            for (src, uninit) in iter.zip(uninit.iter_mut().skip(offset)) {
                uninit.write(src);
            }
            if len & 1 == 1 {
                uninit[new_len - 1].write(T::id());
            }
            // recalculate ancestors
            for p in (1..new_len / 2).rev() {
                // SAFETY: two children of `p` have already been initialized.
                let v =
                    unsafe { T::op(uninit[p * 2].assume_init(), uninit[p * 2 + 1].assume_init()) };
                uninit[p].write(v);
            }
            uninit[0].write(T::id());
        }
        // SAFETY: all elements are initialized.
        unsafe { data.set_len(new_len) };

        Self {
            data: data.into_boxed_slice(),
            offset,
            len,
        }
    }
}
