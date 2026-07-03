/// Verified at <https://judge.yosupo.jp/problem/point_set_range_composite_large_array>
use std::io::{stdin, stdout, BufWriter, Write};

use compress::compress;
use input::{bind, FastInput};
use output::IntBuffer;
use segtree::Segtree;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input: FastInput<std::io::StdinLock<'_>> = FastInput::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    bind! { input >> _: usize, q: usize, }

    let mut query_oxxo = Vec::with_capacity(q);
    let mut query_xoox = Vec::with_capacity(q);
    for _ in 0..q {
        bind! { input >> a: u8, b: u32, c: u32, d: u32, }
        query_oxxo.push((a, d));
        query_xoox.push([b, c]);
    }
    let compressed_xoox = compress::<0>(query_xoox.as_flattened()).0;
    let compressed_xoox = compressed_xoox.as_chunks::<2>().0;

    let mut segtree = Segtree::<OpAffine>::new(2 * q);
    for i in 0..q {
        let t = query_oxxo[i].0;

        if t == 0 {
            let p = compressed_xoox[i][0] as usize;
            let a = query_xoox[i][1];
            let b = query_oxxo[i].1;

            segtree.point_update(p, |_| (a, b));
        } else {
            let l = compressed_xoox[i][0] as usize;
            let r = compressed_xoox[i][1] as usize;
            let x = query_oxxo[i].1 as u64;

            let (a, b) = segtree.range_query(l..r).unwrap();

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

    fn op(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
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
