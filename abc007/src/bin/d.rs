#[warn(unused_mut)]
use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (a, b) = read!((u64, u64));
    println!("{}", solve(a, b + 1));
}

fn solve(a: u64, b: u64) -> u64 {
    if a + 10 >= b {
        (a..b).filter(|&i| has49(i)).count() as u64
    } else if a % 10 > 0 {
        let c = (a / 10 + 1) * 10;
        solve(a, c) + solve(c, b)
    } else if b % 10 > 0 {
        let c = (b / 10) * 10;
        solve(a, c) + solve(c, b)
    } else {
        2 * (b / 10 - a / 10) + 8 * solve(a / 10, b / 10)
    }
}

fn has49(n: u64) -> bool {
    let mut n = n;
    while n > 0 {
        let m = n % 10;
        if m == 4 || m == 9 {
            return true;
        }
        n /= 10;
    }
    false
}
