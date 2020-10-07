use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let m = 2025 - read!(u64);
    for i in 1..=9.min(m) {
        if m % i == 0 && m / i <= 9 {
            println!("{} x {}", i, m / i);
        }
    }
}
