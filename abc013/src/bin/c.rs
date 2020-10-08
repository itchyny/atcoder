use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, h, a, b, c, d, e) = read!((i64, i64, i64, i64, i64, i64, i64));
    let m = (0..=n)
        .map(|k| {
            let r = h + k * b - (n - k) * e;
            let l = if r > 0 { 0 } else { (-r) / (d + e) + 1 };
            a * k + c * l
        })
        .min()
        .unwrap();
    println!("{}", m);
}
