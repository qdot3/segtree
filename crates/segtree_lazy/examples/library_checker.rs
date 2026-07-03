/// Verified at <https://judge.yosupo.jp/problem/range_affine_range_sum>
use std::io::{stdin, stdout, BufWriter, Write};

use op_add::OpAdd;
use output::IntBuffer;
use reader::FastBufReader;
use segtree_lazy::LazySegtree;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input = FastBufReader::<{ 1 << 16 }, _>::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    let n: usize = input.parse_next_token().unwrap();
    let q: usize = input.parse_next_token().unwrap();

    type Update = OpAffine;
    type Query = (OpAdd<u64>, OpAdd<u64>);

    let mut lst = LazySegtree::<Update, Query, _>::from_iter(
        |&(a, b), &(sum, n)| ((a * (sum % MOD) + b * n) % MOD, n),
        (0..n).map(|_| {
            let a: u64 = input.parse_next_token().unwrap();
            (a, 1)
        }),
    );

    for _ in 0..q {
        let t: u8 = input.parse_next_token().unwrap();

        if t == 0 {
            let l: usize = input.parse_next_token().unwrap();
            let r: usize = input.parse_next_token().unwrap();
            let b: u64 = input.parse_next_token().unwrap();
            let c: u64 = input.parse_next_token().unwrap();

            lst.range_update(l..r, (b, c));
        } else {
            let l: usize = input.parse_next_token().unwrap();
            let r: usize = input.parse_next_token().unwrap();

            let sum = lst.range_query(l..r).0 % MOD;

            output.write(buf.format(sum).as_bytes()).unwrap();
            output.write(b"\n").unwrap();
        }
    }
}

pub struct OpAffine;

impl SemiGroup for OpAffine {
    type Set = (u64, u64);

    fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        (lhs.0 * rhs.0 % MOD, (lhs.1 * rhs.0 + rhs.1) % MOD)
    }
}

impl Identity for OpAffine {
    fn id() -> Self::Set {
        (1, 0)
    }
}
