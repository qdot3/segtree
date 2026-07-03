/// Verified at <https://judge.yosupo.jp/problem/point_add_range_sum>
use std::io::{BufWriter, Write, stdin, stdout};

use input::{bind, parse, FastInput};
use op_add::OpAdd;
use output::IntBuffer;
use segtree_lazy::LazySegtree;
use traits::{Identity, SemiGroup};

fn main() {
    let mut input: FastInput<std::io::StdinLock<'_>> = FastInput::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    bind! { input >> n: usize, q: usize, }

    type Update = NoOp;
    type Query = OpAdd<usize>;

    let mut lst = LazySegtree::<Update, Query, _>::from_iter(
        |_, &q| q,
        (0..n).map(|_| parse!(input >> usize)),
    );

    for _ in 0..q {
        bind! { input >> t: u8, a: usize, b: usize }

        if t == 0 {
            lst.point_update(a, |v| v + b);
        } else {
            let sum = lst.range_query(a..b);

            output.write(buf.format(sum).as_bytes()).unwrap();
            output.write(b"\n").unwrap();
        }
    }
}

struct NoOp;

impl SemiGroup for NoOp {
    type Set = ();

    fn op(_: &Self::Set, _: &Self::Set) -> Self::Set {
        ()
    }
}

impl Identity for NoOp {
    fn id() -> Self::Set {
        ()
    }
}
