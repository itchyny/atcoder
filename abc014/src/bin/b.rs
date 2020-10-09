use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, x) = read!((usize, u64));
    let xs = read!([u64; n]);
    let s: u64 = xs
        .iter()
        .enumerate()
        .filter(|(i, _)| (x >> i) & 1 != 0)
        .map(|(_, &y)| y)
        .sum();
    println!("{}", s);
}
