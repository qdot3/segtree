/// Verified at <https://judge.yosupo.jp/problem/range_affine_range_sum>
use std::io::{stdin, stdout, BufWriter, Write};

use input::{bind, parse, FastInput};
use op_add::OpAdd;
use output::IntBuffer;
use segtree_lazy::LazySegtree;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input: FastInput<std::io::StdinLock<'_>> = FastInput::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 16, stdout().lock());
    let mut buf = IntBuffer::new();

    bind! { input >> n: usize, q: usize, }

    type Update = OpAffine;
    type Query = (OpAdd<u64>, OpAdd<u64>);

    let mut lst = LazySegtree::<Update, Query, _>::from_iter(
        |(a, b), (sum, n)| ((a * (sum % MOD) + b * n) % MOD, n),
        (0..n).map(|_| (parse!(input >> u64), 1)),
    );

    for _ in 0..q {
        bind! { input >> t: u8, }

        if t == 0 {
            bind! { input >> l: usize, r: usize, b: u64, c: u64, }

            lst.range_update(l..r, (b, c));
        } else {
            bind! { input >> l: usize, r: usize, }

            let sum = lst.range_query(l..r).0 % MOD;

            output.write(buf.format(sum).as_bytes()).unwrap();
            output.write(b"\n").unwrap();
        }

        #[cfg(debug_assertions)]
        println!("{:?}", lst)
    }
}

pub struct OpAffine;

impl SemiGroup for OpAffine {
    type Set = (u64, u64);

    fn op(lhs: Self::Set, rhs: Self::Set) -> Self::Set {
        (lhs.0 * rhs.0 % MOD, (lhs.1 * rhs.0 + rhs.1) % MOD)
    }
}

impl Identity for OpAffine {
    fn id() -> Self::Set {
        (1, 0)
    }
}
