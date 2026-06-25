use std::ops::RangeBounds;

use traits::Idempotent;

const B: usize = 16;

#[derive(Debug, Clone)]
pub struct SparseTable<T: Idempotent> {
    summary: Box<[T::Set]>,
    partition: Box<[usize]>,

    values: Box<[T::Set]>,
    prefix: Box<[T::Set]>,
    suffix: Box<[T::Set]>,
}

impl<T> SparseTable<T>
where
    T: Idempotent,
    T::Set: Copy,
{
    pub fn range_query<R>(&self, range: R) -> Option<T::Set>
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => l.checked_add(1)?,
            std::ops::Bound::Unbounded => 0,
        };
        let r = match range.end_bound() {
            std::ops::Bound::Included(r) => *r,
            std::ops::Bound::Excluded(r) => r.checked_sub(1)?,
            std::ops::Bound::Unbounded => self.values.len().checked_sub(1)?,
        };

        if l > r || r >= self.values.len() {
            return None;
        }

        let [bl, br] = [l / B, r / B];
        match (br - bl).cmp(&1) {
            std::cmp::Ordering::Greater => {
                let w = (br - bl - 1).ilog2() as usize;
                let layer = &self.summary[self.partition[w]..self.partition[w + 1]];

                Some(T::op(
                    T::op(self.suffix[l], layer[bl + 1]),
                    T::op(layer[br - (1 << w)], self.prefix[r]),
                ))
            }
            std::cmp::Ordering::Equal => Some(T::op(self.suffix[l], self.prefix[r])),
            std::cmp::Ordering::Less => {
                let mut result = self.values[l];
                for i in l + 1..=r {
                    result = T::op(result, self.values[i])
                }
                Some(result)
            }
        }
    }
}

impl<T> From<Box<[T::Set]>> for SparseTable<T>
where
    T: Idempotent,
    T::Set: Copy,
{
    fn from(values: Box<[T::Set]>) -> Self {
        if values.is_empty() {
            return Self {
                summary: Box::new([]),
                partition: Box::new([]),
                values: Box::new([]),
                prefix: Box::new([]),
                suffix: Box::new([]),
            };
        }

        let prefix = {
            let mut pre = values.clone();
            for chunk in pre.chunks_mut(B) {
                for i in 1..chunk.len() {
                    chunk[i] = T::op(chunk[i - 1], chunk[i]);
                }
            }
            pre
        };

        let suffix = {
            let mut suf = values.clone();
            for chunk in suf.chunks_mut(B) {
                for i in (0..chunk.len() - 1).rev() {
                    chunk[i] = T::op(chunk[i], chunk[i + 1]);
                }
            }
            suf
        };

        let (summary, partition) = {
            let len = values.len().div_ceil(B);
            let height = len.ilog2() as usize;

            let mut summary = Vec::with_capacity(len * height);
            for chunk in suffix.chunks(B) {
                summary.push(chunk[0]);
            }

            let mut partition = Vec::with_capacity(height + 1);
            partition.push(0);
            partition.push(summary.len());

            for i in 0..height {
                let w = 1 << i;

                for i in partition[i] + w..partition[i + 1] {
                    summary.push(T::op(summary[i - w], summary[i]));
                }
                partition.push(summary.len());
            }

            (summary.into_boxed_slice(), partition.into_boxed_slice())
        };

        Self {
            summary,
            partition,
            values,
            prefix,
            suffix,
        }
    }
}
