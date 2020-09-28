use std::collections::VecDeque;
use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        (I64_1) => (read!(i64) - 1);
        (Bytes) => (read!(String).into_bytes());
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (r, c) = read!((usize, usize));
    let (sx, sy) = read!((I64_1, I64_1));
    let (gx, gy) = read!((I64_1, I64_1));
    let cs = read!([Bytes; r]);

    let mut xs = vec![vec![std::u64::MAX; c]; r];
    let mut q = VecDeque::new();
    q.push_back((sx, sy, 0));
    xs[0][0] = 0;
    while let Some((x, y, d)) = q.pop_front() {
        for &(dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (x, y) = (x + dx, y + dy);
            if cs[x as usize][y as usize] == b'.' && xs[x as usize][y as usize] == std::u64::MAX {
                q.push_back((x, y, d + 1));
                xs[x as usize][y as usize] = d + 1;
            }
        }
    }
    println!("{}", xs[gx as usize][gy as usize]);
}
