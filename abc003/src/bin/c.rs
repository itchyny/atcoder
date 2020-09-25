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
    let mut rs = read!([f64; n]);
    rs.sort_by(|r1, r2| r1.partial_cmp(r2).unwrap());
    let ans = rs.iter().skip(n - k).fold(0.0, |x, y| (x + y) / 2.0);
    println!("{}", ans);
}
