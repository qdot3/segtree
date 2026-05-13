#![warn(clippy::pedantic, /* clippy::restriction, clippy::cargo*/)]
// #![allow(clippy::indexing_slicing)]
// #![allow(clippy::arithmetic_side_effects)]
// #![allow(clippy::implicit_return)]
// #![allow(clippy::min_ident_chars)]

use std::{fmt::Debug, ops::RangeBounds};

use traits::Monoid;

/// A lazy segment tree supporting range updates and range queries.
#[derive(Clone)]
pub struct LazySegtree<Update, Query, Action>
where
    Update: Monoid<Set: Copy>,
    Query: Monoid<Set: Copy>,
    Action: FnMut(Update::Set, Query::Set) -> Query::Set,
{
    // full binary tree
    data: Box<[Query::Set]>,
    // full binary tree
    lazy: Box<[Update::Set]>,
    action: Action,

    offset: usize,
    len: usize,
}

impl<Update, Query, Action> LazySegtree<Update, Query, Action>
where
    Update: Monoid<Set: Copy>,
    Query: Monoid<Set: Copy>,
    Action: FnMut(Update::Set, Query::Set) -> Query::Set,
{
    /// Creates a new `LazySegtree`.
    ///
    /// # Preconditions
    ///
    /// `action` must distribute over `Query::op`.
    /// That is, for any update `u` and values `a`, `b`, the following must hold:
    ///
    /// ```text, rust
    /// action(u, Query::op(a, b)) == Query::op(action(u, a), action(u, b))
    /// ```
    ///
    /// Additionally, if `e` is the identity element of `Update`,
    /// then `action(e, x) == x` must hold for all `x` in `Query::Set`.
    ///
    /// # Panics
    ///
    /// `len` must be less than `isize::MAX`.
    #[must_use]
    pub fn new(action: Action, len: usize) -> Self {
        static MESSAGE: &str = "`len` must be less than `isize::MAX`";

        let offset = len.checked_next_power_of_two().expect(MESSAGE);
        let len_d = offset.checked_add(len.next_multiple_of(2)).expect(MESSAGE);
        let len_l = (len_d / 2).next_multiple_of(2);

        Self {
            data: vec![Query::id(); len_d].into_boxed_slice(),
            lazy: vec![Update::id(); len_l].into_boxed_slice(),
            action,
            offset,
            len,
        }
    }

    /// Creates a new `LazySegtree` with the given initial values.
    ///
    /// # Preconditions
    ///
    /// `action` must distribute over `Query::op`.
    /// That is, for any update `u` and values `a`, `b`, the following must hold:
    ///
    /// ```text, rust
    /// action(u, Query::op(a, b)) == Query::op(action(u, a), action(u, b))
    /// ```
    ///
    /// Additionally, if `e` is the identity element of `Update`,
    /// then `action(e, x) == x` must hold for all `x` in `Query::Set`.
    ///
    /// # Panics
    ///
    /// Panics if `iter.len()` exceeds `isize::MAX`.
    #[must_use]
    pub fn from_iter(action: Action, iter: impl ExactSizeIterator<Item = Query::Set>) -> Self {
        static MESSAGE: &str = "`iter.len()` must be less than `isize::MAX`.";

        let len = iter.len();
        let offset = len.checked_next_power_of_two().expect(MESSAGE);
        let len_d = offset.checked_add(len.next_multiple_of(2)).expect(MESSAGE);
        let len_l = (len_d / 2).next_multiple_of(2);

        let mut data = Vec::with_capacity(len_d);
        {
            let uninit = data.spare_capacity_mut();
            assert!(uninit.len() >= len_d);

            for uninit in uninit.iter_mut().take(offset).skip(len_d / 2) {
                uninit.write(Query::id());
            }
            // for i in len_d / 2..offset {
            //     uninit[i].write(Query::id());
            // }
            for (src, uninit) in iter.zip(uninit.iter_mut().skip(offset)) {
                uninit.write(src);
            }
            if len & 1 == 1 {
                uninit[len_d - 1].write(Query::id());
            }
            for p in (1..len_d / 2).rev() {
                // SAFETY: two children of `p` have already been initialized.
                let v = unsafe {
                    Query::op(uninit[p * 2].assume_init(), uninit[p * 2 + 1].assume_init())
                };
                uninit[p].write(v);
            }
            uninit[0].write(Query::id());
        }
        // SAFETY: all elements are initialized.
        unsafe { data.set_len(len_d) };

        Self {
            data: data.into_boxed_slice(),
            lazy: vec![Update::id(); len_l].into_boxed_slice(),
            action,
            offset,
            len,
        }
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn propagate_at(&mut self, p: usize) {
        let update = core::mem::replace(&mut self.lazy[p], Update::id());

        // TODO: make unchecked after all methods are tested
        {
            let data = &mut self.data[p * 2..p * 2 + 2];
            data[0] = (self.action)(update, data[0]);
            data[1] = (self.action)(update, data[1]);
        }
        if let Some(lazy) = self.lazy.get_mut(p * 2..p * 2 + 2) {
            lazy[0] = Update::op(lazy[0], update);
            lazy[1] = Update::op(lazy[1], update);
        }
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn recalculate_at(&mut self, p: usize) {
        // TODO: make unchecked after all methods are tested
        self.data[p] = Query::op(self.data[p * 2], self.data[p * 2 + 1]);
    }

    /// Returns a slice containing the updated values.
    ///
    /// # Time complexity
    ///
    /// Θ(N)
    #[must_use]
    pub fn as_slice(&mut self) -> &[Query::Set] {
        for p in 1..self.data.len() / 2 {
            self.propagate_at(p);
        }
        &self.data[self.offset..][..self.len]
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
    pub fn point_update<F>(&mut self, i: usize, f: F)
    where
        F: FnOnce(Query::Set) -> Query::Set,
    {
        assert!(i < self.len, "index out of bounds");
        let i = i + self.offset;

        // step 1. lazy propagation
        for p in (1..=self.offset.trailing_zeros()).rev().map(|d| i >> d) {
            self.propagate_at(p);
        }
        // step 2. update
        self.data[i] = f(self.data[i]);
        // step 3. recalculate ancestors
        for p in (1..=self.offset.trailing_zeros()).map(|d| i >> d) {
            self.recalculate_at(p);
        }
    }

    /// Applies `update` to the elements in `range` using the `action` specified
    /// when the `LazySegtree` was created.
    ///
    /// # Panics
    ///
    /// Panics if `range` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    pub fn range_update<R>(&mut self, range: R, update: Update::Set)
    where
        R: RangeBounds<usize>,
    {
        static MSG: &str = "index out of bounds";

        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => l.checked_add(self.offset).expect(MSG),
            std::ops::Bound::Excluded(l) => l.checked_add(self.offset + 1).expect(MSG),
            std::ops::Bound::Unbounded => self.offset,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r.checked_add(self.offset + 1).expect(MSG),
            std::ops::Bound::Excluded(r) => r.checked_add(self.offset).expect(MSG),
            std::ops::Bound::Unbounded => self.len + self.offset,
        };

        if l >= r {
            return;
        }
        assert!(r <= self.data.len(), "{}", MSG);
        let [l, r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];

        // step 1. lazy propagation
        for d in (1..usize::BITS - l.leading_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        for d in (1..usize::BITS - r.leading_zeros()).rev() {
            self.propagate_at((r - 1) >> d);
        }

        // step 2. update
        {
            let [mut l, mut r] = [l, r];

            while {
                if l >= r {
                    self.data[l] = (self.action)(update, self.data[l]);
                    if let Some(lazy) = self.lazy.get_mut(l) {
                        *lazy = Update::op(*lazy, update);
                    }
                    l += 1;
                    l >>= l.trailing_zeros();
                } else {
                    r -= 1;
                    self.data[r] = (self.action)(update, self.data[r]);
                    if let Some(lazy) = self.lazy.get_mut(r) {
                        *lazy = Update::op(*lazy, update);
                    }
                    r >>= r.trailing_zeros();
                }

                l != r
            } {}
        }

        // step 3. recalculate ancestors
        let mut l = l;
        while l > 1 {
            l /= 2;
            self.recalculate_at(l);
        }
        let mut r = r - 1;
        while r > 1 {
            r /= 2;
            self.recalculate_at(r);
        }
    }

    /// Get `i`-th element.
    ///
    /// # Panics
    ///
    /// Panics if `i` is out of bounds.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    #[must_use]
    pub fn point_query(&mut self, i: usize) -> Query::Set {
        assert!(i < self.len, "index out of bounds");
        let i = i + self.offset;

        // lazy propagation
        for p in (1..=self.offset.trailing_zeros()).rev().map(|d| i >> d) {
            self.propagate_at(p);
        }

        self.data[i]
    }

    /// Reduces the elements in `range` using `Query::op`.
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
    pub fn range_query<R>(&mut self, range: R) -> Query::Set
    where
        R: RangeBounds<usize>,
    {
        static MSG: &str = "index out of bounds";

        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => l.checked_add(self.offset).expect(MSG),
            std::ops::Bound::Excluded(l) => l.checked_add(self.offset + 1).expect(MSG),
            std::ops::Bound::Unbounded => self.offset,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => r.checked_add(self.offset + 1).expect(MSG),
            std::ops::Bound::Excluded(r) => r.checked_add(self.offset).expect(MSG),
            std::ops::Bound::Unbounded => self.len + self.offset,
        };

        if l >= r {
            return Query::id();
        }
        assert!(r <= self.data.len(), "{}", MSG);

        let [l, r] = [l >> l.trailing_zeros(), r >> r.trailing_zeros()];

        // step 1. lazy propagation
        for d in (1..usize::BITS - l.leading_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        for d in (1..usize::BITS - r.leading_zeros()).rev() {
            self.propagate_at((r - 1) >> d);
        }

        // step 2. answer query
        let mut acc_l = Query::id();
        let mut acc_r = Query::id();
        {
            let [mut l, mut r] = [l, r];

            while {
                if l >= r {
                    acc_l = Query::op(acc_l, self.data[l]);
                    l += 1;
                    l >>= l.trailing_zeros();
                } else {
                    r -= 1;
                    acc_r = Query::op(self.data[r], acc_r);
                    r >>= r.trailing_zeros();
                }

                l != r
            } {}
        }

        Query::op(acc_l, acc_r)
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
    pub fn right_partition<P>(&mut self, mut l: usize, mut pred: P) -> (Query::Set, usize)
    where
        P: FnMut(Query::Set) -> bool,
    {
        assert!(l < self.len, "index out of bounds");
        l += self.offset;
        l >>= l.trailing_zeros();

        // step 1-1. lazy propagation
        for d in (1..usize::BITS - l.leading_zeros()).rev() {
            self.propagate_at(l >> d);
        }
        // step 1-2. go up
        let mut acc = Query::id();
        loop {
            let v = Query::op(acc, self.data[l]);
            if !pred(v) {
                break;
            }
            acc = v;

            if l == 1 {
                return (acc, self.len);
            }
            l += 1;
            l >>= l.trailing_zeros();
        }

        // step 2. go down with propagation
        while l < self.data.len() / 2 {
            self.propagate_at(l);

            l *= 2;
            let v = Query::op(acc, self.data[l]);
            if pred(v) {
                acc = v;
                l += 1;
            }
        }

        // underflow occurs iff `self.data.get(l).is_none()`
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
    pub fn left_partition<P>(&mut self, mut r: usize, mut pred: P) -> (Query::Set, usize)
    where
        P: FnMut(Query::Set) -> bool,
    {
        if r == 0 {
            return (Query::id(), 0);
        }
        assert!(r <= self.len, "index out of bounds.");
        r += self.offset;
        r >>= r.trailing_zeros();

        // step 1-1. lazy propagation
        for d in (1..usize::BITS - r.leading_zeros()).rev() {
            self.propagate_at((r - 1) >> d);
        }
        // step 1-2. go up
        let mut acc = Query::id();
        loop {
            r -= 1;

            let v = Query::op(self.data[r], acc);
            if !pred(v) {
                break;
            }
            acc = v;

            if r == 1 {
                return (acc, self.len);
            }
            r >>= r.trailing_zeros();
        }

        // step 2. go down with propagation
        while r < self.data.len() / 2 {
            self.propagate_at(r);

            r = r * 2 + 1;
            let v = Query::op(acc, self.data[r]);
            if pred(v) {
                acc = v;
                r -= 1;
            }
        }

        let (r, b) = (r + 1).overflowing_sub(self.offset);
        (acc, if b { 0 } else { r })
    }
}

impl<Update, Query, Action> Debug for LazySegtree<Update, Query, Action>
where
    Update: Monoid<Set: Copy + Debug>,
    Query: Monoid<Set: Copy + Debug>,
    Action: FnMut(Update::Set, Query::Set) -> Query::Set,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LazySegtree")
            .field("data", &self.data)
            .field("lazy", &self.lazy)
            .field("action", &"******")
            .field("offset", &self.offset)
            .field("len", &self.len)
            .finish()
    }
}
