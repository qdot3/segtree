/// Verified at <https://judge.yosupo.jp/problem/range_affine_range_sum_large_array>
use std::io::{stdin, stdout, BufWriter, Write};

use compress::compress;
use input::{bind, FastInput};
use op_add::OpAdd;
use output::IntBuffer;
use segtree_lazy::LazySegtree;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input: FastInput<std::io::StdinLock<'_>> = FastInput::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 16, stdout().lock());
    let mut buf = IntBuffer::new();

    bind! { input >> _: usize, q: usize, }

    let mut query_oxxoo = Vec::with_capacity(q);
    let mut query_xooxx = Vec::with_capacity(q);
    for _ in 0..q {
        bind! { input >> t: u8, l: u32, r: u32, }
        query_xooxx.push([l, r]);

        if t == 0 {
            bind! { input >> a: u32, b: u32 }

            query_oxxoo.push((t, a, b));
        } else {
            query_oxxoo.push((t, !0, !0));
        }
    }
    let (compressed, restore) = compress::<0>(query_xooxx.as_flattened());

    type Update = OpAffine;
    type Query = (OpAdd<u64>, OpAdd<u32>);

    let mut lst = LazySegtree::<Update, Query, _>::from_iter(
        |(a, b), (sum, n)| {
            let v = (a as u64 * (sum % MOD) + b as u64 * n as u64) % MOD;
            (v, n)
        },
        restore.windows(2).map(|v| (0, (v[1] - v[0]))),
    );

    for ((t, a, b), [l, r]) in query_oxxoo.into_iter().zip(compressed.as_chunks().0) {
        if t == 0 {
            lst.range_update(*l as usize..*r as usize, (a, b));
        } else {
            let sum = lst.range_query(*l as usize..*r as usize).0 as u64 % MOD;

            output.write(buf.format(sum).as_bytes()).unwrap();
            output.write(b"\n").unwrap();
        }
    }
}

pub struct OpAffine;

impl SemiGroup for OpAffine {
    type Set = (u32, u32);

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        let (a, b) = (
            lhs.0 as u64 * rhs.0 as u64 % MOD,
            (lhs.1 as u64 * rhs.0 as u64 + rhs.1 as u64) % MOD,
        );
        (a as u32, b as u32)
    }
}

impl Identity for OpAffine {
    fn id() -> Self::Set {
        (1, 0)
    }
}
