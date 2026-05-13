/// Verified at <https://judge.yosupo.jp/problem/deque_operate_all_composite>
use std::io::{stdin, stdout, BufWriter, Write};

use foldable_deque::FoldableDeque;
use input::{bind, FastInput};
use output::IntBuffer;
use traits::{Identity, SemiGroup};

const MOD: u64 = 998_244_353;

fn main() {
    let mut input: FastInput<std::io::StdinLock<'_>> = FastInput::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    bind! { input >> q: usize, }

    let mut fq = FoldableDeque::<OpAffine>::with_capacity(q);

    for _ in 0..q {
        bind! { input >> t: u8, }

        if t == 0 {
            bind! { input >> a: u32, b: u32, }

            fq.push_front((a, b));
        } else if t == 1 {
            bind! { input >> a: u32, b: u32, }

            fq.push_back((a, b));
        } else if t == 2 {
            fq.pop_front();
        } else if t == 3 {
            fq.pop_back();
        } else {
            bind! { input >> x: u64, }

            let (a, b) = fq.fold();

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
