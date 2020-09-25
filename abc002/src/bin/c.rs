use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (x0, y0) = read!((f64, f64));
    let (x1, y1) = read!((f64, f64));
    let (x2, y2) = read!((f64, f64));
    let (a, b) = (x1 - x0, y1 - y0);
    let (c, d) = (x2 - x0, y2 - y0);
    println!("{}", (a * d - b * c).abs() / 2.0);
}
