/// Verified at <https://judge.yosupo.jp/problem/point_set_range_composite>
use std::io::{stdin, stdout, BufWriter, Write};

use output::IntBuffer;
use reader::FastBufReader;
use segtree::Segtree;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input = FastBufReader::<{ 1 << 16 }, _>::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    let n: usize = input.parse_next_token().unwrap();
    let q: usize = input.parse_next_token().unwrap();

    let mut segtree = Segtree::<OpAffine>::from((0..n).map(|_| {
        (
            input.parse_next_token::<u32>().unwrap(),
            input.parse_next_token::<u32>().unwrap(),
        )
    }));

    for _ in 0..q {
        let t: usize = input.parse_next_token().unwrap();

        if t == 0 {
            let p: usize = input.parse_next_token().unwrap();
            let c: u32 = input.parse_next_token().unwrap();
            let d: u32 = input.parse_next_token().unwrap();

            segtree.point_update(p, |_| (c, d));

            #[cfg(debug_assertions)]
            println!("{:?}", segtree);
        } else {
            let l: usize = input.parse_next_token().unwrap();
            let r: usize = input.parse_next_token().unwrap();
            let x: u64 = input.parse_next_token().unwrap();

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
