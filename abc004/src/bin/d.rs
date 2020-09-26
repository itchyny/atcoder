use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (r, g, b) = read!((i64, i64, i64));
    let ans = ((-g).min(100 - b - g)..=0.max(-100 + r))
        .flat_map(|j| {
            ((-100 - r).min(j - r)..=(-100).min(j - r)).flat_map(move |i| {
                ((100 - b).max(j + g)..=100.max(j + g))
                    .map(move |k| count(-100, r, i) + count(0, g, j) + count(100, b, k))
            })
        })
        .min()
        .unwrap();
    println!("{}", ans);
}

fn count(s: i64, n: i64, e: i64) -> i64 {
    if e <= s - n || s <= e {
        sum(e - s, n).abs()
    } else {
        sum(0, s - e + 1) + sum(0, e + n - s)
    }
}

fn sum(s: i64, n: i64) -> i64 {
    n * (s * 2 + n - 1) / 2
}
