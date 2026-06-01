use traits::{Commutative, Monoid};

#[derive(Debug, Clone)]
pub struct BIT<T>
where
    T: Commutative + Monoid,
{
    data: Vec<T::Set>,
}

impl<T> BIT<T>
where
    T: Commutative + Monoid<Set: Copy>,
{
    /// Constructs new instance initialized with the [`identity element`].
    ///
    /// [`identity element`]: traits::Identity::id
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![T::id(); n],
        }
    }

    /// Reduces the first `n` elements using [`T::op`].
    ///
    /// [`T::op`]: traits::SemiGroup::op
    ///
    /// # Time complexity
    ///
    /// O(log N)
    pub fn prefix_query(&self, mut n: usize) -> T::Set {
        let mut result = T::id();
        while n > 0 {
            result = T::op(result, self.data[n - 1]);
            n &= n - 1;
        }
        result
    }

    /// Updates `i`-th element using `T::op(elem[i], update)`.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    pub fn point_update(&mut self, mut i: usize, update: T::Set) {
        while let Some(v) = self.data.get_mut(i) {
            *v = T::op(*v, update);
            i |= i + 1
        }
    }

    /// Returns the partition point according to the given predicate.
    ///
    /// # Time complexity
    ///
    /// O(log N)
    pub fn partition_point<P>(&self, mut pred: P) -> (usize, T::Set)
    where
        P: FnMut(T::Set) -> bool,
    {
        let mut n = 0;
        let mut prefix = T::id();

        for w in (0..usize::BITS - self.data.len().leading_zeros())
            .rev()
            .map(|w| 1 << w)
        {
            if let Some(v) = self.data.get(n + w - 1).copied() {
                let v = T::op(v, prefix);
                if pred(v) {
                    prefix = v;
                    n += w;
                }
            }
        }

        (n, prefix)
    }
}

impl<T> From<Vec<T::Set>> for BIT<T>
where
    T: Commutative + Monoid<Set: Copy>,
{
    /// # Time complexity
    ///
    /// O(N)
    fn from(mut value: Vec<T::Set>) -> Self {
        for i in 0..value.len() {
            // update parents
            let p = i | (i + 1);
            // FIXME: optimize and remove this
            if p < value.len() {
                value[p] = T::op(value[p], value[i]);
            }
        }

        Self { data: value }
    }
}
