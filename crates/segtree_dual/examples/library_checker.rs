/// Verified at <https://judge.yosupo.jp/problem/range_affine_point_get>
use std::io::{stdin, stdout, BufWriter, Write};

use input::{bind, FastInput};
use output::IntBuffer;
use segtree_dual::DualSegtree;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input: FastInput<std::io::StdinLock<'_>> = FastInput::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    bind! { input >> n: usize, q: usize, a: [u32; n], }

    type Update = OpAffine;
    let mut dual = DualSegtree::<Update>::new(n);

    for _ in 0..q {
        bind! { input >> t: u8, }

        if t == 0 {
            bind! { input >> l: usize, r: usize, b: u32, c: u32, }

            dual.range_update(l..r, (b, c));

            #[cfg(debug_assertions)]
            println!("{:?}", dual)
        } else {
            bind! { input >> i: usize, }

            let (b, c) = dual.point_query(i);

            output
                .write(
                    buf.format((a[i] as u64 * b as u64 + c as u64) % MOD)
                        .as_bytes(),
                )
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
