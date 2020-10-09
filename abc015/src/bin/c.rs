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

    let (n, k) = read!((usize, usize));
    let ts = read!([[u64; k]; n]);
    if (0..k.pow(n as u32)).any(|i| (0..n).fold(0, |x, y| x ^ ts[y][i / k.pow(y as u32) % k]) == 0)
    {
        println!("Found");
    } else {
        println!("Nothing");
    }
}
