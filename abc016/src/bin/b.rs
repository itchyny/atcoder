use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (a, b, c) = read!((i64, i64, i64));
    match (a + b == c, a - b == c) {
        (true, true) => println!("?"),
        (true, false) => println!("+"),
        (false, true) => println!("-"),
        (false, false) => println!("!"),
    }
}
