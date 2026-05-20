//! Verified at <https://judge.yosupo.jp/problem/point_set_range_composite>
use std::io::{stdin, stdout, BufWriter, Write};

use input::{bind, FastInput};
use output::IntBuffer;
use segtree::Segtree;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input: FastInput<std::io::StdinLock<'_>> = FastInput::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    bind! { input >> n: usize, q: usize, }

    let mut segtree = Segtree::<OpAffine>::new(n);
    {
        let mut batch = segtree.batch_updater();
        for i in 0..n {
            bind! { input >> a: u32, b: u32, }
            batch.point_update(i, |_| (a, b));
        }
    }

    const B: u32 = 30;
    let mut updates = Vec::with_capacity(q);

    for _ in 0..q {
        bind! { input >> t: u8, }

        if t == 0 {
            bind! { input >> p: u32, c: u32, d: u32, }

            updates.push((p, c, d));
        } else {
            if !updates.is_empty() {
                updates.sort_by_key(|v| v.0 / B);

                for batch in updates.chunk_by(|a, b| a.0 / B == b.0 / B) {
                    let mut b = segtree.batch_updater();
                    batch.iter().for_each(|(p, c, d)| {
                        b.point_update(*p as usize, |_| (*c, *d));
                    });
                }

                updates.clear();
            }

            bind! { input >> l: usize, r: usize, x: u64, }

            let (a, b) = segtree.range_query(l..r);

            output
                .write(buf.format((a as u64 * x + b as u64) % MOD).as_bytes())
                .unwrap();
            output.write(b"\n").unwrap();
        }
    }
}

#[derive(Debug)]
pub struct OpAffine;

impl SemiGroup for OpAffine {
    type Set = (u32, u32);

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        let a = lhs.0 as u64 * rhs.0 as u64 % MOD;
        let b = (lhs.1 as u64 * rhs.0 as u64 + rhs.1 as u64) % MOD;
        (a as u32, b as u32)
    }
}

impl Identity for OpAffine {
    fn id() -> Self::Set {
        (1, 0)
    }
}
