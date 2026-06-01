use std::io::{stdin, stdout, BufWriter, Write};

use bit::BIT;
use op_add::OpAdd;
use output::IntBuffer;
use reader::FastBufReader;

fn main() {
    let mut input = FastBufReader::<{ 1 << 16 }, _>::new(stdin().lock());
    let mut output = BufWriter::with_capacity(1 << 18, stdout().lock());
    let mut buf = IntBuffer::new();

    let n: usize = input.parse_next_token().unwrap();
    let q: usize = input.parse_next_token().unwrap();

    let mut bit = BIT::<OpAdd<_>>::from(input.parse_next_token_vec::<u64>(n).unwrap());
    #[cfg(debug_assertions)]
    println!("{:?}", bit);

    for _ in 0..q {
        let t = input.next_token().unwrap();

        if unsafe { *t.get_unchecked(0) } == b'0' {
            let p: usize = input.parse_next_token().unwrap();
            let x: u64 = input.parse_next_token().unwrap();

            bit.point_update(p, x);

            #[cfg(debug_assertions)]
            println!("{:?}", bit);
        } else {
            let l: usize = input.parse_next_token().unwrap();
            let r: usize = input.parse_next_token().unwrap();

            let sum = bit.prefix_query(r) - bit.prefix_query(l);

            output.write(buf.format(sum).as_bytes()).unwrap();
            output.write(b"\n").unwrap();
        }
    }
}
