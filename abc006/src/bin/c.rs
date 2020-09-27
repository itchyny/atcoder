use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, m) = read!((usize, usize));
    if m < 2 * n || 4 * n < m {
        println!("-1 -1 -1");
    } else {
        let k = (m - 2 * n) / 2;
        let j = m - 2 * n - 2 * k;
        let i = n - k - j;
        println!("{} {} {}", i, j, k);
    }
}
