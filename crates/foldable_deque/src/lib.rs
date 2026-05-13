use traits::Monoid;

#[derive(Debug, Default, Clone)]
pub struct FoldableDeque<T>
where
    T: Monoid,
{
    // pair of value and folded result
    head: Vec<[T::Set; 2]>,
    tail: Vec<[T::Set; 2]>,
}

impl<T> FoldableDeque<T>
where
    T: Monoid<Set: Copy>,
{
    /// Creates an empty deque with space for at least `capacity` elements.
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
        self.head.reserve(additional);
        self.tail.reserve(additional);
    }

    /// Returns the number of elements in the deque.
    #[must_use]
    pub fn len(&self) -> usize {
        if std::mem::size_of::<T::Set>() == 0 {
            // FIXME
            unimplemented!("bug in FoldableQueue::len()")
        } else {
            self.head.len() + self.tail.len()
        }
    }

    /// Returns `true` if the deque contains no elements.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.head.is_empty() && self.tail.is_empty()
    }

    /// Clear the deque.
    pub fn clear(&mut self) {
        self.head.clear();
        self.tail.clear();
    }

    /// Appends an element to the front of the deque.
    ///
    /// # Time complexity
    ///
    /// O(1)
    pub fn push_front(&mut self, value: T::Set) {
        let folded = if let Some(first) = self.head.last() {
            T::op(value, first[1])
        } else {
            value
        };
        self.head.push([value, folded]);
    }

    /// Appends an element to the back of the deque.
    ///
    /// # Time complexity
    ///
    /// O(1)
    pub fn push_back(&mut self, value: T::Set) {
        let folded = if let Some(last) = self.tail.last() {
            T::op(last[1], value)
        } else {
            value
        };
        self.tail.push([value, folded]);
    }

    /// Removes the first element and returns it, or None if the deque is empty.
    ///
    /// # Time complexity
    ///
    /// O(1) amortized
    pub fn pop_front(&mut self) -> Option<T::Set> {
        if let Some(first) = self.head.pop() {
            return Some(first[0]);
        }

        self.head
            .extend(self.tail.drain(..self.tail.len().div_ceil(2)).rev());

        {
            let mut folded = T::id();
            for pair in &mut self.head {
                folded = T::op(pair[0], folded);
                pair[1] = folded;
            }
        }
        {
            let mut folded = T::id();
            for pair in &mut self.tail {
                folded = T::op(folded, pair[0]);
                pair[1] = folded;
            }
        }

        self.head.pop().map(|first| first[0])
    }

    /// Removes the last element and returns it, or None if the deque is empty.
    ///
    /// # Time complexity
    ///
    /// O(1) amortized
    pub fn pop_back(&mut self) -> Option<T::Set> {
        if let Some(last) = self.tail.pop() {
            return Some(last[0]);
        }

        self.tail
            .extend(self.head.drain(..self.head.len().div_ceil(2)).rev());

        {
            let mut folded = T::id();
            for pair in &mut self.head {
                folded = T::op(pair[0], folded);
                pair[1] = folded;
            }
        }
        {
            let mut folded = T::id();
            for pair in &mut self.tail {
                folded = T::op(folded, pair[0]);
                pair[1] = folded;
            }
        }

        self.tail.pop().map(|last| last[0])
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
