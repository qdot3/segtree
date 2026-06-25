use std::io::{stdin, stdout, BufWriter, Write};

use op_min::OpMin;
use output::IntBuffer;
use reader::FastBufReader;
use sparse_table::SparseTable;

fn main() {
    let mut input = FastBufReader::<{ 1 << 16 }, _>::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    let n: usize = input.parse_next_token().unwrap();
    let q: usize = input.parse_next_token().unwrap();

    let rmq = SparseTable::<OpMin<u32>>::from(
        input
            .parse_next_token_vec::<u32>(n)
            .unwrap()
            .into_boxed_slice(),
    );
    for _ in 0..q {
        let l: usize = input.parse_next_token().unwrap();
        let r: usize = input.parse_next_token().unwrap();

        let a = rmq.range_query(l..r).unwrap();

        output.write(buf.format(a).as_bytes()).unwrap();
        output.write(b"\n").unwrap();
    }
}
