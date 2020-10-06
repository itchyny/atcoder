use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    println!(
        "{}",
        (0..read!(usize)).map(|_| solve(read!(u64))).sum::<u64>()
    );
}

fn solve(n: u64) -> u64 {
    let mut k = n;
    while k % 2 == 0 || k % 3 == 2 {
        k -= 1;
    }
    n - k
}
