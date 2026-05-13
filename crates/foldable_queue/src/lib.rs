use traits::Monoid;

#[derive(Debug, Default, Clone)]
pub struct FoldableQueue<T>
where
    T: Monoid,
{
    head: Vec<[T::Set; 2]>,
    tail: Vec<[T::Set; 2]>,
}

impl<T> FoldableQueue<T>
where
    T: Monoid<Set: Copy>,
{
    /// Creates an empty queue with space for at least `capacity` elements.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            head: Vec::with_capacity(capacity),
            tail: Vec::with_capacity(capacity),
        }
    }

    /// Reserves capacity for at least `additional` more elements.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    pub fn reserve(&mut self, additional: usize) {
        self.tail.reserve(additional);
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        if std::mem::size_of::<T::Set>() == 0 {
            // FIXME
            unimplemented!("bug in FoldableQueue::len()")
        } else {
            self.head.len() + self.tail.len()
        }
    }

    /// Returns `true` if the queue contains no elements.
    pub fn is_empty(&self) -> bool {
        self.head.is_empty() && self.tail.is_empty()
    }

    /// Clear the queue.
    pub fn clear(&mut self) {
        self.head.clear();
        self.tail.clear();
    }

    /// Appends an element to the back of the queue.
    ///
    /// # Time complexity
    ///
    /// O(1)
    pub fn push_back(&mut self, value: T::Set) {
        let folded = if let Some(prev) = self.tail.last() {
            T::op(prev[1], value)
        } else {
            value
        };
        self.tail.push([value, folded]);
    }

    /// Removes the first element and returns it, or None if the queue is empty.
    ///
    /// # Time complexity
    ///
    /// O(1) amortized
    pub fn pop_front(&mut self) -> Option<T::Set> {
        if let Some([value, _]) = self.head.pop() {
            return Some(value);
        }

        std::mem::swap(&mut self.head, &mut self.tail);

        self.head.reverse();
        let mut folded = T::id();
        for pair in &mut self.head {
            folded = T::op(pair[0], folded);
            pair[1] = folded;
        }

        self.head.pop().map(|[value, _]| value)
    }

    /// Folds every element into an accumulator using [`T::op`](traits::SemiGroup::op),
    /// returning the final result.
    ///
    /// # Time complexity
    ///
    /// O(1)
    #[must_use]
    pub fn fold(&self) -> T::Set {
        let head = if let Some(first) = self.head.last() {
            first[1]
        } else {
            T::id()
        };
        let tail = if let Some(last) = self.tail.last() {
            last[1]
        } else {
            T::id()
        };

        T::op(head, tail)
    }
}

impl<T> FromIterator<T::Set> for FoldableQueue<T>
where
    T: Monoid<Set: Copy>,
{
    fn from_iter<I: IntoIterator<Item = T::Set>>(iter: I) -> Self {
        let head = {
            // may be optimized
            let mut head: Vec<_> = iter.into_iter().map(|v| [v, T::id()]).collect();
            head.reverse();

            let mut folded = T::id();
            for pair in &mut head {
                folded = T::op(pair[0], folded);
                pair[1] = folded;
            }

            head
        };

        Self {
            head,
            tail: Vec::new(),
        }
    }
}
